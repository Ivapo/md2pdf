//! Joins a master and the sections it names into one markdown stream, and
//! translates a line of that stream back into the file the author wrote it in.
//!
//! **The join is markdown, before the parse.** `emit` is handed exactly what it
//! has always been handed: one string, one event stream, one walk. That is why
//! every document-wide mechanism this crate has — the figure numbering, the
//! cross-references, the footnote two-walk, the citation namespace — crosses a
//! file boundary on the day the join lands, with nothing added for any of them.
//! None of them was ever written against a file, and none of them can tell where
//! the bytes came from.
//!
//! **The translation is one function at one boundary, not a file threaded
//! through the walk.** Every line inside the emitter is produced by
//! `emit::line_of` from an offset into the joined string, and the carriers
//! between there and the caller are bare integers. Filling a file in down there
//! would mean widening some twenty of them. Instead this module keeps the map it
//! built while joining, and the five entry points in `crate` resolve through it
//! on the way out.
//!
//! **The same map answers where a section's own neighbours are.** A path named
//! inside `sections/method.md` resolves against `sections/`, and `emit` asks this
//! module for the prefix because this is the one place that knows both the
//! destination and the file it was written in.
//!
//! **A document that names no section has a one-entry map**, so the translation
//! is the identity, nothing is prefixed, and every message and every destination
//! is what it was before this module existed — character for character. That is a
//! property of the arithmetic rather than of a branch anybody has to remember.

use crate::emit::{self, Include};
use crate::{Asset, Error, Location, Result};

/// One run of the joined document that came from one file.
struct Segment {
    /// The 1-based line this run begins on, in the joined document.
    joined_line: usize,
    /// The section file it came from, or `None` for the master.
    file: Option<String>,
    /// The 1-based line that run's first line has in its own file.
    source_line: usize,
}

impl Segment {
    /// The directory this run's file sits in, without a trailing separator.
    ///
    /// Empty for the master, whose paths are already written in the frame a
    /// caller supplies assets in, and empty for a section beside the master —
    /// `[](chapter.md)` has no directory to give.
    fn directory(&self) -> &str {
        self.file
            .as_deref()
            .and_then(|file| file.rsplit_once('/'))
            .map_or("", |(directory, _)| directory)
    }
}

/// Where every line of the joined document came from.
///
/// A master with two markers has five segments — master, section, master,
/// section, master — and a document with none has one. The runs are appended in
/// joined order, so they are sorted by construction and a lookup is one binary
/// search.
pub(crate) struct Sources {
    segments: Vec<Segment>,
}

impl Sources {
    /// The file and line an author would recognise, for a line of the joined
    /// document.
    ///
    /// The last segment beginning at or before `line` is the one it fell in, and
    /// the offset within that segment is preserved. For a one-segment map this
    /// returns `Location::at(line)` unchanged.
    pub(crate) fn locate(&self, line: usize) -> Location {
        let segment = self.segment(line);

        Location {
            file: segment.file.clone(),
            line: segment.source_line + line.saturating_sub(segment.joined_line),
        }
    }

    /// The destination a master would have written for a path named on `line`.
    ///
    /// **A section's neighbours are its own**, and this is where that becomes
    /// true: an image named inside `sections/method.md` means
    /// `sections/figure.png`, so a chapter folder holding its own figures can be
    /// moved, copied or shared whole.
    ///
    /// **It is done here rather than in the caller because the written path is an
    /// identity and not just a lookup.** It is what `emit::image_call` writes into
    /// the Typst source, what `crate::collect` keys `supplied`, `seen` and the
    /// world's `FileId` on, and what both wrappers dedupe on. Two sections in
    /// different folders each naming `figure.png` would emit two byte-identical
    /// calls, and a caller resolving them against different directories would read
    /// the first, skip the second as already seen, and set one figure twice — with
    /// no error and nothing on the page to see. Prefixed here the path is unique by
    /// construction, so there is no collision to detect and none to refuse.
    ///
    /// **A section with no directory of its own prefixes with nothing**, and so
    /// does the master. The idiom matters: a naive `format!("{dir}/{dest}")` yields
    /// `/dot.png`, which `emit::written_shape` refuses as absolute — loud rather
    /// than silent, but wrong. A single-file document has a one-segment map whose
    /// only file is absent, so nothing is prefixed and every golden is
    /// byte-identical: the same arithmetic the inertness of `locate` rests on.
    ///
    /// **The result is normalised, and both branches are**, because the identity
    /// above is a string comparison: `figures/plot.svg` named by the master and
    /// `../figures/plot.svg` named by a section under `sections/` are one file and
    /// must arrive as one key. Normalising only the prefixed branch would leave a
    /// master's own `figures/../plot.svg` un-normalised while a section's
    /// equivalent normalised — the same identity failure in the other direction.
    /// It is also what lets `app/src/watch.rs:classify` go on comparing
    /// `root(document).join(asset)`, which no path carrying a `..` would ever
    /// equal.
    ///
    /// **This stays infallible.** A path that will not normalise falls through as
    /// it was written, so `emit::check_image` is still what refuses it and still
    /// names the author's own file and line.
    pub(crate) fn resolve(&self, line: usize, dest: &str) -> String {
        let landed = match self.segment(line).directory() {
            "" => dest.to_string(),
            directory => format!("{directory}/{dest}"),
        };
        emit::normalise(&landed).unwrap_or(landed)
    }

    /// The segment a line of the joined document fell in.
    ///
    /// The last one beginning at or before it, by binary search over runs that
    /// are sorted by construction.
    fn segment(&self, line: usize) -> &Segment {
        let index = self
            .segments
            .partition_point(|segment| segment.joined_line <= line)
            .saturating_sub(1);
        &self.segments[index]
    }

