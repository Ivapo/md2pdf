//! The loop that notices a save, and the one that notices a pause in typing.
//!
//! One recursive watch on the open document's own directory, a filter that
//! sorts an event into the document or one of the assets it names, and a
//! debounce, because one save arrives as several filesystem events. The
//! debounce serves the keyboard too, on an interval of its own. Everything
//! here except the watcher itself is a plain function over plain values.

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use notify::{RecursiveMode, Watcher};

/// How long the loop waits for the filesystem to go quiet before it compiles.
///
/// Measured rather than guessed, by the method the spec's §2 used for the
/// compile timings. Twenty saves under each of three write strategies, on this
/// machine on 2026-08-10, counting the events that name the document and
/// timing the span from the first of them to the last:
///
/// | how the writer saves                   | events per save | span    |
/// |----------------------------------------|-----------------|---------|
/// | truncate the document and write over it | 4              | 0.01 ms |
/// | write a sibling, rename it over         | 2              | 0.00 ms |
/// | write a sibling, `RENAME_SWAP` the two  | 2              | 0.01 ms |
///
/// Medians, over twenty saves each; the largest span of any of the sixty was
/// 0.03 ms, and the first event of a save reached the process 12 ms after the
/// write. The spans are that small because FSEvents hands one save's events
/// over in a single batch, so this interval is not fighting a spread — it is
/// margin against a writer slower than the probe. A hundred milliseconds is
/// some three thousand times the largest span measured, and it is of the same
/// order as the compile itself, which the spec's §2 puts at 8.5 ms and
/// 28.7 ms, so the redraw stays immediate.
pub const DEBOUNCE: Duration = Duration::from_millis(100);

/// How long the pane waits for the typing to stop before it compiles.
///
/// **This is not [`DEBOUNCE`]**, and the difference is what each one is
/// measured against. That one is margin over the filesystem's own batching;
/// this one is measured against the compile it gates and against the hand that
/// is typing.
///
/// Twenty compiles of each document through the pane's own path — a
/// [`crate::document::render`] call in process, release build, no spawn — on
/// this machine on 2026-08-10:
///
/// | document                     | first compile of the process | median of the twenty |
/// |------------------------------|------------------------------|----------------------|
/// | `samples/press-release.md`   | 12.5 ms                      | 0.6 ms               |
/// | `samples/article.md`         | 24.6 ms                      | 1.5 ms               |
///
/// **The compile a keystroke gates is the warm one.** The first compile in a
/// process carries the font parsing and the rest of the one-time setup, and
/// the app pays it at the open, before anyone can type. The spec's §2 puts the
/// same two documents at 8.5 ms and 28.7 ms through the release binary; those
/// include a process spawn each, which is the whole of the difference.
///
/// Three hundred milliseconds is therefore two orders of magnitude above the
/// compile itself, and the compile is not what it is protecting. It is set to
/// the pause between phrases rather than the gap between keystrokes, for a
/// reason particular to this app: **a redraw moves the reader**, so a page
/// redrawn mid-word costs more here than the compile behind it does. The pane
/// now opens on the heading above the caret rather than on page 1, which
/// narrows that cost without removing it — the reader still loses wherever
/// they had scrolled to inside the section they are working in.
pub const TYPING_DEBOUNCE: Duration = Duration::from_millis(300);

/// The one directory a document's watch covers.
///
/// Every path the dialect lets a document name resolves under here —
/// `core/src/emit.rs:check_image` refuses a URI scheme, a leading `/`, a `..`
/// segment and a backslash, and `core/src/frontmatter.rs` puts the bibliography
/// under that same rule — so one recursive watch on this directory covers the
/// document, every asset it names, every asset it will name, and every
/// directory not yet created. It is also computable from the document's path
/// alone, so a document the dialect refuses is watched too, and can be fixed
/// into one that compiles.
///
/// The limit that rides with it: an asset that is a symlink pointing out of
/// this directory is not watched. Its path is legal and resolves inside, and
/// the bytes it names live where a recursive watch on the tree never sees.
pub fn root(document: &Path) -> PathBuf {
    document
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .to_path_buf()
}

