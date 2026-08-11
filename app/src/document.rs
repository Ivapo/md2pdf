//! Everything this app decides without a window.
//!
//! The read, the compile, the title and the error string all live here, in
//! ordinary functions with ordinary tests. Only what genuinely needs a window
//! goes through Tauri's command layer, because a GUI whose logic is reachable
//! only by clicking has no exit gate but a screenshot.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use md2pdf_core::Asset;

/// What one compile produced, and what the document named while producing it.
pub struct Render {
    /// The image paths the document names, in reader order.
    ///
    /// `None` when the document did not parse, and the caller then keeps the
    /// list it already had. Otherwise it arrives even when the compile failed,
    /// because emission reads the text and not the disk: a document whose
    /// figures are all missing still names them. That is what keeps the watch
    /// filter working while the compile does not.
    pub images: Option<Vec<String>>,

    /// The bytes, or the sentence the terminal would print.
    pub pdf: Result<Vec<u8>, String>,
}

/// Compile one markdown string, reading the image files it names from beside
/// the document.
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
pub fn render_with(
    directory: &Path,
    markdown: &str,
    read: impl FnMut(&Path) -> std::io::Result<Vec<u8>>,
) -> Render {
    let images = md2pdf_core::image_paths(markdown)
        .ok()
        .map(|images| images.into_iter().map(|image| image.path).collect());

    let pdf = read_assets_with(markdown, directory, read)
        .and_then(|assets| md2pdf_core::md_to_pdf(markdown, &assets).map_err(|e| e.to_string()));

    Render { images, pdf }
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

/// The directory a document's figures resolve against: the one it sits in.
///
/// An empty parent is a document named with no directory at all, and joining a
/// figure onto `""` resolves it against the working directory, which is what
/// the CLI does for the same input.
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

/// Read every image file the document names, from beside the document.
///
/// This mirrors `cli/src/main.rs:read_assets`: a path resolves against the
/// directory of the open file, so a document and its figures travel as one
/// folder, and an asset keeps the path the markdown wrote, because that is the
/// name the generated Typst source asks for. The duplication between the two
/// wrappers is deliberate. A shared helper crate for forty lines would buy
/// less than it costs, and the two report their errors differently, which is
/// most of what those forty lines do.
///
/// The list arrives in document order and may name one path twice, so this
/// reads each file once.
///
/// The read is a parameter for one gate. Phase 1 asks that a path the document
/// names twice is read *once*, and a caller that counts its own reads is the
/// only way to check that rather than argue it from the loop below.
fn read_assets_with(
    markdown: &str,
    directory: &Path,
    mut read: impl FnMut(&Path) -> std::io::Result<Vec<u8>>,
) -> Result<Vec<Asset>, String> {
    let images = md2pdf_core::image_paths(markdown).map_err(|e| e.to_string())?;

    let mut assets = Vec::new();
    let mut seen = HashSet::new();
    for image in images {
        if !seen.insert(image.path.clone()) {
            continue;
        }

        let file = directory.join(&image.path);
        let bytes = read(&file).map_err(|e| {
            format!(
                "cannot read {} for the image at line {}: {e}",
                file.display(),
                image.line
            )
        })?;

        assets.push(Asset {
            path: image.path,
            bytes,
        });
    }
    Ok(assets)
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
        let assets = read_assets_with(&markdown, &dir, |file| std::fs::read(file)).unwrap();

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
        let error =
            read_assets_with(&markdown, &fixture(""), |file| std::fs::read(file)).unwrap_err();

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
        let assets = read_assets_with(markdown, &dir, |file| {
            reads.push(file.to_path_buf());
            std::fs::read(file)
        })
        .unwrap();

        assert_eq!(assets.len(), 1);
        assert_eq!(assets[0].path, "dot.png");
        assert_eq!(reads, [dir.join("dot.png")]);
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
    fn the_image_list_survives_a_failed_compile_but_not_a_failed_parse() {
        assert_eq!(render_fixture("unsupported_html.md").images, None);

        let render = render_fixture("figure.md");
        assert!(render.pdf.is_err());
        assert_eq!(
            render.images,
            Some(vec!["dot.png".to_string(), "figures/mark.svg".to_string()])
        );
    }

    #[test]
    fn the_title_is_the_documents_file_name() {
        assert_eq!(title(Path::new("/tmp/notes/paper.md")), "paper.md");
    }
}
