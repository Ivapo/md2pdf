//! What the pane is showing, and what keeps it current.
//!
//! [`Preview`] is the state the loop writes: the text the pane holds, the last
//! good PDF bytes, how long they took, the asset list the filter needs,
//! whether the page still belongs to that text, and the error when there is
//! one. [`Session`] is that state plus the two loops that keep it up to date —
//! the watch, and the keyboard. Neither needs a window, so both are tested by
//! ordinary tests rather than by a screenshot.
//!
//! **The buffer is what compiles.** The file beside it need never have held
//! that text, and the two are compared rather than conflated: [`external_change`]
//! is the whole of what an event naming the open document now means.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;

use crate::document;
use crate::watch::{self, Change, Changed, Watch};

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

/// What an external change to the open document did.
///
/// The three are exhaustive over three strings, and they need no dirty flag:
/// the file equal to the buffer is decided first, whatever the last-saved text
/// is, and the rest splits on whether the buffer holds unsaved edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum External {
    /// The file already says what the buffer says — the app's own save
    /// arriving back, or a change that changed nothing.
    Unchanged,
    /// The buffer was clean, so nothing could be lost. The disk copy is the
    /// pane's text now. **This is the loop the app has shipped since Phase 2.**
    Taken,
    /// The buffer held unsaved edits and the disk moved under them. The work
    /// is kept and the divergence is named.
    Diverged,
}

/// The report a refused external change leaves for the author.
///
/// It names both ways out and takes neither: saving overwrites the disk,
/// reopening takes it. **The app does not merge** — a three-way merge is an
/// editor project, and this one is not that.
const DIVERGED: &str = "this file changed on disk, and the pane holds unsaved edits. \
    Save to write the pane over the file, or open the file again to take it.";

/// OQ-5's rule: what an event naming the open document means.
///
/// Three strings and two comparisons. `file` is what the disk holds now,
/// `buffer` is what the pane holds, and `saved` is the text as it stood at the
/// last open or save.
///
/// **Refusing every external change would have been the wrong answer**, and
/// the condition is what makes this one rule rather than a compromise: an
/// author who is not typing has a clean buffer, so a save in another editor
/// still redraws the page with no action taken in the window, which is the
/// loop Phase 2 shipped and the README documents.
///
/// Two limits it accepts. An author who keeps typing between a save and that
/// save's event lands in [`External::Diverged`], so the app can name a
/// divergence that was really its own write — it loses nothing, and the next
/// save clears it. And an external writer that happens to write exactly the
/// author's unsaved text takes [`External::Unchanged`], which leaves the
/// last-saved text unrefreshed. Both err toward keeping work.
pub fn external_change(file: &str, buffer: &str, saved: &str) -> External {
    if file == buffer {
        External::Unchanged
    } else if buffer == saved {
        External::Taken
    } else {
        External::Diverged
    }
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
    /// The report a refused external change left, if there is one.
    ///
    /// **A divergence is not [`State::Stale`]**: nothing failed to compile,
    /// and the page on screen belongs to the text in the pane. It is the file
    /// that has gone elsewhere.
    pub divergence: Option<String>,
    /// How many times a compile has succeeded, so the page can tell new bytes
    /// from a status that merely arrived.
    ///
    /// It is what stops a signal carrying no new page from redrawing the frame
    /// and throwing the reader back to page 1 — which the app's own save now
    /// does, since its event compiles nothing.
    pub revision: u64,
    /// How many times the buffer has been replaced from disk.
    ///
    /// The page re-reads the text on this and on nothing else, so a keystroke
    /// in flight can never lose a race with a fetch of text it just sent.
    pub reloaded: u64,
    /// Where each heading landed in the page the pane is showing.
    ///
    /// The page picks the last one at or above its caret and opens the frame
    /// there. It rides the status because the status is already fetched on the
    /// path that draws, so this needs no command of its own.
    pub anchors: Vec<document::Anchor>,
    /// The sections the open master names, in the order it reads them.
    ///
    /// It rides the status for the reason the anchors do: the status is already
    /// fetched on the path that draws, so the panel needs no command of its own.
    /// Empty for a document that names no section, which is what draws no panel.
    pub sections: Vec<String>,
    /// The file name of the document the pane is holding, if one is open.
    ///
    /// The panel lists the master above the sections it names, and the master
    /// is the one file in that list this page could not otherwise name. The
    /// *name* and not the path: the panel is a list of one document's parts,
    /// and where that document sits on the disk is the title bar's business.
    pub master: Option<String>,
}

/// The pane's state, as Rust holds it.
///
/// The bytes live here rather than only in the page, because the loop is what
/// compiled them and the export has to write the same bytes the pane is
/// showing — a file and a page that disagree would be worse than neither.
///
/// **The text lives here too**, for the same kind of reason: the rule that
/// decides what an external change does is three comparisons over strings, and
/// a buffer that lived only in the page would put that rule in the window,
/// where no test could reach it.
#[derive(Default)]
pub struct Preview {
    document: Option<PathBuf>,
    buffer: String,
    saved: String,
    assets: Vec<String>,
    sections: Vec<String>,
    pdf: Option<Vec<u8>>,
    anchors: Vec<document::Anchor>,
    elapsed: Option<Duration>,
    revision: u64,
    reloaded: u64,
    stale: bool,
    error: Option<String>,
    divergence: Option<String>,
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

