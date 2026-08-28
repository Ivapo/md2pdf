//! The two fixtures `mpdf-009` Phase 5 measures against, and the one test that
//! pins what wrote them.
//!
//! Phase 5 caps what the desktop pane retains. Its gate is read by a person in
//! the Web Inspector — `mpdf-003` OQ-10 records that nothing in this repository
//! reaches `app/dist/index.html` — and every literal that gate names is keyed to
//! a document of an exact length: **71 pages** for `long.md`, **20** for
//! `near.md`, the second sitting just under the budget at the app's own default
//! pane so that a widen crosses it.
//!
//! **So the length is the fixture's whole point, and nothing else here checks
//! it.** A document that drifted to 70 pages would leave the gate reading 17.5
//! MiB against a band of three and 414 MiB against a whole of 71, both quietly
//! wrong, and the phase would look like it passed.
//!
//! **The generator writes them and this file pins them**, which is the shape
//! `page_examples_test.rs` already uses for `web/index.html`: the generator is
//! `#[ignore]`d so `cargo test --workspace` never runs it, and the check reads a
//! compiled-in copy, so a rebuild is what makes it see new bytes.
//!
//! **Both fixtures are single-file by construction** — they name no section, so
//! the app shows no `Sections` panel and the pane measures the 520 px every
//! retention literal in the gate is derived from. A fixture written as a master
//! and its parts would sit at some 305 px instead and void all of them.
//!
//! **The cost is accepted here rather than discovered later.** `long.md` is a
//! 62,000-word document and compiling it takes some 7 s in a debug build, which
//! is what `cargo test --workspace` is. Phase 5's spec says so in as many words;
//! it is the price of a gate whose numbers are true.

use md2pdf_core::md_to_pdf;

/// The long document: 71 pages, and the one this phase is about.
const LONG_MD: &str = include_str!("../../tests/fixtures/long.md");

/// The document that sits *near* the budget rather than over it.
///
/// **It is the fixture that proves the budget is re-evaluated**, which no
/// single-length document can. At the default 520 px pane its 20 pages cost
/// 116.64 MiB and the pane draws them whole; widen to 700 and the same document
/// costs 211.5 MiB and the pane must cross to holding only what the reader is
/// near. An implementation that decides once per open passes every clause
/// `long.md` can reach and fails here.
const NEAR_MD: &str = include_str!("../../tests/fixtures/near.md");

/// A4, in PDF points — `core/assets/template.typ` sets `paper: "a4"`.
///
/// Spelled out rather than read from the `/MediaBox`, because the fraction
/// asserted below is only meaningful against the page size the look chose, and
/// a fixture that silently changed paper should move the literal rather than
/// take the new height along with it.
const PAGE_HEIGHT: f64 = 841.8898;

// -- what the fixtures must be ----------------------------------------------

/// The lengths, and the destination the long document's one reference resolves
/// to.
///
/// **The literals are spelled out and not computed.** A helper that derived them
/// would derive them the way the generator does and agree with a wrong answer —
/// the same argument `each_heading_is_anchored_at_its_own_line` makes in
/// `golden_test.rs`, and it is sharper here, because the generator's whole job
/// is to hit these numbers.
///
/// **Phase 5's gate clause 8 reads its page and fraction off this test**, which
/// is why the failure message prints them: the spec cannot supply them, since
/// they are a property of the document the generator happened to write.
#[test]
fn the_fixtures_are_the_lengths_phase_5_measures_against() {
    let long = md_to_pdf(LONG_MD, &[]).unwrap();
    assert!(long.starts_with(b"%PDF"), "the output is not a PDF");
    assert_eq!(page_count(&long), 71, "long.md is no longer 71 pages");

    let near = md_to_pdf(NEAR_MD, &[]).unwrap();
    assert!(near.starts_with(b"%PDF"), "the output is not a PDF");
    assert_eq!(page_count(&near), 20, "near.md is no longer 20 pages");

    let (page, top) = link_destination(&long);
    let fraction = 1.0 - top / PAGE_HEIGHT;
    assert_eq!(
        page, 64,
        "the long cross-reference now lands on page {page} at {top} pt, \
         which is fraction {fraction:.3}"
    );
    assert!(
        (fraction - 0.620).abs() < 0.01,
        "the long cross-reference now lands at fraction {fraction:.3} of page {page}, \
         {top} pt up an {PAGE_HEIGHT} pt page"
    );
}

