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

use pulldown_cmark::{Alignment, CodeBlockKind, Event, LinkType, Options, Parser, Tag, TagEnd};
use typst::syntax::VirtualPath;
use unicase::UniCase;

use crate::frontmatter::{self, Frontmatter};
use crate::math;
use crate::{Error, ImageRef, Result};

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
const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "svg", "svgz", "pdf"];

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
    /// That block's text, as its events deliver it.
    meta: String,
    /// Where that text starts, so its errors name the user's own lines.
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
    /// Where the open paragraph began in the buffer that holds it.
    para: Option<usize>,
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
            para: None,
            math: false,
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

/// What one walk of the definitions found.
#[derive(Default)]
struct Definitions {
    /// Each definition's translated content, the images inside it and whether it
    /// carried math, or the first error its translation produced. A label
    /// defined twice is held once: the second definition is refused where it
    /// stands.
    ///
    /// The math flag travels with the content for the same reason the images do.
    /// This walk's own `Walk` is thrown away, and the document's walk never
    /// enters a definition's region, so a formula that appears only inside a
    /// footnote would otherwise reach the page with no prelude imported.
    bodies: HashMap<Label, Result<(String, Vec<ImageRef>, bool)>>,
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
    fn body(&self, label: &Label) -> &Result<(String, Vec<ImageRef>, bool)> {
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
                line,
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
fn collect_definitions(md: &str) -> Definitions {
    let mut found = Definitions::default();
    let mut walk = Walk::new();
    let mut open: Option<Label> = None;
    let mut failure: Option<Error> = None;

    for (event, range) in Parser::new_ext(md, options()).into_offset_iter() {
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
            let body = match failure.take() {
                Some(error) => Err(error),
                None => {
                    let math = walk.math;
                    let (content, images) = std::mem::replace(&mut walk, Walk::new()).finish();
                    Ok((content.trim_matches('\n').to_string(), images, math))
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
            && let Err(error) = step(&mut walk, md, event, range, Mode::Definition)
        {
            failure = Some(error);
        }
    }

    found
}

/// Translate one markdown document into Typst markup.
///
/// The second part of the result is every image the document names, in the
/// order a reader meets them — which puts an image inside a footnote definition
/// at the first reference to that footnote, where its content is set. One walk
/// writes both, so the source and the shopping list cannot disagree about which
/// paths the dialect accepts.
///
/// The third is the line each heading sits on, in document order, which
/// [`crate::md_to_pdf_with_anchors`] pairs with the pages the compiled headings
/// landed on. It comes from the same walk for the same reason.
pub(crate) fn emit(md: &str) -> Result<(String, Vec<ImageRef>, Vec<usize>)> {
    let found = collect_definitions(md);
    let mut notes = Notes {
        found: &found,
        seen: HashSet::new(),
        numbers: HashMap::new(),
        skipping: false,
    };

    let mut walk = Walk::new();
    for (event, range) in Parser::new_ext(md, options()).into_offset_iter() {
        step(&mut walk, md, event, range, Mode::Document(&mut notes))?;
    }

    // Taken off the walk before `finish` consumes it, the way the math flag
    // already is, so `Walk::finish` and `collect_definitions` need no change.
    let headings = std::mem::take(&mut walk.headings);

    let mut out = header(&walk.front, walk.math);
    let (body, images) = walk.finish();
    out.push_str(body.trim_end_matches('\n'));
    out.push('\n');
    Ok((out, images, headings))
}

/// Translate one event into the walk's own state.
fn step(
    walk: &mut Walk,
    md: &str,
    event: Event,
    range: Range<usize>,
    mut mode: Mode,
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
        para,
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
                    line: line_of(md, range.start),
                });
            }
        }
        return Ok(());
    }

    // Which form an image takes is known one event late, so the finished
    // call waits here until the next event settles it.
    if let Some((call, opened)) = pending.take() {
        let standalone = opened && matches!(&event, Event::End(TagEnd::Paragraph));
        write_image(bufs, &call, standalone);
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
        }
        Event::End(TagEnd::Paragraph) => {
            *para = None;
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
            let out = top(bufs);
            out.push('\n');
            out.push_str(&table_call(&frame));
            out.push('\n');
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
            let out = top(bufs);
            out.push_str("\n#raw(block: true, ");
            if let Some(lang) = lang {
                out.push_str("lang: ");
                out.push_str(&typst_string(&lang));
                out.push_str(", ");
            }
            out.push_str(&typst_string(&content));
            out.push_str(")\n");
        }

        Event::Text(text) => {
            if let Some((_, content)) = code.as_mut() {
                // Code is not markup, so it is not escaped here. It reaches
                // Typst as a string literal, like inline code before it.
                content.push_str(&text);
            } else if *in_metadata {
                meta_offset.get_or_insert(range.start);
                meta.push_str(&text);
            } else {
                escape_into(top(bufs), &text);
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
            let out = top(bufs);
            out.push_str("$ ");
            out.push_str(&markup);
            out.push_str(" $");
            *math = true;
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
                    line: line_of(md, range.start),
                });
            }
            if !title.is_empty() {
                return Err(Error::UnsupportedConstruct {
                    construct: "link with a title".to_string(),
                    line: line_of(md, range.start),
                });
            }

            // Every other form arrives with its destination resolved, so
            // the email autolink is the one case with work left: its
            // destination is the bare address, and the scheme is ours.
            let url = match link_type {
                LinkType::Email => format!("mailto:{dest_url}"),
                _ => dest_url.into_string(),
            };

            let out = top(bufs);
            out.push_str("#link(");
            out.push_str(&typst_string(&url));
            out.push_str(")[");
        }
        Event::End(TagEnd::Link) => top(bufs).push(']'),

        // An image is the one construct that needs a file, so this arm is
        // where the pipeline decides which files it will ever ask for.
        // `check_image` refuses every shape it cannot carry; what survives
        // opens the alt capture and joins the shopping list.
        Event::Start(Tag::Image {
            dest_url, title, ..
        }) => {
            let line = line_of(md, range.start);
            check_image(&dest_url, &title, line)?;
            images.push(ImageRef {
                path: dest_url.to_string(),
                line,
            });
            *alt = Some(AltCapture {
                opened: *para == Some(top(bufs).len()),
                path: dest_url.into_string(),
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
                        line,
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
                    line: line_of(md, range.start),
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
                    if let Ok((content, inside, inside_math)) = notes.found.body(&label) {
                        out.push_str(content);
                        images.extend(inside.iter().cloned());
                        *math |= inside_math;
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
                line: line_of(md, range.start),
            });
        }
    }

    Ok(())
}

