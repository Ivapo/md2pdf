//! The exit gates for `mpdf-001`'s nine phases and `mpdf-002`'s first, at the
//! library level.
//!
//! Fixtures and golden files live at the workspace root because the CLI tests
//! read the same ones.

use md2pdf_core::{
    Asset, Error, ImageRef, Location, image_paths, md_to_pdf, md_to_pdf_with_anchors, md_to_typst,
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
const NUMBERED_EQUATIONS_MD: &str = include_str!("../../tests/fixtures/numbered_equations.md");
const NUMBERED_EQUATIONS_TYP: &str = include_str!("../../tests/golden/numbered_equations.typ");
const CAPTIONS_MD: &str = include_str!("../../tests/fixtures/captions.md");
const CAPTIONS_TYP: &str = include_str!("../../tests/golden/captions.typ");
const CAPTIONED_BLOCKS_MD: &str = include_str!("../../tests/fixtures/captioned_blocks.md");
const CAPTIONED_BLOCKS_TYP: &str = include_str!("../../tests/golden/captioned_blocks.typ");
const CROSS_REFERENCES_MD: &str = include_str!("../../tests/fixtures/cross_references.md");
const CROSS_REFERENCES_TYP: &str = include_str!("../../tests/golden/cross_references.typ");
const EQUATION_NAMES_MD: &str = include_str!("../../tests/fixtures/equation_names.md");
const EQUATION_NAMES_TYP: &str = include_str!("../../tests/golden/equation_names.typ");
const PLAIN_EQUATION_NAMES_MD: &str = include_str!("../../tests/fixtures/plain_equation_names.md");
const PLAIN_EQUATION_NAMES_TYP: &str = include_str!("../../tests/golden/plain_equation_names.typ");
const EQUATION_NAMES_APART_MD: &str = include_str!("../../tests/fixtures/equation_names_apart.md");
const EQUATION_NAMES_APART_TYP: &str = include_str!("../../tests/golden/equation_names_apart.typ");
const GROUPS_MD: &str = include_str!("../../tests/fixtures/groups.md");
const GROUPS_TYP: &str = include_str!("../../tests/golden/groups.typ");
const SECTIONED_FIGURES_MD: &str = include_str!("../../tests/fixtures/sectioned_figures.md");
const SECTIONED_FIGURES_TYP: &str = include_str!("../../tests/golden/sectioned_figures.typ");
const NUMBERED_HEADINGS_MD: &str = include_str!("../../tests/fixtures/numbered_headings.md");
const NUMBERED_HEADINGS_TYP: &str = include_str!("../../tests/golden/numbered_headings.typ");
const CITATIONS_MD: &str = include_str!("../../tests/fixtures/citations.md");
const CITATIONS_TYP: &str = include_str!("../../tests/golden/citations.typ");
const CITATIONS_PRESS_RELEASE_MD: &str =
    include_str!("../../tests/fixtures/citations_press_release.md");
const CITATIONS_PRESS_RELEASE_TYP: &str =
    include_str!("../../tests/golden/citations_press_release.typ");
const AUTHOR_DATE_MD: &str = include_str!("../../tests/fixtures/author_date.md");
const AUTHOR_DATE_TYP: &str = include_str!("../../tests/golden/author_date.typ");
const AUTHORS_MD: &str = include_str!("../../tests/fixtures/authors.md");
const AUTHORS_TYP: &str = include_str!("../../tests/golden/authors.typ");
const ONE_AFFILIATION_MD: &str = include_str!("../../tests/fixtures/one_affiliation.md");
const ORPHAN_MARKERS_MD: &str = include_str!("../../tests/fixtures/orphan_markers.md");
const ABSTRACT_MD: &str = include_str!("../../tests/fixtures/abstract.md");
const ABSTRACT_TYP: &str = include_str!("../../tests/golden/abstract.typ");
/// A master whose first named section is the whole of the abstract.
///
/// The case no single-file fixture can state: "first" is first in the joined
/// stream, so a first-block test written against the master rather than against
/// what `mpdf-008` joined would refuse this document.
const ABSTRACT_SECTIONS_MD: &str = include_str!("../../tests/fixtures/abstract_sections.md");
const SECTION_ABSTRACT_MD: &str = include_str!("../../tests/fixtures/sections/abstract.md");
const SECTION_INTRO_MD: &str = include_str!("../../tests/fixtures/sections/intro.md");
const KEYWORDS_MD: &str = include_str!("../../tests/fixtures/keywords.md");
const KEYWORDS_TYP: &str = include_str!("../../tests/golden/keywords.typ");
/// A document that opens keywords and no abstract.
///
/// The other direction of the import test: a golden of its own is what says the
/// two flags are independent, since a document with keywords and no abstract
/// moves no shipped golden and so nothing else would notice.
const KEYWORDS_ALONE_MD: &str = include_str!("../../tests/fixtures/keywords_alone.md");
const KEYWORDS_ALONE_TYP: &str = include_str!("../../tests/golden/keywords_alone.typ");
/// Keywords written *above* an abstract, which is the author's own order.
///
/// The case that fails an implementation that ordered the two constructs for
/// the author: floats stack in the order they are issued, so what is written
/// first is set first.
const KEYWORDS_FIRST_MD: &str = include_str!("../../tests/fixtures/keywords_first.md");
/// A master whose first named section is the whole of the keywords block.
const KEYWORDS_SECTIONS_MD: &str = include_str!("../../tests/fixtures/keywords_sections.md");
const SECTION_KEYWORDS_MD: &str = include_str!("../../tests/fixtures/sections/keywords.md");

/// Every bundled look, by the name the `template` key selects it with. **Six
/// tests read these**: the call contract `mpdf-001` Phase 9 fixed, and the five
/// look-side rules that no golden file can pin, because a golden pins emitter
/// output alone. The spec id is written out because this file now carries a
/// second Phase 9 — `mpdf-005`'s, below — and a bare phase number names neither.
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

/// A second SVG, for the one case that needs two images with the same written
/// name: a ring where `mark.svg` is a check, so the two figures differ by eye as
/// well as by bytes.
const RING_SVG: &[u8] = include_bytes!("../../tests/fixtures/ring.svg");

/// The bibliography the two citation fixtures name.
///
/// It rides the same `Asset` type an image does — a named blob, with nothing
/// image-specific in it — which is why this channel needed no new type.
const REFS_YML: &[u8] = include_bytes!("../../tests/fixtures/refs.yml");

/// The other format Typst reads, and the one whose key is a legal figure name.
///
/// `refs.yml`'s key carries a ':' and a '/', which `core/src/emit.rs:check_name`
/// refuses, so the label collision needs a bibliography this one can be named
/// against.
const REFS_BIB: &[u8] = include_bytes!("../../tests/fixtures/refs.bib");

fn citations_assets() -> Vec<Asset> {
    vec![asset("refs.yml", REFS_YML)]
}

/// The bibliography `author_date.md` names: four records of one to four
/// authors, a file of its own rather than four records added to `refs.yml`, so
/// `citations.md`'s golden and the both-formats test keep their key set.
const AUTHOR_DATE_YML: &[u8] = include_bytes!("../../tests/fixtures/author_date.yml");

fn author_date_assets() -> Vec<Asset> {
    vec![asset("author_date.yml", AUTHOR_DATE_YML)]
}

fn bib_assets() -> Vec<Asset> {
    vec![asset("refs.bib", REFS_BIB)]
}

fn images_assets() -> Vec<Asset> {
    vec![
        asset("dot.png", DOT_PNG),
        asset("mark.svg", MARK_SVG),
        asset("fig#2.png", DOT_PNG),
    ]
}

/// The master, and the three files it names in the order it names them.
///
/// Four files and one document: `mpdf-008` Phase 1's own fixture, and the only
/// pair here whose golden is not the emitter's answer to a single string.
const MULTI_FILE_MD: &str = include_str!("../../tests/fixtures/multi_file.md");
const MULTI_FILE_TYP: &str = include_str!("../../tests/golden/multi_file.typ");
const INTRODUCTION_MD: &str = include_str!("../../tests/fixtures/sections/introduction.md");
const METHOD_MD: &str = include_str!("../../tests/fixtures/sections/method.md");
const RESULTS_MD: &str = include_str!("../../tests/fixtures/sections/results.md");

/// The three sections, on the channel an image and a bibliography already ride.
fn multi_file_sections() -> Vec<Asset> {
    vec![
        section("sections/introduction.md", INTRODUCTION_MD),
        section("sections/method.md", METHOD_MD),
        section("sections/results.md", RESULTS_MD),
    ]
}

/// The two images the three sections name between them, read from where the
/// sections name them.
///
/// **They sit in `tests/fixtures/sections/`, beside the files that draw them**,
/// which is the layout this phase is for: `introduction.md` writes a bare
/// `dot.png` and the emitter resolves it against the folder that file lives in.
/// The copies beside the master stay where they are, because ten other fixtures
/// name them from there.
const SECTION_DOT_PNG: &[u8] = include_bytes!("../../tests/fixtures/sections/dot.png");
const SECTION_MARK_SVG: &[u8] = include_bytes!("../../tests/fixtures/sections/mark.svg");

/// The same three, and the two images they name between them.
fn multi_file_assets() -> Vec<Asset> {
    let mut assets = multi_file_sections();
    assets.push(asset("sections/dot.png", SECTION_DOT_PNG));
    assets.push(asset("sections/mark.svg", SECTION_MARK_SVG));
    assets
}

fn section(path: &str, text: &str) -> Asset {
    asset(path, text.as_bytes())
}

fn asset(path: &str, bytes: &[u8]) -> Asset {
    Asset {
        path: path.to_string(),
        bytes: bytes.to_vec(),
    }
}

#[test]
fn basic_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(BASIC_MD, &[]).unwrap(), BASIC_TYP);
}

#[test]
fn basic_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(BASIC_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn hostile_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(HOSTILE_MD, &[]).unwrap(), HOSTILE_TYP);
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
    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            location: Location { file: None, line },
            ..
        }) => assert_eq!(line, 7),
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

// -- Phase 2: frontmatter and column layout ---------------------------------

#[test]
fn the_frontmatter_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(FRONTMATTER_MD, &[]).unwrap(), FRONTMATTER_TYP);
}

#[test]
fn the_frontmatter_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(FRONTMATTER_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

#[test]
fn the_single_column_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(SINGLE_COLUMN_MD, &[]).unwrap(),
        SINGLE_COLUMN_TYP
    );
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
    let typst_source = md_to_typst(FRONTMATTER_MD, &[]).unwrap();
    assert!(
        typst_source.contains("title: \"A Minimal Example\""),
        "the title is missing"
    );
    assert!(
        typst_source.contains("author: ((name: \"Iva Po\", markers: ()),)"),
        "the author is missing"
    );
}

/// Absent frontmatter is valid, and every default applies.
#[test]
fn absent_frontmatter_gets_every_default() {
    assert!(
        md_to_typst(BASIC_MD, &[]).unwrap().contains(
            "template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: \"plain\", figures: \"flat\", headings: \"plain\", citations: \"numeric\")"
        ),
        "the defaults did not reach the template call"
    );
}