// -- reading the compiled bytes ----------------------------------------------
//
// The first byte-structure reader in this repository, and it uses `std` alone.
// It can, because `typst-pdf` compiles under `PdfOptions::default()`, whose
// `pretty` is false: only content streams are deflated, there are no object
// streams anywhere in krilla, and every dictionary is literal ASCII in the file.

/// How many pages the compiled document has.
///
/// **Keyed to `/Type/Pages` rather than to `/Count`**, and the distinction is
/// the whole reliability of this function: an outline node carries a `/Count`
/// too, so the first `/Count` in the file is not necessarily the page tree's.
/// There is exactly one `/Type/Pages` node — krilla writes the page tree flat —
/// and that is asserted rather than assumed.
fn page_count(pdf: &[u8]) -> usize {
    let at = only(pdf, b"/Type/Pages");
    let count = find(pdf, b"/Count ", at).expect("the page tree names no /Count");
    integer(pdf, count + b"/Count ".len()).0 as usize
}

/// The page objects, in the order the page tree kids them.
///
/// A destination names its page by object reference, so this is what turns that
/// reference back into a page number a reader would count to.
fn kids(pdf: &[u8]) -> Vec<u64> {
    let at = only(pdf, b"/Type/Pages");
    let open = find(pdf, b"/Kids[", at).expect("the page tree names no /Kids") + b"/Kids[".len();
    let close = find(pdf, b"]", open).expect("the /Kids array is never closed");

    let mut kids = Vec::new();
    let mut i = open;
    while i < close {
        let (number, next) = integer(pdf, i);
        kids.push(number);
        // Past this reference's own `0 R`, which is two more tokens.
        i = find(pdf, b"R", next).expect("a /Kids entry is not a reference") + 1;
    }
    kids
}

/// Where the document's one internal link points: its 1-based page, and how far
/// up that page in the page's own units.
///
/// **Keyed to `/Subtype/Link`, and that is not interchangeable with "the only
/// `/XYZ` in the file".** Every heading becomes an outline entry carrying an XYZ
/// destination of its own, so a 71-page document with 180 headings holds 180 of
/// them; the link annotations are the ones a reader can click. The generated
/// fixture carries no footnote, no citation and no other link, so there is
/// exactly one — asserted, because a second would make "the" destination a lie.
fn link_destination(pdf: &[u8]) -> (usize, f64) {
    let link = only(pdf, b"/Subtype/Link");
    let dest = find(pdf, b"/Dest ", link).expect("the link carries no /Dest");
    let (object, _) = integer(pdf, dest + b"/Dest ".len());

    // The destination is an indirect object of its own: `[<page> 0 R/XYZ x y z]`,
    // the coordinate already flipped into PDF space and so measured up from the
    // page's bottom edge.
    let header = format!("\n{object} 0 obj\n");
    let at = find(pdf, header.as_bytes(), 0).expect("the destination object is not in the file")
        + header.len();
    let open = find(pdf, b"[", at).expect("the destination is not an array") + 1;
    let (page, after) = integer(pdf, open);
    let xyz = find(pdf, b"/XYZ ", after).expect("the destination is not an /XYZ");
    let (_, after_x) = number(pdf, xyz + b"/XYZ ".len());
    let (top, _) = number(pdf, after_x);

    let index = kids(pdf)
        .iter()
        .position(|&kid| kid == page)
        .expect("the destination names a page the page tree does not kid");
    (index + 1, top)
}