    /// The same error, naming the author's own file and line.
    ///
    /// **This is the whole relocation boundary.** It runs once, on the way out
    /// of an entry point, and never earlier: `crate::collect` answers with the
    /// earliest refusal by line, and a section-local line would be sorted against
    /// a document it does not belong to.
    pub(crate) fn relocate(&self, mut error: Error) -> Error {
        if let Some(location) = error.location_mut() {
            *location = self.locate(location.line);
        }
        error
    }
}

/// Join a master and its sections, and record where each run came from.
///
/// **The joining is this crate's and never the caller's**, because the map is
/// built out of the boundaries the join creates. A caller that concatenated the
/// sections itself would hand in a string with no map, and every message would
/// name a line of a document nobody wrote.
///
/// A master naming one section twice splices its text twice, which is a document
/// declaring every one of that section's names twice — refused by the walk, in
/// the walk's own words.
pub(crate) fn assemble(md: &str, sections: &[Asset]) -> Result<(String, Sources)> {
    let markers = emit::includes(md);

    // The inertness case, said once and plainly: no marker, no join, no map, and
    // the bytes the caller handed in.
    if markers.is_empty() {
        return Ok((md.to_string(), Sources { segments: master() }));
    }

    let mut joined = String::with_capacity(md.len());
    let mut segments = master();
    let mut cursor = 0usize;

    for marker in &markers {
        let text = section_text(marker, sections)?;

        joined.push_str(&md[cursor..marker.span.start]);
        separate(&mut joined);
        segments.push(Segment {
            joined_line: line_count(&joined),
            file: Some(marker.path.clone()),
            source_line: 1,
        });

        joined.push_str(text);
        separate(&mut joined);

        cursor = resume(md, marker.span.end);
        segments.push(Segment {
            joined_line: line_count(&joined),
            file: None,
            source_line: emit::line_of(md, cursor),
        });
    }

    joined.push_str(&md[cursor..]);

    Ok((joined, Sources { segments }))
}

/// The one segment every map opens with: the master, from its first line.
fn master() -> Vec<Segment> {
    vec![Segment {
        joined_line: 1,
        file: None,
        source_line: 1,
    }]
}

/// The text of the section a marker names, or the refusal it earns.
///
/// Three refusals, and each names the section's own file rather than the
/// master's, because that is the file the author would open.
fn section_text<'a>(marker: &Include, sections: &'a [Asset]) -> Result<&'a str> {
    let Some(supplied) = sections.iter().find(|asset| asset.path == marker.path) else {
        // The one refusal located in the master, because it is the master's
        // marker that is wrong and the section has no lines to point at.
        return Err(Error::MissingSection {
            path: marker.path.clone(),
            location: Location::at(marker.line),
        });
    };

    // The precedent is `crate::bibliography::keys`, for the one other file this
    // crate is handed and must read as text: a borrow, no lossy conversion, and
    // a sentence that names the file.
    let Ok(text) = std::str::from_utf8(&supplied.bytes) else {
        return Err(refuse("section that is not UTF-8 text", &marker.path, 1));
    };

    if opens_a_delimiter(text) {
        return Err(refuse("section with its own frontmatter", &marker.path, 1));
    }

    if let Some(nested) = emit::includes(text).first() {
        return Err(refuse(
            "include inside an included section",
            &marker.path,
            nested.line,
        ));
    }

    Ok(text)
}

/// A refusal naming a section's own file and line.
fn refuse(construct: &str, file: &str, line: usize) -> Error {
    Error::UnsupportedConstruct {
        construct: construct.to_string(),
        location: Location {
            file: Some(file.to_string()),
            line,
        },
    }
}

/// Whether a section opens with a `---` line.
///
/// **This is refused rather than ignored, and both silent corruptions it
/// prevents were measured.** A `---` that lands after a paragraph is a setext
/// heading underline, not a delimiter, so the master's last line and the
/// section's first YAML key both become level-2 headings. A `---` that lands
/// after a blank line is read as a second frontmatter block, whose keys merge
/// into the master's and whose error names the master's line.
///
/// A leading thematic break is caught by the same test and loses nothing: a rule
/// at the very top of a section is a rule against the section above it.
fn opens_a_delimiter(text: &str) -> bool {
    text.lines()
        .next()
        .is_some_and(|first| first.trim_end() == "---")
}

/// Push newlines until the buffer ends in a blank line.
///
/// **The blank line is part of the join and not a tidiness rule**, and this is
/// the measurement it exists for: a file ending `Last line of part one.` joined
/// straight onto one beginning `First line of part two.` is *one* paragraph, the
/// two sentences separated by a soft break. Nothing is raised and nothing on the
/// page shows it. The hazard is not limited to paragraphs — a table's last row
/// followed by a caption line has the same shape, as does any two blocks whose
/// boundary CommonMark decides by a blank line.
fn separate(out: &mut String) {
    if out.is_empty() {
        return;
    }
    while !out.ends_with("\n\n") {
        out.push('\n');
    }
}

/// Where the master resumes after a marker paragraph.
///
/// The start of the next line, whether or not the paragraph's own range carried
/// the newline that ended it. That is what keeps `emit::line_of` in step with the
/// buffer, which by then ends in a blank line of the join's own making.
fn resume(md: &str, end: usize) -> usize {
    if end == 0 || md.as_bytes().get(end - 1) == Some(&b'\n') {
        return end;
    }
    match md[end..].find('\n') {
        Some(offset) => end + offset + 1,
        None => md.len(),
    }
}

/// The 1-based line the next character pushed onto this buffer would land on.
fn line_count(out: &str) -> usize {
    out.bytes().filter(|&byte| byte == b'\n').count() + 1
}
