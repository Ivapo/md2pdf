//! `md2pdf-core`, compiled to `wasm32-unknown-unknown` and called from a page.
//!
//! The crate holds no OS access by design, which `mpdf-001` §1.1 said would
//! buy a browser build without a rewrite. This is the first thing to test that
//! claim, and it needed no change to `core` at all.
//!
//! **No image assets cross this boundary**, and that is the honest limit of the
//! spike rather than an oversight. A browser has no filesystem, so
//! `md2pdf_core::image_paths`' shopping list has nowhere to be read from — the
//! "different file story" `mpdf-001` §1.1 named is exactly this, and it is a
//! design question for the spec that eventually owns it.

use wasm_bindgen::prelude::*;

/// Route a Rust panic to the console instead of an unhelpful `unreachable`.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Compile markdown to PDF bytes.
///
/// The error is the same sentence the CLI prints after its `error: ` prefix,
/// because it is the same `Display` — a construct outside the dialect is
/// refused here in the words it is refused at the terminal.
#[wasm_bindgen]
pub fn render(markdown: &str) -> Result<Vec<u8>, JsError> {
    md2pdf_core::md_to_pdf(markdown, &[]).map_err(|e| JsError::new(&e.to_string()))
}

/// The page each heading landed on, as `line:page` pairs.
///
/// This is `mpdf-003` Phase 6's export, answered in a browser. The desktop app
/// uses it to open the pane on the page the author's caret is in; here it is
/// reported as text, because the spike has no caret to follow.
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
