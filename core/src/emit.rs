//! Walks the pulldown-cmark event stream and emits Typst markup.
//!
//! Two rules keep the dialect honest. The emitter escapes every character that
//! Typst markup mode interprets, so body text reaches the PDF verbatim. And a
//! construct outside the dialect is an error that names the construct and its
//! line, never a silent drop.
//!
//! Nested blocks need indentation, and the walk writes into a stack of buffers
//! to get it. A list item and a block quote each open a buffer and indent it as
//! they close, so nothing is ever indented while it is being written. That is
//! what lets `escape_into` keep reading an un-indented line, and it is why a
//! code block, which reaches Typst as one line holding a string literal, cannot
//! be corrupted by the indentation around it.
//!
//! One document takes two walks of the same stream. Typst takes a footnote's
//! content at the reference site, and a markdown definition may sit after the
//! reference that cites it, so `collect_definitions` translates every
//! definition first and `emit` then writes the document.

use std::collections::{HashMap, HashSet};
use std::ops::Range;

use pulldown_cmark::{
    Alignment, BrokenLink, CodeBlockKind, CowStr, Event, LinkType, Options, Parser, Tag, TagEnd,
};
use typst::syntax::{PathError, VirtualPath};
use unicase::UniCase;

use crate::frontmatter::{self, Equations, Frontmatter};
use crate::math;
use crate::sections::Sources;
use crate::{BibliographyRef, Error, ImageRef, Location, Result};

/// Characters that Typst markup mode interprets inside a text run.
///
/// The spec names all of these except `~` and `/`. `~` is a non-breaking space
/// in Typst, and `//` opens a line comment, so both would change the rendered
/// text if they passed through unescaped.
const SPECIAL: &[char] = &[
    '\\', '#', '$', '*', '_', '`', '@', '<', '>', '[', ']', '~', '-', '+', '=', '/',
];

/// The file the math prelude is bound under, and the name the import writes.
///
/// It sits beside `main.typ` in the same virtual root the looks resolve in, so
/// the import needs no path of its own. `core/src/lib.rs` binds the bytes.
pub(crate) const PRELUDE_NAME: &str = "math.typ";

/// What the prelude defines, in the order the import lists them.
///
/// The names are written here rather than imported with `*` because Typst
/// searches user scopes before the library: a glob would shadow `image`, `table`
/// and `raw` — all of which the emitter calls — for the whole document. Naming
/// them also keeps this list and `core/assets/math.typ` in step, since a name in
/// one and not the other fails the compile.
const PRELUDE_NAMES: &str =
    "aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix";

/// The file extensions Typst's own `determine_format_from_path` names.
///
/// An extension outside this table is a construct error at the image arm, so
/// everything that survives the walk has an extension the table names. That is
/// what lets the pre-compile check say "the extension decides the format" and
/// mean it.
///
/// **It is public, and `crate::IMAGE_EXTENSIONS` re-exports it.** A caller that
/// wants to know which files this dialect will accept as a figure — the desktop
/// app's file panel is the one that asked — must read this list rather than
/// write a second one beside it, or the two drift and a perfectly legal `.jpg`
/// becomes invisible to the window that would compile it.
pub const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg", "svgz", "pdf"];

/// One list the walk is inside.
struct ListFrame {
    /// The next item's number, for an ordered list; `None` for a bullet list.
    /// Every item carries its own number, so a start other than `1` needs no
    /// separate mechanism and nothing depends on how Typst continues a counter.
    next: Option<u64>,
    /// Whether the items are separated by a blank line in the output.
    loose: bool,
    /// Each item, already laid out under its marker.
    items: Vec<String>,
}

/// The table the walk is inside.
///
/// A GFM table never nests, so one frame serves the whole walk, as one slot
/// serves a code block. The parser pads a short row with empty cells and drops
/// the excess, following GFM, so every row arrives with the header's cell count
/// and the emitter counts nothing itself.
struct TableFrame {
    /// One entry per column, from the delimiter row.
    align: Vec<Alignment>,
    /// The cells of the row being read.
    cells: Vec<String>,
    /// The header row, filled when the head closes.
    header: Vec<String>,
    /// Every body row, each cell already translated.
    rows: Vec<Vec<String>>,
}

/// What the walk is directly inside, for the one question that needs the answer:
/// whether a paragraph is a list item's own child.
enum Container {
    Item,
    Quote,
}

/// The alt text being flattened out of one image's content.
///
/// CommonMark reads alt text as the plain text of everything inside the image,
/// which is what pulldown-cmark's own HTML renderer implements, and Typst's
/// `alt` is a plain string too. So the walk collects rather than emits between
/// the image's two events.
struct AltCapture {
    /// The destination, exactly as the markdown wrote it.
    path: String,
    /// Whether the image opened its paragraph. Half of the standalone test; the
    /// next event settles the other half.
    opened: bool,
    /// The text collected so far.
    text: String,
    /// How many nested images are open. A nested image flattens by the same
    /// rule, so its own end event must not close the capture.
    depth: usize,
}

/// A block the walk has already written that a caption can attach to, and where
/// it sits.
///
/// Three constructs reach this: a standalone image, a table and a code block.
/// A caption looks *back* at one; nothing is ever held forward waiting for a
/// caption. The flush timing in `step` is load-bearing for `Walk::finish`, for
/// `collect_definitions`, and for `Walk.para`'s offset, so it does not move, and
/// a rewrite of what was written costs none of them anything.
///
/// The record is verified where it is spent rather than invalidated from the
/// walk's other arms, which rests on a property of the whole file: every write
/// into a `bufs` frame is an append. The splice below is the one exception, and
/// it updates this record in the same breath.
struct Figure {
    /// How deep the buffer stack was when the call was written.
    depth: usize,
    /// Where the call begins in that frame.
    start: usize,
    /// Exactly what stands there, checked before the point is spent.
    written: String,
    /// The same call without its `#`, which is what a `#figure(…)` wraps.
    ///
    /// Stale once `captioned` is set, and unread after that: one construct
    /// takes one caption, so a spent point is never wrapped a second time.
    body: String,
    /// Whether a caption has already spliced here.
    ///
    /// This does not fall out of the three checks in [`Figure::live`]. A
    /// spliced region carries `#figure(…)`, which the content check accepts, so
    /// without this flag a second `: ` line would print as prose where the
    /// dialect names an error.
    captioned: bool,
}

impl Figure {
    /// Whether the recorded point is still the thing it recorded.
    ///
    /// Three conditions, all read here and none maintained at a distance: the
    /// same frame is on top, the region still carries the call it recorded, and
    /// everything after it is the separator newlines the paragraph arms push.
    /// Anything written in between — a heading, a rule, prose, an inline image —
    /// fails the third, and a buffer frame opened in between fails the first.
    fn live(&self, bufs: &[String]) -> bool {
        bufs.len() == self.depth
            && bufs[self.depth - 1]
                .get(self.start..)
                .and_then(|rest| rest.strip_prefix(self.written.as_str()))
                .is_some_and(|tail| tail.bytes().all(|byte| byte == b'\n'))
    }
}

/// A display equation the walk has already written, and where it sits.
///
/// The same argument as [`Figure`] with the trailing-separator allowance
/// dropped, because a label is *adjacent* where a caption is a paragraph away:
/// the name rides the closing `$$`, so the parser hands the walk the equation
/// and then the group as the very next event, in the same paragraph. A record
/// is spent only where it still stands at the end of the frame it was written
/// in, so a `SoftBreak`'s newline between them is enough to leave the group the
/// prose it is.
///
/// Nothing is held forward waiting for a name, for the reason `Figure` records
/// at length: rounds 2 to 4 of Phase 1 each measured a different construct
/// broken by holding a call across an event, and nothing here needs a hold,
/// since the equation is already written when the name arrives.
struct Equation {
    /// How deep the buffer stack was when the equation was written.
    depth: usize,
    /// Where it begins in that frame.
    start: usize,
    /// Exactly what stands there, checked before the point is spent.
    written: String,
}

impl Equation {
    /// Whether the recorded span is still the last thing in its own frame.
    fn live(&self, bufs: &[String]) -> bool {
        bufs.len() == self.depth
            && bufs[self.depth - 1].get(self.start..) == Some(self.written.as_str())
    }
}

/// The caption paragraph being collected, over the figure it will attach to.
struct Caption {
    /// The line the `: ` sits on, which every refusal here names.
    line: usize,
    /// The caption's own text, unescaped, for the `{#name}` test.
    ///
    /// The escaped form in the buffer cannot serve: `escape_into` turns the `#`
    /// of a name into `\#`, and the check is on what the author typed.
    text: String,
}

/// One caption paragraph, read.
///
/// What a `: ` line says once its `{#name}` group has left the text: the
/// escaped markup a `caption:` argument takes, and the name the label takes.
/// A single construct spends this at once; a group holds one until its closer,
/// which is the whole of why the reading is a step of its own.
struct Words {
    /// The line the `: ` sits on, which a refusal over the caption names.
    line: usize,
    /// The caption's markup, escaped, with the name group gone.
    content: String,
    /// The name that rode the line's end.
    name: Option<String>,
}

/// The `:::` group the walk is inside, and what it has collected.
///
/// A group is a figure with more than one member, which no arrangement of the
/// `: ` marker can express: that marker attaches to the construct immediately
/// above it, so nothing spells "these two images are one figure, with one
/// number and one caption".
///
/// It needs no second notion of what a member is. [`Figure`] already records
/// every captionable construct's bare call, so a group takes a `body` each time
/// a record is made while it is open, and the three member kinds are the three
/// a caption reaches by construction.
struct Group {
    /// The line the opener sits on, which every refusal over the group names.
    line: usize,
    /// How deep the buffer stack was when the opener stood.
    depth: usize,
    /// Where the opener's paragraph begins in that frame, which the closer
    /// truncates back to.
    start: usize,
    /// Each member's bare call, in the order the author wrote them.
    members: Vec<String>,
    /// The caption the group's own `: ` line carried.
    ///
    /// It is the last block before the closer, so a member taken while this
    /// stands is refused: a `: ` line after a member is exactly the spelling a
    /// subcaption will want, and letting it through as prose would ship a
    /// meaning a later phase would have to take back.
    caption: Option<Words>,
}

/// One name a walk declared, and what declared it.
struct Declaration {
    name: String,
    /// The line the caption or the closing `$$` sits on.
    line: usize,
    /// Whether a display equation declared it, rather than a caption.
    ///
    /// A reference to an equation fails the compile unless the document numbers
    /// its equations, and Typst's message names neither line nor key. This is
    /// what lets `check_references` name both.
    equation: bool,
}

/// The names one walk declared, and the names it referenced.
///
/// A `Vec` on both sides rather than a map and a set, and that is a decision
/// rather than an oversight: where several references are refused, the error
/// is the one on the earliest line, and "the first" out of a set varies between
/// runs. The document holds a handful of names, so the linear scans cost
/// nothing a hash would save.
#[derive(Default)]
struct Names {
    /// Each declared name, the line that declared it, and what did.
    declared: Vec<Declaration>,
    /// Each referenced name and the line its link sits on.
    referenced: Vec<(String, usize)>,
    /// Each citation key this walk wrote, and the line its `[@…]` sits on.
    ///
    /// It travels with the two above rather than beside them, because it needs
    /// exactly what they need: a walk of a footnote definition collects these
    /// and is then discarded, so a citation that appears only inside a footnote
    /// must reach the document's walk on the [`Body`] its reference splices in.
    ///
    /// **The refusal it feeds cannot be raised where the citation is written.**
    /// `collect_definitions` never parses the frontmatter — the metadata block
    /// sits outside every definition — so its `front` is the default one, and a
    /// missing-bibliography test inside the walk would refuse a citation in a
    /// footnote of a document that names a bibliography perfectly well.
    cited: Vec<(String, usize)>,
}

/// The link the walk is inside, held while its text is collected.
///
/// Whether a link's text is empty is knowable only at its end event — the same
/// shape as an image's form, which `pending` settles one event late. So the
/// start arm parks the destination here and opens a frame, and the end arm
/// writes either the reference or the link.
struct LinkFrame {
    /// The destination the emitter would write, scheme included.
    url: String,
    /// The line the link opens on, which a refusal over it names.
    line: usize,
    /// Which shape of link the parser read.
    ///
    /// A citation arrives as an unresolved shortcut or collapsed reference whose
    /// destination the callback wrote, and that is half the discriminator. It
    /// rides `Tag::Link` at `Start` while the emitter decides at
    /// `End(TagEnd::Link)`, so it is carried here rather than read at the end.
    link_type: LinkType,
}

