//! The twelve claims the dialect's front door makes, compiled without the door.
//!
//! `mpdf-006`'s design was that every claim `web/index.html` makes is a snippet
//! the suite compiles, and `mpdf-011` Phase 2 sends the page to Letur. The rows
//! are the only place this dialect's public promises — a caption makes a figure,
//! a `[@key]` cites, a task list is refused — are checked *as a reader meets
//! them*, so they stay here as fixtures of their own while the page goes on to
//! become a landing site somewhere else.
//!
//! **The fixtures under `tests/fixtures/examples/` are the page's own bytes**,
//! taken out of it at the split and frozen there. Neither asset could be
//! borrowed from this tree: `tests/fixtures/refs.yml` is keyed
//! `DBLP:books/lib/Knuth86a` and the citation row cites `knuth1986`, and
//! `samples/pipeline.svg` carries one trailing newline the page's does not —
//! which compiles, and would have made this file pass over a fixture set that
//! is not the one the reader was shown.
//!
//! **A row's expectation is carried by its directory and never by its
//! contents.** Two rows open with `---` and a leading HTML comment is itself a
//! `raw HTML block` refusal, so there is no marker a fixture could hold that
//! the fixture's own subject does not already claim.
//!
//! The directories are read rather than listed here, so the counts below are a
//! measurement rather than a restatement: a row added or deleted without a
//! thought for this file is what they catch.

use md2pdf_core::{Asset, md_to_pdf};
use std::fs;

/// The fixtures, from this crate's manifest rather than a working directory.
///
/// They live at the workspace root because `tests/fixtures/` is where every
/// other fixture this suite reads lives, and `cli/tests/cli_test.rs` reads the
/// same tree.
const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/examples");

/// What each refusal row printed on the page, in name order.
///
/// **The sentence is the one the reader was shown**, transcribed from the row's
/// own visible `<code>` element at the split. `cli/src/main.rs` prints this same
/// `Display` after its `error: ` prefix, so a sentence that drifts here is a
/// sentence that drifted at the terminal.
const REFUSALS: [(&str, &str); 3] = [
    (
        "math-refusal",
        "math error at line 1: unsupported command '\\includegraphics'",
    ),
    (
        "raw-html",
        "unsupported markdown construct 'raw HTML block' at line 3",
    ),
    (
        "task-list",
        "unsupported markdown construct 'task list marker' at line 1",
    ),
];

/// The page's image, at the byte length it carried.
const IMAGE: (&str, usize) = ("pipeline.svg", 509);

/// The page's bibliography, at the byte length it carried.
const BIBLIOGRAPHY: (&str, usize) = ("refs.yml", 151);

/// Every row under one expectation, as `(name, markdown)`, in name order.
///
/// Sorted so a comparison against [`REFUSALS`] is a comparison of two lists and
/// not of two sets, and so a failure names the same row twice running.
fn rows(expect: &str) -> Vec<(String, String)> {
    let dir = format!("{ROOT}/{expect}");
    let mut found: Vec<(String, String)> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {dir}: {e}"))
        .map(|entry| entry.expect("cannot read an entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "md"))
        .map(|path| {
            let name = path
                .file_stem()
                .expect("a fixture with no name")
                .to_string_lossy()
                .into_owned();
            let markdown = fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            (name, markdown)
        })
        .collect();

    found.sort();
    found
}

/// Both files the page carried, as the compiler takes them.
///
/// Every compile is handed both, as the page handed every compile both:
/// `md_to_pdf` ignores an asset the document never names, and a channel open to
/// one row alone would be a page that draws its figure on a click and refuses it
/// on the reader's next keystroke.
fn assets() -> [Asset; 2] {
    [named(IMAGE.0), named(BIBLIOGRAPHY.0)]
}

/// One asset, under the name the rows name it by.
///
/// **The name is the path.** The caption row's own `![…](pipeline.svg)` and the
/// citation row's `bibliography: refs.yml` must equal these, and nothing
/// asserts that equality because nothing needs to: a row naming a different file
/// comes back `MissingImage` or `MissingBibliography` from
/// `core/src/lib.rs:collect` and fails the compile below.
fn named(path: &str) -> Asset {
    let at = format!("{ROOT}/{path}");
    Asset {
        path: path.to_string(),
        bytes: fs::read(&at).unwrap_or_else(|e| panic!("cannot read {at}: {e}")),
    }
}

/// The fixture set is the page's: twelve rows, nine of them accepted, and two files.
///
/// **The byte lengths are the point of the assertion, not decoration.** A later
/// tidy-up that pointed either asset at the engine's own near-copy would leave
/// every other test in this file green — the SVG differs by one trailing newline
/// and compiles — so the length is what tells the page's file from a lookalike.
#[test]
fn the_fixtures_are_the_pages_twelve_rows_and_its_two_assets() {
    let ok = rows("ok");
    let error = rows("error");

    assert_eq!(ok.len(), 9, "the page carried nine accepted rows");
    assert_eq!(error.len(), 3, "the page carried three refusal rows");

    for (path, bytes) in [IMAGE, BIBLIOGRAPHY] {
        assert_eq!(
            named(path).bytes.len(),
            bytes,
            "`{path}` is not the file the page carried"
        );
    }
}

/// Every accepted row compiles, with both of the page's files in hand.
#[test]
fn every_ok_example_compiles() {
    let assets = assets();

    for (name, markdown) in rows("ok") {
        let pdf = md_to_pdf(&markdown, &assets)
            .unwrap_or_else(|e| panic!("the '{name}' example is shown as accepted but fails: {e}"));

        assert!(pdf.starts_with(b"%PDF"), "the '{name}' output is not a PDF");
        assert!(pdf.len() > 1000, "the '{name}' PDF is suspiciously small");
    }
}

/// Every refusal row refuses, in the sentence the reader was shown.
///
/// The names are compared as well as the sentences, so a row renamed or removed
/// cannot leave a sentence in [`REFUSALS`] asserting agreement with nothing.
#[test]
fn every_refusal_prints_the_sentence_beside_it() {
    let assets = assets();
    let rows = rows("error");

    let named: Vec<&str> = rows.iter().map(|(name, _)| name.as_str()).collect();
    let declared: Vec<&str> = REFUSALS.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        named, declared,
        "the refusal fixtures and the sentences name different rows"
    );

    for ((name, markdown), (_, sentence)) in rows.iter().zip(REFUSALS) {
        let error = md_to_pdf(markdown, &assets)
            .err()
            .unwrap_or_else(|| panic!("the '{name}' example is shown as refused but compiles"));

        assert_eq!(
            error.to_string(),
            sentence,
            "the '{name}' row prints a sentence the compiler does not"
        );
    }
}
