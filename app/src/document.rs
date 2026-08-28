//! Everything this app decides without a window.
//!
//! The read, the compile, the title and the error string all live here, in
//! ordinary functions with ordinary tests. Only what genuinely needs a window
//! goes through Tauri's command layer, because a GUI whose logic is reachable
//! only by clicking has no exit gate but a screenshot.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use md2pdf_core::Asset;
use serde::Serialize;

/// One heading, and the page its typeset form landed on.
///
/// This is `md2pdf_core::Anchor` again, and the duplication is deliberate for
/// the same reason [`read_assets_with`] duplicates its counterpart: this one
/// crosses to the page inside `crate::preview::Status`, so it must serialize,
/// and giving `core` a serde dependency for two `usize` fields would widen what
/// the app asks of it well past the one function it gained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Anchor {
    /// The 1-based line of the markdown heading.
    pub line: usize,
    /// The 1-based page its compiled form landed on.
    pub page: usize,
}

/// What one compile produced, and what the document named while producing it.
pub struct Render {
    /// The paths the document names: its sections in the order the master
    /// reads them, then the bibliography it declares, then the images in
    /// reader order.
    ///
    /// **The sections go in first and unconditionally**, and that is the one
    /// thing this list could not be built without.
    /// `md2pdf_core::section_paths` reads the master's own text and cannot
    /// fail, where the other two answer about the assembled document and now
    /// fail with `MissingSection` for a section that does not exist yet — and
    /// `crate::preview::Preview::compile` replaces this list only when it is
    /// `Some`. A list built the way it was built before this phase would stay
    /// empty, `crate::watch::classify` would drop the section's creation
    /// event, and the app would never recover.
    ///
    /// **Three answers, and each says a different thing.**
    ///
    /// - `Some(sections ++ bibliography ++ images)` when both walks answer.
    ///   For a document naming no section that is the vector this returned
    ///   before the sections existed, in the same order, with an empty list in
    ///   front of it.
    /// - `Some(sections)` when they do not and the master names any. This
    ///   *replaces* the list with a shorter one, so a multi-file document with
    ///   a missing section stops watching its figures until that section comes
    ///   back. The trade is deliberate: recovering the section beats watching
    ///   figures through a window in which nothing compiles anyway.
    /// - `None` when neither answers and no section is named. The caller keeps
    ///   the list it already had, which is what stops a transient
    ///   out-of-dialect edit from dropping the images the app knows about.
    ///
    /// Either `Some` arrives even when the compile failed, because emission
    /// reads the text and not the disk: a document whose figures are all
    /// missing still names them. That is what keeps the watch filter working
    /// while the compile does not.
    pub assets: Option<Vec<String>>,

    /// The sections the master names, in the order it reads them.
    ///
    /// The same names [`Render::assets`] puts in front of everything else, kept
    /// on their own because the panel names *files* where that list names paths
    /// to watch — and because that list is `None` exactly when the caller must
    /// keep the one it has, which is not a thing a panel can draw.
    ///
    /// Empty for a document that names no section, and empty while the marker
    /// naming them is mid-edit: `md2pdf_core::section_paths` reads the text, so
    /// this says what the buffer names now and not what is on the disk.
    pub sections: Vec<String>,

    /// The bytes, or the sentence the terminal would print.
    pub pdf: Result<Vec<u8>, String>,

    /// Where each heading landed, for the pane to open on.
    ///
    /// **Only the headings written in the file the pane holds**, which is
    /// [`Pane`]'s whole job and is no longer the same file as the one that
    /// compiles. A line means something in exactly one buffer and the pane
    /// holds one, so an anchor from another file is not a worse match — it is
    /// a number about a document the pane is not showing, and
    /// `app/dist/index.html:caretPage` walks a flat list and breaks at the
    /// first anchor past the caret. Left in, three sections numbered 1, 4 and 1
    /// would open the frame on whatever page the last of them landed on.
    ///
    /// A pure manifest whose own text is in the pane therefore yields none and
    /// the frame opens at page 1, which `caretPage` already documents as its
    /// no-anchor case; a master carrying a preface syncs on its own headings,
    /// and a section in the pane syncs on that section's, which is the state
    /// `mpdf-010` Phase 2 exists to reach.
    ///
    /// Empty when the compile failed, and empty when `core`'s own count guard
    /// declined to answer. Unlike [`Render::assets`] this describes the *page*
    /// rather than the text, so it is only ever as good as the bytes beside it.
    pub anchors: Vec<Anchor>,
}

/// Which file's headings become anchors: the one the pane is holding.
///
/// `md2pdf_core::Location`'s `file` is `None` for a heading written in the
/// master's own text and `Some(path)` for one written in a section, **spelled
/// as the master names it** — so the first two arms below are exactly the two
/// shapes that comparison can take, and the third is the one it cannot.
///
/// **Three arms rather than an `Option`, and the third is why.** An `edited`
/// that does not sit under `main`'s own directory has no master-relative
/// spelling at all. Answering `Master` for it would put the master's heading
/// lines against a buffer whose lines mean something else entirely, which is
/// the defect this filter exists to prevent, arriving through the case that
/// looks like an absence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane<'a> {
    /// The pane holds `main` itself.
    Master,
    /// The pane holds this path, spelled from the master's own directory.
    ///
    /// It matches nothing when the master does not name that file — a
    /// `README.md` opened beside a master contributes no anchors and the page
    /// opens at page 1. Correct rather than special-cased.
    Beside(&'a str),
    /// The pane holds a file the master's directory does not reach, so no
    /// heading in this document is a number about it.
    Away,
}

impl Pane<'_> {
    /// Does this anchor belong to the file the pane is holding?
    fn holds(&self, file: Option<&str>) -> bool {
        match self {
            Pane::Master => file.is_none(),
            Pane::Beside(path) => file == Some(*path),
            Pane::Away => false,
        }
    }
}

/// Compile one markdown string, reading the files it names from beside the
/// document.
///
/// **The markdown is a parameter and not a path**, and that is the whole of
/// what the text pane needed: the string the pane holds is what compiles, and
/// the file beside it need never have held that text. `md2pdf_core::md_to_pdf`
/// already took a `&str`, so `core` gained nothing for this.
///
/// Every failure arrives as the sentence the CLI prints after its `error: `
/// prefix, and the two classes are not the same type. A construct outside the
/// dialect is a `md2pdf_core::Error` and reaches the page through its
/// `Display`; a file that will not read is no `Error` at all, and
/// [`read_assets_with`] builds the plain sentence for it. So a document this
/// app refuses is refused in the same words at the window and at the terminal.
///
/// **The file read is the caller's**, which is the seam Phase 1 opened at
/// [`read_assets_with`] one level up, and it exists for the same reason: a
/// caller that counts its own reads can check a claim about them rather than
/// argue it from the loop. `mpdf-010` Phase 2 is the second thing it bought —
/// the pane's buffer standing in for one file of the document — and
/// [`render_project`] is where that rides.
///
/// **One closure serves both passes**, and that is what keeps both claims worth
/// checking. [`read_sections_with`] borrows it and [`read_assets_with`] takes
/// what is left, so every file this app opens for one compile goes through the
/// one closure — a second would leave half the reads unwatched.
pub fn render_with(
    directory: &Path,
    markdown: &str,
    pane: Pane<'_>,
    mut read: impl FnMut(&Path) -> std::io::Result<Vec<u8>>,
) -> Render {
    // **The sections come first**, exactly as `cli/src/main.rs:run` orders
    // them: a master is not a document until they are joined in, so neither
    // shopping list can be asked anything before they are read. The names are
    // taken separately from the bytes because the list below needs them whether
    // or not the files are there yet.
    let named: Vec<String> = md2pdf_core::section_paths(markdown)
        .map(|sections| sections.into_iter().map(|section| section.path).collect())
        .unwrap_or_default();

    let sections = read_sections_with(markdown, directory, &mut read);
    let supplied: &[Asset] = match &sections {
        Ok(sections) => sections.as_slice(),
        Err(_) => &[],
    };

    // The path travels separately from the bytes, and this is the only place
    // it is built: `read_assets_with` below returns a `Vec<Asset>` that reaches
    // the compile and nothing else, where this list reaches the watch filter.
    // The two walks answer or fail together — the bibliography first, as
    // `cli/src/main.rs:read_assets` orders them — and the sections go in front
    // of both whatever they answer. [`Render::assets`] argues the three
    // branches.
    let assets: Option<Vec<String>> = md2pdf_core::image_paths(markdown, supplied)
        .ok()
        .map(|images| {
            named
                .iter()
                .cloned()
                .chain(
                    md2pdf_core::bibliography_path(markdown, supplied)
                        .ok()
                        .flatten()
                        .map(|named| named.path),
                )
                .chain(images.into_iter().map(|image| image.path))
                .collect()
        })
        .or_else(|| (!named.is_empty()).then(|| named.clone()));

    let rendered = sections
        .and_then(|sections| read_assets_with(markdown, sections, directory, read))
        .and_then(|supplied| {
            md2pdf_core::md_to_pdf_with_anchors(markdown, &supplied).map_err(|e| e.to_string())
        });

    // The anchors describe the bytes, so a failure has none — where `assets`
    // above survives one, because it describes the text. An anchor written in
    // a file the pane is not showing is a number about a document the reader
    // cannot see; [`Pane`] is the one comparison that decides which those are,
    // and [`Render::anchors`] argues why dropping the rest is the only answer
    // that is true by construction.
    let (pdf, anchors) = match rendered {
        Ok(rendered) => (
            Ok(rendered.pdf),
            rendered
                .anchors
                .into_iter()
                .filter(|anchor| pane.holds(anchor.location.file.as_deref()))
                .map(|anchor| Anchor {
                    line: anchor.location.line,
                    page: anchor.page,
                })
                .collect(),
        ),
        Err(message) => (Err(message), Vec::new()),
    };

    Render {
        assets,
        sections: named,
        pdf,
        anchors,
    }
}