/// Everything one walk of the event stream carries from event to event.
///
/// These were locals in one loop body. Grouping them is what lets that body be
/// a function of its own, which a walk that must catch an error, keep it, and
/// carry on to the next event needs: a `?` inside a loop body cannot do it.
struct Walk {
    /// The buffer stack. The base buffer is the document body; a list item, a
    /// block quote and a table cell each push one of their own and pop it as
    /// they close.
    bufs: Vec<String>,
    /// What the walk is directly inside.
    containers: Vec<Container>,
    /// Every list the walk is inside, outermost first.
    lists: Vec<ListFrame>,
    /// The code block the walk is inside: its language tag and its content.
    code: Option<(Option<String>, String)>,
    /// The table the walk is inside.
    table: Option<TableFrame>,
    /// Whether the walk is inside the frontmatter block.
    in_metadata: bool,
    /// The frontmatter text, laid out at the lines the author wrote it on.
    ///
    /// **The layout is why this is padded rather than concatenated.** The
    /// accumulator is never cleared, so a document carrying a second `---` block
    /// hands `frontmatter::parse` both blocks at once — which is what makes a key
    /// repeated across them a duplicate, and the behaviour a document may rely
    /// on. Concatenated end to end, the second block's keys sit at the first
    /// block's line numbers and every message about them names a line the author
    /// cannot find. Padded with the blank lines that stood between them — which
    /// `frontmatter::parse` skips without consuming a key — every key is reported
    /// where it was written.
    meta: String,
    /// Where that text starts, which is the line `meta`'s first line stands on.
    meta_offset: Option<usize>,
    /// The frontmatter this document carries.
    front: Frontmatter,
    /// Every image the walk has met, in the order it met them.
    images: Vec<ImageRef>,
    /// The line every heading the walk has met sits on, in that order.
    ///
    /// Nothing is written into the output for these — the heading is already
    /// there, which is the whole reason it is the anchor. A marker the emitter
    /// emitted would move every shipped golden file for a reason that has
    /// nothing to do with what the document says.
    ///
    /// A definition's walk collects these too and is then discarded, so a
    /// heading inside a footnote definition never reaches the document's list
    /// while its spliced content still typesets one. `md_to_pdf_with_anchors`
    /// guards that mismatch by count.
    headings: Vec<usize>,
    /// The alt text being flattened out of one image's content.
    alt: Option<AltCapture>,
    /// An image call waiting for the next event to settle its form.
    pending: Option<(String, bool)>,
    /// The last captionable block this walk wrote, and where it stands.
    ///
    /// A standalone image, a table or a code block, and **the last one written
    /// rather than the last one of its kind** — which is what makes a caption
    /// reach the construct directly above it and no other. A caption line
    /// splices it into a `#figure(…)`. Nothing else reads it, so a document
    /// with no caption in it carries this and writes exactly what it always
    /// wrote.
    figure: Option<Figure>,
    /// The display equation this walk last wrote, and where it stands.
    ///
    /// A name written immediately after it becomes its Typst label. Nothing
    /// else reads it, so a document that names no equation carries this and
    /// writes exactly what it always wrote.
    equation: Option<Equation>,
    /// The caption being collected, while its paragraph is open.
    caption: Option<Caption>,
    /// The `:::` group the walk is inside, while it is open.
    ///
    /// **An opener and its closer sit in the same frame, not merely at the same
    /// depth.** `- :::` / image / `- :::` puts the two delimiters in different
    /// list items and both at depth 2, so a closer that compared depths alone
    /// would truncate a frame its group never opened. The item and quote arms
    /// retire a group whose frame has gone, which keys this to the frame itself.
    group: Option<Group>,
    /// The link the walk is inside, while its text is collected.
    link: Option<LinkFrame>,
    /// Every figure name this walk declared, and every one it referenced.
    ///
    /// A definition's walk collects these and is then discarded, so they travel
    /// with the body the way its images and its math flag do — otherwise a name
    /// declared inside a footnote would be met only by the walk that is thrown
    /// away, and a reference Typst resolves perfectly well would be refused.
    names: Names,
    /// Where the open paragraph began in the buffer that holds it.
    para: Option<usize>,
    /// Where the open paragraph ends in the source.
    ///
    /// A `:::` delimiter has to be the whole of its paragraph, and whether a
    /// text run is the whole of one is a later event — the trap [`Figure`]
    /// records at length. The paragraph's own span answers it now, so nothing
    /// is held across an event to learn it.
    para_end: Option<usize>,
    /// Whether this walk wrote any math.
    ///
    /// The prelude is imported only by a document that has one, which is what
    /// keeps every shipped golden file two lines long. The walk finishes before
    /// the header is written, so the answer is known in time.
    math: bool,
}

impl Walk {
    fn new() -> Self {
        Self {
            bufs: vec![String::new()],
            containers: Vec::new(),
            lists: Vec::new(),
            code: None,
            table: None,
            in_metadata: false,
            meta: String::new(),
            meta_offset: None,
            front: Frontmatter::default(),
            images: Vec::new(),
            headings: Vec::new(),
            alt: None,
            pending: None,
            figure: None,
            equation: None,
            caption: None,
            group: None,
            link: None,
            names: Names::default(),
            para: None,
            para_end: None,
            math: false,
        }
    }

    /// The refusal a walk that ended with a group still open owes.
    ///
    /// Both walks end, and both reach this. `emit` runs it after its loop;
    /// `collect_definitions` runs it at a definition's own end event, which
    /// OQ-7's finding makes reachable — a caption inside a definition already
    /// works, so a group inside one does too, and the document's walk skips
    /// that region entirely.
    fn unclosed(&self) -> Result<()> {
        match &self.group {
            Some(open) => Err(Error::UnsupportedConstruct {
                construct: "figure group the document never closes".to_string(),
                location: Location::at(open.line),
            }),
            None => Ok(()),
        }
    }

    /// The markup this walk wrote, and every image it met.
    ///
    /// Every image sits inside a block whose end event follows it, so the
    /// stream never runs out with a call still waiting. This writes one anyway,
    /// in the inline form, rather than let a future change drop an image
    /// silently.
    fn finish(mut self) -> (String, Vec<ImageRef>) {
        if let Some((call, _)) = self.pending.take() {
            write_image(&mut self.bufs, &call, false);
        }
        let body = self
            .bufs
            .pop()
            .expect("the document body outlives the walk");
        (body, self.images)
    }
}

/// The parser options every walk reads.
///
/// One builder for all of them: two walks of the same stream that disagreed
/// about the options would disagree about what the document says.
pub(crate) fn options() -> Options {
    let mut options = Options::empty();
    // The parser is what recognises the frontmatter block, so nothing strips it
    // from the input and every reported line number stays true to the user's
    // file. `frontmatter.rs` then reads the text between the delimiters.
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    // A pipe table is a GFM extension rather than CommonMark, so the parser
    // reads one only with this option on. Without it the pipes would reach the
    // PDF as prose, which is the silent flattening the dialect refuses.
    options.insert(Options::ENABLE_TABLES);
    // A footnote is a GFM extension too, and the same argument applies to it:
    // without this option `[^1]` and `[^1]: The source.` are ordinary text, and
    // the escape rule prints the brackets on the page.
    options.insert(Options::ENABLE_FOOTNOTES);
    // The last three carry the same argument, and each closed one arm of
    // `describe` that nothing could reach. Strikethrough and math are both in
    // the dialect now, so only the task list marker still names itself as an
    // error. Without these options `~~x~~`, `- [ ] a`, `$x$` and `$$x$$` arrive
    // as text and the escape rule prints their markers on the page — which is
    // the silent flattening the dialect refuses, whether the construct is one
    // it converts or one it rejects.
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_MATH);
    options
}

/// The broken-link callback every walk reads, as a plain function pointer.
///
/// A `fn` satisfies `BrokenLinkCallback`, so the parser below is a concrete
/// type and nothing is boxed. The lifetime is the source's: the reference the
/// callback hands back borrows the markdown it was read from.
pub(crate) type Citations<'a> = fn(BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)>;

/// The parser every walk reads, options and callback together.
///
/// One constructor for all of them, on the argument [`options`] already carries
/// one builder under: two walks of the same stream that disagreed about the
/// parse would disagree about what the document says. **Both of this file's
/// walks take it** — `collect_definitions` and `emit` — or a citation inside a
/// footnote definition would print literally while the same text in the body
/// cited. So does [`crate::md_to_html`], which is what keeps the demo page's two
/// columns coming out of one parse.
pub(crate) fn parser(md: &str) -> Parser<'_, Citations<'_>> {
    Parser::new_with_broken_link_callback(md, options(), Some(citation_reference))
}

/// One include marker, as a master writes it.
///
/// **The shape is a paragraph whose entire content is one empty-text link whose
/// destination names a markdown file** — `[](sections/method.md)`. `mpdf-005`
/// reserved the empty-text link and claimed one destination under it, `#name`,
/// on the rule that "the empty text is what makes it a reference"; this is the
/// second destination claimed under that same rule, and claiming it cost no
/// document anything. Measured before it was claimed, `[](sections/intro.md)`
/// emitted `#link("sections/intro.md")[]` — a link with no content, which
/// typesets as nothing at all.
///
/// A link carrying text is untouched whatever its destination, exactly as it has
/// been since `mpdf-005`, so `[the method](sections/method.md)` stays the link it
/// is today.
pub(crate) struct Include {
    /// The destination exactly as the master wrote it, which is the name the
    /// caller's asset must carry.
    pub path: String,
    /// The 1-based line of the marker in the file this scan read.
    pub line: usize,
    /// The marker paragraph's byte range in that file — what the join splices
    /// the section's own text over.
    pub span: Range<usize>,
}

