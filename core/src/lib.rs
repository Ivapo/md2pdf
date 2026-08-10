//! Converts one markdown document into one typeset PDF.
//!
//! This crate holds no OS access. Its API takes a markdown string and the image
//! files that string names, and it returns Typst source or PDF bytes. The caller
//! does the file I/O. That split is what lets the same crate compile natively
//! and to `wasm32` without a rewrite, and it is why an image arrives as named
//! bytes rather than as a path this crate would have to open.

mod emit;
mod frontmatter;

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::frontmatter::Template;

use typst::LibraryExt;
use typst::World;
use typst::diag::{FileError, FileResult, SourceDiagnostic, Warned};
use typst::foundations::{Bytes, Datetime, Duration};
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst_pdf::PdfOptions;

/// The errors this crate can return.
///
/// These clone, because a footnote definition's translation is kept until the
/// walk reaches the region it belongs to, and the error it produced is what
/// that region reports.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// A markdown construct outside the supported dialect.
    #[error("unsupported markdown construct '{construct}' at line {line}")]
    UnsupportedConstruct { construct: String, line: usize },

    /// The frontmatter block does not match the schema.
    #[error("frontmatter error at line {line}: {problem}")]
    Frontmatter { line: usize, problem: String },

    /// The document names an image file that the caller did not supply.
    #[error("no image file supplied for '{path}' at line {line}")]
    MissingImage { path: String, line: usize },

    /// A supplied image holds bytes of a format other than the one its name
    /// claims.
    #[error("image file '{path}' at line {line} does not hold {format} data")]
    ImageFormat {
        path: String,
        line: usize,
        format: String,
    },

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

/// One image file, supplied by the caller.
///
/// `path` is the destination exactly as the markdown wrote it, which is the
/// name the generated Typst source asks for.
#[derive(Debug, Clone)]
pub struct Asset {
    pub path: String,
    pub bytes: Vec<u8>,
}

/// One place where a document names an image file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    pub path: String,
    pub line: usize,
}

/// Translate markdown into Typst markup.
///
/// The output imports the look the frontmatter selected, and every bundled
/// look exists only inside this crate's virtual filesystem. It serves
/// inspection, not a standalone `typst compile`. Emission reads no image
/// bytes, so this function needs no assets.
pub fn md_to_typst(md: &str) -> Result<String> {
    Ok(emit::emit(md)?.0)
}

/// List every image file the document names, in reader order.
///
/// This is the caller's shopping list: read these files, then hand them back to
/// [`md_to_pdf`] as assets. The same walk produces it that produces the Typst
/// source, so the two agree on which paths the dialect accepts.
///
/// Reader order is document order, except for an image inside a footnote
/// definition: that one joins the list at the first reference to its footnote,
/// which is where the content is set. Its line stays the line the markdown
/// named it on.
///
/// The list may name one path more than once. The caller deduplicates it.
pub fn image_paths(md: &str) -> Result<Vec<ImageRef>> {
    Ok(emit::emit(md)?.1)
}

/// Translate markdown into Typst markup and compile it to a PDF.
///
/// `assets` supplies the bytes of every path [`image_paths`] listed. An asset
/// the document never names is ignored.
pub fn md_to_pdf(md: &str, assets: &[Asset]) -> Result<Vec<u8>> {
    let (typst_source, images) = emit::emit(md)?;
    let world = TypstWorld::new(typst_source, collect(&images, assets)?)?;

    let Warned { output, .. } = typst::compile(&world);
    let document = output.map_err(|diags| Error::Compile(join(&diags)))?;

    typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|diags| Error::PdfExport(join(&diags)))
}

// -- assets -----------------------------------------------------------------

/// Check every image the document names, then build the map the world serves.
///
/// This runs before the compile, and that order is the whole point. A missing
/// or a mislabeled file would otherwise break the compile second-hand, and
/// Typst's own error would name a span in `main.typ`, which the user has never
/// seen. Here the error names the path and the line the document wrote.
///
/// One path is checked once, at its first reference, so a figure used twice
/// reports one error rather than two.
fn collect(images: &[ImageRef], assets: &[Asset]) -> Result<HashMap<FileId, Bytes>> {
    let supplied: HashMap<&str, &[u8]> = assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset.bytes.as_slice()))
        .collect();

    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    for image in images {
        if !seen.insert(image.path.as_str()) {
            continue;
        }

        let bytes = supplied
            .get(image.path.as_str())
            .ok_or_else(|| Error::MissingImage {
                path: image.path.clone(),
                line: image.line,
            })?;

        // The emitter has already refused every extension outside Typst's own
        // table, so the extension is known here and it alone names the format.
        let extension = emit::extension_of(&image.path).unwrap_or_default();
        if !bytes_match(&extension, bytes) {
            return Err(Error::ImageFormat {
                path: image.path.clone(),
                line: image.line,
                format: format_name(&extension).to_string(),
            });
        }

        map.insert(file_id(&image.path)?, Bytes::new(bytes.to_vec()));
    }
    Ok(map)
}

