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

use pulldown_cmark::{CodeBlockKind, Event, LinkType, Options, Parser, Tag, TagEnd};

use crate::frontmatter::{self, Frontmatter};
use crate::{Error, Result};

/// Characters that Typst markup mode interprets inside a text run.
///
/// The spec names all of these except `~` and `/`. `~` is a non-breaking space
/// in Typst, and `//` opens a line comment, so both would change the rendered
/// text if they passed through unescaped.
const SPECIAL: &[char] = &[
    '\\', '#', '$', '*', '_', '`', '@', '<', '>', '[', ']', '~', '-', '+', '=', '/',
];

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

/// What the walk is directly inside, for the one question that needs the answer:
/// whether a paragraph is a list item's own child.
enum Container {
    Item,
    Quote,
}

/// Translate one markdown document into Typst markup.
pub(crate) fn emit(md: &str) -> Result<String> {
    let mut options = Options::empty();
    // The parser is what recognises the frontmatter block, so nothing strips it
    // from the input and every reported line number stays true to the user's
    // file. `frontmatter.rs` then reads the text between the delimiters.
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    // Tables are outside the dialect, and this is what makes them rejectable.
    // Without the option the parser reads a pipe table as paragraph text, so
    // the pipes would reach the PDF as prose and the arm below would never see
    // the construct it is meant to name.
    options.insert(Options::ENABLE_TABLES);

    // The base buffer is the document body; a list item and a block quote push
    // one of their own and pop it as they close.
    let mut bufs = vec![String::new()];
    let mut containers: Vec<Container> = Vec::new();
    let mut lists: Vec<ListFrame> = Vec::new();
    let mut code: Option<(Option<String>, String)> = None;

    let mut in_metadata = false;
    let mut meta = String::new();
    let mut meta_offset = None;
    let mut front = Frontmatter::default();

    for (event, range) in Parser::new_ext(md, options).into_offset_iter() {
        match event {
            Event::Start(Tag::MetadataBlock(_)) => in_metadata = true,

            // The block is parsed here rather than after the walk, so a bad
            // frontmatter key is reported before any later construct error.
            Event::End(TagEnd::MetadataBlock(_)) => {
                in_metadata = false;
                let first_line = line_of(md, meta_offset.unwrap_or(range.start));
                front = frontmatter::parse(&meta, first_line)?;
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
                top(&mut bufs).push('\n');
            }
            Event::End(TagEnd::Paragraph) => top(&mut bufs).push('\n'),

            Event::Start(Tag::Heading { level, .. }) => {
                let out = top(&mut bufs);
                out.push('\n');
                for _ in 0..level as usize {
                    out.push('=');
                }
                out.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => top(&mut bufs).push('\n'),

            Event::Start(Tag::List(start)) => lists.push(ListFrame {
                next: start,
                loose: false,
                items: Vec::new(),
            }),
            Event::End(TagEnd::List(_)) => {
                let frame = lists.pop().expect("a list end follows its start");
                let separator = if frame.loose { "\n\n" } else { "\n" };
                let rendered = frame.items.join(separator);
                let out = top(&mut bufs);
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
                let out = top(&mut bufs);
                out.push_str("\n#quote(block: true)[\n");
                out.push_str(&prefixed("  ", content.trim_matches('\n')));
                out.push_str("\n]\n");
            }

            // The language tag is the first word of the info string. An
            // indented block, or an empty info string, gets no `lang` argument.
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match &kind {
                    CodeBlockKind::Fenced(info) => {
                        info.split_whitespace().next().map(str::to_string)
                    }
                    CodeBlockKind::Indented => None,
                };
                code = Some((lang, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                let (lang, mut content) = code.take().expect("a block end follows its start");
                // pulldown-cmark reports the final line's terminator as part of
                // the content, and a string literal that kept it would typeset a
                // phantom empty line after every code block.
                if content.ends_with('\n') {
                    content.pop();
                }
                let out = top(&mut bufs);
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
                } else if in_metadata {
                    meta_offset.get_or_insert(range.start);
                    meta.push_str(&text);
                } else {
                    escape_into(top(&mut bufs), &text);
                }
            }
            Event::SoftBreak => top(&mut bufs).push('\n'),

            // The function forms, not Typst's own `_…_` and `*…*`. Those
            // delimiters are word-boundary sensitive and CommonMark permits
            // intraword emphasis, so `foo*bar*baz` would reach the PDF with
            // literal underscores through one and would not compile at all
            // through the other.
            Event::Start(Tag::Emphasis) => top(&mut bufs).push_str("#emph["),
            Event::End(TagEnd::Emphasis) => top(&mut bufs).push(']'),
            Event::Start(Tag::Strong) => top(&mut bufs).push_str("#strong["),
            Event::End(TagEnd::Strong) => top(&mut bufs).push(']'),

            // The content is a string literal, never the markup escape, so it
            // reaches the PDF verbatim whatever it holds.
            Event::Code(inline) => {
                let out = top(&mut bufs);
                out.push_str("#raw(");
                out.push_str(&typst_string(&inline));
                out.push(')');
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

                let out = top(&mut bufs);
                out.push_str("#link(");
                out.push_str(&typst_string(&url));
                out.push_str(")[");
            }
            Event::End(TagEnd::Link) => top(&mut bufs).push(']'),

            // Typst's line break is a `\` before a newline. The same `\`
            // directly before text is an escape sequence instead.
            Event::HardBreak => top(&mut bufs).push_str("\\\n"),

            // The emitter names the rule and owns nothing about its look.
            Event::Rule => top(&mut bufs).push_str("\n#divider()\n"),

            other => {
                return Err(Error::UnsupportedConstruct {
                    construct: describe(&other).to_string(),
                    line: line_of(md, range.start),
                });
            }
        }
    }

    let body = bufs.pop().expect("the document body outlives the walk");
    let mut out = header(&front);
    out.push_str(body.trim_end_matches('\n'));
    out.push('\n');
    Ok(out)
}

/// The buffer the walk is currently writing into.
fn top(bufs: &mut [String]) -> &mut String {
    bufs.last_mut().expect("the document body is always open")
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

/// The two lines the emitter puts above every document.
///
/// `template.typ` reaches the compiler as bytes in the world's virtual
/// filesystem, so this import resolves there and nowhere else. Every argument
/// is named on every call, including the ones the frontmatter left out, so the
/// `--emit-typst` output shows the layout the document actually gets.
fn header(front: &Frontmatter) -> String {
    format!(
        "#import \"template.typ\": template, divider\n\
         #show: template.with(title: {}, author: {}, columns: {})\n",
        typst_string_or_none(front.title.as_deref()),
        typst_string_or_none(front.author.as_deref()),
        front.columns,
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
fn describe(event: &Event) -> &'static str {
    match event {
        Event::Start(tag) => match tag {
            Tag::Strikethrough => "strikethrough",
            Tag::Image { .. } => "image",
            Tag::Table(_) => "table",
            Tag::FootnoteDefinition(_) => "footnote definition",
            Tag::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::End(tag) => match tag {
            TagEnd::Strikethrough => "strikethrough",
            TagEnd::Image => "image",
            TagEnd::Table => "table",
            TagEnd::FootnoteDefinition => "footnote definition",
            TagEnd::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::Html(_) | Event::InlineHtml(_) => "raw HTML",
        Event::FootnoteReference(_) => "footnote reference",
        Event::TaskListMarker(_) => "task list marker",
        Event::InlineMath(_) | Event::DisplayMath(_) => "math",
        // The walk handles these, so they never reach this function. The match
        // must still cover them.
        Event::Text(_) | Event::SoftBreak | Event::Code(_) | Event::HardBreak | Event::Rule => {
            "supported construct"
        }
    }
}
