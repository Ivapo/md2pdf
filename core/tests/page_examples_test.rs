//! The exit gate for `mpdf-006` Phase 1, grown by Phase 3: the demo page
//! cannot claim markdown the compiler refuses.
//!
//! `web/index.html` argues that this dialect is worth writing, and every claim
//! it makes is an example a reader can see. **The failure this file exists to
//! prevent is a page that shows markdown the compiler will not accept** — the
//! one most likely to happen and least likely to be noticed, because a snippet
//! typed into a landing page by hand is one edit away from a refusal and the
//! page would still look right.
//!
//! So the examples are not typed into prose. Each is one `data-example` element
//! in the page, and two consumers read that same element: the page itself, and
//! this file through `include_str!`. One copy of one thing.
//!
//! **The page's one image arrives the same way.** Phase 3 gave the page a file
//! of its own, inline in the same document, and every example here is compiled
//! with it in hand — ignored by the ten that name no image, load-bearing for the
//! one that does. Excluding that row from the gate was refused deliberately: a
//! row no test compiles is exactly the claim this file exists to prevent.
//!
//! **`web/` is deliberately not a workspace member** — its `Cargo.toml` carries
//! an empty `[workspace]` table so `cargo test --workspace` never acquires a
//! five-minute wasm build. Reading a file is not membership: `include_str!`
//! compiles the bytes in, the directory stays out of the build graph, and the
//! gate every phase already runs is what catches a page that lies.

use md2pdf_core::{Asset, md_to_pdf};

const PAGE: &str = include_str!("../../web/index.html");

/// How many `data-example` elements the page is expected to carry.
///
/// Asserted rather than counted, so an element that stops matching the marker —
/// a renamed attribute, a mangled tag — fails the suite instead of silently
/// leaving it and taking its claim with it.
const EXPECTED: usize = 11;

/// The opening of an example element. An example must carry **both** this and a
/// `data-example` attribute: the page will select on the attribute and this file
/// scans for the type, and a row matched by one but not the other would be
/// loaded by the page and checked by nothing.
const OPEN: &str = "<script type=\"text/markdown\"";

/// The opening of the element carrying the page's one image file.
///
/// A different type from the examples', and carrying no `data-example`
/// attribute — so the count above ignores it, and so does the page's
/// `script[data-example] { display: block }` override, which would otherwise
/// show a reader several lines of raw SVG.
const ASSET_OPEN: &str = "<script type=\"image/svg+xml\"";

struct Example<'a> {
    name: &'a str,
    expect: &'a str,
    content: &'a str,
}

/// Every example element in the page, in document order.
///
/// Plain string operations, and no HTML parser or JSON crate enters the
/// workspace for this: every marker is a fixed literal and the region ends at
/// the next closing tag. **A `<script>` holds raw text**, so scanning to
/// `</script` is exactly what an HTML parser would do here — markdown inside one
/// needs no escaping and a block of a non-JavaScript type is never executed.
///
/// The search resumes after each closing tag rather than sweeping the whole
/// file, so an example whose own body one day contains the marker cannot open a
/// second phantom element.
fn examples() -> Vec<Example<'static>> {
    let mut found = Vec::new();
    let mut rest = PAGE;

    while let Some(start) = rest.find(OPEN) {
        let element = &rest[start..];
        let open_end = element
            .find('>')
            .expect("an example element with no closing '>'");
        let close = element
            .find("</script")
            .expect("an example element the page never closes");

        let attributes = &element[..open_end];
        found.push(Example {
            name: attribute(attributes, "data-example"),
            expect: attribute(attributes, "data-expect"),
            content: &element[open_end + 1..close],
        });

        rest = &element[close..];
    }

    found
}

/// The one image file the page carries, as the compiler takes it.
///
/// **The name is read, not duplicated.** The element's attribute value *is* the
/// path handed to `md2pdf_core::Asset`, and the caption row's own
/// `![…](pipeline.svg)` must equal it. Nothing asserts that equality and nothing
/// needs to: a row naming a different file comes back `MissingImage` from
/// `core/src/lib.rs:collect` and fails `every_ok_example_compiles`.
fn asset() -> Asset {
    let start = PAGE
        .find(ASSET_OPEN)
        .expect("the page carries no asset element");
    let element = &PAGE[start..];
    let open_end = element
        .find('>')
        .expect("an asset element with no closing '>'");
    let close = element
        .find("</script")
        .expect("an asset element the page never closes");

    Asset {
        path: attribute(&element[..open_end], "data-asset").to_string(),
        bytes: element[open_end + 1..close].as_bytes().to_vec(),
    }
}

