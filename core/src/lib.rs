//! Converts one markdown document into one typeset PDF.
//!
//! This crate holds no OS access. Its API takes a markdown string and returns
//! Typst source or PDF bytes, and the caller does the file I/O. That split is
//! what lets the same crate compile natively and to `wasm32` without a rewrite.

mod emit;
mod frontmatter;

use std::sync::LazyLock;

use typst::LibraryExt;
use typst::World;
use typst::diag::{FileError, FileResult, SourceDiagnostic, Warned};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst_pdf::PdfOptions;

/// The errors this crate can return.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A markdown construct outside the supported dialect.
    #[error("unsupported markdown construct '{construct}' at line {line}")]
    UnsupportedConstruct { construct: String, line: usize },

    /// The frontmatter block does not match the schema.
    #[error("frontmatter error at line {line}: {problem}")]
    Frontmatter { line: usize, problem: String },

    /// The Typst compiler rejected the generated source.
    #[error("typst compilation failed: {0}")]
    Compile(String),

    /// The Typst PDF exporter failed.
    #[error("pdf export failed: {0}")]
    PdfExport(String),

    /// A bundled asset is malformed. This means a broken build, not bad input.
    #[error("internal error: {0}")]
    Internal(String),
}

/// The result type this crate returns.
pub type Result<T> = std::result::Result<T, Error>;

/// Translate markdown into Typst markup.
///
/// The output names `template.typ`, which exists only inside this crate's
/// virtual filesystem. It serves inspection, not a standalone `typst compile`.
pub fn md_to_typst(md: &str) -> Result<String> {
    emit::emit(md)
}

/// Translate markdown into Typst markup and compile it to a PDF.
pub fn md_to_pdf(md: &str) -> Result<Vec<u8>> {
    let world = TypstWorld::new(md_to_typst(md)?)?;

    let Warned { output, .. } = typst::compile(&world);
    let document = output.map_err(|diags| Error::Compile(join(&diags)))?;

    typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|diags| Error::PdfExport(join(&diags)))
}

fn join(diags: &[SourceDiagnostic]) -> String {
    diags
        .iter()
        .map(|d| d.message.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

// -- fonts ------------------------------------------------------------------

// Fonts are bundled and embedded at compile time on every target. A browser
// sandbox has no OS font access, and OS discovery would make the compiled PDF
// depend on the machine that produced it.
//
// Every face the dialect can reach is here, because Typst renders what it
// finds and never synthesises a missing one. Without the italic, `#emph` would
// come out identical to body text; without the mono, `#raw` would come out as
// the serif. Both would be a PDF that lies about its source. All five files
// come from one Libertinus release, so their metrics agree.
const REGULAR: &[u8] = include_bytes!("../assets/fonts/LibertinusSerif-Regular.otf");
const BOLD: &[u8] = include_bytes!("../assets/fonts/LibertinusSerif-Bold.otf");
const ITALIC: &[u8] = include_bytes!("../assets/fonts/LibertinusSerif-Italic.otf");
const BOLD_ITALIC: &[u8] = include_bytes!("../assets/fonts/LibertinusSerif-BoldItalic.otf");
const MONO: &[u8] = include_bytes!("../assets/fonts/LibertinusMono-Regular.otf");

static FONTS: LazyLock<Vec<Font>> = LazyLock::new(|| {
    [REGULAR, BOLD, ITALIC, BOLD_ITALIC, MONO]
        .into_iter()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .collect()
});

static BOOK: LazyLock<LazyHash<FontBook>> =
    LazyLock::new(|| LazyHash::new(FontBook::from_fonts(FONTS.iter())));

static LIBRARY: LazyLock<LazyHash<typst::Library>> =
    LazyLock::new(|| LazyHash::new(typst::Library::default()));

// -- world ------------------------------------------------------------------

const MAIN_NAME: &str = "main.typ";
const TEMPLATE_NAME: &str = "template.typ";

const TEMPLATE_SOURCE: &str = include_str!("../assets/template.typ");

/// The minimal compilation environment.
///
/// It holds exactly two files and the bundled fonts. There is no package
/// resolution here at all, so nothing in this crate can reach the network on
/// any target.
struct TypstWorld {
    main: Source,
    template: Source,
}

impl TypstWorld {
    fn new(typst_source: String) -> Result<Self> {
        Ok(Self {
            main: Source::new(file_id(MAIN_NAME)?, typst_source),
            template: Source::new(file_id(TEMPLATE_NAME)?, TEMPLATE_SOURCE.to_string()),
        })
    }

    fn lookup(&self, id: FileId) -> Option<&Source> {
        if id == self.main.id() {
            Some(&self.main)
        } else if id == self.template.id() {
            Some(&self.template)
        } else {
            None
        }
    }
}

fn file_id(name: &str) -> Result<FileId> {
    let vpath = VirtualPath::new(name)
        .map_err(|e| Error::Internal(format!("bad virtual path '{name}': {e}")))?;
    Ok(FileId::new(RootedPath::new(VirtualRoot::Project, vpath)))
}

impl World for TypstWorld {
    fn library(&self) -> &LazyHash<typst::Library> {
        &LIBRARY
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &BOOK
    }

    fn main(&self) -> FileId {
        self.main.id()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.lookup(id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().get_without_slash().into()))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.lookup(id)
            .map(|s| Bytes::from_string(s.text().to_string()))
            .ok_or_else(|| FileError::NotFound(id.vpath().get_without_slash().into()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        FONTS.get(index).cloned()
    }

    /// No date is supplied.
    ///
    /// Reading an OS clock would give this crate the OS access it exists to
    /// avoid, and it would make the compiled PDF differ between machines. No
    /// template in this phase uses a date, so `None` costs nothing and keeps
    /// the output reproducible.
    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}
