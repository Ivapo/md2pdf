//! Converts one markdown document into one typeset PDF.
//!
//! This crate holds no OS access. Its API takes a markdown string and the files
//! that string names — its images, its bibliography and its sections — and it
//! returns Typst source, HTML or PDF bytes. The
//! caller does the file I/O. That split is what lets the same crate compile
//! natively and to `wasm32` without a rewrite, and it is why an image arrives as
//! named bytes rather than as a path this crate would have to open.

mod bibliography;
mod emit;
mod frontmatter;
mod math;
mod sections;

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use crate::frontmatter::Template;

use typst::LibraryExt;
use typst::World;
use typst::diag::{FileError, FileResult, SourceDiagnostic, Warned};
// Two of these are in scope for one expression each and look unused at the call
// site: `Introspector` because `query` is a trait method, and `NativeElement`
// because `ELEM` is an associated const. `position`, being inherent, needs
// neither.
use typst::foundations::{Bytes, Datetime, Duration, NativeElement};
use typst::introspection::Introspector;
use typst::model::HeadingElem;
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst_pdf::PdfOptions;

/// Where in the author's own source something is.
///
/// **One type with one `Display`**, because a message carrying a file and a line
/// as two fields would have to choose its phrasing again at each of the nine
/// sites that print one. With no file it renders `at line 12`, which is the
/// phrase every message printed before a document could have sections, character
/// for character. With one it renders `in sections/method.md at line 4`.
///
/// **A source file is never quoted and an asset path always is.** That is what
/// keeps the two apart in the four messages carrying both: `no image file
/// supplied for 'fig.png' in sections/two.md at line 3` reads once and
/// correctly.
///
/// Inside this crate every location is built by [`Location::at`] against the
/// *joined* document and carries no file. The file arrives at one boundary, on
/// the way out, where `sections::Sources` translates a joined line back into the
/// file the author wrote it in. A document naming no section has a one-entry map,
/// so that translation is the identity and every message is what it always was.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    /// The section file the author wrote this line in, or `None` for the
    /// document the caller handed in.
    pub file: Option<String>,
    /// The 1-based line, in `file` where there is one.
    pub line: usize,
}

impl Location {
    /// A location in the document the caller handed in.
    pub fn at(line: usize) -> Self {
        Self { file: None, line }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.file {
            None => write!(f, "at line {}", self.line),
            Some(file) => write!(f, "in {file} at line {}", self.line),
        }
    }
}

