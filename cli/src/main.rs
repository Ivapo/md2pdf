//! `md2pdf` — convert one markdown file into one typeset PDF.
//!
//! This binary owns all file I/O and all terminal output, and, under `--fetch`,
//! the network. The core crate owns the pipeline and touches none of them, which
//! is what keeps it portable.

use std::collections::HashSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

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

    /// Download the images the document names by an http or https URL.
    //
    // Off by default (`mpdf-002` OQ-3): a run without it stays as offline as
    // every run before the flag existed, so a CI build, or a document from
    // someone else, reaches the network only when a person decides it should.
    #[arg(long = "fetch")]
    fetch: bool,

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

/// The line printed under `core`'s refusal of an image nobody fetched.
///
/// `core` cannot say this itself: the flag is this binary's, and another caller
/// has none.
const FETCH_HINT: &str = "pass --fetch to download images named by a URL";

/// How long one fetch may take, connecting and reading the body both.
///
/// It is `ureq`'s global timeout, which bounds the body read as well as the
/// connect: `mpdf-002`'s Phase 4 review measured a trickling body stopped at
/// exactly the limit.
const FETCH_TIMEOUT: Duration = Duration::from_secs(30);

/// The most bytes one fetched image may hold. A body of exactly this many is
/// accepted.
const FETCH_LIMIT: u64 = 20 * 1024 * 1024;

/// How many redirects one fetch follows before it fails.
///
/// Set explicitly although it is also `ureq`'s default, so a version bump that
/// moved the default cannot move this.
const MAX_REDIRECTS: u32 = 10;

/// A run that failed: the message, and a line to print under it where the
/// binary has one.
///
/// Every failure is a message alone except one. `From<String>` is what lets
/// every `?` over a `Result<_, String>` go on reading as it did.
struct Failure {
    message: String,
    hint: Option<&'static str>,
}

