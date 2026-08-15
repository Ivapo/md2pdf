//! The exit gates for `mpdf-001`'s nine phases and `mpdf-002`'s first, at the
//! library level.
//!
//! Fixtures and golden files live at the workspace root because the CLI tests
//! read the same ones.

use md2pdf_core::{
    Asset, Error, ImageRef, image_paths, md_to_pdf, md_to_pdf_with_anchors, md_to_typst,
};

const BASIC_MD: &str = include_str!("../../tests/fixtures/basic.md");
const BASIC_TYP: &str = include_str!("../../tests/golden/basic.typ");
const HOSTILE_MD: &str = include_str!("../../tests/fixtures/hostile.md");
const HOSTILE_TYP: &str = include_str!("../../tests/golden/hostile.typ");
const FRONTMATTER_MD: &str = include_str!("../../tests/fixtures/frontmatter.md");
const FRONTMATTER_TYP: &str = include_str!("../../tests/golden/frontmatter.typ");
const SINGLE_COLUMN_MD: &str = include_str!("../../tests/fixtures/single_column.md");
const SINGLE_COLUMN_TYP: &str = include_str!("../../tests/golden/single_column.typ");
const UNKNOWN_KEY_MD: &str = include_str!("../../tests/fixtures/unknown_key.md");
const INLINE_MD: &str = include_str!("../../tests/fixtures/inline.md");
const INLINE_TYP: &str = include_str!("../../tests/golden/inline.typ");
const HOSTILE_CODE_MD: &str = include_str!("../../tests/fixtures/hostile_code.md");
const HOSTILE_CODE_TYP: &str = include_str!("../../tests/golden/hostile_code.typ");
const BLOCKS_MD: &str = include_str!("../../tests/fixtures/blocks.md");
const BLOCKS_TYP: &str = include_str!("../../tests/golden/blocks.typ");
const LIST_SPACING_MD: &str = include_str!("../../tests/fixtures/list_spacing.md");
const LIST_SPACING_TYP: &str = include_str!("../../tests/golden/list_spacing.typ");
const LINKS_MD: &str = include_str!("../../tests/fixtures/links.md");
const LINKS_TYP: &str = include_str!("../../tests/golden/links.typ");
const HTML_MD: &str = include_str!("../../tests/fixtures/unsupported_html.md");
const TABLE_MD: &str = include_str!("../../tests/fixtures/table.md");
const TABLE_TYP: &str = include_str!("../../tests/golden/table.typ");
const IMAGES_MD: &str = include_str!("../../tests/fixtures/images.md");
const IMAGES_TYP: &str = include_str!("../../tests/golden/images.typ");
const FOOTNOTES_MD: &str = include_str!("../../tests/fixtures/footnotes.md");
const FOOTNOTES_TYP: &str = include_str!("../../tests/golden/footnotes.typ");
const STRIKETHROUGH_MD: &str = include_str!("../../tests/fixtures/strikethrough.md");
const STRIKETHROUGH_TYP: &str = include_str!("../../tests/golden/strikethrough.typ");
const REFUSED_MATH_MD: &str = include_str!("../../tests/fixtures/unsupported_math.md");
const MATH_MD: &str = include_str!("../../tests/fixtures/math.md");
const MATH_TYP: &str = include_str!("../../tests/golden/math.typ");
const DISPLAY_MATH_MD: &str = include_str!("../../tests/fixtures/display_math.md");
const DISPLAY_MATH_TYP: &str = include_str!("../../tests/golden/display_math.typ");
const TASK_LIST_MD: &str = include_str!("../../tests/fixtures/unsupported_task_list.md");
const DATED_MD: &str = include_str!("../../tests/fixtures/dated.md");
const DATED_TYP: &str = include_str!("../../tests/golden/dated.typ");
const PRESS_RELEASE_MD: &str = include_str!("../../tests/fixtures/press_release.md");
const PRESS_RELEASE_TYP: &str = include_str!("../../tests/golden/press_release.typ");
const UNKNOWN_TEMPLATE_MD: &str = include_str!("../../tests/fixtures/unknown_template.md");

/// Every bundled look, by the name the `template` key selects it with. Four
/// tests read these: the header-row rule, and the three Phase 9 cases that no
/// golden file can pin, because a golden pins emitter output alone.
const TEMPLATE_TYP: &str = include_str!("../assets/template.typ");
const PRESS_RELEASE_TEMPLATE_TYP: &str = include_str!("../assets/press-release.typ");
const BUNDLED_TEMPLATES: [(&str, &str); 2] = [
    ("template.typ", TEMPLATE_TYP),
    ("press-release.typ", PRESS_RELEASE_TEMPLATE_TYP),
];

/// The two image files the `images.md` fixture names, and one more name for the
/// same PNG, because a path carrying a `#` is one of the shapes that fixture
/// pins. An asset's name is the path the markdown wrote, not a real filename.
const DOT_PNG: &[u8] = include_bytes!("../../tests/fixtures/dot.png");
const MARK_SVG: &[u8] = include_bytes!("../../tests/fixtures/mark.svg");

fn images_assets() -> Vec<Asset> {
    vec![
        asset("dot.png", DOT_PNG),
        asset("mark.svg", MARK_SVG),
        asset("fig#2.png", DOT_PNG),
    ]
}

fn asset(path: &str, bytes: &[u8]) -> Asset {
    Asset {
        path: path.to_string(),
        bytes: bytes.to_vec(),
    }
}

#[test]
fn basic_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(BASIC_MD).unwrap(), BASIC_TYP);
}

#[test]
fn basic_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(BASIC_MD, &[]).unwrap();
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
    let pdf = md_to_pdf(HOSTILE_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// A construct after the frontmatter still reports its true line number.
///
/// Nothing strips the block from the input, which is what this guards.
#[test]
fn line_numbers_survive_a_frontmatter_block() {
    let md = "---\ntitle: A Title\n---\n\n# Heading\n\n<div>a block</div>\n";
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
    let pdf = md_to_pdf(FRONTMATTER_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn the_single_column_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(SINGLE_COLUMN_MD).unwrap(), SINGLE_COLUMN_TYP);
}

#[test]
fn the_single_column_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(SINGLE_COLUMN_MD, &[]).unwrap();
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
            .contains("template.with(title: none, author: none, columns: 2, date: none)"),
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
    let md = "---\nsubtitle: Bad\n---\n\n<div>a block</div>\n";
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
    let pdf = md_to_pdf(INLINE_MD, &[]).unwrap();
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
    let pdf = md_to_pdf(HOSTILE_CODE_MD, &[]).unwrap();
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

// -- Phase 4: block constructs ----------------------------------------------

#[test]
fn the_blocks_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(BLOCKS_MD).unwrap(), BLOCKS_TYP);
}