/// What a path under the watch is, to the document the pane holds.
///
/// The three are not the same event. An asset that moves is still "the page is
/// out of date, compile", because nothing but the disk supplies a figure or a
/// bibliography. The document that moves is "the disk moved, decide", because
/// the pane's own buffer is what compiles now. And anything else under the root
/// is "the project's files moved" — which the panel needs and the compile must
/// not run for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    /// The open document itself.
    Document,
    /// One of the assets the document names: a figure, or its bibliography.
    Asset,
    /// A path under the root that is neither.
    ///
    /// **This is the events the filter used to drop**, and the panel needs
    /// exactly them: a file created, renamed or deleted anywhere in the project
    /// changes what the panel lists and changes nothing about the page. So it
    /// refreshes the listing and **does not compile**, which is
    /// `crate::preview::Preview`'s `revision` standing still.
    Tree,
}

/// What one settled window of events was about.
///
/// The debounce folds several events into one call, and a save that also
/// rewrites a figure is one window with both marks set, so the callback is
/// handed the pair rather than a single path.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Changed {
    /// The open document moved on disk.
    pub document: bool,
    /// At least one asset the document names moved.
    pub assets: bool,
    /// At least one other path under the root moved, so the panel's listing is
    /// out of date.
    pub tree: bool,
}

/// Which of the three a change to this path is, or none of them.
///
/// It is the document when the path is the document, an asset when it is one of
/// the paths `md2pdf_core::image_paths` and `md2pdf_core::bibliography_path`
/// returned for it, and the tree when it is anything else under the root.
/// Everything outside the root is dropped.
///
/// **The root is a parameter and is no longer re-derived from the document.**
/// It was `root(document)` until `mpdf-010` Phase 1, which is the same
/// directory only while the project is the document's own folder — and the
/// climb that finds a master above an opened section makes the two differ. The
/// *document and its assets* are still resolved against the document's own
/// directory, which is where a path the markdown writes resolves; the root is
/// what `Tree` is measured against.
///
/// **The document stays in here**, though its events no longer mean "compile".
/// A path dropped from the filter would never reach the rule that decides what
/// its events do mean, and the rule is the whole of how the pane survives an
/// external change.
///
/// **Both sides are canonicalized**, and the whole loop depends on it.
/// `notify` canonicalizes a path as it registers it, because FSEvents reports
/// the resolved path, so an event arrives naming `/private/var/…` where the
/// Open dialog handed the app `/var/…`. On macOS `/tmp` and `/var` are both
/// symlinks into `/private`, so that is the default case rather than an exotic
/// one, and comparing the two as they arrive matches nothing — the watcher
/// would run, every event would be dropped, and the page would simply never
/// redraw.
///
/// The document's directory is what gets resolved, not each file, because an
/// asset the document names before anyone creates it has no real path to
/// resolve yet. Joining a relative path onto a resolved directory gives a
/// resolved path either way.
pub fn classify(event: &Path, project: &Path, document: &Path, assets: &[String]) -> Option<Change> {
    let name = document.file_name()?;
    let here = resolve(&root(document));
    let project = resolve(project);
    let event = resolve(event);

    if event == here.join(name) {
        Some(Change::Document)
    } else if assets.iter().any(|asset| event == here.join(asset)) {
        Some(Change::Asset)
    } else if event.starts_with(&project) {
        Some(Change::Tree)
    } else {
        None
    }
}