/// The errors this crate can return.
///
/// These clone, because a footnote definition's translation is kept until the
/// walk reaches the region it belongs to, and the error it produced is what
/// that region reports.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// A markdown construct outside the supported dialect.
    #[error("unsupported markdown construct '{construct}' {location}")]
    UnsupportedConstruct {
        construct: String,
        location: Location,
    },

    /// The frontmatter block does not match the schema.
    #[error("frontmatter error {location}: {problem}")]
    Frontmatter { location: Location, problem: String },

    /// A math span holds LaTeX outside the accepted subset.
    ///
    /// This is not an `UnsupportedConstruct`: the construct is a math span,
    /// which the dialect supports, and what the error names is the LaTeX inside
    /// it — a command, an environment or a character the author typed.
    #[error("math error {location}: {problem}")]
    Math { location: Location, problem: String },

    /// A figure's name, or a reference to one, that the dialect refuses.
    ///
    /// This is not an `UnsupportedConstruct`, on the same argument `Math` was
    /// added under: the construct is a caption or a link, both of which the
    /// dialect supports, and what the error names is the name inside it — which
    /// is what the author typed. Typst resolves a label itself and fails on one
    /// it cannot, but its message names a label the author never wrote and
    /// carries no line, so the check is `core`'s.
    #[error("name error {location}: {problem}")]
    Name { location: Location, problem: String },

    /// A citation the document cannot honour, or a payload the dialect does not
    /// read.
    ///
    /// This is not an `UnsupportedConstruct`, on the argument [`Error::Math`]
    /// and [`Error::Name`] were both added under: the construct is a citation,
    /// which the dialect supports, and what the error names is what the author
    /// typed inside its brackets — or the bibliography key the frontmatter left
    /// out. Typst raises on some of these itself, in its own words and with no
    /// line the author would recognise.
    #[error("citation error {location}: {problem}")]
    Citation { location: Location, problem: String },

    /// The document names an image file that the caller did not supply.
    #[error("no image file supplied for '{path}' {location}")]
    MissingImage { path: String, location: Location },

    /// The frontmatter names a bibliography file that the caller did not supply.
    ///
    /// A sibling of [`Error::MissingImage`] rather than a reuse of it: the words
    /// are the only thing that differs, and they are the whole point — without
    /// this the compile says "file not found (searched at refs.yml)" against a
    /// span in a `main.typ` the user has never seen.
    #[error("no bibliography file supplied for '{path}' {location}")]
    MissingBibliography { path: String, location: Location },

    /// The master names a section file that the caller did not supply.
    ///
    /// A third sibling, on the argument [`Error::MissingBibliography`] was added
    /// under and for the same reason. It lives here rather than in a wrapper
    /// because `web/src/lib.rs:render` calls [`md_to_pdf`] directly with a fixed
    /// asset array, and there is no wrapper there to catch it.
    #[error("no section file supplied for '{path}' {location}")]
    MissingSection { path: String, location: Location },

    /// A supplied image holds bytes of a format other than the one its name
    /// claims.
    #[error("image file '{path}' {location} does not hold {format} data")]
    ImageFormat {
        path: String,
        location: Location,
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

impl Error {
    /// The location this error names, where it names one.
    ///
    /// **This is the whole of the relocation surface**, which is why it is one
    /// exhaustive match and not a `_` arm: `sections::Sources` translates through
    /// it, and a tenth line-carrying variant added later cannot slip past the
    /// translation without the compiler saying so.
    pub(crate) fn location_mut(&mut self) -> Option<&mut Location> {
        match self {
            Error::UnsupportedConstruct { location, .. }
            | Error::Frontmatter { location, .. }
            | Error::Math { location, .. }
            | Error::Name { location, .. }
            | Error::Citation { location, .. }
            | Error::MissingImage { location, .. }
            | Error::MissingBibliography { location, .. }
            | Error::MissingSection { location, .. }
            | Error::ImageFormat { location, .. } => Some(location),
            Error::Compile(_) | Error::PdfExport(_) | Error::Internal(_) => None,
        }
    }
}

/// The result type this crate returns.
pub type Result<T> = std::result::Result<T, Error>;

/// One file supplied by the caller: an image, a bibliography or a section.
///
/// `path` is the destination exactly as the markdown wrote it, which is the
/// name the generated Typst source asks for, and the name a section marker is
/// matched against.
#[derive(Debug, Clone)]
pub struct Asset {
    pub path: String,
    pub bytes: Vec<u8>,
}

/// One place where a document names an image file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    pub path: String,
    pub location: Location,
}

/// The bibliography file a document names, and the frontmatter line that named
/// it.
///
/// [`ImageRef`]'s shape under its own name rather than a second use of that
/// one: `image_paths` names what it returns, and three callers cite that
/// contract. The location is what lets a missing file be refused in the author's
/// own terms — a bibliography is not walked, so this is the only place its
/// position is known.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BibliographyRef {
    pub path: String,
    pub location: Location,
}

/// One place where a master names a section file.
///
/// The third of the same shape, and the only one whose location can never carry
/// a file: [`section_paths`] reads the master's own text alone, and a section
/// may not name a section of its own, so there is nothing for it to relocate
/// through. It is a [`Location`] rather than a bare line so that all four refs
/// read the same, and so a later phase that allowed nesting would have the field
/// already there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionRef {
    pub path: String,
    pub location: Location,
}

/// One heading, and the page its typeset form landed on.
///
/// `location` is the markdown heading's own file and 1-based line; `page` is the
/// 1-based page of the compiled one. The Nth heading in the markdown is the Nth
/// heading in the document, which is what makes this pairing possible without a
/// source map and without the emitter writing an anchor of its own.
///
/// **It is deliberately not `Copy`.** A location owns a `String`, and the two
/// consumers never wanted a copy: `app/src/document.rs` takes the vector by
/// `into_iter` and `web/src/lib.rs` reads it by reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor {
    pub location: Location,
    pub page: usize,
}

/// What one compile produced: the bytes, and where the headings landed.
#[derive(Debug, Clone)]
pub struct Rendered {
    pub pdf: Vec<u8>,
    /// One entry per heading, in document order. **Empty when the two counts
    /// disagreed**, which [`anchors_from`] explains.
    pub anchors: Vec<Anchor>,
}

