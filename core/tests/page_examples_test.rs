//! The exit gate for `mpdf-006` Phase 1, grown by Phases 3 and 4: the demo page
//! cannot claim markdown the compiler refuses, and its other column cannot claim
//! markup the parser does not write.
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
//! **The page's two files arrive the same way.** Phase 3 gave the page an image
//! of its own, inline in the same document, and `mpdf-007` Phase 4 sent a
//! bibliography down the same attribute; every example here is compiled with
//! both in hand — ignored by the eleven that name no image and the eleven that
//! name no bibliography, load-bearing for the one apiece that does. Excluding
//! either row from the gate was refused deliberately: a row no test compiles is
//! exactly the claim this file exists to prevent.
//!
//! **The column beside each example is generated here, and checked here.** Every
//! `md2pdf` column has been compiled by this file since Phase 1; the column
//! opposite was prose nothing checked, and three of the eleven the page then
//! carried had drifted off the baseline it declares. Phase 4 replaced it with
//! `md2pdf_core::md_to_html` over the same source — one parse, one set of
//! options, pulldown-cmark's own writer instead of the emitter — stored in the
//! page between `<!--html:NAME-->` markers. **The generator is this test**:
//! `generated` below produces a row's block, `every_generated_block_is_the_parsers_own_html`
//! compares it against what the page holds, and `bless_the_generated_blocks`
//! writes it. A generator program of its own would implement the substitution
//! twice, which is the drift the whole arrangement exists to prevent.
//!
//! **`web/` is deliberately not a workspace member** — its `Cargo.toml` carries
//! an empty `[workspace]` table so `cargo test --workspace` never acquires a
//! five-minute wasm build. Reading a file is not membership: `include_str!`
//! compiles the bytes in, the directory stays out of the build graph, and the
//! gate every phase already runs is what catches a page that lies.

use md2pdf_core::{Asset, md_to_html, md_to_pdf};

const PAGE: &str = include_str!("../../web/index.html");

/// How many `data-example` elements the page is expected to carry.
///
/// Asserted rather than counted, so an element that stops matching the marker —
/// a renamed attribute, a mangled tag — fails the suite instead of silently
/// leaving it and taking its claim with it.
const EXPECTED: usize = 12;

/// The opening of an example element. An example must carry **both** this and a
/// `data-example` attribute: the page will select on the attribute and this file
/// scans for the type, and a row matched by one but not the other would be
/// loaded by the page and checked by nothing.
const OPEN: &str = "<script type=\"text/markdown\"";

/// How many `data-asset` elements the page is expected to carry.
///
/// **This one moves against [`EXPECTED`] rather than with it**, and is written
/// separately for that reason: it is a count of the page's own files, not of its
/// examples, so a row added or removed leaves it alone and a file added or
/// removed leaves the other alone.
const ASSETS: usize = 2;

/// The opening of the element carrying the page's image file.
///
/// A different type from the examples', and carrying no `data-example`
/// attribute — so the count above ignores it, and so does the page's
/// `script[data-example] { display: block }` override, which would otherwise
/// show a reader several lines of raw SVG.
const IMAGE_OPEN: &str = "<script type=\"image/svg+xml\"";

/// The opening of the element carrying the page's bibliography file.
///
/// **One attribute, two elements, told apart by their `type`.** `data-asset`'s
/// value *is* the path `md2pdf_core::Asset` is given, which is as true of a
/// `.yml` as of an `.svg`, so a second attribute would be a second mechanism for
/// something the page already has one word for. The type is what the page's own
/// module selects on and what this file scans for, and it is also what keeps the
/// `data:` URI substitution below keyed to the image alone.
const BIBLIOGRAPHY_OPEN: &str = "<script type=\"application/yaml\"";

/// What a generated block may never contain.
///
/// The first three would hand this file's own scans a phantom element to find —
/// a second `<script type="text/markdown"`, a thirteenth `data-example="`, a
/// third `data-asset="` — and `<script` would also be executable content reaching the
/// page out of an example. The fourth would break the delimiter the block is
/// found by: an HTML comment cannot nest, so a `<!--` inside one ends it early.
const REFUSED: [&str; 4] = ["<script", "data-example=\"", "data-asset=\"", "<!--"];

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