/// Compile the project: `main`'s own text, with the pane's buffer standing in
/// for the one file the pane is holding.
///
/// **The override rides the closure [`render_with`] already takes**, which is
/// one rule instead of a branch. The closure answers `edited` from the buffer
/// and every other path from the disk, and **`main`'s own text is read through
/// it too** — so it returns the buffer exactly when the pane holds the master,
/// and the disk copy otherwise, with nothing here having to ask which case it
/// is in. That the markdown is `main`'s and the directory is `main`'s is the
/// whole of what separates the file that compiles from the file that is edited:
/// every path the document names resolves against the master, wherever the pane
/// happens to be.
///
/// **The closure yields bytes where the compile wants a string**, so this read
/// decodes as UTF-8 and a `main` that is not text fails here. It fails in
/// [`read_document`]'s own sentence, built by wrapping the decode in the
/// `std::io::Error` `read_to_string` would have raised, so a main that will not
/// read reads the same in the window whichever path reached it.
pub fn render_project(main: &Path, edited: &Path, buffer: &str) -> Result<Render, String> {
    let read = |file: &Path| -> std::io::Result<Vec<u8>> {
        if crate::watch::resolve(file) == crate::watch::resolve(edited) {
            Ok(buffer.as_bytes().to_vec())
        } else {
            std::fs::read(file)
        }
    };

    let markdown = read(main)
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "stream did not contain valid UTF-8",
                )
            })
        })
        .map_err(|e| format!("cannot read {}: {e}", main.display()))?;

    let spelled = under(main, edited);
    let pane = if main == edited {
        Pane::Master
    } else {
        match spelled.as_deref() {
            Some(path) => Pane::Beside(path),
            None => Pane::Away,
        }
    };

    Ok(render_with(directory(main), &markdown, pane, read))
}

/// Read a document's text, in the words the terminal uses for a file it cannot
/// read.
///
/// The read left [`render`] when the pane's text became what compiles, and it
/// landed here rather than at the caller because this sentence is one of the
/// two the app owes the CLI. Phase 1 built it inside the compile; the same
/// string reaches the page from one function further out.
pub fn read_document(document: &Path) -> Result<String, String> {
    std::fs::read_to_string(document)
        .map_err(|e| format!("cannot read {}: {e}", document.display()))
}

/// The directory a document's assets resolve against: the one it sits in.
///
/// An empty parent is a document named with no directory at all, and joining an
/// asset onto `""` resolves it against the working directory, which is what the
/// CLI does for the same input.
pub fn directory(document: &Path) -> &Path {
    document.parent().unwrap_or(Path::new(""))
}

/// The window's title for an open document: the file's own name.
///
/// A path with no file name at all cannot be opened, so the full path is a
/// fallback that no dialog reaches, not a second title format.
pub fn title(document: &Path) -> String {
    document
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| document.display().to_string())
}

/// Where an export lands unless the user says otherwise: the document's path
/// with a `.pdf` extension.
///
/// This is `cli/src/main.rs:default_output`'s rule, and the duplication is
/// deliberate for the reason [`read_assets_with`] duplicates its own
/// counterpart. Sharing it would mean making one crate's binary reachable from
/// the other, and the two front ends are two binaries over one library.
pub fn default_output(document: &Path) -> PathBuf {
    document.with_extension("pdf")
}

// ---------------------------------------------------------------------------
// The project: its root, its files, the main among them, the bytes of one of
// them, and the one fact this app remembers about it. `mpdf-010` Phases 1
// and 5.
//
// Ordinary functions, because a panel that could only be checked by opening a
// window would have no exit gate but a screenshot — this file's own header,
// applied to the newest thing in it.
// ---------------------------------------------------------------------------

/// What one row of the panel is.
///
/// **A directory is never an entry.** The page derives the folder headings and
/// the indentation from the path's own segments, which is a thing a page can do
/// and a thing a nested node type would make `crate::preview::Status` carry
/// twice.
///
/// This crosses to the page inside that `Status`, so it serializes — the same
/// reason [`Anchor`] is declared here rather than borrowed from `core`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Entry {
    /// Root-relative, with `/` separators on every platform, so the page can
    /// split it into segments without knowing what a path is here.
    pub path: String,
    /// Which of the three channels of the pipeline would read this file.
    pub kind: Kind,
    /// True for a path the master names that the disk does not hold.
    ///
    /// It is the row the author most needs to see: it is the state
    /// `md2pdf_core::Error::MissingSection` refuses on, and a panel built from
    /// the disk alone would be silent about exactly the file that broke the
    /// document.
    pub missing: bool,
}

/// The three kinds of file the pipeline reads, which are the three the panel
/// lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Markdown,
    Bibliography,
    Image,
}

/// Which channel would read this path, or `None` for a file the pipeline has no
/// use for.
///
/// **Each channel is compared the way that channel compares it**, and the
/// asymmetry is inherited rather than invented, so nobody "fixes" it into a
/// panel that disagrees with the compiler:
///
/// - markdown is `eq_ignore_ascii_case("md")`, as `core/src/emit.rs`'s
///   `lone_markdown_link` reads an include marker;
/// - a bibliography is folded to lower case and matched against `bib`, `yml`
///   and `yaml`, as `core/src/bibliography.rs` does. **All three, not `.bib`
///   alone**: a document whose frontmatter names `refs.yml` compiles, and a
///   panel blind to it would tell the author this app could not read it —
///   which is `mpdf-010` §2's argument for taking the image list off `core`
///   rather than hand-writing a subset, applied to the channel beside it;
/// - an image is matched case-*sensitively* against
///   [`md2pdf_core::IMAGE_EXTENSIONS`], as `core/src/emit.rs`'s `check_image`
///   does, reading `VirtualPath::extension` — which is the function Typst's own
///   format detection reads.
fn kind_of(path: &str) -> Option<Kind> {
    let name = path.rsplit('/').next()?;
    let (_, extension) = name.rsplit_once('.')?;

    if extension.eq_ignore_ascii_case("md") {
        return Some(Kind::Markdown);
    }
    if matches!(extension.to_lowercase().as_str(), "bib" | "yml" | "yaml") {
        return Some(Kind::Bibliography);
    }
    if md2pdf_core::IMAGE_EXTENSIONS.contains(&extension) {
        return Some(Kind::Image);
    }
    None
}

