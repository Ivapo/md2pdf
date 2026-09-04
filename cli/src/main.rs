//! `md2pdf` — convert one markdown file into one typeset PDF.
//!
//! This binary owns all file I/O and all terminal output. The core crate owns
//! the pipeline and touches neither, which is what keeps it portable.

use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, ValueEnum};

/// Which of the two forms of `--licenses` was asked for.
///
/// One flag rather than two, because the distinction is one clap renders on a
/// single `--help` line in the value form, and because one arm in `run`
/// matching on this does what two booleans would have needed two arms for.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
// No doc comment on either variant: `clap_derive` turns one into a per-value
// description and expands `--help` from the one line the flag is pinned at into
// a `Possible values:` block. What each value means is the flag's own sentence.
enum Licenses {
    Notice,
    Full,
}

#[derive(Parser)]
#[command(
    name = "md2pdf",
    version,
    about = "Convert one markdown file into one typeset PDF."
)]
struct Args {
    /// The markdown file to convert.
    //
    // The `Option` and the attribute do different jobs and both are needed.
    // The `Option` is what lets the field hold "absent": `clap_derive` infers
    // `required(true)` from a non-`Option` field, which collides with
    // `required_unless_present` and panics a debug build on every invocation.
    // The attribute is what keeps clap enforcing the requirement, so a bare
    // `md2pdf` still exits on clap's own message rather than on one written
    // here.
    #[arg(required_unless_present = "licenses")]
    input: Option<PathBuf>,

    /// Where to write the PDF. The default is the input path with a .pdf
    /// extension. This option has no effect with --emit-typst.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Print the generated Typst source instead of compiling a PDF.
    #[arg(long = "emit-typst")]
    emit_typst: bool,

    /// Print what this binary carries and under what terms, then exit;
    /// --licenses=full prints the full licence texts instead.
    //
    // `require_equals` is the load-bearing attribute. Without it clap takes the
    // next bare token as the value, so `--licenses extra.md` — which returns 0
    // with the positional ignored — would become exit 2 on an invalid value.
    // The cost is named: `--licenses full` with a space prints the notice and
    // ignores `full`, exactly as it ignores `extra.md`. The notice writes the
    // form with the equals sign, so a reader who follows it types the right
    // thing.
    #[arg(
        long = "licenses",
        value_enum,
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "notice"
    )]
    licenses: Option<Licenses>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse();

    // Before anything else, and before any file is named: the point of this
    // flag is that a binary carried away from the repository it was built in
    // can still state its terms, so it must not need a document, a readable
    // file or a particular working directory. A positional given beside it is
    // ignored, as `--emit-typst` ignores `-o`.
    if let Some(which) = args.licenses {
        match which {
            Licenses::Notice => print!("{NOTICE}"),
            Licenses::Full => print!("{}", licenses()),
        }
        std::io::stdout()
            .flush()
            .map_err(|e| format!("cannot write to stdout: {e}"))?;
        return Ok(());
    }

    // Clap enforces this. The field is optional only so that `--licenses` can
    // run without it, and `required_unless_present` refuses every other
    // invocation that omits it — with clap's own message, before this runs.
    let input = args
        .input
        .expect("clap requires an input unless --licenses");

    let markdown = std::fs::read_to_string(&input)
        .map_err(|e| format!("cannot read {}: {e}", input.display()))?;

    // The sections come first on every path, `--emit-typst` included: a master
    // is not a document until they are joined in, so nothing downstream can be
    // asked anything about it before they are read.
    let directory = input.parent().unwrap_or(Path::new(""));
    let sections = read_sections(&markdown, directory)?;

    if args.emit_typst {
        let typst_source =
            md2pdf_core::md_to_typst(&markdown, &sections).map_err(|e| e.to_string())?;
        print!("{typst_source}");
        std::io::stdout()
            .flush()
            .map_err(|e| format!("cannot write to stdout: {e}"))?;
        return Ok(());
    }

    let assets = read_assets(&markdown, sections, directory)?;

    let pdf = md2pdf_core::md_to_pdf(&markdown, &assets).map_err(|e| e.to_string())?;
    let output = args.output.unwrap_or_else(|| default_output(&input));

    std::fs::write(&output, pdf).map_err(|e| format!("cannot write {}: {e}", output.display()))
}

