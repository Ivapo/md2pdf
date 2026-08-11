//! What the pane is showing, and what keeps it current.
//!
//! [`Preview`] is the state the loop writes: the last good PDF bytes, how long
//! they took, the image list the filter needs, whether the page still belongs
//! to the text on disk, and the error when there is one. [`Session`] is that
//! state plus the watch that keeps it up to date. Neither needs a window, so
//! both are tested by ordinary tests rather than by a screenshot.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::document;
use crate::watch::{self, Watch};

/// What the window says about the last compile.
///
/// Four states, and the app held one bit until this became four. What separates
/// *stale* from *failed* is whether there are bytes to keep, because
/// [`Preview::compile`] sets the stale mark on **every** failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    /// No document has been opened. The app launches into this and holds it
    /// until the first Open.
    Empty,
    /// The last compile succeeded, and the page belongs to it.
    Current,
    /// The last compile failed, and an older page is still drawn.
    Stale,
    /// The last compile failed with no page to keep — the open that never
    /// compiled.
    Failed,
}

/// The status line, as a value rather than as chrome.
///
/// Every word in it is chosen here and the page only places it. A window that
/// worded its own status would be checkable by eye alone, and the spec keeps
/// that list to the one claim no test can hold.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Status {
    /// Which of the four states the pane is in.
    pub state: State,
    /// How long the compile that produced the drawn page took, worded for the
    /// window: `"28 ms"`. `None` when no page is drawn.
    pub time: Option<String>,
    /// The message from the last compile, if it failed.
    pub error: Option<String>,
    /// Is a page drawn under the message?
    pub page: bool,
}

/// The pane's state, as Rust holds it.
///
/// The bytes live here rather than only in the page, because the loop is what
/// compiled them and the export has to write the same bytes the pane is
/// showing — a file and a page that disagree would be worse than neither.
#[derive(Default)]
pub struct Preview {
    document: Option<PathBuf>,
    images: Vec<String>,
    pdf: Option<Vec<u8>>,
    elapsed: Option<Duration>,
    stale: bool,
    error: Option<String>,
}

impl Preview {
    /// The last good bytes, whether or not they are still current.
    pub fn pdf(&self) -> Option<&[u8]> {
        self.pdf.as_deref()
    }

    /// The document the pane is showing, if one is open.
    pub fn document(&self) -> Option<&Path> {
        self.document.as_deref()
    }

    /// Does the page belong to older text than the file on disk?
    pub fn is_stale(&self) -> bool {
        self.stale
    }

    /// The message from the last compile, if it failed.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Which of the four states the pane is in.
    ///
    /// *Empty* is exactly "no document has been opened", and that is the right
    /// boundary rather than one more condition: [`Preview::compile`] returns
    /// early with no document, and [`Session::open`] sets the document and
    /// compiles inside one lock scope, so no observable state sits between
    /// [`Preview::default`] and the first outcome.
    ///
    /// The last arm also absorbs the pair that enumeration leaves — a document
    /// set with no bytes and no failure — and calling it *failed* is the safe
    /// direction, because a *failed* pane refuses an export.
    pub fn state(&self) -> State {
        match (self.document.is_some(), self.stale, self.pdf.is_some()) {
            (false, _, _) => State::Empty,
            (true, false, true) => State::Current,
            (true, true, true) => State::Stale,
            (true, _, false) => State::Failed,
        }
    }

    /// Everything the window says about the last compile, in one value.
    pub fn status(&self) -> Status {
        Status {
            state: self.state(),
            time: self.elapsed.map(|took| format!("{} ms", took.as_millis())),
            error: self.error.clone(),
            page: self.pdf.is_some(),
        }
    }