/// One double-quoted attribute value out of a tag's attribute region.
fn attribute<'a>(attributes: &'a str, name: &str) -> &'a str {
    let key = format!("{name}=\"");
    let start = attributes
        .find(&key)
        .unwrap_or_else(|| panic!("an example element with no `{name}` attribute"))
        + key.len();
    let value = &attributes[start..];
    let end = value
        .find('"')
        .unwrap_or_else(|| panic!("an unterminated `{name}` attribute"));
    &value[..end]
}

/// The visible sentence a refusal row prints, out of its `<code>` element.
///
/// **This scan does not rest on what the one above rests on**, and the
/// difference is worth knowing. A `<code>` holds *parsed* markup, so its raw
/// slice equals its rendered text only while the sentence inside needs no
/// character reference. That holds for every message this page carries, and
/// `no_expected_message_needs_escaping` below is what keeps it holding: a later
/// message carrying a `<` or an `&` fails the suite loudly rather than passing
/// wrongly.
fn expected_message(name: &str) -> &'static str {
    let key = format!("<code data-error-for=\"{name}\"");
    let start = PAGE
        .find(&key)
        .unwrap_or_else(|| panic!("no `data-error-for` element for the '{name}' example"));
    let element = &PAGE[start..];
    let open_end = element
        .find('>')
        .expect("a `data-error-for` element with no closing '>'");
    let close = element
        .find("</code")
        .expect("a `data-error-for` element the page never closes");

    &element[open_end + 1..close]
}

/// The page carries exactly the examples the phase says it does.
#[test]
fn the_page_carries_eleven_examples() {
    let found = examples();
    assert_eq!(
        found.len(),
        EXPECTED,
        "the page carries {} examples, not {EXPECTED}",
        found.len()
    );

    // The page selects on the attribute; this file scans for the type. A count
    // that disagrees means a row one of the two would miss.
    assert_eq!(
        PAGE.matches("data-example=\"").count(),
        EXPECTED,
        "the `data-example` attributes and the example elements disagree"
    );

    let mut names: Vec<&str> = found.iter().map(|example| example.name).collect();
    names.sort_unstable();
    let unique = names.len();
    names.dedup();
    assert_eq!(unique, names.len(), "two examples share a name: {names:?}");
}

/// Every example is marked, and marked with one of the two words the gate reads.
///
/// Without this an example whose attribute is absent or misspelt would be
/// skipped in silence while the count above still read ten — a claim on the page
/// that nothing compiles.
#[test]
fn every_example_says_what_it_expects() {
    for example in examples() {
        assert!(
            matches!(example.expect, "ok" | "error"),
            "the '{}' example expects '{}', which is neither 'ok' nor 'error'",
            example.name,
            example.expect
        );
    }
}

/// Each example's content is flush left, with no leading and no trailing newline.
///
/// **This is a rule about bytes and it is load-bearing, not tidiness.** An
/// indented example is a different document, and one that still compiles:
/// measured at two spaces of indent, the frontmatter example stops being
/// frontmatter and reaches the page as a setext heading over prose, and the
/// caption example keeps its table but emits `: The measurements.` as literal
/// text — and `md_to_pdf` returns `Ok` for both. A gate asking only "does it
/// compile" would pass while the page's own comparison column described what the
/// reader was looking at.
///
/// A single leading newline is the same hazard one line further on: it moves
/// every refusal's `at line N` by one, and the assertions below are keyed to
/// those numbers.
///
/// The rule needs no stripping step in either consumer, which is why it is
/// written this way rather than as "strip one leading newline": with nothing to
/// strip, `textContent` in the page and the slice taken here are the same bytes
/// by construction, and the two cannot drift apart by normalising differently.
#[test]
fn every_example_is_flush_left_and_unpadded() {
    for example in examples() {
        let name = example.name;
        let content = example.content;

        assert!(
            !content.starts_with(char::is_whitespace),
            "the '{name}' example opens with whitespace"
        );
        assert!(
            !content.starts_with('\n'),
            "the '{name}' example has a leading newline"
        );
        assert!(
            !content.ends_with('\n'),
            "the '{name}' example has a trailing newline"
        );

        for (number, line) in content.lines().enumerate() {
            assert!(
                !line.starts_with(' ') && !line.starts_with('\t'),
                "line {} of the '{name}' example is indented: {line:?}",
                number + 1
            );
        }
    }
}

