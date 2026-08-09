---
title: pipeline
sources:
  - core/src/lib.rs
  - core/src/emit.rs
  - core/src/frontmatter.rs
  - core/assets/template.typ
  - cli/src/main.rs
covers: >
  the markdown-to-PDF pipeline: the supported dialect, the frontmatter schema, the
  escape rule, the rejection rule, the template's title block and column toggle,
  the Typst world and its bundled fonts, and the CLI contract
max_lines: 155
generated: 2026-08-09
---

# Pipeline

Markdown in, PDF out, in three steps. `pulldown-cmark` parses; `core/src/emit.rs:emit`
walks the event stream and writes Typst markup; the embedded Typst compiler produces the
PDF. `core/src/lib.rs:md_to_typst` returns the source, `core/src/lib.rs:md_to_pdf` the
bytes. Neither touches the filesystem, the clock, or the network; `cli/src/main.rs` does
all file I/O and all terminal output.

## The dialect

Fifteen things are supported: headings at levels 1–6, paragraph text, soft breaks,
emphasis, strong emphasis, inline code, hard line breaks, thematic breaks, links, bullet
lists, ordered lists, code blocks, block quotes, pipe tables, and a leading YAML
frontmatter block. Heading levels map to Typst headings of the same level.

The six inline constructs reach Typst as function calls, not as its own markup.
`#emph[…]` and `#strong[…]`, because Typst's `_…_` and `*…*` are word-boundary sensitive
while CommonMark permits intraword emphasis, so `foo*bar*baz` would either keep literal
underscores or fail to compile. Inline code is `#raw("…")`, its content a string literal
through `core/src/emit.rs:typst_string`. A hard break is a `\` before a newline; the same
`\` before text is an escape sequence instead. A thematic break calls
`core/assets/template.typ:divider`, whose look the emitter decides nothing about.

A link is `#link("…")[…]`. The inline, reference and autolink forms all arrive as one
`Tag::Link` with the destination already resolved, so one arm serves them all; an
unresolved reference produces no link event at all and stays literal text. The URL goes
through `typst_string`, so a `#` in a destination survives, and the text keeps the markup
escape, which is what stops Typst reading an autolink's own text as a second link. An
email autolink's destination is the bare address, because pulldown-cmark leaves the scheme
to the renderer, so the emitter prepends `mailto:`.

Lists become `- ` and `N. ` items, every ordered item carrying its own number, so a start
other than 1 needs no mechanism of its own. A code block becomes
`#raw(block: true, lang: …)`, the tag the first word of the fence's info string and absent
for an indented block; one trailing newline is stripped, because pulldown-cmark reports the
final line's terminator as part of the content and a literal that kept it would typeset a
phantom empty line. A block quote becomes `#quote(block: true)[…]`. Neither lists nor
quotes get a template rule: the Typst defaults stand.

Tightness passes through structurally. pulldown-cmark wraps a loose list's item content in
paragraph events, and Typst derives `tight` from the blank lines between items and lets no
`set` rule override it, so the emitter writes adjacent items for one and separated items for
the other, and owns nothing about the spacing. Nesting is by indentation, which
`core/src/emit.rs:prefixed` applies. A list item and a block quote each open a buffer in a
stack and indent it as they close, so nothing is ever indented while it is written — which
keeps `core/src/emit.rs:line_is_all_digits` reading an un-indented line, and is why a code
block, one markup line holding a string literal, survives the indentation around it.

A table becomes `#table(columns: N, align: (…), table.header(…), …)`, one markdown row to
one line. `Options::ENABLE_TABLES` is what parses it at all, a pipe table being GFM rather
than CommonMark. The column count is the alignment vector's length, which an integer
`columns` turns into that many auto-sized columns; `align` maps `None` to `auto` and is
omitted where the delimiter row set none. `table.header` repeats the header row across a
page break and carries the accessibility tagging. `core/src/emit.rs:TableFrame` holds the
one table a walk can be inside, since a GFM table never nests, and each cell opens a buffer
on the same stack, so the inline arms serve cell content unchanged. The emitter counts no
cells: pulldown-cmark pads a short row and drops the excess, following GFM, so the padding
arrives as the empty content block `[]`.