/// A path as the filesystem really spells it, or as given when it names
/// nothing that exists — a file just deleted, or one not yet created.
///
/// **`crate::document`'s walk reads this one rather than spelling
/// canonicalization a second way**, so the panel and the filter agree about
/// what a path is. It returns its *input* when canonicalization fails, which is
/// why it is the walk's spelling and not a write's: a file being created never
/// canonicalizes, so a check built on it would pass a path textually.
pub(crate) fn resolve(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// The quiet period one save has to survive before it becomes a compile.
///
/// Time is a parameter rather than a call to the clock, so the test that pins
/// this needs no filesystem and cannot flake on a slow machine.
pub struct Debounce {
    interval: Duration,
    due: Option<Instant>,
}

impl Debounce {
    pub fn new(interval: Duration) -> Self {
        Self {
            interval,
            due: None,
        }
    }

    /// A relevant event arrived: the compile falls due one quiet interval on.
    pub fn touch(&mut self, now: Instant) {
        self.due = Some(now + self.interval);
    }

    /// How long to wait for the next event, or `None` when nothing is pending
    /// and the loop may block until one arrives.
    pub fn wait(&self, now: Instant) -> Option<Duration> {
        self.due.map(|due| due.saturating_duration_since(now))
    }

    /// Take the pending compile, if the quiet interval has elapsed.
    pub fn take(&mut self, now: Instant) -> bool {
        match self.due {
            Some(due) if now >= due => {
                self.due = None;
                true
            }
            _ => false,
        }
    }
}

/// A running watch. Dropping it stops the loop.
///
/// Dropping the watcher unregisters the directory and drops the sender its
/// handler holds, which disconnects the channel, which ends the thread. That
/// is the whole mechanism by which opening a second document moves the watch.
pub struct Watch {
    _watcher: notify::RecommendedWatcher,
}

/// Watch a directory, and call `on_change` once per settled change.
///
/// `classify` is the filter and runs on every event, before the debounce.
/// `on_change` runs on this watch's own thread, so a compile there does not
/// touch the thread that draws the window. It is handed what the window was
/// about rather than nothing, because the document and an asset now mean two
/// different things.
pub fn start(
    root: &Path,
    interval: Duration,
    classify: impl Fn(&Path) -> Option<Change> + Send + 'static,
    on_change: impl FnMut(Changed) + Send + 'static,
) -> Result<Watch, String> {
    let (events, received) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |event| {
        let _ = events.send(event);
    })
    .map_err(|e| format!("cannot watch {}: {e}", root.display()))?;

    watcher
        .watch(root, RecursiveMode::Recursive)
        .map_err(|e| format!("cannot watch {}: {e}", root.display()))?;

    settle(
        received,
        interval,
        move |changed: &mut Changed, event: notify::Result<notify::Event>| {
            // A watcher error names no file to redraw for, so it is dropped
            // with the same shrug as an event under some other path.
            let Ok(event) = event else {
                return false;
            };

            let mut counted = false;
            for path in &event.paths {
                match classify(path) {
                    Some(Change::Document) => (changed.document, counted) = (true, true),
                    Some(Change::Asset) => (changed.assets, counted) = (true, true),
                    Some(Change::Tree) => (changed.tree, counted) = (true, true),
                    None => {}
                }
            }
            counted
        },
        on_change,
    );

    Ok(Watch { _watcher: watcher })
}

/// A loop that folds a stream of nudges into one call per quiet interval.
///
/// Sending on the returned channel is what nudges it, and **dropping the
/// sender ends the thread**, which is how a session's typing loop goes with
/// the document it belongs to — the same mechanism [`Watch`] uses one door
/// along.
pub fn debounced(
    interval: Duration,
    mut on_settle: impl FnMut() + Send + 'static,
) -> mpsc::Sender<()> {
    let (nudges, received) = mpsc::channel();
    settle(
        received,
        interval,
        |_: &mut (), ()| true,
        move |()| on_settle(),
    );
    nudges
}

