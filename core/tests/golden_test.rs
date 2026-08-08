//! The Phase 1 exit gate, at the library level.
//!
//! Fixtures and golden files live at the workspace root because the CLI tests
//! read the same ones.

use md2pdf_core::{Error, md_to_pdf, md_to_typst};

const BASIC_MD: &str = include_str!("../../tests/fixtures/basic.md");
const BASIC_TYP: &str = include_str!("../../tests/golden/basic.typ");
const HOSTILE_MD: &str = include_str!("../../tests/fixtures/hostile.md");
const HOSTILE_TYP: &str = include_str!("../../tests/golden/hostile.typ");
const LIST_MD: &str = include_str!("../../tests/fixtures/unsupported_list.md");

#[test]
fn basic_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(BASIC_MD).unwrap(), BASIC_TYP);
}

#[test]
fn basic_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(BASIC_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn hostile_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(HOSTILE_MD).unwrap(), HOSTILE_TYP);
}

/// Every character the spec names must appear escaped in the golden file.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so a future edit that drops an
/// escape fails with a message that points at the character it dropped.
#[test]
fn hostile_golden_escapes_every_typst_significant_character() {
    for ch in [
        '#', '$', '*', '_', '`', '@', '<', '>', '[', ']', '\\', '=', '-', '+', '~', '/',
    ] {
        let escaped = format!("\\{ch}");
        assert!(
            HOSTILE_TYP.contains(&escaped),
            "the golden file does not escape '{ch}'"
        );
    }
    // A line-leading digit and dot would open a Typst enumeration.
    assert!(
        HOSTILE_TYP.contains("2\\."),
        "a line-leading '2.' is unescaped"
    );
}

#[test]
fn hostile_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(HOSTILE_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn a_bullet_list_is_an_error_that_names_the_construct_and_the_line() {
    match md_to_typst(LIST_MD) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "bullet list");
            assert_eq!(line, 5);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// Phase 1 ignores a leading frontmatter block. Phase 2 parses it.
#[test]
fn a_leading_frontmatter_block_is_ignored() {
    let md = "---\ntitle: Ignored\n---\n\n# Heading\n\nBody.\n";
    let typst_source = md_to_typst(md).unwrap();
    assert!(
        !typst_source.contains("Ignored"),
        "the frontmatter leaked into the body"
    );
    assert!(typst_source.contains("= Heading"));
}

/// A construct after ignored frontmatter still reports its true line number.
#[test]
fn line_numbers_survive_an_ignored_frontmatter_block() {
    let md = "---\ntitle: Ignored\n---\n\n# Heading\n\n- a bullet\n";
    match md_to_typst(md) {
        Err(Error::UnsupportedConstruct { line, .. }) => assert_eq!(line, 7),
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}