/// Translate markdown into Typst markup.
///
/// The output imports the look the frontmatter selected, and every bundled
/// look exists only inside this crate's virtual filesystem. It serves
/// inspection, not a standalone `typst compile`. Emission reads no image
/// bytes, so `sections` is the only channel it needs: the bytes of every file
/// [`section_paths`] listed, which are markdown and are joined before the walk.
/// An asset the master does not name as a section is ignored, so a caller may
/// hand the whole array here too.
pub fn md_to_typst(md: &str, sections: &[Asset]) -> Result<String> {
    let (joined, sources) = sections::assemble(md, sections)?;
    emit::emit(&joined, &sources)
        .map(|emitted| emitted.source)
        .map_err(|error| sources.relocate(error))
}

/// Translate markdown into HTML, out of the same parse the emitter reads.
///
/// One event stream, written out by pulldown-cmark's own HTML backend instead
/// of by `emit.rs`. **It is not a second pipeline** — it reads no assets,
/// returns no [`Result`] because the parse it runs cannot fail, and nothing on
/// [`md_to_pdf`]'s path calls it.
///
/// It lives here rather than in the caller because the only reader it has —
/// `web/index.html`'s comparison column, generated by
/// `core/tests/page_examples_test.rs` — is only telling the truth if both
/// columns come out of one parse with one set of options, and `emit::parser` is
/// `pub(crate)`. Every other home for this function is a second copy of it.
///
/// **It reads the emitter's own parser, callback included**, so a `[@key]` here
/// is the same event it is there: pulldown-cmark's writer renders it as an
/// ordinary link, `<a href="@k">@k</a>`, which is what a writer with no notion of
/// citations makes of one.
pub fn md_to_html(md: &str) -> String {
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, emit::parser(md));
    html
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
///
/// `sections` is the same channel [`md_to_typst`] takes, and for the same
/// reason: an image named inside a section is only visible once that section's
/// text has been joined in. Each [`ImageRef`] comes back naming the file the
/// author wrote it in.
pub fn image_paths(md: &str, sections: &[Asset]) -> Result<Vec<ImageRef>> {
    let (joined, sources) = sections::assemble(md, sections)?;
    let images = emit::emit(&joined, &sources)
        .map_err(|error| sources.relocate(error))?
        .images;

    Ok(images
        .into_iter()
        .map(|image| ImageRef {
            location: sources.locate(image.location.line),
            path: image.path,
        })
        .collect())
}

/// Name the bibliography file the document's frontmatter declares, if any.
///
/// The shopping list's second half, and a second export rather than a widening
/// of [`image_paths`], whose name and contract three callers cite. It has to be
/// one: a document's images are *found* by reading it, so they fall out of the
/// walk, where a bibliography is one frontmatter value that no walk would ever
/// meet.
///
/// The location is the frontmatter line the key was written on, which is what
/// lets a file the caller did not supply be refused in the author's own terms.
/// Only the master carries frontmatter, so that file is always the master —
/// but it travels as a [`Location`] like every other, because the phrase a
/// message prints comes from one place.
pub fn bibliography_path(md: &str, sections: &[Asset]) -> Result<Option<BibliographyRef>> {
    let (joined, sources) = sections::assemble(md, sections)?;
    let named = emit::emit(&joined, &sources)
        .map_err(|error| sources.relocate(error))?
        .bibliography;

    Ok(named.map(|named| BibliographyRef {
        location: sources.locate(named.location.line),
        path: named.path,
    }))
}

/// Name every section file the master reads, in the order it reads them.
///
/// The shopping list's third entry, and the one that runs *before* the other
/// two: the markers are in the master's own text, so this needs no join, where
/// [`image_paths`] and [`bibliography_path`] can only answer about a document
/// that has already been assembled. So the caller reads these files first and
/// hands them back on every later call.
///
/// **The joining is this crate's and never the caller's.** `core` builds the map
/// that turns a joined line back into a file from the boundaries it creates, so
/// a caller that concatenated the sections itself would leave that map with no
/// source and every message naming a document nobody wrote.
///
/// A [`SectionRef`]'s location never carries a file, because a section may not
/// name a section of its own.
pub fn section_paths(md: &str) -> Result<Vec<SectionRef>> {
    Ok(emit::includes(md)
        .into_iter()
        .map(|include| SectionRef {
            path: include.path,
            location: Location::at(include.line),
        })
        .collect())
}

