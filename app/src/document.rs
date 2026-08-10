//! Everything this app decides without a window.
//!
//! The read, the compile, the title and the error string all live here, in
//! ordinary functions with ordinary tests. Only what genuinely needs a window
//! goes through Tauri's command layer, because a GUI whose logic is reachable
//! only by clicking has no exit gate but a screenshot.

use std::collections::HashSet;
use std::path::Path;

use md2pdf_core::Asset;

/// Read one document and the image files it names, then compile it to a PDF.
///
/// Every failure arrives as the sentence the CLI prints after its `error: `
/// prefix, and the two classes are not the same type. A construct outside the
/// dialect is a `md2pdf_core::Error` and reaches the page through its
/// `Display`; a file that will not read is no `Error` at all, and
/// [`read_assets`] builds the plain sentence for it. So a document this app
/// refuses is refused in the same words at the window and at the terminal.
pub fn render(document: &Path) -> Result<Vec<u8>, String> {
    let markdown = std::fs::read_to_string(document)
        .map_err(|e| format!("cannot read {}: {e}", document.display()))?;

    let directory = document.parent().unwrap_or(Path::new(""));
    let assets = read_assets(&markdown, directory)?;

    md2pdf_core::md_to_pdf(&markdown, &assets).map_err(|e| e.to_string())
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
pub fn read_assets(markdown: &str, directory: &Path) -> Result<Vec<Asset>, String> {
    // The closure is not noise: `std::fs::read` names one lifetime where the
    // parameter below asks for any, so passing it directly does not compile.
    read_assets_with(markdown, directory, |file| std::fs::read(file))
}

/// [`read_assets`], with the file read supplied by the caller.
///
/// The seam exists for one gate. Phase 1 asks that a path the document names
/// twice is read *once*, and a caller that counts its own reads is the only
/// way to check that rather than argue it from the loop below.
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
        let assets = read_assets(&markdown, &dir).unwrap();

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
        let error = read_assets(&markdown, &fixture("")).unwrap_err();

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

    /// A construct outside the dialect names itself and its line, in the words
    /// the terminal uses. The window shows this sentence; that half is read by
    /// eye, and this is the half a test can hold.
    #[test]
    fn a_construct_outside_the_dialect_names_itself_and_its_line() {
        let error = render(&fixture("unsupported_html.md")).unwrap_err();

        assert!(error.contains("raw HTML block"), "{error}");
        assert!(error.contains("line 5"), "{error}");
    }

    #[test]
    fn the_title_is_the_documents_file_name() {
        assert_eq!(title(Path::new("/tmp/notes/paper.md")), "paper.md");
    }
}