/// Every include marker in one file, in the order it wrote them.
///
/// **This is a scan of its own rather than a branch inside [`step`].** The walk
/// only ever sees the *joined* document, where a master's markers have already
/// been replaced by the text they named, so a marker surviving to the walk could
/// only have been written inside a section — which `crate::sections::assemble`
/// refuses by name. Nothing here touches the standalone test at the pending-image
/// flush, which `mpdf-005` §2 calls "the one discrimination every image in the
/// dialect flows through" and scoped a whole phase around not disturbing.
///
/// **A marker must be a whole paragraph, at the top level, beginning its own
/// line.** A marker inside a sentence would splice headings and tables into the
/// middle of a clause. A marker inside a block quote, a list item or a footnote
/// definition would be worse still — the join copies bytes over the paragraph's
/// range, and a chapter spliced inside a `> ` context walks straight out of it.
/// Every link this scan declines stays exactly the link it is today.
pub(crate) fn includes(md: &str) -> Vec<Include> {
    let events: Vec<(Event<'_>, Range<usize>)> = parser(md).into_offset_iter().collect();
    let mut found = Vec::new();
    let mut depth = 0usize;

    for (index, (event, range)) in events.iter().enumerate() {
        match event {
            // A paragraph never nests, so it is counted on neither side and
            // `depth` is exactly the number of containers around this one.
            Event::Start(Tag::Paragraph) => {
                if depth == 0
                    && at_line_start(md, range.start)
                    && let Some(path) = lone_markdown_link(&events[index + 1..])
                {
                    found.push(Include {
                        path,
                        line: line_of(md, range.start),
                        span: range.clone(),
                    });
                }
            }
            Event::End(TagEnd::Paragraph) => {}
            Event::Start(_) => depth += 1,
            Event::End(_) => depth = depth.saturating_sub(1),
            _ => {}
        }
    }

    found
}

/// The destination, where the rest of this paragraph is one empty-text link
/// naming a markdown file and nothing else.
///
/// The three events are the whole test: a link start, a link end with no text
/// event between them, and the paragraph's own end. A single `Event::Text` in
/// the middle — which is what `[the method](x.md)` produces — fails the pattern,
/// and so does a second inline anything.
fn lone_markdown_link(rest: &[(Event<'_>, Range<usize>)]) -> Option<String> {
    let [
        (
            Event::Start(Tag::Link {
                dest_url, title, ..
            }),
            _,
        ),
        (Event::End(TagEnd::Link), _),
        (Event::End(TagEnd::Paragraph), _),
        ..,
    ] = rest
    else {
        return None;
    };

    if !title.is_empty() {
        return None;
    }

    // The same path rule the image arm and the `bibliography` key read, so a
    // destination this dialect would refuse as a path is not a marker at all —
    // it stays the link it has always been.
    let dest = dest_url.as_ref();
    if portable_path(dest).is_err() {
        return None;
    }
    if !extension_of(dest).is_some_and(|extension| extension.eq_ignore_ascii_case("md")) {
        return None;
    }

    Some(dest.to_string())
}

/// Whether a byte offset is the first byte of its line.
///
/// A paragraph indented by one to three spaces is still a paragraph, and
/// splicing a section's first line in after that indent would set it apart from
/// its own second line. Such a marker is declined instead.
fn at_line_start(md: &str, offset: usize) -> bool {
    offset == 0 || md.as_bytes().get(offset - 1) == Some(&b'\n')
}

/// Claim a broken shortcut reference that begins `@` or `-@`, and no other.
///
/// **A `[@key]` is not a link until this makes it one.** A CommonMark shortcut
/// reference is a link only where a matching reference definition exists, and a
/// citation never has one: without this callback `See [@smith2020] ok.` parses
/// to five `Event::Text` runs and no `Tag::Link` at all, so the walk would never
/// see the construct. Stitching those runs back together was the alternative and
/// is refused — it would put a second, hand-written inline parser beside
/// `pulldown-cmark`, which is the thing a parser was chosen to avoid.
///
/// **The reference itself is the destination**, never an empty string: the
/// `Tag::Link` arm errors on an empty one, and the value also reaches
/// `crate::md_to_html`, where a citation renders as `<a href="@k">@k</a>` — which
/// is honest for a writer that has no notion of citations.
///
/// **`-@` is not decoration.** It is Pandoc's suppressed-author form, which the
/// dialect refuses by name; under an `@`-only predicate `[-@k]` would stay five
/// text runs the emitter never sees and would reach the page as `\[\-\@k\]` — a
/// refusal the dialect promises and the parse could not deliver.
///
/// Returning `None` leaves the source exactly as it is, which is what keeps
/// `an [ open bracket, a ] close bracket`, `[see @k]` and `[a@b.com]` the
/// literal text they have always been.
fn citation_reference<'a>(link: BrokenLink<'a>) -> Option<(CowStr<'a>, CowStr<'a>)> {
    is_citation(&link.reference).then_some((link.reference, CowStr::Borrowed("")))
}

/// Whether a reference is one this dialect reads as a citation.
///
/// Read twice over the same string — once by the callback above, once by the
/// link's end arm over the destination the callback wrote — so the parse and the
/// emitter cannot disagree about what a citation is.
fn is_citation(reference: &str) -> bool {
    reference.starts_with('@') || reference.starts_with("-@")
}

/// One footnote label, folded the way the parser folds it.
///
/// pulldown-cmark keys its own label map by `UniCase`, so `[^A]` cites a
/// definition written `[^a]:`, and the events carry each label's original
/// spelling. Everything here that touches a label — the map, the citedness
/// test, the generated name — runs over that same equivalence. Keying by the
/// raw spelling would miss on valid input.
type Label = UniCase<String>;

fn label_of(text: &str) -> Label {
    UniCase::new(text.to_string())
}

/// One footnote definition's translation, and everything that travels with it.
///
/// The three parts beside the content are here for one reason: this walk's own
/// `Walk` is thrown away, and the document's walk never enters a definition's
/// region. So a formula that appears only inside a footnote would reach the page
/// with no prelude imported, a file named only there would never join the
/// shopping list, and a figure named only there would be a name `core` never
/// saw — refusing a reference Typst resolves perfectly well.
struct Body {
    content: String,
    images: Vec<ImageRef>,
    math: bool,
    names: Names,
}

/// What one walk of the definitions found.
#[derive(Default)]
struct Definitions {
    /// Each definition's translation, or the first error it produced. A label
    /// defined twice is held once: the second definition is refused where it
    /// stands.
    bodies: HashMap<Label, Result<Body>>,
    /// Every label that a reference outside a definition names.
    cited: HashSet<Label>,
}

impl Definitions {
    /// One definition's translation.
    ///
    /// Every reference the parser emits has a definition somewhere in the
    /// document — an unresolved one produces no event at all and stays literal
    /// text — so a lookup at a reference finds one, and so does a lookup at a
    /// region the walk has just entered.
    fn body(&self, label: &Label) -> &Result<Body> {
        self.bodies
            .get(label)
            .expect("the parser resolves every reference against a definition")
    }
}

/// What the document's own walk carries about footnotes.
struct Notes<'a> {
    /// What the walk of the definitions found.
    found: &'a Definitions,
    /// The definitions this walk has passed, so a second one for the same
    /// label is refused at the line it sits on.
    seen: HashSet<Label>,
    /// The name each label was given, by first reference.
    numbers: HashMap<Label, usize>,
    /// Whether the walk is inside a definition's region.
    skipping: bool,
}

impl Notes<'_> {
    /// Settle what a definition's region owes, then skip it.
    ///
    /// Three shapes are errors, per the escape-and-reject rule. A second
    /// definition for one label would lose a body, and choosing between two
    /// bodies is a guess the dialect does not make. A definition no reference
    /// cites would reach no page, and content that vanishes is what the rule
    /// exists to prevent. Last, the definition's own translation may have
    /// failed, and this is the position that error belongs to.
    ///
    /// They run in the order of the lines they name: the first two name this
    /// definition's own line, and a kept error names a line inside the region.
    fn enter(&mut self, label: Label, line: usize) -> Result<()> {
        let refuse = |construct: &str| {
            Err(Error::UnsupportedConstruct {
                construct: construct.to_string(),
                location: Location::at(line),
            })
        };

        if !self.seen.insert(label.clone()) {
            return refuse("footnote definition for a label already defined");
        }
        if !self.found.cited.contains(&label) {
            return refuse("footnote definition that no reference cites");
        }
        if let Err(error) = self.found.body(&label) {
            return Err(error.clone());
        }

        self.skipping = true;
        Ok(())
    }
}

/// Which of the two walks is running.
enum Mode<'a, 'b> {
    /// The walk of the definitions, inside one definition's region.
    Definition,
    /// The walk of the document, with what the first walk found.
    Document(&'a mut Notes<'b>),
}

/// Translate every footnote definition, and record what the document cites.
///
/// This walk enters the definitions and nothing else. Outside a region two
/// events matter: the one that opens a region, and a reference, which is what
/// makes a definition cited.
///
/// It never raises. A region whose translation fails keeps that error, and the
/// document's walk reports it where the region stands, so the first error in
/// document order is still the one the user reads — a frontmatter error
/// included, which is parsed in that second walk and nowhere here.
fn collect_definitions(md: &str, sources: &Sources) -> Definitions {
    let mut found = Definitions::default();
    let mut walk = Walk::new();
    let mut open: Option<Label> = None;
    let mut failure: Option<Error> = None;

    for (event, range) in parser(md).into_offset_iter() {
        if open.is_none() {
            match &event {
                Event::Start(Tag::FootnoteDefinition(label)) => {
                    open = Some(label_of(label));
                    walk = Walk::new();
                    failure = None;
                }
                Event::FootnoteReference(label) => {
                    found.cited.insert(label_of(label));
                }
                _ => {}
            }
            continue;
        }

        if matches!(event, Event::End(TagEnd::FootnoteDefinition)) {
            let label = open.take().expect("a region is open");
            // A group left open inside a definition is refused where this walk
            // ends, which is here: the document's walk never enters the region,
            // so nothing else would ever meet it.
            let body = match failure.take().or_else(|| walk.unclosed().err()) {
                Some(error) => Err(error),
                None => {
                    let math = walk.math;
                    let names = std::mem::take(&mut walk.names);
                    let (content, images) = std::mem::replace(&mut walk, Walk::new()).finish();
                    Ok(Body {
                        content: content.trim_matches('\n').to_string(),
                        images,
                        math,
                        names,
                    })
                }
            };
            // The first definition of a label is the one held. A second one is
            // an error the document's walk raises where that second one sits.
            found.bodies.entry(label).or_insert(body);
            continue;
        }

        // Once a region has failed, the rest of it is skipped: the error it
        // already holds is the first one in that region, and that is the one
        // to report.
        if failure.is_none()
            && let Err(error) = step(&mut walk, md, event, range, Mode::Definition, sources)
        {
            failure = Some(error);
        }
    }

    found
}

/// What one walk of a document produced.
///
/// A record rather than a tuple because six things travel and four callers each
/// want a different one. Every field comes out of the same walk that wrote the
/// source, which is what stops the markup and the shopping list disagreeing
/// about which paths the dialect accepts.
pub(crate) struct Emitted {
    /// The Typst markup, header and body and reference list.
    pub source: String,
    /// Every image the document names, in the order a reader meets them — which
    /// puts an image inside a footnote definition at the first reference to that
    /// footnote, where its content is set.
    pub images: Vec<ImageRef>,
    /// The line each heading sits on, in document order, which
    /// [`crate::md_to_pdf_with_anchors`] pairs with the pages the compiled
    /// headings landed on.
    pub headings: Vec<usize>,
    /// The bibliography the frontmatter named, if it named one.
    pub bibliography: Option<BibliographyRef>,
    /// Every citation key this document writes, with the line its `[@…]` sits
    /// on, from [`Names::cited`].
    ///
    /// It leaves the walk because the refusal it feeds cannot be raised inside
    /// one. Whether the bibliography holds a key is a question about the file's
    /// *bytes*, and emission reads no bytes on either channel — so the check
    /// lives beside `crate::collect`, which is the only place they exist.
    pub cited: Vec<(String, usize)>,
    /// Every name a `[](#name)` pointed at, with the line that link sits on,
    /// from [`Names::referenced`].
    ///
    /// It travels for one refusal only: Typst's labels are one namespace, so a
    /// name this document declares that the bibliography also holds is refused
    /// where a reference points at it. [`check_references`] has already refused
    /// every reference to a name nothing declares, so each of these is a
    /// declared name by the time the bytes are known.
    pub referenced: Vec<(String, usize)>,
}

/// Translate one markdown document into Typst markup.
pub(crate) fn emit(md: &str, sources: &Sources) -> Result<Emitted> {
    let found = collect_definitions(md, sources);
    let mut notes = Notes {
        found: &found,
        seen: HashSet::new(),
        numbers: HashMap::new(),
        skipping: false,
    };

    let mut walk = Walk::new();
    for (event, range) in parser(md).into_offset_iter() {
        step(
            &mut walk,
            md,
            event,
            range,
            Mode::Document(&mut notes),
            sources,
        )?;
    }

    walk.unclosed()?;
    check_references(&walk.names, walk.front.equations)?;
    check_citations(&walk.names, walk.front.bibliography.as_ref())?;

    // Taken off the walk before `finish` consumes it, the way the math flag
    // already is, so `Walk::finish` and `collect_definitions` need no change.
    // The two checks above have already read the names, and `declared` is
    // dropped here: every entry in `referenced` is a declared name once
    // `check_references` has passed, so nothing downstream needs the third.
    let headings = std::mem::take(&mut walk.headings);
    let bibliography = walk.front.bibliography.take();
    let Names {
        cited, referenced, ..
    } = std::mem::take(&mut walk.names);

    let mut out = header(&walk.front, walk.math);
    let (body, images) = walk.finish();
    out.push_str(body.trim_end_matches('\n'));
    out.push('\n');

    // The reference list goes after the document's own content, which is where
    // a reference list goes and the only placement the markdown gives it: the
    // source names the file in the frontmatter and never names a position. What
    // it *looks* like is the look's, reached with a `show bibliography:` rule.
    //
    // **`title: none` is not a style preference.** Typst's default `title: auto`
    // realises a real heading, so a document with one markdown heading would
    // compile two — and `crate::anchors_from` withdraws *every* anchor on a
    // count mismatch, taking the desktop app's scroll sync with it and raising
    // nothing anywhere. The label above the list is the look's, and it must not
    // be a heading.
    if let Some(named) = &bibliography {
        out.push('\n');
        out.push_str("#bibliography(");
        out.push_str(&typst_string(&named.path));
        out.push_str(", title: none)\n");
    }

    Ok(Emitted {
        source: out,
        images,
        headings,
        bibliography,
        cited,
        referenced,
    })
}