/// Where the project the author opened begins.
///
/// **The opened file's parent is where the search starts, not where it stops.**
/// [`crate::watch::root`] is `document.parent()`, so a double-click on
/// `showcase/sections/text.md` would root the project at `showcase/sections` —
/// below the master that names it, which the panel would then never list and
/// discovery would never find. Taking that parent as the root makes the whole
/// point of the panel unreachable, which is why this exists rather than being
/// left to `watch::root`.
///
/// **The rule**: start at the opened file's parent; if any `.md` in *that
/// directory's* parent names the opened file as one of its sections, the root is
/// the parent instead. `md2pdf_core::section_paths` is what answers "names it",
/// reading text and constructing `Ok` unconditionally, so the test is total over
/// every markdown file in that one directory. Which of them matched is not
/// returned, so the unordered `read_dir` costs no determinism: every match gives
/// the same answer.
///
/// **One level is a cap, and it is chosen rather than derived.** An earlier
/// draft argued it was a property of `mpdf-008`'s refusal of an include inside
/// an included section; that refusal is about a master naming a *master*, where
/// this looks for the master of a *file*, which a deeper relative path reaches
/// with no nesting at all — `[](parts/ch1/text.md)` is a supported marker. So
/// `parts/ch1/text.md` roots at `parts/ch1`, below its own master, which is
/// verbatim the failure the paragraph above says this prevents, surviving one
/// level down. **That cost is asserted in this file's own tests** rather than
/// left as a defect nobody noticed, and `specs/file_panel_spec.md` OQ-7 carries
/// the question. The cap is argued on cost instead: climbing further means
/// reading markdown in `~/Documents` and above to guess where the project is,
/// and this app has never opened a file the author did not name or a document
/// did not name.
///
/// Two edges answer the same way, and neither is an error in the window: a
/// parent with no parent, and a grandparent that will not `read_dir`. Both are
/// *no candidate found*, so the root is the opened file's own parent, which is
/// [`crate::watch::root`]'s answer unchanged and is every single-file document.
pub fn project_root(opened: &Path) -> PathBuf {
    let here = crate::watch::root(opened);
    let Some(above) = here.parent().filter(|up| !up.as_os_str().is_empty()) else {
        return here;
    };
    let Ok(entries) = std::fs::read_dir(above) else {
        return here;
    };

    let opened = crate::watch::resolve(opened);
    for entry in entries.flatten() {
        let candidate = entry.path();
        if kind_of(&candidate.to_string_lossy()) != Some(Kind::Markdown) || !candidate.is_file() {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&candidate) else {
            continue;
        };
        let names_it = md2pdf_core::section_paths(&text).is_ok_and(|sections| {
            sections
                .into_iter()
                .any(|section| crate::watch::resolve(&above.join(&section.path)) == opened)
        });
        if names_it {
            return above.to_path_buf();
        }
    }

    here
}

/// Every file under `root`, root-relative, in the panel's order.
///
/// **The order is total and computed here**, so the panel cannot reorder itself
/// between two compiles of the same tree: within each directory, files
/// alphabetically first, then subdirectories alphabetically, each expanded where
/// it sits. Byte-wise on the segment and not a locale collation — a
/// locale-dependent order is not reproducible by a second person, which is what
/// the exit gate needs it to be.
///
/// **The walk obeys the confinement rule, and that is not only a later phase's
/// concern.** A symlink under the root pointing at a directory elsewhere would
/// otherwise put that directory's files in the panel as though they were the
/// project's, and the phase after this one would open one of them in the pane.
/// Links are resolved *before* the check rather than after, and a directory
/// already visited is not visited again, so a link that loops back inside the
/// root costs one skip rather than a stack.
fn walk(root: &Path) -> Vec<String> {
    let real = crate::watch::resolve(root);
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    descend(&real, &real, "", &mut seen, &mut found);
    found
}

fn descend(
    root: &Path,
    here: &Path,
    prefix: &str,
    seen: &mut HashSet<PathBuf>,
    found: &mut Vec<String>,
) {
    if !seen.insert(here.to_path_buf()) {
        return;
    }
    let Ok(entries) = std::fs::read_dir(here) else {
        return;
    };

    let mut files: Vec<String> = Vec::new();
    let mut directories: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        let landed = crate::watch::resolve(&entry.path());
        if !landed.starts_with(root) {
            continue;
        }
        if landed.is_dir() {
            directories.push(name);
        } else {
            files.push(name);
        }
    }

    files.sort_unstable();
    directories.sort_unstable();

    for name in files {
        found.push(format!("{prefix}{name}"));
    }
    for name in directories {
        let below = crate::watch::resolve(&here.join(&name));
        descend(root, &below, &format!("{prefix}{name}/"), seen, found);
    }
}

/// The panel's order, as a comparison, so the union below sorts by the same rule
/// the walk emitted.
///
/// At the first segment the two paths differ on: a path with nothing left after
/// it is a file *in this directory* and sorts before one that still has
/// segments to go, which is a file in a subdirectory beside it. Otherwise the
/// segments are compared as bytes.
fn order(left: &str, right: &str) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let mut left = left.split('/');
    let mut right = right.split('/');

    loop {
        return match (left.next(), right.next()) {
            (Some(here), Some(there)) => {
                let (last_here, last_there) = (
                    left.clone().next().is_none(),
                    right.clone().next().is_none(),
                );
                if here == there && last_here == last_there {
                    continue;
                }
                match (last_here, last_there) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => here.as_bytes().cmp(there.as_bytes()),
                }
            }
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        };
    }
}

/// What the panel lists is [`files_under`] and [`merge`], and it is two
/// functions rather than one because the app runs them at two different rates.
///
/// **The union is the point.** A tree built from the disk alone loses the one
/// thing the panel this replaces was good at: a section the master names and the
/// disk does not hold is exactly the row the author needs to see. And splitting
/// it is what keeps the panel honest while a marker is half-typed — the disk
/// half is stable and only the marked-missing half moves, which is strictly less
/// motion than the shipped panel had.
///
/// The disk half: every file under `root` the pipeline can read.
///
/// **This is the half that costs a walk.** `crate::preview::Preview` holds its
/// answer and refreshes it at an open and at a `crate::watch::Change::Tree`
/// event, where [`merge`] below runs on every status. A panel that walked the
/// disk to answer a keystroke would put a `read_dir` per directory in front of
/// every character typed.
pub fn files_under(root: &Path) -> Vec<Entry> {
    walk(root)
        .into_iter()
        .filter_map(|path| {
            kind_of(&path).map(|kind| Entry {
                path,
                kind,
                missing: false,
            })
        })
        .collect()
}

/// The union: the files found, plus the paths `named` holds that they do not.
///
/// Pure, and cheap enough to run on every render — which is what lets the disk
/// half stay still while the marked-missing half follows the text.
///
/// `named` is root-relative, which is `crate::preview::Preview`'s job to make
/// it: `md2pdf_core::section_paths` answers relative to the master, and the
/// master need not sit at the root.
pub fn merge(found: Vec<Entry>, named: &[String]) -> Vec<Entry> {
    let mut entries = found;

    for path in named {
        if entries.iter().all(|entry| &entry.path != path) {
            entries.push(Entry {
                path: path.clone(),
                kind: Kind::Markdown,
                missing: true,
            });
        }
    }

    entries.sort_by(|left, right| order(&left.path, &right.path));
    entries
}

/// Every `.md` **directly in** `root` that names a section, in the panel's order.
///
/// **Discovery is total, so the common case needs no configuration.**
/// `md2pdf_core::section_paths` reads the master's own text and its body cannot
/// fail — it returns `Result` for signature symmetry with the two walks beside
/// it and constructs `Ok` unconditionally — so *"a `.md` here whose text names
/// section markers"* is a decidable test over every markdown file in one
/// directory.
///
/// **It does not recurse, and the reason is a property rather than a
/// preference.** A master cannot name a section above itself:
/// `core/src/emit.rs:landed_path` refuses a marker that climbs out of the
/// document's own folder, so `[](../a.md)` is not an include at all. Every
/// section therefore sits at or below its master's directory, which means the
/// master of the opened file is at the root or *above* it and never in a
/// subdirectory of it — [`project_root`]'s climb answers "above", and this
/// answers "at".
///
/// **A recursive walk got this wrong in the window, which is what the exit
/// gate's `samples/article.md` case is now here to catch.** `samples/` holds a
/// single-file document beside the whole `showcase/` project, so recursion
/// found `showcase/showcase.md`, called it the one master, and compiled it for
/// an author who had opened `article.md`. A `.md` in a subdirectory that names
/// sections is another project's master, not this root's.
pub fn masters(root: &Path) -> Vec<String> {
    walk(root)
        .into_iter()
        .filter(|path| !path.contains('/'))
        .filter(|path| kind_of(path) == Some(Kind::Markdown))
        .filter(|path| {
            std::fs::read_to_string(root.join(path))
                .ok()
                .and_then(|text| md2pdf_core::section_paths(&text).ok())
                .is_some_and(|sections| !sections.is_empty())
        })
        .collect()
}

/// Which file under `root` compiles, when nothing is remembered about it.
///
/// - **exactly one master → that file**, whatever the author opened;
/// - **no master → the file the author opened**, which is every single-file
///   document and is this app's behaviour before the panel existed;
/// - **more than one master → the opened file if it is itself one of them,
///   otherwise the byte-wise alphabetically first**, and the panel marks which
///   it landed on.
///
/// **This never leaves the main unset.** An empty pane and no page is a worse
/// answer than a guess the author can see and correct in one action, and the
/// mark in the panel is what makes the guess visible. Alphabetical is not a
/// claim about which is right — it is a claim that the same folder opens the
/// same way twice, which a set iteration order would not be.
pub fn discover_main(root: &Path, opened: &Path) -> String {
    let here = relative(root, opened).unwrap_or_else(|| {
        opened
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    });

    let mut masters = masters(root);
    match masters.len() {
        0 => here,
        1 => masters.remove(0),
        _ if masters.contains(&here) => here,
        _ => masters.remove(0),
    }
}

