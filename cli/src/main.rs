//! `md2pdf` — convert one markdown file into one typeset PDF.
//!
//! This binary owns all file I/O and all terminal output. The core crate owns
//! the pipeline and touches neither, which is what keeps it portable.

use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

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

    /// Print every licence this binary carries and exit: this program's own,
    /// the bundled fonts' and every compiled-in crate's.
    #[arg(long = "licenses")]
    licenses: bool,
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
    if args.licenses {
        print!("{}", licenses());
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