    /// The text the pane holds, which is the text that compiles.
    pub fn text(&self) -> &str {
        &self.buffer
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
            divergence: self.divergence.clone(),
            revision: self.revision,
            reloaded: self.reloaded,
            anchors: self.anchors.clone(),
            sections: self.sections.clone(),
            master: self
                .document
                .as_ref()
                .and_then(|path| path.file_name())
                .map(|name| name.to_string_lossy().into_owned()),
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

    /// Take the pane's text.
    ///
    /// It compiles nothing. The typing debounce decides when a compile falls
    /// due, because one keystroke is not a document.
    pub fn edit(&mut self, text: String) {
        if self.document.is_some() {
            self.buffer = text;
        }
    }

    /// Read the document from disk into the buffer, and compile it.
    ///
    /// A file that will not read leaves the same message and the same *failed*
    /// state a compile failure leaves, because that is what the author needs
    /// to see either way and it is the sentence the terminal prints.
    pub fn load(&mut self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        match document::read_document(&document) {
            Ok(text) => {
                self.take(text);
                self.compile();
            }
            Err(message) => {
                self.stale = true;
                self.error = Some(message);
            }
        }
    }

    /// Write the buffer to the open document's path.
    ///
    /// The last-saved text moves with it, which is what makes the buffer clean
    /// again — and what makes this save's own filesystem event take
    /// [`External::Unchanged`] a moment later, with no second compile and no
    /// suppression that would have to win a race.
    pub fn save(&mut self) -> Result<(), String> {
        let document = self
            .document
            .clone()
            .ok_or_else(|| "no document is open".to_string())?;

        std::fs::write(&document, &self.buffer)
            .map_err(|e| format!("cannot write {}: {e}", document.display()))?;

        self.saved = self.buffer.clone();
        self.divergence = None;
        Ok(())
    }

    /// The disk moved under the open document: decide what that means.
    ///
    /// This is [`external_change`] with the file read for it and its answer
    /// carried out. A document that will not read at this instant — one caught
    /// mid-write — counts as [`External::Unchanged`]: the app keeps what it
    /// has, and the write's next event decides.
    pub fn reload(&mut self) -> External {
        let Some(document) = self.document.clone() else {
            return External::Unchanged;
        };
        let Ok(file) = document::read_document(&document) else {
            return External::Unchanged;
        };

        let outcome = external_change(&file, &self.buffer, &self.saved);
        match outcome {
            External::Unchanged => {}
            External::Taken => {
                self.take(file);
                self.compile();
            }
            External::Diverged => self.divergence = Some(DIVERGED.to_string()),
        }
        outcome
    }

    /// Take a text from disk as both the buffer and the last-saved text.
    ///
    /// The count it bumps is how the page knows to re-read: it replaces its
    /// own text on this and on nothing else, so text the author is typing is
    /// never overwritten by a fetch that raced it.
    fn take(&mut self, text: String) {
        self.saved = text.clone();
        self.buffer = text;
        self.reloaded += 1;
        self.divergence = None;
    }

    /// Compile the pane's text and take in what came back.
    ///
    /// A success replaces the bytes and clears both the error and the stale
    /// mark. **A failure keeps the bytes**, records the message and sets the
    /// mark: an author mid-edit passes through broken states constantly, and
    /// blanking the pane on each one would lose their place and make the loop
    /// worse than the command it replaces. The mark is what stops the kept
    /// page from silently claiming to be the current text.
    ///
    /// **The duration and the anchors travel with the bytes**, replaced on a
    /// success and kept on a failure exactly as they are, so the time the window
    /// shows and the page the pane opens on always describe the page on screen
    /// rather than the last attempt at one.
    pub fn compile(&mut self) {
        let Some(document) = self.document.clone() else {
            return;
        };

        let started = Instant::now();
        let render = document::render(document::directory(&document), &self.buffer);
        let took = started.elapsed();

        if let Some(assets) = render.assets {
            self.assets = assets;
        }

        // Taken whether or not the compile succeeded, as the asset list above
        // is and for the same reason: it is read off the text rather than the
        // page, so a document that will not compile still names its sections.
        self.sections = render.sections;

        match render.pdf {
            Ok(pdf) => {
                self.pdf = Some(pdf);
                self.anchors = render.anchors;
                self.elapsed = Some(took);
                self.revision += 1;
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

/// One open document: its preview, and the two loops that keep it current.
///
/// Opening a second document moves both, because the old [`Watch`] and the old
/// typing channel are dropped before the new ones start.
pub struct Session {
    state: Arc<Mutex<Preview>>,
    on_render: Arc<dyn Fn() + Send + Sync>,
    watch: Option<Watch>,
    typing: Option<mpsc::Sender<()>>,
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
            typing: None,
        }
    }

    /// What the pane is showing. Phase 3's export reads the same values.
    pub fn preview(&self) -> std::sync::MutexGuard<'_, Preview> {
        self.state.lock().expect("the preview lock was poisoned")
    }

    /// Open a document: read it, compile it once, and watch its directory from
    /// now on.
    ///
    /// The previous document's page and text go with it. A page kept across an
    /// open would belong to a file the window no longer names.
    pub fn open(&mut self, document: PathBuf) -> Result<(), String> {
        let root = watch::root(&document);

        {
            let mut preview = self.preview();
            *preview = Preview {
                document: Some(document.clone()),
                ..Preview::default()
            };
            preview.load();
        }
        (self.on_render)();

        // The old loops go before the new ones start, so no two of them ever
        // hold the same document.
        self.watch = None;
        self.typing = None;

        self.typing = Some(watch::debounced(
            watch::TYPING_DEBOUNCE,
            self.recompile(document.clone()),
        ));
        self.watch = Some(watch::start(
            &root,
            watch::DEBOUNCE,
            self.classifier(document.clone()),
            self.on_change(document),
        )?);

        Ok(())
    }

    /// Take the pane's text, and start the clock on the compile it will want.
    ///
    /// The keystroke crosses the IPC boundary and the debounce is Rust's,
    /// which is what puts this on the testable side of the window.
    pub fn edit(&self, text: String) {
        self.preview().edit(text);
        if let Some(typing) = &self.typing {
            let _ = typing.send(());
        }
    }

    /// Write the pane's text to the document's own path.
    pub fn save(&self) -> Result<(), String> {
        self.preview().save()
    }

    /// The filter, closed over the asset list the last successful parse left.
    ///
    /// That list follows the buffer, because the buffer is the document now: a
    /// figure named in text that has not been saved is watched for all the
    /// same.
    fn classifier(&self, document: PathBuf) -> impl Fn(&Path) -> Option<Change> + Send + 'static {
        let state = Arc::clone(&self.state);
        move |path| {
            let assets = state
                .lock()
                .expect("the preview lock was poisoned")
                .assets
                .clone();
            watch::classify(path, &document, &assets)
        }
    }

    /// What one settled window of filesystem events does.
    ///
    /// **The document and the assets reach different code**, which is the
    /// whole of what this phase changed in the loop. An asset that moved is
    /// still a bare recompile, because nothing but the disk supplies a figure
    /// or a bibliography. The document that moved runs [`Preview::reload`],
    /// because the pane's own text is what compiles and the file is now a
    /// second opinion about it.
    ///
    /// A window that took the disk copy compiled inside the rule, and it read
    /// the new assets on the way, so the two never compile twice for one
    /// window. And nothing is announced when nothing happened: the app's own
    /// save arrives here, changes nothing, and must not redraw a frame the
    /// reader has scrolled.
    fn on_change(&self, document: PathBuf) -> impl FnMut(Changed) + Send + 'static {
        let state = Arc::clone(&self.state);
        let on_render = Arc::clone(&self.on_render);

        move |changed: Changed| {
            let mut announce = false;
            {
                let mut preview = state.lock().expect("the preview lock was poisoned");
                if preview.document.as_deref() != Some(document.as_path()) {
                    return;
                }

                let taken = if changed.document {
                    let outcome = preview.reload();
                    announce = outcome != External::Unchanged;
                    outcome == External::Taken
                } else {
                    false
                };

                if changed.assets && !taken {
                    preview.compile();
                    announce = true;
                }
            }
            if announce {
                on_render();
            }
        }
    }