/// Both files the page carries, as the compiler takes them.
///
/// The page hands every compile both, so this file does too: `md_to_pdf` ignores
/// an asset the document never names, and a channel open to one row alone would
/// be a page that draws its figure on a click and refuses it on the reader's
/// next keystroke.
fn assets() -> [Asset; 2] {
    [image(), bibliography()]
}

/// The image file the page carries, as the compiler takes it.
///
/// **The name is read, not duplicated.** The element's attribute value *is* the
/// path handed to `md2pdf_core::Asset`, and the caption row's own
/// `![…](pipeline.svg)` must equal it. Nothing asserts that equality and nothing
/// needs to: a row naming a different file comes back `MissingImage` from
/// `core/src/lib.rs:collect` and fails `every_ok_example_compiles`.
///
/// It is named apart from [`bibliography`] because the `data:` URI substitution
/// below is the image's alone — a bibliography is named in the frontmatter and
/// never as an image destination, so generalizing that rule would add a branch
/// nothing takes.
fn image() -> Asset {
    named(IMAGE_OPEN)
}

/// The bibliography file the page carries, as the compiler takes it.
///
/// Same reading as [`image`] and for the same reason: the citation row's own
/// `bibliography: refs.yml` must equal this element's attribute, and a row naming
/// a different file comes back `MissingBibliography` from
/// `core/src/lib.rs:collect` rather than needing an assertion of its own.
fn bibliography() -> Asset {
    named(BIBLIOGRAPHY_OPEN)
}

/// One `data-asset` element, as the compiler takes it.
fn named(open: &str) -> Asset {
    let (attributes, content) = asset_element(open);

    Asset {
        path: attribute(attributes, "data-asset").to_string(),
        bytes: content.as_bytes().to_vec(),
    }
}

/// One asset element, as its attribute region and its content.
///
/// **The `type` is what picks it out**, not document order — the page's own
/// module selects on the same attribute pair, so neither reader depends on which
/// of the two elements happens to come first. Two readers want different halves
/// of the same element — the compiler wants the path and the bytes, the generated
/// `data:` URI wants the media type — and one scan serves both rather than each
/// finding the element for itself.
fn asset_element(open: &str) -> (&'static str, &'static str) {
    let start = PAGE
        .find(open)
        .unwrap_or_else(|| panic!("the page carries no `{open}` asset element"));
    let element = &PAGE[start..];
    let open_end = element
        .find('>')
        .expect("an asset element with no closing '>'");
    let close = element
        .find("</script")
        .expect("an asset element the page never closes");

    (&element[..open_end], &element[open_end + 1..close])
}

/// The media type the page declares for its image.
///
/// Read off the element rather than spelled a second time here: it is what the
/// `data:` URI below announces, and a URI announcing a type the page does not
/// carry would be this file asserting something the page never said.
fn asset_media() -> &'static str {
    attribute(asset_element(IMAGE_OPEN).0, "type")
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

/// The bytes the page stores as one row's generated column.
///
/// **This region is the one place in this file that cannot end at a closing
/// tag**, and every other scan above rests on that convention. The `raw-html`
/// block ends `<div>a raw HTML block</div>` and the `footnote` block carries a
/// `<div class="footnote-definition">`, so a wrapper closed by `</div>` is
/// mis-delimited by the very rule that bought the plain string scan — and
/// counting opens against closes is an HTML parser under another name, which the
/// same paragraph refuses.
///
/// So the block sits between a pair of comments keyed to the row's own
/// `data-example` value: a comment cannot nest, `push_html` never emits one, and
/// the name is not written a third time. **What lies between them is exactly
/// what `generated` returned** — nothing trimmed at either end, which is what
/// lets the page and this file compare the same bytes by construction. A wrapper
/// element for styling may sit outside the markers, where it is not compared.
fn stored_html(name: &str) -> &'static str {
    let open = open_marker(name);
    let close = close_marker(name);

    let start = PAGE
        .find(&open)
        .unwrap_or_else(|| panic!("no generated block for the '{name}' example"))
        + open.len();
    let end = PAGE[start..]
        .find(&close)
        .unwrap_or_else(|| panic!("the '{name}' generated block is never closed"))
        + start;

    &PAGE[start..end]
}

fn open_marker(name: &str) -> String {
    format!("<!--html:{name}-->")
}

fn close_marker(name: &str) -> String {
    format!("<!--/html:{name}-->")
}

