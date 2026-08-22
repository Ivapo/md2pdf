//! `md2pdf-core`, compiled to `wasm32-unknown-unknown` and called from a page.
//!
//! The crate holds no OS access by design, which `mpdf-001` §1.1 said would
//! buy a browser build without a rewrite. This tested that claim and needed no
//! change to `core` at all — and `mpdf-006` then made the page a front door
//! rather than an experiment. **The PDF a visitor sees is the PDF the CLI
//! writes**: `render` below calls the same `md2pdf_core::md_to_pdf`, from the
//! same crate, byte for byte.
//!
//! **One image crosses this boundary, and it is the page's rather than the
//! reader's.** `mpdf-006` Phase 3 answered the question the spike parked here:
//! the page carries one file inline and hands it to every compile, so the
//! caption-over-an-image case reaches the pane. A browser still has no
//! filesystem, so `md2pdf_core::image_paths`' shopping list still has nowhere
//! to be read from — a *reader's own* files are the "different file story"
//! `mpdf-001` §1.1 named, and that stays parked.

use wasm_bindgen::prelude::*;

/// Route a Rust panic to the console instead of an unhelpful `unreachable`.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Compile markdown to PDF bytes, with the page's one image in hand.
///
/// `asset_path` is the destination as the markdown writes it and `asset_bytes`
/// is the file; the page reads both off its own `data-asset` element, so the
/// name it passes and the name its example writes are one string. **Every
/// compile passes it** — the typing path and the row buttons alike — because
/// `md2pdf_core::md_to_pdf` ignores an asset the document never names, and a
/// channel open to only one of the two acts would draw the figure on a click
/// and refuse it on the next keystroke.
///
/// The error is the same sentence the CLI prints after its `error: ` prefix,
/// because it is the same `Display` — a construct outside the dialect is
/// refused here in the words it is refused at the terminal.
#[wasm_bindgen]
pub fn render(markdown: &str, asset_path: &str, asset_bytes: &[u8]) -> Result<Vec<u8>, JsError> {
    let assets = [md2pdf_core::Asset {
        path: asset_path.to_string(),
        bytes: asset_bytes.to_vec(),
    }];

    md2pdf_core::md_to_pdf(markdown, &assets).map_err(|e| JsError::new(&e.to_string()))
}

/// The page each heading landed on, as `line:page` pairs.
///
/// This is `mpdf-003` Phase 6's export, answered in a browser. The desktop app
/// uses it to open the pane on the page the author's caret is in; the page no
/// longer calls it, and it stays because it is that phase's export rather than
/// this page's.
#[wasm_bindgen]
pub fn anchors(markdown: &str) -> Result<String, JsError> {
    let rendered =
        md2pdf_core::md_to_pdf_with_anchors(markdown, &[]).map_err(|e| JsError::new(&e.to_string()))?;

    Ok(rendered
        .anchors
        .iter()
        .map(|anchor| format!("{}:{}", anchor.line, anchor.page))
        .collect::<Vec<_>>()
        .join(" "))
}
