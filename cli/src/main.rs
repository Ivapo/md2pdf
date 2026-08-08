//! `md2pdf` — convert one markdown file into one typeset PDF.
//!
//! This binary owns all file I/O and all terminal output. The core crate owns
//! the pipeline and touches neither, which is what keeps it portable.

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
    input: PathBuf,

    /// Where to write the PDF. The default is the input path with a .pdf
    /// extension. This option has no effect with --emit-typst.
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,

    /// Print the generated Typst source instead of compiling a PDF.
    #[arg(long = "emit-typst")]
    emit_typst: bool,
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

    let markdown = std::fs::read_to_string(&args.input)
        .map_err(|e| format!("cannot read {}: {e}", args.input.display()))?;

    if args.emit_typst {
        let typst_source = md2pdf_core::md_to_typst(&markdown).map_err(|e| e.to_string())?;
        print!("{typst_source}");
        std::io::stdout()
            .flush()
            .map_err(|e| format!("cannot write to stdout: {e}"))?;
        return Ok(());
    }

    let pdf = md2pdf_core::md_to_pdf(&markdown).map_err(|e| e.to_string())?;
    let output = args.output.unwrap_or_else(|| default_output(&args.input));

    std::fs::write(&output, pdf).map_err(|e| format!("cannot write {}: {e}", output.display()))
}

/// The input path with its extension replaced by `.pdf`.
fn default_output(input: &Path) -> PathBuf {
    input.with_extension("pdf")
}