/// Translate one event into the walk's own state.
fn step(
    walk: &mut Walk,
    md: &str,
    event: Event,
    range: Range<usize>,
    mut mode: Mode,
    sources: &Sources,
) -> Result<()> {
    let Walk {
        bufs,
        containers,
        lists,
        code,
        table,
        in_metadata,
        meta,
        meta_offset,
        front,
        images,
        headings,
        alt,
        pending,
        figure,
        equation,
        caption,
        group,
        link,
        names,
        para,
        para_end,
        math,
    } = walk;

    // A definition's region is not part of the document's flow: its content
    // travelled to the reference that cites it. This sits above the capture
    // below because a definition is a block, so no capture is ever open across
    // one.
    if let Mode::Document(notes) = &mut mode
        && notes.skipping
    {
        if matches!(event, Event::End(TagEnd::FootnoteDefinition)) {
            notes.skipping = false;
        }
        return Ok(());
    }

    // An open capture takes every event, because alt text is collected and
    // never emitted. A construct outside the dialect still errors here.
    if let Some(capture) = alt.as_mut() {
        match &event {
            Event::Text(text) => capture.text.push_str(text),
            Event::Code(code) => capture.text.push_str(code),
            // Alt text is plain text by CommonMark, so a formula inside it
            // contributes its LaTeX source and the span contributes nothing —
            // the disposition strikethrough got. No `$…$` is written into a
            // string that cannot typeset it, and a document whose only math
            // sits here still imports no prelude.
            //
            // Both forms belong here rather than below. `describe` no longer
            // names either one, so a display span left to the reject arm would
            // refuse an in-dialect construct with the nonsense message
            // `unsupported markdown construct 'supported construct'`.
            Event::InlineMath(latex) | Event::DisplayMath(latex) => capture.text.push_str(latex),
            // A break is whitespace under the alt reading, and one space is
            // what pulldown-cmark's own flattening writes for it.
            Event::SoftBreak | Event::HardBreak => capture.text.push(' '),
            // A wrapper contributes nothing of its own; its content still
            // arrives as text events inside it. Strikethrough belongs here
            // rather than below: it can occur inside alt content, so leaving
            // it to the reject arm would refuse an in-dialect construct.
            Event::Start(Tag::Emphasis | Tag::Strong | Tag::Strikethrough | Tag::Link { .. })
            | Event::End(
                TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link,
            ) => {}
            // A nested image's destination and title are not content under
            // the alt reading, so they are neither checked nor listed.
            Event::Start(Tag::Image { .. }) => capture.depth += 1,
            Event::End(TagEnd::Image) => {
                if capture.depth > 0 {
                    capture.depth -= 1;
                } else {
                    let capture = alt.take().expect("the capture is open");
                    *pending = Some((image_call(&capture.path, &capture.text), capture.opened));
                }
            }
            other => {
                return Err(Error::UnsupportedConstruct {
                    construct: describe(other).to_string(),
                    location: Location::at(line_of(md, range.start)),
                });
            }
        }
        return Ok(());
    }

    // Which form an image takes is known one event late, so the finished
    // call waits here until the next event settles it.
    if let Some((call, opened)) = pending.take() {
        let standalone = opened && matches!(&event, Event::End(TagEnd::Paragraph));
        let start = top(bufs).len();
        write_image(bufs, &call, standalone);
        // Only the standalone form is a candidate for a caption. A figure is a
        // block that floats, and an image inside a clause is not one, so the
        // inline form records nothing — and, by writing where a record would
        // have to end in newlines, retires any point still standing.
        if standalone {
            *figure = Some(Figure {
                depth: bufs.len(),
                start,
                written: top(bufs)[start..].to_string(),
                body: call,
                captioned: false,
            });
            take_member(group, figure, bufs)?;
        }
    }

    // A group holds its three member kinds and its own caption line, and
    // nothing else. A block of any other kind between two members is content
    // reaching a `grid` cell, which is the silent re-layout the dialect
    // refuses, so it is named where it was written. A paragraph is decided at
    // its end, where whether it wrote a member is known; every other block is
    // decided here.
    if group.is_some()
        && matches!(
            &event,
            Event::Start(Tag::Heading { .. } | Tag::List(_) | Tag::BlockQuote(_)) | Event::Rule
        )
    {
        return Err(uncaptionable(line_of(md, range.start)));
    }

    match event {
        Event::Start(Tag::MetadataBlock(_)) => *in_metadata = true,

        // The block is parsed here rather than after the walk, so a bad
        // frontmatter key is reported before any later construct error.
        Event::End(TagEnd::MetadataBlock(_)) => {
            *in_metadata = false;
            let first_line = line_of(md, meta_offset.unwrap_or(range.start));
            *front = frontmatter::parse(meta, first_line)?;
        }

        Event::Start(Tag::Paragraph) => {
            // pulldown-cmark wraps a loose list's item content in paragraphs
            // and a tight list's in bare inlines. That is the whole
            // tightness signal, and Typst draws the same distinction from
            // the blank lines between items, so the emitter passes it
            // through structurally and owns nothing about the spacing.
            if matches!(containers.last(), Some(Container::Item))
                && let Some(frame) = lists.last_mut()
            {
                frame.loose = true;
            }
            // The length here is what an image compares itself against, to
            // learn whether it opened the paragraph.
            let out = top(bufs);
            out.push('\n');
            *para = Some(out.len());
            // The paragraph's own span, for the one question a text run inside
            // it cannot answer about itself: whether it is the whole of it.
            *para_end = Some(range.end);
        }
        Event::End(TagEnd::Paragraph) => {
            // A caption's paragraph is consumed rather than printed: its
            // content goes into the figure above it, and the `'\n'` this arm
            // pushes below closes the figure's own block.
            if let Some(open) = caption.take() {
                let content = bufs.pop().expect("a caption end follows its start");
                let words = caption_words(names, &open, &content)?;
                match group.as_mut() {
                    // Inside a group the caption is the group's own and waits
                    // for the closer, which is what puts one caption and one
                    // number over several members.
                    Some(held) => held.caption = Some(words),
                    None => splice_caption(bufs, figure, &words),
                }
            } else if let Some(held) = group.as_ref()
                && bufs.len() == held.depth
                && !top(bufs)[held.start..].bytes().all(|byte| byte == b'\n')
            {
                // Every member leaves the buffer as it is recorded, so what
                // stands past the group's own start is what no member wrote:
                // prose, an inline image, a display equation, two images in one
                // paragraph. This is `Figure::live`'s content check one level up.
                return Err(uncaptionable(line_of(md, range.start)));
            }
            *para = None;
            *para_end = None;
            top(bufs).push('\n');
        }

        Event::Start(Tag::Heading { level, .. }) => {
            // The line, for the anchor. It is recorded and never written, so
            // the emitted markup is exactly what it was before this existed.
            headings.push(line_of(md, range.start));

            let out = top(bufs);
            out.push('\n');
            for _ in 0..level as usize {
                out.push('=');
            }
            out.push(' ');
        }
        Event::End(TagEnd::Heading(_)) => top(bufs).push('\n'),

        Event::Start(Tag::List(start)) => lists.push(ListFrame {
            next: start,
            loose: false,
            items: Vec::new(),
        }),
        Event::End(TagEnd::List(_)) => {
            let frame = lists.pop().expect("a list end follows its start");
            let separator = if frame.loose { "\n\n" } else { "\n" };
            let rendered = frame.items.join(separator);
            let out = top(bufs);
            out.push('\n');
            out.push_str(&rendered);
            out.push('\n');
        }

        Event::Start(Tag::Item) => {
            containers.push(Container::Item);
            bufs.push(String::new());
        }
        Event::End(TagEnd::Item) => {
            containers.pop();
            let content = bufs.pop().expect("an item end follows its start");
            escaped_frame(group, bufs)?;
            let frame = lists.last_mut().expect("an item sits inside a list");
            let marker = match frame.next.as_mut() {
                Some(number) => {
                    let marker = format!("{number}. ");
                    *number += 1;
                    marker
                }
                None => String::from("- "),
            };
            let item = prefixed(&marker, content.trim_matches('\n'));
            frame.items.push(item);
        }

        Event::Start(Tag::BlockQuote(_)) => {
            containers.push(Container::Quote);
            bufs.push(String::new());
        }
        // The default look stands. Any styling later is a rule in
        // `template.typ`, as with everything else the emitter names.
        Event::End(TagEnd::BlockQuote(_)) => {
            containers.pop();
            let content = bufs.pop().expect("a quote end follows its start");
            escaped_frame(group, bufs)?;
            let out = top(bufs);
            out.push_str("\n#quote(block: true)[\n");
            out.push_str(&prefixed("  ", content.trim_matches('\n')));
            out.push_str("\n]\n");
        }

        // Each cell opens a buffer on the same stack a list item uses, so
        // the emphasis, code and link arms serve cell content unchanged,
        // and the markup escape is what keeps a `]` in a cell from closing
        // its block. A GFM cell holds inline content only, so nothing
        // inside one needs the indentation a nested block would.
        Event::Start(Tag::Table(align)) => {
            *table = Some(TableFrame {
                align,
                cells: Vec::new(),
                header: Vec::new(),
                rows: Vec::new(),
            });
        }
        Event::End(TagEnd::Table) => {
            let frame = table.take().expect("a table end follows its start");
            write_block(bufs, figure, table_call(&frame));
            take_member(group, figure, bufs)?;
        }

        // The head holds its cells directly; no row event wraps them.
        Event::Start(Tag::TableHead | Tag::TableRow) => {}
        Event::End(TagEnd::TableHead) => {
            let frame = table.as_mut().expect("a head sits inside a table");
            frame.header = std::mem::take(&mut frame.cells);
        }
        Event::End(TagEnd::TableRow) => {
            let frame = table.as_mut().expect("a row sits inside a table");
            let row = std::mem::take(&mut frame.cells);
            frame.rows.push(row);
        }

        Event::Start(Tag::TableCell) => bufs.push(String::new()),
        Event::End(TagEnd::TableCell) => {
            let content = bufs.pop().expect("a cell end follows its start");
            let frame = table.as_mut().expect("a cell sits inside a table");
            frame.cells.push(content.trim_matches('\n').to_string());
        }

        // The language tag is the first word of the info string. An
        // indented block, or an empty info string, gets no `lang` argument.
        Event::Start(Tag::CodeBlock(kind)) => {
            let lang = match &kind {
                CodeBlockKind::Fenced(info) => info.split_whitespace().next().map(str::to_string),
                CodeBlockKind::Indented => None,
            };
            *code = Some((lang, String::new()));
        }
        Event::End(TagEnd::CodeBlock) => {
            let (lang, mut content) = code.take().expect("a block end follows its start");
            // pulldown-cmark reports the final line's terminator as part of
            // the content, and a string literal that kept it would typeset a
            // phantom empty line after every code block.
            if content.ends_with('\n') {
                content.pop();
            }
            // One arm serves the fenced block and the indented one, differing
            // only in whether a `lang` argument is written, so both take a
            // caption. Splitting them would make a `: ` line a caption after
            // one kind of block and prose after another, with nothing on the
            // page to tell an author which they had written.
            write_block(bufs, figure, raw_call(lang.as_deref(), &content));
            take_member(group, figure, bufs)?;
        }

        Event::Text(text) => {
            if let Some((_, content)) = code.as_mut() {
                // Code is not markup, so it is not escaped here. It reaches
                // Typst as a string literal, like inline code before it.
                content.push_str(&text);
            } else if *in_metadata {
                let first = line_of(md, *meta_offset.get_or_insert(range.start));
                let line = line_of(md, range.start);
                while lines_in(meta) < line.saturating_sub(first) + 1 {
                    meta.push('\n');
                }
                meta.push_str(&text);
            } else {
                let opens = *para == Some(top(bufs).len());

                // `:::` is reserved at the first text of a paragraph, and only
                // there: a `:::` inside a sentence, one standing later in a
                // paragraph, and one inside a fenced or indented code block are
                // all untouched — which is where a document that documents this
                // syntax puts one. The run has to be the whole paragraph, and
                // that is what makes the tight div — one paragraph joined by
                // soft breaks — a named error rather than an opener whose
                // group can never close. **A paragraph that begins `:::` is a
                // delimiter or a mistyped one and never prose**, so there is no
                // such thing as a lone `:::` reaching the page.
                if opens && text.trim().starts_with(":::") {
                    let line = line_of(md, range.start);
                    // The run has to be the whole paragraph *and* a delimiter's
                    // own shape, and both failures are the same error, on the
                    // sentence above.
                    let marker = whole_paragraph(md, &range, *para_end)
                        .then(|| group_marker(text.trim()))
                        .flatten();
                    let Some(marker) = marker else {
                        return Err(Error::UnsupportedConstruct {
                            construct:
                                "figure group delimiter that is neither an opener nor a closer"
                                    .to_string(),
                            location: Location::at(line),
                        });
                    };
                    match (group.is_some(), marker) {
                        (true, Marker::Bare) => {
                            let open = group.take().expect("a group is open");
                            close_group(bufs, open)?;
                        }
                        (true, Marker::Word) => {
                            return Err(Error::UnsupportedConstruct {
                                construct: "figure group inside a figure group".to_string(),
                                location: Location::at(line),
                            });
                        }
                        (false, _) => {
                            // The opener is a block boundary, so a record
                            // standing before it is retired: a caption inside
                            // the group is the group's own.
                            *figure = None;
                            *group = Some(Group {
                                line,
                                depth: bufs.len(),
                                start: top(bufs).len(),
                                members: Vec::new(),
                                caption: None,
                            });
                        }
                    }
                    return Ok(());
                }

                // A paragraph that opens with `: ` captions the group it stands
                // in, or the standalone image, table or code block it stands
                // under. Everywhere else the marker is the ordinary prose it is
                // today, which is what keeps it from being a ban on a line an
                // author may already have written: the collision window is one
                // paragraph in one position.
                let marked = match opens {
                    true => caption_marker(&text),
                    false => None,
                };
                let attaches =
                    group.is_some() || figure.as_ref().is_some_and(|recorded| recorded.live(bufs));
                if let Some(rest) = marked
                    && attaches
                {
                    let line = line_of(md, range.start);
                    // One construct takes one caption, and so does one group.
                    // Neither refusal falls out of the checks around it: a
                    // spliced region carries `#figure(…)`, which the content
                    // check accepts, and a group's caption is held rather than
                    // written at all.
                    let second = match group.as_ref() {
                        Some(held) => held.caption.is_some().then_some("figure group"),
                        None => figure
                            .as_ref()
                            .is_some_and(|recorded| recorded.captioned)
                            .then_some("figure"),
                    };
                    if let Some(what) = second {
                        return Err(Error::UnsupportedConstruct {
                            construct: format!("second caption for one {what}"),
                            location: Location::at(line),
                        });
                    }
                    *caption = Some(Caption {
                        line,
                        text: rest.to_string(),
                    });
                    // The caption is prose, so its content is walked as inline
                    // markdown into a frame of its own — the mechanism a list
                    // item, a block quote and a table cell already use.
                    //
                    // `para` is an offset into the frame below, and comparing
                    // it against this one's length would be reading a number
                    // against the wrong buffer. Clearing it is also what makes
                    // an image inside a caption unable to take the standalone
                    // form: a caption is not a paragraph.
                    *para = None;
                    bufs.push(String::new());
                    escape_into(top(bufs), rest);
                } else if equation
                    .as_ref()
                    .is_some_and(|recorded| recorded.live(bufs))
                    && let Some(name) = equation_name(&text, line_of(md, range.start))?
                {
                    // A name riding the closing `$$`, written where the
                    // equation was just written. Liveness is tested first, so a
                    // run after a dead record is the prose it has always been
                    // and raises nothing; the run itself is consumed rather
                    // than escaped, which is what takes the group off the page.
                    declare(names, name, line_of(md, range.start), true)?;
                    top(bufs).push_str(&format!(" <{name}>"));
                    *equation = None;
                } else {
                    if let Some(open) = caption.as_mut() {
                        open.text.push_str(&text);
                    }
                    escape_into(top(bufs), &text);
                }
            }
        }
        Event::SoftBreak => top(bufs).push('\n'),

        // The function forms, not Typst's own `_…_` and `*…*`. Those
        // delimiters are word-boundary sensitive and CommonMark permits
        // intraword emphasis, so `foo*bar*baz` would reach the PDF with
        // literal underscores through one and would not compile at all
        // through the other.
        Event::Start(Tag::Emphasis) => top(bufs).push_str("#emph["),
        Event::End(TagEnd::Emphasis) => top(bufs).push(']'),
        Event::Start(Tag::Strong) => top(bufs).push_str("#strong["),
        Event::End(TagEnd::Strong) => top(bufs).push(']'),

        // The function form here is the only form there is: Typst has no
        // markup for a strike, so the delimiter argument above does not
        // arise. The parser admits a run of one tilde as well as two, so
        // `~struck~` reaches this arm too.
        Event::Start(Tag::Strikethrough) => top(bufs).push_str("#strike["),
        Event::End(TagEnd::Strikethrough) => top(bufs).push(']'),

        // The content is a string literal, never the markup escape, so it
        // reaches the PDF verbatim whatever it holds.
        Event::Code(inline) => {
            let out = top(bufs);
            out.push_str("#raw(");
            out.push_str(&typst_string(&inline));
            out.push(')');
        }

        // A formula travels the same way a code span's content does: it is
        // Typst markup by the time it is written, so it does not go through
        // `escape_into`, which would break every span it touched.
        //
        // The metadata block needs no guard here, unlike the text arm: its body
        // reaches the walk as raw code text, so no inline event is produced
        // inside it at all.
        Event::InlineMath(latex) => {
            let markup = math::convert(&latex, line_of(md, range.start))?;
            let out = top(bufs);
            out.push('$');
            out.push_str(&markup);
            out.push('$');
            *math = true;
        }

        // Typst's block equation is the same two delimiters with whitespace
        // inside them — `typst::syntax::ast::Equation::block` tests for a space
        // after the opening one and before the closing one, so `$ x $` is a
        // block where `$x$`, `$ x$` and `$x $` are not. How that block then
        // sits — its spacing, its alignment, whether it is numbered — is a look
        // decision, and a look reaches it with
        // `show math.equation.where(block: true)` over a Typst element, the way
        // both bundled ones already reach `raw` and `table.cell`. So nothing
        // here is exported and nothing joins the look contract.
        //
        // The arm consults no position. An image carries no signal about which
        // form its author wanted, which is why `write_image` infers one from the
        // paragraph; `$$` *is* that signal. So the block form is written
        // wherever the span sits, and a paragraph holding one is split by it —
        // which is what the author's own `$$` asked for.
        Event::DisplayMath(latex) => {
            let markup = math::convert(&latex, line_of(md, range.start))?;
            let written = format!("$ {markup} $");
            let depth = bufs.len();
            let out = top(bufs);
            let start = out.len();
            out.push_str(&written);
            *math = true;

            // The record a `{#name}` group looks back at. **Nothing is recorded
            // while a caption is open**, and that guard is the whole of what
            // stops a display span inside a caption stealing the caption's own
            // name: `: See $$x = 1$$ {#fig:one}` names the *figure*, as it has
            // since Phase 3. The record's liveness test does not refuse it — the
            // marker arm pushes the caption's frame before anything later in
            // that paragraph is written, so the span records at that deeper
            // frame and would be spent at the same one, with nothing in between
            // to fail the content check.
            //
            // A `Walk` field rather than a frame-depth test, because a display
            // span nested inside a link inside a caption sits deeper still.
            *equation = caption.is_none().then_some(Equation {
                depth,
                start,
                written,
            });
        }

        // A URL reaches Typst as a string literal, never as markup, so
        // `typst_string` carries it whatever it holds. The link text is
        // ordinary inline content, so the markup escape still applies to
        // it — which is what stops Typst reading an autolink's own text as
        // a second link, or an email address as a reference.
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            ..
        }) => {
            // Neither shape can be emitted honestly. `#link("")` fails the
            // compile naming neither construct nor line, and a title is
            // something neither `link` nor the PDF can carry, so passing it
            // on would mean dropping it.
            if dest_url.is_empty() {
                return Err(Error::UnsupportedConstruct {
                    construct: "link with an empty destination".to_string(),
                    location: Location::at(line_of(md, range.start)),
                });
            }
            if !title.is_empty() {
                return Err(Error::UnsupportedConstruct {
                    construct: "link with a title".to_string(),
                    location: Location::at(line_of(md, range.start)),
                });
            }

            // Every other form arrives with its destination resolved, so
            // the email autolink is the one case with work left: its
            // destination is the bare address, and the scheme is ours.
            let url = match link_type {
                LinkType::Email => format!("mailto:{dest_url}"),
                _ => dest_url.into_string(),
            };

            // The text goes into a frame of its own, the way a table cell's
            // already does, because whether it is empty is a later event and
            // that emptiness is the whole discriminator. Nothing else changes:
            // the end arm reassembles exactly the call this arm used to write.
            //
            // A frame here makes `para == top(bufs).len()` newly *reachable*
            // inside a link, so `caption_marker` can fire on a text run there.
            // It can never attach: `Figure::live` wants the recorded frame on
            // top, and a link frame is always one deeper than any recorded
            // block. An image inside a link flushes into this frame for the
            // same reason and writes the same bytes, and it is never standalone
            // there, since `para` is never `Some(0)`.
            *link = Some(LinkFrame {
                url,
                line: line_of(md, range.start),
                link_type,
            });
            bufs.push(String::new());
        }
        // A link with no text at all and a `#` destination is a reference to a
        // name the document declares. Every link that carries text is the link
        // it has always been, whatever its destination — scoping by the text
        // rather than by declaredness is what leaves `[Introduction](#introduction)`,
        // the ordinary anchor idiom, meaning what it always meant.
        //
        // **Empty means empty.** `[ ](#name)` is a link, because the
        // discriminator has to be statable and `is_empty` is, where "text that
        // renders to nothing" is not.
        //
        // `#ref(<name>)` is the function form, not Typst's `@name` marker, on
        // the argument that put `#emph[…]` here over `_…_`: the marker swallows
        // what follows it, so `[](#fig:one)s` would emit `@fig:ones` and fail
        // the compile on a label the author never typed.
        Event::End(TagEnd::Link) => {
            let text = bufs.pop().expect("a link end follows its start");
            let frame = link.take().expect("a link end follows its start");

            // A citation is decided before either of the two below, and its
            // collected text is discarded: the payload is the destination the
            // callback wrote, and the brackets' content is that same payload
            // escaped as prose.
            if wrote_citation(&frame) {
                let key = cite_key(&frame.url, frame.line)?;
                names.cited.push((key.to_string(), frame.line));
                let out = top(bufs);
                out.push_str("#cite(label(");
                out.push_str(&typst_string(key));
                out.push_str("))");
                return Ok(());
            }

            match frame.url.strip_prefix('#').filter(|_| text.is_empty()) {
                Some(name) => {
                    names.referenced.push((name.to_string(), frame.line));
                    let out = top(bufs);
                    out.push_str("#ref(<");
                    out.push_str(name);
                    out.push_str(">)");
                }
                None => {
                    let out = top(bufs);
                    out.push_str("#link(");
                    out.push_str(&typst_string(&frame.url));
                    out.push_str(")[");
                    out.push_str(&text);
                    out.push(']');
                }
            }
        }

        // An image is the one construct that needs a file, so this arm is
        // where the pipeline decides which files it will ever ask for. What
        // survives both halves of the check opens the alt capture and joins
        // the shopping list.
        //
        // **`check_image` is handed both the written destination and the one it
        // landed on, and the division is which string each shape is read off —
        // not which of them is computed first.** A scheme, a leading `/`, a
        // backslash and an empty destination are properties of what the author
        // *wrote*: `typst-syntax` normalises a non-leading empty segment away,
        // so `/x.png` prefixed to `one//x.png` would read as `/one/x.png` and an
        // absolute path would be laundered into a relative one, silently, and
        // `![alt]()` prefixed to `one/` would stop being empty at all.
        //
        // Leaving the document's folder is the one shape that is a property of
        // where the path *lands*, so it is the one read off the resolved
        // destination. That is what lets a section name a figure beside the
        // master: `../figures/plot.svg` written in `sections/method.md` lands on
        // `figures/plot.svg`, inside the folder, and only `../../escape.png`
        // actually climbs out.
        Event::Start(Tag::Image {
            dest_url, title, ..
        }) => {
            let line = line_of(md, range.start);

            // The path the master would have written. It is the identity every
            // downstream reader keys on — the Typst source, the world's
            // `FileId`, and both wrappers' dedupe — which is why it is settled
            // here, at the one place that knows both the destination and the
            // file it was written in.
            let dest = sources.resolve(line, &dest_url);
            check_image(&dest_url, &dest, &title, line)?;
            images.push(ImageRef {
                path: dest.clone(),
                location: Location::at(line),
            });
            *alt = Some(AltCapture {
                opened: *para == Some(top(bufs).len()),
                path: dest,
                text: String::new(),
                depth: 0,
            });
        }

        // A definition's content belongs to the reference that cites it, so
        // the region itself is never emitted. What it owes is settled at the
        // line it sits on, and the walk then skips it.
        Event::Start(Tag::FootnoteDefinition(label)) => {
            let line = line_of(md, range.start);
            match &mut mode {
                Mode::Document(notes) => notes.enter(label_of(&label), line)?,
                // The parser hoists a definition written inside another one to
                // a sibling at the top level, so this does not arrive.
                // Refusing it keeps the walk of the definitions free of a
                // nesting it has no answer for.
                Mode::Definition => {
                    return Err(Error::UnsupportedConstruct {
                        construct: "footnote definition inside a footnote definition".to_string(),
                        location: Location::at(line),
                    });
                }
            }
        }

        // Typst takes a footnote's content at the reference site, so the first
        // reference to a label carries the content and every later one points
        // at the name that first one wrote. The user's own label text never
        // reaches the output: a markdown label may hold any character and a
        // Typst label may not, and generating the name removes the escaping
        // question rather than answering it. Typst numbers footnotes in
        // placement order, which is the order GFM numbers them in, so the
        // emitter writes no number itself.
        Event::FootnoteReference(label) => {
            let Mode::Document(notes) = &mut mode else {
                // Resolving a footnote inside a footnote would mean a recursive
                // substitution with a cycle check, for a construct real
                // articles do not carry.
                return Err(Error::UnsupportedConstruct {
                    construct: "footnote reference inside a footnote definition".to_string(),
                    location: Location::at(line_of(md, range.start)),
                });
            };

            let label = label_of(&label);
            match notes.numbers.get(&label) {
                Some(number) => top(bufs).push_str(&format!("#footnote(<fn-{number}>)")),
                None => {
                    let number = notes.numbers.len() + 1;
                    notes.numbers.insert(label.clone(), number);

                    let out = top(bufs);
                    out.push_str("#footnote[");
                    // A definition whose translation failed writes an empty
                    // footnote here and does not raise. Its own region raises,
                    // at the position that error belongs to, which is what lets
                    // an error between this reference and that region be
                    // reported first.
                    if let Ok(body) = notes.found.body(&label) {
                        out.push_str(&body.content);
                        images.extend(body.images.iter().cloned());
                        *math |= body.math;
                        // A name inside a definition is declared where the
                        // definition is *set*, which is here — so an uncited
                        // definition declares nothing, and a reference to a name
                        // only it carries is refused, which is what Typst would
                        // do with a label that reached no page.
                        for seen in &body.names.declared {
                            declare(names, &seen.name, seen.line, seen.equation)?;
                        }
                        names
                            .referenced
                            .extend(body.names.referenced.iter().cloned());
                        // A citation inside a definition is cited where the
                        // definition is set, which is here — so an uncited
                        // definition cites nothing, exactly as it declares
                        // nothing.
                        names.cited.extend(body.names.cited.iter().cloned());
                    }
                    top(bufs).push_str(&format!("]<fn-{number}>"));
                }
            }
        }

        // Typst's line break is a `\` before a newline. The same `\`
        // directly before text is an escape sequence instead.
        Event::HardBreak => top(bufs).push_str("\\\n"),

        // The emitter names the rule and owns nothing about its look.
        Event::Rule => top(bufs).push_str("\n#divider()\n"),

        other => {
            return Err(Error::UnsupportedConstruct {
                construct: describe(&other).to_string(),
                location: Location::at(line_of(md, range.start)),
            });
        }
    }

    Ok(())
}

