//! Converts one markdown document into one typeset PDF.
//!
//! This crate holds no OS access. Its API takes a markdown string and the files
//! that string names — its images, its bibliography and its sections — and it
//! returns Typst source, HTML or PDF bytes. The
//! caller does the file I/O. That split is what lets the same crate compile
//! natively and to `wasm32` without a rewrite, and it is why an image arrives as
//! named bytes rather than as a path this crate would have to open.

mod bibliography;
mod diagram;
mod emit;
mod frontmatter;
mod math;
mod sections;

/// The file extensions this dialect accepts as an image.
///
/// **The crate's first re-export**, and it is one because the constant belongs
/// beside `emit::check_image`, the refusal it decides — moving it here would
/// split the table from that refusal. `collect` reads it too, for a URL, whose
/// bytes may hold any format in it. A caller listing the files a
/// document could draw reads this rather than keeping a list of its own.
pub use emit::IMAGE_EXTENSIONS;

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
/// keeps the two apart in the five messages carrying both: `no image file
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

    /// A `mermaid` fence the dialect refuses, or one the renderer could not
    /// draw.
    ///
    /// This is not an `UnsupportedConstruct`, on the argument [`Error::Math`]
    /// was added under: the construct is a fenced block, which the dialect
    /// supports, and what the error names is what the author wrote inside it —
    /// a diagram type outside the list, a directive, a syntax error at its own
    /// line — or where the block stood.
    #[error("diagram error {location}: {problem}")]
    Diagram { location: Location, problem: String },

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

    /// The document names an image by URL, and the caller supplied no bytes for
    /// it.
    ///
    /// A sibling of [`Error::MissingImage`] on the argument
    /// [`Error::MissingBibliography`] was added under: the words are the point.
    /// "No image file supplied" names a file that does not exist. This crate
    /// fetches nothing, so whether a URL is fetched at all is the caller's
    /// decision, and this is what a caller that did not fetch it hears.
    #[error("no image fetched for '{url}' {location}")]
    UnfetchedImage { url: String, location: Location },

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
    /// it, and a line-carrying variant added later cannot slip past the
    /// translation without the compiler saying so.
    pub(crate) fn location_mut(&mut self) -> Option<&mut Location> {
        match self {
            Error::UnsupportedConstruct { location, .. }
            | Error::Frontmatter { location, .. }
            | Error::Math { location, .. }
            | Error::Diagram { location, .. }
            | Error::Name { location, .. }
            | Error::Citation { location, .. }
            | Error::MissingImage { location, .. }
            | Error::UnfetchedImage { location, .. }
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

/// One place where a document names an image: a file, or a URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageRef {
    pub path: String,
    pub location: Location,
}

impl ImageRef {
    /// Whether this image is named by an `http` or `https` URL rather than by a
    /// path beside the document.
    ///
    /// A caller that joins every path onto a directory checks this first.
    /// Otherwise a URL becomes an OS error about a file that was never meant to
    /// exist. It is a method rather than a field, so no caller that builds or
    /// destructures an `ImageRef` breaks.
    pub fn is_url(&self) -> bool {
        emit::is_url(&self.path)
    }
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
/// It lives here rather than in the caller because the column it writes is only
/// telling the truth if both halves come out of one parse with one set of
/// options, and `emit::parser` is `pub(crate)`. Every other home for this
/// function is a second copy of it.
///
/// **The reader left with the page.** `mpdf-011` Phase 2 sent the demo to
/// Letur, so that column is now Letur's own, generated there by
/// `app/tests/page_examples_test.rs`, and nothing in this repository calls this
/// function. `mpdf-011` OQ-7 holds what the missing exercise costs.
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
/// **A URL is a name on this list like any other.** It comes back exactly as the
/// author wrote it, never prefixed or normalised, and [`ImageRef::is_url`] says
/// which entries are URLs. This crate fetches nothing: a caller that fetches a
/// URL supplies its bytes under the URL itself, and a caller that does not
/// supplies nothing for it, which [`md_to_pdf`] refuses as
/// [`Error::UnfetchedImage`].
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
/// **A URL is looked up by the URL itself**, and served under
/// [`emit::remote_name`] of it, which is the name the source asks for. Its
/// ending names no format, so its bytes must hold *some* format in
/// [`IMAGE_EXTENSIONS`]: that is an any-of over [`bytes_match`], the same
/// predicates Typst's own detection reads. No bytes at all is
/// [`Error::UnfetchedImage`].
///
/// **Every insert goes through [`insert`]**, which refuses a `FileId` already
/// held under a different name. Two URLs could share a hash, and a bibliography
/// could be named `remote/<hash>`; serving one set of bytes for both names would
/// be worse than refusing. The refusal joins the contest below at the second
/// name's line, so a refusal the author can act on still wins.
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
                if let Err(error) = insert(&mut map, file_id(&named.path)?, &named.path, bytes) {
                    refusals.push((named.location.line, error));
                }
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

        let url = emit::is_url(&image.path);

        let Some(bytes) = supplied.get(image.path.as_str()) else {
            let error = if url {
                Error::UnfetchedImage {
                    url: image.path.clone(),
                    location: image.location.clone(),
                }
            } else {
                Error::MissingImage {
                    path: image.path.clone(),
                    location: image.location.clone(),
                }
            };
            refusals.push((image.location.line, error));
            continue;
        };

        // For a file, the emitter has already refused every extension outside
        // Typst's own table, so the extension is known here and it alone names
        // the format. A URL names none, and `None` takes whichever format its
        // bytes hold.
        let expected = if url {
            None
        } else {
            Some(emit::extension_of(&image.path).unwrap_or_default())
        };
        let holds_image = match &expected {
            Some(extension) => bytes_match(extension, bytes),
            None => IMAGE_EXTENSIONS
                .iter()
                .any(|extension| bytes_match(extension, bytes)),
        };
        if !holds_image {
            refusals.push((
                image.location.line,
                Error::ImageFormat {
                    path: image.path.clone(),
                    location: image.location.clone(),
                    format: format_name(expected.as_deref().unwrap_or_default()).to_string(),
                },
            ));
            continue;
        }

        let id = if url {
            file_id(&emit::remote_name(&image.path))?
        } else {
            file_id(&image.path)?
        };
        if let Err(error) = insert(&mut map, id, &image.path, bytes) {
            refusals.push((image.location.line, error));
        }
    }

    match refusals.into_iter().min_by_key(|(line, _)| *line) {
        Some((_, error)) => Err(error),
        None => Ok(map
            .into_iter()
            .map(|(id, (_, bytes))| (id, bytes))
            .collect()),
    }
}

