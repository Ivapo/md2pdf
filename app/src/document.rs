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