/// Every example the page presents as accepted is one the compiler accepts.
///
/// Every one of them is handed the page's asset, because the page hands it to
/// every compile: `md_to_pdf` ignores an asset the document never names, and a
/// channel open to one row alone would be a page that draws its figure on a
/// click and refuses it on the reader's next keystroke.
#[test]
fn every_ok_example_compiles() {
    let assets = [asset()];
    for example in examples() {
        if example.expect != "ok" {
            continue;
        }
        let pdf = md_to_pdf(example.content, &assets).unwrap_or_else(|e| {
            panic!("the '{}' example is shown as accepted but fails: {e}", example.name)
        });
        assert!(pdf.starts_with(b"%PDF"), "the output is not a PDF");
        assert!(pdf.len() > 1000, "the PDF is suspiciously small");
    }
}

/// Every refusal row prints the sentence the compiler actually prints.
///
/// **The checked sentence is the one the reader sees**, taken from the row's own
/// visible `<code>` element rather than from a copy in an attribute. An
/// attribute would let this test prove agreement between the compiler and a
/// string nobody reads while the printed prose said something else, which voids
/// the only argument the page makes for carrying its refusals at all.
///
/// The equality is exact: `web/src/lib.rs:render` hands the error's `Display` to
/// the page unchanged, and `cli/src/main.rs` prints that same `Display` after
/// its `error: ` prefix, so the sentence in the page is the sentence at the
/// terminal.
#[test]
fn every_refusal_prints_the_sentence_beside_it() {
    let assets = [asset()];
    for example in examples() {
        if example.expect != "error" {
            continue;
        }
        let name = example.name;
        let error = md_to_pdf(example.content, &assets)
            .err()
            .unwrap_or_else(|| panic!("the '{name}' example is shown as refused but compiles"));

        assert_eq!(
            error.to_string(),
            expected_message(name),
            "the '{name}' row prints a sentence the compiler does not"
        );
    }
}

/// The page carries a message element for each refusal and no others.
///
/// A `<code data-error-for=…>` left behind after a row was renamed or removed
/// would never be read by the test above, and would sit on the page asserting a
/// sentence nothing checks.
#[test]
fn the_message_elements_match_the_refusals() {
    let mut refusals: Vec<&str> = examples()
        .into_iter()
        .filter(|example| example.expect == "error")
        .map(|example| example.name)
        .collect();
    refusals.sort_unstable();

    let mut declared: Vec<&str> = PAGE
        .match_indices("<code data-error-for=\"")
        .map(|(at, key)| {
            let value = &PAGE[at + key.len()..];
            &value[..value.find('"').expect("an unterminated `data-error-for`")]
        })
        .collect();
    declared.sort_unstable();

    assert_eq!(
        declared, refusals,
        "the message elements and the refusal examples name different rows"
    );
}

/// The page carries one image file, and it obeys the half of the byte rule it can.
///
/// One file, not several: the page selects the first such element and this file
/// scans for the first, so a second would be read by neither and would sit there
/// looking like a second channel. **No leading and no trailing newline**, for the
/// same reason the examples have none — `textContent` in the page and the slice
/// taken here are then the same bytes by construction. The indentation inside is
/// deliberately unconstrained: these bytes reach Typst's image loader rather than
/// a markdown parser whose parse depends on them.
#[test]
fn the_page_carries_one_asset() {
    assert_eq!(
        PAGE.matches("data-asset=\"").count(),
        1,
        "the page carries more than one asset element"
    );

    let asset = asset();
    assert!(!asset.path.is_empty(), "the asset element names no path");

    let content = std::str::from_utf8(&asset.bytes).expect("the asset is not UTF-8");
    assert!(
        !content.starts_with('\n'),
        "the asset element has a leading newline"
    );
    assert!(
        !content.ends_with('\n'),
        "the asset element has a trailing newline"
    );
}

/// No expected message needs a character reference to reach the page intact.
///
/// This is what makes the raw slice `expected_message` takes equal to the text a
/// reader sees. All three of the messages today are plain ASCII; one carrying a
/// `<` or an `&` would have to be written escaped in the HTML, and the raw slice
/// would then be a string the compiler never produced. The failure is caught
/// here, with its cause named, rather than as a baffling inequality above.
#[test]
fn no_expected_message_needs_escaping() {
    for example in examples() {
        if example.expect != "error" {
            continue;
        }
        let message = expected_message(example.name);
        for bad in ['<', '&', '\n'] {
            assert!(
                !message.contains(bad),
                "the '{}' message contains {bad:?}, which a raw slice cannot read back",
                example.name
            );
        }
    }
}