    /// What one settled pause in the typing does.
    ///
    /// It checks the document first. Dropping a [`Watch`] or a typing channel
    /// does not join its thread, so a thread that was mid-compile when a
    /// second document opened could otherwise write its page over the newer
    /// one.
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
    ///
    /// It clears both intervals several times over: a keystroke's compile
    /// falls due after [`watch::TYPING_DEBOUNCE`], which is the longer of the
    /// two, and a filesystem event's after [`watch::DEBOUNCE`].
    fn settle() {
        std::thread::sleep(watch::TYPING_DEBOUNCE * 4);
    }

    /// A copy of `samples/article.md` and both the figures it names.
    fn article_in(dir: &Path) -> PathBuf {
        let document = dir.join("article.md");
        std::fs::copy(sample("article.md"), &document).unwrap();
        std::fs::copy(sample("pipeline.svg"), dir.join("pipeline.svg")).unwrap();
        std::fs::copy(sample("check.svg"), dir.join("check.svg")).unwrap();
        document
    }

    /// A copy of `tests/fixtures/citations.md` and the bibliography it names.
    fn citing_document_in(dir: &Path) -> PathBuf {
        let document = dir.join("citations.md");
        std::fs::copy(fixture("citations.md"), &document).unwrap();
        std::fs::copy(fixture("refs.yml"), dir.join("refs.yml")).unwrap();
        document
    }

    /// A copy of `tests/fixtures/multi_file.md`, the three sections it names
    /// and the two figures those sections name.
    ///
    /// The figures go into `sections/` and not beside the master, which is the
    /// layout Phase 2 shipped: `introduction.md` writes a bare `dot.png` and
    /// the emitter resolves it against the folder that file lives in.
    fn multi_file_in(dir: &Path) -> PathBuf {
        let document = dir.join("multi_file.md");
        std::fs::copy(fixture("multi_file.md"), &document).unwrap();

        std::fs::create_dir_all(dir.join("sections")).unwrap();
        for name in [
            "introduction.md",
            "method.md",
            "results.md",
            "dot.png",
            "mark.svg",
        ] {
            std::fs::copy(
                fixture(&format!("sections/{name}")),
                dir.join("sections").join(name),
            )
            .unwrap();
        }
        document
    }

