//! `md2pdf-core`, compiled to `wasm32-unknown-unknown` and called from a page.
//!
//! The crate holds no OS access by design, which `mpdf-001` §1.1 said would
//! buy a browser build without a rewrite. This tested that claim and needed no
//! change to `core` at all — and `mpdf-006` then made the page a front door
//! rather than an experiment. **The PDF a visitor sees is the PDF the CLI
//! writes**: `render` below calls the same `md2pdf_core::md_to_pdf`, from the
//! same crate, byte for byte.
//!
//! **Two files cross this boundary, and both are the page's rather than the
//! reader's.** `mpdf-006` Phase 3 answered the question the spike parked here
//! with one image carried inline; `mpdf-007` Phase 4 sent a bibliography down
//! the same channel, so the caption-over-an-image case and the reference list
//! both reach the pane. They arrive as scalar pairs rather than an array
//! deliberately: `web/Cargo.toml` carries `wasm-bindgen` alone, so a
//! `Vec<Vec<u8>>` across this boundary is a new dependency on a page whose
//! entire cost is its 7.8 MB — and the set is closed, `mpdf-006` §1.2 parking
//! a reader's own files permanently.
//!
//! A browser still has no filesystem, so `md2pdf_core::image_paths`' shopping
//! list and `md2pdf_core::bibliography_path`'s second half still have nowhere
//! to be read from — a *reader's own* files are the "different file story"
//! `mpdf-001` §1.1 named, and that stays parked.

use wasm_bindgen::prelude::*;

/// Route a Rust panic to the console instead of an unhelpful `unreachable`.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Compile markdown to PDF bytes, with the page's two files in hand.
///
/// Each path is the name as the markdown writes it — an image destination, and
/// the frontmatter's `bibliography:` value — and each `bytes` is that file. The
/// page reads all four off its own `data-asset` elements, so the names it passes
/// and the names its examples write are one string apiece. **Every compile
/// passes both** — the typing path and the row buttons alike — because
/// `md2pdf_core::md_to_pdf` ignores an asset the document never names, and a
/// channel open to only one of the two acts would draw the figure on a click
/// and refuse it on the next keystroke.
///
/// The error is the same sentence the CLI prints after its `error: ` prefix,
/// because it is the same `Display` — a construct outside the dialect is
/// refused here in the words it is refused at the terminal.
#[wasm_bindgen]
pub fn render(
    markdown: &str,
    asset_path: &str,
    asset_bytes: &[u8],
    bibliography_path: &str,
    bibliography_bytes: &[u8],
) -> Result<Vec<u8>, JsError> {
    let assets = assets(
        asset_path,
        asset_bytes,
        bibliography_path,
        bibliography_bytes,
    );

    md2pdf_core::md_to_pdf(markdown, &assets).map_err(|e| JsError::new(&e.to_string()))
}

/// The page each heading landed on, as `line:page` pairs.
///
/// This is `mpdf-003` Phase 6's export, answered in a browser. The desktop app
/// uses it to open the pane on the page the author's caret is in; the page no
/// longer calls it, and it stays because it is that phase's export rather than
/// this page's.
///
/// **It takes the same two files `render` does.** It passed `&[]` until Phase 4,
/// which meant it could answer for neither of the page's own rows — the image
/// one came back `MissingImage` and a citation one would come back
/// `MissingBibliography`. An export that cannot answer for the source beside it
/// is a hole in that phase's browser answer rather than a limit worth recording.
#[wasm_bindgen]
pub fn anchors(
    markdown: &str,
    asset_path: &str,
    asset_bytes: &[u8],
    bibliography_path: &str,
    bibliography_bytes: &[u8],
) -> Result<String, JsError> {
    let assets = assets(
        asset_path,
        asset_bytes,
        bibliography_path,
        bibliography_bytes,
    );

    let rendered = md2pdf_core::md_to_pdf_with_anchors(markdown, &assets)
        .map_err(|e| JsError::new(&e.to_string()))?;

    Ok(rendered
        .anchors
        .iter()
        .map(|anchor| format!("{}:{}", anchor.location.line, anchor.page))
        .collect::<Vec<_>>()
        .join(" "))
}

/// The page's two files as `md2pdf_core` takes them.
///
/// One place rather than two, so the exports above cannot disagree about the
/// order or about which scalar is which path.
fn assets(
    asset_path: &str,
    asset_bytes: &[u8],
    bibliography_path: &str,
    bibliography_bytes: &[u8],
) -> [md2pdf_core::Asset; 2] {
    [
        md2pdf_core::Asset {
            path: asset_path.to_string(),
            bytes: asset_bytes.to_vec(),
        },
        md2pdf_core::Asset {
            path: bibliography_path.to_string(),
            bytes: bibliography_bytes.to_vec(),
        },
    ]
}
