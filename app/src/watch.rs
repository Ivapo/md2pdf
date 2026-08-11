//! The loop that notices a save.
//!
//! One recursive watch on the open document's own directory, a filter that
//! admits the document and the figures it names, and a debounce, because one
//! save arrives as several filesystem events. Everything here except the
//! watcher itself is a plain function over plain values.

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

/// The one directory a document's watch covers.
///
/// Every path the dialect lets a document name resolves under here —
/// `core/src/emit.rs:check_image` refuses a URI scheme, a leading `/`, a `..`
/// segment and a backslash — so one recursive watch on this directory covers
/// the document, every figure it names, every figure it will name, and every
/// directory not yet created. It is also computable from the document's path
/// alone, so a document the dialect refuses is watched too, and can be fixed
/// into one that compiles.
///
/// The limit that rides with it: a figure that is a symlink pointing out of
/// this directory is not watched. Its path is legal and resolves inside, and
/// the bytes it names live where a recursive watch on the tree never sees.
pub fn root(document: &Path) -> PathBuf {
    document
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .to_path_buf()
}

/// Should a change to this path redraw the page?
///
/// It should when the path is the document, or one of the paths
/// `md2pdf_core::image_paths` returned for it. Everything else under the
/// directory is dropped, which is what a directory-valued watch buys and pays
/// for.
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
/// The document's directory is what gets resolved, not each file, because a
/// figure the document names before anyone creates it has no real path to
/// resolve yet. Joining a relative path onto a resolved directory gives a
/// resolved path either way.
pub fn is_relevant(event: &Path, document: &Path, images: &[String]) -> bool {
    let Some(name) = document.file_name() else {
        return false;
    };
    let root = resolve(&root(document));
    let event = resolve(event);

    event == root.join(name) || images.iter().any(|image| event == root.join(image))
}

/// A path as the filesystem really spells it, or as given when it names
/// nothing that exists — a file just deleted, or one not yet created.
fn resolve(path: &Path) -> PathBuf {
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
/// `relevant` is the filter and runs on every event, before the debounce.
/// `on_change` runs on this watch's own thread, so a compile there does not
/// touch the thread that draws the window.
pub fn start(
    root: &Path,
    interval: Duration,
    relevant: impl Fn(&Path) -> bool + Send + 'static,
    mut on_change: impl FnMut() + Send + 'static,
) -> Result<Watch, String> {
    let (events, received) = mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |event| {
        let _ = events.send(event);
    })
    .map_err(|e| format!("cannot watch {}: {e}", root.display()))?;

    watcher
        .watch(root, RecursiveMode::Recursive)
        .map_err(|e| format!("cannot watch {}: {e}", root.display()))?;

    std::thread::spawn(move || {
        let mut debounce = Debounce::new(interval);

        loop {
            let event = match debounce.wait(Instant::now()) {
                Some(timeout) => match received.recv_timeout(timeout) {
                    Ok(event) => Some(event),
                    Err(mpsc::RecvTimeoutError::Timeout) => None,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                },
                None => match received.recv() {
                    Ok(event) => Some(event),
                    Err(mpsc::RecvError) => break,
                },
            };

            // A watcher error names no file to redraw for, so it is dropped
            // with the same shrug as an event under some other path.
            if let Some(Ok(event)) = event
                && event.paths.iter().any(|path| relevant(path))
            {
                debounce.touch(Instant::now());
            }

            if debounce.take(Instant::now()) {
                on_change();
            }
        }
    });

    Ok(Watch { _watcher: watcher })
}