/// Read every file the document names, from beside the document.
///
/// A path resolves against the directory of the input file, so a document, its
/// figures and its bibliography travel as one folder. An asset keeps the path
/// the markdown wrote, because that is the name the generated Typst source asks
/// for, and it is the name every later error uses.
///
/// The image list arrives in document order and may name one path twice, so
/// this reads each file once. The bibliography is one frontmatter value rather
/// than something the walk finds, so it comes from an export of its own — and
/// it is read first, since the line it names is the earliest one in the file.
///
/// **The sections are already read when this runs**, and they arrive here so
/// they ride out on the same array: the other two lists cannot be asked for
/// until the document they belong to has been assembled, which is why
/// `read_sections` is a pass of its own and this one takes its result.
///
/// **Every path joins the master's directory, a section's own images included**,
/// and that is not the limitation it once was: `core` writes a section's own
/// folder into the destination before the list reaches here, so an image drawn
/// in `sections/method.md` arrives as `sections/figure.png` and is found beside
/// the file that drew it. This function needed nothing for that, which is why
/// the app inherits the rule rather than carrying a copy of it.
///
/// `core` reads nothing itself, on any of the three channels. That split is what
/// lets the same crate compile natively and to `wasm32`.
fn read_assets(
    markdown: &str,
    sections: Vec<md2pdf_core::Asset>,
    directory: &Path,
) -> Result<Vec<md2pdf_core::Asset>, String> {
    let images = md2pdf_core::image_paths(markdown, &sections).map_err(|e| e.to_string())?;
    let bibliography =
        md2pdf_core::bibliography_path(markdown, &sections).map_err(|e| e.to_string())?;

    let mut seen: HashSet<String> = sections.iter().map(|s| s.path.clone()).collect();
    let mut assets = sections;

    if let Some(named) = bibliography {
        let file = directory.join(&named.path);
        let bytes = std::fs::read(&file).map_err(|e| {
            format!(
                "cannot read {} for the bibliography {}: {e}",
                file.display(),
                named.location
            )
        })?;

        seen.insert(named.path.clone());
        assets.push(md2pdf_core::Asset {
            path: named.path,
            bytes,
        });
    }

    for image in images {
        if !seen.insert(image.path.clone()) {
            continue;
        }

        let file = directory.join(&image.path);
        let bytes = std::fs::read(&file).map_err(|e| {
            format!(
                "cannot read {} for the image {}: {e}",
                file.display(),
                image.location
            )
        })?;

        assets.push(md2pdf_core::Asset {
            path: image.path,
            bytes,
        });
    }
    Ok(assets)
}

/// Read every section file the master names, in the order it names them.
///
/// **This runs before the other two lists and before `--emit-typst`**, because
/// the markers are in the master's own text and every later question is about
/// the document they assemble into. One extra round trip through `core`, no
/// recursion here, and one place that ever concatenates — which is `core`,
/// because it is the joining that builds the map every message is translated
/// through.
///
/// A section that will not open is exit 1 naming the resolved path, the line the
/// master named it on, and the message the OS gave — the third of the same
/// sentence the image and the bibliography already print.
fn read_sections(markdown: &str, directory: &Path) -> Result<Vec<md2pdf_core::Asset>, String> {
    let named = md2pdf_core::section_paths(markdown).map_err(|e| e.to_string())?;

    let mut sections = Vec::with_capacity(named.len());
    for section in named {
        let file = directory.join(&section.path);
        let bytes = std::fs::read(&file).map_err(|e| {
            format!(
                "cannot read {} for the section {}: {e}",
                file.display(),
                section.location
            )
        })?;

        sections.push(md2pdf_core::Asset {
            path: section.path,
            bytes,
        });
    }

    Ok(sections)
}