    /// A preview holding one document, read from disk and compiled, built
    /// without a session.
    fn compiled(document: &Path) -> Preview {
        let mut preview = Preview {
            document: Some(document.to_path_buf()),
            ..Preview::default()
        };
        preview.load();
        preview
    }

    /// The two figures `samples/article.md` names, read by this test rather
    /// than by the reader under test.
    ///
    /// It names each of them once, so there is no dedup subtlety to mirror.
    /// Reading them here is what makes the assertion independent: an
    /// `md_to_pdf` fed by `app`'s own reader would only prove that reader
    /// agrees with itself.
    fn article_assets(dir: &Path) -> Vec<md2pdf_core::Asset> {
        ["pipeline.svg", "check.svg"]
            .into_iter()
            .map(|name| md2pdf_core::Asset {
                path: name.to_string(),
                bytes: std::fs::read(dir.join(name)).unwrap(),
            })
            .collect()
    }

    /// A compile error keeps the bytes and sets the mark. Fixing the text
    /// clears both.
    ///
    /// The broken states are typed rather than written to the file, because
    /// typing is how an author reaches them: a half-typed table, a fence not
    /// yet closed.
    #[test]
    fn a_failed_compile_keeps_the_last_good_page_and_marks_it_stale() {
        let dir = scratch_dir("stale");
        let mut preview = compiled(&article_in(&dir));

        let good = preview.pdf().unwrap().to_vec();
        assert!(good.starts_with(b"%PDF"));
        assert!(!preview.is_stale());
        assert_eq!(preview.error(), None);

        preview.edit("# Broken\n\n<div>raw HTML</div>\n".to_string());
        preview.compile();

        assert_eq!(preview.pdf(), Some(good.as_slice()));
        assert!(preview.is_stale());
        assert!(preview.error().unwrap().contains("raw HTML block"));

        preview.edit("# Fixed\n\nOrdinary text.\n".to_string());
        preview.compile();

        assert!(!preview.is_stale());
        assert_eq!(preview.error(), None);
        assert_ne!(preview.pdf(), Some(good.as_slice()));
    }

    /// The document and a figure it names each redraw the page. This is the
    /// real watcher, on a real directory, through the code the window runs.
    ///
    /// The document's half is the loop Phase 2 shipped, and it survives the
    /// text pane exactly because the buffer here is clean: nobody has typed,
    /// so nothing can be lost, and the rule takes the disk copy.
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

