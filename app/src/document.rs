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
    /// **Only the headings written in the file the pane holds**, which is the
    /// master. A line means something in exactly one buffer and the pane holds
    /// one, so an anchor from a section is not a worse match — it is a number
    /// about a document the pane is not showing, and
    /// `app/dist/index.html:caretPage` walks a flat list and breaks at the
    /// first anchor past the caret. Left in, three sections numbered 1, 4 and 1
    /// would open the frame on whatever page the last of them landed on.
    ///
    /// A pure manifest therefore yields none and the frame opens at page 1,
    /// which `caretPage` already documents as its no-anchor case; a master
    /// carrying a preface syncs on its own headings and syncs correctly.
    ///
    /// Empty when the compile failed, and empty when `core`'s own count guard
    /// declined to answer. Unlike [`Render::assets`] this describes the *page*
    /// rather than the text, so it is only ever as good as the bytes beside it.
    pub anchors: Vec<Anchor>,
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
pub fn render(directory: &Path, markdown: &str) -> Render {
    // The closure is not noise: `std::fs::read` names one lifetime where the
    // parameter below asks for any, so passing it directly does not compile.
    render_with(directory, markdown, |file| std::fs::read(file))
}

/// [`render`], with the file read supplied by the caller.
///
/// The seam is Phase 1's [`read_assets_with`], one level up, and it exists for
/// the same reason: a caller that counts its own reads can check a claim about
/// them rather than argue it from the loop.
///
/// **One closure serves both passes**, and that is what keeps the claim worth
/// checking. [`read_sections_with`] borrows it and [`read_assets_with`] takes
/// what is left, so every file this app opens for one compile goes through the
/// counter — a second closure would leave half the reads unwatched.
pub fn render_with(
    directory: &Path,
    markdown: &str,
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
    // above survives one, because it describes the text. An anchor carrying a
    // file was written in a section, and the pane is not showing that file;
    // [`Render::anchors`] argues why dropping it is the only answer that is
    // true by construction.
    let (pdf, anchors) = match rendered {
        Ok(rendered) => (
            Ok(rendered.pdf),
            rendered
                .anchors
                .into_iter()
                .filter(|anchor| anchor.location.file.is_none())
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
        let rendered = render_with(&dir, &markdown, |file| {
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
}