/// Where `needle` sits, having checked it sits in exactly one place.
fn only(hay: &[u8], needle: &[u8]) -> usize {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(at) = find(hay, needle, from) {
        found.push(at);
        from = at + 1;
    }
    assert_eq!(
        found.len(),
        1,
        "{} occurs {} times in the compiled document",
        String::from_utf8_lossy(needle),
        found.len()
    );
    found[0]
}

fn find(hay: &[u8], needle: &[u8], from: usize) -> Option<usize> {
    if from >= hay.len() {
        return None;
    }
    hay[from..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map(|at| at + from)
}

/// The unsigned integer at `at`, and where it ends. Leading spaces are skipped.
fn integer(hay: &[u8], at: usize) -> (u64, usize) {
    let (value, end) = number(hay, at);
    (value as u64, end)
}

/// The number at `at`, and where it ends.
fn number(hay: &[u8], at: usize) -> (f64, usize) {
    let mut start = at;
    while hay.get(start) == Some(&b' ') {
        start += 1;
    }
    let mut end = start;
    while matches!(
        hay.get(end),
        Some(b'0'..=b'9') | Some(b'.') | Some(b'-') | Some(b'+')
    ) {
        end += 1;
    }
    let text = std::str::from_utf8(&hay[start..end]).expect("a number is not ASCII");
    (
        text.parse()
            .unwrap_or_else(|_| panic!("'{text}' is not a number")),
        end,
    )
}

// -- the generator -----------------------------------------------------------

/// Where the two fixtures live. The repo-root `tests/fixtures/`, because that is
/// where every other fixture lives and the CLI tests read the same tree.
const LONG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/long.md");
const NEAR_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/near.md");

/// Write the two fixtures.
///
/// **Not part of the gate** — `#[ignore]` keeps it out of the
/// `cargo test --workspace` every phase runs — and the constraint it satisfies
/// is that a human must be able to produce a document of an exact page count
/// without hand-tuning one. It compiles as it goes and grows or shrinks the text
/// until the count is the one asked for, so the fixtures are *reproducible*
/// rather than lucky.
///
/// Run it in release. It compiles the long document a dozen or more times on the
/// way in, and Typst is some 30× slower unoptimised:
///
/// ```console
/// $ cargo test --release -p md2pdf-core --test long_document_test -- --ignored write
/// $ cargo test --workspace
/// ```
///
/// Then run the suite: `include_str!` compiled the old fixtures in, so the test
/// above reads the new ones only after a rebuild, which changing the files is
/// what triggers.
#[test]
#[ignore = "rewrites tests/fixtures/long.md and near.md; run it deliberately, then run the suite"]
fn write_the_long_and_near_fixtures() {
    let long = grow(71, true);
    std::fs::write(LONG_PATH, &long).expect("could not write tests/fixtures/long.md");

    let near = grow(20, false);
    std::fs::write(NEAR_PATH, &near).expect("could not write tests/fixtures/near.md");

    let (page, top) = link_destination(&md_to_pdf(&long, &[]).unwrap());
    println!(
        "long.md: 71 pages, its reference lands on page {page} at fraction {:.3}",
        1.0 - top / PAGE_HEIGHT
    );
}

/// A document of exactly `pages` pages.
///
/// **The search is proportional and then linear**, which is what makes it cheap
/// and what makes it terminate. A page holds some ten of these paragraphs, so
/// the first estimate is off by a long way and a proportional jump closes most
/// of it in three or four compiles; the last page is then filled one paragraph
/// at a time, which cannot overshoot because one paragraph is far smaller than
/// one page. A bisection would be tidier and would compile more.
fn grow(pages: usize, referable: bool) -> String {
    let mut paragraphs = pages * 11;

    for _ in 0..64 {
        let text = document(paragraphs, referable);
        let got = page_count(&md_to_pdf(&text, &[]).unwrap());
        if got == pages {
            return text;
        }

        let short = pages as isize - got as isize;
        paragraphs = if short.abs() > 1 {
            // A jump, sized by what this document is actually costing per page.
            let per = (paragraphs as f64 / got as f64).max(1.0);
            (paragraphs as isize + (short as f64 * per).round() as isize).max(1) as usize
        } else {
            (paragraphs as isize + short.signum()).max(1) as usize
        };
    }

    panic!("no document of {pages} pages was reached in 64 compiles");
}

/// The markdown itself: frontmatter, then `paragraphs` paragraphs under headings
/// that fall at fixed intervals through them.
///
/// **The prose is filler and says so.** Nothing about the dialect is being
/// tested here — `golden_test.rs` covers all of that against fixtures small
/// enough to read — and salting this one with figures, footnotes and citations
/// would make its page count move whenever the emitter changed, breaking a test
/// that has nothing to do with the emitter.
///
/// **`referable` is the long document's one cross-reference**, and it is the
/// only link either fixture carries. Phase 5's gate follows it into a page the
/// pane has not drawn, so it wants a target as far from the reference as the
/// document allows — the reference sits in the opening paragraph and the table
/// it names sits nine tenths of the way down. A table takes a name with no asset
/// bytes at all, which is how a single-file fixture carries a referable target
/// without an image beside it.
fn document(paragraphs: usize, referable: bool) -> String {
    let mut out = String::from(if referable {
        "---\ntitle: A Document At Thesis Length\n"
    } else {
        "---\ntitle: A Document Near The Budget\n"
    });
    out.push_str("author: Iva Po\ndate: 26 August 2026\ntemplate: article\ncolumns: 1\n---\n");

    let mut words = Lcg(0x20_26_08_26);
    let table_at = (paragraphs * 9 / 10).max(1);

    for i in 0..paragraphs {
        if i % 9 == 0 {
            out.push_str(&format!("\n# Chapter {}\n", i / 9 + 1));
        }
        if i % 3 == 0 {
            out.push_str(&format!("\n## Section {}.{}\n", i / 9 + 1, i % 9 / 3 + 1));
        }
        out.push('\n');
        out.push_str(&paragraph(&mut words));
        out.push('\n');

        if referable && i == 0 {
            out.push_str("\nThe table at [](#tab:late) sets the rest of it out.\n");
        }
        if referable && i == table_at {
            out.push_str(
                "\n| Construct | Counter |\n\
                 | --------- | :-----: |\n\
                 | a table   | its own |\n\
                 \n\
                 : The table this document names from its very first page. {#tab:late}\n",
            );
        }
    }

    out
}

/// One paragraph of between four and eight sentences.
fn paragraph(words: &mut Lcg) -> String {
    let sentences = 4 + words.upto(5);
    let mut out = String::new();

    for s in 0..sentences {
        if s > 0 {
            out.push(' ');
        }
        let length = 8 + words.upto(17);
        for w in 0..length {
            if w > 0 {
                out.push(' ');
            }
            let word = WORDS[words.upto(WORDS.len())];
            if w == 0 {
                out.push(word.as_bytes()[0].to_ascii_uppercase() as char);
                out.push_str(&word[1..]);
            } else {
                out.push_str(word);
            }
        }
        out.push('.');
    }

    out
}

/// The whole vocabulary: function words only, so no line of it reads as a claim
/// about anything.
const WORDS: [&str; 48] = [
    "a", "about", "across", "after", "against", "all", "also", "an", "and", "are", "as", "at",
    "be", "because", "been", "before", "between", "both", "but", "by", "can", "during", "each",
    "for", "from", "has", "have", "in", "into", "is", "it", "may", "more", "not", "of", "on",
    "one", "or", "other", "over", "some", "such", "than", "that", "the", "their", "then", "there",
];

/// A linear congruential generator, so the fixtures are the same bytes on every
/// machine and no dependency is added for one of them.
struct Lcg(u64);

impl Lcg {
    fn upto(&mut self, bound: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) % bound as u64) as usize
    }
}