/// One row's other column: the parser's own HTML over that row's source.
///
/// **The generator is this test.** One function produces a block, the assertion
/// below compares it against the page, and the blessing mode writes it — because
/// a generator program of its own would implement the substitution twice, and two
/// implementations of one rule are the drift this whole arrangement prevents.
///
/// **One substitution, and exactly one.** An image destination equal to the
/// page's asset name becomes a `data:` URI over those same bytes. The file is
/// carried inline in the page rather than published beside it — `pages.yml`
/// assembles the site from `web/index.html` and `web/pkg/` alone — so a verbatim
/// destination would render a broken image on the published page while every
/// local server showed it working: the comparison column lying about what
/// markdown can do. Everything else is `md_to_html`'s output, byte for byte.
fn generated(source: &str, asset: &Asset) -> String {
    md_to_html(source).replace(
        &destination(asset),
        &format!(
            "src=\"data:{},{}\"",
            asset_media(),
            percent_encode(&asset.bytes)
        ),
    )
}

/// The `src` attribute `md_to_html` writes for the page's own image.
fn destination(asset: &Asset) -> String {
    format!("src=\"{}\"", asset.path)
}

/// Percent-encode over an explicit set: every byte outside ASCII letters, digits
/// and `-._~`.
///
/// **The set is named because the reflex is broken and the break is invisible to
/// the assertion below.** Measured 2026-08-22: `pulldown_cmark`'s own
/// `escape_href` leaves `#` unencoded, and the page's SVG carries
/// `stroke="#1e3c82"` — so a raw `data:image/svg+xml,<svg…>` truncates at the
/// fragment and renders nothing, while an equality check agrees with itself
/// about the broken bytes. Only the browser half of the gate can see that, which
/// is why it is told to look for the diagram rather than for a block.
fn percent_encode(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len());
    for &byte in bytes {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

/// The page carries exactly the examples the phase says it does.
#[test]
fn the_page_carries_twelve_examples() {
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
/// skipped in silence while the count above still read eleven — a claim on the page
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
/// Every one of them is handed **both** the page's files, because the page hands
/// both to every compile: `md_to_pdf` ignores an asset the document never names,
/// and a channel open to one row alone would be a page that draws its figure on
/// a click and refuses it on the reader's next keystroke.
#[test]
fn every_ok_example_compiles() {
    let assets = assets();
    for example in examples() {
        if example.expect != "ok" {
            continue;
        }
        let pdf = md_to_pdf(example.content, &assets).unwrap_or_else(|e| {
            panic!(
                "the '{}' example is shown as accepted but fails: {e}",
                example.name
            )
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
    let assets = assets();
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

/// The page carries its two files, and each obeys the half of the byte rule it can.
///
/// **Two, and told apart by their `type`.** The page and this file both select on
/// the type rather than taking the first `data-asset` element, so neither is
/// keyed to document order — but the count is still asserted, because a third
/// element under this attribute would be read by neither and would sit there
/// looking like a channel. **No leading and no trailing newline**, for the same
/// reason the examples have none: `textContent` in the page and the slice taken
/// here are then the same bytes by construction.
///
/// What is *not* asserted is indentation, and the two files differ on why. The
/// SVG's is free — those bytes reach Typst's image loader rather than a parser
/// whose reading depends on them. The bibliography's is load-bearing and reaches
/// a YAML reader that does care, but a wrong indent there is not a silent pass:
/// it is a parse failure `core/src/bibliography.rs:keys` names, or a key the
/// citation row cannot resolve, and `every_ok_example_compiles` fails on either.
#[test]
fn the_page_carries_two_assets() {
    assert_eq!(
        PAGE.matches("data-asset=\"").count(),
        ASSETS,
        "the page carries a number of asset elements other than {ASSETS}"
    );

    for asset in assets() {
        assert!(!asset.path.is_empty(), "an asset element names no path");

        let content = std::str::from_utf8(&asset.bytes)
            .unwrap_or_else(|_| panic!("the '{}' asset is not UTF-8", asset.path));
        assert!(
            !content.starts_with('\n'),
            "the '{}' asset element has a leading newline",
            asset.path
        );
        assert!(
            !content.ends_with('\n'),
            "the '{}' asset element has a trailing newline",
            asset.path
        );
    }
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

/// Every example carries one generated block, delimited by its own name.
///
/// The counts are asserted the way the example count is: a marker that stops
/// matching — a renamed row, a mangled comment — fails the suite rather than
/// silently leaving a column nothing checks, which is the state Phase 4 found the
/// page in.
#[test]
fn every_example_carries_one_generated_block() {
    assert_eq!(
        PAGE.matches("<!--html:").count(),
        EXPECTED,
        "the page opens a number of generated blocks other than {EXPECTED}"
    );
    assert_eq!(
        PAGE.matches("<!--/html:").count(),
        EXPECTED,
        "the page closes a number of generated blocks other than {EXPECTED}"
    );

    for example in examples() {
        // A name reaching an HTML comment must hold no `--`, which closes one.
        assert!(
            !example.name.contains("--"),
            "the '{}' example's name cannot key an HTML comment",
            example.name
        );
        // Panics with the row named if either marker is missing.
        stored_html(example.name);
    }
}

/// The other column is what the parser writes for that row's own source.
///
/// **This is the assertion the column never had.** Every `md2pdf` column has been
/// compiled since Phase 1 and the one beside it was prose — and measured through
/// `md_to_html`, three of the eleven the page then carried asserted behaviour
/// these options contradict:
/// two described the dollars printing as literal text, which `ENABLE_MATH`
/// refuses, and one promised a rule this backend draws no `<hr>` for.
#[test]
fn every_generated_block_is_the_parsers_own_html() {
    let image = image();
    for example in examples() {
        assert_eq!(
            stored_html(example.name),
            generated(example.content, &image),
            "the '{}' row's other column is not what the parser writes for its source \u{2014} \
             regenerate with `cargo test -p md2pdf-core --test page_examples_test -- \
             --ignored bless`",
            example.name
        );
    }
}

/// The page's image is named once across the twelve outputs, and inlined there.
///
/// **The assertion above cannot fail on this.** A `data:` URI encoded wrongly
/// leaves the page and the generator agreeing about bytes that render nothing, so
/// what is checkable here is the other half: that the substitution has exactly one
/// site, and that no raw destination survives it. A row that stopped naming the
/// page's file, or a second that started, is caught here rather than by a reader
/// meeting a broken image on the published page.
#[test]
fn the_assets_destination_is_inlined_exactly_once() {
    let image = image();
    let raw: usize = examples()
        .iter()
        .map(|example| {
            md_to_html(example.content)
                .matches(&destination(&image))
                .count()
        })
        .sum();
    assert_eq!(
        raw, 1,
        "the page's image is named {raw} times across the generated columns, not once"
    );

    for example in examples() {
        assert!(
            !generated(example.content, &image).contains(&destination(&image)),
            "the '{}' row's column still names the image file rather than carrying it",
            example.name
        );
    }
}

/// No generated block carries a marker this file or the page scans for.
#[test]
fn no_generated_block_carries_a_marker_of_its_own() {
    let image = image();
    for example in examples() {
        let block = generated(example.content, &image);
        for refused in REFUSED {
            assert!(
                !block.contains(refused),
                "the '{}' row's generated column contains {refused:?}",
                example.name
            );
        }
    }
}

/// Write the twelve generated blocks into the page.
///
/// **Not part of the gate** — `#[ignore]` keeps it out of the
/// `cargo test --workspace` every phase runs — and the constraint it satisfies is
/// that a human must be able to produce the blocks without hand-writing them. It
/// fills the region between each row's markers, which are what the page carries
/// before there is anything to put between them.
///
/// Run it, then run the suite: `include_str!` compiled the old page in, so the
/// assertion above reads the new one only after a rebuild, which changing the
/// file is what triggers.
///
/// ```console
/// $ cargo test -p md2pdf-core --test page_examples_test -- --ignored bless
/// $ cargo test --workspace
/// ```
#[test]
#[ignore = "rewrites web/index.html; run it deliberately, then run the suite"]
fn bless_the_generated_blocks() {
    let image = image();
    let mut page = PAGE.to_string();

    for example in examples() {
        let open = open_marker(example.name);
        let close = close_marker(example.name);

        let start = page
            .find(&open)
            .unwrap_or_else(|| panic!("no generated block for the '{}' example", example.name))
            + open.len();
        let end = page[start..]
            .find(&close)
            .unwrap_or_else(|| panic!("the '{}' generated block is never closed", example.name))
            + start;

        page.replace_range(start..end, &generated(example.content, &image));
    }

    std::fs::write(SOURCE, page).expect("could not write the page");
}

/// The page on disk, which `PAGE` above is a compiled-in copy of.
const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../web/index.html");
