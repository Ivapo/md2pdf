//! Every sentence `md2pdf_core::Error` prints, one row per variant.
//!
//! **The failure this file exists to prevent is a message that changed while
//! nothing was watching.** `mpdf-008` gave every line-carrying variant a
//! `Location` so a message could name the file the author wrote it in, and a
//! document that names no section must still print, character for character, the
//! sentence it printed before that type existed. Forty-three assertions in
//! `golden_test.rs` reach these variants and every one of them destructures the
//! fields — none produces a `Display` string at all — so before this file the
//! repo's only byte-exact `Display` assertion was
//! `page_examples_test.rs:every_refusal_prints_the_sentence_beside_it`, which
//! covers three rows across two of the nine.
//!
//! So the nine are enumerated here by hand, twice: once with no file, which is
//! the inertness half, and once with one, which pins the `in FILE at line N`
//! phrasing in the other direction. **The list is the spec's table**
//! (`specs/multi_file_documents_spec.md` §2) written out a second time on
//! purpose — a test that derived its rows from the enum would agree with
//! whatever the enum said.
//!
//! The sentence a user sees is exactly this one: `cli/src/main.rs` prints it
//! after its `error: ` prefix, `web/src/lib.rs:render` hands it to the page
//! unchanged, and the app puts it in its error pane.

use md2pdf_core::{Asset, Error, Location, md_to_pdf};

/// A location in the document the caller handed in.
fn at(line: usize) -> Location {
    Location::at(line)
}

/// A location in a section file.
fn within(file: &str, line: usize) -> Location {
    Location {
        file: Some(file.to_string()),
        line,
    }
}

/// One of each of the nine line-carrying variants, located by `place`.
///
/// The two callers differ only in what they hand in here, so nothing but the
/// location can differ between the two sentences a variant prints.
fn every_variant(place: impl Fn(usize) -> Location) -> Vec<Error> {
    vec![
        Error::UnsupportedConstruct {
            construct: "raw HTML block".to_string(),
            location: place(3),
        },
        Error::Frontmatter {
            location: place(3),
            problem: "the key 'subtitle' is not one this dialect reads".to_string(),
        },
        Error::Math {
            location: place(1),
            problem: "unsupported command '\\includegraphics'".to_string(),
        },
        Error::Name {
            location: place(9),
            problem: "nothing declares the name 'fig:absent'".to_string(),
        },
        Error::Citation {
            location: place(8),
            problem: "'@nosuchkey' is cited and the bibliography does not hold it".to_string(),
        },
        Error::MissingImage {
            path: "figures/mark.svg".to_string(),
            location: place(5),
        },
        Error::MissingBibliography {
            path: "refs.yml".to_string(),
            location: place(3),
        },
        Error::MissingSection {
            path: "sections/method.md".to_string(),
            location: place(7),
        },
        Error::ImageFormat {
            path: "mark.svg".to_string(),
            location: place(5),
            format: "SVG".to_string(),
        },
    ]
}

/// With no file, every sentence is the one this crate has always printed.
///
/// This is the inertness half, and it is byte-exact rather than a `contains`:
/// a document written in one file is every document written before `mpdf-008`,
/// and not one of its messages may move.
#[test]
fn every_variant_prints_the_sentence_it_always_printed() {
    let expected = [
        "unsupported markdown construct 'raw HTML block' at line 3",
        "frontmatter error at line 3: the key 'subtitle' is not one this dialect reads",
        "math error at line 1: unsupported command '\\includegraphics'",
        "name error at line 9: nothing declares the name 'fig:absent'",
        "citation error at line 8: '@nosuchkey' is cited and the bibliography does not hold it",
        "no image file supplied for 'figures/mark.svg' at line 5",
        "no bibliography file supplied for 'refs.yml' at line 3",
        "no section file supplied for 'sections/method.md' at line 7",
        "image file 'mark.svg' at line 5 does not hold SVG data",
    ];

    let errors = every_variant(at);
    assert_eq!(errors.len(), expected.len(), "a variant lost its row");

    for (error, sentence) in errors.iter().zip(expected) {
        assert_eq!(error.to_string(), sentence);
    }
}

/// With a file, every sentence names it, and names it once.
///
/// **The two paths never collide, because only one of them is quoted.** An asset
/// is `'mark.svg'` and a source file is bare after `in`, so the four rows below
/// that carry both read once and correctly.
#[test]
fn every_variant_names_the_file_where_there_is_one() {
    let expected = [
        "unsupported markdown construct 'raw HTML block' in sections/method.md at line 3",
        "frontmatter error in sections/method.md at line 3: the key 'subtitle' is not one this dialect reads",
        "math error in sections/method.md at line 1: unsupported command '\\includegraphics'",
        "name error in sections/method.md at line 9: nothing declares the name 'fig:absent'",
        "citation error in sections/method.md at line 8: '@nosuchkey' is cited and the bibliography does not hold it",
        "no image file supplied for 'figures/mark.svg' in sections/method.md at line 5",
        "no bibliography file supplied for 'refs.yml' in sections/method.md at line 3",
        "no section file supplied for 'sections/method.md' in sections/method.md at line 7",
        "image file 'mark.svg' in sections/method.md at line 5 does not hold SVG data",
    ];

    let errors = every_variant(|line| within("sections/method.md", line));
    assert_eq!(errors.len(), expected.len(), "a variant lost its row");

    for (error, sentence) in errors.iter().zip(expected) {
        assert_eq!(error.to_string(), sentence);
    }
}

/// The four refusals this phase added, printed and *reached*.
///
/// Each row compiles a real master beside a real section and compares the whole
/// sentence, so the construct names are pinned to what the compiler says rather
/// than to a second copy of them. Nothing this phase added falls outside the
/// enumeration above; these are the sentences that enumeration cannot construct
/// on its own, because the location has to come from the join.
#[test]
fn every_section_refusal_prints_the_sentence_the_compiler_prints() {
    let master = "---\ntitle: A Report\n---\n\n[](sections/two.md)\n";

    for (bytes, sentence, what) in [
        (
            b"---\ntitle: Section two\n---\n\nText.\n".to_vec(),
            "unsupported markdown construct 'section with its own frontmatter' \
             in sections/two.md at line 1",
            "a section carrying frontmatter of its own",
        ),
        (
            b"Text.\n\nMore text.\n\n[](sections/three.md)\n".to_vec(),
            "unsupported markdown construct 'include inside an included section' \
             in sections/two.md at line 5",
            "a section including a section",
        ),
        (
            vec![0xff, 0xfe, b'T'],
            "unsupported markdown construct 'section that is not UTF-8 text' \
             in sections/two.md at line 1",
            "a section that is not text at all",
        ),
    ] {
        let sections = [Asset {
            path: "sections/two.md".to_string(),
            bytes,
        }];

        match md_to_pdf(master, &sections) {
            Err(error) => assert_eq!(error.to_string(), sentence, "for {what}"),
            Ok(_) => panic!("{what} compiled"),
        }
    }

    // The ninth variant, reached the way `web/src/lib.rs:render` would reach it:
    // `md_to_pdf` called directly with a fixed asset array and no wrapper in
    // front of it to catch a file it could not open.
    match md_to_pdf(master, &[]) {
        Err(error) => assert_eq!(
            error.to_string(),
            "no section file supplied for 'sections/two.md' at line 5"
        ),
        Ok(_) => panic!("a master with no sections compiled"),
    }
}