/// The thread both debounces run on.
///
/// `absorb` takes each item into the accumulator and says whether it counted;
/// an item that counts restarts the quiet interval, and one that does not is
/// dropped without disturbing a compile already falling due. When the interval
/// elapses, `on_settle` is handed the accumulator and the next window starts
/// from `A::default()`.
///
/// It is one loop rather than two because the app now has two intervals over
/// one shape: the filesystem's, which accumulates what changed, and the
/// keyboard's, which accumulates nothing.
fn settle<T, A>(
    received: mpsc::Receiver<T>,
    interval: Duration,
    mut absorb: impl FnMut(&mut A, T) -> bool + Send + 'static,
    mut on_settle: impl FnMut(A) + Send + 'static,
) where
    T: Send + 'static,
    A: Default + Send + 'static,
{
    std::thread::spawn(move || {
        let mut debounce = Debounce::new(interval);
        let mut pending = A::default();

        loop {
            let item = match debounce.wait(Instant::now()) {
                Some(timeout) => match received.recv_timeout(timeout) {
                    Ok(item) => Some(item),
                    Err(mpsc::RecvTimeoutError::Timeout) => None,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                },
                None => match received.recv() {
                    Ok(item) => Some(item),
                    Err(mpsc::RecvError) => break,
                },
            };

            if let Some(item) = item
                && absorb(&mut pending, item)
            {
                debounce.touch(Instant::now());
            }

            if debounce.take(Instant::now()) {
                on_settle(std::mem::take(&mut pending));
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch directory this test process owns. It sits under
    /// `std::env::temp_dir()` deliberately: on macOS that resolves through a
    /// symlink, so a canonicalization bug fails here rather than passing under
    /// some directory that happens not to be symlinked.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("md2pdf-watch-test-{}", std::process::id()))
            .join(name);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn the_watch_set_is_the_documents_own_directory() {
        assert_eq!(
            root(Path::new("/tmp/notes/paper.md")),
            Path::new("/tmp/notes")
        );
        assert_eq!(root(Path::new("paper.md")), Path::new("."));
    }

    /// The document, both the figures it names and the bibliography it
    /// declares reach the loop, and they reach it as three different things. A
    /// sibling the document never names is the third, `Change::Tree`, which the
    /// panel reads and the compile ignores. Only a path outside the root is
    /// nothing at all.
    ///
    /// The bibliography is nothing special here, and that is the finding: it
    /// is one more string in the one list, sorted by the same comparison.
    ///
    /// **`mpdf-010` Phase 1's gate, clause 7.** The root and the document's own
    /// directory are the same here, which is every document this app opened
    /// before the climb existed; the test below is the one where they differ.
    #[test]
    fn the_filter_sorts_the_document_from_its_assets_and_its_project() {
        let dir = scratch_dir("filter");
        let document = dir.join("figure.md");
        let assets = [
            "refs.yml".to_string(),
            "dot.png".to_string(),
            "figures/mark.svg".to_string(),
        ];

        let real = dir.canonicalize().unwrap();
        let sort = |path: PathBuf| classify(&path, &dir, &document, &assets);

        assert_eq!(sort(real.join("figure.md")), Some(Change::Document));
        assert_eq!(sort(real.join("refs.yml")), Some(Change::Asset));
        assert_eq!(sort(real.join("dot.png")), Some(Change::Asset));
        assert_eq!(sort(real.join("figures/mark.svg")), Some(Change::Asset));
        assert_eq!(
            sort(real.join("notes.txt")),
            Some(Change::Tree),
            "a file under the root the document does not name is the panel's"
        );
        assert_eq!(
            sort(real.parent().unwrap().join("elsewhere.md")),
            None,
            "outside the root is nothing at all"
        );
    }

    /// The root and the document's directory are two things now, and each
    /// answer is measured against the right one.
    ///
    /// A section under `sections/` is the document; its own figure beside it is
    /// an asset, resolved against the *document's* directory; the master above
    /// it is the tree, because the root is the project rather than the folder
    /// the pane's file sits in.
    #[test]
    fn a_root_above_the_document_still_resolves_the_assets_beside_it() {
        let project = scratch_dir("above");
        let inside = project.join("sections");
        std::fs::create_dir_all(&inside).unwrap();

        let document = inside.join("method.md");
        let assets = ["mark.svg".to_string()];
        let real = project.canonicalize().unwrap();
        let sort = |path: PathBuf| classify(&path, &project, &document, &assets);

        assert_eq!(sort(real.join("sections/method.md")), Some(Change::Document));
        assert_eq!(sort(real.join("sections/mark.svg")), Some(Change::Asset));
        assert_eq!(sort(real.join("book.md")), Some(Change::Tree));
        assert_eq!(sort(real.join("figures/plot.svg")), Some(Change::Tree));
        assert_eq!(sort(real.parent().unwrap().join("outside.md")), None);
    }

    /// The event and the document name the same file and differ only by
    /// `/var` against `/private/var`, which is what every real event does.
    #[test]
    fn an_event_under_private_var_names_a_document_under_var() {
        let dir = scratch_dir("canonical");
        let document = dir.join("paper.md");
        std::fs::write(&document, "# Paper\n").unwrap();

        let event = document.canonicalize().unwrap();
        assert_ne!(
            event, document,
            "the scratch directory is not symlinked, so this case proves nothing"
        );

        assert_eq!(
            classify(&event, &dir, &document, &[]),
            Some(Change::Document)
        );
    }

    /// Two events inside the window are one compile; two outside are two.
    #[test]
    fn the_debounce_folds_one_saves_events_into_one_compile() {
        let interval = Duration::from_millis(100);
        let mut debounce = Debounce::new(interval);
        let start = Instant::now();

        // One save, arriving as two events a quarter of the window apart.
        debounce.touch(start);
        assert!(!debounce.take(start + Duration::from_millis(25)));
        debounce.touch(start + Duration::from_millis(25));
        assert!(!debounce.take(start + Duration::from_millis(100)));
        assert!(debounce.take(start + Duration::from_millis(130)));

        // And having fired, it does not fire again on its own.
        assert!(!debounce.take(start + Duration::from_millis(400)));

        // Two saves, a window and a half apart: two compiles.
        debounce.touch(start + Duration::from_millis(500));
        assert!(debounce.take(start + Duration::from_millis(600)));
        debounce.touch(start + Duration::from_millis(650));
        assert!(debounce.take(start + Duration::from_millis(750)));
    }

    /// The keyboard's interval folds a burst of typing into one compile, and
    /// lets a pause through.
    ///
    /// It is the same shape the filesystem's interval uses and the same reason
    /// it takes the time as a parameter: this case needs no clock and no
    /// keyboard, so it cannot flake.
    #[test]
    fn the_typing_debounce_folds_a_burst_into_one_compile() {
        let mut debounce = Debounce::new(TYPING_DEBOUNCE);
        let start = Instant::now();

        // Four keystrokes, each well inside the window: one compile.
        for gap in [0, 80, 160, 240] {
            debounce.touch(start + Duration::from_millis(gap));
            assert!(!debounce.take(start + Duration::from_millis(gap)));
        }
        assert!(!debounce.take(start + Duration::from_millis(500)));
        assert!(debounce.take(start + Duration::from_millis(560)));

        // Two keystrokes with a pause between them: two compiles.
        let later = start + Duration::from_secs(1);
        debounce.touch(later);
        assert!(debounce.take(later + TYPING_DEBOUNCE));
        debounce.touch(later + Duration::from_millis(500));
        assert!(debounce.take(later + Duration::from_millis(500) + TYPING_DEBOUNCE));
    }

    /// Nothing pending means the loop blocks rather than spins.
    #[test]
    fn an_idle_debounce_asks_for_no_timeout() {
        let mut debounce = Debounce::new(Duration::from_millis(100));
        let start = Instant::now();

        assert_eq!(debounce.wait(start), None);
        debounce.touch(start);
        assert_eq!(debounce.wait(start), Some(Duration::from_millis(100)));
    }
}
