//! The bibliography file, read for the keys it holds.
//!
//! Bytes in, a key set out, and nothing else: this module decides whether a
//! citation can resolve, never how the reference list is set. Typst does the
//! rendering, from the same bytes, through the asset map.
//!
//! **The reader is `hayagriva`, and it is Typst's own.** `typst-library` 0.15.1
//! depends on it directly, so this crate names no new dependency and the
//! `wasm32` build gains nothing; and because it is the same reader at the same
//! version, this crate's key set and Typst's cannot disagree — which is the
//! faithfulness risk any second parser would have carried.
//!
//! The alternative was to read Typst's own diagnostic and map it back to a
//! line, `crate::emit::Names::cited` already holding every key with the line its
//! `[@…]` sits on. It is refused: it means matching a message's *wording*, which
//! a version bump moves silently, and it leaves the label collision no route at
//! all, since that one needs the key set rather than a message.

use std::collections::HashSet;

use crate::emit;

/// The keys one bibliography file holds, or the sentence that refuses it.
///
/// The sentence travels without a line, because the only line a bibliography
/// has is the frontmatter line that named it, and that is the caller's to
/// attach — the same way `crate::Error::MissingBibliography` gets its own.
///
/// **The dispatch is the extension, folded.** `typst-library`'s own
/// `decode_library` matches on `ext.to_lowercase()` while
/// [`emit::extension_of`] returns the extension exactly as the author wrote it,
/// so `bibliography: refs.YML` compiles perfectly well and a dispatch that did
/// not fold would call it neither format.
///
/// **The third arm is not a fallthrough.** Typst refuses an extension outside
/// the pair itself, with "unknown bibliography format (must be .yaml/.yml or
/// .bib)" against a span in a `main.typ` the user has never seen, so the
/// refusal is taken here where the frontmatter's line is known. Nothing checks
/// an extension against its content on either side, so a `.yml` holding
/// BibLaTeX reaches this function too and is refused by the parse.
pub(crate) fn keys(path: &str, bytes: &[u8]) -> std::result::Result<HashSet<String>, String> {
    let Ok(text) = std::str::from_utf8(bytes) else {
        return Err(format!("the bibliography '{path}' is not UTF-8 text"));
    };

    let extension = emit::extension_of(path).unwrap_or_default().to_lowercase();
    let library = match extension.as_str() {
        "yml" | "yaml" => hayagriva::io::from_yaml_str(text)
            .map_err(|e| format!("the bibliography '{path}' does not parse as Hayagriva: {e}"))?,
        "bib" => hayagriva::io::from_biblatex_str(text).map_err(|errors| {
            // Typst reports the first of these and drops the rest, so this does
            // too rather than printing a list a line-numbered error cannot hold.
            let first = errors
                .first()
                .map(|e| e.to_string())
                .unwrap_or_else(|| "no entry could be read".to_string());
            format!("the bibliography '{path}' does not parse as BibLaTeX: {first}")
        })?,
        _ => {
            return Err(format!(
                "the bibliography '{path}' names no format this dialect reads, and a bibliography is a '.yml', a '.yaml' or a '.bib'"
            ));
        }
    };

    Ok(library.keys().map(str::to_string).collect())
}