/// Translate markdown into Typst markup and compile it to a PDF.
///
/// `assets` supplies the bytes of every path [`image_paths`] listed, of the one
/// [`bibliography_path`] named, and of every section [`section_paths`] named. An
/// asset the document never names is ignored.
///
/// This is [`md_to_pdf_with_anchors`] with the anchors dropped, rather than a
/// second path to the same bytes. Two paths over the same input that could
/// disagree eventually do.
pub fn md_to_pdf(md: &str, assets: &[Asset]) -> Result<Vec<u8>> {
    md_to_pdf_with_anchors(md, assets).map(|rendered| rendered.pdf)
}

/// Compile to a PDF, and say which page each heading landed on.
///
/// The bytes are exactly what [`md_to_pdf`] returns — this reads the compiled
/// document and writes nothing into it, so a caller that wants only the PDF
/// loses nothing by asking for both.
///
/// **The extraction is inline here and cannot be factored out.** `typst`
/// re-exports `typst-library`, `typst-syntax` and `typst-utils` and not
/// `typst-layout`, so the compiled document's type is unnameable in this crate:
/// method calls on the value the compiler infers are fine, a helper taking it as
/// a parameter is not, and writing one would mean adding a dependency to a
/// workspace that pins every one it has.
///
/// **The relocation happens here and not one step earlier.** Everything below
/// works in the joined document's own coordinates, because `collect` answers
/// with the earliest refusal by line and a section-local line would sort against
/// a different document. The joined line becomes a file and a line on the way
/// out, once, for the error and for every anchor.
pub fn md_to_pdf_with_anchors(md: &str, assets: &[Asset]) -> Result<Rendered> {
    let (joined, sources) = sections::assemble(md, assets)?;
    let rendered = render(&joined, &sources, assets).map_err(|error| sources.relocate(error))?;

    Ok(Rendered {
        pdf: rendered.pdf,
        anchors: rendered
            .anchors
            .into_iter()
            .map(|anchor| Anchor {
                location: sources.locate(anchor.location.line),
                page: anchor.page,
            })
            .collect(),
    })
}

/// Compile the joined document, answering in the joined document's own lines.
fn render(md: &str, sources: &sections::Sources, assets: &[Asset]) -> Result<Rendered> {
    let emitted = emit::emit(md, sources)?;
    let assets = collect(&emitted, assets)?;
    let world = TypstWorld::new(emitted.source, assets)?;

    let Warned { output, .. } = typst::compile(&world);
    let document = output.map_err(|diags| Error::Compile(join(&diags)))?;

    // The export comes first because it is what names the document's type.
    // `typst::compile` is generic over its output and this crate cannot write
    // that type down, so nothing may call a method on `document` until a use
    // like this one has pinned it.
    let pdf = typst_pdf::pdf(&document, &PdfOptions::default())
        .map_err(|diags| Error::PdfExport(join(&diags)))?;

    // The headings the document actually typeset, in document order. Neither
    // bundled look emits one of its own — both set their title with `text`, not
    // with a heading — so every element here came from the walk's own markup.
    let introspector = document.introspector();
    let pages: Vec<usize> = introspector
        .query(&HeadingElem::ELEM.select())
        .iter()
        .filter_map(|heading| heading.location())
        .filter_map(|location| introspector.position(location))
        .map(|position| position.page.get())
        .collect();

    Ok(Rendered {
        pdf,
        anchors: anchors_from(emitted.headings, pages),
    })
}

/// Pair each walked heading line with the page its typeset form landed on.
///
/// **Unequal counts return nothing**, and a pane fed no anchors behaves exactly
/// as it did before they existed. The counts can genuinely differ: a heading
/// inside a footnote definition is walked into a `Walk` that is discarded and
/// its content is spliced in at the *reference*, so it typesets a heading the
/// document walk never counted — and its line would name the wrong place.
///
/// Two things this guard is not. It catches one extra or one missing, **not one
/// of each**. And what it guards is a mis-scroll rather than a wrong document —
/// no byte of the PDF depends on it.
fn anchors_from(lines: Vec<usize>, pages: Vec<usize>) -> Vec<Anchor> {
    if lines.len() != pages.len() {
        return Vec::new();
    }

    lines
        .into_iter()
        .zip(pages)
        .map(|(line, page)| Anchor {
            location: Location::at(line),
            page,
        })
        .collect()
}