/// The buffer the walk is currently writing into.
fn top(bufs: &mut [String]) -> &mut String {
    bufs.last_mut().expect("the document body is always open")
}

// -- paths ------------------------------------------------------------------

/// A shape a path in this dialect may not have.
///
/// Two renderings rather than one sentence, because the two readers name the
/// path differently: an image arm says what the image is, and a frontmatter key
/// says what the key takes. The rule underneath is the same rule, which is the
/// point of the enum.
///
/// **The first three are properties of what the author wrote and the last is a
/// property of where the path lands**, which is the division
/// [`written_shape`] and [`landed_path`] are named after.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathShape {
    Scheme,
    Absolute,
    Backslash,
    Escapes,
}

impl PathShape {
    /// The fragment the image arm names, unchanged since it was written.
    fn image(self) -> &'static str {
        match self {
            PathShape::Scheme => "a URL destination",
            PathShape::Absolute => "an absolute path",
            PathShape::Backslash => "a backslash in its path",
            PathShape::Escapes => "a path that leaves the document's folder",
        }
    }

    /// The fragment a frontmatter key that takes a path names.
    ///
    /// `Escapes` reads the same words in both, and that is deliberate: the other
    /// three are about how a destination is *spelled*, where this one is about
    /// its effect, and an effect has no second phrasing to earn.
    pub fn key(self) -> &'static str {
        match self {
            PathShape::Scheme => "a URL",
            PathShape::Absolute => "an absolute path",
            PathShape::Backslash => "a path with a backslash",
            PathShape::Escapes => "a path that leaves the document's folder",
        }
    }
}