/// A path under `root`, spelled the way an [`Entry`] is: root-relative, `/`
/// separators, and `None` when it is not under the root at all.
///
/// Both sides are canonicalized, for [`crate::watch::classify`]'s reason: on
/// macOS the Open dialog hands over `/var/…` where the filesystem spells it
/// `/private/var/…`, so comparing them as they arrive matches nothing.
pub fn relative(root: &Path, path: &Path) -> Option<String> {
    spell(&crate::watch::resolve(root), &crate::watch::resolve(path))
}

/// [`relative`] without the canonicalization, for two paths this app built
/// from one root.
///
/// **It reads nothing off the disk, and that is why it is a function of its
/// own.** `relative` resolves both sides because it answers about a path that
/// came from outside — a command, a filesystem event — where the two callers
/// here already hold `root` and `root.join(…)` and have nothing to resolve.
/// `crate::preview::Preview::status` is one of them, and it runs on every
/// render: a `canonicalize` in there would put two syscalls in front of every
/// status and falsify that function's own stated invariant.
pub fn spell(root: &Path, path: &Path) -> Option<String> {
    let rest = path.strip_prefix(root).ok()?;

    let mut spelled = String::new();
    for part in rest.components() {
        if !spelled.is_empty() {
            spelled.push('/');
        }
        spelled.push_str(&part.as_os_str().to_string_lossy());
    }
    (!spelled.is_empty()).then_some(spelled)
}

/// How the master would name this file: [`beside`] run backwards.
///
/// **It is `beside`'s inverse and not a call to it.** That one takes a path the
/// master names to a root-relative one; this takes a root-relative path back to
/// the spelling `md2pdf_core::Location` carries, which is what the anchor filter
/// compares against. `None` is a file the master's own directory does not reach,
/// which has no such spelling at all — [`Pane::Away`], not [`Pane::Master`].
pub fn under(main: &Path, edited: &Path) -> Option<String> {
    spell(directory(main), edited)
}

/// One root-relative path joined onto the directory another sits in.
///
/// The master need not sit at the root, and `md2pdf_core::section_paths`
/// answers relative to the master — so a master at `parts/book.md` naming
/// `text.md` means the root-relative `parts/text.md`. A `..` is resolved
/// lexically, which is enough: `core/src/emit.rs`'s `landed_path` has already
/// refused any path that climbs out of the master's own folder.
pub fn beside(main: &str, named: &str) -> String {
    let mut segments: Vec<&str> = match main.rsplit_once('/') {
        Some((directory, _)) => directory.split('/').collect(),
        None => Vec::new(),
    };
    for part in named.split('/') {
        match part {
            "." | "" => {}
            ".." => {
                segments.pop();
            }
            part => segments.push(part),
        }
    }
    segments.join("/")
}

/// The bytes of one file under `root`, for the window to draw.
///
/// **It confines rather than checking existence**, which is
/// [`crate::preview::Session::set_main`]'s rule applied a third time: the path
/// comes from the panel, which got it from this app's own listing — but a
/// command is a command, and `root.join("../../secrets.png")` names a real file
/// on plenty of machines. [`relative`] canonicalizes both sides, so a path that
/// lands outside the root has no root-relative spelling to answer back, and one
/// that reaches outside through a symlink has none either. The refusal is the
/// sentence the other two use, so one refusal reads one way wherever it came
/// from.
///
/// **It is a function and not the command**, per this file's own header:
/// `app/src/main.rs` has no test module, the crate is bin-only and
/// `tauri::State` has a private field and no public constructor, so a rule
/// written into the command is a rule no test in this repository can reach.
/// `mpdf-010` Phase 5.
pub fn asset_bytes(root: &Path, path: &str) -> Result<Vec<u8>, String> {
    let landed = root.join(path);
    if !landed.is_file() || relative(root, &landed).as_deref() != Some(path) {
        return Err(format!("{path} is not a file in this project"));
    }

    std::fs::read(&landed).map_err(|e| format!("cannot read {}: {e}", landed.display()))
}

/// The one file the store lives in, inside the directory the platform gives
/// this app.
///
/// **Not a dotfile in the author's own folder**, and that was refused for two
/// reasons either of which is sufficient: it is the manifest
/// `specs/desktop_app_spec.md` §1.1 parks, arriving by another name; and it
/// writes a file into a directory the author may have under version control,
/// which this app has never done and should not start doing as a side effect of
/// a panel. The cost is accepted and stated: **the choice does not travel with
/// the files.**
pub fn store_file(support: &Path) -> PathBuf {
    support.join("projects.json")
}

/// What one root is remembered as compiling, keyed by the root's canonical path.
///
/// A `BTreeMap` so two writes of the same map produce the same bytes.
type Store = std::collections::BTreeMap<String, String>;

/// The store as it stands, or an empty one.
///
/// **A missing, unreadable or malformed store is nothing remembered, never an
/// error in the window.** The one fact in here is a convenience; a window that
/// refused to open a document because a JSON file in Application Support had
/// been truncated would be trading the whole app for it.
fn read_store(store: &Path) -> Store {
    std::fs::read_to_string(store)
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default()
}

/// Which file this root is remembered as compiling, if any.
pub fn read_override(store: &Path, root: &Path) -> Option<String> {
    read_store(store).remove(&key(root))
}

/// Remember that this root compiles `main`.
///
/// **A failed write is reported**, where a failed read is not: a set-main that
/// silently does not stick is worse than one that says why, and the author has
/// just asked for it in as many words.
pub fn write_override(store: &Path, root: &Path, main: &str) -> Result<(), String> {
    let mut held = read_store(store);
    held.insert(key(root), main.to_string());

    let text = serde_json::to_string_pretty(&held)
        .map_err(|e| format!("cannot write {}: {e}", store.display()))?;

    if let Some(parent) = store.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    std::fs::write(store, text).map_err(|e| format!("cannot write {}: {e}", store.display()))
}

/// A root as the store keys it: the path the filesystem really spells.
fn key(root: &Path) -> String {
    crate::watch::resolve(root).to_string_lossy().into_owned()
}

/// Read every file the document names, from beside the document.
///
/// This mirrors `cli/src/main.rs:read_assets`: a path resolves against the
/// directory of the open file, so a document, its figures and its bibliography
/// travel as one folder, and an asset keeps the path the markdown wrote,
/// because that is the name the generated Typst source asks for. The
/// duplication between the two wrappers is deliberate. A shared helper crate
/// for forty lines would buy less than it costs, and the two report their
/// errors differently, which is most of what those forty lines do.
///
/// The image list arrives in document order and may name one path twice, so
/// this reads each file once. The bibliography is one frontmatter value rather
/// than something the walk finds, so it comes from an export of its own — and
/// it is read first of the two, since the line it names is the earliest one in
/// the file.
///
/// **The sections are already read when this runs**, and they arrive here so
/// they ride out on the same array: neither list above can be asked for until
/// the document they belong to has been assembled, which is why
/// [`read_sections_with`] is a pass of its own and this one takes its result.
/// Their paths seed the same `seen` set, so no file is opened twice across the
/// two passes.
///
/// **Every path joins the master's directory, a section's own images
/// included.** `core` writes a section's own folder into the destination before
/// the list reaches here, so an image drawn in `sections/method.md` arrives as
/// `sections/figure.png` and is found beside the file that drew it — the rule
/// this app inherits rather than carries a copy of.
///
/// The read is a parameter for one gate. Phase 1 asks that a path the document
/// names twice is read *once*, and a caller that counts its own reads is the
/// only way to check that rather than argue it from the loop below.
fn read_assets_with(
    markdown: &str,
    sections: Vec<Asset>,
    directory: &Path,
    mut read: impl FnMut(&Path) -> std::io::Result<Vec<u8>>,
) -> Result<Vec<Asset>, String> {
    let images = md2pdf_core::image_paths(markdown, &sections).map_err(|e| e.to_string())?;
    let bibliography =
        md2pdf_core::bibliography_path(markdown, &sections).map_err(|e| e.to_string())?;

    let mut seen: HashSet<String> = sections.iter().map(|s| s.path.clone()).collect();
    let mut assets = sections;

    if let Some(named) = bibliography {
        let file = directory.join(&named.path);
        let bytes = read(&file).map_err(|e| {
            format!(
                "cannot read {} for the bibliography {}: {e}",
                file.display(),
                named.location
            )
        })?;

        seen.insert(named.path.clone());
        assets.push(Asset {
            path: named.path,
            bytes,
        });
    }

    for image in images {
        if !seen.insert(image.path.clone()) {
            continue;
        }

        let file = directory.join(&image.path);
        let bytes = read(&file).map_err(|e| {
            format!(
                "cannot read {} for the image {}: {e}",
                file.display(),
                image.location
            )
        })?;

        assets.push(Asset {
            path: image.path,
            bytes,
        });
    }
    Ok(assets)
}