**Everything else is an error** — an image, raw HTML, a footnote, strikethrough, and math.
`core/src/emit.rs:describe` names the construct, and `Error::UnsupportedConstruct` carries
that name with the 1-based line. The CLI prints it to stderr and exits 1. Nothing is ever
dropped or flattened silently.

Two link shapes are errors too, and the link arm names them itself rather than through
`describe`. An empty destination, legal CommonMark, would reach Typst as `#link("")`,
whose compile error names neither construct nor line; the test is on the resolved
destination, so a reference definition with an empty destination is caught with it. A
non-empty link title is something neither `link` nor the PDF can carry, so passing the link
on would drop it. An empty title is not a title, and the link stays in-dialect.

## The frontmatter

Three keys: `title` and `author`, optional strings, and `columns`, `1` or `2`. An absent
block is valid; `core/src/frontmatter.rs:Frontmatter::default` gives no title block and two
columns. Every document gets those, because `core/src/emit.rs:header` always names all three.

`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` recognises the block, so nothing strips it
from the input and every reported line number stays true to the user's file.
`core/src/frontmatter.rs:parse` runs at `TagEnd::MetadataBlock`, inside the walk, so a bad
key is reported before any later construct error.

That parser is hand-written over a documented YAML subset, not a dependency, and applies
the dialect's policy: one `key: value` scalar per line, blank and `#` comment lines
skipped, one pair of quotes stripped. Nesting, a missing colon, an unknown key, a repeated
key, and any other `columns` value are `Error::Frontmatter`, naming the key and the line.

## The escape rule

`core/src/emit.rs:escape_into` backslash-escapes every occurrence of
`\ # $ * _ ` @ < > [ ] ~ - + = /`. The spec names all of these except `~`, a non-breaking
space in Typst, and `/`, which opens a `//` line comment.

One rule depends on position: a `.` is escaped when every character before it on the
output line is a digit, because `2. text` at a line start opens a Typst enumeration.
`core/src/emit.rs:line_is_all_digits` is that test.

`core/src/emit.rs:typst_string` is a second, smaller escape, for what reaches Typst as a
string literal rather than as markup: the title, the author, every `#raw` content, and
every link destination. A literal interprets only `\` and `"`, and the markup escape
inside one would reach the PDF.
A newline becomes `\n`, which only a code block needs, because CommonMark folds a code
span's line endings to spaces.

Quotation marks in body text are **not** escaped. `core/assets/template.typ` sets
`smartquote(enabled: false)` instead, which is why they still reach the PDF verbatim.

## The template

`core/assets/template.typ:template` owns all styling: page, fonts, heading style, title
block, and column count. The emitter passes the three frontmatter keys and adds no styling
of its own, so a new look is a new `.typ` file. Its own defaults never reach a document.

The title block uses `place(scope: "parent", float: true)`, which lifts it out of the
column grid so it spans the page; Typst supports that scope only together with `float`. A
document with neither `title` nor `author` gets no title block at all.

`show table.cell.where(y: 0): strong` sets a table's header row in strong type. A GFM table
has exactly one header row and it is always the first, so row zero is the header by
construction. Typst's own default sets it in body type, which would flatten a distinction
the markdown source draws.

## The world

`core/src/lib.rs:TypstWorld` holds exactly two files, `main.typ` and `template.typ`, both
under `VirtualRoot::Project`. It implements no package resolution, so no import can reach
the network on any target.

Fonts are embedded with `include_bytes!` from `core/assets/fonts/`, under the OFL: five
faces from one Libertinus release, so their metrics agree. Serif Regular, Bold, Italic and
BoldItalic carry body text; Libertinus Mono carries `#raw`, which
`core/assets/template.typ:template` names in a `show raw` rule. Every face the dialect can
reach is bundled, because Typst renders the closest match it finds and synthesises none —
without the italic, `#emph` would come out as body text. No target discovers fonts from
the OS, so the same markdown compiles to the same PDF on every machine.

`World::today` returns `None`. The spec's §2 lists the current date among what the world
supplies, but an OS clock would break the same section's no-OS-access rule and make the
PDF differ between machines. No template uses a date.

## The CLI

`md2pdf input.md [-o output.pdf] [--emit-typst]`. Without `-o` the PDF lands at the input
path with a `.pdf` extension. `--emit-typst` prints the Typst source and ignores `-o`; that
output names `template.typ`, which exists only inside the world, so it serves inspection
and not a standalone `typst compile`.
