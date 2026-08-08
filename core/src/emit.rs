//! Walks the pulldown-cmark event stream and emits Typst markup.
//!
//! Two rules keep the dialect honest. The emitter escapes every character that
//! Typst markup mode interprets, so body text reaches the PDF verbatim. And a
//! construct outside the dialect is an error that names the construct and its
//! line, never a silent drop.

use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};

use crate::{Error, Result};

/// The header the emitter puts above every document. `template.typ` reaches the
/// compiler as bytes in the world's virtual filesystem, so this import resolves
/// there and nowhere else.
const HEADER: &str = "#import \"template.typ\": template\n#show: template.with()\n";

/// Characters that Typst markup mode interprets inside a text run.
///
/// The spec names all of these except `~` and `/`. `~` is a non-breaking space
/// in Typst, and `//` opens a line comment, so both would change the rendered
/// text if they passed through unescaped.
const SPECIAL: &[char] = &[
    '\\', '#', '$', '*', '_', '`', '@', '<', '>', '[', ']', '~', '-', '+', '=', '/',
];

/// Translate one markdown document into Typst markup.
pub(crate) fn emit(md: &str) -> Result<String> {
    let mut options = Options::empty();
    // Recognising the metadata block is what lets this phase ignore frontmatter
    // without editing the input, so every reported line number stays true to
    // the user's file. Phase 2 parses the block instead of ignoring it.
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    let mut out = String::from(HEADER);
    let mut body = String::new();
    let mut in_metadata = false;

    for (event, range) in Parser::new_ext(md, options).into_offset_iter() {
        match event {
            Event::Start(Tag::MetadataBlock(_)) => in_metadata = true,
            Event::End(TagEnd::MetadataBlock(_)) => in_metadata = false,

            Event::Start(Tag::Paragraph) => body.push('\n'),
            Event::End(TagEnd::Paragraph) => body.push('\n'),

            Event::Start(Tag::Heading { level, .. }) => {
                body.push('\n');
                for _ in 0..level as usize {
                    body.push('=');
                }
                body.push(' ');
            }
            Event::End(TagEnd::Heading(_)) => body.push('\n'),

            Event::Text(text) => {
                if !in_metadata {
                    escape_into(&mut body, &text);
                }
            }
            Event::SoftBreak => body.push('\n'),

            other => {
                return Err(Error::UnsupportedConstruct {
                    construct: describe(&other).to_string(),
                    line: line_of(md, range.start),
                });
            }
        }
    }

    out.push_str(body.trim_end_matches('\n'));
    out.push('\n');
    Ok(out)
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
fn describe(event: &Event) -> &'static str {
    match event {
        Event::Start(tag) => match tag {
            Tag::List(None) => "bullet list",
            Tag::List(Some(_)) => "ordered list",
            Tag::Item => "list item",
            Tag::Emphasis => "emphasis",
            Tag::Strong => "strong emphasis",
            Tag::Strikethrough => "strikethrough",
            Tag::CodeBlock(_) => "code block",
            Tag::BlockQuote(_) => "block quote",
            Tag::Link { .. } => "link",
            Tag::Image { .. } => "image",
            Tag::Table(_) => "table",
            Tag::FootnoteDefinition(_) => "footnote definition",
            Tag::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::End(tag) => match tag {
            TagEnd::List(false) => "bullet list",
            TagEnd::List(true) => "ordered list",
            TagEnd::Item => "list item",
            TagEnd::Emphasis => "emphasis",
            TagEnd::Strong => "strong emphasis",
            TagEnd::Strikethrough => "strikethrough",
            TagEnd::CodeBlock => "code block",
            TagEnd::BlockQuote(_) => "block quote",
            TagEnd::Link => "link",
            TagEnd::Image => "image",
            TagEnd::Table => "table",
            TagEnd::FootnoteDefinition => "footnote definition",
            TagEnd::HtmlBlock => "raw HTML block",
            _ => "markdown construct",
        },
        Event::Code(_) => "inline code",
        Event::Html(_) | Event::InlineHtml(_) => "raw HTML",
        Event::HardBreak => "hard line break",
        Event::Rule => "thematic break",
        Event::FootnoteReference(_) => "footnote reference",
        Event::TaskListMarker(_) => "task list marker",
        Event::InlineMath(_) | Event::DisplayMath(_) => "math",
        // The walk handles these two, so they never reach this function. The
        // match must still cover them.
        Event::Text(_) | Event::SoftBreak => "text",
    }
}