impl From<String> for Failure {
    fn from(message: String) -> Self {
        Self {
            message,
            hint: None,
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(failure) => {
            eprintln!("error: {}", failure.message);
            if let Some(hint) = failure.hint {
                eprintln!("hint: {hint}");
            }
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Failure> {
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

    let assets = read_assets(&markdown, sections, directory, args.fetch)?;

    // The hint is printed for one refusal only, so every other message stays
    // the sentence it always was.
    let pdf = md2pdf_core::md_to_pdf(&markdown, &assets).map_err(|e| Failure {
        hint: (matches!(e, md2pdf_core::Error::UnfetchedImage { .. }) && !args.fetch)
            .then_some(FETCH_HINT),
        message: e.to_string(),
    })?;
    let output = args.output.unwrap_or_else(|| default_output(&input));

    std::fs::write(&output, pdf).map_err(|e| format!("cannot write {}: {e}", output.display()))?;
    Ok(())
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
/// **An image named by a URL is fetched under `--fetch` and skipped without
/// it.** Skipped, `core` refuses it as `no image fetched for '…'` and `main`
/// prints the hint under it. Fetched, each distinct URL is one GET, one at a
/// time, in document order beside the file reads; the bytes ride out under the
/// URL itself, which is the name `core` looks them up by. A fetch that fails
/// stops the run the way a file that will not read does, naming the URL, the
/// line and the reason.
///
/// One consequence is accepted rather than fixed: this stops at its first
/// failure in document order and runs before `core` checks anything, so with a
/// URL skipped on line 3 and an unreadable file on line 9, line 9 is what the
/// author hears. The earliest-line rule holds inside `core`, not across the two.
///
/// `core` reads nothing itself, on any of the three channels, and fetches
/// nothing. That split is what lets the same crate compile natively and to
/// `wasm32`.
fn read_assets(
    markdown: &str,
    sections: Vec<md2pdf_core::Asset>,
    directory: &Path,
    fetch: bool,
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

    // Built at the first URL, so a run that names none builds no client.
    let mut agent: Option<ureq::Agent> = None;

    for image in images {
        // Without `--fetch` a URL gets no bytes, and `core`'s own refusal is
        // what names it. It is never joined onto the directory: that would hand
        // the OS `dir/https://…` and the author an OS error about a file that
        // was never meant to exist.
        if (image.is_url() && !fetch) || !seen.insert(image.path.clone()) {
            continue;
        }

        let bytes = if image.is_url() {
            let agent = agent.get_or_insert_with(fetch_agent);
            fetch_image(agent, &image.path).map_err(|reason| {
                format!(
                    "cannot fetch {} for the image {}: {reason}",
                    image.path, image.location
                )
            })?
        } else {
            let file = directory.join(&image.path);
            std::fs::read(&file).map_err(|e| {
                format!(
                    "cannot read {} for the image {}: {e}",
                    file.display(),
                    image.location
                )
            })?
        };

        assets.push(md2pdf_core::Asset {
            path: image.path,
            bytes,
        });
    }
    Ok(assets)
}

/// The client every fetch in one run goes through, with each limit set here.
///
/// **Every guard is a named constant or an explicit call**, never a default a
/// version bump could move:
///
/// - `FETCH_TIMEOUT` is the global timeout, which bounds the body read as well
///   as the connect.
/// - `MAX_REDIRECTS` is set although it is also the default. A redirect cannot
///   leave `http` and `https`, because `ureq` refuses any other scheme in a
///   `Location`.
/// - `http_status_as_error(false)` hands every status back, so [`fetch_image`]
///   decides which succeed. `ureq`'s own check refuses 4xx and 5xx only, so a
///   304 would reach `core` as an empty body and be refused for the wrong
///   reason.
///
/// Two guards live in the crate's features rather than here: no cookie store,
/// and no `gzip`, so the size cap counts exactly the bytes `core` is handed.
/// Nothing is cached and nothing is written to disk: a second run fetches
/// again.
///
/// **A proxy the environment names is honoured.** `ureq` routes every fetch
/// through the first of `ALL_PROXY`, `HTTPS_PROXY` and `HTTP_PROXY` that is
/// set, in either case, and `NO_PROXY` exempts hosts. A run inside a network
/// that requires a proxy should still reach the image.
fn fetch_agent() -> ureq::Agent {
    let config = ureq::Agent::config_builder()
        .timeout_global(Some(FETCH_TIMEOUT))
        .max_redirects(MAX_REDIRECTS)
        .http_status_as_error(false)
        .build();
    ureq::Agent::new_with_config(config)
}

/// Download one image, or say why not in words that finish the sentence
/// `cannot fetch {url} for the image {location}: …`.
///
/// Three reasons are written here and the rest are `ureq`'s own:
///
/// - **A status outside 2xx** is its code and canonical reason, `404 Not
///   Found`, or the bare code where there is none, rather than `ureq`'s
///   `http status: 404`.
/// - **The cap** is `larger than 20 MB`, rather than `ureq`'s sentence naming
///   the `+ 1` below.
/// - **Anything else**, a timeout, a refused connection, a malformed URL such as
///   `https:/example.com/x.png`, or too many redirects, is `ureq::Error` as it
///   displays itself.
///
/// `Content-Type` is not read: `core`'s check of the bytes is the authority on
/// what they hold, and a header is one more thing a server can get wrong.
fn fetch_image(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>, String> {
    let mut response = agent.get(url).call().map_err(|e| e.to_string())?;

    let status = response.status();
    if !status.is_success() {
        return Err(match status.canonical_reason() {
            Some(reason) => format!("{} {reason}", status.as_u16()),
            None => status.as_u16().to_string(),
        });
    }

    response
        .body_mut()
        .with_config()
        // `ureq`'s `LimitReader` refuses a body that *reaches* its limit, not
        // only one that passes it, so a body of exactly `FETCH_LIMIT` bytes
        // needs one byte of headroom to be accepted.
        .limit(FETCH_LIMIT + 1)
        .read_to_vec()
        .map_err(|e| match e {
            ureq::Error::BodyExceedsLimit(_) => {
                format!("larger than {} MB", FETCH_LIMIT / (1024 * 1024))
            }
            other => other.to_string(),
        })
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
/// returns run to 1500 lines, and the two facts a reader holding the executable
/// would actually be surprised by — that the Typst compiler is compiled in
/// under Apache-2.0, and that six font faces travel under two licences of their
/// own — are one table row and one filename line somewhere inside them. This is
/// what the bare flag prints; `--licenses=full` still prints the texts.
///
/// Hand-written prose, and every fact in it has a source the suite reads back:
/// the copyright line is `LICENSE`'s; the three versions, the crate count, the
/// licence terms and the MPL-2.0 crates are `THIRD-PARTY-LICENSES.md`'s table;
/// and the two face counts are `core/assets/fonts/`. The six URLs are the one
/// kind of fact no test can check; they are opened by hand when this text is
/// touched.
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

    merman 0.8.0-alpha.6
    Project   https://github.com/Latias94/merman
    Bundled   as a crate compiled in; it draws Mermaid diagrams as SVG
    Licence   MIT OR Apache-2.0

    Libertinus Serif and Libertinus Mono
    Project   https://github.com/alerque/libertinus
    Bundled   five faces, the body and code fonts of every page
    Licence   SIL Open Font License 1.1

    NewCMMath-Regular
    Project   New Computer Modern — https://ctan.org/pkg/newcomputermodern
    Bundled   one face, the math font
    Licence   GUST Font License

Everything else is a Rust crate compiled from source. THIRD-PARTY-LICENSES.md
names the 379 crates the resolve reaches, each under one or more of MIT,
Apache-2.0 (once with the LLVM-exception), BSD-2-Clause, BSD-3-Clause, ISC,
Zlib, Unicode-3.0, 0BSD, CC0-1.0, BSL-1.0, the Unlicense, MPL-2.0 and, for the
TLS root certificates, CDLA-Permissive-2.0. Of those, cssparser,
cssparser-macros, dtoa-short and selectors are MPL-2.0, which is copyleft per
file: they are compiled in unmodified, and their source is on crates.io under
the name and version that list gives each. What the rest ask for is
attribution, which is this notice. The full texts — the MIT terms, both font
licences and that list — are compiled into this binary too:

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