/// The buffer the walk is currently writing into.
fn top(bufs: &mut [String]) -> &mut String {
    bufs.last_mut().expect("the document body is always open")
}

// -- images -----------------------------------------------------------------

/// Refuse every image destination the pipeline cannot carry, naming the shape.
///
/// The first two mirror the link arm, for the same two reasons. The next three
/// are the relative-path rule: a scheme is a fetch request and nothing fetches,
/// an absolute path converts on one machine only, and a `..` segment escapes
/// both the document's directory and the world's virtual root. The sixth is a
/// path Typst's own virtual filesystem cannot hold; a Windows separator is the
/// way to write one. The last is the format gate's first half — Typst reads the
/// extension before the content, so an extension it does not name leaves the
/// format undecided, and the dialect refuses to guess.
fn check_image(dest: &str, title: &str, line: usize) -> Result<()> {
    let refuse = |construct: String| Err(Error::UnsupportedConstruct { construct, line });

    if dest.is_empty() {
        return refuse("image with an empty destination".to_string());
    }
    if !title.is_empty() {
        return refuse("image with a title".to_string());
    }
    if has_scheme(dest) {
        return refuse("image with a URL destination".to_string());
    }
    if dest.starts_with('/') {
        return refuse("image with an absolute path".to_string());
    }
    if dest.split('/').any(|segment| segment == "..") {
        return refuse("image with a '..' path segment".to_string());
    }

    let Ok(vpath) = VirtualPath::new(dest) else {
        return refuse("image with a backslash in its path".to_string());
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

/// Render one table as a Typst `table` call, one markdown row to one line.
///
/// The column count is the alignment vector's length, and an integer `columns`
/// gives that many auto-sized columns. The header row travels as
/// `table.header`, which is what repeats it across page breaks and carries the
/// accessibility tagging; `template.typ` owns how it looks. A row a cell short
/// arrives padded, so its last cell is the empty content block `[]`.
fn table_call(frame: &TableFrame) -> String {
    let mut out = format!("#table(\n  columns: {},\n", frame.align.len());

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
/// A bundled look therefore accepts all four, and one missing an argument
/// would fail the compile with an error naming neither the document nor the
/// key.
fn header(front: &Frontmatter, math: bool) -> String {
    let prelude = match math {
        true => format!("#import \"{PRELUDE_NAME}\": {PRELUDE_NAMES}\n"),
        false => String::new(),
    };
    format!(
        "#import \"{}\": template, divider\n\
         {prelude}\
         #show: template.with(title: {}, author: {}, columns: {}, date: {})\n",
        front.template.file(),
        typst_string_or_none(front.title.as_deref()),
        typst_string_or_none(front.author.as_deref()),
        front.columns,
        typst_string_or_none(front.date.as_deref()),
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
/// the markup set inside one would put the backslashes into the PDF. Three
/// things travel this way: the title, the author, and the content of every
/// `#raw` call the walk writes, for inline code and for code blocks alike.
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
fn line_of(md: &str, offset: usize) -> usize {
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