/// The shapes a destination is refused for by how it is *written*.
///
/// A scheme is a fetch request and nothing fetches, which catches `data:` and
/// the drive path `C:\figure.png` with it; an absolute path converts on one
/// machine only; a Windows separator writes a segment Typst's own virtual
/// filesystem cannot hold.
///
/// **These are read off the destination the author wrote and never off the one
/// it resolved to**, which is load-bearing rather than tidy: `Sources::resolve`
/// is a `format!` with no guard, so `/x.png` written in a section becomes
/// `sections//x.png`, and `typst-syntax` maps a non-leading empty segment to
/// `Component::Current` and ignores it — read off the prefixed string, an
/// absolute path would be laundered into a relative one with nothing raised.
///
/// **The backslash is tested here rather than inferred from `VirtualPath`'s own
/// error**, because [`landed_path`] can now fail for a second reason and the two
/// would be indistinguishable. `components()` splits on `/` and a backslash
/// lands inside a segment `Segment::new` rejects, so this test is exactly the
/// `PathError::Backslash` it replaces.
fn written_shape(dest: &str) -> std::result::Result<(), PathShape> {
    if has_scheme(dest) {
        return Err(PathShape::Scheme);
    }
    if dest.starts_with('/') {
        return Err(PathShape::Absolute);
    }
    if dest.contains('\\') {
        return Err(PathShape::Backslash);
    }
    Ok(())
}

/// The virtual path a destination lands on, or the one shape that is a property
/// of *where* it lands rather than of how it was written.
///
/// A document and the files it names travel as one folder, so a path that leaves
/// that folder is refused — but leaving it is not the same statement as carrying
/// a `..`. `sections/../figures/plot.svg` carries one and lands inside;
/// `../figures/plot.svg` carries one and does not. `Segments::push_component`
/// pops a segment for a parent component and returns `PathError::Escapes` only
/// when there is nothing left to pop, which is this dialect's rule stated once by
/// the layer that owns the virtual root.
///
/// **The resolution happens here rather than after the check**, so the extension
/// is read off the same `VirtualPath` `crate::file_id` will later build, and a
/// caller never hands `file_id` a path it cannot build — which returns
/// `Error::Internal`, whose own contract says a broken build rather than bad
/// input.
fn landed_path(dest: &str) -> std::result::Result<VirtualPath, PathShape> {
    VirtualPath::new(dest).map_err(|error| match error {
        PathError::Escapes => PathShape::Escapes,
        PathError::Backslash => PathShape::Backslash,
    })
}

/// Both halves, for a caller whose written destination is where it lands.
///
/// The `bibliography` frontmatter key is one — it is the master's own key, and
/// `crate::frontmatter::parse` holds no `Sources` to prefix anything with — and
/// [`lone_markdown_link`] is the other, reading it as a predicate to tell an
/// include marker from a link the dialect would refuse as a path.
pub(crate) fn portable_path(dest: &str) -> std::result::Result<(), PathShape> {
    written_shape(dest)?;
    landed_path(dest)?;
    Ok(())
}

// -- images -----------------------------------------------------------------

/// Refuse every image destination the pipeline cannot carry, naming the shape.
///
/// **Two destinations rather than one, because the shapes divide by what each is
/// a property of.** `written` is what the author typed; `landed` is where it
/// resolved to — the same string for an image the master names, and prefixed
/// with the section's own directory for one written in a section.
///
/// The first two mirror the link arm, for the same two reasons. The next three
/// are [`written_shape`]'s and are read off `written`, which is what keeps
/// `![alt]()` an empty destination and `/x.png` an absolute path rather than
/// whatever they would look like with `sections/` in front of them. The next is
/// [`landed_path`]'s and is read off `landed`, because leaving the document's
/// folder is a property of where a path ends up: `../figures/plot.svg` written
/// under `sections/` lands inside the folder and only `../../escape.png` climbs
/// out. **Which string reaches which check is the whole of the division** — not
/// when the prefix is computed. The last is the format gate's first half: Typst
/// reads the extension before the content, so an extension it does not name
/// leaves the format undecided, and the dialect refuses to guess.
///
/// A shape is named before an extension, as it always has been, so `../../a.bmp`
/// is refused for leaving the folder and not for its ending.
fn check_image(written: &str, landed: &str, title: &str, line: usize) -> Result<()> {
    let refuse = |construct: String| {
        Err(Error::UnsupportedConstruct {
            construct,
            location: Location::at(line),
        })
    };

    if written.is_empty() {
        return refuse("image with an empty destination".to_string());
    }
    if !title.is_empty() {
        return refuse("image with a title".to_string());
    }
    if let Err(shape) = written_shape(written) {
        return refuse(format!("image with {}", shape.image()));
    }

    let vpath = match landed_path(landed) {
        Ok(vpath) => vpath,
        Err(shape) => return refuse(format!("image with {}", shape.image())),
    };
    match vpath.extension() {
        None => refuse("image with no file extension".to_string()),
        Some(extension) if !IMAGE_EXTENSIONS.contains(&extension) => {
            refuse(format!("image with a .{extension} extension"))
        }
        Some(_) => Ok(()),
    }
}

/// The extension of a path the walk has already accepted.
///
/// This reads `VirtualPath::extension`, the function Typst's own format
/// detection reads, so the two can never disagree about where a name ends and
/// its extension begins.
pub(crate) fn extension_of(path: &str) -> Option<String> {
    VirtualPath::new(path).ok()?.extension().map(str::to_string)
}

/// The same path with its `.` and `..` segments resolved away, where it resolves
/// at all.
///
/// `crate::sections::Sources::resolve` reads this so that one file named two
/// ways arrives as one string: `figures/plot.svg` written by the master and
/// `../figures/plot.svg` written by a section under `sections/` are the same
/// file, and `crate::collect` keys `supplied`, `seen` and the world's `FileId` on
/// that string. **`None` is not a refusal** — it is a path that leaves the
/// folder, which [`check_image`] refuses in the author's own words and at the
/// author's own line.
pub(crate) fn normalise(path: &str) -> Option<String> {
    VirtualPath::new(path)
        .ok()
        .map(|vpath| vpath.get_without_slash().to_string())
}

/// True when the destination opens with a URI scheme, as RFC 3986 writes one.
///
/// This is what turns `https:` and `data:` into errors, and the Windows drive
/// path `C:\figure.png` with them. The relative form is the portable one.
fn has_scheme(dest: &str) -> bool {
    let Some(colon) = dest.find(':') else {
        return false;
    };
    let mut chars = dest[..colon].chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
}

/// One image as a Typst call, without the leading `#`.
///
/// The path and the alt both travel as string literals, never as markup, so a
/// `#` in a filename and a `"` in an alt text both survive. An empty alt leaves
/// the argument out rather than naming an empty description.
fn image_call(path: &str, alt: &str) -> String {
    let mut out = String::from("image(");
    out.push_str(&typst_string(path));
    if !alt.is_empty() {
        out.push_str(", alt: ");
        out.push_str(&typst_string(alt));
    }
    out.push(')');
    out
}

/// Write a finished image call in the form the source drew.
///
/// Typst lays an image out as a block, and its documented inline form is
/// `box(image(..))`. A paragraph holding one image and nothing else is a block
/// in the source too, so it stays one; every other occurrence would otherwise
/// split the paragraph around it, which rewrites the user's prose.
fn write_image(bufs: &mut [String], call: &str, standalone: bool) {
    let out = top(bufs);
    if standalone {
        out.push('#');
        out.push_str(call);
    } else {
        out.push_str("#box(");
        out.push_str(call);
        out.push(')');
    }
}

// -- captions ---------------------------------------------------------------

/// Write a block-level call in a block of its own, and record where it stands.
///
/// A table and a code block own their separators where a standalone image does
/// not: this pushes both the `'\n'` that opens the block and the one that
/// closes it, where an image takes both from the paragraph arms around it. So
/// **the record starts after the leading newline**. Recorded from before it,
/// `splice_caption`'s truncate would eat the block separator and glue
/// `#figure(` onto the line above.
///
/// The call arrives without its `#`, the way `image_call` returns one, because
/// that bare call is what a `#figure(…)` wraps: a `#` inside a code context is
/// a syntax error rather than a mismatch.
fn write_block(bufs: &mut [String], figure: &mut Option<Figure>, call: String) {
    let depth = bufs.len();
    let out = top(bufs);
    out.push('\n');
    let start = out.len();
    out.push('#');
    out.push_str(&call);
    out.push('\n');

    *figure = Some(Figure {
        depth,
        start,
        written: format!("#{call}"),
        body: call,
        captioned: false,
    });
}

/// The caption a paragraph's first text run opens with, if it opens with the
/// marker at all.
///
/// `: ` is the marker because it costs nothing in this dialect: it is Pandoc's
/// own table-caption spelling, GFM gives it no meaning, and `options` parses
/// such a line as an ordinary paragraph.
///
/// The colon alone is the marker too. It is how an author writes one with
/// nothing after it — the parser strips a line's trailing space, so `": "`
/// never survives as a text event — and that shape is a mistake the dialect
/// names rather than a caption it guesses at.
fn caption_marker(text: &str) -> Option<&str> {
    match text {
        ":" => Some(""),
        _ => text.strip_prefix(": "),
    }
}

/// Read one caption paragraph: its words, and the name that rode its end.
///
/// Shared by the single construct and the group, because the line is the same
/// line and its two refusals are the same two. What differs is only when the
/// reading is spent — at once for one construct, at the closer for a group.
///
/// The name is read from the unescaped copy the walk kept, because
/// `escape_into` has turned `{#fig-two}` into `{\#fig\-two}` in the buffer by
/// now — `#`, `-` and `_` are all in `SPECIAL`.
fn caption_words(names: &mut Names, caption: &Caption, content: &str) -> Result<Words> {
    let content = content.trim();
    let typed = caption.text.trim_end();
    let named = caption_name(typed, caption.line)?;

    // The group leaves the caption, and that is not a substring removal of what
    // the author typed. It is stripped from the buffer as its own escaped form,
    // which is exact: `escape_into`'s one positional rule looks back no further
    // than the last newline, and the group opens with `{`, so no character
    // inside it escapes differently in place than it does alone.
    let content = match named {
        Some((group, _)) => {
            let mut escaped = String::new();
            escape_into(&mut escaped, group);
            content
                .strip_suffix(escaped.as_str())
                .expect("the caption buffer ends with the group the text does")
                .trim_end()
        }
        None => content,
    };

    // A marker with no caption after it would put a bare "Figure 1:" on the
    // page, which is a mistake better named than emitted. The test runs after
    // the group is dropped, so a line carrying a name and no words is refused
    // here too rather than captioning a figure with nothing.
    if content.is_empty() {
        return Err(Error::UnsupportedConstruct {
            construct: "caption with no text".to_string(),
            location: Location::at(caption.line),
        });
    }

    if let Some((_, name)) = named {
        declare(names, name, caption.line, false)?;
    }

    Ok(Words {
        line: caption.line,
        content: content.to_string(),
        name: named.map(|(_, name)| name.to_string()),
    })
}