/// Put one named file into the world's map, refusing an id another name holds.
///
/// Two names on one `FileId` would be served one set of bytes, silently. That
/// cannot happen between two local paths, whose id is built from the path
/// itself. It can happen between a URL and something else, because a URL's id
/// is built from [`emit::remote_name`]. The same name twice is not a collision,
/// and the first bytes stay.
///
/// The refusal is `Error::Internal` because nothing the author wrote says which
/// name should win, and [`collect`] enters it into its earliest-line contest
/// rather than returning it at once.
fn insert(
    map: &mut HashMap<FileId, (String, Bytes)>,
    id: FileId,
    name: &str,
    bytes: &[u8],
) -> Result<()> {
    match map.get(&id) {
        Some((held, _)) if held != name => Err(Error::Internal(format!(
            "'{name}' and '{held}' would be served as one file"
        ))),
        Some(_) => Ok(()),
        None => {
            map.insert(id, (name.to_string(), Bytes::new(bytes.to_vec())));
            Ok(())
        }
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
/// content when the extension says nothing — is deliberately not mirrored for a
/// file, so an extension outside the table never reaches this function. A URL,
/// which is not a file name, is asked of every extension in the table in turn.
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

/// The licences the six embedded faces are used under, each under the filename
/// it ships as.
///
/// **It lives here rather than in the front end because the front end cannot
/// reach these files.** A published `md2pdf-cli` archive holds nothing of this
/// crate — its own sources, tests, manifests, README and the two root licence
/// files, and no `assets/` — so an `include_str!` reaching into
/// `core/assets/fonts/` resolves in a checkout and fails on the registry. That
/// the fonts are this crate's makes the export right; that the CLI cannot see
/// them makes it forced.
///
/// The text is a compile-time constant, so a caller printing it reads no file
/// and has no path on which to fail. `md2pdf --licenses=full` is the first
/// such caller; any embedder shipping a binary with these faces in it has the same
/// obligation and the same answer.
pub const FONT_LICENSES: &[(&str, &str)] = &[
    ("OFL.txt", include_str!("../assets/fonts/OFL.txt")),
    (
        "GUST-FONT-LICENSE.txt",
        include_str!("../assets/fonts/GUST-FONT-LICENSE.txt"),
    ),
];

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

    /// One `FileId` under two names is refused, and under one name is not.
    ///
    /// Neither collision can be reached from a document without finding two
    /// URLs that share an FNV-1a hash, so the helper is asked directly. The
    /// second half pins that a bibliography and an image naming one file stay
    /// legal, as they were before the helper existed.
    #[test]
    fn one_file_id_under_two_names_is_refused() {
        let id = file_id("remote/e195045359e9f05c").unwrap();
        let mut map = HashMap::new();

        insert(&mut map, id, "https://example.com/figures/plot.png", b"one").unwrap();
        match insert(&mut map, id, "remote/e195045359e9f05c", b"two") {
            Err(Error::Internal(message)) => assert_eq!(
                message,
                "'remote/e195045359e9f05c' and 'https://example.com/figures/plot.png' \
                 would be served as one file"
            ),
            other => panic!("expected an internal error, got {other:?}"),
        }

        insert(
            &mut map,
            id,
            "https://example.com/figures/plot.png",
            b"three",
        )
        .unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map[&id].1.as_slice(), b"one", "the first bytes stay");
    }

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

    /// The two font licences are both there, under the names they ship as.
    ///
    /// A list that named one file twice would still print, still concatenate
    /// and still look like two entries to a caller counting them, so the keys
    /// are asserted and not only the length: the maths font is under different
    /// terms from the other five, and dropping its text is the whole failure
    /// this constant exists to prevent.
    #[test]
    fn both_font_licences_travel_under_their_own_filenames() {
        assert_eq!(
            FONT_LICENSES
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>(),
            vec!["OFL.txt", "GUST-FONT-LICENSE.txt"]
        );

        for (name, text) in FONT_LICENSES {
            assert!(!text.is_empty(), "{name} carries no text");
        }
    }

    // -- `mpdf-012`'s sizing rule, restated for the two tests that hold the
    // looks to it ---------------------------------------------------------

    /// What the frontmatter gains, then the look's caption size in pt, its
    /// margin in cm, and the column count that results.
    const LOOKS: [(&str, f64, f64, f64); 4] = [
        ("columns: 2\n", 9.0, 2.5, 2.0),
        ("", 9.0, 2.5, 1.0),
        ("template: press-release\n", 9.5, 3.0, 1.0),
        ("template: press-release\ncolumns: 2\n", 9.5, 3.0, 2.0),
    ];
    const FLOOR: f64 = 8.0;
    const LABEL_PX: f64 = 16.0;
    // A4, and Typst's own conversion: 72 pt to the inch.
    const PAGE: f64 = 210.0 / 25.4 * 72.0;
    const PT_PER_CM: f64 = 72.0 / 2.54;

    /// The text width and one column's width, with Typst's default gutter,
    /// which neither look sets.
    fn geometry(margin: f64, cols: f64) -> (f64, f64) {
        let text = PAGE - 2.0 * margin * PT_PER_CM;
        (text, (text - (cols - 1.0) * 0.04 * text) / cols)
    }

    /// `mpdf-012` §2's rule, restated: the width the image gets, and whether
    /// its figure floats.
    fn rule(px: f64, captioned: bool, size: f64, margin: f64, cols: f64) -> (f64, bool) {
        let (text, column) = geometry(margin, cols);
        let want = px * size / LABEL_PX;
        let in_column = want.min(column);
        let wide = captioned && cols > 1.0 && size * (in_column / want) < FLOOR;
        (if wide { want.min(text) } else { in_column }, wide)
    }

    /// The width each call carries, which is the `viewBox`'s. The escaped SVG
    /// holds no bare `"`, so the literal closes at the first one after the
    /// root's `</svg>` — merman ends the document with a newline, so not
    /// immediately after it.
    fn widths(source: &str) -> Vec<f64> {
        source
            .lines()
            .filter(|line| line.starts_with("#diagram(bytes(\""))
            .map(|line| {
                let tail = &line[line.rfind("</svg>").expect("the SVG closes")..];
                let rest = &tail[tail.find("\"), ").expect("the literal closes") + 4..];
                rest[..rest.find(',').unwrap()].parse().unwrap()
            })
            .collect()
    }

    /// An image keeps its width as a relative length; the look's is all
    /// absolute, so anything else is a look that sized by something else.
    fn points(value: typst::foundations::Value) -> f64 {
        use typst::foundations::Value;
        match value {
            Value::Relative(width) if width.rel.get() == 0.0 && width.abs.em.get() == 0.0 => {
                width.abs.abs.to_pt()
            }
            Value::Length(width) if width.em.get() == 0.0 => width.abs.to_pt(),
            other => panic!("an image width that is not an absolute length: {other:?}"),
        }
    }

    /// `mpdf-012` Phase 1's gate 2: the look sizes every diagram by the rule,
    /// read off the compiled document rather than judged by eye.
    ///
    /// **Here and not in `tests/`**, because the public API returns only bytes
    /// and the answer is in the introspector: each image's `width` and each
    /// figure's `scope`, as the look's `diagram` set them at layout time. So the
    /// test runs `render`'s own steps inline — the compiled document's type
    /// cannot be written down, so it cannot be handed to a helper — and pins
    /// that type through `typst_pdf::pdf` exactly as `render` does.
    ///
    /// The fixture's six diagrams each sit in a band of their own, measured
    /// under merman's shipped configuration, and **the bands are asserted first,
    /// as preconditions**: a renderer that moved a diagram out of its band then
    /// fails as a precondition, loudly, rather than as a wrong placement. The
    /// rule is then computed here from each look's own literals and compared,
    /// within 0.01 pt, in all four configurations — which is what pins
    /// `press-release`'s literals as well as `article`'s. #3 is the case a look
    /// that dropped the float fails, and #2 the one that dropped the tolerance;
    /// between them and `press-release`'s two columns the floor is pinned to
    /// (7.93, 8.46] pt.
    #[test]
    fn diagrams_are_sized_by_the_looks_rule_in_all_four_configurations() {
        use typst::foundations::{Label, Value};
        use typst::model::FigureElem;
        use typst::utils::PicoStr;
        use typst::visualize::ImageElem;

        const FIXTURE: &str = include_str!("../../tests/fixtures/diagrams.md");
        const CAPTIONED: [bool; 6] = [true, true, true, true, false, true];

        for (look, size, margin, cols) in LOOKS {
            let md = FIXTURE.replacen("---\n", &format!("---\n{look}"), 1);
            let (joined, sources) = sections::assemble(&md, &[]).unwrap();
            let emitted = emit::emit(&joined, &sources).unwrap();
            let px = widths(&emitted.source);
            assert_eq!(
                px.len(),
                6,
                "{look:?}: the fixture drew {} diagrams",
                px.len()
            );

            // The bands, from `article`'s literals at two columns, whatever the
            // look under test: they are properties of the diagrams.
            let (text, column) = geometry(2.5, 2.0);
            let fits = column * LABEL_PX / 9.0;
            let tolerated = column * LABEL_PX / FLOOR;
            let seven = column * LABEL_PX / 7.0;
            let page = text * LABEL_PX / 9.0;
            for (number, holds) in [
                (1, px[0] <= fits),
                (2, fits < px[1] && px[1] <= tolerated),
                (3, tolerated < px[2] && px[2] < seven),
                (4, tolerated < px[3] && px[3] < seven),
                (5, tolerated < px[4]),
                (6, page < px[5]),
            ] {
                assert!(
                    holds,
                    "precondition: diagram {number} at {} px has left its band \
                     (edges {fits:.2}, {tolerated:.2}, {seven:.2}, {page:.2})",
                    px[number - 1]
                );
            }

            let assets = collect(&emitted, &[]).unwrap();
            let world = TypstWorld::new(emitted.source, assets).unwrap();
            let Warned { output, .. } = typst::compile(&world);
            let document = output.unwrap_or_else(|diags| panic!("{look:?}: {}", join(&diags)));
            typst_pdf::pdf(&document, &PdfOptions::default())
                .unwrap_or_else(|diags| panic!("{look:?}: {}", join(&diags)));
            let introspector = document.introspector();

            let expected: Vec<(f64, bool)> = px
                .iter()
                .zip(CAPTIONED)
                .map(|(&px, captioned)| rule(px, captioned, size, margin, cols))
                .collect();

            let images = introspector.query(&ImageElem::ELEM.select());
            assert_eq!(images.len(), 6, "{look:?}: {} images", images.len());
            for (index, (image, (width, _))) in images.iter().zip(&expected).enumerate() {
                let found = points(image.get_by_name("width").unwrap());
                assert!(
                    (found - width).abs() < 0.01,
                    "{look:?}: diagram {} is {found:.3} pt wide, and the rule gives {width:.3}",
                    index + 1
                );
            }

            // Only a captioned diagram is a figure, so the five pair with the
            // captioned rows in order: query order follows the source, floats
            // included.
            let figures = introspector.query(&FigureElem::ELEM.select());
            let captioned: Vec<(usize, bool)> = expected
                .iter()
                .enumerate()
                .filter(|(index, _)| CAPTIONED[*index])
                .map(|(index, (_, wide))| (index + 1, *wide))
                .collect();
            assert_eq!(
                figures.len(),
                captioned.len(),
                "{look:?}: {} figures",
                figures.len()
            );
            for (figure, (number, wide)) in figures.iter().zip(&captioned) {
                let Value::Str(scope) = figure.get_by_name("scope").unwrap() else {
                    panic!("{look:?}: figure {number}'s scope is not a string");
                };
                let want = if *wide { "parent" } else { "column" };
                assert_eq!(scope.as_str(), want, "{look:?}: diagram {number}'s scope");
            }

            // The rule restated above could share a mistake with the look's, so
            // the outcomes the spec names are held as well: in two columns,
            // `article` floats #3, #4 and #6, and `press-release`, whose column
            // is narrower, #2 besides; in one column nothing floats.
            let floated: Vec<usize> = captioned
                .iter()
                .filter(|(_, wide)| *wide)
                .map(|(number, _)| *number)
                .collect();
            let named: &[usize] = match (size == 9.0, cols == 2.0) {
                (true, true) => &[3, 4, 6],
                (false, true) => &[2, 3, 4, 6],
                (_, false) => &[],
            };
            assert_eq!(floated, named, "{look:?}: what floats");

            // The name crossed into the call and reached the figure the look
            // built, which the compile resolving `[](#fig:sequence)` also says.
            let label = Label::new(PicoStr::intern("fig:sequence")).unwrap();
            let named = introspector.query_label(label).unwrap();
            assert!(
                named.is::<FigureElem>(),
                "{look:?}: the name is not on a figure"
            );
            assert_eq!(
                named.get_by_name("kind").unwrap(),
                Value::Func(ImageElem::ELEM.into()),
                "{look:?}: the sequence diagram is not numbered with the images"
            );
        }
    }

    /// `mpdf-012` Phase 2's gate 2: the narrow families take the column, and
    /// the ER diagram is kept in it by the tolerance — read off the compiled
    /// document by Phase 1's method.
    ///
    /// Class and state diagrams fit `article`'s column at caption size, so they
    /// are the in-column branch. The ER fixture is the tolerance's own case: at
    /// 9 pt it is wider than the column, and shrinking it into the column keeps
    /// its 16 px labels at 8 pt or more, so a look that dropped the tolerance
    /// floats it across the page. **The bands are asserted first, as
    /// preconditions**, then every width and scope is held to the rule in all
    /// four configurations, and then the outcomes the gate names: nothing
    /// floats anywhere — the ER diagram, at 404 px, is inside `press-release`'s
    /// narrower band at two columns as well — and in `article` at two columns
    /// the ER diagram is the column's width with its labels between 8 and 9 pt.
    #[test]
    fn the_narrow_families_take_the_column_and_er_the_tolerance() {
        use typst::foundations::Value;
        use typst::model::FigureElem;
        use typst::visualize::ImageElem;

        // The family, its fixture, and how many diagrams it draws.
        const FIXTURES: [(&str, &str, usize); 3] = [
            (
                "class",
                include_str!("../../tests/fixtures/diagrams_class.md"),
                1,
            ),
            (
                "state",
                include_str!("../../tests/fixtures/diagrams_state.md"),
                2,
            ),
            ("ER", include_str!("../../tests/fixtures/diagrams_er.md"), 1),
        ];

        // The bands, from `article`'s literals at two columns, whatever the
        // look under test: they are properties of the diagrams.
        let (_, column) = geometry(2.5, 2.0);
        let fits = column * LABEL_PX / 9.0;
        let tolerated = column * LABEL_PX / FLOOR;

        for (family, fixture, count) in FIXTURES {
            for (look, size, margin, cols) in LOOKS {
                let md = fixture.replacen("---\n", &format!("---\n{look}"), 1);
                let (joined, sources) = sections::assemble(&md, &[]).unwrap();
                let emitted = emit::emit(&joined, &sources).unwrap();
                let px = widths(&emitted.source);
                assert_eq!(px.len(), count, "{family}, {look:?}: {} diagrams", px.len());
                for &px in &px {
                    let holds = match family {
                        "ER" => fits < px && px <= tolerated,
                        _ => px <= fits,
                    };
                    assert!(
                        holds,
                        "precondition: the {family} diagram at {px} px has left its band \
                         (edges {fits:.2}, {tolerated:.2})"
                    );
                }

                let assets = collect(&emitted, &[]).unwrap();
                let world = TypstWorld::new(emitted.source, assets).unwrap();
                let Warned { output, .. } = typst::compile(&world);
                let document =
                    output.unwrap_or_else(|diags| panic!("{family}, {look:?}: {}", join(&diags)));
                typst_pdf::pdf(&document, &PdfOptions::default())
                    .unwrap_or_else(|diags| panic!("{family}, {look:?}: {}", join(&diags)));
                let introspector = document.introspector();

                // Every diagram in these fixtures is captioned, so images and
                // figures pair one to one, in source order.
                let images = introspector.query(&ImageElem::ELEM.select());
                let figures = introspector.query(&FigureElem::ELEM.select());
                assert_eq!(images.len(), count, "{family}, {look:?}: images");
                assert_eq!(figures.len(), count, "{family}, {look:?}: figures");
                for ((image, figure), &px) in images.iter().zip(&figures).zip(&px) {
                    let (width, wide) = rule(px, true, size, margin, cols);
                    let found = points(image.get_by_name("width").unwrap());
                    assert!(
                        (found - width).abs() < 0.01,
                        "{family}, {look:?}: {found:.3} pt wide, and the rule gives {width:.3}"
                    );
                    let Value::Str(scope) = figure.get_by_name("scope").unwrap() else {
                        panic!("{family}, {look:?}: the scope is not a string");
                    };
                    let want = if wide { "parent" } else { "column" };
                    assert_eq!(scope.as_str(), want, "{family}, {look:?}: the scope");
                    // The rule restated above could share a mistake with the
                    // look's, so the outcome is held as a literal too.
                    assert_eq!(scope.as_str(), "column", "{family}, {look:?}: it floated");
                }

                // The gate's own clause. The label size is read back from the
                // width the look gave the image, not from the rule.
                if family == "ER" && size == 9.0 && cols == 2.0 {
                    let found = points(images[0].get_by_name("width").unwrap());
                    assert!(
                        (found - column).abs() < 0.01,
                        "the ER diagram is {found:.3} pt, not the column's {column:.3}"
                    );
                    let label = found / px[0] * LABEL_PX;
                    assert!(
                        (8.0..9.0).contains(&label),
                        "the ER diagram's labels set at {label:.2} pt"
                    );
                }
            }
        }
    }
}