// -- assets -----------------------------------------------------------------

/// Check every file the document names and every key it cites, then build the
/// map the world serves.
///
/// This runs before the compile, and that order is the whole point. A missing
/// file, a mislabeled one or a key nothing holds would otherwise break the
/// compile second-hand, and Typst's own error would name a span in `main.typ`,
/// which the user has never seen. Here the error names the path, the key and
/// the line the document wrote.
///
/// One path is checked once, at its first reference, so a figure used twice
/// reports one error rather than two.
///
/// **The bibliography goes in unchecked by [`bytes_match`], and first.**
/// Unchecked because the file is parsed for its keys a few lines down and names
/// its own error there, where an image's magic bytes are the only thing that
/// could — and a `.yml` needs no second channel, since `Asset` is a named blob
/// with nothing image-specific in it and [`TypstWorld::file`] already answers
/// from this map by `FileId`. First because its line comes from the frontmatter
/// and is therefore earlier than every image's.
///
/// **The two citation checks live here rather than in `emit` because this is
/// the only place the bibliography's bytes exist** — emission reads no file on
/// either channel, which is what lets `md_to_typst` work on a document whose
/// bibliography is not beside it.
///
/// **Where several refusals are candidates, the earliest line is the error.**
/// One rule over every refusal this function can raise, not one per class: a
/// document with a missing image on line 3 and an absent key on line 9 has two,
/// and answering with the later one would send the author past the first thing
/// that is wrong. This is what `emit::check_references` and
/// `emit::check_citations` already do inside the walk, and for the reason they
/// record — "the first" out of a set varies between runs. Two refusals on one
/// line are settled by the order below, which is the order this function
/// already checked in.
fn collect(emitted: &emit::Emitted, assets: &[Asset]) -> Result<HashMap<FileId, Bytes>> {
    let supplied: HashMap<&str, &[u8]> = assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset.bytes.as_slice()))
        .collect();

    let mut map = HashMap::new();
    let mut seen = HashSet::new();
    let mut refusals: Vec<(usize, Error)> = Vec::new();

    if let Some(named) = &emitted.bibliography {
        match supplied.get(named.path.as_str()) {
            None => refusals.push((
                named.location.line,
                Error::MissingBibliography {
                    path: named.path.clone(),
                    location: named.location.clone(),
                },
            )),
            Some(bytes) => {
                map.insert(file_id(&named.path)?, Bytes::new(bytes.to_vec()));
                match bibliography::keys(&named.path, bytes) {
                    // A file that does not parse has no key set, so the two
                    // checks below have nothing to run against. Its own line is
                    // the frontmatter's and is earlier than either of theirs.
                    Err(problem) => refusals.push((
                        named.location.line,
                        Error::Citation {
                            location: named.location.clone(),
                            problem,
                        },
                    )),
                    Ok(keys) => refusals.extend(unresolved(&keys, emitted)),
                }
            }
        }
    }

    for image in &emitted.images {
        if !seen.insert(image.path.as_str()) {
            continue;
        }

        let Some(bytes) = supplied.get(image.path.as_str()) else {
            refusals.push((
                image.location.line,
                Error::MissingImage {
                    path: image.path.clone(),
                    location: image.location.clone(),
                },
            ));
            continue;
        };

        // The emitter has already refused every extension outside Typst's own
        // table, so the extension is known here and it alone names the format.
        let extension = emit::extension_of(&image.path).unwrap_or_default();
        if !bytes_match(&extension, bytes) {
            refusals.push((
                image.location.line,
                Error::ImageFormat {
                    path: image.path.clone(),
                    location: image.location.clone(),
                    format: format_name(&extension).to_string(),
                },
            ));
            continue;
        }

        map.insert(file_id(&image.path)?, Bytes::new(bytes.to_vec()));
    }

    match refusals.into_iter().min_by_key(|(line, _)| *line) {
        Some((_, error)) => Err(error),
        None => Ok(map),
    }
}

