//! What the pane is showing, and what keeps it current.
//!
//! [`Preview`] is the state the loop writes: the last good PDF bytes, the
//! image list the filter needs, whether the page still belongs to the text on
//! disk, and the error when there is one. [`Session`] is that state plus the
//! watch that keeps it up to date. Neither needs a window, so both are tested
//! by ordinary tests rather than by a screenshot.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::document;
use crate::watch::{self, Watch};

/// The pane's state, as Rust holds it.
///
/// The bytes live here rather than only in the page, because the loop is what
/// compiled them and Phase 3's export has to write the same bytes the pane is
/// showing — a file and a page that disagree would be worse than neither.
#[derive(Default)]
pub struct Preview {
    document: Option<PathBuf>,
    images: Vec<String>,
    pdf: Option<Vec<u8>>,
    stale: bool,
    error: Option<String>,
}

impl Preview {
    /// The last good bytes, whether or not they are still current.
    pub fn pdf(&self) -> Option<&[u8]> {
        self.pdf.as_deref()
    }

    /// Does the page belong to older text than the file on disk?
    pub fn is_stale(&self) -> bool {
        self.stale
    }

    /// The message from the last compile, if it failed.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Compile the open document and take in what came back.
    ///
    /// A success replaces the bytes and clears both the error and the stale
    /// mark. **A failure keeps the bytes**, records the message and sets the
    /// mark: an author mid-edit passes through broken states constantly, and
    /// blanking the pane on each one would lose their place and make the loop
    /// worse than the command it replaces. The mark is what stops the kept
    /// page from silently claiming to be the current text.
    pub fn compile(&mut self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        let render = document::render(&document);

        if let Some(images) = render.images {
            self.images = images;
        }

        match render.pdf {
            Ok(pdf) => {
                self.pdf = Some(pdf);
                self.stale = false;
                self.error = None;
            }
            Err(message) => {
                self.stale = true;
                self.error = Some(message);
            }
        }
    }
}

/// One open document: its preview, and the watch that keeps it current.
///
/// Opening a second document moves the watch, because the old [`Watch`] is
/// dropped before the new one starts.
pub struct Session {
    state: Arc<Mutex<Preview>>,
    on_render: Arc<dyn Fn() + Send + Sync>,
    watch: Option<Watch>,
}

impl Session {
    /// A session that calls `on_render` after every compile, its own included.
    ///
    /// The callback carries no payload. The window's copy of it emits an event
    /// and the page then asks for the bytes, because handing them through the
    /// event would serialize them as a JSON array of numbers, one per byte.
    pub fn new(on_render: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            state: Arc::new(Mutex::new(Preview::default())),
            on_render: Arc::new(on_render),
            watch: None,
        }
    }

    /// What the pane is showing. Phase 3's export reads the same values.
    pub fn preview(&self) -> std::sync::MutexGuard<'_, Preview> {
        self.state.lock().expect("the preview lock was poisoned")
    }

    /// Open a document: compile it once, and watch its directory from now on.
    ///
    /// The previous document's page goes with it. A page kept across an open
    /// would belong to a file the window no longer names.
    pub fn open(&mut self, document: PathBuf) -> Result<(), String> {
        let root = watch::root(&document);

        {
            let mut preview = self.preview();
            *preview = Preview {
                document: Some(document.clone()),
                ..Preview::default()
            };
            preview.compile();
        }
        (self.on_render)();

        // The old watch goes before the new one starts, so the two never both
        // hold the same directory.
        self.watch = None;
        self.watch = Some(watch::start(
            &root,
            watch::DEBOUNCE,
            self.filter(document.clone()),
            self.recompile(document),
        )?);

        Ok(())
    }

    /// The filter, closed over the image list the last successful parse left.
    fn filter(&self, document: PathBuf) -> impl Fn(&Path) -> bool + Send + 'static {
        let state = Arc::clone(&self.state);
        move |path| {
            let images = state
                .lock()
                .expect("the preview lock was poisoned")
                .images
                .clone();
            watch::is_relevant(path, &document, &images)
        }
    }

    /// What one settled change does.
    ///
    /// It checks the document first. Dropping a [`Watch`] does not join its
    /// thread, so a thread that was mid-compile when a second document opened
    /// could otherwise write its page over the newer one.
    fn recompile(&self, document: PathBuf) -> impl FnMut() + Send + 'static {
        let state = Arc::clone(&self.state);
        let on_render = Arc::clone(&self.on_render);

        move || {
            {
                let mut preview = state.lock().expect("the preview lock was poisoned");
                if preview.document.as_deref() != Some(document.as_path()) {
                    return;
                }
                preview.compile();
            }
            on_render();
        }
    }
}