    /// The bytes an export may write, or why it may not.
    ///
    /// Only a *current* pane has them. **The two refusals are two sentences
    /// because they are two problems**: an *empty* pane holds no bytes at all,
    /// where a *stale* or *failed* one holds bytes that are known to belong to
    /// older text. A caller that reported one for the other would send the
    /// reader looking for the wrong thing.
    fn exportable(&self) -> Result<&[u8], String> {
        match (self.state(), self.pdf.as_deref()) {
            (State::Current, Some(pdf)) => Ok(pdf),
            (State::Empty, _) => Err("no document is open".to_string()),
            _ => Err("the last compile failed, so the page is out of date".to_string()),
        }
    }

    /// Where a Save-a-copy dialog opens, or why it does not open at all.
    ///
    /// It refuses before the dialog rather than after it, so a pane that cannot
    /// be exported never asks the user for a path it will not use.
    pub fn export_path(&self) -> Result<PathBuf, String> {
        self.exportable()?;
        self.document()
            .map(document::default_output)
            .ok_or_else(|| "no document is open".to_string())
    }

    /// Write the page's own bytes where the user asked.
    ///
    /// **Nothing here compiles.** The export writes what the pane is already
    /// showing, so the file and the page cannot disagree.
    pub fn export(&self, path: &Path) -> Result<(), String> {
        let pdf = self.exportable()?;
        std::fs::write(path, pdf).map_err(|e| format!("cannot write {}: {e}", path.display()))
    }