/// The input path with its extension replaced by `.pdf`.
fn default_output(input: &Path) -> PathBuf {
    input.with_extension("pdf")
}

/// What the binary carries and under what terms, in one page.
///
/// **A provenance notice, not a dependency dump.** The texts `licenses` below
/// returns run to 982 lines, and the two facts a reader holding the executable
/// would actually be surprised by — that the Typst compiler is compiled in
/// under Apache-2.0, and that six font faces travel under two licences of their
/// own — are one table row and one filename line somewhere inside them. This is
/// what the bare flag prints; `--licenses=full` still prints the texts.
///
/// Hand-written prose, and every fact in it has a source the suite reads back:
/// the copyright line is `LICENSE`'s, the two versions, the crate count and the
/// licence terms are `THIRD-PARTY-LICENSES.md`'s table, and the two face counts
/// are `core/assets/fonts/`. The five URLs are the one kind of fact no test can
/// check; they are opened by hand when this text is touched.
const NOTICE: &str = "md2pdf — its own source is MIT.

    Copyright (c) 2026 ivapo
    https://github.com/Ivapo/md2pdf/blob/main/LICENSE

This binary is statically linked, so it carries work that is not md2pdf's and
is not covered by that licence:

    Typst 0.15.1
    Project   https://github.com/typst/typst
    Bundled   as crates compiled in — typst, typst-pdf and their siblings
    Licence   Apache-2.0

    mitex 0.2.4
    Project   https://github.com/mitex-rs/mitex
    Bundled   as a crate compiled in; it translates LaTeX math into Typst's
    Licence   Apache-2.0

    Libertinus Serif and Libertinus Mono
    Project   https://github.com/alerque/libertinus
    Bundled   five faces, the body and code fonts of every page
    Licence   SIL Open Font License 1.1

    NewCMMath-Regular
    Project   New Computer Modern — https://ctan.org/pkg/newcomputermodern
    Bundled   one face, the math font
    Licence   GUST Font License

Everything else is a Rust crate compiled from source. THIRD-PARTY-LICENSES.md
names the 334 crates the resolve reaches, each under one or more of MIT,
Apache-2.0 (once with the LLVM-exception), BSD-2-Clause, BSD-3-Clause, Zlib,
Unicode-3.0, 0BSD, CC0-1.0, BSL-1.0 and the Unlicense. None is copyleft, so
what they ask for is attribution, which is this notice. The full texts — the
MIT terms, both font licences and that list — are compiled into this binary
too:

    md2pdf --licenses=full

This records provenance and is not legal advice.
";

/// Every licence this binary carries, as one block of text.
///
/// Four parts joined by a blank line: this program's own MIT terms, the two
/// font licences under the filenames they ship as, and the generated list
/// covering every crate compiled in.
///
/// All four are `include_str!`, so nothing is generated at run time, nothing is
/// read from disk and there is no path on which this can fail — which is the
/// point. `md2pdf` is statically linked and its faces are embedded, so the
/// terms of 300-odd crates and six fonts follow the executable whether or not
/// the tree it was built from is anywhere near it.
///
/// **This is what `--licenses=full` prints.** The bare flag prints `NOTICE`,
/// which names this form in its last paragraph.
///
/// The font pair arrives through `md2pdf_core::FONT_LICENSES` rather than from
/// `core/assets/` directly, because a published `md2pdf-cli` archive holds no
/// part of the core crate: an `include_str!` reaching over there resolves in a
/// checkout and fails for everyone who installs from the registry.
fn licenses() -> String {
    let mut parts = vec![include_str!("../LICENSE").to_string()];

    for (filename, text) in md2pdf_core::FONT_LICENSES {
        parts.push(format!("{filename}\n{text}"));
    }

    parts.push(include_str!("../THIRD-PARTY-LICENSES.md").to_string());
    parts.join("\n\n")
}