/// Rewrite a recorded block into the figure its caption makes it.
///
/// **The caption is what makes a figure**, which is why this runs only where a
/// caption line stands. An uncaptioned `#figure` prints no number and still
/// consumes the counter, so the next captioned one would read "Figure 2" with
/// no Figure 1 anywhere, and `figure` centres its body where a bare block sits
/// flush left. Wrapping unconditionally would therefore re-lay-out and
/// mis-number every document that shows an image, a table or a code block,
/// which is the property `mpdf-004` Phase 3 stated: no document's typeset
/// output changes unless its author asks.
///
/// The truncate unwinds two newlines the paragraph arms have already pushed —
/// one closing the image's paragraph, one opening the caption's, for a
/// paragraph that is about to be consumed. It is one of the two writes in this
/// file that are not appends — [`close_group`] is the other — and it updates
/// the record it spends.
///
/// The emitter writes no supplement, no separator and no number: the author
/// supplies the words, and the look decides what a caption looks like.
fn splice_caption(bufs: &mut [String], figure: &mut Option<Figure>, words: &Words) {
    let recorded = figure
        .as_mut()
        .expect("a caption opens only over a recorded figure or a group");
    let mut call = format!("#figure({}, caption: [{}])", recorded.body, words.content);
    // The label rides the same string the record keeps. Appended to the buffer
    // alone it would fail `Figure::live`'s content check, and Phase 1's
    // second-caption refusal would silently stop firing over a named figure.
    if let Some(name) = &words.name {
        call.push_str(&format!(" <{name}>"));
    }

    let out = top(bufs);
    out.truncate(recorded.start);
    out.push_str(&call);

    recorded.written = call;
    recorded.captioned = true;
}

// -- groups -----------------------------------------------------------------

/// Which delimiter a `:::` paragraph is.
enum Marker {
    /// `:::` — the closer, and the opener where no group is open.
    Bare,
    /// `::: word` — an opener, and only ever an opener.
    Word,
}

/// The delimiter a paragraph's whole text is, if it is one at all.
///
/// The word after an opener is the author's convention and **the dialect does
/// not read it**: Typst infers a figure's kind from the `grid` it is handed, so
/// `::: table` around two images is a Figure, exactly as `{#tab:pipeline}` on
/// an image is. One word and no more, because `::::`, `:::x` and
/// `::: two words` are mistypings better named than guessed at — a reserved
/// position that is reserved for some spellings and not others is a rule no
/// author can hold.
fn group_marker(run: &str) -> Option<Marker> {
    if run == ":::" {
        return Some(Marker::Bare);
    }
    let word = run.strip_prefix("::: ")?;
    (!word.is_empty() && !word.contains(char::is_whitespace)).then_some(Marker::Word)
}

/// Whether a text run is the whole of the paragraph it opens.
///
/// The one question a run cannot answer about itself, and the trap [`Figure`]
/// records at length: the next event settles it, and nothing here is held
/// across one. The paragraph's own source span answers it now, since the parser
/// hands a `Start` event the range of the whole element — whitespace is all that
/// may stand between the run's end and the paragraph's.
fn whole_paragraph(md: &str, run: &Range<usize>, para_end: Option<usize>) -> bool {
    match para_end {
        Some(end) if run.end <= end => md[run.end..end].trim().is_empty(),
        _ => false,
    }
}

/// The refusal a block inside a group that is not a member takes.
///
/// A paragraph sitting between two members is content reaching a `grid` cell,
/// which is a silent re-layout of what the author wrote — so it is named at its
/// own line rather than softened into pass-through prose.
fn uncaptionable(line: usize) -> Error {
    Error::UnsupportedConstruct {
        construct: "block inside a figure group that is not an image, a table or a code block"
            .to_string(),
        location: Location::at(line),
    }
}

/// Refuse a group whose own buffer frame has just been popped.
///
/// **An opener and its closer sit in the same frame, not merely at the same
/// depth.** `- :::` / image / `- :::` puts the two delimiters in different list
/// items and *both at depth 2*, so a depth-only test would accept the pair and
/// truncate a frame the group never opened. Retiring a group where its frame
/// goes is what keys the record to the frame itself, and a list item and a
/// block quote are the only two frames an opener's paragraph can stand in.
fn escaped_frame(group: &Option<Group>, bufs: &[String]) -> Result<()> {
    match group {
        Some(open) if bufs.len() < open.depth => Err(Error::UnsupportedConstruct {
            construct: "figure group the document never closes".to_string(),
            location: Location::at(open.line),
        }),
        _ => Ok(()),
    }
}

/// Take the block just recorded into the group it stands in.
///
/// A group collects a [`Figure`]'s `body` each time a record is made while it
/// is open, so it needs no second notion of what a member is. The call leaves
/// the buffer as it is collected and every member is written back inside one
/// `grid` at the closer — the second write in this file that is not an append,
/// and it spends the record it removes in the same breath.
fn take_member(
    group: &mut Option<Group>,
    figure: &mut Option<Figure>,
    bufs: &mut [String],
) -> Result<()> {
    let Some(open) = group.as_mut() else {
        return Ok(());
    };
    if bufs.len() != open.depth {
        return Ok(());
    }

    // The group's caption is the last block before the closer. A `: ` line with
    // a member after it is exactly the spelling a subcaption will want, so it
    // is refused now rather than shipped as a meaning to take back.
    if let Some(words) = &open.caption {
        return Err(Error::UnsupportedConstruct {
            construct: "figure group caption with a member after it".to_string(),
            location: Location::at(words.line),
        });
    }

    let recorded = figure
        .take()
        .expect("a member is taken where a record was just made");
    open.members.push(recorded.body);
    top(bufs).truncate(open.start);
    Ok(())
}

/// Write the group its closer ends as one figure over a `grid`.
///
/// `columns` is structural and the gutter is not: the emitter writes how many
/// members there are and never how far apart they sit, which is each look's own
/// call, reached with a `show` rule and nothing crossing the seam. **No `kind`
/// is written either** — Typst infers one through the `grid`, so a group of
/// images is a Figure and a group of tables is a Table with nothing configured.
///
/// The truncate takes back the delimiter paragraphs' own newlines along with
/// whatever the members left, so a group emits the `\n#figure(…)\n` a spliced
/// caption emits and the closer's paragraph end supplies the last of them.
fn close_group(bufs: &mut [String], open: Group) -> Result<()> {
    // The one refusal here that would otherwise reach Typst: `grid(columns: 0)`
    // fails the compile with `number must be positive`, naming no line and no
    // construct the author would recognise.
    if open.members.is_empty() {
        return Err(Error::UnsupportedConstruct {
            construct: "figure group with no member".to_string(),
            location: Location::at(open.line),
        });
    }
    // The caption is what makes a figure, over one member or several.
    let Some(words) = open.caption else {
        return Err(Error::UnsupportedConstruct {
            construct: "figure group with no caption".to_string(),
            location: Location::at(open.line),
        });
    };

    let mut call = format!(
        "#figure(grid(columns: {}, {}), caption: [{}])",
        open.members.len(),
        open.members.join(", "),
        words.content
    );
    if let Some(name) = &words.name {
        call.push_str(&format!(" <{name}>"));
    }

    let out = top(bufs);
    out.truncate(open.start);
    out.push_str(&call);
    Ok(())
}

// -- names ------------------------------------------------------------------

/// What a name may be, whatever construct declared it.
///
/// The set is closed rather than "whatever Typst accepts", because the error has
/// to be able to name what it accepts, and each clause is a measurement:
///
/// - `fig:one`, `fig-two`, `fig_three`, `fig.four` and `fig5` are all labels.
/// - **A name opening with `:` or `.` is not a label at all.** Typst's markup
///   enters a label only where the character after `<` continues an identifier,
///   so `#figure(…) <:foo>` typesets the literal `<:foo>` on the page and raises
///   nothing — the silent drop the dialect exists to refuse, reached through a
///   name it would otherwise have accepted.
/// - **`fn-N` is a namespace the emitter already owns**, written by the
///   `Event::FootnoteReference` arm. Those names are generated rather than
///   declared, so the duplicate check would never see the collision; Typst
///   would, with a message naming a label and no line.
///
/// These clauses are shared and the *finding* rule is not: a caption's group
/// sits at the end of a line, where an equation's is the whole of a text run.
fn check_name(name: &str, line: usize) -> Result<()> {
    let refuse = |problem: String| {
        Err(Error::Name {
            location: Location::at(line),
            problem,
        })
    };

    if name.is_empty() {
        return refuse("a name is empty".to_string());
    }
    if let Some(bad) = name
        .chars()
        .find(|ch| !(ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ':' | '.')))
    {
        return refuse(format!(
            "'{name}' holds '{bad}', and a name may hold only letters, digits, '-', '_', ':' and '.'"
        ));
    }
    if name.starts_with(':') || name.starts_with('.') {
        return refuse(format!(
            "'{name}' begins with ':' or '.', which Typst does not read as a name at all"
        ));
    }
    if let Some(digits) = name.strip_prefix("fn-")
        && !digits.is_empty()
        && digits.chars().all(|ch| ch.is_ascii_digit())
    {
        return refuse(format!("'{name}' is reserved for footnotes"));
    }

    Ok(())
}

/// The `{#…}` group a caption line ends with, and the name inside it.
///
/// Returns the group as it was typed — which is what the buffer's escaped copy
/// must be stripped of — beside the name the label takes.
///
/// The prefix is the author's convention and the dialect neither requires nor
/// reads it: the kind comes from the body Typst was handed, so `{#pipeline}` on
/// an image is a figure and so is `{#tab:pipeline}`.
fn caption_name(typed: &str, line: usize) -> Result<Option<(&str, &str)>> {
    let Some(open) = typed.rfind('{') else {
        return Ok(None);
    };
    if !typed.ends_with('}') || !typed[open..].starts_with("{#") {
        return Ok(None);
    }

    let group = &typed[open..];
    let name = &group[2..group.len() - 1];
    check_name(name, line)?;

    Ok(Some((group, name)))
}

/// The name a text run is entirely, where that run follows a display equation.
///
/// **The group must be the whole of the run**, which is the same discipline that
/// keeps `: ` a marker in one position rather than a ban: `$$…$$ {#eq:one} and
/// more` and `$$…$$ see {#eq:one}` are both the prose they have always been. The
/// run is trimmed at both ends because the parser hands the group over with the
/// space the author typed before it, so `{#eq:one}`, ` {#eq:one}` and
/// `$$…$${#eq:one}` all name.
///
/// A finding rule of its own rather than [`caption_name`]'s: that one takes the
/// *last* group on a line, so reusing it would label the leading-text shape.
fn equation_name(run: &str, line: usize) -> Result<Option<&str>> {
    let Some(name) = run
        .trim()
        .strip_prefix("{#")
        .and_then(|rest| rest.strip_suffix('}'))
    else {
        return Ok(None);
    };
    check_name(name, line)?;

    Ok(Some(name))
}

/// Record a declared name, refusing one the document has already declared.
///
/// Backward-looking, so the walk already knows: the error stands where the
/// second declaration does. Typst's own message —
/// ``label `<dup>` occurs multiple times in the document`` — names no line.
fn declare(names: &mut Names, name: &str, line: usize, equation: bool) -> Result<()> {
    if names.declared.iter().any(|seen| seen.name == name) {
        return Err(Error::Name {
            location: Location::at(line),
            problem: format!("'{name}' is declared twice"),
        });
    }
    names.declared.push(Declaration {
        name: name.to_string(),
        line,
        equation,
    });
    Ok(())
}