    /// Compile the open document and take in what came back.
    ///
    /// A success replaces the bytes and clears both the error and the stale
    /// mark. **A failure keeps the bytes**, records the message and sets the
    /// mark: an author mid-edit passes through broken states constantly, and
    /// blanking the pane on each one would lose their place and make the loop
    /// worse than the command it replaces. The mark is what stops the kept
    /// page from silently claiming to be the current text.
    ///
    /// **The duration travels with the bytes**, replaced on a success and kept
    /// on a failure exactly as they are, so the time the window shows always
    /// describes the page on screen rather than the last attempt at one.
    pub fn compile(&mut self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        let started = Instant::now();
        let render = document::render(&document);
        let took = started.elapsed();

        if let Some(images) = render.images {
            self.images = images;
        }

        match render.pdf {
            Ok(pdf) => {
                self.pdf = Some(pdf);
                self.elapsed = Some(took);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

    fn sample(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../samples")
            .join(name)
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../tests/fixtures")
            .join(name)
    }

    /// A scratch directory this test owns.
    ///
    /// It sits under `std::env::temp_dir()` deliberately: on macOS that
    /// resolves through a symlink, so a filter that forgets to canonicalize
    /// fails these cases loudly rather than passing under some directory that
    /// happens not to be symlinked.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("md2pdf-preview-test-{}", std::process::id()))
            .join(name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_ne!(
            dir.canonicalize().unwrap(),
            dir,
            "the scratch directory is not symlinked, so these cases prove less than they claim"
        );
        dir
    }

    /// A session whose compiles can be counted, which is the seam every case
    /// below is read through.
    fn counted() -> (Session, Arc<AtomicUsize>) {
        let compiles = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&compiles);
        let session = Session::new(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        });
        (session, compiles)
    }

    /// Wait for the loop to reach a compile count, or give up.
    ///
    /// The bound is generous because FSEvents' own coalescing sits under the
    /// debounce. It is a bound on wiring, not a measurement: the count itself
    /// is pinned by `watch::tests`, which needs no filesystem.
    fn wait_for(compiles: &AtomicUsize, target: usize) -> usize {
        let deadline = Instant::now() + Duration::from_secs(10);
        while Instant::now() < deadline {
            let seen = compiles.load(Ordering::SeqCst);
            if seen >= target {
                return seen;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        compiles.load(Ordering::SeqCst)
    }

    /// Give a change that should *not* compile long enough to prove it.
    fn settle() {
        std::thread::sleep(watch::DEBOUNCE * 8);
    }

    /// A copy of `samples/article.md` and both the figures it names.
    fn article_in(dir: &Path) -> PathBuf {
        let document = dir.join("article.md");
        std::fs::copy(sample("article.md"), &document).unwrap();
        std::fs::copy(sample("pipeline.svg"), dir.join("pipeline.svg")).unwrap();
        std::fs::copy(sample("check.svg"), dir.join("check.svg")).unwrap();
        document
    }

    /// A compile error keeps the bytes and sets the mark. Fixing the document
    /// clears both.
    #[test]
    fn a_failed_compile_keeps_the_last_good_page_and_marks_it_stale() {
        let dir = scratch_dir("stale");
        let document = article_in(&dir);

        let mut preview = Preview {
            document: Some(document.clone()),
            ..Preview::default()
        };
        preview.compile();

        let good = preview.pdf().unwrap().to_vec();
        assert!(good.starts_with(b"%PDF"));
        assert!(!preview.is_stale());
        assert_eq!(preview.error(), None);

        std::fs::write(&document, "# Broken\n\n<div>raw HTML</div>\n").unwrap();
        preview.compile();

        assert_eq!(preview.pdf(), Some(good.as_slice()));
        assert!(preview.is_stale());
        assert!(preview.error().unwrap().contains("raw HTML block"));

        std::fs::write(&document, "# Fixed\n\nOrdinary text.\n").unwrap();
        preview.compile();

        assert!(!preview.is_stale());
        assert_eq!(preview.error(), None);
        assert_ne!(preview.pdf(), Some(good.as_slice()));
    }

    /// The document and a figure it names each redraw the page. This is the
    /// real watcher, on a real directory, through the code the window runs.
    #[test]
    fn a_saved_document_and_a_replaced_figure_each_compile_again() {
        let dir = scratch_dir("watch-article");
        let document = article_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");

        let markdown = std::fs::read_to_string(&document).unwrap();
        std::fs::write(&document, markdown.replace("Introduction", "The start")).unwrap();
        assert_eq!(wait_for(&compiles, 2), 2, "saving the document compiles");

        std::fs::write(
            dir.join("pipeline.svg"),
            std::fs::read(sample("check.svg")).unwrap(),
        )
        .unwrap();
        assert_eq!(wait_for(&compiles, 3), 3, "replacing a figure compiles");

        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));
        assert!(!session.preview().is_stale());
    }

    /// A figure the document names minutes before anyone creates it.
    ///
    /// This is the case a watch set of files could not have held — `notify`'s
    /// macOS backend refuses to register a path that does not exist — and the
    /// one the directory answer exists for.
    #[test]
    fn a_figure_that_does_not_exist_yet_is_watched_and_then_compiles() {
        let dir = scratch_dir("figure-to-come");
        let document = dir.join("paper.md");
        std::fs::write(&document, "# Paper\n\n![a mark to come](figures/new.svg)\n").unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);
        assert!(
            session
                .preview()
                .error()
                .unwrap()
                .contains("figures/new.svg")
        );
        assert!(session.preview().pdf().is_none());

        std::fs::create_dir_all(dir.join("figures")).unwrap();
        std::fs::copy(fixture("mark.svg"), dir.join("figures/new.svg")).unwrap();

        assert!(wait_for(&compiles, 2) >= 2, "creating the figure compiles");
        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));
        assert!(!session.preview().is_stale());
    }

    /// Opening a second document moves the watch. An implementer who set the
    /// watcher up once rather than per document passes every case above and
    /// fails this one.
    #[test]
    fn opening_a_second_document_moves_the_watch() {
        let first_dir = scratch_dir("first");
        let second_dir = scratch_dir("second");
        let first = article_in(&first_dir);
        let second = article_in(&second_dir);

        let (mut session, compiles) = counted();
        session.open(first.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        session.open(second.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 2), 2);

        std::fs::write(&first, "# The first, edited\n").unwrap();
        settle();
        assert_eq!(
            compiles.load(Ordering::SeqCst),
            2,
            "the first document is no longer watched"
        );

        std::fs::write(&second, "# The second, edited\n").unwrap();
        assert_eq!(wait_for(&compiles, 3), 3, "the second document is watched");
    }
}