#[test]
fn an_unknown_frontmatter_key_is_an_error_that_names_it() {
    match md_to_typst(UNKNOWN_KEY_MD, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 3);
            assert!(problem.contains("subtitle"), "problem was: {problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

#[test]
fn a_columns_value_outside_the_schema_is_an_error_that_names_the_key() {
    let md = "---\ncolumns: 3\n---\n\n# Heading\n";
    match md_to_typst(md, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
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
    match md_to_typst(md, &[]) {
        Err(Error::Frontmatter { problem, .. }) => {
            assert!(problem.contains("subtitle"), "problem was: {problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

// -- Phase 3: inline constructs ---------------------------------------------

#[test]
fn the_inline_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(INLINE_MD, &[]).unwrap(), INLINE_TYP);
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
    assert_eq!(md_to_typst(HOSTILE_CODE_MD, &[]).unwrap(), HOSTILE_CODE_TYP);
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
    assert_eq!(md_to_typst(BLOCKS_MD, &[]).unwrap(), BLOCKS_TYP);
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
    assert_eq!(md_to_typst(LIST_SPACING_MD, &[]).unwrap(), LIST_SPACING_TYP);
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
    assert_eq!(md_to_typst(LINKS_MD, &[]).unwrap(), LINKS_TYP);
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
    match md_to_typst(HTML_MD, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
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
    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
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
    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
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
    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
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
    let typst = md_to_typst(md, &[]).unwrap();
    assert!(
        typst.contains(r#"#link("https://typst.app")[link]"#),
        "the link did not survive its empty title: {typst}"
    );
}

// -- Phase 6: tables --------------------------------------------------------

#[test]
fn the_table_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(TABLE_MD, &[]).unwrap(), TABLE_TYP);
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
    let typst = md_to_typst(md, &[]).unwrap();
    assert!(
        typst.contains("align: (left,),"),
        "the one alignment is not an array: {typst}"
    );
}

/// A table whose delimiter row sets no alignment leaves the argument out.
#[test]
fn a_table_without_alignments_omits_the_argument() {
    let md = "| a | b |\n| - | - |\n| 1 | 2 |\n";
    let typst = md_to_typst(md, &[]).unwrap();
    assert!(
        !typst.contains("align:"),
        "an alignment argument that says nothing was written: {typst}"
    );
}

// -- mpdf-002 Phase 1: images -----------------------------------------------

#[test]
fn the_images_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(IMAGES_MD, &[]).unwrap(), IMAGES_TYP);
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
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(
            typst.contains(expected),
            "the alt capture does not flatten {what} as `{expected}`: {typst}"
        );
    }
}

/// A construct outside the dialect still errors inside alt text.
#[test]
fn an_out_of_dialect_construct_inside_alt_text_is_still_an_error() {
    match md_to_typst("![before <b>bold</b> after](dot.png)\n", &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
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
        image_paths(md, &[]).unwrap(),
        vec![
            ImageRef {
                path: "a.png".to_string(),
                location: Location::at(1)
            },
            ImageRef {
                path: "b.svg".to_string(),
                location: Location::at(3)
            },
            ImageRef {
                path: "a.png".to_string(),
                location: Location::at(5)
            },
            ImageRef {
                path: "d.png".to_string(),
                location: Location::at(7)
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
        // A `..` is refused for where it lands, not for the segment it is
        // spelled with: written in the master there is nothing above to climb
        // into, so this one leaves the folder.
        (
            "# H\n\nA ![alt](../a.png) here.\n",
            "image with a path that leaves the document's folder",
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
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location: Location { file: None, line },
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
    let typst = md_to_typst("# H\n\nA ![alt](a.png \"\") here.\n", &[]).unwrap();
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
        Err(Error::MissingImage {
            path,
            location: Location { file: None, line },
        }) => {
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
        Err(Error::MissingImage {
            path,
            location: Location { file: None, line },
        }) => {
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
        Err(Error::ImageFormat {
            path,
            location: Location { file: None, line },
            format,
        }) => {
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
    assert_eq!(md_to_typst(FOOTNOTES_MD, &[]).unwrap(), FOOTNOTES_TYP);
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
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
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
        match md_to_typst(md, &[]) {
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
        image_paths(md, &[]).unwrap(),
        vec![
            ImageRef {
                path: "a.png".to_string(),
                location: Location::at(1)
            },
            ImageRef {
                path: "c.png".to_string(),
                location: Location::at(7)
            },
            ImageRef {
                path: "b.png".to_string(),
                location: Location::at(5)
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
    let typst = md_to_typst("# H\n\nA dangling reference[^gone] here.\n", &[]).unwrap();
    assert!(
        typst.contains(r"reference\[^gone\] here."),
        "the dangling reference did not stay escaped text: {typst}"
    );
}

// -- Phase 8: strikethrough, and the two constructs beside it ----------------

#[test]
fn the_strikethrough_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(STRIKETHROUGH_MD, &[]).unwrap(),
        STRIKETHROUGH_TYP
    );
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
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location: Location { file: None, line },
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
    assert_eq!(md_to_typst(DATED_MD, &[]).unwrap(), DATED_TYP);
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
    assert_eq!(
        md_to_typst(PRESS_RELEASE_MD, &[]).unwrap(),
        PRESS_RELEASE_TYP
    );
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
    let typst_source = md_to_typst(DATED_MD, &[]).unwrap();
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
    let typst_source = md_to_typst(PRESS_RELEASE_MD, &[]).unwrap();
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
        let typst_source = md_to_typst(md, &[]).unwrap();
        assert!(
            typst_source.contains("columns: 2,"),
            "the explicit count lost: {typst_source}"
        );
    }
}

/// A look outside the set names the key and lists what it accepts.
#[test]
fn a_template_name_outside_the_set_is_an_error_that_lists_the_names() {
    match md_to_typst(UNKNOWN_TEMPLATE_MD, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
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
/// `header` names all nine arguments on every call, and imports two of the
/// four exported names on every document, the third on a document that opened
/// an abstract and the fourth on one that opened a keywords block, so a look
/// missing one would fail the compile with an error naming neither the document
/// nor the key. Golden files pin emitter output only, so the templates' side of
/// the contract needs an artifact of its own.
///
/// **`abstract` and `keywords` join the export needles rather than taking a test
/// of their own**, where a caption, a gutter and a listing's inset each took
/// one: those are `show` rules crossing no argument, and these are names the
/// import line names. Neither is a parameter — the call stood at eight from
/// `mpdf-001` Phase 11 until `citations` made it nine in `mpdf-007` Phase 5,
/// and both front-matter blocks crossed as exported names rather than moving
/// it — and what a label looks like, and whether a look sets one at all, is
/// each look's own call, which is why the words `Abstract` and `Keywords` are
/// deliberately not needles.
///
/// `equations` brings a second needle with it, `math.equation`: the parameter
/// alone would be satisfied by a look that took the argument and ignored it,
/// and this is the Typst element any numbering rule has to reach. The format
/// string is deliberately not a needle — `(1)` against `1.` is each look's own
/// call, which is the seam the phase rests on. A rule that reaches the element
/// but from inside a scoped block satisfies both needles and still numbers
/// nothing, which is why one PDF per look is read by eye as well.
///
/// `figures` brings its own pair on that precedent, and the second is
/// `counter(figure.where(kind:` — the per-section counter reset, which is the
/// one line no look can carry while ignoring the key. `set figure(numbering:`
/// would not do: a look may set that and never section anything. The format
/// stays off the list for the same reason `equations`' does.
///
/// `headings` brings its own pair on the same precedent, and the second is
/// `int(headings)` — the conversion the cap comparison needs, which is the one
/// fragment no look can carry while ignoring the key. **`n.pos().len() <=` is
/// deliberately not the needle**: a look that hardcoded its own depth carries
/// it and never reads the key, so it would check the rule's shape where
/// `int(headings)` checks that the key reaches it. The `1.1` pattern stays off
/// the list for the same reason the other two formats do.
///
/// `affiliation` brings its own pair on that same precedent, and the second is
/// `super(` — the superscript a marker is rendered as, which is the one call no
/// look can carry while ignoring the relation. The parameter alone would be
/// satisfied by a look that took both lists and printed the names alone. What a
/// marker looks like past being a superscript — its separator, whether the
/// affiliations set italic, how far beneath the names they sit — stays off the
/// list for the reason the three formats do.
///
/// `citations` brings its own pair on the same precedent, and the second is
/// `harvard-cite-them-right` — the style name only a look that maps the scheme
/// can carry. Round 1 of the phase's review found that without it a
/// press-release look writing `"ieee"` unconditionally passed every case,
/// since the fixture, the cross-tree hash and the showcase all exercise the
/// article look. `"ieee"` is deliberately not a needle: a look that wrote
/// `auto` would render the default's page, and the hash is what holds that,
/// not a string.
///
/// These needles join the contract test rather than taking one of their own,
/// where a caption, a gutter and a listing's alignment each took one: those
/// cross no argument at all, and this is a call-contract parameter, which is
/// what `equations` established here.
#[test]
fn every_bundled_template_meets_the_call_contract() {
    for (file, source) in BUNDLED_TEMPLATES {
        for needle in [
            "#let template(",
            "#let divider(",
            "#let abstract(",
            "#let keywords(",
            "title:",
            "author:",
            "affiliation",
            "super(",
            "columns:",
            "date:",
            "equations",
            "math.equation",
            "figures",
            "counter(figure.where(kind:",
            "headings",
            "int(headings)",
            "citations",
            "harvard-cite-them-right",
        ] {
            assert!(source.contains(needle), "{file} does not carry `{needle}`");
        }
    }
}

// -- mpdf-004 Phase 1: inline math -------------------------------------------

#[test]
fn the_math_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(MATH_MD, &[]).unwrap(), MATH_TYP);
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
        match md_to_typst(md, &[]) {
            Err(Error::Math {
                problem: found,
                location: Location { file: None, line },
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
    assert!(md_to_typst(md, &[]).unwrap().contains("$1 0 0 % o f y$"));

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
        (md_to_typst(BASIC_MD, &[]).unwrap(), "the basic fixture"),
        (
            md_to_typst(FOOTNOTES_MD, &[]).unwrap(),
            "the footnotes fixture",
        ),
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
    let typst = md_to_typst(md, &[]).unwrap();
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
    let typst = md_to_typst(md, &[]).unwrap();
    assert!(typst.contains("math.typ"), "{typst}");
    assert!(typst.contains("$mitexsqrt(x ) <= x$"), "{typst}");

    let pdf = md_to_pdf(md, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

// -- mpdf-004 Phase 2: display math ------------------------------------------

#[test]
fn the_display_math_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(DISPLAY_MATH_MD, &[]).unwrap(), DISPLAY_MATH_TYP);
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
    let typst = md_to_typst(md, &[]).unwrap();
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
    match md_to_typst(md, &[]) {
        Err(Error::Math {
            problem,
            location: Location { file: None, line },
        }) => {
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
    let lines: Vec<usize> = rendered.anchors.iter().map(|a| a.location.line).collect();
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
    let typst = md_to_typst(md, &[]).unwrap();
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

// -- mpdf-004 Phase 3: numbered display equations -----------------------------

#[test]
fn the_numbered_equations_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(NUMBERED_EQUATIONS_MD, &[]).unwrap(),
        NUMBERED_EQUATIONS_TYP
    );
}

/// The phase's observable, as far as an automated instrument reaches.
///
/// A golden pins emitter output and `%PDF` pins nothing about a page, so
/// neither can see the number this phase exists to put there. That is read by
/// eye, once, on one PDF per look — the answer `mpdf-001` Phase 9 recorded for
/// the same wall. What these two cases hold is that the argument reaches the
/// look in the form the look can use, and that the document still compiles with
/// it.
#[test]
fn the_numbered_equations_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(NUMBERED_EQUATIONS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
#[test]
fn the_numbered_equations_golden_carries_each_form() {
    for (form, what) in [
        (
            r#"equations: "numbered""#,
            "the key, quoted — unquoted it would fail the compile as an unknown variable",
        ),
        (
            "$ aligned( a &= b + c \\ &= b + d + e \\ &= f ) $",
            "the three-line derivation, as one block equation and so one number",
        ),
        (
            "$ sum _(i = 1 )^(n ) i = frac(n \\(n + 1 \\),2 ) $",
            "the span after it, which takes the next number",
        ),
        (
            "$frac(a ,b )$",
            "the inline span, still unspaced — Typst numbers the block form alone",
        ),
    ] {
        assert!(
            NUMBERED_EQUATIONS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// The default is inert: naming `plain` is the same as leaving the key out.
///
/// "Byte-identical to what it compiled to before this phase" has no referent in
/// the repo — no PDF is committed — so the property is held inside one commit
/// instead, by the two forms of the default compiling to the same bytes. An
/// implementation that made `numbered` the default fails here.
#[test]
fn the_two_forms_of_the_default_compile_to_the_same_bytes() {
    let body = "\n# Heading\n\nProse.\n\n$$\n\\sqrt{x^2 + y^2}\n$$\n";
    let absent = md_to_pdf(&format!("---\ntitle: A\n---\n{body}"), &[]).unwrap();
    let explicit = md_to_pdf(
        &format!("---\ntitle: A\nequations: plain\n---\n{body}"),
        &[],
    )
    .unwrap();

    assert!(absent.starts_with(b"%PDF"), "the output is not a PDF");
    assert_eq!(
        absent, explicit,
        "an absent `equations` key and `equations: plain` produced different bytes"
    );
}

/// A name outside the set names the key, its line, and what it accepts.
///
/// This is the case that would otherwise let a bad name through the schema and
/// reach Typst as `unknown variable`, naming an identifier the author never
/// typed. It reads exactly as the `template` key's error does, because it is
/// the same mechanism.
#[test]
fn an_equations_value_outside_the_schema_is_an_error_that_lists_the_names() {
    let md = "---\nequations: yes\n---\n\n# Heading\n";
    match md_to_typst(md, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 2);
            for needle in ["equations", "plain", "numbered"] {
                assert!(problem.contains(needle), "problem was: {problem}");
            }
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

// -- mpdf-005 Phase 1: a captioned figure -------------------------------------

/// The phase's own document, and the one that pins every case here but the
/// refusals.
///
/// It carries six shapes on purpose. The captioned figure is the observable;
/// the two consecutive standalone images, the footnote-final one and the
/// document-final one are the three the rejected design was measured breaking;
/// and the inline image with a `: ` paragraph beneath it, plus the image with
/// `: …` on the very next line, are what hold the marker to one paragraph in
/// one position.
///
/// They are here rather than in `tests/fixtures/images.md` or
/// `tests/fixtures/footnotes.md` because editing either would move a shipped
/// golden, which this phase asserts it does not — the same reason `mpdf-004`
/// Phase 2 wrote a fixture of its own rather than extending `math.md`.
#[test]
fn the_captions_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(CAPTIONS_MD, &[]).unwrap(), CAPTIONS_TYP);
}

#[test]
fn the_captions_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(CAPTIONS_MD, &images_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole file, but it cannot say which line is
/// load-bearing. These name the rules, so an edit that drops one fails with a
/// message pointing at the rule it dropped.
#[test]
fn the_captions_golden_carries_each_form() {
    for (form, what) in [
        (
            r#"#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: ["#,
            "the captioned image, wrapped — the call the phase exists to write",
        ),
        (
            "#emph[emitter]",
            "the caption's emphasis, as Typst markup — a caption is prose, not a string literal",
        ),
        (
            r#"#raw("raw")"#,
            "the caption's code span, walked the same way for the same reason",
        ),
        (
            "#image(\"mark.svg\", alt: \"a check mark\")\n\n#image(\"dot.png\", alt: \"The three steps again\")",
            "two consecutive standalone images, both bare — the shape a longer deferral demoted",
        ),
        (
            "#footnote[The last block of this definition is an image.\n\n#image(",
            "a definition whose last block is a standalone image, still standalone",
        ),
        (
            "#box(image(\"mark.svg\", alt: \"a check mark\")) sits inside this sentence",
            "the inline form, which takes no caption from the paragraph beneath it",
        ),
        (
            "\n: This paragraph follows prose rather than a figure",
            "a `: ` paragraph following prose, reaching the page as the prose it is",
        ),
        (
            "#box(image(\"dot.png\", alt: \"The three steps, drawn as boxes\"))\n: This line has no blank line",
            "the missing blank line: one paragraph, so the inline form and a literal marker",
        ),
    ] {
        assert!(
            CAPTIONS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// An image with no caption line beneath it is the bare block it always was.
///
/// **This is the case that holds the phase's central decision**, and the one an
/// implementer who wraps every standalone image fails while passing everything
/// above. Round 1 measured what that costs: an uncaptioned `#figure` prints no
/// number and still consumes the counter, so the next captioned one reads
/// "Figure 2" with no Figure 1 on the page, and `figure` centres its body where
/// a bare block sits flush left — a bounding box moving from `xMin=70.87` to
/// `277.48`.
///
/// **The needle is `#figure(` and not `figure`.** Every golden's
/// `template.with` line has carried a `figures:` argument since `mpdf-005`
/// Phase 7, so the looser word is in all 27 of them and would fail here for the
/// key's name rather than for a wrapper. `#figure(` is what
/// `core/src/emit.rs:splice_caption` writes, which is the thing this asserts the
/// absence of.
#[test]
fn an_uncaptioned_standalone_image_is_not_wrapped() {
    assert_eq!(md_to_typst(IMAGES_MD, &[]).unwrap(), IMAGES_TYP);
    assert!(
        !IMAGES_TYP.contains("#figure("),
        "the images golden gained a figure"
    );
}

/// Each caption shape the dialect refuses names its construct and its line.
///
/// The first would put a bare "Figure 1:" on the page. The second is the one
/// that does not fall out of the record's own checks — a spliced region carries
/// `#figure(…)`, which the content check accepts, so an implementer who left it
/// to those three gets silence here instead of an error.
///
/// A third case stood here until Phase 3: a caption ending in `{#name}`, which
/// that phase turns into a Typst label. What replaces it is
/// `each_name_refusal_names_the_authors_line`, and the shape it protected —
/// a name that silently does nothing — is still refused, one level down.
#[test]
fn each_caption_refusal_names_its_construct_and_its_line() {
    for (md, construct, line, what) in [
        (
            "# H\n\n![alt](dot.png)\n\n:\n",
            "caption with no text",
            5,
            "the marker with nothing after it",
        ),
        (
            "# H\n\n![alt](dot.png)\n\n: A caption.\n\n: A second one.\n",
            "second caption for one figure",
            7,
            "a second caption for one figure",
        ),
    ] {
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

/// Both bundled looks decide what a caption looks like.
///
/// Deliberately not an extension of
/// `every_bundled_template_meets_the_call_contract`: that test is named for the
/// five-argument call contract, and a caption does not widen it — a `show` rule
/// over an element the emitter emits needs no export, the way both looks
/// already reach `raw` and `table.cell`. Hanging these needles off it would
/// leave a test whose name had stopped describing it.
///
/// The caption's *position* is deliberately not a needle. Above or below is
/// each look's own call, which is the seam this phase rests on, and where it
/// landed is what the by-eye read on one PDF per look is for.
#[test]
fn every_bundled_template_styles_a_caption() {
    for (file, source) in BUNDLED_TEMPLATES {
        for needle in ["show figure", "figure.caption"] {
            assert!(source.contains(needle), "{file} does not carry `{needle}`");
        }
    }
}

// -- mpdf-005 Phase 2: tables and listings take the same treatment ------------

/// The phase's own document, and one document rather than three.
///
/// **The property this phase exists for is that the three counters do not
/// share**, and only one document can show that — so this fixture carries a
/// captioned image beside the captioned table and the captioned code block,
/// rather than a fixture per construct. It then carries an uncaptioned table
/// followed by a captioned listing, which is the case Phase 1 had only one
/// recordable construct to express.
///
/// It is a fixture of its own rather than an extension of
/// `tests/fixtures/captions.md`, whose golden is shipped work this phase
/// asserts does not move — the same reason Phase 1 wrote one rather than
/// extending `tests/fixtures/images.md`.
#[test]
fn the_captioned_blocks_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(CAPTIONED_BLOCKS_MD, &[]).unwrap(),
        CAPTIONED_BLOCKS_TYP
    );
}

/// The compile is what catches the inner `#`.
///
/// `table_call` wrote `#table(` where `image_call` writes `image(` without one,
/// and a `#` inside a code context is a **syntax error** rather than a
/// mismatch. So an implementer who leaves it in fails here rather than in the
/// comparison above, with a message about generated source the author never
/// wrote — which is why this case exists as well as the golden.
#[test]
fn the_captioned_blocks_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(CAPTIONED_BLOCKS_MD, &[asset("dot.png", DOT_PNG)]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole file, but it cannot say which line is
/// load-bearing. These name the rules, so an edit that drops one fails with a
/// message pointing at the rule it dropped.
#[test]
fn the_captioned_blocks_golden_carries_each_form() {
    for (form, what) in [
        // The two calls this phase exists to write, each wrapping a bare call
        // rather than one carrying its own `#`.
        (
            "#figure(table(\n  columns: 2,",
            "the captioned table, wrapped",
        ),
        (
            r#"#figure(raw(block: true, lang: "rust", "fn main("#,
            "the captioned code block, wrapped",
        ),
        // The third kind, unchanged from Phase 1, in the same document — which
        // is what makes the three counters readable apart on the page.
        (
            r#"#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: ["#,
            "the captioned image beside them",
        ),
        // Phase 1's "walked as inline markdown", held over the constructs it
        // did not cover.
        (
            "caption: [The constructs and the #emph[counters] they keep.]",
            "the table caption's emphasis, as Typst markup",
        ),
        (
            r#"#raw("raw") span in its caption"#,
            "the listing caption's code span, walked the same way",
        ),
        // A caption reaches the construct above it and no other: the record is
        // the last one written rather than the last one of its kind. Phase 1
        // had one recordable construct and could not express this.
        (
            "#table(\n  columns: 2,\n  table.header([Uncaptioned], [Table]),\n  [stays], [bare],\n)\n\n#figure(raw(block: true, lang: \"rust\", \"fn second() {}\")",
            "the uncaptioned table left bare beneath a captioned listing",
        ),
    ] {
        assert!(
            CAPTIONED_BLOCKS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // The inner `#` again, this time as a property of the whole file rather
    // than of the two calls above: nothing a `#figure(…)` wraps carries one.
    assert!(
        !CAPTIONED_BLOCKS_TYP.contains("#figure(#"),
        "a wrapped call kept its own `#`, which is a Typst syntax error"
    );
}

/// An uncaptioned table and an uncaptioned code block are the bare blocks they
/// always were.
///
/// **This is Phase 1's central decision applied to two more constructs** — the
/// caption is what makes a figure — and the case an implementer who wraps
/// unconditionally fails while passing everything above. An uncaptioned
/// `#figure` prints no number and still consumes the counter, so the next
/// captioned one reads "Table 2" with no Table 1 on the page, and `figure`
/// centres its body where a bare block sits at its own edge — the prose's for a
/// table, and 2em off it for a code block since `mpdf-005` Phase 9 inset every
/// block of code in both looks. The counter half of the argument carries the
/// decision on its own, which is why the qualification costs it nothing.
///
/// The blast radius is one golden each: `tests/golden/table.typ` carries the
/// only `#table(` in the corpus and `tests/golden/blocks.typ` the only
/// `#raw(block: true`.
///
/// **The needle is `#figure(` and not `figure`**, for the reason
/// `an_uncaptioned_standalone_image_is_not_wrapped` records: the `figures:`
/// argument puts the looser word on every golden's `template.with` line.
#[test]
fn an_uncaptioned_table_and_code_block_are_not_wrapped() {
    assert_eq!(md_to_typst(TABLE_MD, &[]).unwrap(), TABLE_TYP);
    assert_eq!(md_to_typst(BLOCKS_MD, &[]).unwrap(), BLOCKS_TYP);

    for (golden, what) in [(TABLE_TYP, "table"), (BLOCKS_TYP, "blocks")] {
        assert!(
            !golden.contains("#figure("),
            "the {what} golden gained a figure"
        );
    }
}

/// Phase 1's caption refusals hold over both new constructs, each naming its
/// line.
///
/// They run through the code Phase 1 shipped, so this is a regression net
/// rather than new behaviour — and it is cheap, which is the argument for
/// having it rather than assuming it. The message reads "second caption for one
/// figure" over a table too, because a captioned table *is* a Typst `figure`
/// and the message names the element the emitter writes.
///
/// There were three until Phase 3, whose label takes the `{#name}` case out of
/// this net and into `each_name_refusal_names_the_authors_line`.
#[test]
fn each_caption_refusal_holds_over_a_table_and_a_code_block() {
    let table = "# H\n\n| a |\n| - |\n| 1 |\n\n";
    let block = "# H\n\n```\nx = 1\n```\n\n";

    for (md, construct, line, what) in [
        (
            format!("{table}:\n"),
            "caption with no text",
            7,
            "the empty marker after a table",
        ),
        (
            format!("{table}: One.\n\n: Two.\n"),
            "second caption for one figure",
            9,
            "the second caption after a table",
        ),
        (
            format!("{block}:\n"),
            "caption with no text",
            7,
            "the empty marker after a code block",
        ),
        (
            format!("{block}: One.\n\n: Two.\n"),
            "second caption for one figure",
            9,
            "the second caption after a code block",
        ),
    ] {
        match md_to_typst(&md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

/// An indented code block takes a caption exactly as a fenced one does.
///
/// **The code-block arm is one arm**: the two differ only in whether a `lang`
/// argument is written, and both reach Typst as `raw(block: true, …)`.
/// Splitting them would make a `: ` line a caption after one kind of block and
/// prose after another, with nothing on the page to tell an author which they
/// had written. The fixture carries fenced blocks alone, so this is the case
/// that holds the other half.
#[test]
fn an_indented_code_block_takes_a_caption_too() {
    let typst = md_to_typst("# H\n\n    x = 1\n\n: The block above.\n", &[]).unwrap();
    assert!(
        typst.contains(r#"#figure(raw(block: true, "x = 1"), caption: [The block above.])"#),
        "an indented block took no caption: {typst}"
    );
}

// -- mpdf-005 Phase 3: labels and cross-references ----------------------------

/// The phase's own document: three named kinds, and a reference to each.
///
/// A fixture of its own rather than an extension of `captions.md` or
/// `captioned_blocks.md`, whose goldens are shipped work this phase asserts
/// does not move — the same reason each of the two phases before it wrote one.
///
/// It carries the shapes the gate names and nothing it does not need: a named
/// image, table and listing; a reference ending a sentence; a reference
/// standing above the caption it names; two names whose prefixes say nothing
/// about their kinds; the two link forms `[](#name)` does not touch; a name
/// declared inside a footnote definition; and a display equation, which takes
/// no name at all.
#[test]
fn the_cross_references_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(CROSS_REFERENCES_MD, &[]).unwrap(),
        CROSS_REFERENCES_TYP
    );
}

#[test]
fn the_cross_references_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(CROSS_REFERENCES_MD, &images_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each part of the fixture reaches Typst as the form the spec chose.
///
/// The equality test above pins the whole file, but it cannot say which line is
/// load-bearing. These name the rules, so an edit that drops one fails with a
/// message pointing at the rule it dropped.
#[test]
fn the_cross_references_golden_carries_each_form() {
    for (form, what) in [
        // The label rides the call it names, in the same string the record
        // keeps, and the reference is the function form rather than `@name`.
        (
            "caption: [The conversion pipeline.]) <fig:pipeline>",
            "the label, immediately after the figure it names",
        ),
        (
            "As #ref(<fig:pipeline>) shows",
            "the reference, as `ref` rather than as a link",
        ),
        (
            "#ref(<tab:counters>)",
            "the table's reference, which the table's own caption declared",
        ),
        (
            "#ref(<lst:main>)",
            "the listing's reference, declared the same way",
        ),
        // The caption text as an exact string. The `{#name}` group leaves the
        // caption, and an implementation that leaves it there passes every
        // other case here — the golden it writes itself included. Asserted
        // positively rather than by sweeping for `{`, because this same fixture
        // carries a named listing and a `raw` string routinely holds a brace.
        (
            "caption: [The constructs and their #emph[counters].]) <tab:counters>",
            "the group gone from the caption, the emphasis beside it kept",
        ),
        // A reference in the shape ordinary prose puts it in: one ending a
        // sentence, so a full stop and a space follow the call.
        (
            "as it does in #ref(<fig:pipeline>). The prose",
            "a reference ending a sentence",
        ),
        // A reference may precede the caption that declares it, which is why
        // the undeclared check runs after the walk rather than during it.
        (
            "so #ref(<fig:later>) resolves",
            "a reference standing above its own declaration",
        ),
        // The prefix is not a kind. Both of these are images, and both are
        // figures, whatever the name in front of the colon says.
        (
            r#"#figure(image("dot.png", alt: "The three steps again"), caption: [A figure named with no prefix at all.]) <pipeline>"#,
            "a figure named with no prefix",
        ),
        (
            r#"alt: "A check mark again"), caption: [A figure named with a table's prefix.]) <tab:pipeline>"#,
            "a figure named with a table's prefix, still a figure",
        ),
        // OQ-8's scoping, both halves. Empty means empty.
        (
            r##"#link("#fig:pipeline")[some words]"##,
            "a link that carries text, untouched whatever its destination",
        ),
        (
            r##"#link("#fig:pipeline")[ ]"##,
            "a link whose text is one space, which is text",
        ),
        // A name declared inside a definition, and the reference outside that
        // reaches it. The document's walk skips the region, so the name travels
        // with the body.
        (
            "caption: [The figure inside the note.]) <fig:note>]<fn-1>",
            "the label inside the footnote definition, beside the generated one",
        ),
        (
            "#ref(<fig:note>) reaches it from out here",
            "the reference outside the definition that declared the name",
        ),
    ] {
        assert!(
            CROSS_REFERENCES_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // The reference is a `ref` and not a link with no content. An
    // implementation that left the link arm alone writes `#link("#…")[]`, which
    // compiles and puts nothing on the page — the silent drop the dialect
    // refuses, and the shape OQ-8 measured before claiming it.
    assert!(
        !CROSS_REFERENCES_TYP.contains(r##"#link("#fig:pipeline")[]"##),
        "a reference reached the page as an empty link"
    );
}

/// **The property the phase exists for, and the one no golden can see.**
///
/// Two documents differing only by a captioned figure inserted above the
/// referenced one. The emitted Typst for the reference is byte-identical in
/// both — the number is Typst's, which is the point rather than a limitation —
/// so what the source can show is that the emitter wrote the same thing twice.
/// That the page then reads *Figure 1* in one and *Figure 2* in the other is
/// read by eye, with the rest of gate (9).
#[test]
fn a_reference_stays_true_when_a_figure_is_inserted_above_it() {
    let inserted = "![An earlier diagram](mark.svg)\n\n: The figure inserted above.\n\n";
    let tail = concat!(
        "![The three steps](dot.png)\n\n",
        ": The conversion pipeline. {#fig:pipeline}\n\n",
        "As [](#fig:pipeline) shows, the emitter sits in the middle.\n",
    );

    let one = format!("# H\n\n{tail}");
    let two = format!("# H\n\n{inserted}{tail}");

    let sentence = "As #ref(<fig:pipeline>) shows, the emitter sits in the middle.";
    for (md, what) in [
        (&one, "the document without the insertion"),
        (&two, "with it"),
    ] {
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(
            typst.contains(sentence),
            "{what} did not carry the reference as `{sentence}`: {typst}"
        );

        let pdf = md_to_pdf(md, &images_assets()).unwrap();
        assert!(pdf.starts_with(b"%PDF"), "{what} is not a PDF");
    }
}

/// Each name the dialect refuses names the author's own line.
///
/// Every one of these is `core`'s rather than Typst's for a measured reason.
/// An undeclared reference fails the compile with ``label `<nosuchthing>` does
/// not exist in the document``, a repeated one with ``label `<dup>` occurs
/// multiple times in the document``, and the reserved collision with that same
/// message — all naming a Typst label the author never typed, none carrying a
/// line. A name opening with `:` raises nothing at all: it is not a label, so
/// `<:foo>` reaches the page as literal text, which is the silent drop the
/// dialect exists to refuse.
#[test]
fn each_name_refusal_names_the_authors_line() {
    let image = "# H\n\n![alt](dot.png)\n\n";

    for (md, line, needle, what) in [
        (
            format!("{image}: A caption. {{#fig one}}\n"),
            5,
            "letters, digits, '-', '_', ':' and '.'",
            "a character outside the set, the error listing the set",
        ),
        (
            format!("{image}: A caption. {{#:foo}}\n"),
            5,
            "begins with ':' or '.'",
            "a name Typst would not read as a name at all",
        ),
        (
            format!("{image}: A caption. {{#.foo}}\n"),
            5,
            "begins with ':' or '.'",
            "the same rule over a leading full stop",
        ),
        (
            format!("{image}: A caption. {{#fn-1}}\n"),
            5,
            "reserved for footnotes",
            "the namespace the emitter already owns",
        ),
        (
            format!("{image}: A caption. {{#one}}\n\n![alt](dot.png)\n\n: Another. {{#one}}\n"),
            9,
            "declared twice",
            "a name declared twice, refused where the second one stands",
        ),
        (
            format!("{image}: A caption. {{#one}}\n\nA reference to [](#other).\n"),
            7,
            "nothing declares the name 'other'",
            "a reference to a name the document does not declare",
        ),
    ] {
        match md_to_typst(&md, &[]) {
            Err(Error::Name {
                location:
                    Location {
                        file: None,
                        line: found,
                    },
                problem,
            }) => {
                assert_eq!(found, line, "for {what}");
                assert!(
                    problem.contains(needle),
                    "for {what}, the problem `{problem}` does not name `{needle}`"
                );
            }
            other => panic!("expected a name error for {what}, got {other:?}"),
        }
    }
}

/// Where two references are undeclared, the error names the earlier line.
///
/// Asserted rather than assumed, because the obvious container is a set and
/// "the first" out of one varies between runs. The document is built so that
/// document order and collection order disagree: the reference inside the
/// footnote definition sits on the later line and is collected first, at the
/// reference that cites it.
#[test]
fn the_undeclared_reference_reported_is_the_one_on_the_earliest_line() {
    let md = "# H\n\nA note[^n] cited early.\n\nA reference [](#early) here.\n\n[^n]: The note holds [](#late).\n";

    match md_to_typst(md, &[]) {
        Err(Error::Name {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 5, "the earlier of the two undeclared references");
            assert!(problem.contains("'early'"), "problem: {problem}");
        }
        other => panic!("expected a name error, got {other:?}"),
    }
}

/// Phase 1's second-caption refusal still fires when the first caption was
/// named.
///
/// `Figure::live` compares the recorded region against `written`, so a label
/// appended to the buffer without being carried into the record fails the
/// content check — and the second `: ` paragraph prints as prose where the
/// dialect names an error. Phase 2's cases for this refusal carry no name, so
/// an implementation that made that mistake passes the whole suite without it.
#[test]
fn the_second_caption_refusal_survives_a_named_first_caption() {
    let md = "# H\n\n![alt](dot.png)\n\n: A caption. {#fig:one}\n\n: A second one.\n";

    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
            assert_eq!(construct, "second caption for one figure");
            assert_eq!(line, 7);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// A caption line carrying a name and no words is still a caption with no text.
///
/// The group leaves the caption before the emptiness test runs, so this reaches
/// the same refusal a bare `: ` does. Emitting it would put a labelled, bare
/// "Figure 1:" on the page.
#[test]
fn a_caption_line_carrying_only_a_name_is_refused() {
    match md_to_typst("# H\n\n![alt](dot.png)\n\n: {#fig:one}\n", &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
            assert_eq!(construct, "caption with no text");
            assert_eq!(line, 5);
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// A `: ` line after a display equation is the ordinary paragraph it has always
/// been, and the equation in this fixture stays unnamed.
///
/// **Phase 4 held this rather than reversing it.** OQ-10 chose the closing `$$`
/// as the carrier for an equation's name, permanently rather than for one
/// phase, so the caption marker still attaches after a standalone image, a
/// table and a code block and nowhere else. An implementer who reached for the
/// caption line as the carrier after all fails here, and this shipped golden is
/// the only one where a display equation stands next to a `: ` marker.
#[test]
fn a_marker_line_after_a_display_equation_is_still_prose() {
    assert!(
        CROSS_REFERENCES_TYP.contains("\n: This paragraph follows a display equation,"),
        "the `: ` line after the equation stopped being ordinary prose"
    );
    assert!(
        !CROSS_REFERENCES_TYP.contains("<eq:"),
        "the fixture's equation took a label it never asked for"
    );

    // The equation itself is untouched, and so is the key that numbers it.
    assert!(
        CROSS_REFERENCES_TYP.contains(r#"equations: "plain""#),
        "the fixture stopped taking the shipped default"
    );

    // A name written on a `: ` line beneath an equation is prose, marker and
    // all: there is no figure above it to record, and though the equation's
    // record is live a paragraph away since Phase 12, `: A line. {#eq:one}` is
    // a run the group is not the whole of, so `equation_name` finds nothing.
    let typst = md_to_typst("# H\n\n$$\nx = 1\n$$\n\n: A line. {#eq:one}\n", &[]).unwrap();
    assert!(
        typst.contains("\n: A line. {\\#eq:one}"),
        "a name after an equation was read as a declaration: {typst}"
    );
}

// -- mpdf-005 Phase 4: equations join ---------------------------------------

/// The named equation and the reference that points at it, byte for byte.
///
/// A fixture of its own rather than an addition to `cross_references.md`, whose
/// golden is shipped work gate (4) asserts does not move.
#[test]
fn the_equation_names_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(EQUATION_NAMES_MD, &[]).unwrap(),
        EQUATION_NAMES_TYP
    );
}

#[test]
fn the_equation_names_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(EQUATION_NAMES_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The label sits immediately after the closing `$`, and the group is gone.
///
/// The reference is `#ref(<name>)`, Phase 3's spelling reused unchanged — no
/// `#link(` is written for it, and nothing of `{#eq:pythagoras}` reaches the
/// page.
#[test]
fn the_equation_names_golden_carries_each_form() {
    for (form, what) in [
        (
            "$ a ^(2 ) + b ^(2 ) = c ^(2 ) $ <eq:pythagoras>",
            "the label immediately after the closing `$`",
        ),
        ("#ref(<eq:pythagoras>)", "the reference to it"),
        (
            "in #ref(<eq:pythagoras>). The prose",
            "a reference ending a sentence, the shape ordinary prose puts one in",
        ),
        (
            "$ sum _(i = 1 )^(n ) i = frac(n \\(n + 1 \\),2 ) $\n",
            "the unnamed equation after it, taking no label",
        ),
    ] {
        assert!(
            EQUATION_NAMES_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    // The group leaves the page with the name, the way a caption's does.
    assert!(
        !EQUATION_NAMES_TYP.contains("{\\#"),
        "the name group reached the page"
    );
    assert!(
        !EQUATION_NAMES_TYP.contains("#link("),
        "a reference was written as a link"
    );
}

/// A reference to an equation in a document that did not number its equations
/// is refused in `core`, naming the line and naming the key.
///
/// **This is the case the phase exists for**, and it is the *default* path:
/// both looks answer the key with `set math.equation(numbering: … else
/// { none })` and `plain` is what an absent key means, so Typst fails the whole
/// compile with `cannot reference equation without numbering` — a message
/// carrying neither the author's line nor the key they would have to set.
#[test]
fn an_equation_reference_without_the_key_is_refused_naming_the_key() {
    let md =
        "---\nequations: plain\n---\n\n# H\n\n$$\nx = 1\n$$ {#eq:one}\n\nAs [](#eq:one) shows.\n";

    match md_to_typst(md, &[]) {
        Err(Error::Name {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 11, "the line the reference sits on");
            assert!(
                problem.contains("equations: numbered"),
                "the problem `{problem}` does not name the key"
            );
            assert!(
                problem.contains("'eq:one'"),
                "the problem `{problem}` does not name what the author typed"
            );
        }
        other => panic!("expected a name error, got {other:?}"),
    }
}

/// Naming an equation is not refused there — only pointing at one is.
///
/// The pair with the case above is what pins where the refusal sits. A labelled
/// unnumbered equation compiles perfectly well, and refusing the name would
/// break a document that names an equation before it points at one, which is a
/// direction the dialect's rule does not run in.
#[test]
fn a_name_on_an_equation_in_a_plain_document_is_not_refused() {
    assert_eq!(
        md_to_typst(PLAIN_EQUATION_NAMES_MD, &[]).unwrap(),
        PLAIN_EQUATION_NAMES_TYP
    );
    assert!(
        PLAIN_EQUATION_NAMES_TYP.contains("$ E = m c ^(2 ) $ <eq:energy>"),
        "the label is missing from a document that numbers nothing"
    );
    assert!(
        PLAIN_EQUATION_NAMES_TYP.contains(r#"equations: "plain""#),
        "the fixture stopped taking the shipped default"
    );
}

#[test]
fn the_plain_equation_names_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(PLAIN_EQUATION_NAMES_MD, &images_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// An unnamed display equation is byte-for-byte unchanged, in all three shipped
/// goldens that carry one.
///
/// The three are named individually because the count is what a sweep gets
/// wrong: `cross_references.typ` is the one Phase 3 shipped, and the only
/// shipped golden where a display equation stands next to a `: ` marker
/// paragraph — the exact interaction this phase touches. Byte-exactness itself
/// is each fixture's own equality test; this is the claim they add up to,
/// written where a reader can find it.
#[test]
fn no_shipped_display_equation_took_a_label() {
    for (golden, file) in [
        (DISPLAY_MATH_TYP, "display_math.typ"),
        (NUMBERED_EQUATIONS_TYP, "numbered_equations.typ"),
        (CROSS_REFERENCES_TYP, "cross_references.typ"),
    ] {
        assert!(
            golden.contains("$ "),
            "{file} stopped carrying a display equation"
        );
        assert!(golden.contains("$\n"), "{file} lost its block delimiters");
        assert!(!golden.contains(" <eq"), "{file} took an equation label");
    }
}

/// The group must be the whole of the run, on both sides, and an inline span
/// takes no name.
///
/// **The leading-text shape is the one that matters here.** `core`'s caption
/// finder takes the *last* group on a line, so an implementation that reused it
/// whole would refuse the trailing shape and label the leading one — passing
/// every other case in this phase and shipping a rule the dialect does not
/// have.
///
/// **The paragraph-ending inline row is Phase 12's.** Its group is the whole of
/// a run that does not open its paragraph, so a refusal scoped to the run
/// rather than the paragraph turns a line Phase 4 decided stays prose into an
/// error beside a visible formula. The soft-break row that stood in its place
/// until Phase 12 inverted — a name a line below now names — and lives in
/// `a_name_a_line_or_a_paragraph_below_still_names`.
#[test]
fn a_group_that_is_not_the_whole_run_names_nothing() {
    // Emitted rather than read off the golden: a needle over a golden constant
    // holds only what that file says, and the file is written by the same
    // implementation the case is meant to catch.
    for (md, form, what) in [
        (
            "# H\n\n$$\nw = 4\n$$ {#eq:trailing} and more\n",
            "$ w = 4 $ {\\#eq:trailing} and more",
            "text after the group",
        ),
        (
            "# H\n\n$$\ny = 5\n$$ see {#eq:leading}\n",
            "$ y = 5 $ see {\\#eq:leading}",
            "text before the group",
        ),
        (
            "# H\n\nAn inline $x + 1$ {#eq:inline} here.\n",
            "$x + 1$ {\\#eq:inline}",
            "an inline span, which Typst never numbers",
        ),
        (
            "# H\n\nAn inline $x + 1$ {#eq:inline}\n",
            "$x + 1$ {\\#eq:inline}",
            "an inline span whose group ends its paragraph, which a run-scoped refusal breaks",
        ),
    ] {
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(
            typst.contains(form),
            "{what} stopped reaching the page as prose: {typst}"
        );
    }

    // And the same three, standing in a document that compiles.
    for form in [
        "$ w = 4 $ {\\#eq:trailing} and more",
        "$ y = 5 $ see {\\#eq:leading}",
        "$x + 1$ {\\#eq:inline}",
    ] {
        assert!(
            PLAIN_EQUATION_NAMES_TYP.contains(form),
            "the golden file does not carry `{form}`"
        );
    }
}

/// A caption line holding a display span keeps its own name.
///
/// **The record's liveness test does not refuse this and a draft of the phase
/// claimed it did.** The caption's marker arm pushes its `bufs` frame before
/// anything later in that paragraph is written, so a span inside a caption
/// records at that deeper frame and would be spent at the same one, with
/// nothing in between to fail the content check. Both conditions hold, and
/// without the guard the label lands on the equation while the figure loses the
/// name it has carried since Phase 3 — a silent reassignment of a shipped
/// meaning, which nothing else here would catch.
#[test]
fn a_caption_holding_a_display_span_keeps_its_own_name() {
    // Emitted rather than read off the golden, for the reason above: this is
    // the case no other one covers, so it has to fail a wrong implementation on
    // its own bytes.
    let typst = md_to_typst(
        "# H\n\n![alt](dot.png)\n\n: See $$x = 1$$ {#fig:one}\n\nAt [](#fig:one).\n",
        &[],
    )
    .unwrap();
    assert!(
        typst.contains(
            "#figure(image(\"dot.png\", alt: \"alt\"), caption: [See $ x = 1 $]) <fig:one>"
        ),
        "the display span inside the caption took the caption's name: {typst}"
    );
    assert!(
        typst.contains("#ref(<fig:one>)"),
        "the reference stopped resolving against the figure: {typst}"
    );

    assert!(
        PLAIN_EQUATION_NAMES_TYP.contains(
            "caption: [The pipeline, whose middle step is $ y = m x + b $]) <fig:pipeline>"
        ),
        "the same shape in a document that compiles"
    );
}

/// Phase 3's name rules hold over the fourth construct, each naming the line.
///
/// The clauses are shared with a caption's name and the *finding* rule is not,
/// so these run through code Phase 3 shipped and this is a regression net
/// rather than new behaviour — except the last, which is new: an equation and a
/// figure share one namespace, because a document has one set of names.
#[test]
fn each_equation_name_refusal_names_the_authors_line() {
    let equation = "# H\n\n$$\nx = 1\n$$ ";

    for (md, line, needle, what) in [
        (
            format!("{equation}{{#eq one}}\n"),
            5,
            "letters, digits, '-', '_', ':' and '.'",
            "a character outside the set, the error listing the set",
        ),
        (
            format!("{equation}{{#:foo}}\n"),
            5,
            "begins with ':' or '.'",
            "a name Typst would not read as a name at all",
        ),
        (
            format!("{equation}{{#.foo}}\n"),
            5,
            "begins with ':' or '.'",
            "the same rule over a leading full stop",
        ),
        (
            format!("{equation}{{#fn-1}}\n"),
            5,
            "reserved for footnotes",
            "the namespace the emitter already owns",
        ),
        (
            format!("{equation}{{#}}\n"),
            5,
            "a name is empty",
            "a group with no name inside it",
        ),
        (
            format!("{equation}{{#one}}\n\n![alt](dot.png)\n\n: A caption. {{#one}}\n"),
            9,
            "declared twice",
            "one namespace over an equation and a figure, refused where the second stands",
        ),
    ] {
        match md_to_typst(&md, &[]) {
            Err(Error::Name {
                location:
                    Location {
                        file: None,
                        line: found,
                    },
                problem,
            }) => {
                assert_eq!(found, line, "for {what}");
                assert!(
                    problem.contains(needle),
                    "for {what}, the problem `{problem}` does not name `{needle}`"
                );
            }
            other => panic!("expected a name error for {what}, got {other:?}"),
        }
    }
}

/// Both after-the-walk refusals are one pass, and the earliest line wins.
///
/// A document may hold an undeclared reference on one line and an equation
/// reference in a `plain` document on another. Which is reported is a choice
/// rather than an accident, and the container is a `Vec` so that two runs over
/// one document agree.
#[test]
fn the_earliest_line_wins_across_both_reference_refusals() {
    let named = "---\nequations: plain\n---\n\n# H\n\n$$\nx = 1\n$$ {#eq:one}\n\n";

    // The equation reference stands first, so it is the error even though the
    // undeclared one below it is the older of the two classes.
    match md_to_typst(
        &format!("{named}A [](#eq:one) here.\n\nAnd [](#missing).\n"),
        &[],
    ) {
        Err(Error::Name {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 11);
            assert!(
                problem.contains("equations: numbered"),
                "problem: {problem}"
            );
        }
        other => panic!("expected a name error, got {other:?}"),
    }

    // Reversed, the undeclared one wins — so neither class is preferred.
    match md_to_typst(
        &format!("{named}And [](#missing).\n\nA [](#eq:one) here.\n"),
        &[],
    ) {
        Err(Error::Name {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 11);
            assert!(problem.contains("'missing'"), "problem: {problem}");
        }
        other => panic!("expected a name error, got {other:?}"),
    }
}

/// A name declared on an equation inside a footnote definition keeps its kind.
///
/// Those names are met only by the walk of the definitions and travel with the
/// body to the reference that sets it, the way its images and its math flag do.
/// An implementation that carried the name without carrying *what declared it*
/// passes every case above and then lets a `plain` document reach Typst with a
/// reference Typst fails the whole compile over.
#[test]
fn an_equation_named_inside_a_footnote_definition_still_needs_the_key() {
    let body = "# H\n\nA note[^n], and [](#eq:inside) reaches it.\n\n[^n]: The note holds a formula.\n\n    $$\n    x = 1\n    $$ {#eq:inside}\n";

    let typst = md_to_typst(&format!("---\nequations: numbered\n---\n\n{body}"), &[]).unwrap();
    assert!(
        typst.contains("$ x = 1 $ <eq:inside>]<fn-1>"),
        "the label did not travel with the definition's body: {typst}"
    );
    assert!(
        typst.contains("#ref(<eq:inside>)"),
        "the reference outside the definition did not resolve: {typst}"
    );

    match md_to_typst(&format!("---\nequations: plain\n---\n\n{body}"), &[]) {
        Err(Error::Name {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 7, "the line the reference sits on");
            assert!(
                problem.contains("equations: numbered"),
                "problem: {problem}"
            );
        }
        other => panic!("expected a name error, got {other:?}"),
    }
}

// -- mpdf-005 Phase 5: a figure may have more than one member ---------------

/// A group of two images under one caption and one name, byte for byte.
///
/// A fixture of its own rather than an addition to `captions.md` or
/// `cross_references.md`, whose goldens are shipped work gate (8) asserts do
/// not move.
#[test]
fn the_groups_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(GROUPS_MD, &[]).unwrap(), GROUPS_TYP);
}

#[test]
fn the_groups_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(GROUPS_MD, &images_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// One `#figure` over a `grid`, and **no `kind` argument anywhere**.
///
/// The absence is asserted rather than assumed: Typst infers a figure's kind
/// from the `grid` it is handed, so a group of images is a Figure and a group
/// of tables is a Table with nothing configured. Writing a `kind` is the cheap
/// implementation that passes every other case here and puts the emitter back
/// in the business of naming a kind — which is the seam this spec rests on.
#[test]
fn the_groups_golden_carries_each_form() {
    for (form, what) in [
        (
            "#figure(grid(columns: 2, image(\"dot.png\", alt: \"The first of a pair\"), \
             image(\"dot.png\", alt: \"The second of a pair\")), \
             caption: [Two images under one caption, side by side.]) <fig:pair>",
            "a group of two images as one figure, named",
        ),
        ("#ref(<fig:pair>)", "a reference to the group"),
        ("#figure(grid(columns: 2, table(", "a group of two tables"),
        (
            "#figure(grid(columns: 2, raw(block: true, lang: \"rust\", \"fn first() {}\"), \
             raw(block: true, lang: \"rust\", \"fn second() {}\")), \
             caption: [Two listings under one caption.])",
            "a group of two listings",
        ),
    ] {
        assert!(
            GROUPS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }

    assert!(
        !GROUPS_TYP.contains("kind:"),
        "the emitter named a kind, which Typst infers from the grid"
    );
    // Four groups and no more: every other `#figure(` in the file is a single
    // captioned construct, which is Phase 1's and Phase 2's shape unchanged.
    assert_eq!(
        GROUPS_TYP.matches("#figure(grid(").count(),
        4,
        "the golden file does not carry exactly the four groups the fixture writes"
    );
}

/// A `: ` line with a member after it, inside a group, is refused.
///
/// The group's own caption is the last block before the closer, so this is a
/// marker that is *not* last — which is what distinguishes it from the caption
/// the fixture writes. **It costs nothing now and keeps OQ-12 open**: a `: `
/// line after a member is the exact spelling a subcaption will want, so a phase
/// that let it reach the page as prose would ship a meaning it would later have
/// to take back.
#[test]
fn a_group_caption_with_a_member_after_it_is_refused() {
    let md = "# H\n\n:::\n\n![a](dot.png)\n\n: A caption.\n\n![b](dot.png)\n\n:::\n";

    match md_to_typst(md, &[]) {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location { file: None, line },
        }) => {
            assert_eq!(construct, "figure group caption with a member after it");
            assert_eq!(line, 7, "the line the `: ` sits on");
        }
        other => panic!("expected an UnsupportedConstruct error, got {other:?}"),
    }
}

/// The other seven refusals, each naming the author's line.
///
/// Two carry the case. **The empty group is the one that otherwise reaches
/// Typst**: it satisfies every other rule here and emits `grid(columns: 0)`,
/// which fails the compile with `number must be positive`, naming no line and
/// no construct the author would recognise. And **a group opened in one list
/// item and closed in the next is the one a depth-only check accepts** — both
/// delimiters land at depth 2, in different frames, so a closer that compared
/// depths would truncate a frame its group never opened.
#[test]
fn each_group_refusal_names_its_construct_and_its_line() {
    for (md, construct, line, what) in [
        (
            "# H\n\n:::\n\n![a](dot.png)\n\n:::\n",
            "figure group with no caption",
            3,
            "a group with no caption line",
        ),
        (
            "# H\n\n:::\n\n: A caption.\n\n:::\n",
            "figure group with no member",
            3,
            "a group with no member, which would emit `grid(columns: 0)`",
        ),
        (
            "# H\n\n:::\n\n![a](dot.png)\n\n: One.\n\n: Two.\n\n:::\n",
            "second caption for one figure group",
            9,
            "a second caption line inside a group",
        ),
        (
            "# H\n\n:::\n\n![a](dot.png)\n\n::: table\n\n:::\n",
            "figure group inside a figure group",
            7,
            "a `:::` inside a group",
        ),
        (
            "# H\n\n:::\n\n![a](dot.png)\n\n: A caption.\n",
            "figure group the document never closes",
            3,
            "a group the document body never closes",
        ),
        (
            "# H\n\nA note.[^1]\n\n[^1]: The note.\n\n    :::\n\n    ![a](dot.png)\n",
            "figure group the document never closes",
            7,
            "a group a footnote definition never closes, which the other walk ends",
        ),
        (
            "# H\n\n- :::\n\n  ![a](dot.png)\n\n:::\n",
            "figure group the document never closes",
            3,
            "a group opened in a list item and left to close outside it",
        ),
        (
            "# H\n\n- :::\n\n  ![a](dot.png)\n\n- :::\n",
            "figure group the document never closes",
            3,
            "a group opened in one list item and closed in the next",
        ),
        (
            "# H\n\n::::\n\n![a](dot.png)\n\n:::\n",
            "figure group delimiter that is neither an opener nor a closer",
            3,
            "a mistyped delimiter",
        ),
        (
            "# H\n\n::: two words\n\n![a](dot.png)\n\n:::\n",
            "figure group delimiter that is neither an opener nor a closer",
            3,
            "an opener carrying more than one word",
        ),
        (
            "# H\n\n:::\n![a](dot.png)\n:::\n",
            "figure group delimiter that is neither an opener nor a closer",
            3,
            "the tight div, whose one paragraph begins `:::` and is no valid opener",
        ),
        (
            "# H\n\n:::\n\n![a](dot.png)\n\nSome prose.\n\n![b](dot.png)\n\n: A caption.\n\n:::\n",
            "block inside a figure group that is not an image, a table or a code block",
            7,
            "a paragraph of prose between two members",
        ),
        (
            "# H\n\n:::\n\n![a](dot.png)\n\n## Nope\n\n: A caption.\n\n:::\n",
            "block inside a figure group that is not an image, a table or a code block",
            7,
            "a heading inside a group",
        ),
    ] {
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

/// The reservation reaches the first text of a paragraph and nothing else.
///
/// **Emitted rather than read off the golden**, per Phase 4's note: a needle
/// over a golden constant holds only what that file says, and the file is
/// written by the same implementation the case is meant to catch.
///
/// The code-block case is the load-bearing one — it is where a document that
/// documents this syntax puts a `:::`, this repository's own README included —
/// and both block kinds are asserted, since one arm serves the fenced block and
/// the indented one.
#[test]
fn the_reservation_reaches_the_first_text_of_a_paragraph_and_nothing_else() {
    for (md, form, what) in [
        (
            "# H\n\nA line reading ::: opens a group.\n",
            "A line reading ::: opens a group.",
            "a `:::` inside a sentence",
        ),
        (
            "# H\n\nSome prose.\n::: not a group\n",
            "Some prose.\n::: not a group",
            "a `:::` later in a paragraph whose first text is something else",
        ),
        (
            "# H\n\n```markdown\n:::\n\n![a](dot.png)\n\n: A caption.\n\n:::\n```\n",
            "#raw(block: true, lang: \"markdown\", \":::\\n\\n![a](dot.png)\\n\\n: A caption.\\n\\n:::\")",
            "a `:::` inside a fenced code block",
        ),
        (
            "# H\n\n    :::\n    ![a](dot.png)\n    :::\n",
            "#raw(block: true, \":::\\n![a](dot.png)\\n:::\")",
            "a `:::` inside an indented code block",
        ),
    ] {
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(
            typst.contains(form),
            "{what} stopped reaching the page unchanged: {typst}"
        );
    }

    // And the same four, standing in a document that compiles.
    for form in [
        "A\nline reading ::: inside a sentence is prose",
        "::: not a group.",
        "#raw(block: true, lang: \"markdown\", \":::\\n\\n",
        "#raw(block: true, \":::\\n![tight](dot.png)\\n:::\")",
    ] {
        assert!(
            GROUPS_TYP.contains(form),
            "the golden file does not carry `{form}`"
        );
    }
}

/// Both bundled looks separate one figure's members.
///
/// A test of its own rather than an extension of
/// `every_bundled_template_styles_a_caption`, whose name would stop describing
/// it — the same argument Phase 1 made when it refused to hang a caption
/// assertion off the call-contract test.
///
/// **The needle is `set grid(gutter:` and not `show figure`**, because both
/// looks already carry `show figure: set block(…)`, so a test keyed to the
/// looser phrase would pass before the rule existed. The gutter *value* is
/// deliberately not a needle: Typst's default is zero, so two members would
/// touch, and how far apart they sit is each look's own call.
#[test]
fn every_bundled_template_separates_a_figures_members() {
    for (file, source) in BUNDLED_TEMPLATES {
        assert!(
            source.contains("set grid(gutter:"),
            "{file} does not separate a figure's members"
        );
    }
}

// -- mpdf-005 Phase 6: a listing sits where its code sits ---------------------

/// Both bundled looks decide where a captioned listing sits.
///
/// A test of its own rather than an extension of
/// `every_bundled_template_styles_a_caption` or of
/// `every_bundled_template_separates_a_figures_members` — each is named for
/// what it asserts, which is the argument Phase 1 gate (6) and Phase 5 gate (8)
/// both made.
///
/// **The needle is the whole rule, `figure.where(kind: raw): set align(left)`,
/// and the selector alone will not do** — which `mpdf-005` Phase 9 measured and
/// repaired here. This test was written with the selector as its needle, on the
/// claim that it was "the first `.where(kind: …)` rule either look carries".
/// **Phase 7 falsified both halves**: its per-section counter reset writes
/// `counter(figure.where(kind: raw)).update(0)` into each look, so the string
/// occurs twice per file and this assertion passed with the alignment rule
/// deleted outright. An assertion that cannot fail is not one.
///
/// The alignment's *direction* is still deliberately outside the needle, on the
/// same ground the caption's position and the gutter's value are: what a needle
/// can hold is that each look answers the question, and where the block landed
/// is what the by-eye read on one PDF per look is for. What the repair adds is
/// that the answer is a `show` rule at all.
#[test]
fn every_bundled_template_places_a_listing() {
    for (file, source) in BUNDLED_TEMPLATES {
        assert!(
            source.contains("figure.where(kind: raw): set align(left)"),
            "{file} does not decide where a listing sits"
        );
    }
}

// -- mpdf-007 Phase 1: a cited source reaches the reference list --------------

#[test]
fn citations_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(CITATIONS_MD, &[]).unwrap(), CITATIONS_TYP);
}

#[test]
fn citations_press_release_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(CITATIONS_PRESS_RELEASE_MD, &[]).unwrap(),
        CITATIONS_PRESS_RELEASE_TYP
    );
}

/// Both fixtures compile, and the compile is the assertion.
///
/// **It is stronger than "it runs".** `#cite` compiles only when its key
/// resolves against a bibliography the document actually contains, so a green
/// compile proves the mark reached the body *and* the list reached the end —
/// which no golden file can say, since a golden pins emitter output alone.
///
/// The key carries a `:` and a `/`, which is the whole reason the fixture is
/// keyed the way it is: `#cite(<DBLP:books/lib/Knuth86a>)` fails to *parse* —
/// "unclosed label; unexpected slash", naming neither line nor construct —
/// where `#cite(label("…"))` carries any key a bibliography file holds.
#[test]
fn each_citation_fixture_compiles_to_a_pdf() {
    for (md, what) in [
        (CITATIONS_MD, "the article look"),
        (CITATIONS_PRESS_RELEASE_MD, "the press-release look"),
    ] {
        let pdf = md_to_pdf(md, &citations_assets()).unwrap();
        assert!(pdf.starts_with(b"%PDF"), "{what} is not a PDF");
        assert!(pdf.len() > 1000, "{what} is suspiciously small");
    }
}

/// A bibliography withdraws no heading anchor, under either look.
///
/// **This is the assertion whose absence would have let the phase silently
/// empty every anchor list in the project.** Typst's default `title: auto`
/// realises a real `HeadingElem`, so a document with one markdown heading would
/// compile two, and `anchors_from` returns an *empty* vector on a count
/// mismatch — taking `mpdf-003`'s scroll sync with it, with no error anywhere.
///
/// It also pins what no needle can: that neither look's `show bibliography:`
/// rule draws its label as a heading either. Both fixtures carry exactly one
/// markdown heading, so both must report exactly one anchor.
#[test]
fn a_bibliography_withdraws_no_heading_anchor() {
    for (md, what) in [
        (CITATIONS_MD, "the article look"),
        (CITATIONS_PRESS_RELEASE_MD, "the press-release look"),
    ] {
        let rendered = md_to_pdf_with_anchors(md, &citations_assets()).unwrap();
        assert_eq!(
            rendered.anchors.len(),
            1,
            "{what} did not report exactly one anchor: {:?}",
            rendered.anchors
        );
    }
}

/// Both bundled looks label the reference list.
///
/// A test of its own rather than an extension of any earlier look-contract
/// test, each of which is named for what it asserts. **The needle is
/// `show bibliography`**: the emitter writes `title: none`, so without a rule
/// here the list runs straight on from the last paragraph with no label at all.
///
/// The label's *words* and its size are deliberately not needles — the word is
/// the look's, as the caption separator is — and that it is not a `heading` is
/// pinned by `a_bibliography_withdraws_no_heading_anchor` instead, which is a
/// stronger check than any needle could be.
#[test]
fn every_bundled_template_labels_a_reference_list() {
    for (file, source) in BUNDLED_TEMPLATES {
        assert!(
            source.contains("show bibliography"),
            "{file} does not label a reference list"
        );
    }
}

/// The callback claims a citation and nothing else.
///
/// **Emitted rather than read off the golden**, per the argument Phase 4 of
/// `mpdf-005` made: a needle over a golden constant holds only what that file
/// says, and the file is written by the same implementation the case is meant
/// to catch. The one exception is the last line, which is the point: an
/// unscoped broken-link callback turns `hostile.md`'s bracket pair into a link
/// and moves a shipped golden, and that is the assertion that can fail for the
/// right reason.
///
/// Pandoc's prefix form is on this list because this dialect has no prefix
/// form. `[see @k]` prints for the same reason any unclaimed markdown prints,
/// which is a stated boundary rather than a discovered one. `[+ @k]` is on it
/// because a sigil counts only glued to its at-sign: a callback firing on a
/// bare `+` would claim `[+ 3]`.
#[test]
fn the_callback_claims_a_citation_and_nothing_else() {
    for (md, form, what) in [
        (
            "# H\n\nan [ open bracket, a ] close bracket\n",
            r"an \[ open bracket, a \] close bracket",
            "a bracket pair that is not a reference",
        ),
        (
            "# H\n\na prefix form [see @k] here\n",
            r"\[see \@k\]",
            "Pandoc's prefix form, which this dialect has not",
        ),
        (
            "# H\n\na bracketed email [a@b.com] here\n",
            r"\[a\@b.com\]",
            "a bracketed email address",
        ),
        (
            "# H\n\na loose sigil [+ @k] here\n",
            r"\[\+ \@k\]",
            "a plus not glued to its at-sign, which is text and not the prose form",
        ),
        (
            "# H\n\nan a[0] index\n",
            r"a\[0\]",
            "an index that looks like a shortcut reference",
        ),
        (
            "# H\n\nan email a@b.com and a bare @thing\n",
            r"a\@b.com and a bare \@thing",
            "an unbracketed at-sign, which is why the bare form is not adopted",
        ),
        (
            "# H\n\na [labelled][d] reference\n\n[d]: https://typst.app\n",
            r#"#link("https://typst.app")[labelled]"#,
            "a reference that resolves, which the callback never sees",
        ),
    ] {
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(
            typst.contains(form),
            "{what} stopped reaching the page unchanged: {typst}"
        );
    }

    // The shipped golden that an unscoped callback would move.
    assert!(
        HOSTILE_TYP.contains(r"an \[ open bracket, a \] close bracket"),
        "the hostile golden no longer carries its bracket pair"
    );
    assert_eq!(md_to_typst(HOSTILE_MD, &[]).unwrap(), HOSTILE_TYP);
}

/// Each citation the dialect refuses names the author's own line.
///
/// Three payloads are refused, and each is named where it stands rather than
/// guessed at or silently dropped: a form over a group, which `cite_key`
/// refuses last so that the locator and the piece that is not a key are named
/// first; a locator, which the dialect does not read; and a piece between
/// semicolons that is not a key. `[@a; @b]` and `[-@k]` were on this list from
/// `mpdf-007` Phase 1 to Phase 5 as reservations, and both land now — the
/// group as Typst's own merge and the suppressed author as the year form — so
/// their rows left rather than moved.
///
/// The rows that are not about the payload are the ones a document that names
/// no bibliography earns. A `[@key]` in such a document is refused rather than
/// printed: mapping only where the frontmatter key is present would leave it
/// on the page as `\[\@smith2020\]`, visible and meaningless, which is the
/// silent flattening the dialect refuses for every other construct. The
/// `[+@k]` row is here rather than in a table of its own so this file's one
/// destructuring site for a citation error stays one, and the forty-eight that
/// `messages_test.rs` records over this file holds.
#[test]
fn each_refused_citation_names_the_authors_line() {
    for (md, line, needle, what) in [
        (
            "# H\n\nA cite [@smith2020] here.\n".to_string(),
            3,
            "names no bibliography",
            "a citation in a document that declares none",
        ),
        (
            "# H\n\nA prose group [+@a; @b] here.\n".to_string(),
            3,
            "puts a form on several sources",
            "the prose form over a group",
        ),
        (
            "# H\n\nA year group [-@a; @b] here.\n".to_string(),
            3,
            "puts a form on several sources",
            "the year form over a group",
        ),
        (
            "# H\n\nA locator [@k, p. 33] here.\n".to_string(),
            3,
            "carries a locator",
            "Pandoc's locator form",
        ),
        (
            "# H\n\nA prose locator [+@k, p. 33] here.\n".to_string(),
            3,
            "carries a locator",
            "a locator under a form, which is the locator first",
        ),
        (
            "# H\n\nA piece [@a; b] here.\n".to_string(),
            3,
            "is not a key",
            "a group whose second piece is not a key",
        ),
        (
            "# H\n\nA prose [+@k] here.\n".to_string(),
            3,
            "names no bibliography",
            "a prose citation in a document that declares none",
        ),
        (
            "# H\n\nA note[^1].\n\n[^1]: See [@k].\n".to_string(),
            5,
            "names no bibliography",
            "a citation only a footnote definition carries",
        ),
    ] {
        match md_to_typst(&md, &[]) {
            Err(Error::Citation {
                location:
                    Location {
                        file: None,
                        line: found,
                    },
                problem,
            }) => {
                assert_eq!(found, line, "for {what}");
                assert!(
                    problem.contains(needle),
                    "for {what}, the problem `{problem}` does not name `{needle}`"
                );
            }
            other => panic!("expected a citation error for {what}, got {other:?}"),
        }
    }
}

/// A citation inside a footnote definition cites, rather than printing.
///
/// The definition's own walk never parses the frontmatter — the metadata block
/// sits outside every definition — so the missing-bibliography test cannot run
/// where the citation is written. The citation travels out on the same `Body`
/// that already carries a definition's images, its math flag and its names, and
/// this is the case that proves it: the document names a bibliography, and the
/// citation inside the note must not be refused for the lack of one.
#[test]
fn a_citation_inside_a_footnote_definition_cites() {
    let typst = md_to_typst(CITATIONS_MD, &[]).unwrap();
    assert!(
        typst.contains(r#"#footnote[The definition cites #cite(label("DBLP:books/lib/Knuth86a"))"#),
        "the citation inside the note did not reach the reference site: {typst}"
    );
}

/// The author-date fixture matches its golden, and the golden pins the three
/// forms and the merge.
///
/// **The wrong build this discriminates against** passes the form as `#cite`'s
/// `style` argument: it compiles, renders the wrong mark, and fails the `form:`
/// needles below. The group needle carries no byte between its two calls,
/// because a space would be a byte the author did not write. The needles are
/// asserted over the golden beside the whole-file comparison, so a re-bless
/// that lost one of them fails here by name rather than by diff.
#[test]
fn author_date_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(AUTHOR_DATE_MD, &[]).unwrap(), AUTHOR_DATE_TYP);
    for needle in [
        "headings: \"plain\", citations: \"author-date\")\n",
        r#"#cite(label("two"), form: "prose")"#,
        r#"#cite(label("one"), form: "year")"#,
        r#"#cite(label("three"))#cite(label("one"))"#,
        "\n#bibliography(\"author_date.yml\", title: none)\n",
    ] {
        assert!(
            AUTHOR_DATE_TYP.contains(needle),
            "the golden does not carry `{needle}`"
        );
    }
    assert!(
        !AUTHOR_DATE_TYP.contains("style:"),
        "a form reached the call as a style"
    );
}

/// The author-date fixture compiles, and the page was read by hand.
///
/// The suite reads no PDF text, on the precedent `rules/pipeline.md` records
/// for `figures` and `headings`, so what the marks render as is recorded here
/// from `pdftotext` over `md_to_pdf`'s output on 2026-09-01, under
/// `harvard-cite-them-right`:
///
/// - `[@one]` is *(Postigo, 2026)*, and the collapsed `[@two][]` is
///   *(Claude and Knuth, 2025)*;
/// - `[+@two]` reads in the sentence as *Claude and Knuth (2025)*;
/// - `[-@one]` is the year alone, *2026*;
/// - `[@three; @one]` is one parenthesis with a semicolon,
///   *(Lovelace, Turing and Hopper, 2024; Postigo, 2026)*;
/// - `[+@four]` shortens from four, *Hamilton et al. (2023)*;
/// - and the list runs Claude, Hamilton, Lovelace, Postigo — alphabetical,
///   where `ieee` lists in the order the marks were cited.
#[test]
fn the_author_date_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(AUTHOR_DATE_MD, &author_date_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// Each form is emitted, not read off the golden — and under no `citations`
/// key at all, which is what says the form is the citation's and the style is
/// the document's. Each document names `bibliography: refs.yml`, because
/// `check_citations` refuses a cited key without one whatever the scheme; the
/// keys need not be in the file, since emission reads no asset.
///
/// The three spellings of a group emit the same two calls: `[@a ; @b]`
/// reaches the callback with its spaces, measured, and each piece is trimmed.
/// The collapsed form still emits the call it always did, which is the byte
/// Phase 1 promised for `[@k][]`.
#[test]
fn each_citation_form_is_emitted_under_the_default_scheme() {
    for (body, form, what) in [
        (
            "A prose [+@k] form.",
            r#"#cite(label("k"), form: "prose")"#,
            "the prose form",
        ),
        (
            "A year [-@k] form.",
            r#"#cite(label("k"), form: "year")"#,
            "the year form",
        ),
        (
            "A group [@a; @b] here.",
            r#"#cite(label("a"))#cite(label("b"))"#,
            "a group",
        ),
        (
            "A group [@a;@b] here.",
            r#"#cite(label("a"))#cite(label("b"))"#,
            "a group with no space",
        ),
        (
            "A group [@a ; @b] here.",
            r#"#cite(label("a"))#cite(label("b"))"#,
            "a group with a space either side of the semicolon",
        ),
        (
            "A collapsed [@k][] form.",
            r#"#cite(label("k"))"#,
            "the collapsed form, unchanged",
        ),
    ] {
        let md = format!("---\nbibliography: refs.yml\n---\n\n# H\n\n{body}\n");
        let typst = md_to_typst(&md, &[]).unwrap();
        assert!(typst.contains(form), "{what} did not emit `{form}`: {typst}");
        assert!(
            typst.contains("citations: \"numeric\""),
            "{what} moved the default scheme: {typst}"
        );
    }
}

/// A bibliography the caller did not supply is refused by name.
///
/// `collect` exists so a missing file is named with the author's own path
/// before Typst is asked — without it the compile says "file not found
/// (searched at refs.yml)" against a span in a `main.typ` the user has never
/// seen. The line is the frontmatter line, which is the only place a
/// bibliography's position is ever known.
#[test]
fn a_bibliography_the_caller_did_not_supply_names_the_path_and_the_line() {
    match md_to_pdf(CITATIONS_MD, &[]) {
        Err(Error::MissingBibliography {
            path,
            location: Location { file: None, line },
        }) => {
            assert_eq!(path, "refs.yml");
            assert_eq!(line, 3);
        }
        other => panic!("expected a missing bibliography, got {other:?}"),
    }
}

/// The bibliography path rule refuses a whole document, not only a block.
///
/// One rule with two renderings: the image arm says what the image is, the key
/// says what the key takes. `core/src/frontmatter.rs` pins each shape's
/// sentence; this pins that a document carrying one never reaches the compiler,
/// where a backslash path would otherwise become `Error::Internal` — whose own
/// contract says a broken build rather than bad input.
#[test]
fn a_bibliography_path_outside_the_shape_rule_refuses_the_document() {
    for value in [
        "https://example.com/refs.yml",
        "/etc/refs.yml",
        "../refs.yml",
        r"refs\bib.yml",
    ] {
        let md = format!("---\nbibliography: {value}\n---\n\n# H\n");
        match md_to_typst(&md, &[]) {
            Err(Error::Frontmatter {
                location: Location { file: None, line },
                problem,
            }) => {
                assert_eq!(line, 2, "wrong line for {value}");
                assert!(
                    problem.contains("beside the document"),
                    "problem was: {problem}"
                );
            }
            other => panic!("expected a Frontmatter error for {value}, got {other:?}"),
        }
    }
}

// -- mpdf-007 Phase 2: a key the bibliography does not hold -------------------

/// A key the bibliography does not hold names the key and the citation's line.
///
/// Typst raises this itself, and its whole sentence is ``citation key `k` is
/// not present in the bibliography`` — the key, and no line at all, because
/// `core/src/lib.rs:join` keeps the message and drops the span, and there is no
/// map from a `main.typ` span back to the markdown anyway. This is the refusal
/// the rejection rule asks for: the author's own key, on the author's own line.
///
/// The check runs beside `collect` rather than in the walk, so this is
/// `md_to_pdf` and not `md_to_typst` — the bibliography's bytes are the only
/// thing that can answer the question, and emission reads no bytes.
#[test]
fn an_absent_key_names_the_key_and_the_citations_line() {
    let md = "---\ntitle: T\nbibliography: refs.yml\n---\n\n# H\n\nA cite [@nosuchkey] here.\n";
    match md_to_pdf(md, &citations_assets()) {
        Err(Error::Citation {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 8);
            assert!(problem.contains("nosuchkey"), "problem was: {problem}");
            assert!(
                problem.contains("does not hold it"),
                "problem was: {problem}"
            );
        }
        other => panic!("expected a citation error, got {other:?}"),
    }
}

/// A name the document and the bibliography both hold is refused where it is
/// pointed at.
///
/// **The reference is the trigger, and the fixture would assert nothing without
/// it.** Measured over the whole matrix: a figure named `{#knuth1986}` in a
/// document whose bibliography holds `knuth1986` compiles clean, and so does the
/// same document citing `[@knuth1986]`; Typst raises ``label `<knuth1986>`
/// occurs both in the document and a bibliography`` only where a `[](#knuth1986)`
/// points at the shared label, and then whether or not the key is cited. The
/// second half of this test is the clean case, which is what makes the first
/// half about the reference rather than about the name.
///
/// A table carries the name rather than an image, so the case needs no image
/// asset beside the bibliography.
#[test]
fn a_name_the_document_and_the_bibliography_both_hold_is_refused_at_the_reference() {
    let figure = "---\ntitle: T\nbibliography: refs.bib\n---\n\n# H\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\n: A table. {#knuth1986}\n";

    match md_to_pdf(
        &format!("{figure}\nAs [](#knuth1986) shows.\n"),
        &bib_assets(),
    ) {
        Err(Error::Citation {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 14);
            assert!(problem.contains("knuth1986"), "problem was: {problem}");
            assert!(
                problem.contains("cannot mean both"),
                "problem was: {problem}"
            );
        }
        other => panic!("expected a citation error for the collision, got {other:?}"),
    }

    // The same document with no reference, citing the shared key: clean.
    let pdf = md_to_pdf(
        &format!("{figure}\nA cite [@knuth1986] and no reference.\n"),
        &bib_assets(),
    )
    .unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the clean case is not a PDF");
}

/// Where two keys are absent, the error is the one on the earlier line.
///
/// **The construction is the footnote splice, and the obvious document has no
/// teeth.** `core/src/emit.rs` extends `names.cited` from a definition's body at
/// the *reference*, carrying the line the citation had inside the definition —
/// so with the reference on line 8, a body citation on line 10 and the
/// definition's on line 12, the vector is `[12, 10]` where the lines are
/// `[10, 12]`, and `.first()` and `min_by_key` disagree. Two plain body
/// citations come out in document order and would pass either way.
#[test]
fn where_two_keys_are_absent_the_earlier_line_is_the_error() {
    let md = "---\ntitle: T\nbibliography: refs.yml\n---\n\n# H\n\nA note[^n] here.\n\nA cite [@alpha] here.\n\n[^n]: The definition cites [@omega].\n";
    match md_to_pdf(md, &citations_assets()) {
        Err(Error::Citation {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 10, "the error was: {problem}");
            assert!(problem.contains("alpha"), "problem was: {problem}");
        }
        other => panic!("expected a citation error, got {other:?}"),
    }
}

/// A bibliography whose own parse fails is refused by name, with the
/// frontmatter's line.
///
/// The file's only line is the one that named it, which is the line
/// `Error::MissingBibliography` already reports. Nothing checks an extension
/// against its content on either side, so a `.yml` holding something that is not
/// Hayagriva reaches the reader and is refused there rather than by a panic or a
/// Typst diagnostic.
#[test]
fn a_bibliography_that_does_not_parse_names_itself_and_the_frontmatter_line() {
    let md = "---\ntitle: T\nbibliography: refs.yml\n---\n\n# H\n\nA cite [@k] here.\n";
    let bad = asset("refs.yml", b"this is not a bibliography at all\n");
    match md_to_pdf(md, &[bad]) {
        Err(Error::Citation {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 3);
            assert!(problem.contains("refs.yml"), "problem was: {problem}");
            assert!(
                problem.contains("does not parse as Hayagriva"),
                "problem was: {problem}"
            );
        }
        other => panic!("expected a citation error, got {other:?}"),
    }
}

/// Both formats resolve a key, and a third extension is refused by name.
///
/// The extension is what dispatches, so both accepted spellings are exercised
/// rather than one — and the third case is the failure that is reachable from
/// ordinary markdown today and names no line: `core/src/frontmatter.rs` checks
/// the path's *shape* and nothing about its extension, so `bibliography:
/// refs.txt` over a perfectly good Hayagriva file reaches Typst's "unknown
/// bibliography format (must be .yaml/.yml or .bib)" against a span in a
/// `main.typ` the user has never seen.
#[test]
fn both_bibliography_formats_resolve_a_key_and_a_third_is_refused() {
    for (path, key, assets, what) in [
        (
            "refs.yml",
            "DBLP:books/lib/Knuth86a",
            citations_assets(),
            "Hayagriva",
        ),
        ("refs.bib", "knuth1986", bib_assets(), "BibLaTeX"),
    ] {
        let md =
            format!("---\ntitle: T\nbibliography: {path}\n---\n\n# H\n\nA cite [@{key}] here.\n");
        let pdf = md_to_pdf(&md, &assets).unwrap();
        assert!(pdf.starts_with(b"%PDF"), "{what} did not produce a PDF");
    }

    // Good Hayagriva bytes under an extension neither reader is dispatched on.
    let md = "---\ntitle: T\nbibliography: refs.txt\n---\n\n# H\n\nA cite [@DBLP:books/lib/Knuth86a] here.\n";
    match md_to_pdf(md, &[asset("refs.txt", REFS_YML)]) {
        Err(Error::Citation {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 3);
            assert!(problem.contains("refs.txt"), "problem was: {problem}");
            assert!(
                problem.contains("no format this dialect reads"),
                "problem was: {problem}"
            );
        }
        other => panic!("expected a citation error for the extension, got {other:?}"),
    }
}

/// This phase emits no markup at all, so no golden may move for any reason.
///
/// The cheapest possible check that the walk was not disturbed, and stronger
/// than Phase 1's: that one allowed the two new goldens, where this one allows
/// nothing. Every pair the file holds is asserted individually above; this
/// re-asserts the two the citation channel owns, which are the ones a change to
/// `Emitted` or to `emit`'s tail could move.
#[test]
fn phase_two_moves_no_golden() {
    assert_eq!(md_to_typst(CITATIONS_MD, &[]).unwrap(), CITATIONS_TYP);
    assert_eq!(
        md_to_typst(CITATIONS_PRESS_RELEASE_MD, &[]).unwrap(),
        CITATIONS_PRESS_RELEASE_TYP
    );
    assert_eq!(md_to_typst(HOSTILE_MD, &[]).unwrap(), HOSTILE_TYP);
    assert_eq!(
        md_to_typst(CROSS_REFERENCES_MD, &[]).unwrap(),
        CROSS_REFERENCES_TYP
    );
}

// -- mpdf-005 Phase 7: a figure number may carry its section ------------------

#[test]
fn the_sectioned_figures_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(SECTIONED_FIGURES_MD, &[]).unwrap(),
        SECTIONED_FIGURES_TYP
    );
}

/// The fixture compiles, and under both looks.
///
/// The compile is a real assertion here rather than a smoke test: the section
/// prefix is built by a closure that reads `counter(heading)`, and a numbering
/// function that reached for a counter Typst could not give it in context would
/// fail here rather than print a wrong number.
#[test]
fn the_sectioned_figures_fixture_compiles_to_a_pdf() {
    for (md, what) in [
        (SECTIONED_FIGURES_MD.to_string(), "the article look"),
        (
            SECTIONED_FIGURES_MD.replace(
                "figures: sectioned",
                "figures: sectioned\ntemplate: press-release",
            ),
            "the press-release look",
        ),
    ] {
        let pdf = md_to_pdf(&md, &[asset("dot.png", DOT_PNG)])
            .unwrap_or_else(|e| panic!("{what} did not compile: {e}"));
        assert!(pdf.starts_with(b"%PDF"), "{what} did not produce a PDF");
    }
}

/// The key crosses to the look, and the reference the page's number is read
/// through survives the phase.
///
/// The *number* is deliberately not here and cannot be: the emitter writes no
/// numbering at all, so `1.1` exists only in the compiled PDF. What a golden can
/// hold is that the author's ask reached the call and that the reference is
/// still a `#ref`, which is what gate (2) reads the page for.
#[test]
fn the_sectioned_figures_golden_carries_each_form() {
    for (form, what) in [
        ("figures: \"sectioned\"", "the key the author set"),
        ("#ref(<tab:one>)", "the reference that reads the number"),
    ] {
        assert!(
            SECTIONED_FIGURES_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// Sectioned numbering withdraws no heading anchor.
///
/// **This is the assertion whose absence would let the phase silently empty
/// every anchor list in the project**, on the precedent
/// `a_bibliography_withdraws_no_heading_anchor` set: this phase installs a
/// `show heading.where(level: 1):` rule, and whether a `show` rule changes what
/// `introspector.query(HeadingElem)` returns is invisible until a pane stops
/// scrolling. `core/src/lib.rs:anchors_from` answers an *empty* vector on a
/// count mismatch, with no error anywhere.
///
/// The fixture carries three headings — two `#` and one `##`. The walk pushes a
/// line for a heading before it looks at `level` at all and the query names no
/// level, so all three are counted on both sides.
#[test]
fn sectioned_numbering_withdraws_no_heading_anchor() {
    let rendered =
        md_to_pdf_with_anchors(SECTIONED_FIGURES_MD, &[asset("dot.png", DOT_PNG)]).unwrap();
    assert_eq!(
        rendered.anchors.len(),
        3,
        "the fixture's three headings did not come back: {:?}",
        rendered.anchors
    );
}

/// A name outside the set names the key, its line, and what it accepts.
///
/// It reads exactly as the `template` and `equations` errors do, because it is
/// the same mechanism. The bad value is `numbered` deliberately: it is the
/// *other* numbering key's valid name, which is the mistake an author who knows
/// `equations` actually makes, and it would otherwise reach Typst as a string
/// no rule matches and section nothing, silently.
#[test]
fn a_figures_value_outside_the_schema_is_an_error_that_lists_the_names() {
    let md = "---\nfigures: numbered\n---\n\n# Heading\n";
    match md_to_typst(md, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 2);
            for needle in ["figures", "flat", "sectioned"] {
                assert!(problem.contains(needle), "problem was: {problem}");
            }
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }
}

// -- mpdf-005 Phase 8: a heading may carry its number -------------------------

#[test]
fn the_numbered_headings_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(NUMBERED_HEADINGS_MD, &[]).unwrap(),
        NUMBERED_HEADINGS_TYP
    );
}

/// The fixture compiles, and under both looks.
///
/// The compile is a real assertion here rather than a smoke test, for the
/// reason `the_sectioned_figures_fixture_compiles_to_a_pdf` records one level
/// down: the number is built by a closure that compares `n.pos().len()` against
/// `int(headings)`, and a look that reached for the depth wrongly — converting a
/// name that is not a number, or indexing a level that is not there — fails here
/// rather than printing a wrong number.
#[test]
fn the_numbered_headings_fixture_compiles_to_a_pdf() {
    for (md, what) in [
        (NUMBERED_HEADINGS_MD.to_string(), "the article look"),
        (
            NUMBERED_HEADINGS_MD.replace("headings: 2", "headings: 2\ntemplate: press-release"),
            "the press-release look",
        ),
    ] {
        let pdf = md_to_pdf(&md, &[]).unwrap_or_else(|e| panic!("{what} did not compile: {e}"));
        assert!(pdf.starts_with(b"%PDF"), "{what} did not produce a PDF");
    }
}

/// Both keys cross to the look, and the reference the page's number is read
/// through survives the phase.
///
/// The *numbers* are deliberately not here and cannot be: the emitter writes no
/// numbering at all, so `1.1 Background` and `Table 1.1` exist only in the
/// compiled PDF. What a golden can hold is that the author's two asks reached
/// the call and that the reference is still a `#ref`, which is what the by-eye
/// read of one PDF per look is for. The depth crosses as the string it was
/// written as, which is what keeps the conversion in the look.
#[test]
fn the_numbered_headings_golden_carries_each_form() {
    for (form, what) in [
        ("headings: \"2\"", "the depth the author set"),
        ("figures: \"sectioned\"", "the scheme it composes with"),
        ("#ref(<tab:one>)", "the reference that reads the number"),
    ] {
        assert!(
            NUMBERED_HEADINGS_TYP.contains(form),
            "the golden file does not carry {what} as `{form}`"
        );
    }
}

/// Numbering the headings withdraws no heading anchor.
///
/// **This is the assertion whose absence would let the phase silently empty
/// every anchor list in the project**, and it is not inherited from
/// `sectioned_numbering_withdraws_no_heading_anchor` by having passed once:
/// that phase changed whether a heading advanced a counter, and this one
/// changes what every heading *renders as*. `core/src/lib.rs:anchors_from`
/// answers an *empty* vector when the walk's heading count and the compiled
/// document's disagree, silently, taking `mpdf-003` Phase 6's scroll sync and
/// `web/src/lib.rs:anchors` with it.
///
/// The fixture carries five headings — two `#`, two `##` and one `###`. The
/// `###` is past the cap and carries no number, which is exactly the case a
/// depth implemented by suppressing the heading *element* would have lost here
/// rather than on the page.
#[test]
fn numbered_headings_withdraw_no_heading_anchor() {
    let rendered = md_to_pdf_with_anchors(NUMBERED_HEADINGS_MD, &[]).unwrap();
    assert_eq!(
        rendered.anchors.len(),
        5,
        "the fixture's five headings did not come back: {:?}",
        rendered.anchors
    );
}

/// A value outside the set names the key, its line, and what it accepts.
///
/// It reads exactly as the `template`, `equations` and `figures` errors do,
/// because it is the same mechanism. Both bad values are deliberate. `numbered`
/// is the *other* numbering keys' valid name and the guess an author who knows
/// them makes, and this schema has no synonym for `6` — so the error is where
/// they are told the depth is the way to ask. `7` is the boundary, one past the
/// six levels markdown has.
#[test]
fn a_headings_value_outside_the_schema_is_an_error_that_lists_the_names() {
    for value in ["numbered", "7"] {
        let md = format!("---\nheadings: {value}\n---\n\n# Heading\n");
        match md_to_typst(&md, &[]) {
            Err(Error::Frontmatter {
                location: Location { file: None, line },
                problem,
            }) => {
                assert_eq!(line, 2, "wrong line for {value}");
                for needle in ["headings", "plain", "1", "6"] {
                    assert!(problem.contains(needle), "problem was: {problem}");
                }
            }
            other => panic!("expected a Frontmatter error for {value}, got {other:?}"),
        }
    }
}

// -- mpdf-008 Phase 1: a master and the sections it names ----------------------

/// Four files, one golden.
///
/// The pair here is a master and three sections rather than one fixture, and the
/// golden is the emitter's answer to what `core` joined out of them — which is
/// the whole claim: the emitter is handed one string, exactly as it always was.
#[test]
fn the_multi_file_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(MULTI_FILE_MD, &multi_file_sections()).unwrap(),
        MULTI_FILE_TYP
    );
}

#[test]
fn the_multi_file_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(MULTI_FILE_MD, &multi_file_assets()).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "not a PDF");
    assert!(pdf.len() > 1000, "suspiciously small: {} bytes", pdf.len());
}

/// Every document-wide mechanism, crossing a file boundary.
///
/// The equality test above pins the whole output; this says why it is right.
/// **Not one of these needed anything added for it.** `collect_definitions` and
/// `emit` walk one event stream, and `check_references` and `check_citations` run
/// once after the walk over the names `declare` gathered — none of them can tell
/// where the bytes came from, and none of them needs to.
#[test]
fn the_multi_file_golden_carries_each_form() {
    let source = md_to_typst(MULTI_FILE_MD, &multi_file_sections()).unwrap();

    // A figure declared in the first file and one declared in the second, each
    // labelled, so the sectioned scheme numbers them 1.1 and 2.1.
    assert!(
        source.contains("caption: [The pipeline, declared in the first file.]) <fig:pipeline>")
    );
    assert!(source.contains("caption: [The mark, declared in the second file.]) <fig:mark>"));
    assert!(source.contains("figures: \"sectioned\""));

    // Both read from the third file.
    assert!(source.contains("#ref(<fig:pipeline>) produced the mark of #ref(<fig:mark>)"));

    // A footnote cited in the second file and defined in the third, set at the
    // reference the way Typst takes one.
    assert!(source.contains("A claim the third file footnotes.#footnote[The note, defined"));

    // The master's frontmatter is the document's, and the sections carry none.
    assert!(source.contains("title: \"A Report Written Across Four Files\""));

    // No marker survives into the source: each was replaced by what it named.
    assert!(
        !source.contains("#link(\"sections/"),
        "a marker reached the page"
    );
}

/// **The join is a blank line, and a naive one merges two blocks silently.**
///
/// A file ending `Last line of part one.` concatenated straight onto one
/// beginning `First line of part two.` is *one* paragraph, the two sentences
/// separated by a soft break — no error, and nothing on the page to see. This is
/// the case `cat a.md b.md` fails, so it is asserted on the shape of the output
/// rather than on the absence of an error.
#[test]
fn two_sections_meeting_at_a_paragraph_stay_two_paragraphs() {
    let master = "[](one.md)\n\n[](two.md)\n";
    let sections = [
        section("one.md", "Last line of part one."),
        section("two.md", "First line of part two."),
    ];

    let source = md_to_typst(master, &sections).unwrap();

    assert!(
        source.contains("Last line of part one.\n\nFirst line of part two."),
        "the two sections merged into one paragraph: {source:?}"
    );
}

/// **An error inside the third section names the third section's own file and
/// its own line**, and not a line of a document nobody wrote.
///
/// Measured before this phase: a `\undefinedcmd` on line 4 of a second section
/// reported `math error at line 11` — line 11 of the joined document, naming no
/// file at all. Asserted here on the exact string, because the phrasing is the
/// deliverable.
#[test]
fn an_error_inside_a_section_names_that_sections_file_and_line() {
    let master = "---\ntitle: A Report\n---\n\n[](one.md)\n\n[](two.md)\n\n[](three.md)\n";
    let sections = [
        section("one.md", "# One\n\nA paragraph.\n"),
        section("two.md", "# Two\n\nAnother paragraph.\n"),
        section(
            "three.md",
            "# Three\n\nText.\n\nA formula $\\undefinedcmd$.\n",
        ),
    ];

    match md_to_typst(master, &sections) {
        Err(error) => assert_eq!(
            error.to_string(),
            "math error in three.md at line 5: unsupported command '\\undefinedcmd'"
        ),
        Ok(_) => panic!("the formula compiled"),
    }
}

/// A master may carry prose of its own, and its own lines stay its own.
///
/// The map covers the master's runs as well as the sections', so a construct
/// error after a spliced section still names the master's line — where the
/// joined line is several further down. Without that the master would be the one
/// file whose messages were wrong.
#[test]
fn the_masters_own_lines_survive_a_splice_above_them() {
    let master = "---\ntitle: A Report\n---\n\nA preface of the master's own.\n\n[](one.md)\n\n<div>a block</div>\n";
    let sections = [section("one.md", "# One\n\nA.\n\nB.\n\nC.\n\nD.\n\nE.\n")];

    match md_to_typst(master, &sections) {
        Err(error) => assert_eq!(
            error.to_string(),
            "unsupported markdown construct 'raw HTML block' at line 9"
        ),
        Ok(_) => panic!("the block compiled"),
    }
}

/// **A marker is a whole paragraph at the top level, and nothing else is one.**
///
/// `mpdf-005` reserved the empty-text link and this phase claims one further
/// destination under it. Every other shape stays exactly the link it was: a
/// marker inside a sentence, a link carrying text, and a lone link inside a block
/// quote or a list item, where splicing raw bytes over the paragraph would walk
/// the section straight out of its container.
#[test]
fn only_a_top_level_paragraph_of_its_own_is_a_marker() {
    for (md, what) in [
        (
            "A [](one.md) inside a sentence.\n",
            "a marker inside a sentence",
        ),
        ("[the section](one.md)\n", "a link carrying text"),
        ("> [](one.md)\n", "a lone link inside a block quote"),
        ("- [](one.md)\n", "a lone link inside a list item"),
        ("[](one.png)\n", "a destination that is not markdown"),
        ("[](../one.md)\n", "a destination the path rule refuses"),
    ] {
        assert!(
            md2pdf_core::section_paths(md).unwrap().is_empty(),
            "{what} was read as a marker"
        );

        // And it reaches the page as the link it has always been, with no
        // section supplied and no refusal raised.
        let source = md_to_typst(md, &[]).unwrap();
        assert!(
            source.contains("#link("),
            "{what} stopped being a link: {source:?}"
        );
    }
}

/// The shopping list names the sections in the order the master reads them.
///
/// It runs before the other two and takes the master's own text alone, because
/// the markers are in the master and no join is needed to see them.
#[test]
fn section_paths_names_every_section_in_reader_order() {
    let named = md2pdf_core::section_paths(MULTI_FILE_MD).unwrap();
    let paths: Vec<&str> = named.iter().map(|s| s.path.as_str()).collect();

    assert_eq!(
        paths,
        vec![
            "sections/introduction.md",
            "sections/method.md",
            "sections/results.md"
        ]
    );

    // Each carries the master's own line, and never a file: a section may not
    // name a section, so there is nothing for one to relocate through.
    assert_eq!(
        named.iter().map(|s| s.location.clone()).collect::<Vec<_>>(),
        vec![Location::at(7), Location::at(9), Location::at(11)]
    );
}

/// The image list names the file each image was drawn in, and the path that
/// file's own directory resolves it to.
///
/// The two halves answer different questions and both are the shopping list's.
/// The **location** is where the author would go to edit it; the **path** is what
/// the caller must open and what the Typst source asks for. Each section writes
/// a bare `dot.png` or `mark.svg`, and each comes back under `sections/`.
#[test]
fn the_image_list_names_the_section_that_drew_each_image() {
    let images = image_paths(MULTI_FILE_MD, &multi_file_sections()).unwrap();

    assert_eq!(
        images,
        vec![
            ImageRef {
                path: "sections/dot.png".to_string(),
                location: Location {
                    file: Some("sections/introduction.md".to_string()),
                    line: 6
                }
            },
            ImageRef {
                path: "sections/mark.svg".to_string(),
                location: Location {
                    file: Some("sections/method.md".to_string()),
                    line: 8
                }
            },
        ]
    );
}

/// An anchor names the file its heading was written in.
///
/// `mpdf-003` Phase 6 pairs the Nth walked heading with the Nth typeset one, and
/// that pairing is untouched: what widens is what each half of the pair says. The
/// app does not read the file until Phase 3, and OQ-5 is what decides what it
/// does with it.
#[test]
fn an_anchor_names_the_file_its_heading_was_written_in() {
    let rendered = md_to_pdf_with_anchors(MULTI_FILE_MD, &multi_file_assets()).unwrap();

    let places: Vec<(Option<&str>, usize)> = rendered
        .anchors
        .iter()
        .map(|anchor| (anchor.location.file.as_deref(), anchor.location.line))
        .collect();

    assert_eq!(
        places,
        vec![
            (Some("sections/introduction.md"), 1),
            (Some("sections/method.md"), 4),
            (Some("sections/results.md"), 1),
        ]
    );
}

/// A document that names no section is joined to itself.
///
/// The inertness property, said at the level the join lives at: one segment, one
/// map entry, and a translation that is the identity by arithmetic rather than by
/// a branch. Every golden above is the same claim, one document at a time.
#[test]
fn a_document_with_no_marker_is_the_document_it_always_was() {
    for md in [BASIC_MD, FOOTNOTES_MD, CROSS_REFERENCES_MD, CITATIONS_MD] {
        assert_eq!(
            md_to_typst(md, &[]).unwrap(),
            md_to_typst(md, &multi_file_sections()).unwrap(),
            "an unnamed section changed the output"
        );
    }
}

// -- mpdf-008 Phase 2: a section names its own neighbours ---------------------

/// The master, and two chapter folders that each hold a `figure.svg`.
///
/// **This is the case the written-path identity made impossible.** Both chapters
/// write the same three characters, and before this phase both emitted
/// `#image("figure.svg")` — two byte-identical calls with nothing to tell them
/// apart, so a caller resolving them against different directories would read the
/// first, skip the second as already seen, and set one figure twice.
fn two_chapters() -> (&'static str, Vec<Asset>) {
    let master = "---\ntitle: Two Chapters\n---\n\n[](one/chapter.md)\n\n[](two/chapter.md)\n";
    let sections = vec![
        section(
            "one/chapter.md",
            "# One\n\n![The first figure](figure.svg)\n",
        ),
        section(
            "two/chapter.md",
            "# Two\n\n![The second figure](figure.svg)\n",
        ),
    ];
    (master, sections)
}

/// Two folders, two figures, one written name.
///
/// The observable: a PDF whose chapter folders each hold their own figure, so
/// the paths inside a folder survive it being moved. Asserted on the source as
/// well as read, because two distinct destinations are what the compiled page
/// showing two different images rests on.
#[test]
fn two_chapter_folders_each_keep_their_own_figure() {
    let (master, sections) = two_chapters();

    let source = md_to_typst(master, &sections).unwrap();
    assert!(
        source.contains(r#"image("one/figure.svg", alt: "The first figure")"#),
        "{source}"
    );
    assert!(
        source.contains(r#"image("two/figure.svg", alt: "The second figure")"#),
        "{source}"
    );

    // Two files because they are two names. Nothing detects a collision, because
    // the prefix is what stops there being one.
    let mut assets = sections;
    assets.push(asset("one/figure.svg", MARK_SVG));
    assets.push(asset("two/figure.svg", RING_SVG));

    let pdf = md_to_pdf(master, &assets).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "not a PDF");
    assert!(pdf.len() > 1000, "suspiciously small: {} bytes", pdf.len());
}

/// The rule reaches an image inside a footnote definition too.
///
/// **This is the only case that exercises the second walk.**
/// `collect_definitions` translates every definition before the document is
/// written, in a walk of its own that produces `ImageRef`s of its own, so a
/// prefix applied in `emit` and not there would lose every image inside a
/// definition — silently, since the destination would simply stay as written.
#[test]
fn an_image_inside_a_footnote_definition_takes_its_sections_directory() {
    let master = "[](one/chapter.md)\n";
    let sections = [section(
        "one/chapter.md",
        "# One\n\nA claim.[^n]\n\n[^n]: The note, which draws ![a figure](figure.svg).\n",
    )];

    let source = md_to_typst(master, &sections).unwrap();
    assert!(
        source.contains(r#"image("one/figure.svg", alt: "a figure")"#),
        "the definition's image kept the path it was written with: {source}"
    );

    // And the shopping list names it, so the caller opens the right file.
    let images = image_paths(master, &sections).unwrap();
    assert_eq!(
        images,
        vec![ImageRef {
            path: "one/figure.svg".to_string(),
            location: Location {
                file: Some("one/chapter.md".to_string()),
                line: 5
            }
        }]
    );
}

/// A file the caller did not supply is named by the path it resolved to.
///
/// **Both halves of the sentence, and they answer different questions.** The
/// quoted path is what the caller must open, in the frame it supplies assets in;
/// the bare file after `in` is where the author would go to edit it. Asserted at
/// the library level because that is the only level that reaches it — the CLI
/// fails earlier at its own `std::fs::read` and prints the resolved path itself.
#[test]
fn a_missing_image_in_a_section_names_the_path_it_resolved_to() {
    let master = "---\ntitle: A Report\n---\n\n[](sections/method.md)\n";
    let sections = [section(
        "sections/method.md",
        "# Method\n\n![A figure](figure.png)\n",
    )];

    match md_to_pdf(master, &sections) {
        Err(error) => assert_eq!(
            error.to_string(),
            "no image file supplied for 'sections/figure.png' in sections/method.md at line 3"
        ),
        Ok(_) => panic!("a document with no image file compiled"),
    }
}

/// The two shapes that prefix with nothing.
///
/// A master's own images are already written in the frame the caller supplies
/// assets in, and a section beside the master has no directory to give. The idiom
/// is what is being pinned: a naive `format!("{dir}/{dest}")` yields `/dot.png`,
/// which `portable_path` refuses as absolute — loud rather than silent, but
/// wrong.
#[test]
fn a_file_with_no_directory_of_its_own_prefixes_with_nothing() {
    // The master's own image, in a document that also names a section.
    let master = "![A dot](dot.png)\n\n[](sections/method.md)\n";
    let sections = [section("sections/method.md", "# Method\n\nText.\n")];
    let source = md_to_typst(master, &sections).unwrap();
    assert!(
        source.contains(r#"image("dot.png", alt: "A dot")"#),
        "the master's own image moved: {source}"
    );

    // A section beside the master.
    let master = "[](chapter.md)\n";
    let sections = [section("chapter.md", "# One\n\n![A dot](dot.png)\n")];
    let source = md_to_typst(master, &sections).unwrap();
    assert!(
        source.contains(r#"image("dot.png", alt: "A dot")"#),
        "a section beside the master gained a prefix: {source}"
    );
}

/// The prefix launders no path the dialect refuses.
///
/// **The written half of the check runs on what the author wrote**, which is why
/// the absolute row below is still a refusal: prefixed first it would have become
/// `sections//x.png`, and `typst-syntax` normalises a non-leading empty segment
/// away — so the check would have read `/sections/x.png` and accepted it, turning
/// an absolute path into a relative one with nothing raised. An empty destination
/// is the second row of the same argument: prefixed first it would have stopped
/// being empty and been refused for having no extension instead.
///
/// **These rows are what proves the shape check was split and not flipped**, and
/// they are byte-identical to the ones that shipped before `..` was allowed to
/// land back inside the folder.
#[test]
fn the_prefix_launders_no_path_the_dialect_refuses() {
    let master = "---\ntitle: A Report\n---\n\n[](sections/method.md)\n";

    for (dest, shape) in [
        ("/x.png", "an absolute path"),
        ("https://example.com/x.png", "a URL destination"),
        ("", "an empty destination"),
    ] {
        let sections = [section(
            "sections/method.md",
            &format!("# Method\n\n![A figure]({dest})\n"),
        )];

        match md_to_typst(master, &sections) {
            Err(error) => assert_eq!(
                error.to_string(),
                format!(
                    "unsupported markdown construct 'image with {shape}' \
                     in sections/method.md at line 3"
                )
            ),
            Ok(_) => panic!("'{dest}' inside a section was accepted"),
        }
    }
}

/// A section may name a figure beside the master.
///
/// **The observable this phase produces.** `../figures/plot.svg` written in
/// `sections/one.md` is prefixed to `sections/../figures/plot.svg`, which lands
/// on `figures/plot.svg` — inside the master's own folder, escaping nothing — and
/// it is the landing place the dialect judges. Before this phase there was no
/// legal way to write it at all.
#[test]
fn a_section_may_name_a_figure_beside_the_master() {
    let master = "---\ntitle: A Report\n---\n\n[](sections/one.md)\n";
    let one = section(
        "sections/one.md",
        "# One\n\n![A plot](../figures/plot.svg)\n",
    );

    // The path the master would have written, which is the frame a caller
    // supplies assets in.
    let source = md_to_typst(master, std::slice::from_ref(&one)).unwrap();
    assert!(
        source.contains(r#"image("figures/plot.svg", alt: "A plot")"#),
        "the section's figure did not land beside the master: {source}"
    );

    let assets = [one, asset("figures/plot.svg", MARK_SVG)];
    let pdf = md_to_pdf(master, &assets).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "not a PDF");
}

/// A section that climbs past the master's own folder is still refused.
///
/// The rule did not go away; it moved to the destination's landing place. One
/// `..` from `sections/` lands inside, two climb out, and the second is refused
/// in the section's own file at the author's own line.
#[test]
fn a_section_that_climbs_out_of_the_document_is_refused() {
    let master = "---\ntitle: A Report\n---\n\n[](sections/one.md)\n";
    let sections = [section(
        "sections/one.md",
        "# One\n\n![A figure](../../escape.png)\n",
    )];

    match md_to_typst(master, &sections) {
        Err(error) => assert_eq!(
            error.to_string(),
            "unsupported markdown construct 'image with a path that leaves \
             the document's folder' in sections/one.md at line 3"
        ),
        Ok(_) => panic!("a path out of the document was accepted"),
    }
}

/// One file named two ways arrives as one string.
///
/// `crate::collect` keys `supplied`, `seen` and the world's `FileId` on the
/// resolved path, so `figures/plot.svg` written by the master and
/// `../figures/plot.svg` written by a section under `sections/` must normalise to
/// the same key or the caller reads one file and sets the other figure from it.
///
/// **`image_paths` deduplicates nothing** — its own contract says the list may
/// name one path more than once, and `seen` in the caller is what supplies one
/// asset — so what is asserted is two entries naming one string.
#[test]
fn one_file_named_two_ways_resolves_to_one_path() {
    let master = "---\ntitle: A Report\n---\n\n\
        ![The master's](figures/plot.svg)\n\n[](sections/one.md)\n";
    let sections = [section(
        "sections/one.md",
        "# One\n\n![The section's](../figures/plot.svg)\n",
    )];

    let named: Vec<String> = image_paths(master, &sections)
        .unwrap()
        .into_iter()
        .map(|image| image.path)
        .collect();
    assert_eq!(named, ["figures/plot.svg", "figures/plot.svg"]);

    let source = md_to_typst(master, &sections).unwrap();
    assert_eq!(
        source.matches(r#"image("figures/plot.svg""#).count(),
        2,
        "the two spellings did not write one destination: {source}"
    );
}

/// A master's own path normalises too, which is the other half of the identity.
///
/// `Sources::resolve` normalises the branch that prefixes nothing as well as the
/// one that prefixes a directory. Normalising only the prefixed branch would
/// leave this document naming `figures/../plot.svg` while a section's equivalent
/// named `plot.svg` — the same identity failure in the other direction.
#[test]
fn a_masters_own_path_normalises_as_a_sections_does() {
    let source = md_to_typst("![A plot](figures/../plot.svg)\n", &[]).unwrap();
    assert!(
        source.contains(r#"image("plot.svg", alt: "A plot")"#),
        "the master's own path was not normalised: {source}"
    );
}

/// A marker path is read under the same widened rule, and is not normalised.
///
/// `lone_markdown_link` decides by `portable_path`, so `[](sub/../one.md)` is a
/// marker where it used to be a plain link — and `[](../one.md)` still is not
/// one, because it still leaves the folder. **What the marker carries is the
/// author's own spelling**: it is what a message names and what a caller is asked
/// to open.
#[test]
fn a_marker_path_may_climb_back_inside_and_keeps_its_spelling() {
    let named = md2pdf_core::section_paths("[](sub/../one.md)\n").unwrap();
    assert_eq!(
        named.iter().map(|s| s.path.as_str()).collect::<Vec<_>>(),
        ["sub/../one.md"]
    );

    let sections = [section("sub/../one.md", "# One\n\nText.\n")];
    let source = md_to_typst("[](sub/../one.md)\n", &sections).unwrap();
    assert!(
        source.contains("= One"),
        "the section was not spliced: {source}"
    );
}

// -- a defect fix, owned by no phase: the accumulator's own lines -------------

/// A key repeated across two frontmatter blocks is reported where it was
/// written.
///
/// **A latent defect in shipped code, reachable by any single document carrying
/// two `---` blocks**, and recorded by `mpdf-008` §2 rather than owned by it —
/// which is why this test sits under no phase. The accumulator is never cleared,
/// so the second block's parse sees both blocks; before the fix it also saw them
/// concatenated end to end and against the *first* block's starting line, so the
/// document below reported `duplicate key 'title' at line 3` — the master's
/// closing `---`, where the offending key is on line 8.
///
/// What the fix does not change is what the two blocks *mean*. Keys that do not
/// collide still merge, exactly as they did, because that is a decision and this
/// is a line number.
#[test]
fn a_key_repeated_across_two_frontmatter_blocks_names_its_own_line() {
    let md = "---\ntitle: Master\n---\n\nSome text.\n\n---\ntitle: Section\n---\n\nMore text.\n";

    match md_to_typst(md, &[]) {
        Err(Error::Frontmatter {
            location: Location { file: None, line },
            problem,
        }) => {
            assert_eq!(line, 8, "the key 'title' is on line 8");
            assert!(problem.contains("duplicate key 'title'"), "{problem}");
        }
        other => panic!("expected a Frontmatter error, got {other:?}"),
    }

    // Two blocks whose keys do not collide still merge, and both reach the look.
    let merged = "---\ntitle: Master\n---\n\nSome text.\n\n---\nauthor: Someone\n---\n";
    let source = md_to_typst(merged, &[]).unwrap();
    assert!(
        source.contains("title: \"Master\", author: ((name: \"Someone\", markers: ()),)"),
        "{source}"
    );
}

// -- mpdf-005 Phase 9: a listing sits off the margin --------------------------

/// Both bundled looks send a block of code off the margin, and a listing's
/// caption with it.
///
/// A test of its own rather than an extension of
/// `every_bundled_template_places_a_listing`, whose name is about where a
/// listing is *aligned*: hanging an inset off it would leave a test whose name
/// had stopped describing it, which is the argument Phase 1 gate (6) and Phase 5
/// gate (8) each made.
///
/// **The needles are `raw.where(block: true)` and
/// `figure.caption.where(kind: raw)`, and the `2em` is deliberately not one.**
/// The value is each look's own call, on the precedent this file already records
/// for an equation's format, a group's gutter and a caption's separator.
///
/// **Two rules and not one, because the first cannot carry the second.** The
/// inset is on `raw` rather than on the `figure` — a figure rule reaches only a
/// *captioned* listing, so the same code would stand at two edges in one
/// document, which is the defect
/// `every_bundled_template_places_a_listing`'s rule removed. And a caption is
/// not a `raw` block, so it stays at the margin under a block that has moved
/// unless a rule of its own moves it.
///
/// Even together the two are needles over source, so they would pass a look
/// carrying both rules and a third that defeated them. That the twins land on
/// one edge is what the by-eye read on one PDF per look is for.
#[test]
fn every_bundled_template_insets_a_listing() {
    for (file, source) in BUNDLED_TEMPLATES {
        for needle in ["raw.where(block: true)", "figure.caption.where(kind: raw)"] {
            assert!(source.contains(needle), "{file} does not carry `{needle}`");
        }
    }
}

// -- mpdf-001 Phase 11: several authors, and their affiliations ----------------

/// The fixture that carries the whole grammar, against its golden file.
///
/// Four things are pinned here that nothing else in the suite says. The
/// comma'd name `Po, Iva` survives as **one** element, which is the schema's
/// sharpest call — a comma is an ordinary way to write one person's name, so
/// the separator is a `;`. The markers written `^1, 2`, with the space an
/// author actually types, reduce to two numbers. A one-element array carries
/// its trailing comma, because `("Iva Po")` is a parenthesized string and not
/// an array at all. And an author naming two affiliations keeps both, in
/// written order.
#[test]
fn the_authors_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(AUTHORS_MD, &[]).unwrap(), AUTHORS_TYP);
}

#[test]
fn the_authors_fixture_compiles() {
    let pdf = md_to_pdf(AUTHORS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// With exactly one affiliation the markers are optional — the middle rung of a
/// three-way answer — and a document that writes none is valid.
///
/// This is the case OQ-11 said its gate could not state. The schema answers by
/// making the marker optional rather than by refusing it: one lab and several
/// authors is the commonest real paper, and refusing the unmarked form would
/// leave its author only the lone superscript on every name that says nothing.
/// From two affiliations up the relation has to be stated; with none there is
/// no relation and the markers are cleared, which
/// `markers_with_nothing_to_point_at_are_cleared` reads.
///
/// A marker written anyway at exactly one affiliation is honoured, because the
/// author wrote it and it is true. That is the half of OQ-11 the zero case
/// cannot carry, and this test is where it is pinned.
///
/// Both forms are read, and in both looks. The marked twin is inline rather
/// than a second fixture, because what separates the two is one character in
/// the frontmatter and a fixture would say nothing the substitution does not.
///
/// **What the source shows is that the markers are absent, not that the page is
/// bare.** Whether a marker reaches the page is the look's, read off exactly
/// this data — so an empty `markers` on every author is the fact the looks
/// branch on, and the PDFs are read by eye beside it.
#[test]
fn one_affiliation_carries_its_markers_or_leaves_them_out() {
    let marked = ONE_AFFILIATION_MD.replace(
        "author: Iva Po; Someone Else",
        "author: Iva Po^1; Someone Else^1",
    );
    assert_ne!(
        marked, ONE_AFFILIATION_MD,
        "the substitution matched nothing"
    );

    for look in ["", "template: press-release\n"] {
        let unmarked = ONE_AFFILIATION_MD.replacen("---\n", &format!("---\n{look}"), 1);
        let marked = marked.replacen("---\n", &format!("---\n{look}"), 1);

        let source = md_to_typst(&unmarked, &[]).unwrap();
        assert!(
            source.contains(
                "author: ((name: \"Iva Po\", markers: ()), (name: \"Someone Else\", markers: ()))"
            ),
            "an unmarked author reached the look with a marker: {source}"
        );
        assert!(
            source.contains("affiliation: (\"Anthropic, San Francisco\",)"),
            "the one affiliation is missing its trailing comma: {source}"
        );

        let source = md_to_typst(&marked, &[]).unwrap();
        assert!(
            source.contains(
                "author: ((name: \"Iva Po\", markers: (1,)), (name: \"Someone Else\", markers: (1,)))"
            ),
            "a marker the author wrote did not reach the look: {source}"
        );

        for document in [&unmarked, &marked] {
            let pdf = md_to_pdf(document, &[]).unwrap();
            assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
        }
    }
}

// -- mpdf-001 Phase 12: a marker with nothing to point at ----------------------

/// A marker in a document that names no affiliation is cleared, and the
/// document compiles where today it exits non-zero.
///
/// The refusal this narrows was stated absolutely by Phase 11, so `^1` with no
/// `affiliation` key at all stopped the build over markers that state nothing —
/// a state an author reaches by commenting the key out while drafting, or by
/// writing the markers before the key.
///
/// **`core` clears the markers rather than a look ignoring them**, which is what
/// this reads: `markers: ()` on **every** author and `affiliation: none`, so
/// what crosses the seam is the truth about the document and neither look
/// changes by a character. A clearing pass that emptied only the first author
/// would pass a needle over one name and still set a `2` in the byline, since
/// both looks call `super()` per author with no guard on `affiliation` — so the
/// assertion names both authors in one string.
///
/// This is an inline assertion rather than a golden file, on the shape
/// `one_affiliation_carries_its_markers_or_leaves_them_out` already uses: a
/// golden would pin the whole document where what this phase changes is one
/// argument.
#[test]
fn markers_with_nothing_to_point_at_are_cleared() {
    let source = md_to_typst(ORPHAN_MARKERS_MD, &[]).unwrap();
    assert!(
        source.contains(
            "author: ((name: \"Iva Po\", markers: ()), (name: \"Someone Else\", markers: ()))"
        ),
        "a marker survived a document that names no affiliation: {source}"
    );
    assert!(
        source.contains("affiliation: none"),
        "the document grew an affiliation it never wrote: {source}"
    );

    let pdf = md_to_pdf(ORPHAN_MARKERS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

// -- mpdf-005 Phase 10: the abstract a paper opens with ----------------------

/// The fixture matches its golden, and the golden carries the widened import.
///
/// The import is the load-bearing half. `header` names `abstract` only for a
/// document that opened one, which is what keeps every shipped golden file
/// byte-identical — the suite reads all thirty-five of them, so an
/// implementation that widened it unconditionally fails here and there at once.
/// The count read thirty when this test was written and the directory held
/// thirty-one; re-measured with `ls tests/golden | wc -l`, which is the
/// instrument, so a later reader checks it rather than trusting it.
#[test]
fn the_abstract_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(ABSTRACT_MD, &[]).unwrap(), ABSTRACT_TYP);
    assert!(
        ABSTRACT_TYP.starts_with("#import \"template.typ\": template, divider, abstract\n"),
        "the golden does not carry the widened import line"
    );
    // Two paragraphs and one call over both of them: the block collects what
    // stands between the delimiters rather than only the first block of it.
    assert_eq!(
        ABSTRACT_TYP.matches("#abstract[").count(),
        1,
        "the golden does not carry exactly one abstract"
    );
}

#[test]
fn the_abstract_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(ABSTRACT_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The abstract takes no number and withdraws no anchor.
///
/// **This is the case that fails a look spelling the label as a `heading`**, and
/// the only one that does: the string `Abstract` is on the page either way, so
/// the read-by-eye case passes such a look too. `anchors_from` returns an empty
/// vector on any mismatch between the headings the walk counted and the headings
/// the compiled document carries — silently, by design — so one heading the
/// markdown never wrote withdraws every anchor in the document and takes the
/// desktop pane's scroll sync with it.
///
/// The fixture's `figures: sectioned` and `headings: 2` are what make the other
/// half of that defect visible, read by eye once: its `#` heading sets `1` and
/// its captioned table `Table 1.1`, where a heading-shaped label sets `2` and
/// `Table 2.1`.
#[test]
fn an_abstract_withdraws_no_heading_anchor() {
    let rendered = md_to_pdf_with_anchors(ABSTRACT_MD, &[]).unwrap();
    assert_eq!(
        rendered.anchors.len(),
        1,
        "the abstract moved the heading count: {:?}",
        rendered.anchors
    );
}

/// An abstract may be a section file of its own.
///
/// `mpdf-008` joins before the walk begins, so the first block of the document
/// is the first block of the *stream* and a section file carrying nothing else
/// is where an abstract most naturally lives. This is the case a first-block
/// test written against the master would refuse.
#[test]
fn an_abstract_may_be_written_in_its_own_section_file() {
    let sections = vec![
        section("sections/abstract.md", SECTION_ABSTRACT_MD),
        section("sections/intro.md", SECTION_INTRO_MD),
    ];
    let source = md_to_typst(ABSTRACT_SECTIONS_MD, &sections).unwrap();
    assert!(
        source.contains("#abstract["),
        "the abstract did not survive the join: {source}"
    );

    let pdf = md_to_pdf(ABSTRACT_SECTIONS_MD, &sections).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// Every abstract refusal names its construct and its line.
///
/// **Sixteen rows over eight refusals and one message.** The arithmetic is the
/// eight, plus two more for the frame refusal, one more for the not-first-block
/// one, the three pairings the nesting message covers, and one each for the two
/// refusals whose second half is a construct of its own rather than a second
/// spelling — the image, which the block guard cannot see, and the citation.
///
/// **The three frame rows do not fail independently, and that is not what makes
/// them three.** A list item and a block quote both fail `bufs.len() == 1`; only
/// the footnote definition reaches `Mode::Document`. The row is right because an
/// implementation carrying the first two conditions alone passes it **by
/// compiling**, which is the strongest discrimination available and does not
/// depend on what literal the implementation chose.
///
/// **The footnote row's definition must be cited.** `Notes::enter` refuses an
/// uncited definition with its own message *before* the stored error is
/// re-raised, so a row whose `[^1]` is never referenced asserts
/// `footnote definition that no reference cites` instead — measured, not
/// assumed.
///
/// **The not-first-block row is asserted twice** and the two do not differ in
/// mechanism: `step`'s heading arm pushes its newline, `=`s and space at
/// `Start`, so a content check catches the heading exactly as it catches the
/// paragraph. They are kept because they are free; the frame rows are the ones
/// that discriminate.
#[test]
fn each_abstract_refusal_names_its_construct_and_its_line() {
    for (md, construct, line, what) in [
        (
            "::: abstract\n\nOne.\n\n:::\n\n::: abstract\n\nTwo.\n\n:::\n",
            "second abstract in one document",
            7,
            "a second abstract in one document",
        ),
        (
            "Body.\n\n::: abstract\n\nOne.\n\n:::\n",
            "abstract that is not the document's first block",
            3,
            "an abstract under a paragraph",
        ),
        (
            "# H\n\n::: abstract\n\nOne.\n\n:::\n",
            "abstract that is not the document's first block",
            3,
            "an abstract under a heading",
        ),
        (
            "- ::: abstract\n\n  One.\n\n  :::\n",
            "abstract inside a list item, a block quote or a footnote definition",
            1,
            "an abstract inside a list item, whose frame is empty where the document's is not",
        ),
        (
            "> ::: abstract\n>\n> One.\n>\n> :::\n",
            "abstract inside a list item, a block quote or a footnote definition",
            1,
            "an abstract inside a block quote, on the same frame reasoning",
        ),
        (
            "A note.[^1]\n\n[^1]: The note.\n\n    ::: abstract\n\n    One.\n\n    :::\n",
            "abstract inside a list item, a block quote or a footnote definition",
            5,
            "an abstract inside a cited footnote definition, which a frame test alone admits",
        ),
        (
            "::: abstract\n\nOne.\n\n## Nope\n\n:::\n",
            "block inside an abstract that is not a paragraph",
            5,
            "a heading inside an abstract",
        ),
        (
            "::: abstract\n\n![a](dot.png)\n\n:::\n",
            "image inside an abstract",
            3,
            "a standalone image, which is a paragraph at the event level",
        ),
        (
            "::: abstract\n\n$$a^2$$\n\n:::\n",
            "display equation inside an abstract",
            3,
            "a display equation, which is an inline event inside a paragraph",
        ),
        (
            "::: abstract\n\nAs [@key] says.\n\n:::\n",
            "citation inside an abstract",
            3,
            "a citation, which is inline text",
        ),
        (
            "::: abstract\n\n:::\n",
            "abstract with no text",
            1,
            "an empty abstract, which would set a label over nothing",
        ),
        (
            "::: abstract\n\n: A caption.\n\n:::\n",
            "caption line inside an abstract",
            3,
            "a `: ` line inside an abstract, which has nowhere to put one",
        ),
        (
            "::: abstract\n\nOne.\n",
            "abstract the document never closes",
            1,
            "an abstract the document never closes, told about an abstract",
        ),
        (
            ":::\n\n![a](dot.png)\n\n::: abstract\n\n:::\n",
            "abstract inside a figure group",
            5,
            "an abstract opened inside a group",
        ),
        (
            "::: abstract\n\nOne.\n\n::: table\n\n:::\n",
            "figure group inside an abstract",
            5,
            "a group opened inside an abstract",
        ),
        (
            "::: abstract\n\nOne.\n\n::: abstract\n\n:::\n",
            "abstract inside an abstract",
            5,
            "an abstract opened inside an abstract, which is nesting and not a second one",
        ),
    ] {
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

/// The reservation is two words, and the marker's other positions are unmoved.
///
/// **`abstract` and `keywords` are the only words the dialect reads.** Every
/// other one still opens the figure group it always did, which is what holds
/// the two narrowings to the two words the census found unused — and a `: `
/// line standing where nothing captionable does is still the ordinary prose it
/// has been since Phase 1, now with a front-matter block above it.
///
/// The name and this comment moved at Phase 11: the assertions did not, because
/// `::: table` is what they probe and a second reserved word leaves it exactly
/// where it was.
#[test]
fn the_dialect_reads_its_reserved_words_and_leaves_the_rest_alone() {
    let group = md_to_typst(
        "::: table\n\n![a](dot.png)\n\n![b](dot.png)\n\n: Two tables' worth.\n\n:::\n",
        &[],
    )
    .unwrap();
    assert!(
        group.contains("#figure(grid(columns: 2,"),
        "a word the dialect does not read stopped opening a group: {group}"
    );
    assert!(
        !group.contains("abstract") && !group.contains("keywords"),
        "a group grew a front-matter block: {group}"
    );

    let after = md_to_typst("::: abstract\n\nOne.\n\n:::\n\n: Just prose.\n", &[]).unwrap();
    assert!(
        after.ends_with(": Just prose.\n"),
        "a `: ` line under no construct stopped being prose: {after}"
    );
}

// -- mpdf-005 Phase 11: the keywords a paper is indexed by -------------------

/// The fixture matches its golden, and the golden is where the escape is pinned.
///
/// **The bytes are the point of this case.** The closer reads a region the walk
/// has *already* markup-escaped, so the terms cross as an array of **content**
/// and no escape runs in this phase at all. Two wrong builds are what the
/// assertion below discriminates against, and neither is subtle:
/// `("cross\-references", …)` is the string-literal draft, where those escapes
/// are not string escapes at all and the compile fails; and
/// `[cross\\\-references]` is a second `escape_into` pass, which puts the
/// backslashes on the page.
///
/// A hyphen is the common case rather than an edge — `-` and `#` are both in
/// `SPECIAL` — which is why the fixture carries `cross-references` and
/// `C# and C++` and a comma-carrying term beside them.
#[test]
fn the_keywords_fixture_matches_its_golden_file() {
    assert_eq!(md_to_typst(KEYWORDS_MD, &[]).unwrap(), KEYWORDS_TYP);
    assert!(
        KEYWORDS_TYP
            .starts_with("#import \"template.typ\": template, divider, abstract, keywords\n"),
        "the golden does not carry the four-name import line"
    );
    assert!(
        KEYWORDS_TYP.contains(
            "#keywords(([cross\\-references], [C\\# and C\\+\\+], \
             [figure numbering, sectioned], [markdown]))"
        ),
        "the golden does not carry the four terms as escaped content"
    );
}

#[test]
fn the_keywords_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(KEYWORDS_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The two import flags are independent, in both directions.
///
/// `abstract.typ` is the one shipped golden that fails an implementation
/// importing `keywords` for every document that has an abstract, and every other
/// golden fails one that widens the list unconditionally. **The reverse
/// direction moves no shipped golden at all**, which is why a document with
/// keywords and no abstract needs a golden of its own.
#[test]
fn a_document_with_keywords_and_no_abstract_imports_neither_the_other_way() {
    assert_eq!(
        md_to_typst(KEYWORDS_ALONE_MD, &[]).unwrap(),
        KEYWORDS_ALONE_TYP
    );
    assert!(
        KEYWORDS_ALONE_TYP.starts_with("#import \"template.typ\": template, divider, keywords\n"),
        "the golden does not carry the three-name import line"
    );
    assert!(
        !KEYWORDS_ALONE_TYP.contains("abstract"),
        "a document with no abstract imported one: {KEYWORDS_ALONE_TYP}"
    );
    assert!(
        !ABSTRACT_TYP.contains("keywords"),
        "a document with no keywords imported them: {ABSTRACT_TYP}"
    );
}

/// The terms cross as separate elements, and the separator is never written here.
///
/// **The check is the array's structure and not a scan for separator
/// characters**, and the fixture is why: the array's own syntax *is* a comma and
/// a space, and one of these four terms contains a comma. Four bracket groups
/// delimited by `], [` is the property, and it fails an emitter that joined the
/// terms itself into one element.
#[test]
fn the_emitter_writes_the_terms_as_elements_and_joins_nothing() {
    let source = md_to_typst(KEYWORDS_MD, &[]).unwrap();
    let call = source
        .split_once("#keywords(")
        .expect("the fixture opens a keywords block")
        .1
        .split_once("))")
        .expect("the call closes")
        .0;
    assert_eq!(
        call.matches("], [").count(),
        3,
        "four terms did not reach the call as four elements: {call}"
    );
    assert!(
        !call.contains('"'),
        "the terms crossed as string literals rather than as content: {call}"
    );
}

/// The separator between two terms is each look's own, applied with `join`.
///
/// **The needle is scoped to the slice beginning at `#let keywords(`, and that
/// is the whole of what makes this case mean anything**: both looks already
/// carry `join(` three times each for the author block, so a whole-file needle
/// passes on a tree with no keywords code in either look — and would still pass
/// a look that wrote `terms.at(0) + ", " + terms.at(1)`, which is the exact
/// defect this exists to catch.
///
/// **The two looks are deliberately not required to differ.** Phase 6 and Phase
/// 9 both recorded that two looks agreeing is not the seam collapsing, so a
/// check that forced disagreement would make house style a correctness property.
/// The separator *values* stay off the needle list for the reason `(1)` against
/// `1.` does.
#[test]
fn every_bundled_look_joins_the_terms_itself() {
    for (file, source) in BUNDLED_TEMPLATES {
        let definition = source
            .split_once("#let keywords(")
            .unwrap_or_else(|| panic!("{file} does not export keywords"))
            .1;
        assert!(
            definition.contains("join("),
            "{file} does not join the terms inside its own keywords definition"
        );
    }
}

/// Keywords take no number and withdraw no anchor.
///
/// The label is styled text in both looks and never a `heading`. One spelled as
/// a heading typesets one `core` never counted, and `anchors_from` returns an
/// **empty vector** on that mismatch — silently, taking the desktop pane's
/// scroll sync with it and leaving nothing on the page to show for it. It would
/// also restart every counter under `figures: sectioned`, which the fixture's
/// own `Table 1.1` is read by eye for.
#[test]
fn keywords_withdraw_no_heading_anchor() {
    let rendered = md_to_pdf_with_anchors(KEYWORDS_MD, &[]).unwrap();
    assert_eq!(
        rendered.anchors.len(),
        1,
        "the keywords moved the heading count: {:?}",
        rendered.anchors
    );
}

/// Keywords may stand above an abstract, and the order is the author's.
///
/// **Phase 11 widened Phase 10's refusal rather than replacing it.** The
/// position rule is now "everything above a front-matter block is front matter",
/// so either may follow the other, and the floats stack in the order they are
/// issued. This is the case that fails an implementation that ordered the two
/// constructs for the author.
#[test]
fn keywords_may_stand_above_an_abstract() {
    let source = md_to_typst(KEYWORDS_FIRST_MD, &[]).unwrap();
    let keywords = source
        .find("#keywords(")
        .expect("the document has keywords");
    let r#abstract = source
        .find("#abstract[")
        .expect("the document has an abstract");
    assert!(
        keywords < r#abstract,
        "the emitter reordered the two blocks: {source}"
    );

    let pdf = md_to_pdf(KEYWORDS_FIRST_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// Keywords may be a section file of their own.
///
/// `mpdf-008` joins before the walk begins, so the front matter is the front
/// matter of the *stream*. This is the case a position test written against the
/// master would refuse.
#[test]
fn keywords_may_be_written_in_their_own_section_file() {
    let sections = vec![
        section("sections/keywords.md", SECTION_KEYWORDS_MD),
        section("sections/intro.md", SECTION_INTRO_MD),
    ];
    let source = md_to_typst(KEYWORDS_SECTIONS_MD, &sections).unwrap();
    assert!(
        source.contains("#keywords("),
        "the keywords did not survive the join: {source}"
    );

    let pdf = md_to_pdf(KEYWORDS_SECTIONS_MD, &sections).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
}

/// Every keywords refusal names its construct and its line.
///
/// **Twenty-nine rows over nine refusals and one message.** The arithmetic is
/// the nine, plus two more for the frame refusal, one more for the
/// body-content-above one, three more for the leading, embedded and trailing
/// empty term, eight more for the nine spellings the plain-text rule refuses,
/// the five pairings the nesting message covers — and **one more than the
/// phase's own count, which said twenty-eight**: a formula is one spelling and
/// *two* events, `InlineMath` and `DisplayMath`, so it is two call sites and a
/// row that covered one would leave the other unguarded. The count is written
/// here rather than carried over, because a count whose sentence no longer adds
/// up is this corpus's recurring defect.
///
/// **Each of the nine spellings takes a row of its own**, on the abstract
/// table's own precedent: a single message covering nine constructs is the
/// implementation this refusal exists to fail. The names cannot come from
/// `describe`, which answers `"markdown construct"` for an emphasis and
/// `"supported construct"` for inline code, a formula and a hard break.
///
/// **The footnote row's definition must be cited**, or `Notes::enter` refuses
/// the uncited definition first and the row asserts the wrong string —
/// Phase 10's measurement, reused rather than rediscovered. It is cited from
/// inside the block, which is also the only place a document with keywords in
/// its front matter has to cite it from.
///
/// **Each nesting row's nested opener stands first in the block**, since a
/// nested opener written after a term is a second paragraph — and although the
/// second-paragraph refusal is tested at the closer and so cannot fire on an
/// opener at all, a row that depended on that would be pinning an implementation
/// detail rather than a message.
#[test]
fn each_keywords_refusal_names_its_construct_and_its_line() {
    for (md, construct, line, what) in [
        (
            "::: keywords\n\na\n\n:::\n\n::: keywords\n\nb\n\n:::\n",
            "second keywords block in one document",
            7,
            "a second keywords block in one document",
        ),
        (
            "Body.\n\n::: keywords\n\na\n\n:::\n",
            "keywords with body content above them",
            3,
            "keywords under a paragraph",
        ),
        (
            "# H\n\n::: keywords\n\na\n\n:::\n",
            "keywords with body content above them",
            3,
            "keywords under a heading",
        ),
        (
            "- ::: keywords\n\n  a\n\n  :::\n",
            "keywords inside a list item, a block quote or a footnote definition",
            1,
            "keywords inside a list item, whose frame is empty where the document's is not",
        ),
        (
            "> ::: keywords\n>\n> a\n>\n> :::\n",
            "keywords inside a list item, a block quote or a footnote definition",
            1,
            "keywords inside a block quote, on the same frame reasoning",
        ),
        (
            "A note.[^1]\n\n[^1]: The note.\n\n    ::: keywords\n\n    a\n\n    :::\n",
            "keywords inside a list item, a block quote or a footnote definition",
            5,
            "keywords inside a cited footnote definition, which a frame test alone admits",
        ),
        (
            "::: keywords\n\na\n\n## Nope\n\n:::\n",
            "block inside keywords that is not a paragraph",
            5,
            "a heading inside keywords",
        ),
        (
            "::: keywords\n\na\n\nb\n\n:::\n",
            "second paragraph inside keywords",
            1,
            "a second paragraph inside keywords, caught at the closer",
        ),
        (
            "::: keywords\n\n*a*; b\n\n:::\n",
            "emphasis inside keywords",
            3,
            "emphasis inside keywords",
        ),
        (
            "::: keywords\n\n~~a~~; b\n\n:::\n",
            "strikethrough inside keywords",
            3,
            "strikethrough inside keywords",
        ),
        (
            "::: keywords\n\n`a`; b\n\n:::\n",
            "inline code inside keywords",
            3,
            "inline code inside keywords, whose content could hold the separator",
        ),
        (
            "::: keywords\n\n[a](https://example.com); b\n\n:::\n",
            "link inside keywords",
            3,
            "a link inside keywords, whose URL could hold the separator",
        ),
        (
            "::: keywords\n\n$x$; b\n\n:::\n",
            "formula inside keywords",
            3,
            "an inline formula inside keywords",
        ),
        (
            "::: keywords\n\n$$x$$; b\n\n:::\n",
            "formula inside keywords",
            3,
            "a display formula inside keywords",
        ),
        (
            "::: keywords\n\na[^1]; b\n\n:::\n\n[^1]: The note.\n",
            "footnote inside keywords",
            3,
            "a footnote reference inside keywords, its definition cited from there",
        ),
        (
            "::: keywords\n\n![a](dot.png); b\n\n:::\n",
            "image inside keywords",
            3,
            "an image inside keywords",
        ),
        (
            "::: keywords\n\n[@one]; b\n\n:::\n",
            "citation inside keywords",
            3,
            "a citation inside keywords, which is a link until its end event",
        ),
        (
            "::: keywords\n\na\\\nb\n\n:::\n",
            "hard break inside keywords",
            3,
            "a hard break inside keywords, which an abstract permits",
        ),
        (
            "::: keywords\n\n:::\n",
            "keywords with no terms",
            1,
            "an empty keywords block, which Typst would otherwise set as a bare label",
        ),
        (
            "::: keywords\n\n;a\n\n:::\n",
            "keywords with an empty term",
            1,
            "a leading separator",
        ),
        (
            "::: keywords\n\na;;b\n\n:::\n",
            "keywords with an empty term",
            1,
            "an embedded empty term",
        ),
        (
            "::: keywords\n\na;\n\n:::\n",
            "keywords with an empty term",
            1,
            "a trailing separator",
        ),
        (
            "::: keywords\n\n: A caption.\n\n:::\n",
            "caption line inside keywords",
            3,
            "a `: ` line inside keywords, which `attaches` would otherwise let through",
        ),
        (
            "::: keywords\n\na\n",
            "keywords the document never closes",
            1,
            "keywords the document never closes",
        ),
        (
            "::: abstract\n\n::: keywords\n\n:::\n",
            "keywords inside an abstract",
            3,
            "keywords opened inside an abstract",
        ),
        (
            "::: keywords\n\n::: abstract\n\n:::\n",
            "abstract inside keywords",
            3,
            "an abstract opened inside keywords",
        ),
        (
            "::: keywords\n\n::: keywords\n\n:::\n",
            "keywords inside keywords",
            3,
            "keywords opened inside keywords",
        ),
        (
            "::: table\n\n::: keywords\n\n:::\n",
            "keywords inside a figure group",
            3,
            "keywords opened inside a figure group",
        ),
        (
            "::: keywords\n\n::: table\n\n:::\n",
            "figure group inside keywords",
            3,
            "a figure group opened inside keywords",
        ),
    ] {
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct: found,
                location:
                    Location {
                        file: None,
                        line: found_line,
                    },
            }) => {
                assert_eq!(found, construct, "for {what}");
                assert_eq!(found_line, line, "for {what}");
            }
            other => panic!("expected `{construct}` for {what}, got {other:?}"),
        }
    }
}

// -- mpdf-005 Phase 12: a name may stand on the line below --------------------

/// The fixture matches its golden, and the golden is where the insert is pinned.
///
/// Each label sits immediately after its own closing `$`, and the group's own
/// separator newlines stand *after* it rather than being unwound the way
/// `splice_caption` unwinds a caption's: the consumed paragraph leaves four,
/// the soft break three — one from the break, two from the paragraph's end and
/// the next one's start — and the adjacent form two. **The wrong build this
/// discriminates against is the append that shipped before Phase 12**, which
/// writes `$ x = 1 $\n\n <eq:soft>`: a label after a paragraph break, attached
/// to nothing.
#[test]
fn the_equation_names_apart_fixture_matches_its_golden_file() {
    assert_eq!(
        md_to_typst(EQUATION_NAMES_APART_MD, &[]).unwrap(),
        EQUATION_NAMES_APART_TYP
    );
    for (needle, what) in [
        ("$ x = 1 $ <eq:soft>\n\n\nOr", "a name a soft break below"),
        ("$ y = 2 $ <eq:para>\n\n\n\nOr", "a name a paragraph below"),
        ("$ z = 3 $ <eq:adjacent>\n\n#ref", "a name on the fence"),
        (
            "$ w = 4 $ <eq:four>\n\n\nThis sentence",
            "a name a paragraph below with prose on the line after it",
        ),
    ] {
        assert!(
            EQUATION_NAMES_APART_TYP.contains(needle),
            "{what} does not leave `{}`",
            needle.escape_debug()
        );
    }
    for wrong in ["$\n <eq:", "$\n\n <eq:"] {
        assert!(
            !EQUATION_NAMES_APART_TYP.contains(wrong),
            "a label was appended after the break rather than inserted at the record's end"
        );
    }
}

#[test]
fn the_equation_names_apart_fixture_compiles_to_a_pdf() {
    let pdf = md_to_pdf(EQUATION_NAMES_APART_MD, &[]).unwrap();
    assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
    assert!(pdf.len() > 1000, "the PDF is suspiciously small");
}

/// The insert is the append in the adjacent case, and this is the assertion
/// that says so.
///
/// `plain_equation_names.md`'s first sentence stated the old rule and moved
/// with Phase 12, so its golden was re-blessed. The three markup lines in that
/// file are the adjacent, trailing-text and leading-text spellings, and each
/// has to come through the re-bless byte for byte: a build whose insert point
/// is anything but the record's end moves the first, and one that widened the
/// run rule moves the other two. The other thirty-three goldens hold the same
/// claim through their own whole-file assertions.
#[test]
fn the_plain_equation_names_markup_lines_are_unchanged() {
    for line in [
        "\n$ E = m c ^(2 ) $ <eq:energy>\n",
        "\n$ w = 4 $ {\\#eq:trailing} and more\n",
        "\n$ y = 5 $ see {\\#eq:leading}\n",
    ] {
        assert!(
            PLAIN_EQUATION_NAMES_TYP.contains(line),
            "the re-blessed golden no longer carries `{}`",
            line.escape_debug()
        );
    }
}

/// A name a line below, or a paragraph below, names — the row that inverted.
///
/// The first document is byte for byte the fourth row Phase 4's gate put in
/// `a_group_that_is_not_the_whole_run_names_nothing`, asserting the group
/// stayed prose; it is the one shipped assertion this phase reverses. The
/// second has two blank lines between the fence and the group, because the
/// bound is that everything between is newlines, not that there is one of
/// them. Emitted rather than read off a golden, for the reason that test gives.
#[test]
fn a_name_a_line_or_a_paragraph_below_still_names() {
    for (md, form, what) in [
        (
            "# H\n\n$$\nz = 6\n$$\n{#eq:nextline}\n",
            "$ z = 6 $ <eq:nextline>\n",
            "a group a soft break below, which Phase 4 left as prose",
        ),
        (
            "# H\n\n$$\nz = 6\n$$\n\n\n{#eq:two}\n",
            "$ z = 6 $ <eq:two>\n",
            "a group two blank lines below",
        ),
    ] {
        let typst = md_to_typst(md, &[]).unwrap();
        assert!(typst.contains(form), "{what} did not name: {typst}");
        assert!(
            !typst.contains("{\\#eq:"),
            "{what} reached the page as prose as well: {typst}"
        );
    }
}

/// A paragraph that is nothing but a group, with nothing to name, is refused
/// at its own line.
///
/// Every row printed the group on the page before Phase 12, except the sixth,
/// which took the figure group's own message. **The sixth is the only case in
/// the phase that reaches the opener's retirement of the equation record, and
/// the equation before the opener is what makes it a test at all**: without it
/// there is no record for the opener to retire, a bare `::: figure` /
/// `{#eq:one}` errors under every build, and the row passes green with the
/// retirement unbuilt. With it, a build that keeps the record live inserts the
/// label across `Group.start` and yields the figure-group message, which is
/// what the row asserts against.
///
/// A tight list item is deliberately not a row: `- {#fig:one}` is bare inlines
/// with no paragraph for the refusal to be the whole of, so it stays prose.
#[test]
fn each_nameless_group_refusal_names_its_line() {
    for (md, line, what) in [
        (
            "# H\n\n$$\nx = 1\n$$\n\nSome prose.\n\n{#eq:one}\n",
            9,
            "a group after prose that killed the record",
        ),
        ("# H\n\n{#eq:one}\n", 3, "a group after a heading"),
        ("{#eq:one}\n", 1, "a group with nothing above it at all"),
        (
            "# H\n\n![a](dot.png)\n\n{#fig:one}\n",
            5,
            "a group under an uncaptioned image, whose record is not an equation's",
        ),
        (
            "# H\n\n- one\n\n- {#eq:one}\n",
            5,
            "a group alone in a loose list item",
        ),
        (
            "# H\n\n$$x = 1$$\n\n::: figure\n\n{#eq:one}\n",
            7,
            "a group after a `:::` opener, which retires the equation before it",
        ),
    ] {
        match md_to_typst(md, &[]) {
            Err(Error::UnsupportedConstruct {
                construct,
                location:
                    Location {
                        file: None,
                        line: found,
                    },
            }) => {
                assert_eq!(construct, "name group with nothing to name", "for {what}");
                assert_eq!(found, line, "for {what}");
            }
            other => panic!("expected a nameless-group refusal for {what}, got {other:?}"),
        }
    }
}