/// Read every section file the master names, in the order it names them.
///
/// This mirrors `cli/src/main.rs:read_sections`, and it runs **before** either
/// shopping list for the reason that function records: the markers are in the
/// master's own text, so the sections can be read with no join, where every
/// later question is about the document they assemble into. One extra round
/// trip through `core`, no recursion here, and one place that ever
/// concatenates — which is `core`, because it is the joining that builds the
/// map every message is translated through.
///
/// The read is borrowed rather than taken, so [`read_assets_with`] goes on to
/// use the same closure. A section that will not open is the third of the
/// sentence the image and the bibliography already print, and the third this
/// app owes the terminal.
fn read_sections_with(
    markdown: &str,
    directory: &Path,
    mut read: impl FnMut(&Path) -> std::io::Result<Vec<u8>>,
) -> Result<Vec<Asset>, String> {
    let named = md2pdf_core::section_paths(markdown).map_err(|e| e.to_string())?;

    let mut sections = Vec::with_capacity(named.len());
    for section in named {
        let file = directory.join(&section.path);
        let bytes = read(&file).map_err(|e| {
            format!(
                "cannot read {} for the section {}: {e}",
                file.display(),
                section.location
            )
        })?;

        sections.push(Asset {
            path: section.path,
            bytes,
        });
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// [`render_with`] with the disk supplying every file and the pane holding
    /// the master, which is what every test below but the project's own means.
    fn render(directory: &Path, markdown: &str) -> Render {
        // The closure is not noise: `std::fs::read` names one lifetime where
        // the parameter asks for any, so passing it directly does not compile.
        render_with(directory, markdown, Pane::Master, |file| {
            std::fs::read(file)
        })
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../tests/fixtures")
            .join(name)
    }

    /// A scratch directory that this test process owns, so runs do not
    /// collide and the repository stays clean.
    fn scratch_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("md2pdf-app-test-{}", std::process::id()))
            .join(name);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A document and both the files it names, each read from beside it.
    ///
    /// `figures/mark.svg` pins that a path in a subdirectory resolves against
    /// the document's directory and not against the current one, and that the
    /// asset keeps the path the markdown wrote rather than the resolved one.
    #[test]
    fn a_document_and_its_images_read_as_two_assets() {
        let dir = scratch_dir("figure-doc");
        std::fs::copy(fixture("dot.png"), dir.join("dot.png")).unwrap();
        std::fs::create_dir_all(dir.join("figures")).unwrap();
        std::fs::copy(fixture("mark.svg"), dir.join("figures/mark.svg")).unwrap();

        let markdown = std::fs::read_to_string(fixture("figure.md")).unwrap();
        let assets =
            read_assets_with(&markdown, Vec::new(), &dir, |file| std::fs::read(file)).unwrap();

        let paths: Vec<&str> = assets.iter().map(|a| a.path.as_str()).collect();
        assert_eq!(paths, ["dot.png", "figures/mark.svg"]);
        assert!(assets.iter().all(|a| !a.bytes.is_empty()));
    }

    /// The same document beside no `figures/` directory, which is how
    /// `tests/fixtures/` actually stands: `dot.png` sits there and
    /// `figures/mark.svg` does not, so the second reference fails.
    #[test]
    fn a_missing_image_names_the_path_the_line_and_the_reason() {
        let markdown = std::fs::read_to_string(fixture("figure.md")).unwrap();
        let error = read_assets_with(&markdown, Vec::new(), &fixture(""), |file| {
            std::fs::read(file)
        })
        .unwrap_err();

        assert!(error.contains("figures/mark.svg"), "{error}");
        assert!(error.contains("line 5"), "{error}");
        assert!(error.contains("os error"), "{error}");
    }

    /// One path named twice is one asset and one read.
    ///
    /// The document is written here rather than taken from
    /// `tests/fixtures/images.md`, which repeats `dot.png` on lines 3 and 10
    /// but names `fig#2.png` on line 7: a reader fails there before it ever
    /// reaches the repeat.
    #[test]
    fn a_path_named_twice_is_read_once() {
        let dir = scratch_dir("repeated-image");
        std::fs::copy(fixture("dot.png"), dir.join("dot.png")).unwrap();

        let markdown = "![the first](dot.png)\n\nText between them.\n\n![the second](dot.png)\n";

        let mut reads = Vec::new();
        let assets = read_assets_with(markdown, Vec::new(), &dir, |file| {
            reads.push(file.to_path_buf());
            std::fs::read(file)
        })
        .unwrap();

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].path, "dot.png");
        assert_eq!(reads, [dir.join("dot.png")]);
    }

    /// A document and the bibliography it declares, read from beside it.
    ///
    /// The second asset channel at this app's own seam, mirroring
    /// `cli/src/main.rs:read_assets`: the bibliography is read **first**, and
    /// the asset keeps the path the frontmatter wrote rather than the resolved
    /// one. `tests/fixtures/citations.md` names no image, so the one asset is
    /// the whole list.
    #[test]
    fn a_document_and_its_bibliography_read_as_one_asset() {
        let dir = scratch_dir("bibliography-doc");
        std::fs::copy(fixture("refs.yml"), dir.join("refs.yml")).unwrap();

        let markdown = std::fs::read_to_string(fixture("citations.md")).unwrap();
        let mut reads = Vec::new();
        let assets = read_assets_with(&markdown, Vec::new(), &dir, |file| {
            reads.push(file.to_path_buf());
            std::fs::read(file)
        })
        .unwrap();

        assert_eq!(reads, [dir.join("refs.yml")]);
        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].path, "refs.yml");
        assert!(!assets[0].bytes.is_empty());
    }

    /// The same document beside no bibliography at all, which is how
    /// `scratch_dir` stands until something copies one in.
    ///
    /// The sentence is `cli/src/main.rs:read_assets`' own, word for word, and
    /// the line is the frontmatter's rather than any the walk could reach —
    /// a bibliography is one value the walk never meets.
    #[test]
    fn a_missing_bibliography_names_the_path_the_line_and_the_reason() {
        let dir = scratch_dir("bibliography-absent");
        let _ = std::fs::remove_file(dir.join("refs.yml"));

        let markdown = std::fs::read_to_string(fixture("citations.md")).unwrap();
        let error =
            read_assets_with(&markdown, Vec::new(), &dir, |file| std::fs::read(file)).unwrap_err();

        assert!(error.contains("refs.yml"), "{error}");
        assert!(error.contains("for the bibliography"), "{error}");
        assert!(error.contains("line 3"), "{error}");
        assert!(error.contains("os error"), "{error}");
    }

    /// A fixture compiled the way the pane compiles: its text as a string,
    /// against the directory it sits in.
    fn render_fixture(name: &str) -> Render {
        let document = fixture(name);
        let markdown = read_document(&document).unwrap();
        render(directory(&document), &markdown)
    }

    /// A construct outside the dialect names itself and its line, in the words
    /// the terminal uses. The window shows this sentence; that half is read by
    /// eye, and this is the half a test can hold.
    #[test]
    fn a_construct_outside_the_dialect_names_itself_and_its_line() {
        let error = render_fixture("unsupported_html.md").pdf.unwrap_err();

        assert!(error.contains("raw HTML block"), "{error}");
        assert!(error.contains("line 5"), "{error}");
    }

    /// A document that will not read names itself, in the sentence the
    /// terminal prints for the same file.
    #[test]
    fn a_document_that_will_not_read_names_the_path_and_the_reason() {
        let error = read_document(&fixture("no-such-document.md")).unwrap_err();

        assert!(error.contains("no-such-document.md"), "{error}");
        assert!(error.contains("os error"), "{error}");
    }

    /// A document that does not parse hands back no list, and the caller keeps
    /// the one it had. A document that parses but will not compile hands one
    /// back anyway, which is what keeps the watch filter alive while a figure
    /// is missing.
    #[test]
    fn the_asset_list_survives_a_failed_compile_but_not_a_failed_parse() {
        assert_eq!(render_fixture("unsupported_html.md").assets, None);

        let render = render_fixture("figure.md");
        assert!(render.pdf.is_err());
        assert_eq!(
            render.assets,
            Some(vec!["dot.png".to_string(), "figures/mark.svg".to_string()])
        );
    }

    /// A master beside no sections at all, in the words the terminal uses for
    /// the same file.
    ///
    /// The third of this app's hand-built sentences, beside the image's and the
    /// bibliography's, and word for word the one `cli/src/main.rs:read_sections`
    /// prints. A `SectionRef`'s location never carries a file — a section may
    /// not name a section — so the phrase is `at line N` and the line is the
    /// master's own.
    #[test]
    fn a_missing_section_names_the_path_the_line_and_the_reason() {
        let dir = scratch_dir("section-absent");
        let markdown = std::fs::read_to_string(fixture("multi_file.md")).unwrap();
        let error = read_sections_with(&markdown, &dir, |file| std::fs::read(file)).unwrap_err();

        assert!(error.contains("sections/introduction.md"), "{error}");
        assert!(error.contains("for the section"), "{error}");
        assert!(error.contains("at line 7"), "{error}");
        assert!(error.contains("os error"), "{error}");
    }

    /// Every file a master names is opened once, across both passes.
    ///
    /// [`a_path_named_twice_is_read_once`] extended to the channel that added a
    /// second pass over the same directory. One closure serves both, so a
    /// second one — or a section read again as an asset — shows up as an extra
    /// entry here rather than as an argument about the loops.
    ///
    /// The two images are named bare inside the sections and reach the list as
    /// `sections/dot.png` and `sections/mark.svg`, which is Phase 2's rule
    /// arriving in this wrapper with nothing added for it.
    #[test]
    fn every_file_a_master_names_is_read_once_across_both_passes() {
        let dir = fixture("");
        let markdown = std::fs::read_to_string(fixture("multi_file.md")).unwrap();

        let mut reads = Vec::new();
        let rendered = render_with(&dir, &markdown, Pane::Master, |file: &Path| {
            reads.push(file.to_path_buf());
            std::fs::read(file)
        });

        assert!(rendered.pdf.is_ok(), "{:?}", rendered.pdf.as_ref().err());
        assert_eq!(
            reads,
            [
                dir.join("sections/introduction.md"),
                dir.join("sections/method.md"),
                dir.join("sections/results.md"),
                dir.join("sections/dot.png"),
                dir.join("sections/mark.svg"),
            ]
        );
        assert_eq!(
            rendered.assets,
            Some(vec![
                "sections/introduction.md".to_string(),
                "sections/method.md".to_string(),
                "sections/results.md".to_string(),
                "sections/dot.png".to_string(),
                "sections/mark.svg".to_string(),
            ])
        );
    }

    /// The panel's list is the master's own, in the order the master reads it.
    ///
    /// [`every_file_a_master_names_is_read_once_across_both_passes`] already
    /// pins that order — as the *prefix of `assets`*, which is a list of paths
    /// to watch and not a list of parts to draw. This asserts the field the
    /// panel actually reads, so the two are checked to be one answer arriving
    /// twice rather than one assumed from the other. It is a plain `Vec` where
    /// `assets` is an `Option`, and the type is the claim: a panel draws what
    /// the text names, where `assets`' `None` says something about a watch
    /// filter that a panel cannot draw.
    #[test]
    fn the_sections_a_master_names_are_its_parts_in_master_order() {
        let dir = fixture("");
        let markdown = std::fs::read_to_string(fixture("multi_file.md")).unwrap();
        let rendered = render(&dir, &markdown);

        assert!(rendered.pdf.is_ok(), "{:?}", rendered.pdf.as_ref().err());
        assert_eq!(
            rendered.sections,
            [
                "sections/introduction.md",
                "sections/method.md",
                "sections/results.md",
            ]
        );
    }

    /// A master whose sections are not on the disk still names them.
    ///
    /// The list is read off the *text* and never off the read, which is what
    /// lets the panel name the parts of a document that will not compile —
    /// `md2pdf_core::section_paths` walks the markers and the asset walk only
    /// borrows the result. True by construction today and asserted here because
    /// an implementer who took the list from `read_sections_with`'s answer
    /// instead would pass every other case in this module and empty the panel
    /// exactly when the author most needs to see which file is missing.
    #[test]
    fn a_master_whose_sections_are_missing_still_names_them() {
        let dir = scratch_dir("sections-absent-still-named");
        let markdown = std::fs::read_to_string(fixture("multi_file.md")).unwrap();
        let rendered = render(&dir, &markdown);

        assert!(rendered.pdf.is_err(), "no section is on disk here");
        assert_eq!(
            rendered.sections,
            [
                "sections/introduction.md",
                "sections/method.md",
                "sections/results.md",
            ]
        );
    }

    /// A document that names no section names none, and that is an answer.
    ///
    /// `section_paths` cannot fail — its `Result` is symmetry with the two
    /// shopping lists, not a channel that carries anything — so an empty list
    /// is never a failure to answer and the panel is right to draw nothing.
    /// [`a_single_file_document_keeps_its_anchors_and_its_bytes`] asserts the
    /// `assets` analogue of this and is deliberately not the same claim: that
    /// list is `Some(Vec::new())`, an answer wrapped in the channel that can
    /// also say *no answer*, and this one has no such wrapper to be read
    /// through.
    #[test]
    fn a_document_that_names_no_section_has_no_parts() {
        let markdown = std::fs::read_to_string(fixture("basic.md")).unwrap();
        let rendered = render(&fixture(""), &markdown);

        assert!(rendered.pdf.is_ok(), "{:?}", rendered.pdf.as_ref().err());
        assert!(rendered.sections.is_empty());
    }

    /// Only the headings written in the file the pane holds become anchors.
    ///
    /// Both directions, because an unasserted absence is what
    /// `md2pdf_core::anchors_from`'s count guard punishes silently.
    /// `tests/fixtures/multi_file.md` is a pure manifest and `core` answers it
    /// with three anchors — `(sections/introduction.md, 1)`,
    /// `(sections/method.md, 4)` and `(sections/results.md, 1)`, pinned by
    /// `core/tests/golden_test.rs:an_anchor_names_the_file_its_heading_was_written_in`
    /// — so an empty list here is the filter working rather than `core`
    /// declining to answer. Left in, those three numbers would send
    /// `app/dist/index.html:caretPage`, which walks a flat list, to whatever
    /// page the last of them landed on.
    #[test]
    fn only_the_headings_the_pane_holds_become_anchors() {
        let manifest = render_fixture("multi_file.md");
        assert!(manifest.pdf.is_ok(), "{:?}", manifest.pdf.as_ref().err());
        assert!(manifest.anchors.is_empty(), "{:?}", manifest.anchors);

        let dir = scratch_dir("master-with-a-heading");
        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(
            dir.join("sections/one.md"),
            "# A heading the pane does not hold\n\nText.\n",
        )
        .unwrap();

        let markdown = "# A preface of the master's own\n\nText.\n\n[](sections/one.md)\n";
        let rendered = render(&dir, markdown);

        assert!(rendered.pdf.is_ok(), "{:?}", rendered.pdf.as_ref().err());
        assert_eq!(rendered.anchors, [Anchor { line: 1, page: 1 }]);
    }

    /// A single-file document is what it always was: the same anchors, and the
    /// same bytes.
    ///
    /// [`render_with`] now calls `md2pdf_core::section_paths` and threads a
    /// section array on *every* compile. A document naming no section has a
    /// one-entry map, so nothing is joined, nothing is prefixed and no anchor
    /// carries a file — arithmetic rather than a branch, and asserted here
    /// rather than assumed. The bytes are compared against a compile this test
    /// asked for itself, so an app that quietly agreed with itself could not
    /// pass.
    #[test]
    fn a_single_file_document_keeps_its_anchors_and_its_bytes() {
        let markdown = std::fs::read_to_string(fixture("basic.md")).unwrap();
        let rendered = render(&fixture(""), &markdown);

        let lines: Vec<usize> = rendered.anchors.iter().map(|anchor| anchor.line).collect();
        assert_eq!(lines, [1, 5, 10, 14, 16, 18]);
        assert_eq!(rendered.assets, Some(Vec::new()));
        assert_eq!(
            rendered.pdf.unwrap(),
            md2pdf_core::md_to_pdf(&markdown, &[]).unwrap()
        );
    }

    #[test]
    fn the_title_is_the_documents_file_name() {
        assert_eq!(title(Path::new("/tmp/notes/paper.md")), "paper.md");
    }

    // -- the project: root, discovery, listing, store ----------------------
    //
    // `mpdf-010` Phase 1's exit gate, clauses 1 through 6. The fixture is
    // `tests/fixtures/panel/`, created by that phase: `samples/showcase/`
    // gains a `showcase.pdf` for any developer who has run its own README, and
    // `pdf` is in `IMAGE_EXTENSIONS`, so an exact-enumeration gate over the
    // sample tree is not reproducible by a second person.

    /// Clause 1. **The root climbs, and it climbs exactly once.**
    ///
    /// The first case is the whole phase's observable: opening a section from
    /// Finder must find the master above it, or the window compiles one section
    /// standalone as it does today.
    ///
    /// **The last case asserts the cap rather than accepting it.**
    /// `parts/ch1/deep.md` is named by `book.md` as `[](parts/ch1/deep.md)` — a
    /// supported marker, not merely an unrefused one — and it still roots at
    /// `parts/ch1`, because `parts/` holds no markdown for the climb to find.
    /// That is `specs/file_panel_spec.md` §2's stated cost, pinned here so a
    /// later change to the climb has to change this line deliberately.
    #[test]
    fn the_root_climbs_one_level_and_stops() {
        let panel = fixture("panel");

        assert_eq!(project_root(&panel.join("sections/text.md")), panel);
        assert_eq!(project_root(&panel.join("book.md")), panel);
        assert_eq!(
            project_root(&panel.join("loose/orphan.md")),
            panel.join("loose"),
            "no `.md` above `loose/` names it, so it is its own root"
        );
        assert_eq!(
            project_root(&panel.join("parts/ch1/deep.md")),
            panel.join("parts/ch1"),
            "the cap: `parts/` holds no markdown, so the climb finds nothing"
        );
    }

    /// Clause 2. Discovery is the `.md` files whose text names sections.
    #[test]
    fn discovery_is_every_markdown_that_names_a_section() {
        assert_eq!(masters(&fixture("panel")), ["book.md"]);
        assert_eq!(masters(&fixture("panel/loose")), Vec::<String>::new());
        assert_eq!(masters(&fixture("panel-pair")), ["alpha.md", "beta.md"]);
    }

    /// **A master in a subdirectory belongs to another project, not this root.**
    ///
    /// `samples/` is the case that proves it and the case that caught it: a
    /// single-file document sits there beside the whole `showcase/` project, so
    /// a discovery that recursed found `showcase/showcase.md`, called it the one
    /// master, and compiled the showcase for an author who opened
    /// `article.md` — which the window gate reported and no fixture had.
    ///
    /// It is a property rather than a preference. A master cannot name a
    /// section above itself, `landed_path` refusing a marker that climbs out of
    /// the document's folder, so every section sits at or below its master and
    /// a master is never below its own sections. `project_root` answers
    /// "above"; this answers "at"; below is somebody else's document.
    ///
    /// **The real tree, not a fixture**, because the defect was that the
    /// fixtures were all tidier than the repository the app is developed in.
    #[test]
    fn a_master_in_a_subdirectory_is_not_this_roots_master() {
        let samples = Path::new(env!("CARGO_MANIFEST_DIR")).join("../samples");

        assert!(
            samples.join("showcase/showcase.md").is_file(),
            "the tree this case is about has moved"
        );
        assert_eq!(
            masters(&samples),
            Vec::<String>::new(),
            "a project one directory down was taken for this root's own"
        );
        assert_eq!(
            discover_main(&samples, &samples.join("article.md")),
            "article.md",
            "opening a single-file document compiled somebody else's master"
        );

        // And the showcase, opened as itself, still finds its own.
        let showcase = samples.join("showcase");
        assert_eq!(masters(&showcase), ["showcase.md"]);
        assert_eq!(
            discover_main(&showcase, &showcase.join("sections/text.md")),
            "showcase.md"
        );
    }

    /// Clause 3's discovery half. The store is read by the session, not here;
    /// what this pins is the answer when nothing is remembered.
    #[test]
    fn the_main_is_the_one_master_or_the_first_of_several() {
        let panel = fixture("panel");
        let pair = fixture("panel-pair");

        // One master: whatever the author opened, the master compiles.
        assert_eq!(
            discover_main(&panel, &panel.join("sections/text.md")),
            "book.md"
        );
        assert_eq!(discover_main(&panel, &panel.join("book.md")), "book.md");

        // None: the file the author opened, which is every single-file document
        // and is this app's behaviour before the panel existed.
        let loose = panel.join("loose");
        assert_eq!(discover_main(&loose, &loose.join("orphan.md")), "orphan.md");

        // Several: the opened file when it is one of them, else the byte-wise
        // first — a claim that the same folder opens the same way twice, not a
        // claim about which is right.
        assert_eq!(discover_main(&pair, &pair.join("note.md")), "alpha.md");
        assert_eq!(discover_main(&pair, &pair.join("beta.md")), "beta.md");
    }

    /// Clause 4. **The listing, against the manifest beside the fixture.**
    ///
    /// The manifest is a `.txt` and sits in `tests/fixtures/` rather than in
    /// `tests/fixtures/panel/`, so it cannot become a row in the listing it
    /// defines.
    ///
    /// `sections/missing.md` is handed in as a path the master names. `book.md`
    /// deliberately does not name it: a master naming a file the disk lacks
    /// refuses with `MissingSection`, and the byte-for-byte claim in
    /// `crate::preview`'s own test needs `book.md` to compile.
    #[test]
    fn the_listing_is_the_disk_and_what_the_master_names() {
        let named = [
            "sections/text.md".to_string(),
            "parts/ch1/deep.md".to_string(),
            "sections/missing.md".to_string(),
        ];
        let listed = merge(files_under(&fixture("panel")), &named);

        let spelled: Vec<(&str, Kind, bool)> = listed
            .iter()
            .map(|entry| (entry.path.as_str(), entry.kind, entry.missing))
            .collect();

        assert_eq!(
            spelled,
            [
                ("book.md", Kind::Markdown, false),
                ("cover.jpg", Kind::Image, false),
                ("other.md", Kind::Markdown, false),
                ("plan.pdf", Kind::Image, false),
                ("refs.bib", Kind::Bibliography, false),
                ("refs.yml", Kind::Bibliography, false),
                ("loose/orphan.md", Kind::Markdown, false),
                ("parts/ch1/deep.md", Kind::Markdown, false),
                ("sections/mark.svg", Kind::Image, false),
                ("sections/missing.md", Kind::Markdown, true),
                ("sections/text.md", Kind::Markdown, false),
            ],
            "the listing and `tests/fixtures/panel-manifest.txt` disagree"
        );
    }

    /// Clause 5. **Confinement, tested where it can fail.**
    ///
    /// `tests/fixtures/panel/outside` is a symlink to
    /// `tests/fixtures/panel-decoy/`, a committed sibling so the link resolves
    /// the same on any clone. The target holds a `.md` and a `.png` that both
    /// match the filter — a link to a directory holding nothing it matched
    /// would pass under an implementation with no confinement at all, which is
    /// why the target holds two files that do.
    #[test]
    fn the_walk_does_not_follow_a_link_out_of_the_root() {
        let decoy = fixture("panel-decoy");
        assert!(
            fixture("panel/outside").is_dir(),
            "the fixture's symlink did not survive the checkout"
        );
        assert_eq!(
            files_under(&decoy)
                .iter()
                .map(|entry| entry.path.as_str())
                .collect::<Vec<_>>(),
            ["decoy.md", "decoy.png"],
            "the decoy must hold rows the filter matches, or the clause below proves nothing"
        );

        assert!(
            files_under(&fixture("panel"))
                .iter()
                .all(|entry| !entry.path.starts_with("outside")),
            "a link out of the root put its target's files in the panel"
        );
    }

    /// Clause 6. The store round-trips, and a truncated one is nothing
    /// remembered rather than an error in the window.
    #[test]
    fn the_store_remembers_one_fact_per_root_and_forgives_a_bad_file() {
        let dir = scratch_dir("store");
        let store = store_file(&dir);
        let root = fixture("panel");
        let other = fixture("panel-pair");

        assert_eq!(read_override(&store, &root), None, "a store with no file");

        write_override(&store, &root, "other.md").unwrap();
        write_override(&store, &other, "beta.md").unwrap();
        assert_eq!(read_override(&store, &root).as_deref(), Some("other.md"));
        assert_eq!(read_override(&store, &other).as_deref(), Some("beta.md"));

        // A second write of the same root replaces rather than accumulates.
        write_override(&store, &root, "book.md").unwrap();
        assert_eq!(read_override(&store, &root).as_deref(), Some("book.md"));
        assert_eq!(read_override(&store, &other).as_deref(), Some("beta.md"));

        std::fs::write(&store, "{\"/some/root\": ").unwrap();
        assert_eq!(
            read_override(&store, &root),
            None,
            "a truncated store is nothing remembered, and never an error"
        );
    }

    /// The path arithmetic the panel's union needs: a master that does not sit
    /// at the root names its sections relative to itself.
    #[test]
    fn a_section_is_named_beside_the_master_that_reads_it() {
        assert_eq!(beside("book.md", "sections/text.md"), "sections/text.md");
        assert_eq!(beside("parts/book.md", "text.md"), "parts/text.md");
        assert_eq!(
            beside("parts/book.md", "../shared/text.md"),
            "shared/text.md"
        );
    }

    // -- the pane and the main are two files -------------------------------
    //
    // `mpdf-010` Phase 2. The compile is the master's and the pane holds one
    // file of it, so three things have to be right: which text compiles, which
    // file's bytes the buffer stands in for, and whose headings become anchors.

    /// `beside` run backwards, and the file it cannot reach.
    #[test]
    fn a_file_in_the_pane_is_named_the_way_the_master_would_name_it() {
        let root = Path::new("/p");

        assert_eq!(
            under(&root.join("book.md"), &root.join("sections/text.md")).as_deref(),
            Some("sections/text.md")
        );
        assert_eq!(
            under(&root.join("parts/book.md"), &root.join("parts/text.md")).as_deref(),
            Some("text.md"),
            "a master in a subdirectory names its neighbour by its bare name"
        );
        assert_eq!(
            under(&root.join("parts/book.md"), &root.join("README.md")),
            None,
            "a file the master's own directory does not reach has no such spelling"
        );
        assert_eq!(
            spell(root, &root.join("sections/text.md")).as_deref(),
            Some("sections/text.md")
        );
        assert_eq!(spell(root, root), None, "the root is not under itself");
    }

    /// Whose headings become anchors, all three answers.
    ///
    /// **The third is why [`Pane`] is not an `Option`.** `Master` and `Away`
    /// would collapse into one absence, and the master's own heading lines would
    /// then be handed to a buffer whose line 1 is a different sentence
    /// altogether — which is the exact defect this filter exists to prevent,
    /// arriving through the case that looks like nothing.
    #[test]
    fn the_anchors_are_the_headings_of_the_file_the_pane_is_holding() {
        let dir = scratch_dir("pane-arms");
        std::fs::create_dir_all(dir.join("sections")).unwrap();
        std::fs::write(
            dir.join("sections/one.md"),
            "\n# The section's own heading\n\nText.\n",
        )
        .unwrap();

        let markdown = "# A preface of the master's own\n\nText.\n\n[](sections/one.md)\n";
        let lines = |pane| {
            render_with(&dir, markdown, pane, |file: &Path| std::fs::read(file))
                .anchors
                .into_iter()
                .map(|anchor| anchor.line)
                .collect::<Vec<usize>>()
        };

        assert_eq!(lines(Pane::Master), [1], "the master's own heading");
        assert_eq!(
            lines(Pane::Beside("sections/one.md")),
            [2],
            "the section's own heading, at its own line inside its own file"
        );
        assert!(
            lines(Pane::Away).is_empty(),
            "no heading in this document is a number about a file it does not reach"
        );
    }

    /// The pane's buffer stands in for the one file it is holding, and the rest
    /// of the document comes off the disk.
    ///
    /// The claim is checked against a compile this test asked for itself, after
    /// putting the buffer on the disk — so an app that quietly agreed with
    /// itself could not pass.
    #[test]
    fn the_panes_buffer_stands_in_for_the_file_it_is_holding() {
        let dir = scratch_dir("pane-override");
        std::fs::create_dir_all(dir.join("sections")).unwrap();

        let main = dir.join("book.md");
        let edited = dir.join("sections/one.md");
        std::fs::write(&main, "# Book\n\n[](sections/one.md)\n").unwrap();
        std::fs::write(&edited, "# On disk\n\nThe disk's own text.\n").unwrap();

        let buffer = "# In the pane\n\nText nobody has saved.\n";
        let unsaved = render_project(&main, &edited, buffer).expect("the master would not read");

        std::fs::write(&edited, buffer).unwrap();
        let master = std::fs::read_to_string(&main).unwrap();
        let saved = render_project(&main, &main, &master).expect("the master would not read");

        assert_eq!(
            unsaved.pdf.expect("the unsaved compile failed"),
            saved.pdf.expect("the saved compile failed"),
            "the buffer reached the compile, or it did not"
        );
    }

    /// A `main` this app cannot read fails in [`read_document`]'s own sentence.
    ///
    /// Both classes, because the closure yields bytes where the compile wants a
    /// string and only one of the two is an `io::Error` to begin with. The
    /// messages are compared against `read_document`'s rather than spelled out
    /// again, which is the claim: a main that will not read reads the same in
    /// the window whichever path reached it.
    ///
    /// **Only reachable while the pane holds another file**, and that is the
    /// mechanism working rather than a gap in the test. With the pane on the
    /// main the closure answers from the buffer and never touches the disk, so
    /// there is nothing there to fail — the buffer is the document, which is
    /// what this app has meant since `mpdf-003` Phase 2.
    #[test]
    fn a_main_that_will_not_read_says_so_in_the_terminals_own_words() {
        let dir = scratch_dir("pane-unreadable");
        let pane = dir.join("held.md");
        std::fs::write(&pane, "# Held\n").unwrap();

        let missing = dir.join("nothing.md");
        assert_eq!(
            render_project(&missing, &pane, "# Held\n").err(),
            read_document(&missing).err()
        );

        let binary = dir.join("binary.md");
        std::fs::write(&binary, [0xff, 0xfe, 0x00]).unwrap();
        assert_eq!(
            render_project(&binary, &pane, "# Held\n").err(),
            read_document(&binary).err()
        );

        assert!(
            render_project(&missing, &missing, "# Text the disk never held\n").is_ok(),
            "the pane holds the main, so the disk is not consulted at all"
        );
    }

    // -- one of the project's files, read for the window to draw ------------
    //
    // `mpdf-010` Phase 5. The panel has listed the project's figures since
    // Phase 1 and could not show one. The read is here rather than in the
    // command for this file's own reason, and these three clauses are what
    // that split buys.

    /// Clause 1. The bytes are the file's, whatever kind of figure it is.
    #[test]
    fn a_figure_reads_back_the_bytes_the_disk_holds() {
        let root = fixture("panel");
        for path in ["cover.jpg", "sections/mark.svg"] {
            assert_eq!(
                asset_bytes(&root, path).unwrap(),
                std::fs::read(root.join(path)).unwrap(),
                "{path} did not come back as the disk holds it"
            );
        }
    }

    /// Clause 2. A path that leaves the project is refused, and the second
    /// case is the one that can only be refused by confinement.
    ///
    /// `outside/decoy.png` is a file the disk really holds — through
    /// `tests/fixtures/panel/outside`, the committed symlink to
    /// `tests/fixtures/panel-decoy/` that the walk's own clause uses — so
    /// `is_file()` cannot refuse it and only the root-relative spelling can.
    #[test]
    fn a_figure_outside_the_project_is_refused_by_name() {
        let root = fixture("panel");

        assert!(
            root.join("outside/decoy.png").is_file(),
            "the decoy must be reachable through the link, or this clause proves nothing"
        );

        for path in ["/tmp/escape.png", "outside/decoy.png"] {
            assert_eq!(
                asset_bytes(&root, path).err().as_deref(),
                Some(format!("{path} is not a file in this project").as_str()),
                "{path} was not refused in the sentence the other two refusals use"
            );
        }
    }

    /// Clause 3. A `..` is refused **by the rule and not by the file being
    /// absent**, which is why the test writes the file it asks for.
    ///
    /// It runs over a scratch root rather than the fixture: `escape.png` has to
    /// exist in the root's *parent* for `is_file()` to pass and the
    /// confinement to be what refuses, and `tests/fixtures/` is tracked —
    /// [`scratch_dir`]'s own reason for existing.
    #[test]
    fn a_figure_reached_by_climbing_out_of_the_root_is_refused() {
        let above = scratch_dir("figure-escape");
        let root = above.join("project");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(above.join("escape.png"), b"not the project's").unwrap();

        assert!(
            root.join("../escape.png").is_file(),
            "the file has to be there, or `is_file()` refuses before the rule runs"
        );
        assert_eq!(
            asset_bytes(&root, "../escape.png").err().as_deref(),
            Some("../escape.png is not a file in this project")
        );
    }
}