/// Every citation the bibliography cannot honour, and every name both of them
/// hold.
///
/// Two refusals, one key set, and Typst raises both itself in words that carry
/// no line the author would recognise: ``citation key `k` is not present in the
/// bibliography``, and ``label `<k>` occurs both in the document and a
/// bibliography``.
///
/// **The collision needs three ingredients and the reference is the third.** A
/// figure named `{#k}` in a document whose bibliography holds `k` compiles
/// perfectly well, and so does the same document citing `[@k]`; Typst raises
/// only where a `[](#k)` points at the shared label, and then whether or not the
/// key is cited. That is where the message lives — it is raised while resolving
/// a reference, not while realising a bibliography — so this is a test on
/// [`emit::Emitted::referenced`] and never on what was declared or cited.
fn unresolved(keys: &HashSet<String>, emitted: &emit::Emitted) -> Vec<(usize, Error)> {
    let absent = emitted
        .cited
        .iter()
        .filter(|(key, _)| !keys.contains(key))
        .map(|(key, line)| {
            (
                *line,
                Error::Citation {
                    location: Location::at(*line),
                    problem: format!("'@{key}' is cited and the bibliography does not hold it"),
                },
            )
        });

    let shared = emitted
        .referenced
        .iter()
        .filter(|(name, _)| keys.contains(name))
        .map(|(name, line)| {
            (
                *line,
                Error::Citation {
                    location: Location::at(*line),
                    problem: format!(
                        "'{name}' names something in this document and a key in the bibliography, and one reference cannot mean both"
                    ),
                },
            )
        });

    absent.chain(shared).collect()
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
/// The math font, which is a different kind of file from the four above: it
/// carries an OpenType MATH table, and without one Typst has no glyphs for a
/// math variable or a Greek letter at all — a formula sets as a row of boxes.
/// The name it registers under is the one Typst's own default asks for, so no
/// bundled look names a math family and the look contract is unchanged.
///
/// It is under the GUST Font License rather than the OFL the others carry;
/// `assets/fonts/GUST-FONT-LICENSE.txt` is that licence.
const MATH: &[u8] = include_bytes!("../assets/fonts/NewCMMath-Regular.otf");

static FONTS: LazyLock<Vec<Font>> = LazyLock::new(|| {
    [REGULAR, BOLD, ITALIC, BOLD_ITALIC, MONO, MATH]
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
    /// The math prelude, bound beside the looks rather than as one of them.
    ///
    /// It is deliberately not a `Template` variant: `Template::from_name`
    /// resolves the `template` frontmatter key, so a variant here would make the
    /// prelude selectable as a document look. A document that names no math
    /// never imports it.
    prelude: Source,
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
            prelude: Source::new(
                file_id(emit::PRELUDE_NAME)?,
                include_str!("../assets/math.typ").to_string(),
            ),
            main: Source::new(file_id(MAIN_NAME)?, typst_source),
            templates,
            assets,
        })
    }

    fn lookup(&self, id: FileId) -> Option<&Source> {
        if id == self.main.id() {
            return Some(&self.main);
        }
        if id == self.prelude.id() {
            return Some(&self.prelude);
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The zip pairs by ordinal, and refuses to when the counts disagree.
    ///
    /// It is its own function so this case needs no document at all: the
    /// mismatch it guards is reachable from markdown, and
    /// `golden_test.rs:a_heading_inside_a_footnote_definition_withdraws_the_anchors`
    /// reaches it — but a guard tested only through a document is a guard whose
    /// boundary nothing pins.
    #[test]
    fn the_zip_pairs_by_ordinal_and_withdraws_on_a_mismatch() {
        assert_eq!(
            anchors_from(vec![3, 9, 20], vec![1, 1, 2]),
            vec![
                Anchor {
                    location: Location::at(3),
                    page: 1
                },
                Anchor {
                    location: Location::at(9),
                    page: 1
                },
                Anchor {
                    location: Location::at(20),
                    page: 2
                },
            ]
        );

        for (lines, pages, what) in [
            (vec![3, 9], vec![1], "one page missing"),
            (vec![3], vec![1, 1], "one page too many"),
            (vec![3], vec![], "no pages at all"),
            (vec![], vec![1], "a page with no heading"),
        ] {
            assert!(
                anchors_from(lines, pages).is_empty(),
                "the guard let {what} through"
            );
        }

        // Nothing on either side is not a mismatch; it is a document without
        // headings, and the answer is the same empty list.
        assert!(anchors_from(vec![], vec![]).is_empty());
    }
}
