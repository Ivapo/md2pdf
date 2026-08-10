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
  escape rule, the rejection rule, the two walks footnotes need, the image asset
  channel, the template's title block and column toggle, the Typst world and its
  bundled fonts, and the CLI contract
max_lines: 245
generated: 2026-08-09
---

# Pipeline

Markdown in, PDF out, in three steps. `pulldown-cmark` parses;
`core/src/emit.rs:collect_definitions` and `core/src/emit.rs:emit` walk the event stream
and write Typst markup; the embedded Typst compiler produces the PDF.
`core/src/lib.rs:md_to_typst` returns the source. `core/src/lib.rs:md_to_pdf` takes the
markdown and a slice of `core/src/lib.rs:Asset` — one image file each, named by the path
the markdown wrote — and returns the bytes; `core/src/lib.rs:image_paths` is the shopping
list that names those files, in the order a reader meets them, which puts an image inside
a footnote definition at the first reference to that footnote. None of the three touches
the filesystem, the clock, or the network; `cli/src/main.rs` does all file I/O and all
terminal output, the image files included.

## The dialect

Seventeen things are supported: headings at levels 1–6, paragraph text, soft breaks,
emphasis, strong emphasis, inline code, hard line breaks, thematic breaks, links,
images, bullet lists, ordered lists, code blocks, block quotes, pipe tables, footnotes,
and a leading YAML frontmatter block. Heading levels map to Typst headings of the same
level.

The inline constructs reach Typst as function calls, not as its own markup.
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

An image is `#image(…)` or `#box(image(…))`; the section below holds the whole subject.

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

A footnote reference is `#footnote[…]<fn-N>` at its first use and `#footnote(<fn-N>)` at
every later one, Typst's own form for pointing at a footnote that already exists. `N`
counts labels by first use, and the emitter writes no numbers: Typst numbers footnotes in
placement order, which is the order GFM numbers them in. The user's label text never
reaches the output, because a markdown label may hold any character and a Typst label may
not. `Options::ENABLE_FOOTNOTES` is what parses one at all.

Typst takes a footnote's content at the reference site while a markdown definition may
sit after the reference, so one document takes two walks.
`core/src/emit.rs:collect_definitions` enters the definition regions and nothing else,
each through the arms that already exist and a buffer on the same stack a list item uses;
`core/src/emit.rs:emit` then writes the document and skips those regions, with
`core/src/emit.rs:Notes` holding what the first walk found. Labels are matched under
Unicode case folding, through `core/src/emit.rs:Label`, because the parser keys its own
label map that way: `[^A]` cites `[^a]:`, and matching the raw spelling would miss on
valid input. A reference the parser cannot resolve produces no event at all and stays
literal text, so a dangling reference needs no error shape.

Three footnote shapes are errors, and `core/src/emit.rs:Notes::enter` names them where
the definition sits: one no reference cites, which would reach no page; a second
definition for a label already defined, which would lose a body; and a reference inside a
definition, which would need a recursive substitution with a cycle check. The first walk
never raises. A definition whose translation failed keeps that error, and the second walk
reports it at the region, so the first error in document order is still the one reported
and the frontmatter still wins over a construct error below it.

**Raw HTML is an error.** `core/src/emit.rs:describe` names the construct, and
`Error::UnsupportedConstruct` carries that name with the 1-based line. The CLI prints it
to stderr and exits 1.

Two constructs are neither supported nor refused, and that is a gap rather than a
decision. Strikethrough and math each need a parser option, neither is set, so `~~x~~`
and `$x$` arrive as text and the escape rule prints them on the page. `describe` names
both on arms nothing reaches. Footnotes flattened the same way until they were mapped.

Two link shapes are errors too, and the link arm names them itself rather than through
`describe`. An empty destination, legal CommonMark, would reach Typst as `#link("")`,
whose compile error names neither construct nor line; the test is on the resolved
destination, so a reference definition with an empty destination is caught with it. A
non-empty link title is something neither `link` nor the PDF can carry, so passing the link
on would drop it. An empty title is not a title, and the link stays in-dialect.