/// Refuse a reference the document cannot honour, naming the earliest one.
///
/// Two refusals, one pass. A reference to a name nothing declares is the first.
/// The second is a reference to an *equation* in a document that did not write
/// `equations: numbered`: both looks answer that key with
/// `set math.equation(numbering: … else { none })`, and Typst then fails the
/// whole compile with `cannot reference equation without numbering`, a message
/// naming neither the line nor the key. **Naming an equation is not refused
/// there, only pointing at one** — a labelled unnumbered equation compiles
/// perfectly well, and refusing the name would break a document that names one
/// before it points at it.
///
/// Both run after the walk rather than during it, because a reference may
/// precede its declaration and the emitter needs no pre-pass to write correctly:
/// Typst is what resolves a label. The check exists only so the error names the
/// author's own line. A declaration pre-pass would keep the ordering and costs
/// more than it is worth — a name is declared only on a caption line that
/// *attaches*, so finding one means re-running the walk that decides attachment.
///
/// **This is the one error in the pipeline that does not keep its document
/// position.** The walk aborts at the first construct error, so a document with
/// a bad reference on line 3 and a raw HTML block on line 5 reports the HTML.
///
/// The earliest line is the error, across both classes together, which
/// `min_by_key` over a `Vec` settles deterministically where "the first" out of
/// a set does not.
fn check_references(names: &Names, equations: Equations) -> Result<()> {
    let refusal = names
        .referenced
        .iter()
        .filter_map(
            |(name, line)| match names.declared.iter().find(|seen| &seen.name == name) {
                None => Some((*line, format!("nothing declares the name '{name}'"))),
                Some(seen) if seen.equation && equations != Equations::Numbered => Some((
                    *line,
                    format!(
                        "'{name}' names an equation, and a document that points at one must set 'equations: numbered'"
                    ),
                )),
                Some(_) => None,
            },
        )
        .min_by_key(|(line, _)| *line);

    match refusal {
        Some((line, problem)) => Err(Error::Name {
            location: Location::at(line),
            problem,
        }),
        None => Ok(()),
    }
}

/// Refuse a citation the document cannot honour, naming the earliest one.
///
/// **A `[@key]` in a document that names no bibliography is refused, not
/// printed.** Mapping only where the frontmatter key is present would leave it
/// reaching the page as `\\[\\@smith2020\\]` — visible, meaningless, and exactly
/// the silent flattening the dialect refuses for every other construct. Typst
/// would raise on its own, with "the document does not contain a bibliography",
/// which carries neither the construct nor the line.
///
/// This runs after the walk for the reason [`Names::cited`] records, and so it
/// shares [`check_references`]' one limit: the walk aborts at the first
/// construct error, so a document with a citation on line 3 and a raw HTML block
/// on line 5 reports the HTML.
fn check_citations(names: &Names, bibliography: Option<&BibliographyRef>) -> Result<()> {
    if bibliography.is_some() {
        return Ok(());
    }

    match names.cited.iter().min_by_key(|(_, line)| *line) {
        Some((key, line)) => Err(Error::Citation {
            location: Location::at(*line),
            problem: format!("'@{key}' is cited and the frontmatter names no bibliography"),
        }),
        None => Ok(()),
    }
}

/// Whether a finished link is a citation the callback claimed.
///
/// Two halves, and both are load-bearing. The type has to be an unresolved
/// shortcut **or** an unresolved collapsed reference — `[@k][]` is the second,
/// and one arm alone would send it to the generic link arm as `#link("@k")[@k]`,
/// a wrong document where today it is literal text. And the destination has to
/// be one [`is_citation`] claims, which is the same predicate the callback read,
/// so the parse and the emitter cannot disagree about what a citation is.
fn wrote_citation(frame: &LinkFrame) -> bool {
    matches!(
        frame.link_type,
        LinkType::ShortcutUnknown | LinkType::CollapsedUnknown
    ) && is_citation(&frame.url)
}

/// The one key a citation's payload names, or the refusal its payload earns.
///
/// Pandoc spells three more things inside these brackets, and this dialect reads
/// none of them, so each is named at its own line rather than guessed at or
/// silently dropped — OQ-3 is where whether they ever land is argued. The
/// suppressed-author arm is the `else` rather than a test of its own, because
/// [`is_citation`] has already established that a payload reaching here begins
/// `@` or `-@`.
///
/// **The key is not checked against a character set**, which is where this parts
/// from `check_name`. A figure name is authored inside this dialect and can be
/// constrained; a citation key is authored in a file the author often did not
/// write and cannot change, so a rule here would refuse real bibliographies
/// rather than protect anyone.
fn cite_key(payload: &str, line: usize) -> Result<&str> {
    let refuse = |problem: String| Error::Citation {
        location: Location::at(line),
        problem,
    };

    let Some(key) = payload.strip_prefix('@') else {
        return Err(refuse(format!(
            "'{payload}' suppresses the author, which the dialect does not read"
        )));
    };
    if key.contains(';') {
        return Err(refuse(format!(
            "'{payload}' cites several sources at once, and one citation cites one"
        )));
    }
    if key.contains(',') {
        return Err(refuse(format!(
            "'{payload}' carries a locator, which the dialect does not read"
        )));
    }
    Ok(key)
}

/// Lay a block out under a prefix: the first line takes `prefix`, and every
/// later line takes the same width in spaces.
///
/// This is what makes a list item's continuation and a quote's content indented
/// past their opener, which is how Typst reads them as belonging to it. An empty
/// line stays empty, so no line ever carries trailing whitespace, and empty
/// content leaves the prefix alone rather than a trailing space after a marker.
fn prefixed(prefix: &str, content: &str) -> String {
    if content.is_empty() {
        return prefix.trim_end().to_string();
    }

    let padding = " ".repeat(prefix.len());
    let mut out = String::new();
    for (index, line) in content.lines().enumerate() {
        if index > 0 {
            out.push('\n');
        }
        if !line.is_empty() {
            out.push_str(if index == 0 { prefix } else { &padding });
            out.push_str(line);
        }
    }
    out
}

/// Render one code block as a Typst `raw` call, without the leading `#`.
///
/// An indented block has no info string and gets no `lang` argument. The
/// content travels as a string literal rather than as markup, the way inline
/// code already does, so nothing inside a fence is escaped and nothing needs
/// to be.
fn raw_call(lang: Option<&str>, content: &str) -> String {
    let mut out = String::from("raw(block: true, ");
    if let Some(lang) = lang {
        out.push_str("lang: ");
        out.push_str(&typst_string(lang));
        out.push_str(", ");
    }
    out.push_str(&typst_string(content));
    out.push(')');
    out
}

/// Render one table as a Typst `table` call, without the leading `#`, one
/// markdown row to one line.
///
/// The column count is the alignment vector's length, and an integer `columns`
/// gives that many auto-sized columns. The header row travels as
/// `table.header`, which is what repeats it across page breaks and carries the
/// accessibility tagging; `template.typ` owns how it looks. A row a cell short
/// arrives padded, so its last cell is the empty content block `[]`.
fn table_call(frame: &TableFrame) -> String {
    let mut out = format!("table(\n  columns: {},\n", frame.align.len());

    // A delimiter row that sets no alignment at all leaves the argument out,
    // rather than naming `auto` for every column and saying nothing with it.
    if frame
        .align
        .iter()
        .any(|align| !matches!(align, Alignment::None))
    {
        let names: Vec<&str> = frame.align.iter().map(align_name).collect();
        out.push_str("  align: (");
        out.push_str(&names.join(", "));
        // One column needs the trailing comma, or Typst reads a parenthesised
        // word where an array is meant.
        if names.len() == 1 {
            out.push(',');
        }
        out.push_str("),\n");
    }

    out.push_str("  table.header(");
    out.push_str(&row_cells(&frame.header));
    out.push_str("),\n");

    for row in &frame.rows {
        out.push_str("  ");
        out.push_str(&row_cells(row));
        out.push_str(",\n");
    }

    out.push(')');
    out
}

/// One row's cells, each a content block.
fn row_cells(row: &[String]) -> String {
    row.iter()
        .map(|cell| format!("[{cell}]"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The Typst alignment for one column, `auto` where the delimiter row set none.
fn align_name(align: &Alignment) -> &'static str {
    match align {
        Alignment::None => "auto",
        Alignment::Left => "left",
        Alignment::Center => "center",
        Alignment::Right => "right",
    }
}

/// The two lines the emitter puts above every document, and the third a
/// document with math gets.
///
/// The math import is conditional because it is the only thing keeping the
/// prelude out of every shipped golden file: a document that names no formula
/// compiles to exactly the same two lines it always did. It sits between the
/// look's import and the show rule, where a reader of `--emit-typst` finds both
/// imports together.
///
/// The import names the look the frontmatter selected, and every bundled look
/// reaches the compiler as bytes in the world's virtual filesystem, so this
/// import resolves there and nowhere else. A fixed name would make two
/// documents in two looks emit identical source, and the `--emit-typst` output
/// exists to show what a document compiles to.
///
/// Every argument is named on every call, including the ones the frontmatter
/// left out, so that same output shows the layout the document actually gets.
/// A bundled look therefore accepts all six, and one missing an argument
/// would fail the compile with an error naming neither the document nor the
/// key.
///
/// `equations` and `figures` cross as Typst strings rather than through
/// `typst_string_or_none`: the schema resolves each to a name on every
/// document, so the `none` arm would be dead, and a bare `plain` reaching the
/// call unquoted fails the compile with `unknown variable: plain` — at compile
/// time rather than at the schema, naming an identifier the author never typed.
/// The name is all that crosses; what a number looks like, and whether it
/// carries the section it stands in, is the look's own.
fn header(front: &Frontmatter, math: bool) -> String {
    let prelude = match math {
        true => format!("#import \"{PRELUDE_NAME}\": {PRELUDE_NAMES}\n"),
        false => String::new(),
    };
    format!(
        "#import \"{}\": template, divider\n\
         {prelude}\
         #show: template.with(title: {}, author: {}, columns: {}, date: {}, equations: {}, figures: {})\n",
        front.template.file(),
        typst_string_or_none(front.title.as_deref()),
        typst_string_or_none(front.author.as_deref()),
        front.columns,
        typst_string_or_none(front.date.as_deref()),
        typst_string(front.equations.name()),
        typst_string(front.figures.name()),
    )
}

/// Render a frontmatter value as a Typst string literal, or `none` where the
/// document left the key out.
fn typst_string_or_none(value: Option<&str>) -> String {
    match value {
        Some(value) => typst_string(value),
        None => String::from("none"),
    }
}

/// Render a string as a Typst string literal.
///
/// This is a different escape from `escape_into`. That one escapes what markup
/// mode interprets; a string literal interprets only `\` and `"`, and escaping
/// the markup set inside one would put the backslashes into the PDF. Two kinds
/// of thing travel this way: the frontmatter's own strings — the title, the
/// author, the date and the `equations` and `figures` names — the content of
/// every `#raw` call the walk writes, for inline code and for code blocks
/// alike, and the two a citation needs, the key inside `label(…)` and the
/// bibliography's own path.
///
/// A newline is the one addition a code block needs. A literal cannot hold one,
/// and inline code never carries one, because CommonMark folds a code span's
/// line endings to spaces.
fn typst_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '\\' | '"' => {
                out.push('\\');
                out.push(ch);
            }
            '\n' => out.push_str("\\n"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

/// Append `text` to `out`, escaping everything Typst would otherwise interpret.
///
/// One rule depends on position: a `.` that follows only digits since the last
/// newline opens a Typst enumeration, so it is escaped there and left alone
/// everywhere else. That case is reachable, because CommonMark does not let an
/// ordered list whose start number is not `1` interrupt a paragraph.
fn escape_into(out: &mut String, text: &str) {
    for ch in text.chars() {
        if SPECIAL.contains(&ch) {
            out.push('\\');
            out.push(ch);
        } else if ch == '.' && line_is_all_digits(out) {
            out.push_str("\\.");
        } else {
            out.push(ch);
        }
    }
}

/// True when the current output line holds at least one digit and nothing else.
///
/// The buffer this reads is never indented while it is being written, so a list
/// item's own continuation line is tested the same way a top-level one is.
fn line_is_all_digits(out: &str) -> bool {
    let line = match out.rfind('\n') {
        Some(i) => &out[i + 1..],
        None => out,
    };
    !line.is_empty() && line.chars().all(|c| c.is_ascii_digit())
}

/// The 1-based line that a byte offset falls on.
/// How many lines a buffer holds — the line its next character would land on.
fn lines_in(text: &str) -> usize {
    text.bytes().filter(|&byte| byte == b'\n').count() + 1
}

pub(crate) fn line_of(md: &str, offset: usize) -> usize {
    md[..offset.min(md.len())]
        .bytes()
        .filter(|&b| b == b'\n')
        .count()
        + 1
}

/// A name for an out-of-dialect construct, for the error message.
///
/// Only what the walk can still reject is named here. A construct the walk
/// handles is not listed, so this function keeps telling the truth about what
/// the dialect leaves out.
///
/// Every arm below is reachable, which is what makes that claim checkable. A
/// name needs a parser option before the parser produces the event it names, so
/// an arm whose option `options` leaves off is an arm that refuses nothing while
/// the construct it names prints on the page.
fn describe(event: &Event) -> &'static str {
    match event {
        Event::Start(tag) => match tag {
            Tag::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::End(tag) => match tag {
            TagEnd::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::Html(_) | Event::InlineHtml(_) => "raw HTML",
        Event::FootnoteReference(_) => "footnote reference",
        Event::TaskListMarker(_) => "task list marker",
        // The walk handles these, so they never reach this function. The match
        // must still cover them.
        Event::Text(_)
        | Event::SoftBreak
        | Event::Code(_)
        | Event::HardBreak
        | Event::Rule
        | Event::InlineMath(_)
        | Event::DisplayMath(_) => "supported construct",
    }
}