/// Whether the bytes hold the format that the extension names.
///
/// This mirrors `typst-library` 0.15.1's own detection: the magic bytes for the
/// raster formats and for PDF, the gzip magic for `svgz`, and a namespace
/// search over the first 2048 bytes for `svg`. Typst's fallback — detect the
/// content when the extension says nothing — is deliberately not mirrored, so
/// an extension outside the table never reaches this function.
///
/// The recorded limit: a file that is corrupt past its magic bytes still fails
/// at compile time, with the compiler's own message. Catching that would mean
/// decoding every image twice.
fn bytes_match(extension: &str, bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(2048)];
    match extension {
        "png" => bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "jpg" | "jpeg" => bytes.starts_with(b"\xff\xd8\xff"),
        "gif" => bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a"),
        "webp" => bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP",
        "svg" => holds(head, b"http://www.w3.org/2000/svg"),
        "svgz" => bytes.starts_with(&[0x1f, 0x8b]),
        "pdf" => holds(head, b"%PDF-"),
        _ => false,
    }
}

/// Whether `haystack` holds `needle` anywhere. `needle` is never empty.
fn holds(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

/// The name of a format, for the error message an extension mismatch produces.
fn format_name(extension: &str) -> &'static str {
    match extension {
        "png" => "PNG",
        "jpg" | "jpeg" => "JPEG",
        "gif" => "GIF",
        "webp" => "WebP",
        "svg" => "SVG",
        "svgz" => "gzip-compressed SVG",
        "pdf" => "PDF",
        _ => "image",
    }
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

/// The bytes of one bundled look, embedded at compile time.
///
/// `frontmatter::Template` owns the filename and this function owns the
/// content, so the same enum drives both and no look can be selectable under a
/// name that binds no file.
fn template_source(template: Template) -> &'static str {
    match template {
        Template::Article => include_str!("../assets/template.typ"),
        Template::PressRelease => include_str!("../assets/press-release.typ"),
    }
}

/// The minimal compilation environment.
///
/// It holds the generated source, every bundled template, the images the
/// document names, and the bundled fonts. There is no package resolution here
/// at all, so nothing in this crate can reach the network on any target.
///
/// Every template is bound, not only the selected one, so the walk never has
/// to plumb its choice out here. The templates are compile-time constants
/// either way, the dialect has no syntax for a raw Typst import, and only the
/// emitter ever writes one.
///
/// The assets ride the same virtual filesystem that already serves the
/// templates, which is why images need no second channel. `main.typ` sits at
/// the virtual root, so a relative path in the generated source resolves to
/// the file id built from that same path.
struct TypstWorld {
    main: Source,
    templates: Vec<Source>,
    assets: HashMap<FileId, Bytes>,
}

impl TypstWorld {
    fn new(typst_source: String, assets: HashMap<FileId, Bytes>) -> Result<Self> {
        let mut templates = Vec::with_capacity(Template::ALL.len());
        for template in Template::ALL {
            let id = file_id(template.file())?;
            templates.push(Source::new(id, template_source(template).to_string()));
        }

        Ok(Self {
            main: Source::new(file_id(MAIN_NAME)?, typst_source),
            templates,
            assets,
        })
    }

    fn lookup(&self, id: FileId) -> Option<&Source> {
        if id == self.main.id() {
            return Some(&self.main);
        }
        self.templates.iter().find(|source| source.id() == id)
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

    /// An image asks for its bytes here, and so does a template.
    ///
    /// `source` is untouched by the assets, because an image is never Typst
    /// source. That is what keeps the import story, and the network story with
    /// it, exactly as it was.
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(bytes) = self.assets.get(&id) {
            return Ok(bytes.clone());
        }
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
    /// avoid, and it would make the compiled PDF differ between machines. The
    /// break would also ship silently: this call touches the compile alone and
    /// never the emitted source, so the golden files would stay byte-stable
    /// over a PDF that differed by machine.
    ///
    /// Every bundled template does typeset a date, and takes it from the
    /// frontmatter's `date` key. The author writes the dateline, so `None`
    /// costs nothing and keeps the output reproducible.
    fn today(&self, _offset: Option<Duration>) -> Option<Datetime> {
        None
    }
}