#[test]
fn the_blocks_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(BLOCKS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each block construct reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that drops the nesting,
/// the start number, or the language tag fails with a message that points at it.
#[test]
fn the_blocks_golden_carries_each_construct() {
    for (form, what) in [
        ("- second item\n  - a nested item", "the nested bullet list"),
        // Every item carries its own number, so nothing depends on how Typst
        // continues an implicit counter.
        ("3. third\n4. fourth", "the ordered list starting at three"),
        // Two paragraphs in one item, the second indented past the marker.
        (
            "- The first paragraph of a loose item.\n\n  The second",
            "the loose item's continuation",
        ),
        ("#raw(block: true, lang: \"rust\", ", "the language tag"),
        ("#quote(block: true)[", "the block quote"),
    ] {
        assert!(
            BLOCKS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // An indented block has no info string, so it gets no `lang` argument. Its
    // content is a string literal, so the `#` inside it is untouched too.
    assert!(
        BLOCKS_TYP.contains(r##"#raw(block: true, "#raw text"##),
        "the indented block carries a language argument"
    );

    // pulldown-cmark reports the final line's terminator as part of a code
    // block's content, and a literal that kept it would typeset a phantom empty
    // line after every block. This is the escape sequence, not a real newline.
    assert!(
        !BLOCKS_TYP.contains("\\n\")"),
        "a code block kept its trailing newline"
    );
}

#[test]
fn the_list_spacing_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(LIST_SPACING_MD).unwrap(), LIST_SPACING_TYP);
}

#[test]
fn the_list_spacing_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(LIST_SPACING_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// A tight list and a loose one differ in the blank lines and nothing else.
///
/// Typst derives `tight` from exactly that adjacency and no `set` rule overrides
/// it, so the blank line is the whole mechanism. The emitter passes the
/// distinction through structurally and owns nothing about the spacing.
#[test]
fn the_two_list_spacings_differ_only_in_the_blank_lines() {
    assert!(
        LIST_SPACING_TYP.contains("- alpha\n- beta\n- gamma"),
        "the tight list is not on adjacent lines"
    );
    assert!(
        LIST_SPACING_TYP.contains("- alpha\n\n- beta\n\n- gamma"),
        "the loose list is not separated by blank lines"
    );
}

// -- Phase 5: links ---------------------------------------------------------

#[test]
fn the_links_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(LINKS_MD).unwrap(), LINKS_TYP);
}

#[test]
fn the_links_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(LINKS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each link form reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that drops the scheme,
/// the string escape, or the escape on the link text fails with a message that
/// points at what it dropped.
#[test]
fn the_links_golden_carries_each_form() {
    for (form, what) in [
        (
            r#"#link("https://typst.app")[inline link]"#,
            "the inline link",
        ),
        // pulldown-cmark resolves the definition, so a reference link arrives
        // carrying the destination and needs no mechanism of its own.
        (
            r#"#link("https://spec.commonmark.org/0.31.2/")[reference"#,
            "the reference link, resolved",
        ),
        // An autolink's text is ordinary inline content, so the markup escape
        // still applies to it. Without that, Typst would read the text as a
        // second link of its own.
        (
            r#"#link("https://github.com/")[https:\/\/github.com\/]"#,
            "the autolink",
        ),
        // The destination arrives as the bare address, so the scheme is the
        // emitter's to add. The text keeps its own escape.
        (
            r#"#link("mailto:ivapo@example.com")[ivapo\@example.com]"#,
            "the email autolink",
        ),
        // A string literal interprets only `\` and `"`, so the `#` travels
        // untouched while the `"` is escaped.
        (
            r#"#link("https://example.com/a\"b#frag")[a hostile URL]"#,
            "the hostile URL",
        ),
    ] {
        assert!(
            LINKS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // The markup escape must never reach a URL. It would put the backslashes
    // into the PDF, and no character in this fixture's prose asks for one.
    assert!(
        !LINKS_TYP.contains(r"\#"),
        "the markup escape reached a URL"
    );
}

/// Rejection survives the widening. Images left the out-of-dialect list in
/// `mpdf-002`'s Phase 1, and a raw HTML block took over this gate: the parser
/// reads one with no option at all, where strikethrough, footnotes and math
/// would each need one.
#[test]
fn a_raw_html_block_is_an_error_that_names_the_construct_and_the_line() {
    match md_to_typst(HTML_MD) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "raw HTML block");
            assert_eq!(line, 5);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// `#link("")` fails Typst's compile naming neither construct nor line, so the
/// emitter names both before the compiler ever sees the document.
#[test]
fn an_empty_link_destination_is_an_error_that_names_the_construct_and_the_line() {
    let md = "# Heading\n\nAn [empty destination]() in a sentence.\n";
    match md_to_typst(md) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "link with an empty destination");
            assert_eq!(line, 3);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// A reference definition is the second route to an empty destination, and the
/// test is on the resolved destination, so it catches this one too.
#[test]
fn an_empty_reference_definition_is_the_same_error() {
    let md = "# Heading\n\nA [reference link][ref] in a sentence.\n\n[ref]: <>\n";
    match md_to_typst(md) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "link with an empty destination");
            assert_eq!(line, 3);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// Neither Typst's `link` nor the PDF can carry a title, so passing the link on
/// would mean dropping it silently.
#[test]
fn a_link_title_is_an_error_that_names_the_construct_and_the_line() {
    let md = "# Heading\n\nA [titled link](https://typst.app \"a title\") in a sentence.\n";
    match md_to_typst(md) {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "link with a title");
            assert_eq!(line, 3);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// An empty title is not a title, so the link stays inside the dialect.
#[test]
fn an_empty_link_title_is_not_an_error() {
    let md = "# Heading\n\nA [link](https://typst.app \"\") in a sentence.\n";
    let typst = md_to_typst(md).unwrap();
    assert!(
        typst.contains(r#"#link("https://typst.app")[link]"#),
        "the link did not survive its empty title: {typst}"
    );
}

// -- Phase 6: tables --------------------------------------------------------

#[test]
fn the_table_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(TABLE_MD).unwrap(), TABLE_TYP);
}

/// The compile is what exercises the template's header rule, which no golden
/// file can pin.
#[test]
fn the_table_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(TABLE_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the table reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that drops the column
/// count, an alignment, the header row, or a cell's own translation fails with
/// a message that points at what it dropped.
#[test]
fn the_table_golden_carries_each_part() {
    for (form, what) in [
        // An integer gives that many auto-sized columns.
        ("columns: 4,", "the column count"),
        // The delimiter row set one column of each kind, and a column it left
        // alone is `auto` rather than a guess.
        ("align: (auto, left, center, right),", "the four alignments"),
        // This is what repeats the header across a page break and carries the
        // accessibility tagging.
        (
            "table.header([Construct], [Left], [Center], [Right]),",
            "the header row",
        ),
        // A cell holds inline content, so the arms that already exist serve it.
        ("[#emph[emphasis in a cell]]", "emphasis inside a cell"),
        (r#"[#raw("inline code")]"#, "inline code inside a cell"),
        (
            r#"[#link("https://typst.app")[Typst]]"#,
            "a link inside a cell",
        ),
        // A pipe is not Typst markup, so an escaped one reaches the cell as
        // itself and the markup escape leaves it alone.
        ("[a | pipe]", "the escaped pipe"),
        // pulldown-cmark pads a short row, following GFM, so the emitter counts
        // no cells and the padding arrives as an empty content block.
        (
            "[short row], [three cells], [only], [],",
            "the padded short row",
        ),
    ] {
        assert!(
            TABLE_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// The header row is set in strong type, and the template owns that rule.
///
/// Golden files pin emitter output only, so the template's side of the decision
/// needs an artifact of its own. Without the rule the PDF would set the header
/// in body type and flatten a distinction the markdown source draws.
#[test]
fn the_template_sets_the_header_row_in_strong_type() {
    assert!(
        TEMPLATE_TYP.contains("show table.cell.where(y: 0): strong"),
        "the template does not set row zero in strong type"
    );
}

/// One column still emits an array, not a parenthesised word.
///
/// The fixture cannot reach this: it has four columns, and `(left, right)` is
/// an array whatever else happens. `(left)` alone is not one.
#[test]
fn a_single_column_table_emits_an_array_of_one_alignment() {
    let md = "| head |\n| :--- |\n| body |\n";
    let typst = md_to_typst(md).unwrap();
    assert!(
        typst.contains("align: (left,),"),
        "the one alignment is not an array: {typst}"
    );
}

/// A table whose delimiter row sets no alignment leaves the argument out.
#[test]
fn a_table_without_alignments_omits_the_argument() {
    let md = "| a | b |\n| - | - |\n| 1 | 2 |\n";
    let typst = md_to_typst(md).unwrap();
    assert!(
        !typst.contains("align:"),
        "an alignment argument that says nothing was written: {typst}"
    );
}

// -- mpdf-002 Phase 1: images -----------------------------------------------

#[test]
fn the_images_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(IMAGES_MD).unwrap(), IMAGES_TYP);
}

/// The compile is the phase's observable, at the library level: markdown that
/// names image files becomes a PDF that holds them.
#[test]
fn the_images_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(IMAGES_MD, &images_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each image form reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that boxes a standalone
/// image, drops a box, or sends a path or an alt through the markup escape
/// fails with a message that points at what it dropped.
#[test]
fn the_images_golden_carries_each_form() {
    for (form, what) in [
        // A paragraph holding one image and nothing else stays a block, which
        // is what a later figure treatment can address.
        (
            "\n#image(\"dot.png\", alt: \"The three steps, drawn as boxes\")\n",
            "the standalone image, bare",
        ),
        // Typst lays an image out as a block, so mid-sentence it needs the box.
        (
            r#"icon #box(image("mark.svg", alt: "a check mark")) sits"#,
            "the inline image, boxed",
        ),
        // An image that opens its paragraph and is followed by text is inline
        // too. An implementation that decides on what preceded gets this wrong.
        (
            r#"#box(image("fig#2.png", alt: "an \"outline\" of the whole idea")) opens"#,
            "the image that opens its paragraph, boxed",
        ),
        // An empty alt leaves the argument out rather than naming an empty
        // description.
        ("\n#image(\"dot.png\")\n", "the empty alt, omitted"),
    ] {
        assert!(
            IMAGES_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // The path and the alt travel as string literals. The markup escape would
    // put its backslashes into the PDF, and it would break the `#` in a path.
    assert!(
        !IMAGES_TYP.contains(r"\#2.png"),
        "the markup escape reached an image path"
    );
}

/// The alt text is the flattened plain text of the image's content.
///
/// This is CommonMark's own reading of alt, and what pulldown-cmark's HTML
/// renderer implements. Typst's `alt` is a plain string, so there is nothing
/// else it could carry.
#[test]
fn the_alt_text_flattens_the_image_content() {
    for (md, expected, what) in [
        (
            "![plain *emphasis* and `code`](dot.png)\n",
            r#"alt: "plain emphasis and code""#,
            "styling and code",
        ),
        // A break is whitespace under the alt reading, and one space is what
        // pulldown-cmark's own flattening writes for it.
        (
            "![a break\nfollows](dot.png)\n",
            r#"alt: "a break follows""#,
            "the soft break, as one space",
        ),
        (
            "![a break\\\nfollows](dot.png)\n",
            r#"alt: "a break follows""#,
            "the hard break, as one space",
        ),
        // A nested image contributes its own inner text and nothing else. Its
        // end event must not close the outer capture.
        (
            "![outer ![inner](in.png) end](dot.png)\n",
            r#"alt: "outer inner end""#,
            "the nested image, flattened by the same rule",
        ),
        (
            "![a [linked](https://typst.app) word](dot.png)\n",
            r#"alt: "a linked word""#,
            "the link wrapper, contributing nothing",
        ),
    ] {
        let typst = md_to_typst(md).unwrap();
        assert!(
            typst.contains(expected),
            "the alt capture does not flatten {what} as `{expected}`: {typst}"
        );
    }
}

/// A construct outside the dialect still errors inside alt text.
#[test]
fn an_out_of_dialect_construct_inside_alt_text_is_still_an_error() {
    match md_to_typst("![before <b>bold</b> after](dot.png)\n") {
        Err(Error::UnsupportedConstruct { construct, line }) => {
            assert_eq!(construct, "raw HTML");
            assert_eq!(line, 1);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// The shopping list keeps document order and repeats a repeated path.
///
/// A nested image's destination is not content under the alt reading, so it
/// stays out of the list and out of the validation with it.
#[test]
fn image_paths_lists_every_reference_in_document_order() {
    let md = "![one](a.png)\n\nText ![two](b.svg) more.\n\n![again](a.png)\n\n![outer ![in](c.png) x](d.png)\n";
    assert_eq!(
        image_paths(md).unwrap(),
        vec![
            ImageRef {
                path: "a.png".to_string(),
                line: 1
            },
            ImageRef {
                path: "b.svg".to_string(),
                line: 3
            },
            ImageRef {
                path: "a.png".to_string(),
                line: 5
            },
            ImageRef {
                path: "d.png".to_string(),
                line: 7
            },
        ]
    );
}

/// Every destination shape the pipeline cannot carry names itself and its line.
///
/// The first two mirror the link arm. The next four are the relative-path rule.
/// The last two are the format gate's first half: Typst reads the extension
/// before the content, so an extension it does not name leaves the format
/// undecided, and the dialect refuses to guess.
#[test]
fn each_bad_image_destination_names_its_shape_and_its_line() {
    for (md, construct) in [
        (
            "# H\n\nA ![alt]() here.\n",
            "image with an empty destination",
        ),
        (
            "# H\n\nA ![alt](a.png \"a title\") here.\n",
            "image with a title",
        ),
        (
            "# H\n\nA ![alt](https://example.com/a.png) here.\n",
            "image with a URL destination",
        ),
        (
            "# H\n\nA ![alt](data:image/png;base64,iVBOR) here.\n",
            "image with a URL destination",
        ),
        // A Windows drive path reads as a scheme, and the error says so.
        (
            "# H\n\nA ![alt](C:/figure.png) here.\n",
            "image with a URL destination",
        ),
        (
            "# H\n\nA ![alt](/figures/a.png) here.\n",
            "image with an absolute path",
        ),
        (
            "# H\n\nA ![alt](../a.png) here.\n",
            "image with a '..' path segment",
        ),
        // Typst's virtual filesystem cannot hold a backslash segment, so this
        // path would fail the compile with a message naming generated source.
        (
            "# H\n\nA ![alt](figures\\a.png) here.\n",
            "image with a backslash in its path",
        ),
        (
            "# H\n\nA ![alt](a.bmp) here.\n",
            "image with a .bmp extension",
        ),
        // Typst's own table is lowercase, and its fallback to content detection
        // is deliberately not mirrored.
        (
            "# H\n\nA ![alt](a.PNG) here.\n",
            "image with a .PNG extension",
        ),
        (
            "# H\n\nA ![alt](figure) here.\n",
            "image with no file extension",
        ),
    ] {
        match md_to_typst(md) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                line,
            }) => {
                assert_eq!(found, construct, "for: {md}");
                assert_eq!(line, 3, "for: {md}");
            }
            other => panic!("expected `{construct}`, got {other:?}"),
        }
    }
}

/// An empty title is not a title, so the image stays inside the dialect. This
/// mirrors the link arm, which draws the same line.
#[test]
fn an_empty_image_title_is_not_an_error() {
    let typst = md_to_typst("# H\n\nA ![alt](a.png \"\") here.\n").unwrap();
    assert!(
        typst.contains(r#"#box(image("a.png", alt: "alt"))"#),
        "the image did not survive its empty title: {typst}"
    );
}

/// A file the caller never supplied is an error naming the path and the line.
///
/// Typst's own error would name a span in `main.typ`, which the user has never
/// seen, so the check runs before the compile.
#[test]
fn a_missing_image_file_names_the_path_and_the_line() {
    match md_to_pdf("# H\n\n![alt](missing.png)\n", &[]) {
        Err(Error::MissingImage { path, line }) => {
            assert_eq!(path, "missing.png");
            assert_eq!(line, 3);
        }
        other => panic!("expected a MissingImage error, got {other:?}"),
    }
}

/// A path used twice reports once, at its first reference's line.
#[test]
fn a_repeated_missing_path_reports_its_first_reference() {
    match md_to_pdf("# H\n\n![one](twice.png)\n\n![two](twice.png)\n", &[]) {
        Err(Error::MissingImage { path, line }) => {
            assert_eq!(path, "twice.png");
            assert_eq!(line, 3);
        }
        other => panic!("expected a MissingImage error, got {other:?}"),
    }
}

/// Bytes that disagree with the extension are an error before the compile.
///
/// The extension decides the format, because that is the order Typst's own
/// detection follows, so `core` requires the content to agree with the name.
#[test]
fn image_bytes_that_the_extension_does_not_name_are_an_error() {
    let assets = vec![asset("mark.svg", DOT_PNG)];
    match md_to_pdf("# H\n\n![alt](mark.svg)\n", &assets) {
        Err(Error::ImageFormat { path, line, format }) => {
            assert_eq!(path, "mark.svg");
            assert_eq!(line, 3);
            assert_eq!(format, "SVG");
        }
        other => panic!("expected an ImageFormat error, got {other:?}"),
    }
}

/// The check reads each format the table names, not the PNG magic alone.
#[test]
fn each_format_check_reads_the_magic_its_extension_names() {
    for (path, bytes, ok) in [
        ("a.png", DOT_PNG, true),
        ("a.svg", MARK_SVG, true),
        ("a.jpg", b"\xff\xd8\xff\x00rest".as_slice(), true),
        ("a.jpeg", b"\xff\xd8\xff\x00rest".as_slice(), true),
        ("a.gif", b"GIF89a-rest".as_slice(), true),
        ("a.webp", b"RIFF\x00\x00\x00\x00WEBPrest".as_slice(), true),
        ("a.svgz", b"\x1f\x8b-rest".as_slice(), true),
        ("a.pdf", b"%PDF-1.7 rest".as_slice(), true),
        ("a.gif", b"GIF88a-rest".as_slice(), false),
        ("a.webp", b"RIFF\x00\x00\x00\x00WAVErest".as_slice(), false),
        ("a.svgz", DOT_PNG, false),
        ("a.pdf", DOT_PNG, false),
        // An SVG without the namespace declaration is one usvg would reject.
        ("a.svg", b"<svg><rect/></svg>".as_slice(), false),
    ] {
        let md = format!("# H\n\n![alt]({path})\n");
        let result = md_to_pdf(&md, &[asset(path, bytes)]);
        let rejected = matches!(result, Err(Error::ImageFormat { .. }));
        assert_eq!(!rejected, ok, "for {path} with {} bytes", bytes.len());
    }
}

// -- Phase 7: footnotes -----------------------------------------------------

#[test]
fn the_footnotes_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(FOOTNOTES_MD).unwrap(), FOOTNOTES_TYP);
}

/// The compile is the phase's observable, at the library level: markdown whose
/// footnotes reach the foot of the column that holds their references.
#[test]
fn the_footnotes_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(FOOTNOTES_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of a footnote reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that drops the content
/// from the first reference, numbers by definition order, or lets a user's own
/// label text through fails with a message that points at what it dropped.
#[test]
fn the_footnotes_golden_carries_each_form() {
    for (form, what) in [
        // The first reference carries the content, because Typst takes a
        // footnote's body at the reference site.
        (
            r#"#footnote[A note with #emph[emphasis], #raw("inline code"), and more below."#,
            "the first reference, carrying its definition",
        ),
        // Block content inside a definition arrives through the arms that serve
        // it everywhere else.
        (
            "A second paragraph inside the same definition.\n\n- a list item\n- a second list item]<fn-1>",
            "the second paragraph and the list inside the definition",
        ),
        // The repeat points at the name the first reference wrote. Its label is
        // spelled `[^AFTER]` in the fixture, so this pins the case fold too.
        ("#footnote(<fn-1>)", "the repeated reference"),
        // Numbered by first use, not by the order the definitions are written:
        // this one is defined first and referenced second.
        (
            "#footnote[A definition written above the reference that cites it.]<fn-2>",
            "the definition that precedes its reference",
        ),
    ] {
        assert!(
            FOOTNOTES_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // A markdown label may hold any character and a Typst label may not, so the
    // emitter generates the name and the user's own text never reaches Typst.
    for label in ["[^", "after", "AFTER", "before"] {
        assert!(
            !FOOTNOTES_TYP.contains(label),
            "the user's own label text `{label}` reached the output"
        );
    }

    // The definition's region is not emitted where it is written. Its content
    // appears once, at the reference that cites it.
    assert_eq!(
        FOOTNOTES_TYP.matches("A definition written above").count(),
        1,
        "a definition was left in the body as well"
    );
}

/// Each footnote shape the dialect refuses names its construct and its line.
///
/// The first would lose a body, and choosing between two is a guess. The second
/// would reach no page, and content that vanishes is what the escape-and-reject
/// rule exists to prevent. The third would need a recursive substitution with a
/// cycle check, for a construct real articles do not carry.
#[test]
fn each_footnote_error_shape_names_its_construct_and_its_line() {
    for (md, construct, line, what) in [
        (
            "# H\n\nA paragraph that cites nothing.\n\n[^a]: An orphan note.\n",
            "footnote definition that no reference cites",
            5,
            "the uncited definition",
        ),
        // The two spellings differ in case, which pins the fold on the error
        // path: the parser resolves both to one body, so the second is refused.
        (
            "# H\n\nA cited note[^a] here.\n\n[^a]: One body.\n\n[^A]: Another body.\n",
            "footnote definition for a label already defined",
            7,
            "the second definition for one label",
        ),
        (
            "# H\n\nA cited note[^a] here.\n\n[^a]: A note that cites[^b] another.\n\n[^b]: The other.\n",
            "footnote reference inside a footnote definition",
            5,
            "the reference inside a definition",
        ),
    ] {
        match md_to_typst(md) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                line: found_line,
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

/// A frontmatter error still wins over a definition error later in the file.
///
/// A definition is translated in a walk of its own, before the document's, and
/// this is what pins that the error it produced is reported where the region
/// sits rather than where that first walk met it.
#[test]
fn a_frontmatter_error_wins_over_a_later_footnote_error() {
    for (md, what) in [
        (
            "---\nsubtitle: Bad\n---\n\nA cited note[^a] here.\n\n[^a]: A note holding <b>raw HTML</b>.\n",
            "a definition whose own translation failed",
        ),
        (
            "---\nsubtitle: Bad\n---\n\nA paragraph.\n\n[^a]: An orphan note.\n",
            "an uncited definition",
        ),
    ] {
        match md_to_typst(md) {
            Err(Error::Frontmatter { problem, .. }) => {
                assert!(problem.contains("subtitle"), "problem was: {problem}");
            }
            other => panic!("expected a Frontmatter error over {what}, got {other:?}"),
        }
    }
}

/// A definition's images join the shopping list at its first reference.
///
/// The list runs in the order a reader meets the images, which is where the
/// content is set rather than where the definition is written. Each entry keeps
/// the line the markdown named it on, because that is what an error must say.
#[test]
fn image_paths_lists_a_definitions_images_at_its_first_reference() {
    let md = "![one](a.png)\n\nText[^n] and more[^n].\n\n![two](b.png)\n\n[^n]: A note ![in](c.png) here.\n";
    assert_eq!(
        image_paths(md).unwrap(),
        vec![
            ImageRef {
                path: "a.png".to_string(),
                line: 1
            },
            ImageRef {
                path: "c.png".to_string(),
                line: 7
            },
            ImageRef {
                path: "b.png".to_string(),
                line: 5
            },
        ]
    );
}

/// A reference with no definition produces no event and stays literal text.
///
/// That is the parser's own behaviour, and it is why a dangling reference needs
/// no error shape of its own: every reference the walk sees has a definition
/// somewhere in the document.
#[test]
fn a_footnote_reference_with_no_definition_stays_text() {
    let typst = md_to_typst("# H\n\nA dangling reference[^gone] here.\n").unwrap();
    assert!(
        typst.contains(r"reference\[^gone\] here."),
        "the dangling reference did not stay escaped text: {typst}"
    );
}

// -- Phase 8: strikethrough, and the two constructs beside it ----------------

#[test]
fn the_strikethrough_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(STRIKETHROUGH_MD).unwrap(), STRIKETHROUGH_TYP);
}

/// The compile is the phase's observable, at the library level: markdown whose
/// struck text reaches the page struck.
#[test]
fn the_strikethrough_fixture_compiles_to_a_pdf() {
    let assets = vec![asset("dot.png", DOT_PNG)];
    let pdf = md_to_pdf(STRIKETHROUGH_MD, &assets).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rule, so an edit that drops the one-tilde
/// form, refuses a strike inside alt text, or sends a `~~` in code through the
/// markup escape fails with a message that points at what it dropped.
#[test]
fn the_strikethrough_golden_carries_each_form() {
    for (form, what) in [
        // Typst has no markup for a strike, so the function form is the only
        // form there is.
        ("#strike[struck text]", "the strike, alone"),
        // The parser reads a delimiter run of one as well as one of two, so
        // this spelling is strikethrough under the dialect rather than prose.
        ("#strike[one tilde]", "the one-tilde form"),
        (
            "#emph[emphasis around #strike[a struck phrase]]",
            "the strike nested inside emphasis",
        ),
        (
            r#"#strike[#link("https://typst.app")[Typst]]"#,
            "the strike around a link",
        ),
        // A strike can occur inside alt content, so the capture treats it as a
        // wrapper: its inner text arrives and the wrapper contributes nothing.
        (
            r#"alt: "a struck caption""#,
            "the strike inside alt text, flattened",
        ),
        // Code content is a string literal, never markup, so a pair of tildes
        // inside it is not a delimiter run and reaches the page as itself.
        (r#"#raw("a ~~ pair")"#, "the tilde pair inside inline code"),
        // A backslash suppresses the math span, and the escape rule then puts
        // the dollar on the page as itself.
        (r"a \$5 to \$10 range", "the escaped dollar pair, as prose"),
    ] {
        assert!(
            STRIKETHROUGH_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// The construct beside strikethrough names itself and its line.
///
/// Both arms of `describe` were unreachable until this phase set their parser
/// options, so both printed their markers on the page while the code claimed to
/// refuse them.
///
/// Math was refused in both its forms when this phase shipped, and this test
/// held the display half after `mpdf-004` Phase 1 took the inline one. Phase 2
/// of that spec took the display form too, so the marker is what is left here:
/// `describe` no longer names math at all. `\$` is still the one-character way
/// to keep a dollar as prose, and the strikethrough golden above still pins it.
#[test]
fn each_refused_construct_names_itself_and_its_line() {
    for (md, construct, what) in [(TASK_LIST_MD, "task list marker", "the task list marker")] {
        match md_to_typst(md) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                line,
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(line, 3, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

// -- Phase 9: the look the frontmatter chooses -------------------------------

#[test]
fn the_dated_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(DATED_MD).unwrap(), DATED_TYP);
}

/// The compile is half of the phase's observable: the article look, dated.
#[test]
fn the_dated_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(DATED_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn the_press_release_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(PRESS_RELEASE_MD).unwrap(), PRESS_RELEASE_TYP);
}

/// The other half: the same dialect in a second look.
#[test]
fn the_press_release_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(PRESS_RELEASE_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// A document with no `template` key takes the article look, unchanged.
///
/// The equality tests above pin the whole output, but they cannot say *why* it
/// is right. This one names the rule, so an edit that renames the default look's
/// file, or that drops the date, fails with a message naming what it dropped.
#[test]
fn the_default_look_keeps_the_import_line_every_document_carried() {
    let typst_source = md_to_typst(DATED_MD).unwrap();
    assert!(
        typst_source.starts_with("#import \"template.typ\": template, divider\n"),
        "the import line moved: {typst_source}"
    );
    assert!(
        typst_source.contains("date: \"10 August 2026\""),
        "the date is not a string literal on the call: {typst_source}"
    );
}

/// The selected look reaches the import line, and its convention the call.
///
/// A fixed import name would make two documents in two looks emit identical
/// source, and `--emit-typst` exists to show what a document compiles to. The
/// column count comes from the look, because this fixture names none.
#[test]
fn a_press_release_names_its_own_file_and_takes_one_column() {
    let typst_source = md_to_typst(PRESS_RELEASE_MD).unwrap();
    assert!(
        typst_source.starts_with("#import \"press-release.typ\": template, divider\n"),
        "the import does not name the selected look: {typst_source}"
    );
    assert!(
        typst_source.contains("columns: 1,"),
        "the look's column count did not reach the call: {typst_source}"
    );
}

/// An explicit `columns` wins over the look's convention.
///
/// The convention applies where the document left the key out, and nowhere
/// else. Both orderings are tested, because the resolution runs after the whole
/// block is read rather than at the line the key sits on.
#[test]
fn an_explicit_column_count_wins_over_the_looks_convention() {
    for md in [
        "---\ntemplate: press-release\ncolumns: 2\n---\n\n# Heading\n",
        "---\ncolumns: 2\ntemplate: press-release\n---\n\n# Heading\n",
    ] {
        let typst_source = md_to_typst(md).unwrap();
        assert!(
            typst_source.contains("columns: 2,"),
            "the explicit count lost: {typst_source}"
        );
    }
}

/// A look outside the set names the key and lists what it accepts.
#[test]
fn a_template_name_outside_the_set_is_an_error_that_lists_the_names() {
    match md_to_typst(UNKNOWN_TEMPLATE_MD) {
        Err(Error::Frontmatter { line, problem }) => {
            assert_eq!(line, 3);
            for needle in ["template", "article", "press-release"] {
                assert!(problem.contains(needle), "problem was: {problem}");
            }
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

/// Every bundled look meets the call contract the emitter writes.
///
/// `header` names all four arguments on every call and imports both names, so a
/// look missing one would fail the compile with an error naming neither the
/// document nor the key. Golden files pin emitter output only, so the
/// templates' side of the contract needs an artifact of its own.
#[test]
fn every_bundled_template_meets_the_call_contract() {
    for (file, source) in BUNDLED_TEMPLATES {
        for needle in [
            "#let template(",
            "#let divider(",
            "title:",
            "author:",
            "columns:",
            "date:",
        ] {
            assert!(source.contains(needle), "{file} does not carry `{needle}`");
        }
    }
}

// -- mpdf-004 Phase 1: inline math -------------------------------------------

#[test]
fn the_math_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(MATH_MD).unwrap(), MATH_TYP);
}

/// The phase's observable, and the case that proves the bundled prelude
/// complete.
///
/// The fixture sets one formula per command the dialect allows, so a symbol the
/// derivation missed is an unknown identifier here and the compile fails. That
/// is why `mpdf-004` §2 states the prelude's derivation rather than its
/// membership: this test cannot be wrong about it, and a list in prose could.
#[test]
fn the_math_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(MATH_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole output, but it cannot say *why* the
/// output is right. This one names the rules an edit could quietly drop: that
/// the content is markup rather than escaped text, that the import is by name,
/// and that the escapes the dialect rests on reach the page.
#[test]
fn the_math_golden_carries_each_form() {
    for (form, what) in [
        // The prelude is imported by name, not with a glob: Typst searches user
        // scopes before the library, so `*` would shadow `image`, `table` and
        // `raw` — all of which the emitter calls — for the whole document.
        (
            r#"#import "math.typ": aligned, bmatrix, diff"#,
            "the prelude import, by name",
        ),
        // Converted markup, written between the delimiters unescaped. Through
        // `escape_into` this would read `frac\(a \,b \)` and set as letters.
        ("$frac(a ,b )$", "the fraction, as markup"),
        // A prelude member doing its job: `mat` with no delimiter.
        ("$matrix( a zws , b zws ; c zws , d )$", "the matrix"),
        // Version skew rather than a MiTeX helper: mitex 0.2.4 writes the
        // pre-0.13 spelling of `inter`, which is exactly the kind of entry a
        // hand-written prelude omits.
        ("$A sect B$", "the intersection, under mitex's own spelling"),
        // The escape the `%` refusal rests on. Refusing an unescaped `%` costs
        // the author nothing only because this reaches the page.
        ("$1 0 0 %$", "the escaped percent sign"),
        // A dollar inside a formula, and a dollar beside one. Both survive.
        ("$dollar 5$", "the escaped dollar, inside math"),
        (r"a \$5 to \$10 range", "the escaped dollar pair, as prose"),
    ] {
        assert!(
            MATH_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// Every shape the dialect refuses inside a formula names the LaTeX the author
/// typed, and its line.
///
/// The first four are the escapes that decided the design. `\includegraphics`
/// converts to an `#image` call for a path no check ever saw, `\label` to the
/// empty string, and `\begin{itemize}` to markup-mode list syntax that Typst
/// then flattens to an operator — none of them visible in the converted output,
/// which is why the check reads the LaTeX instead. `\notacommand` is the
/// ordinary unknown, and an implementer who tested only that one would ship the
/// other three.
///
/// The last two are not command refusals and no command list would reach them.
/// `%` opens a LaTeX comment, so `mitex` drops the rest of the line and the PDF
/// shows truncated prose. `\text` is the deferral `mpdf-004` OQ-6 carries, and
/// it is tested rather than trusted: an implementer who adds it to the list
/// passes every other case here and fails this one.
#[test]
fn each_math_refusal_names_its_latex_and_its_line() {
    for (md, problem, what) in [
        (
            REFUSED_MATH_MD,
            r"unsupported command '\includegraphics'",
            "the asset-contract escape, from the fixture",
        ),
        (
            "# H\n\nA $\\label{eq}$ here.\n",
            r"unsupported command '\label'",
            "the silent drop",
        ),
        (
            "# H\n\nA $\\begin{itemize}\\item a\\end{itemize}$ here.\n",
            "unsupported environment 'itemize'",
            "the silent flattening",
        ),
        (
            "# H\n\nA $\\notacommand$ here.\n",
            r"unsupported command '\notacommand'",
            "the ordinary unknown",
        ),
        (
            "# H\n\nA $x = 100 % of y$ here.\n",
            "unescaped '%' — write '\\%' for a percent sign",
            "the comment that would truncate the line",
        ),
        (
            "# H\n\nA $\\text{a}$ here.\n",
            r"unsupported command '\text'",
            "the deferred command",
        ),
    ] {
        match md_to_typst(md) {
            Err(Error::Math {
                problem: found,
                line,
            }) => {
                assert_eq!(found, problem, "for {what}");
                assert_eq!(line, 3, "for {what}");
            }
            other => panic!("expected `{problem}` for {what}, got {other:?}"),
        }
    }
}

/// The escaped percent sign converts and reaches the page.
///
/// This is the half that makes the refusal above a redirection rather than a
/// ban: the whole argument for refusing `%` is that `\%` costs one character and
/// works.
#[test]
fn the_escaped_percent_sign_converts_and_compiles() {
    let md = "# H\n\nA $100\\% of y$ here.\n";
    assert!(md_to_typst(md).unwrap().contains("$1 0 0 % o f y$"));

    let pdf = md_to_pdf(md, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// A document with no math imports no prelude.
///
/// The import is conditional for exactly this reason: every golden file shipped
/// before this phase opens with the same two lines it always did, so "no shipped
/// golden file changed" and "the prelude is bundled" are both true.
#[test]
fn a_document_without_math_imports_no_prelude() {
    for (typst, what) in [
        (md_to_typst(BASIC_MD).unwrap(), "the basic fixture"),
        (md_to_typst(FOOTNOTES_MD).unwrap(), "the footnotes fixture"),
    ] {
        assert!(!typst.contains("math.typ"), "{what} imports the prelude");
    }
    assert!(
        BASIC_TYP.starts_with("#import \"template.typ\": template, divider\n#show: template.with(")
    );
}

/// Math inside an image's alt text contributes its LaTeX source, and nothing
/// else.
///
/// Alt is plain text by CommonMark and Typst's `alt` is a string, so the span
/// contributes what the author typed and the wrapper contributes nothing — the
/// same disposition strikethrough got. No `$…$` is written into a string that
/// cannot typeset it, and a document whose only formula sits here imports no
/// prelude.
#[test]
fn math_in_alt_text_becomes_its_latex_source() {
    let md = "# H\n\nA ![a $x+y$ b](dot.png) here.\n";
    let typst = md_to_typst(md).unwrap();
    assert!(typst.contains(r#"alt: "a x+y b""#), "{typst}");
    assert!(!typst.contains("math.typ"), "{typst}");

    let pdf = md_to_pdf(md, &[asset("dot.png", DOT_PNG)]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// A formula inside a footnote definition still imports the prelude.
///
/// The definitions take a walk of their own, and that walk is thrown away once
/// its content is stored — so the flag that decides the import travels with the
/// content, exactly as the images do. Without it a document whose only formula
/// sits in a footnote emits `$…$` with nothing defining what it uses, and the
/// compile fails naming source the author never wrote.
#[test]
fn math_inside_a_footnote_definition_imports_the_prelude() {
    let md = "# H\n\nA claim.[^1]\n\n[^1]: Because $\\sqrt{x} \\leq x$ here.\n";
    let typst = md_to_typst(md).unwrap();
    assert!(typst.contains("math.typ"), "{typst}");
    assert!(typst.contains("$mitexsqrt(x ) <= x$"), "{typst}");

    let pdf = md_to_pdf(md, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

// -- mpdf-004 Phase 2: display math ------------------------------------------

#[test]
fn the_display_math_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(DISPLAY_MATH_MD).unwrap(), DISPLAY_MATH_TYP);
}

/// The phase's observable: a centred display equation, which is what a formula
/// on its own lines is for.
///
/// The fixture's standalone formula is `\sqrt`, whose head only the bundled
/// prelude defines. So an arm that wrote the block form but forgot `Walk.math`
/// fails here as an unknown identifier, rather than passing on a formula whose
/// heads Typst defines anyway.
#[test]
fn the_display_math_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(DISPLAY_MATH_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The two forms are distinct in the output, and the display one is Typst's
/// block.
///
/// `typst::syntax::ast::Equation::block` tests for a space immediately inside
/// each delimiter, so the spacing is the whole difference between a block and an
/// inline equation — an arm that wrote `$…$` for both would satisfy every other
/// assertion in this phase.
///
/// The second display span sits mid-sentence rather than alone in its
/// paragraph, because every other arm here is satisfied by a standalone one: an
/// arm that consulted position — block when alone, inline in a sentence — would
/// pass them all while contradicting the scope `mpdf-004` OQ-5 fixed.
#[test]
fn the_display_math_golden_carries_each_form() {
    for (form, what) in [
        // The flag reached the header from a document whose math is all display.
        (
            r#"#import "math.typ": aligned, bmatrix, diff"#,
            "the prelude import, by name",
        ),
        (
            "$ mitexsqrt(x ^(2 ) + y ^(2 )) <= | x | + | y | $",
            "the standalone display span, in the spaced block form",
        ),
        (
            "$ sum _(i = 1 )^(n ) i $",
            "the mid-paragraph display span, in that same block form",
        ),
        (
            "$frac(a ,b )$",
            "the inline span beside it, still in the unspaced form",
        ),
    ] {
        assert!(
            DISPLAY_MATH_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// A display span inside an image's alt text contributes its LaTeX source, the
/// way the inline one does.
///
/// This is the case a gate on the main walk alone would not reach. `describe` no
/// longer names math, so a capture with no arm for `DisplayMath` refuses an
/// in-dialect construct with the nonsense message `unsupported markdown
/// construct 'supported construct'`.
#[test]
fn display_math_in_alt_text_becomes_its_latex_source() {
    let md = "# H\n\nA ![a $$x+y$$ b](dot.png) here.\n";
    let typst = md_to_typst(md).unwrap();
    assert!(typst.contains(r#"alt: "a x+y b""#), "{typst}");
    assert!(!typst.contains("math.typ"), "{typst}");

    let pdf = md_to_pdf(md, &[asset("dot.png", DOT_PNG)]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// The scan is not silently inline-only.
///
/// Both arms call `core/src/math.rs:convert`, which scans before it converts, so
/// this is one mechanism rather than two — but an implementation that reached
/// for `mitex::convert_math` directly in the display arm would pass every other
/// case in this phase and let `\notacommand` through.
#[test]
fn a_refused_display_formula_names_its_latex_and_its_line() {
    let md = "# H\n\nA $$\\notacommand$$ here.\n";
    match md_to_typst(md) {
        Err(Error::Math { problem, line }) => {
            assert_eq!(problem, r"unsupported command '\notacommand'");
            assert_eq!(line, 3);
        }
        other => panic!("expected a math error, got {other:?}"),
    }
}

// -- mpdf-003 Phase 6: the page the author is on -----------------------------

/// The sample the app's own gate opens, and the two figures it names on lines
/// 158 and 162.
///
/// It is here rather than under `tests/fixtures/` because the case it serves is
/// about *pages*, and this is the document this project has that runs to three
/// of them. A fixture short enough to fit the others would answer 1 for every
/// heading and prove nothing.
const ARTICLE_MD: &str = include_str!("../../samples/article.md");
const PIPELINE_SVG: &[u8] = include_bytes!("../../samples/pipeline.svg");
const CHECK_SVG: &[u8] = include_bytes!("../../samples/check.svg");

fn article_assets() -> Vec<Asset> {
    vec![
        asset("pipeline.svg", PIPELINE_SVG),
        asset("check.svg", CHECK_SVG),
    ]
}

/// Every heading is anchored, at the line the markdown wrote it on, in order.
///
/// The lines are spelled out rather than computed, because a helper that
/// derived them would derive them the same way the walk does and agree with a
/// wrong answer.
#[test]
fn each_heading_is_anchored_at_its_own_line() {
    let md = "---\ntitle: T\n---\n\n# One\n\nText.\n\n## Two\n\nText.\n\n### Three\n\nText.\n";

    let rendered = md_to_pdf_with_anchors(md, &[]).unwrap();
    let lines: Vec<usize> = rendered.anchors.iter().map(|a| a.line).collect();
    assert_eq!(lines, vec![5, 9, 13], "{:?}", rendered.anchors);

    let pages: Vec<usize> = rendered.anchors.iter().map(|a| a.page).collect();
    assert!(
        pages.windows(2).all(|pair| pair[0] <= pair[1]),
        "the pages run backwards: {pages:?}"
    );
    assert!(
        pages.iter().all(|&page| page >= 1),
        "a page is numbered from zero: {pages:?}"
    );
}

/// The anchors reach a page that is not the first one.
///
/// **This is the case that proves the feature rather than the plumbing.** Every
/// other case here is met by an implementation that always answers 1, and so is
/// the app: a pane fed nothing but page 1 behaves exactly as it did before this
/// phase existed. `samples/article.md` runs to three pages, and its last heading
/// is on the last of them.
#[test]
fn the_articles_last_heading_is_not_on_the_first_page() {
    let rendered = md_to_pdf_with_anchors(ARTICLE_MD, &article_assets()).unwrap();

    let last = rendered.anchors.last().expect("the article names headings");
    assert!(
        last.page > 1,
        "the last heading came back on page {}: {:?}",
        last.page,
        rendered.anchors
    );
}

/// A document with no headings has no anchors, and the pane then asks for no
/// page — which is the same branch a caret above the first heading takes.
#[test]
fn a_document_with_no_headings_has_no_anchors() {
    let md = "---\ntitle: T\n---\n\nJust a paragraph.\n";

    let rendered = md_to_pdf_with_anchors(md, &[]).unwrap();
    assert!(rendered.anchors.is_empty(), "{:?}", rendered.anchors);
    assert!(rendered.pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// The counts can disagree over real markdown, and the guard then answers
/// nothing rather than guessing.
///
/// A heading inside a footnote definition is walked by
/// `core/src/emit.rs:collect_definitions` into a `Walk` that is discarded, and
/// its content is spliced in at the *reference* — so the compiled document holds
/// a heading the document walk never counted, and pairing by ordinal from there
/// on would name the wrong line for every heading after it.
#[test]
fn a_heading_inside_a_footnote_definition_withdraws_the_anchors() {
    let md = "---\ntitle: T\n---\n\n# One\n\nA claim.[^1]\n\n[^1]: # A heading in a note\n";

    // The mismatch is real: the walk sees one heading and the document typesets
    // two.
    let typst = md_to_typst(md).unwrap();
    assert!(
        typst.contains("#footnote[= A heading in a note]"),
        "the definition no longer splices a heading: {typst}"
    );

    let rendered = md_to_pdf_with_anchors(md, &[]).unwrap();
    assert!(
        rendered.anchors.is_empty(),
        "the guard let a mismatched pairing through: {:?}",
        rendered.anchors
    );
    assert!(rendered.pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// `md_to_pdf` returns what it always returned, and the anchors cost the bytes
/// nothing.
///
/// The wrapper is the point: two paths over the same input that could disagree
/// eventually do, so this asserts they are one path rather than two that happen
/// to agree today.
#[test]
fn the_anchors_change_no_byte_of_the_pdf() {
    let plain = md_to_pdf(ARTICLE_MD, &article_assets()).unwrap();
    let rendered = md_to_pdf_with_anchors(ARTICLE_MD, &article_assets()).unwrap();

    assert_eq!(
        plain, rendered.pdf,
        "the two calls produced different bytes"
    );
}