    /// The bibliography moves and nothing else does, and the page that comes
    /// back is a real one.
    ///
    /// **The compile count is not the assertion here.** [`Session::on_change`]
    /// calls `on_render()` whenever the asset mark is set, a failed compile
    /// included, so a counter alone passes an app that publishes the path to
    /// the filter and never supplies the bytes: it would go 1 → 2 while every
    /// compile of the two errored `MissingBibliography`. The bytes and the
    /// stale mark are what tell the two apart.
    #[test]
    fn a_replaced_bibliography_compiles_again_and_the_page_is_good() {
        let dir = scratch_dir("watch-bibliography");
        let document = citing_document_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");
        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));

        // The same key under a different record, so the citation still
        // resolves: a compile that reached these bytes succeeds, and one that
        // reached nothing fails on the file rather than on the key.
        std::fs::write(
            dir.join("refs.yml"),
            concat!(
                "\"DBLP:books/lib/Knuth86a\":\n",
                "  type: book\n",
                "  title: The TeXbook\n",
                "  author: Knuth, Donald E.\n",
                "  date: 1984\n",
                "  publisher: Addison-Wesley\n",
            ),
        )
        .unwrap();
        assert_eq!(
            wait_for(&compiles, 2),
            2,
            "rewriting the bibliography compiles"
        );

        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));
        assert!(!session.preview().is_stale());
    }

    /// A bibliography the document names minutes before anyone creates it.
    ///
    /// The sibling of
    /// [`a_figure_that_does_not_exist_yet_is_watched_and_then_compiles`], and
    /// the case that fails for an app publishing the path only out of a
    /// successful read: the first compile has no bytes at all, so the list the
    /// filter holds can only have come from the text.
    #[test]
    fn a_bibliography_that_does_not_exist_yet_is_watched_and_then_compiles() {
        let dir = scratch_dir("bibliography-to-come");
        let document = dir.join("citations.md");
        std::fs::copy(fixture("citations.md"), &document).unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        let error = session.preview().error().unwrap().to_string();
        assert!(error.contains("refs.yml"), "{error}");
        assert!(error.contains("line 3"), "{error}");
        assert!(session.preview().pdf().is_none());

        std::fs::copy(fixture("refs.yml"), dir.join("refs.yml")).unwrap();

        assert!(
            wait_for(&compiles, 2) >= 2,
            "creating the bibliography compiles"
        );
        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));
        assert!(!session.preview().is_stale());
    }

    /// A section changes and the page comes back, and it is a real one.
    ///
    /// The multi-file sibling of
    /// [`a_replaced_bibliography_compiles_again_and_the_page_is_good`], and its
    /// warning applies here too: [`Session::on_change`] announces whenever the
    /// asset mark is set, a failed compile included, so a counter alone would
    /// pass an app that published the section's path to the filter and never
    /// supplied its bytes. The bytes and the stale mark tell the two apart.
    ///
    /// **It is a bounded wait and not a latency.** Two intervals sit under it —
    /// [`watch::DEBOUNCE`] at 100 ms for the filesystem and
    /// [`watch::TYPING_DEBOUNCE`] at 300 ms for the pane — and neither is
    /// asserted end to end anywhere, as [`wait_for`]'s own comment says.
    #[test]
    fn a_section_that_changes_compiles_again_and_the_page_is_good() {
        let dir = scratch_dir("watch-section");
        let document = multi_file_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");
        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));

        // Appended rather than replaced: this section declares `#fig:mark` and
        // cites the footnote the third file defines, so a rewrite that dropped
        // either would fail on the name and never reach the bytes.
        let section = dir.join("sections/method.md");
        let mut text = std::fs::read_to_string(&section).unwrap();
        text.push_str("\nA paragraph this test added to the second file.\n");
        std::fs::write(&section, text).unwrap();

        assert_eq!(
            wait_for(&compiles, 2),
            2,
            "editing a section compiles again"
        );
        assert!(session.preview().pdf().unwrap().starts_with(b"%PDF"));
        assert!(!session.preview().is_stale());
    }

    /// An error inside a section names that section's own file and its own
    /// line, on the exact string.
    ///
    /// That string is `md2pdf_core::Error`'s `Display`, so this is the
    /// `in FILE at line N` phrase Phase 1 shipped, arriving at the window
    /// unaltered. The author wrote the block on line 3 of a file of their own;
    /// the joined document it was refused in exists nowhere.
    #[test]
    fn an_error_in_a_section_names_that_file_and_its_own_line() {
        let dir = scratch_dir("error-in-a-section");
        let document = dir.join("report.md");
        std::fs::write(&document, "[](sections/one.md)\n").unwrap();
        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(
            dir.join("sections/one.md"),
            "# A section\n\n<div>a raw HTML block</div>\n",
        )
        .unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        assert_eq!(
            session.preview().error(),
            Some("unsupported markdown construct 'raw HTML block' in sections/one.md at line 3")
        );
    }

    /// A section the master names minutes before anyone creates it.
    ///
    /// The sibling of
    /// [`a_bibliography_that_does_not_exist_yet_is_watched_and_then_compiles`],
    /// and the case the unconditional section list in `document::Render::assets`
    /// exists for. Both shopping lists fail with `MissingSection` here, and
    /// [`Preview::compile`] replaces the list only when it is `Some` — so an app
    /// that published the path out of a successful read would leave the list
    /// empty, `watch::classify` would drop the creation event, and the window
    /// would never recover on its own.
    #[test]
    fn a_section_that_does_not_exist_yet_is_watched_and_then_compiles() {
        let dir = scratch_dir("section-to-come");
        let document = dir.join("report.md");
        std::fs::write(&document, "[](sections/one.md)\n").unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        let error = session.preview().error().unwrap().to_string();
        assert!(error.contains("sections/one.md"), "{error}");
        assert!(error.contains("for the section"), "{error}");
        assert!(error.contains("at line 1"), "{error}");
        assert!(session.preview().pdf().is_none());

        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(dir.join("sections/one.md"), "# One\n\nText.\n").unwrap();

        assert!(wait_for(&compiles, 2) >= 2, "creating the section compiles");
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

    /// **The pane's text is what compiles, and the file beside it need never
    /// have held that text.**
    ///
    /// This is the whole of what Phase 4 changed one layer down, and the file
    /// is asserted to be untouched so that a compile which quietly went back
    /// to reading the disk could not pass.
    #[test]
    fn the_pane_compiles_text_that_is_not_on_disk() {
        let dir = scratch_dir("buffer-compiles");
        let document = article_in(&dir);

        let mut preview = compiled(&document);
        let from_disk = preview.pdf().unwrap().to_vec();

        let typed = "# Typed, never saved\n\nThis text is in the pane and nowhere else.\n";
        preview.edit(typed.to_string());
        preview.compile();

        assert_eq!(
            std::fs::read_to_string(&document).unwrap(),
            std::fs::read_to_string(sample("article.md")).unwrap(),
            "the file moved, so this proves nothing about the buffer"
        );
        assert_ne!(preview.pdf().unwrap(), from_disk.as_slice());
        assert_eq!(
            preview.pdf().unwrap(),
            md2pdf_core::md_to_pdf(typed, &[]).unwrap()
        );
    }

    /// A save writes the buffer, and **the save's own event compiles nothing**
    /// — the rule's first outcome, reached by comparing content rather than by
    /// winning a race against a 12 ms event.
    ///
    /// The figure at the end is the half that proves the filter narrowed
    /// rather than stopped: an implementer who dropped the watch while the
    /// pane owns the document passes everything above it and fails here.
    #[test]
    fn a_save_writes_the_buffer_and_the_loop_compiles_no_second_time() {
        let dir = scratch_dir("save");
        let document = article_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");

        let typed = format!(
            "{}\nA paragraph the file has never held.\n",
            std::fs::read_to_string(&document).unwrap()
        );
        session.edit(typed.clone());
        assert_eq!(wait_for(&compiles, 2), 2, "a pause in the typing compiles");

        session.save().unwrap();
        assert_eq!(std::fs::read_to_string(&document).unwrap(), typed);

        settle();
        assert_eq!(
            compiles.load(Ordering::SeqCst),
            2,
            "the save's own event compiled a second time"
        );

        std::fs::write(
            dir.join("pipeline.svg"),
            std::fs::read(sample("check.svg")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            wait_for(&compiles, 3),
            3,
            "a figure no longer redraws the page"
        );
    }

    /// The first outcome: the file already says what the pane says.
    ///
    /// This is the app's own save arriving back, and **nothing at all
    /// happens** — which the whole status is compared to prove, because a
    /// compile here would redraw a page the reader has scrolled.
    #[test]
    fn an_external_change_matching_the_buffer_does_nothing() {
        let dir = scratch_dir("external-unchanged");
        let document = article_in(&dir);

        let mut preview = compiled(&document);
        let before = preview.status();

        std::fs::write(&document, preview.text()).unwrap();
        assert_eq!(preview.reload(), External::Unchanged);

        assert_eq!(preview.status(), before);
    }

    /// The second outcome: the buffer is clean, so nothing can be lost.
    ///
    /// **This is Phase 2's shipped loop**, and it is the case an unconditional
    /// refusal would have broken — an author who is not typing saves in
    /// another editor and the page redraws, with no action at the window.
    #[test]
    fn an_external_change_over_a_clean_buffer_is_taken_and_recompiled() {
        let dir = scratch_dir("external-taken");
        let document = article_in(&dir);

        let mut preview = compiled(&document);
        let first = preview.pdf().unwrap().to_vec();
        let before = preview.status();

        let theirs = "# Edited elsewhere\n\nBy another program, while nobody typed.\n";
        std::fs::write(&document, theirs).unwrap();
        assert_eq!(preview.reload(), External::Taken);

        assert_eq!(preview.text(), theirs);
        assert_ne!(preview.pdf().unwrap(), first.as_slice());

        let status = preview.status();
        assert_eq!(status.state, State::Current);
        assert_eq!(status.divergence, None);
        assert!(status.revision > before.revision, "it did not recompile");
        assert!(
            status.reloaded > before.reloaded,
            "it did not take the text"
        );
    }

    /// The third outcome: the buffer holds unsaved edits, so the disk copy is
    /// refused and the divergence is named.
    ///
    /// An implementer who tests only this one ships a pane that stops redrawing
    /// on an external save. It is here as one of three for that reason.
    #[test]
    fn an_external_change_over_a_dirty_buffer_is_refused_and_reported() {
        let dir = scratch_dir("external-diverged");
        let document = article_in(&dir);

        let mut preview = compiled(&document);
        preview.edit("# Mine, unsaved\n\nStill being written.\n".to_string());
        preview.compile();

        let mine = preview.pdf().unwrap().to_vec();
        let before = preview.status();

        std::fs::write(&document, "# Theirs\n\nWritten by another program.\n").unwrap();
        assert_eq!(preview.reload(), External::Diverged);

        assert_eq!(preview.text(), "# Mine, unsaved\n\nStill being written.\n");
        assert_eq!(preview.pdf().unwrap(), mine.as_slice());

        let status = preview.status();
        assert_eq!(status.revision, before.revision, "it compiled anyway");
        assert_eq!(status.reloaded, before.reloaded, "it took the disk copy");
        // A divergence is not staleness: nothing failed to compile, and the
        // page belongs to the text in the pane.
        assert_eq!(status.state, State::Current);
        assert!(!preview.is_stale());
        assert!(
            status.divergence.as_deref().unwrap().contains("unsaved"),
            "{:?}",
            status.divergence
        );
    }

    /// A document opened, edited, saved and reopened round-trips byte for byte
    /// **against the buffer at save**, which an edit has already made unequal
    /// to the original.
    ///
    /// The text carries a CRLF and no trailing newline, which are the two a
    /// text pane is likeliest to normalise away.
    #[test]
    fn an_edited_document_round_trips_byte_for_byte_through_a_save() {
        let dir = scratch_dir("round-trip");
        let document = article_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        let typed = "# Edited\r\n\r\nA line under a CRLF, and no newline at the end.";
        session.edit(typed.to_string());
        session.save().unwrap();

        assert_eq!(std::fs::read(&document).unwrap(), typed.as_bytes());

        session.open(document).unwrap();
        assert_eq!(session.preview().text(), typed);
    }

    /// The export writes the page itself, and the page is what the core crate
    /// makes of this document.
    ///
    /// This is one half of the byte-identity claim; the other half lives in
    /// `cli/tests/cli_test.rs`, because `CARGO_BIN_EXE_md2pdf` reaches only
    /// integration tests of the package that defines that binary, and nothing
    /// in `app/src/` is importable from there. The middle leg — the in-test
    /// `md_to_pdf` call — is what composes them.
    #[test]
    fn the_export_writes_the_bytes_the_pane_is_showing() {
        let dir = scratch_dir("export");
        let document = article_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document.clone()).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        // The default path is `cli/src/main.rs:default_output`'s rule.
        let output = document.with_extension("pdf");
        assert_eq!(session.preview().export_path().unwrap(), output);

        session.preview().export(&output).unwrap();
        let written = std::fs::read(&output).unwrap();
        assert_eq!(written, session.preview().pdf().unwrap());

        // Without this assertion the two halves of the gate would meet only
        // through a reading of the two asset readers, and a later divergence
        // in either would pass both while the wrappers disagreed.
        let markdown = std::fs::read_to_string(&document).unwrap();
        let expected = md2pdf_core::md_to_pdf(&markdown, &article_assets(&dir)).unwrap();
        assert_eq!(written, expected);

        // Nothing recompiled: not for the export, and not for the PDF it left
        // in the very directory the loop watches.
        settle();
        assert_eq!(compiles.load(Ordering::SeqCst), 1);
    }

    /// The first of two refusals: the bytes exist and are known to be old.
    #[test]
    fn export_is_refused_while_the_pane_is_stale() {
        let dir = scratch_dir("export-stale");
        let document = article_in(&dir);

        let mut preview = compiled(&document);
        assert_eq!(preview.state(), State::Current);

        preview.edit("# Broken\n\n<div>raw HTML</div>\n".to_string());
        preview.compile();
        assert_eq!(preview.state(), State::Stale);

        let output = dir.join("refused.pdf");
        let refusal = preview.export(&output).unwrap_err();
        assert!(refusal.contains("out of date"), "{refusal}");
        assert!(preview.export_path().is_err(), "the dialog would open");
        assert!(!output.exists(), "a refused export wrote a file");
    }

    /// The second refusal is not the first. `Preview::default()` has the stale
    /// mark clear and no bytes at all, and an implementer who tests only the
    /// case above leaves the launch state to panic or to write nothing.
    #[test]
    fn export_is_refused_while_no_document_is_open() {
        let dir = scratch_dir("export-empty");
        let preview = Preview::default();
        assert_eq!(preview.state(), State::Empty);

        let output = dir.join("refused.pdf");
        let refusal = preview.export(&output).unwrap_err();
        assert!(refusal.contains("no document is open"), "{refusal}");
        assert!(preview.export_path().is_err(), "the dialog would open");
        assert!(!output.exists(), "a refused export wrote a file");
    }

    /// The state the app launches into and holds until the first Open.
    #[test]
    fn the_empty_status_names_no_document_and_no_time() {
        let status = Preview::default().status();

        assert_eq!(status.state, State::Empty);
        assert_eq!(status.time, None);
        assert_eq!(status.error, None);
        assert!(!status.page);
    }

    #[test]
    fn the_current_status_names_the_compile_time() {
        let dir = scratch_dir("status-current");
        let status = compiled(&article_in(&dir)).status();

        assert_eq!(status.state, State::Current);
        assert_eq!(status.error, None);
        assert!(status.page);
        assert!(
            status.time.as_deref().unwrap().ends_with(" ms"),
            "{:?}",
            status.time
        );
    }

    #[test]
    fn the_stale_status_names_the_error_and_keeps_the_page() {
        let dir = scratch_dir("status-stale");
        let mut preview = compiled(&article_in(&dir));

        preview.edit("# Broken\n\n<div>raw HTML</div>\n".to_string());
        preview.compile();
        let status = preview.status();

        assert_eq!(status.state, State::Stale);
        assert!(status.page);
        assert!(
            status.error.as_deref().unwrap().contains("raw HTML block"),
            "{:?}",
            status.error
        );
        // The time belongs to the page still drawn, not to the attempt that
        // failed, because the duration travels with the bytes.
        assert!(status.time.is_some());
    }

    #[test]
    fn the_failed_status_names_the_error_and_has_no_page() {
        let dir = scratch_dir("status-failed");
        let document = dir.join("paper.md");
        std::fs::write(&document, "# Paper\n\n![a mark to come](figures/new.svg)\n").unwrap();

        let status = compiled(&document).status();

        assert_eq!(status.state, State::Failed);
        assert!(!status.page);
        assert_eq!(status.time, None);
        assert!(
            status.error.as_deref().unwrap().contains("figures/new.svg"),
            "{:?}",
            status.error
        );
    }

    /// A master that stops naming sections stops having parts.
    ///
    /// **This belongs at `Preview::compile` and nowhere else.**
    /// `document::render_with` is stateless, so at *that* surface a master
    /// whose markers were just deleted and a document that never had one are
    /// the same call, and the assertion would hold however this list were kept.
    /// It is [`Preview::sections`] — retained across compiles, replaced
    /// unconditionally — that the question was ever about: under a rule that
    /// kept the last non-empty answer, a master whose markers are genuinely
    /// deleted would hold a phantom panel for the life of the open document,
    /// because no later compile could restore an empty list.
    ///
    /// `master` rides along, being the other field the panel reads and the
    /// other one nothing else asserts. It is a file *name* and not a path,
    /// because the panel lists one document's parts and where that document
    /// sits is the window title's business.
    #[test]
    fn a_master_that_stops_naming_sections_loses_its_parts() {
        let dir = scratch_dir("parts-deleted");
        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(dir.join("sections/one.md"), "# One\n\nText.\n").unwrap();
        std::fs::write(dir.join("sections/two.md"), "# Two\n\nText.\n").unwrap();
        let document = dir.join("report.md");
        std::fs::write(&document, "[](sections/one.md)\n\n[](sections/two.md)\n").unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");

        let opened = session.preview().status();
        assert_eq!(opened.master.as_deref(), Some("report.md"));
        assert_eq!(opened.sections, ["sections/one.md", "sections/two.md"]);

        session.edit("# A master that names nothing now.\n".to_string());
        assert_eq!(wait_for(&compiles, 2), 2, "a pause in the typing compiles");

        let after = session.preview().status();
        assert!(
            after.sections.is_empty(),
            "the deleted markers left rows behind: {:?}",
            after.sections
        );
        assert_eq!(
            after.master.as_deref(),
            Some("report.md"),
            "the document is still the one that is open"
        );
    }

    /// The `@property` names of one `@typedef {object} …` block in the page.
    ///
    /// Plain string operations, and no HTML parser enters this crate for it:
    /// the markers are fixed literals and the block ends at the first `*/`
    /// after one. The type is skipped by brace depth rather than by the first
    /// `}`, so an inline object type would not silently truncate the name.
    fn typedef_properties(page: &str, name: &str) -> Vec<String> {
        let marker = format!("@typedef {{object}} {name}\n");
        let at = page
            .find(&marker)
            .unwrap_or_else(|| panic!("the page declares no `@typedef {{object}} {name}`"));
        let block = &page[at + marker.len()..];
        let block = &block[..block.find("*/").expect("an unterminated JSDoc block")];

        block
            .lines()
            .filter_map(|line| {
                let rest = line.trim_start().strip_prefix("* @property ")?;
                let rest = rest.strip_prefix('{')?;
                let mut depth = 1usize;
                let close = rest.char_indices().find_map(|(at, glyph)| {
                    match glyph {
                        '{' => depth += 1,
                        '}' => {
                            depth -= 1;
                            if depth == 0 {
                                return Some(at);
                            }
                        }
                        _ => {}
                    }
                    None
                })?;
                Some(rest[close + 1..].split_whitespace().next()?.to_string())
            })
            .collect()
    }

    /// The page's typedefs and this crate's `Status` name the same fields.
    ///
    /// **This is the narrow edge `specs/desktop_app_spec.md` OQ-10 names.**
    /// `invoke` answers with an untyped value and `app/dist/index.html` reads
    /// ten fields off it by name, so a field renamed here and not there breaks
    /// the window silently, at runtime, with no console anyone reads. The type
    /// check over that file (`app/typecheck.mjs`) makes the typedef bind on the
    /// page's side; this makes it bind on Rust's. **Two declarations compared
    /// against each other**, rather than usage compared against a declaration.
    ///
    /// `anchors` holds one, and must: `Anchor` is not reachable in the JSON at
    /// all while the list is empty.
    #[test]
    fn the_page_typedefs_name_exactly_the_fields_status_serializes() {
        const PAGE: &str = include_str!("../dist/index.html");

        let status = Status {
            state: State::Current,
            time: Some("28 ms".to_string()),
            error: None,
            page: true,
            divergence: None,
            revision: 3,
            reloaded: 1,
            anchors: vec![document::Anchor { line: 1, page: 1 }],
            sections: vec!["sections/one.md".to_string()],
            master: Some("report.md".to_string()),
        };
        let sent = serde_json::to_value(&status).expect("a `Status` that will not serialize");

        let mut carried: Vec<String> = sent
            .as_object()
            .expect("a `Status` that is not a JSON object")
            .keys()
            .cloned()
            .collect();
        let mut declared = typedef_properties(PAGE, "Status");
        carried.sort_unstable();
        declared.sort_unstable();
        // Both counts, not just the equality: two empty lists are equal, and a
        // marker that stopped matching should fail loudly rather than pass.
        assert_eq!(
            declared.len(),
            10,
            "the page's `Status` typedef declares {} properties: {:?}",
            declared.len(),
            declared
        );
        assert_eq!(
            declared, carried,
            "the page's `Status` typedef and the serialized `Status` name different fields"
        );

        let mut riding: Vec<String> = sent["anchors"][0]
            .as_object()
            .expect("an `Anchor` that is not a JSON object")
            .keys()
            .cloned()
            .collect();
        let mut named = typedef_properties(PAGE, "Anchor");
        riding.sort_unstable();
        named.sort_unstable();
        assert_eq!(
            named.len(),
            2,
            "the page's `Anchor` typedef declares {} properties: {:?}",
            named.len(),
            named
        );
        assert_eq!(
            named, riding,
            "the page's `Anchor` typedef and the anchors a `Status` carries name different fields"
        );
    }
}