## Images and their files

An image is `#image("…", alt: "…")` where its paragraph holds it and nothing else, and
`#box(image(…))` everywhere else — Typst lays an image out as a block and documents `box`
as the inline form, so a bare call mid-sentence would split the paragraph around it. The
form is known one event late, so `core/src/emit.rs:emit` parks the finished call in a
pending slot and the next event settles it; `core/src/emit.rs:write_image` writes either.
The path and the alt go through `typst_string`, and an empty alt leaves the argument out.
No size is emitted: with neither dimension forced, Typst bounds an image by the available
space and keeps its aspect ratio.

Alt text is flattened, not emitted, because Typst's `alt` is a plain string.
`core/src/emit.rs:AltCapture` takes every event between the image's two: text and code
contribute their text, a soft or hard break contributes one space, emphasis, strong and
link wrappers contribute nothing, and an out-of-dialect construct still errors. A nested
image flattens the same way under a depth count, contributing only its inner text; its
destination and title are not content under that reading, so they are neither checked nor
listed.

`core/src/emit.rs:check_image` refuses seven destination shapes, each an
`UnsupportedConstruct` naming the shape and the line, so `--emit-typst` rejects them too.
An empty destination and a title mirror the link arm. A URI scheme is a fetch request and
nothing fetches, which catches `data:` and the drive path `C:\figure.png` with it; an
absolute path converts on one machine only; a `..` segment escapes the document's
directory and the world's virtual root; a backslash is a segment `VirtualPath` cannot
hold. Last, the extension must sit in Typst's own table — `png`, `jpg`, `jpeg`, `gif`,
`webp`, `svg`, `svgz`, `pdf` — read case-sensitively through `VirtualPath::extension`,
the function Typst's own detection reads.

`core/src/lib.rs:collect` then checks the bytes before the compile, once per path at its
first reference: no asset is `Error::MissingImage`, bytes that disagree with the extension
are `Error::ImageFormat`, and both name the path and the line. That order is the point —
Typst's own error would name a span in `main.typ`, which the user has never seen.
`core/src/lib.rs:bytes_match` mirrors typst-library 0.15.1 by hand: the magic bytes for
the raster formats and PDF, the gzip magic for `svgz`, and a search for the SVG namespace
over the first 2048 bytes. Typst's fallback to content detection is not mirrored, because
the emitter has already refused every extension it would apply to. A file corrupt past its
magic still fails at compile time, with the compiler's own message.

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

A footnote gets no rule, for the reason lists and quotes get none: the Typst default
already sets one apart, with a superscript marker, a separator, and the note at the foot
of the column that holds the reference — the column, not the page, because the composer
keeps its footnote insertions per column.

## The world

`core/src/lib.rs:TypstWorld` holds two source files, `main.typ` and `template.typ`, and
the images the document names, all under `VirtualRoot::Project`. It implements no package
resolution, so no import can reach the network on any target.

An asset rides that same virtual filesystem rather than a channel of its own, because
`World::file` already serves `template.typ` from it. `main.typ` sits at the virtual root,
so a relative path in the generated source resolves to the file id built from that same
path, and `collect` keys the map by it. `World::source` is untouched by the assets — an
image is never Typst source — which is what keeps the import story exactly as it was.

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

`cli/src/main.rs:read_assets` fills the shopping list: each path joins the parent
directory of the input file, so a figure is found beside the document and not beside the
current directory, and a repeated path is read once. The asset keeps the path the
markdown wrote, never the resolved one, because that is the name the generated source
asks for. A file that will not read is exit 1 naming the resolved path, the line, and the
message the OS gave. `--emit-typst` returns before that call, so emission reads no image
and works on a document whose figures are absent.
