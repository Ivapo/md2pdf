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

/// The report a refused *switch* leaves for the author.
///
/// **Its own sentence, and not [`DIVERGED`]'s.** That one opens *"this file
/// changed on disk"*, which is false on this occasion — nothing moved, the
/// author asked to put another file in the pane — so reusing the constant would
/// put a lie in the window. The shape is the same and deliberately so: name both
/// ways out, and take neither.
///
/// Both ride `Preview::divergence`, whose meaning is therefore *a refused
/// change* rather than *a refused external change*. **One field means one
/// occasion at a time**: a switch refused while a real divergence stands
/// overwrites this sentence and is overwritten by the next, which costs nothing
/// — the two name the same two exits, and both are cleared by the same two
/// actions.
const SWITCHING: &str = "the pane holds unsaved edits, so it is still holding this file. \
    Save to keep them, or discard them to open the other file.";

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
    /// The project's files, in the order the panel draws them.
    ///
    /// It rides the status for the reason the anchors do: the status is already
    /// fetched on the path that draws, so the panel needs no command of its own.
    /// Empty exactly when no document is open.
    pub entries: Vec<document::Entry>,
    /// Which of them compiles, root-relative with `/` separators.
    ///
    /// **Spelled the way an entry is**, and not as the bare file *name* this
    /// field carried while the panel listed one document's parts — the page has
    /// to match it against a row to mark it, and two files of that name in
    /// different folders must not both light up.
    pub main: Option<String>,
    /// Which of them the pane is holding, spelled the same way.
    ///
    /// It rides beside [`Status::main`] because **the page cannot derive one
    /// from the other**: they are equal at every open and differ the moment a
    /// row is clicked, and the panel draws a mark for each. Equal to `main`
    /// exactly while the pane holds the file that compiles.
    pub edited: Option<String>,
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
    /// What the panel lists, and what the watch is rooted at.
    ///
    /// **It does not move when a row is clicked.** [`Session::open`] re-roots
    /// the watch on every open, so a click that re-rooted would strand the
    /// author below their own project with no way back up. The root changes on
    /// an explicit Open and at no other time.
    root: Option<PathBuf>,
    /// Which file under the root compiles, root-relative with `/` separators.
    main: Option<String>,
    /// Which file the pane holds and `⌘S` writes.
    ///
    /// Equal to [`Preview::main`] resolved against the root at every open, and
    /// free to differ from it from the first row click on. **`main` is what
    /// compiles and this is what is edited**: [`Preview::compile`] reads the
    /// first and [`Preview::save`], [`Preview::load`] and [`Preview::reload`]
    /// the second.
    edited: Option<PathBuf>,
    /// The files under the root, as the last walk of the disk found them.
    ///
    /// **Refreshed on two occasions only** — an open, and a `Change::Tree`
    /// event — and never recomputed in [`Preview::status`], which the page calls
    /// on every render and which would then walk the disk on every keystroke.
    /// The marked-missing rows are not in here: they come off the text in
    /// [`Preview::status`], which is why the disk half of the panel is stable
    /// and only the missing half moves while a marker is half-typed.
    tree: Vec<document::Entry>,
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
        self.edited.as_deref()
    }

    /// The project the panel is listing, if one is open.
    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    /// The file that compiles, as a path.
    ///
    /// [`Preview::main`] is root-relative because that is the spelling the
    /// panel needs; every reader inside this file wants it joined back on.
    fn main_path(&self) -> Option<PathBuf> {
        Some(self.root.as_ref()?.join(self.main.as_ref()?))
    }

    /// The file the pane holds, spelled the way [`Preview::main`] is.
    ///
    /// **Textual, off no disk.** Both paths were built here by joining onto one
    /// root, so there is nothing to canonicalize, and [`Preview::status`] calls
    /// this on every render.
    fn edited_relative(&self) -> Option<String> {
        document::spell(self.root.as_deref()?, self.edited.as_deref()?)
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
        match (self.edited.is_some(), self.stale, self.pdf.is_some()) {
            (false, _, _) => State::Empty,
            (true, false, true) => State::Current,
            (true, true, true) => State::Stale,
            (true, _, false) => State::Failed,
        }
    }

    /// Everything the window says about the last compile, in one value.
    ///
    /// **The panel's two halves are put together here and nothing is read off
    /// the disk.** [`Preview::tree`] is the walk, taken at an open and at a
    /// `Change::Tree` event; the marked-missing rows come off `sections`, which
    /// every compile assigns from the master's own text. So a keystroke that
    /// half-types a marker moves one row and walks no directory, which is what
    /// makes this cheap enough to call on every render.
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
            entries: self.entries(),
            main: self.main.clone(),
            edited: self.edited_relative(),
        }
    }

    /// The panel's rows: the disk walk, plus the sections the master names that
    /// the walk did not find.
    fn entries(&self) -> Vec<document::Entry> {
        let Some(main) = self.main.as_deref() else {
            return Vec::new();
        };
        let named: Vec<String> = self
            .sections
            .iter()
            .map(|section| document::beside(main, section))
            .collect();
        document::merge(self.tree.clone(), &named)
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
    ///
    /// **It names the file that compiles and not the file in the pane**, which
    /// are two different files since `mpdf-010` Phase 2. The bytes it offers to
    /// write are the master's, so `showcase.pdf` is the honest default where
    /// `mathematics.pdf` would name a section for a PDF holding the whole book.
    pub fn export_path(&self) -> Result<PathBuf, String> {
        self.exportable()?;
        self.main_path()
            .map(|main| document::default_output(&main))
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
        if self.edited.is_some() {
            self.buffer = text;
        }
    }

    /// Read the document from disk into the buffer, and compile it.
    ///
    /// A file that will not read leaves the same message and the same *failed*
    /// state a compile failure leaves, because that is what the author needs
    /// to see either way and it is the sentence the terminal prints.
    pub fn load(&mut self) {
        let Some(document) = self.edited.clone() else {
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
            .edited
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
        let Some(document) = self.edited.clone() else {
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
    ///
    /// **It compiles [`Preview::main`], not the file in the pane.** The pane's
    /// text reaches it through the closure `document::render_project` builds,
    /// which answers `edited` from this buffer and everything else from the
    /// disk — so the page shows the whole document while the author edits one
    /// file of it, and shows exactly what the pane says while the two are the
    /// same file. A `main` this app cannot read at all leaves the message and
    /// the *failed* state [`Preview::load`] leaves, which is where a document
    /// that will not read has always landed.
    pub fn compile(&mut self) {
        let (Some(main), Some(edited)) = (self.main_path(), self.edited.clone()) else {
            return;
        };

        let started = Instant::now();
        let render = match document::render_project(&main, &edited, &self.buffer) {
            Ok(render) => render,
            Err(message) => {
                self.stale = true;
                self.error = Some(message);
                return;
            }
        };
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
    /// The one file this app writes outside the author's own folders: which
    /// root is remembered as compiling which file.
    ///
    /// **It is a parameter and not a call to the platform**, so a test hands in
    /// a scratch directory and the rule that reads it stays on the testable
    /// side of the window. `crate::main` resolves the real one from Tauri's own
    /// path resolver, which is the authority on the bundle identifier.
    store: PathBuf,
    watch: Option<Watch>,
    typing: Option<mpsc::Sender<()>>,
}

impl Session {
    /// A session that calls `on_render` after every compile, its own included.
    ///
    /// The callback carries no payload. The window's copy of it emits an event
    /// and the page then asks for the bytes, because handing them through the
    /// event would serialize them as a JSON array of numbers, one per byte.
    pub fn new(store: PathBuf, on_render: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            state: Arc::new(Mutex::new(Preview::default())),
            on_render: Arc::new(on_render),
            store,
            watch: None,
            typing: None,
        }
    }

    /// What the pane is showing. Phase 3's export reads the same values.
    pub fn preview(&self) -> std::sync::MutexGuard<'_, Preview> {
        self.state.lock().expect("the preview lock was poisoned")
    }

    /// Open what the author picked: find the project it sits in, read the file
    /// that project compiles, and watch the whole of it from now on.
    ///
    /// **This opens the file it landed on rather than the file it was handed.**
    /// The root is `crate::document::project_root`'s one-level climb, so a
    /// double-click on a section finds the master above it; the main is the
    /// store's answer for that root when it has one, and
    /// `crate::document::discover_main` when it does not.
    ///
    /// **The store is read first, on every open**, or it is a thing written and
    /// never used. An override naming a file the disk no longer holds falls
    /// through to discovery rather than opening nothing.
    ///
    /// The previous document's page and text go with it. A page kept across an
    /// open would belong to a file the window no longer names.
    pub fn open(&mut self, opened: PathBuf) -> Result<(), String> {
        let root = document::project_root(&opened);
        let main = document::read_override(&self.store, &root)
            .filter(|main| root.join(main).is_file())
            .unwrap_or_else(|| document::discover_main(&root, &opened));

        self.open_at(root, main)
    }

    /// The open, once the root and the main are decided.
    ///
    /// Shared with [`Session::set_main`], so the store and the window can never
    /// disagree about which file compiles: there is one path that puts a
    /// document in the pane, and both callers take it.
    fn open_at(&mut self, root: PathBuf, main: String) -> Result<(), String> {
        let document = root.join(&main);

        {
            let mut preview = self.preview();
            *preview = Preview {
                root: Some(root.clone()),
                main: Some(main),
                edited: Some(document.clone()),
                tree: document::files_under(&root),
                ..Preview::default()
            };
            preview.load();
        }
        (self.on_render)();
        self.arm(root, document.clone(), document)
    }

    /// Point both loops at these two files, dropping whatever they held.
    ///
    /// **Shared with [`Session::set_edited`], which is not an open.** Both
    /// closures guard on `Preview::edited` against a path captured when they
    /// were started, so a command that moved the pane and stopped there would
    /// leave the typing debounce compiling nothing and every filesystem event
    /// dropped. The guard itself stays: it is what stops a thread mid-compile
    /// from writing its page over a newer one.
    ///
    /// The old loops go before the new ones start, so no two of them ever hold
    /// the same document.
    fn arm(&mut self, root: PathBuf, main: PathBuf, edited: PathBuf) -> Result<(), String> {
        self.watch = None;
        self.typing = None;

        self.typing = Some(watch::debounced(
            watch::TYPING_DEBOUNCE,
            self.recompile(edited.clone()),
        ));
        let classify = self.classifier(root.clone(), main, edited.clone());
        let on_change = self.on_change(root.clone(), edited);
        self.watch = Some(watch::start(&root, watch::DEBOUNCE, classify, on_change)?);

        Ok(())
    }

    /// Set which file under the open root compiles, and remember it.
    ///
    /// The store is written *before* the open, so a window that opened and then
    /// failed to remember cannot happen: the fact is on disk or the author is
    /// told why it is not.
    pub fn set_main(&mut self, main: String) -> Result<(), String> {
        let root = self
            .preview()
            .root()
            .map(Path::to_path_buf)
            .ok_or_else(|| "no document is open".to_string())?;

        // **Confined, and not merely checked for existence.** The path comes
        // from the panel, which got it from this app's own listing — but a
        // command is a command, and `root.join("../../secrets.md")` names a
        // real file on plenty of machines. `document::confined` is the walk's
        // own test, shared with `set_edited` and with the figure read.
        if document::confined(&root, &main).is_none() {
            return Err(format!("{main} is not a file in this project"));
        }

        if self.refused_while_dirty() {
            return Ok(());
        }

        document::write_override(&self.store, &root, &main)?;
        self.open_at(root, main)
    }

    /// Put another of the project's files in the pane, leaving the main alone.
    ///
    /// **This is not an open, and the difference is the counters.**
    /// [`Session::open_at`] assigns `Preview { ..Preview::default() }`, which
    /// zeroes `revision` and `reloaded`, and `app/dist/index.html`'s `clear()`
    /// — which resets the counters the page compares them against — runs on an
    /// Open and not on a row click. So this sets `edited`, reads that file into
    /// the buffer, and leaves the root, the main, the listing, the bytes and
    /// both counters exactly as it found them: they *advance* here, they do not
    /// restart.
    ///
    /// It confines the path as [`Session::set_main`] does, and refuses on the
    /// same terms while the buffer diverges from the last-saved text.
    pub fn set_edited(&mut self, path: String) -> Result<(), String> {
        let (root, main) = {
            let preview = self.preview();
            match (preview.root.clone(), preview.main.clone()) {
                (Some(root), Some(main)) => (root, main),
                _ => return Err("no document is open".to_string()),
            }
        };

        let Some(landed) = document::confined(&root, &path) else {
            return Err(format!("{path} is not a file in this project"));
        };

        if self.refused_while_dirty() {
            return Ok(());
        }

        {
            let mut preview = self.preview();
            preview.edited = Some(landed.clone());
            preview.load();
        }
        (self.on_render)();

        self.arm(root.clone(), root.join(main), landed)
    }

    /// Drop what the pane holds and take the file again.
    ///
    /// **The second way out both refusals name.** `Preview::load` already reads
    /// the edited file into the buffer and the last-saved text together, which
    /// is exactly "discard"; this is that path behind a command. It clears the
    /// divergence through `Preview::take`, so one action answers a refused
    /// switch and a refused external change alike.
    pub fn discard(&self) {
        self.preview().load();
        (self.on_render)();
    }

    /// Is there unsaved work a switch would throw away? Then say so and stop.
    ///
    /// **It reports through `Preview::divergence` and not through an `Err`.**
    /// The caller's `Err` is where a path outside the project goes, and the
    /// window draws that in the error bar; a refusal that arrived both ways
    /// would be one problem in two places. This is a status, and the page places
    /// it exactly as it places every other status sentence.
    ///
    /// It announces, because nothing else will: no compile ran, so the page
    /// would otherwise never fetch the status carrying the sentence.
    fn refused_while_dirty(&self) -> bool {
        {
            let mut preview = self.preview();
            if preview.buffer == preview.saved {
                return false;
            }
            preview.divergence = Some(SWITCHING.to_string());
        }
        (self.on_render)();
        true
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
    fn classifier(
        &self,
        root: PathBuf,
        main: PathBuf,
        edited: PathBuf,
    ) -> impl Fn(&Path) -> Option<Change> + Send + 'static {
        let state = Arc::clone(&self.state);
        move |path| {
            let assets = state
                .lock()
                .expect("the preview lock was poisoned")
                .assets
                .clone();
            watch::classify(path, &root, &main, &edited, &assets)
        }
    }

    /// What one settled window of filesystem events does.
    ///
    /// **The edited file and everything else reach different code.** The file
    /// the pane holds runs [`Preview::reload`], because the pane's own buffer is
    /// what stands against it and losing that buffer is the one thing this app
    /// refuses to do quietly. Everything else the compile reads — an asset, and
    /// since `mpdf-010` Phase 2 the master itself — is a bare recompile, because
    /// nothing but the disk supplies it. `Change::Edited` is decided before
    /// both, so while the pane holds the main an event still runs the rule,
    /// exactly as it did before the two could differ.
    ///
    /// A window that took the disk copy compiled inside the rule, and it read
    /// the new assets on the way, so the two never compile twice for one
    /// window. And nothing is announced when nothing happened: the app's own
    /// save arrives here, changes nothing, and must not redraw a frame the
    /// reader has scrolled.
    fn on_change(&self, root: PathBuf, edited: PathBuf) -> impl FnMut(Changed) + Send + 'static {
        let state = Arc::clone(&self.state);
        let on_render = Arc::clone(&self.on_render);

        move |changed: Changed| {
            let mut announce = false;
            {
                let mut preview = state.lock().expect("the preview lock was poisoned");
                if preview.edited.as_deref() != Some(edited.as_path()) {
                    return;
                }

                let taken = if changed.edited {
                    let outcome = preview.reload();
                    announce = outcome != External::Unchanged;
                    outcome == External::Taken
                } else {
                    false
                };

                if (changed.document || changed.assets) && !taken {
                    preview.compile();
                    announce = true;
                }

                // **A file the document does not name moved: the panel is out
                // of date and the page is not.** This walks the disk and stops
                // — no compile, so `revision` stands still and the page draws
                // nothing again. It is announced all the same, because the
                // panel is drawn off the status the announcement fetches.
                if changed.tree {
                    preview.tree = document::files_under(&root);
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
                if preview.edited.as_deref() != Some(document.as_path()) {
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
    ///
    /// Its store is a scratch file this process owns, so a test never reads or
    /// writes the store the installed app keeps — and a case that wants an
    /// override writes one into it and says so.
    fn counted() -> (Session, Arc<AtomicUsize>) {
        counted_with(document::store_file(&scratch_dir("store-of-the-session")))
    }

    /// The same, with the store named, for the cases that put something in it.
    fn counted_with(store: PathBuf) -> (Session, Arc<AtomicUsize>) {
        let compiles = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&compiles);
        let session = Session::new(store, move || {
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
    ///
    /// The root is the document's own directory and the main is the document,
    /// which is what a single-file open lands on and is what every case using
    /// this is about.
    fn compiled(document: &Path) -> Preview {
        let root = watch::root(document);
        let name = document::title(document);
        let mut preview = Preview {
            root: Some(root.clone()),
            main: Some(name),
            edited: Some(document.to_path_buf()),
            tree: document::files_under(&root),
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

        // **Nothing recompiled**, not for the export and not for the PDF it
        // left in the very directory the loop watches — `revision` is what
        // "recompiled" means, and it has not moved.
        //
        // The *announcement* did happen, and that is `mpdf-010` Phase 1's
        // `Change::Tree` arriving on a real filesystem rather than in a
        // constructed case: a file appeared in the project, so the panel is out
        // of date and the page is fetched again to redraw it. The counter
        // beside `revision` is what tells the two apart.
        settle();
        let after = session.preview().status();
        assert_eq!(
            after.revision, 1,
            "a file appearing in the project compiled"
        );
        assert!(
            compiles.load(Ordering::SeqCst) > 1,
            "the panel was never told the export had landed"
        );

        // And it is listed, PDF and all. `pdf` is in
        // `md2pdf_core::IMAGE_EXTENSIONS` because a PDF is a legal figure in
        // this dialect, so a document's own export appears beside its figures.
        // That is `specs/file_panel_spec.md` OQ-3, pinned rather than
        // special-cased on a guess.
        assert!(
            after
                .entries
                .iter()
                .any(|entry| entry.path == "article.pdf" && entry.kind == document::Kind::Image),
            "the exported PDF is not in the panel: {:?}",
            after.entries.iter().map(|e| &e.path).collect::<Vec<_>>()
        );
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
    /// **The disk half does not move with it, and that is the point.** Before
    /// `mpdf-010` the whole panel was this list, so deleting a marker emptied
    /// the panel; now the files are still on the disk and still listed, and
    /// what the deletion costs is the one marked-missing row. That is the
    /// flicker `mpdf-008` §2 accepted, reduced to the half that has to move.
    ///
    /// `main` rides along, being the other field the panel reads and the other
    /// one nothing else asserts. It is root-relative and not a bare file name,
    /// because the page matches it against a row.
    #[test]
    fn a_master_that_stops_naming_sections_loses_its_marked_missing_rows() {
        let dir = scratch_dir("parts-deleted");
        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(dir.join("sections/one.md"), "# One\n\nText.\n").unwrap();
        let document = dir.join("report.md");
        std::fs::write(&document, "[](sections/one.md)\n\n[](sections/two.md)\n").unwrap();

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");

        let opened = session.preview().status();
        assert_eq!(opened.main.as_deref(), Some("report.md"));
        assert_eq!(
            opened
                .entries
                .iter()
                .map(|entry| (entry.path.as_str(), entry.missing))
                .collect::<Vec<_>>(),
            [
                ("report.md", false),
                ("sections/one.md", false),
                ("sections/two.md", true),
            ],
            "the section that is named and absent is the row the author needs"
        );

        session.edit("# A master that names nothing now.\n".to_string());
        assert_eq!(wait_for(&compiles, 2), 2, "a pause in the typing compiles");

        let after = session.preview().status();
        assert_eq!(
            after
                .entries
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>(),
            ["report.md", "sections/one.md"],
            "the deleted marker left its row behind"
        );
        assert!(
            after.entries.iter().all(|entry| !entry.missing),
            "nothing is named and absent once the markers are gone"
        );
        assert_eq!(
            after.main.as_deref(),
            Some("report.md"),
            "the document is still the one that is open"
        );
    }

    // -- the project, at the session -------------------------------------
    //
    // `mpdf-010` Phase 1's exit gate, clauses 3 and 7. The pieces they are
    // built from are pinned in `crate::document`'s own tests; these are the
    // claims about what the window does with them.

    /// **Clause 3, and the phase's whole observable.**
    ///
    /// Opening a section from Finder compiles the master above it. Before this
    /// phase the same double-click compiled that one section standalone, so the
    /// two openings produce the same PDF now and produced different ones then —
    /// which is the difference a reader of this test is meant to see.
    ///
    /// The bytes are compared rather than the paths, and `is_some` is asserted
    /// beside them: two failures are equal too, and a fixture that stopped
    /// compiling would otherwise leave this passing and proving nothing.
    #[test]
    fn opening_a_section_compiles_the_master_that_names_it() {
        let panel = fixture("panel");
        let store = document::store_file(&scratch_dir("nothing-remembered"));

        let (mut session, compiles) = counted_with(store.clone());
        session.open(panel.join("sections/text.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1, "opening compiles once");

        let (main, edited, bytes) = {
            let preview = session.preview();
            (
                preview.status().main,
                preview.document().map(Path::to_path_buf),
                preview.pdf().map(<[u8]>::to_vec),
            )
        };
        assert_eq!(main.as_deref(), Some("book.md"));
        assert_eq!(
            edited,
            Some(panel.join("book.md")),
            "the pane holds the main, which is where Phase 1 leaves the two"
        );

        let (mut direct, direct_compiles) = counted_with(store);
        direct.open(panel.join("book.md")).unwrap();
        assert_eq!(wait_for(&direct_compiles, 1), 1);

        assert!(
            bytes.is_some(),
            "the fixture master did not compile, so the comparison below is vacuous"
        );
        assert_eq!(
            bytes,
            direct.preview().pdf().map(<[u8]>::to_vec),
            "opening the section and opening the master produced different pages"
        );
    }

    /// Clause 3's second half: the store is read first, and discovery is not
    /// consulted when it answers.
    ///
    /// `other.md` names no section, so discovery would never land on it — which
    /// is what makes this case tell the two apart.
    #[test]
    fn a_stored_override_decides_which_file_compiles() {
        let panel = fixture("panel");
        let store = document::store_file(&scratch_dir("override"));
        document::write_override(&store, &panel, "other.md").unwrap();

        let (mut session, compiles) = counted_with(store);
        session.open(panel.join("sections/text.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        assert_eq!(
            session.preview().status().main.as_deref(),
            Some("other.md"),
            "discovery answered where the store already had"
        );
    }

    /// An override naming a file the disk no longer holds falls through to
    /// discovery, rather than opening nothing.
    ///
    /// The store is keyed by root and a wrong root cannot be corrected from
    /// inside the panel, so a stale key must not be able to strand a window.
    #[test]
    fn an_override_naming_nothing_falls_back_to_discovery() {
        let panel = fixture("panel");
        let store = document::store_file(&scratch_dir("stale-override"));
        document::write_override(&store, &panel, "deleted.md").unwrap();

        let (mut session, compiles) = counted_with(store);
        session.open(panel.join("sections/text.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        assert_eq!(session.preview().status().main.as_deref(), Some("book.md"));
    }

    /// Setting the main moves the pane, marks the new row, and is remembered
    /// the next time that folder is opened.
    ///
    /// The reopen is what makes the last claim worth asserting: a field written
    /// into `Preview` and not onto the disk would pass every line above it.
    #[test]
    fn setting_the_main_moves_the_pane_and_is_remembered() {
        let dir = scratch_dir("set-main");
        std::fs::write(dir.join("first.md"), "# First\n\nText.\n").unwrap();
        std::fs::write(dir.join("second.md"), "# Second\n\nText.\n").unwrap();

        let store = document::store_file(&scratch_dir("set-main-store"));
        let (mut session, compiles) = counted_with(store.clone());
        session.open(dir.join("first.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);
        assert_eq!(session.preview().status().main.as_deref(), Some("first.md"));

        session.set_main("second.md".to_string()).unwrap();
        assert_eq!(wait_for(&compiles, 2), 2, "the switch compiles once");
        let after = session.preview().status();
        assert_eq!(after.main.as_deref(), Some("second.md"));
        assert!(
            session.preview().text().contains("# Second"),
            "the pane is still holding the file it was told to leave"
        );

        // **A path out of the project is refused, and nothing moves** — both
        // the one that names nothing and the one that names a real file
        // somewhere else, which is the case an existence check alone would let
        // through.
        std::fs::write(dir.parent().unwrap().join("escape.md"), "# Elsewhere\n").unwrap();
        for asked in ["../escape.md", "nothing.md", "/etc/hosts"] {
            assert!(
                session.set_main(asked.to_string()).is_err(),
                "{asked} was accepted as this project's main"
            );
            assert_eq!(
                session.preview().status().main.as_deref(),
                Some("second.md")
            );
        }

        // And a second window over the same folder lands on it without asking.
        let (mut again, again_compiles) = counted_with(store);
        again.open(dir.join("first.md")).unwrap();
        assert_eq!(wait_for(&again_compiles, 1), 1);
        assert_eq!(again.preview().status().main.as_deref(), Some("second.md"));
    }

    /// **Clause 7 on a real filesystem.** A file appearing in the project
    /// refreshes the panel and compiles nothing.
    ///
    /// `revision` is what "compiles nothing" means as an assertion: it moves
    /// only on a compile that produced bytes. The announcement does happen, and
    /// must — the panel is drawn off the status that announcement fetches.
    #[test]
    fn a_file_appearing_in_the_project_refreshes_the_panel_and_compiles_nothing() {
        let dir = scratch_dir("tree-event");
        let document = article_in(&dir);

        let (mut session, compiles) = counted();
        session.open(document).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        let before = session.preview().status();
        assert!(
            !before.entries.iter().any(|entry| entry.path == "notes.md"),
            "the file under test was already listed"
        );

        std::fs::write(dir.join("notes.md"), "# A file nobody names\n").unwrap();
        assert_eq!(wait_for(&compiles, 2), 2, "the panel was never told");
        settle();

        let after = session.preview().status();
        assert_eq!(
            after.revision, before.revision,
            "a file the document does not name caused a compile"
        );
        assert!(
            after.entries.iter().any(|entry| entry.path == "notes.md"),
            "the new file is not in the panel: {:?}",
            after.entries.iter().map(|e| &e.path).collect::<Vec<_>>()
        );
    }

    // -- the pane and the main are two files -------------------------------
    //
    // `mpdf-010` Phase 2's exit gate. It runs over a **writable copy of
    // `samples/showcase/`**: two of the clauses write, `samples/` is tracked,
    // and a suite that left the repository dirty would also destroy the first
    // clause's own premise the second time it was run.

    /// A writable copy of `samples/showcase/`, and its own store.
    fn showcase_in(name: &str) -> (PathBuf, PathBuf) {
        let root = scratch_dir(name);
        copy_tree(&sample("showcase"), &root);
        (
            root,
            document::store_file(&scratch_dir(&format!("{name}-store"))),
        )
    }

    fn copy_tree(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).unwrap();
        for entry in std::fs::read_dir(from).unwrap().flatten() {
            let (source, target) = (entry.path(), to.join(entry.file_name()));
            if source.is_dir() {
                copy_tree(&source, &target);
            } else {
                std::fs::copy(&source, &target).unwrap();
            }
        }
    }

    /// A loaded [`Preview`] with the two files named separately.
    ///
    /// [`compiled`] is its single-file counterpart and cannot express this: it
    /// derives the main from the document, which is the conflation this phase
    /// ends.
    fn project(root: &Path, main: &str, edited: &str) -> Preview {
        let mut preview = Preview {
            root: Some(root.to_path_buf()),
            main: Some(main.to_string()),
            edited: Some(root.join(edited)),
            tree: document::files_under(root),
            ..Preview::default()
        };
        preview.load();
        preview
    }

    fn lines_of(preview: &Preview) -> Vec<usize> {
        preview.anchors.iter().map(|anchor| anchor.line).collect()
    }

    /// **Clause 1, and the phase's whole observable.** The page is the master's,
    /// and the pane's unsaved text is in it.
    ///
    /// Checked against a compile of the same tree after the buffer has been put
    /// on the disk, in the same directory and at the same absolute paths — so
    /// nothing here turns on a compile being reproducible across two locations.
    #[test]
    fn the_panes_unsaved_section_reaches_the_master_that_compiles() {
        let (root, _) = showcase_in("phase2-override");

        let mut preview = project(&root, "showcase.md", "sections/mathematics.md");
        let typed = format!("{}\nA paragraph nobody has saved.\n", preview.text());
        preview.edit(typed.clone());
        preview.compile();
        let unsaved = preview.pdf().expect("the unsaved compile failed").to_vec();

        std::fs::write(root.join("sections/mathematics.md"), &typed).unwrap();
        let saved = project(&root, "showcase.md", "showcase.md");

        assert_eq!(
            unsaved,
            saved.pdf().expect("the saved compile failed"),
            "the buffer did not reach the compile"
        );
    }

    /// **Clause 2. The anchors are lines, not files.**
    ///
    /// `document::Anchor` is `{ line, page }` and the filter under test is
    /// precisely what drops `location.file`, so the claim is keyed to what
    /// survives it. The two sets are disjoint by construction: `mathematics.md`
    /// has one heading, on its own first line, where the master's own three sit
    /// at lines 12, 27 and 54 of `showcase.md` — below its ten-line frontmatter,
    /// and nowhere near line 1.
    #[test]
    fn the_anchors_are_the_headings_of_whichever_file_the_pane_holds() {
        let (root, _) = showcase_in("phase2-anchors");

        let section = project(&root, "showcase.md", "sections/mathematics.md");
        let master = project(&root, "showcase.md", "showcase.md");

        let (theirs, its_own) = (lines_of(&section), lines_of(&master));
        assert_eq!(theirs, [1], "the section's own heading");
        assert_eq!(its_own, [12, 27, 54], "the master's own three");
        assert!(
            theirs.iter().all(|line| !its_own.contains(line)),
            "the two sets are not disjoint, so this clause proves nothing"
        );
    }

    /// **Clause 3.** `⌘S` writes the file in the pane and nothing else.
    #[test]
    fn the_save_writes_the_file_in_the_pane_and_leaves_the_master_alone() {
        let (root, _) = showcase_in("phase2-save");
        let master = std::fs::read(root.join("showcase.md")).unwrap();

        let mut preview = project(&root, "showcase.md", "sections/mathematics.md");
        let typed = format!(
            "{}\nA paragraph the author means to keep.\n",
            preview.text()
        );
        preview.edit(typed.clone());
        preview.save().unwrap();

        assert_eq!(
            std::fs::read_to_string(root.join("sections/mathematics.md")).unwrap(),
            typed
        );
        assert_eq!(
            std::fs::read(root.join("showcase.md")).unwrap(),
            master,
            "the save wrote the master too"
        );
    }

    /// **Clause 4.** The switch refuses over unsaved work, names two ways out
    /// without claiming the file moved, and the discard is the second of them.
    #[test]
    fn a_switch_refuses_over_unsaved_work_and_the_discard_lets_it_through() {
        let (root, store) = showcase_in("phase2-refusal");
        let (mut session, compiles) = counted_with(store);
        session.open(root.join("showcase.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        let mine = "# Mine, unsaved\n\nStill being written.\n".to_string();
        session.edit(mine.clone());
        settle();

        session
            .set_edited("sections/mathematics.md".to_string())
            .unwrap();

        let refused = session.preview().status();
        assert_eq!(
            refused.edited.as_deref(),
            Some("showcase.md"),
            "the switch moved the pane anyway"
        );
        assert_eq!(session.preview().text(), mine, "it took the work");

        let sentence = refused.divergence.expect("the refusal said nothing");
        assert!(
            !sentence.contains("changed on disk"),
            "the refusal claims the file moved, which it did not: {sentence:?}"
        );
        assert!(
            sentence.contains("Save") && sentence.contains("discard"),
            "the refusal names fewer than two ways out: {sentence:?}"
        );

        session
            .set_main("sections/mathematics.md".to_string())
            .unwrap();
        assert_eq!(
            session.preview().status().main.as_deref(),
            Some("showcase.md"),
            "set_main took the same work the switch refused to lose"
        );

        session.discard();
        let clean = session.preview().status();
        assert_eq!(clean.divergence, None, "the discard left the refusal up");
        assert_ne!(session.preview().text(), mine, "the discard kept the work");

        session
            .set_edited("sections/mathematics.md".to_string())
            .unwrap();
        assert_eq!(
            session.preview().status().edited.as_deref(),
            Some("sections/mathematics.md"),
            "the switch was refused twice"
        );
        assert_eq!(
            session.preview().status().main.as_deref(),
            Some("showcase.md"),
            "the switch moved the main"
        );
    }

    /// **Clause 5, first half.** The master moving on disk is a bare recompile,
    /// and it is one even while the pane holds work of its own.
    #[test]
    fn an_external_write_to_the_master_recompiles_and_does_not_run_the_rule() {
        let (root, store) = showcase_in("phase2-master-moved");
        let (mut session, compiles) = counted_with(store);
        session.open(root.join("showcase.md")).unwrap();
        assert_eq!(wait_for(&compiles, 1), 1);

        session
            .set_edited("sections/mathematics.md".to_string())
            .unwrap();
        settle();
        let seen = compiles.load(Ordering::SeqCst);
        let before = session.preview().status();

        session.edit(format!("{}\nUnsaved.\n", session.preview().text()));
        assert_eq!(wait_for(&compiles, seen + 1), seen + 1);

        let master = std::fs::read_to_string(root.join("showcase.md")).unwrap();
        std::fs::write(
            root.join("showcase.md"),
            format!("{master}\nA line another program added.\n"),
        )
        .unwrap();
        assert_eq!(
            wait_for(&compiles, seen + 2),
            seen + 2,
            "it never recompiled"
        );
        settle();

        let after = session.preview().status();
        assert_eq!(
            after.divergence, None,
            "the master's own event ran the divergence rule"
        );
        assert!(
            after.revision > before.revision,
            "the master moved and nothing recompiled"
        );
    }

    /// **Clause 5, second half.** The file in the pane moving runs the rule —
    /// and it still does when that file is the master, which is Phase 1's
    /// behaviour and every single-file document's.
    #[test]
    fn an_external_write_to_the_file_in_the_pane_runs_the_rule() {
        for (name, edited) in [
            ("phase2-section-moved", "sections/mathematics.md"),
            ("phase2-only-file-moved", "showcase.md"),
        ] {
            let (root, store) = showcase_in(name);
            let (mut session, compiles) = counted_with(store);
            session.open(root.join("showcase.md")).unwrap();
            assert_eq!(wait_for(&compiles, 1), 1);

            if edited != "showcase.md" {
                session.set_edited(edited.to_string()).unwrap();
                settle();
            }
            let seen = compiles.load(Ordering::SeqCst);

            session.edit("# Mine, unsaved\n\nStill being written.\n".to_string());
            assert_eq!(wait_for(&compiles, seen + 1), seen + 1);

            std::fs::write(root.join(edited), "# Theirs\n\nBy another program.\n").unwrap();
            assert_eq!(
                wait_for(&compiles, seen + 2),
                seen + 2,
                "{edited}: the change was never noticed"
            );
            settle();

            let status = session.preview().status();
            assert!(
                status
                    .divergence
                    .as_deref()
                    .is_some_and(|said| said.contains("changed on disk")),
                "{edited}: the rule did not run — {:?}",
                status.divergence
            );
            assert_eq!(
                session.preview().text(),
                "# Mine, unsaved\n\nStill being written.\n",
                "{edited}: the work was taken"
            );
        }
    }

    /// **Clause 6.** A single-file document is what it always was.
    ///
    /// "Equal to what it produced before this phase" has nothing committed to
    /// compare against — `tests/golden/` holds `.typ` and no PDF — so the
    /// reproducible form of the claim is a compile this test asks the library
    /// for itself, which is how
    /// `document::tests::a_single_file_document_keeps_its_anchors_and_its_bytes`
    /// already makes it one level down.
    #[test]
    fn a_document_that_is_its_own_project_compiles_to_the_librarys_own_bytes() {
        let dir = scratch_dir("phase2-article");
        article_in(&dir);

        let preview = project(&dir, "article.md", "article.md");
        let markdown = std::fs::read_to_string(dir.join("article.md")).unwrap();

        assert_eq!(
            preview.pdf().expect("the compile failed"),
            md2pdf_core::md_to_pdf(&markdown, &article_assets(&dir)).unwrap()
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
    /// eleven fields off it by name, so a field renamed here and not there breaks
    /// the window silently, at runtime, with no console anyone reads. The type
    /// check over that file (`app/typecheck.mjs`) makes the typedef bind on the
    /// page's side; this makes it bind on Rust's. **Two declarations compared
    /// against each other**, rather than usage compared against a declaration.
    ///
    /// `anchors` holds one, and must: `Anchor` is not reachable in the JSON at
    /// all while the list is empty. `entries` holds one for the same reason.
    ///
    /// **The count moved from ten to eleven in `mpdf-010` Phase 2**, which
    /// added `edited` beside `main` so the panel can mark the row the pane is
    /// holding. Phase 1 left it at ten by coincidence — it removed `sections`
    /// and `master` and added `entries` and `main` — and said so here, in a
    /// note this phase's own scope is the authority for rewriting. A literal
    /// that has now moved once is still not a thing to "fix" to silence a
    /// failure: a failure here is about a field.
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
            entries: vec![document::Entry {
                path: "sections/one.md".to_string(),
                kind: document::Kind::Markdown,
                missing: false,
            }],
            main: Some("report.md".to_string()),
            edited: Some("sections/method.md".to_string()),
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
            11,
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

        let mut listed: Vec<String> = sent["entries"][0]
            .as_object()
            .expect("an `Entry` that is not a JSON object")
            .keys()
            .cloned()
            .collect();
        let mut drawn = typedef_properties(PAGE, "Entry");
        listed.sort_unstable();
        drawn.sort_unstable();
        assert_eq!(
            drawn.len(),
            3,
            "the page's `Entry` typedef declares {} properties: {:?}",
            drawn.len(),
            drawn
        );
        assert_eq!(
            drawn, listed,
            "the page's `Entry` typedef and the entries a `Status` carries name different fields"
        );

        // The kind crosses as a bare lowercase word, which is what the page
        // switches on. A `rename_all` dropped here would send `"Markdown"` and
        // every row would draw as the fallback.
        assert_eq!(sent["entries"][0]["kind"], "markdown");
    }
}
