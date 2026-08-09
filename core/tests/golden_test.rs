//! The exit gates for Phases 1 to 3, at the library level.
//!
//! Fixtures and golden files live at the workspace root because the CLI tests
//! read the same ones.

use md2pdf_core::{Error, md_to_pdf, md_to_typst};

const BASIC_MD: &str = include_str!("../../tests/fixtures/basic.md");
const BASIC_TYP: &str = include_str!("../../tests/golden/basic.typ");
const HOSTILE_MD: &str = include_str!("../../tests/fixtures/hostile.md");
const HOSTILE_TYP: &str = include_str!("../../tests/golden/hostile.typ");
const TABLE_MD: &str = include_str!("../../tests/fixtures/unsupported_table.md");
const FRONTMATTER_MD: &str = include_str!("../../tests/fixtures/frontmatter.md");
const FRONTMATTER_TYP: &str = include_str!("../../tests/golden/frontmatter.typ");
const SINGLE_COLUMN_MD: &str = include_str!("../../tests/fixtures/single_column.md");
const SINGLE_COLUMN_TYP: &str = include_str!("../../tests/golden/single_column.typ");
const UNKNOWN_KEY_MD: &str = include_str!("../../tests/fixtures/unknown_key.md");
const INLINE_MD: &str = include_str!("../../tests/fixtures/inline.md");
const INLINE_TYP: &str = include_str!("../../tests/golden/inline.typ");
const HOSTILE_CODE_MD: &str = include_str!("../../tests/fixtures/hostile_code.md");
const HOSTILE_CODE_TYP: &str = include_str!("../../tests/golden/hostile_code.typ");

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
fn a_table_is_an_error_that_names_the_construct_and_the_line() {
    match md_to_typst(TABLE_MD) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "table");
            assert_eq!(line, 5);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// A construct after the frontmatter still reports its true line number.
///
/// Nothing strips the block from the input, which is what this guards.
#[test]
fn line_numbers_survive_a_frontmatter_block() {
    let md = "---\ntitle: A Title\n---\n\n# Heading\n\n| a | b |\n| - | - |\n";
    match md_to_typst(md) {
        Err(Error::UnsupportedConstruct { line, .. }) => assert_eq!(line, 7),
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

// -- Phase 2: frontmatter and column layout ---------------------------------

#[test]
fn the_frontmatter_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(FRONTMATTER_MD).unwrap(), FRONTMATTER_TYP);
}

#[test]
fn the_frontmatter_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(FRONTMATTER_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn the_single_column_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(SINGLE_COLUMN_MD).unwrap(), SINGLE_COLUMN_TYP);
}

#[test]
fn the_single_column_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(SINGLE_COLUMN_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The title and the author reach the generated source.
///
/// The equality tests above pin the whole output, but they cannot say *why* it
/// is right. This one names the rule, so an edit that drops a key fails with a
/// message that points at the key it dropped.
#[test]
fn the_generated_source_carries_the_title_and_the_author() {
    let typst_source = md_to_typst(FRONTMATTER_MD).unwrap();
    assert!(
        typst_source.contains("title: \"A Minimal Example\""),
        "the title is missing"
    );
    assert!(
        typst_source.contains("author: \"Iva Po\""),
        "the author is missing"
    );
}

/// Absent frontmatter is valid, and every default applies.
#[test]
fn absent_frontmatter_gets_every_default() {
    assert!(
        md_to_typst(BASIC_MD)
            .unwrap()
            .contains("template.with(title: none, author: none, columns: 2)"),
        "the defaults did not reach the template call"
    );
}

#[test]
fn an_unknown_frontmatter_key_is_an_error_that_names_it() {
    match md_to_typst(UNKNOWN_KEY_MD) {
        Err(Error::Frontmatter { line, problem }) => {
            assert_eq!(line, 3);
            assert!(problem.contains("subtitle"), "problem was: {problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

#[test]
fn a_columns_value_outside_the_schema_is_an_error_that_names_the_key() {
    let md = "---\ncolumns: 3\n---\n\n# Heading\n";
    match md_to_typst(md) {
        Err(Error::Frontmatter { line, problem }) => {
            assert_eq!(line, 2);
            assert!(problem.contains("columns"), "problem was: {problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

/// A bad key is reported before a later unsupported construct.
///
/// The frontmatter is the first thing the user reads, so it is the first thing
/// the error should name.
#[test]
fn a_frontmatter_error_wins_over_a_later_construct_error() {
    let md = "---\nsubtitle: Bad\n---\n\n| a | b |\n| - | - |\n";
    match md_to_typst(md) {
        Err(Error::Frontmatter { problem, .. }) => {
            assert!(problem.contains("subtitle"), "problem was: {problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

// -- Phase 3: inline constructs ---------------------------------------------

#[test]
fn the_inline_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(INLINE_MD).unwrap(), INLINE_TYP);
}

#[test]
fn the_inline_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(INLINE_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each inline construct reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that swaps a function
/// call for Typst's own markup fails with a message that points at it.
#[test]
fn the_inline_golden_uses_the_function_forms() {
    for (form, what) in [
        ("#emph[emphasis]", "emphasis"),
        ("#strong[strong emphasis]", "strong emphasis"),
        ("#emph[#strong[both at once]]", "the two nested"),
        ("#raw(\"inline code\")", "inline code"),
        ("#divider()", "the thematic break"),
        // A `\` before a newline is Typst's line break. The same `\` before
        // text is an escape sequence instead.
        ("this line,\\\n", "the hard line break"),
        // Typst's own `_…_` and `*…*` cannot express this one, which is the
        // whole reason the emitter writes function calls.
        ("foo#emph[bar]baz", "intraword emphasis"),
    ] {
        assert!(
            INLINE_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

#[test]
fn the_hostile_code_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(HOSTILE_CODE_MD).unwrap(), HOSTILE_CODE_TYP);
}

#[test]
fn the_hostile_code_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(HOSTILE_CODE_MD).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Inline code travels as a string literal, so only `\` and `"` are escaped.
///
/// A string literal interprets nothing else, and applying the markup escape
/// inside one would put the backslashes into the PDF. Each case below holds a
/// character the markup escape *would* have touched.
#[test]
fn the_hostile_code_golden_applies_only_the_string_escape() {
    for (literal, what) in [
        // `r##` throughout, because `("#` would close an `r#` string.
        (r##"#raw("\\backslash")"##, "a backslash, doubled"),
        (r##"#raw("#hash")"##, "a hash, untouched"),
        (r##"#raw("$dollar")"##, "a dollar sign, untouched"),
        (r##"#raw("`backtick`")"##, "backticks, untouched"),
    ] {
        assert!(
            HOSTILE_CODE_TYP.contains(literal),
            "the golden file does not show {what} as `{literal}`"
        );
    }
}
