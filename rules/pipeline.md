---
title: pipeline
sources:
  - core/src/lib.rs
  - core/src/emit.rs
  - core/src/frontmatter.rs
  - core/src/math.rs
  - core/assets/math.typ
  - core/assets/template.typ
  - core/assets/press-release.typ
  - cli/src/main.rs
covers: >
  the markdown-to-PDF pipeline: the supported dialect, the frontmatter schema, the
  escape rule, the rejection rule, the two walks footnotes need, the image asset
  channel, the LaTeX subset a formula may hold and the prelude it compiles
  against, the bundled looks and the call contract they meet, the Typst world and
  its bundled fonts, and the CLI contract
max_lines: 340
generated: 2026-08-14
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

Nineteen things are supported: headings at levels 1–6, paragraph text, soft breaks,
emphasis, strong emphasis, strikethrough, inline code, inline math, hard line breaks,
thematic breaks, links, images, bullet lists, ordered lists, code blocks, block quotes,
pipe tables, footnotes, and a leading YAML frontmatter block. Heading levels map to Typst headings of
the same level.

The inline constructs reach Typst as function calls, not as its own markup.
`#emph[…]` and `#strong[…]`, because Typst's `_…_` and `*…*` are word-boundary sensitive
while CommonMark permits intraword emphasis, so `foo*bar*baz` would either keep literal
underscores or fail to compile. `#strike[…]` because Typst has no markup form for a strike
at all; `Options::ENABLE_STRIKETHROUGH` parses one, and a delimiter run of one tilde counts
as well as one of two, so `~struck~` strikes as `~~struck~~` does. Inline code is
`#raw("…")`, its content a string literal through `core/src/emit.rs:typst_string`. A hard
break is a `\` before a newline; the same `\` before text is an escape sequence instead. A
thematic break calls `divider`, which every look exports and whose look the emitter decides
nothing about.

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

**Everything else is an error** — raw HTML, a task list marker, and display math.
`core/src/emit.rs:describe` names the construct, `Error::UnsupportedConstruct` carries
that name with the 1-based line, and the CLI prints it to stderr and exits 1. Nothing is
dropped or flattened silently.

Every arm of `describe` is reachable, which is a property rather than an accident: a name
refuses nothing until a parser option produces the event it names, and
`Options::ENABLE_TASKLISTS` and `Options::ENABLE_MATH` are what make the last two arrive.
Typst has no checkbox element, and a drawn marker would be a look decision the template
owns. Display math is refused whole, which is what keeps that arm reachable now the
inline form converts; the section below holds the inline form.

Two link shapes are errors too, and the link arm names them itself rather than through
`describe`. An empty destination, legal CommonMark, would reach Typst as `#link("")`,
whose compile error names neither construct nor line; the test is on the resolved
destination, so a reference definition with an empty destination is caught with it. A
non-empty link title is something neither `link` nor the PDF can carry, so passing the link
on would drop it. An empty title is not a title, and the link stays in-dialect.

## Math

An inline `$…$` span becomes `$…$` of Typst markup: `core/src/math.rs:convert` scans the
LaTeX, `mitex::convert_math` converts it, and the result is written between the delimiters
**unescaped**, because it is markup by then — through `escape_into` a fraction would set as
letters. A display `$$…$$` span is still an error.

The scan runs on what the author wrote, ahead of the conversion, and refuses anything off
three closed lists in `core/src/math.rs` — `COMMANDS` (the Greek letters in both cases, the
relations, the operators, the set and logic commands, the large operators, the constants,
`\frac \sqrt \binom \left \right`, the accents, the font commands, the named operators),
`SYMBOLS` (`\\ \, \; \: \! \{ \} \% \& \_ \# \$`), and `ENVIRONMENTS`
(`matrix pmatrix bmatrix vmatrix cases aligned`, matched case-sensitively). An unescaped
`%` is refused too. `core/src/math.rs:walk` is the one traversal that finds them, which is
what lets a test read the fixture back through the same rule the refusal uses.
`Error::Math` carries the problem and the 1-based line: `math error at line 3: unsupported
command '\notacommand'`.

It reads the input rather than the converted output because the failures that matter are
invisible in that output. `\includegraphics` converts to an `#image` call for a path
`check_image` never saw; `\label` to the empty string; `\begin{itemize}` to markup-mode
list syntax Typst then reads as an operator; and `%` opens a LaTeX comment, so mitex drops
the rest of the line and the PDF shows truncated prose. `\%` is that one's one-character
exit. `\text` is off the list for a reason of its own: its argument reaches the page as
Typst markup, where `\text{= head}` sets a heading and `\text{a \alpha b}` sets the words
"a alpha b". `\$` is still the exit math leaves prose — the backslash suppresses the span,
and the dollar reaches the page as itself.

What mitex repairs is accepted rather than fixed. `$\frac{a}{$` converts to `frac(a ,zws )`
and sets a fraction with an empty denominator: wrong on the page, which is the property the
`%` case lacked, and closing it would mean validating LaTeX group structure here.
`core/src/math.rs:normalise` collapses the markup to one line and keeps it from ending in a
backslash, which would escape the `$` that closes the equation.

`core/assets/math.typ` defines the ten names mitex writes that Typst 0.15.1 does not: the
four matrix environments, `aligned`, `mitexsqrt`, `mitexmathbf`, `negthinspace`, and `sect`
and `diff` — mitex writing the pre-0.13 spellings of `inter` and `partial`. That set is
derived, not chosen: convert every allowed command, take the multi-character heads, subtract
Typst's global, `math` and `sym` scopes, define the remainder. So it moves with the Typst
version, and `tests/fixtures/math.md` is what proves it complete — one formula per allowed
command, where a missing definition is a compile error. `core/src/emit.rs:header` imports
those names, and only for a document that has math, so every golden file written before
still opens with the same two lines. The flag rides a footnote definition's content the way
its images do, because that walk is discarded before the header is written.

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
contribute their text, a soft or hard break contributes one space, emphasis, strong,
strikethrough and link wrappers contribute nothing, and an out-of-dialect construct still
errors. A nested
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

Five keys, all optional: `title` and `author`, strings; `date`, a free string the template
typesets verbatim; `columns`, `1` or `2`; and `template`, the look, `article` or
`press-release`. An absent block is valid, and `core/src/frontmatter.rs:Frontmatter::default`
gives the article look, no title block, no date and two columns. Every document gets a value
for all five, because `core/src/emit.rs:header` always names all four arguments and the
selected file.

An absent `columns` takes the selected look's convention — `2` for `article`, `1` for
`press-release` — resolved in `core/src/frontmatter.rs:parse` after the whole block is read,
because `template` may sit below `columns`. An explicit value wins either way. The schema is
the home of every default; a template's own defaults are the fallback for a hand-written
call and never reach a document.

`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` recognises the block, so nothing strips it
from the input and every reported line number stays true to the user's file.
`core/src/frontmatter.rs:parse` runs at `TagEnd::MetadataBlock`, inside the walk, so a bad
key is reported before any later construct error.

That parser is hand-written over a documented YAML subset, not a dependency, and applies
the dialect's policy: one `key: value` scalar per line, blank and `#` comment lines
skipped, one pair of quotes stripped. Nesting, a missing colon, an unknown key, a repeated
key, any other `columns` value, and a `template` name outside the set are
`Error::Frontmatter`, naming the key and the line. The `template` error lists the names it
accepts.

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

Quotation marks in body text are **not** escaped. Every look sets
`smartquote(enabled: false)` instead, which is why they still reach the PDF verbatim.

## The templates

Two looks are bundled, and `core/src/frontmatter.rs:Template` names both:
`core/assets/template.typ` is `article`, the default, and `core/assets/press-release.typ` is
`press-release`. Each owns all styling — page, fonts, heading style, title block, column
count — and the emitter passes the frontmatter through and adds no styling of its own, so a
third look is a third `.typ` file and one enum variant.

Every look exports `template` and `divider`, and its `template` takes `title`, `author`,
`columns` and `date` before the trailing `doc`. That is the contract, because
`core/src/emit.rs:header` names all four on every call; a look missing one would fail the
compile with an error naming neither the document nor the key. No golden file pins it, so a
test in `core/tests/golden_test.rs` reads each look's source and asserts it.

The article's title block uses `place(scope: "parent", float: true)`, which lifts it out of
the column grid so it spans the page; Typst supports that scope only together with `float`.
The date sets beneath the author. The press release sets its dateline above a flush-left
title, over a `divider` rule, and runs in one column. Either look omits its title block when
`title`, `author` and `date` are all absent — the date joins that test, because a key the
author wrote that reached no page would vanish.

Both looks carry `show table.cell.where(y: 0): strong`, which sets a table's header row in
strong type. A GFM table
has exactly one header row and it is always the first, so row zero is the header by
construction. Typst's own default sets it in body type, which would flatten a distinction
the markdown source draws.

A footnote gets no rule, for the reason lists and quotes get none: the Typst default
already sets one apart, with a superscript marker, a separator, and the note at the foot
of the column that holds the reference — the column, not the page, because the composer
keeps its footnote insertions per column.

## The world

`core/src/lib.rs:TypstWorld` holds `main.typ`, every bundled template, the math prelude,
and the images the document names, all under `VirtualRoot::Project`. The prelude is bound
beside the looks rather than as one of them: a `Template` variant would make it selectable
from the `template` frontmatter key. It implements no package resolution, so no
import can reach the network on any target. It binds every look rather than the selected one,
so the walk never has to plumb its choice out: the looks are compile-time constants either
way, and `core/src/lib.rs:template_source` maps each `Template` variant to its `include_str!`,
so the same enum drives the filename and the bytes.

An asset rides that same virtual filesystem rather than a channel of its own, because
`World::file` already serves the templates from it. `main.typ` sits at the virtual root,
so a relative path in the generated source resolves to the file id built from that same
path, and `collect` keys the map by it. `World::source` is untouched by the assets — an
image is never Typst source — which is what keeps the import story exactly as it was.

Fonts are embedded with `include_bytes!` from `core/assets/fonts/`: five faces from one
Libertinus release, so their metrics agree, under the OFL. Serif Regular, Bold, Italic and
BoldItalic carry body text; Libertinus Mono carries `#raw`, which every look names in a
`show raw` rule. NewCMMath-Regular carries math and is the sixth, under the GUST Font
License beside them — a math font is a different kind of file, carrying an OpenType MATH
table, and without one Typst has no glyph for a variable or a Greek letter and a formula
sets as a row of boxes. It registers under the family Typst's own default math asks for,
so no look names a math family. Every face the dialect can
reach is bundled, because Typst renders the closest match it finds and synthesises none —
without the italic, `#emph` would come out as body text. No target discovers fonts from
the OS, so the same markdown compiles to the same PDF on every machine.

`World::today` returns `None`. The spec's §2 lists the current date among what the world
supplies, but an OS clock would break the same section's no-OS-access rule and make the
PDF differ between machines — silently, because this call touches the compile alone and never
the emitted source, so the golden files would stay byte-stable over a PDF that differed by
machine. Every look does typeset a date, and takes it from the frontmatter's `date` key: the
author writes the dateline.

## The CLI

`md2pdf input.md [-o output.pdf] [--emit-typst]`. Without `-o` the PDF lands at the input
path with a `.pdf` extension. `--emit-typst` prints the Typst source and ignores `-o`; that
output imports the selected look, which exists only inside the world, so it serves inspection
and not a standalone `typst compile`.

`cli/src/main.rs:read_assets` fills the shopping list: each path joins the parent
directory of the input file, so a figure is found beside the document and not beside the
current directory, and a repeated path is read once. The asset keeps the path the
markdown wrote, never the resolved one, because that is the name the generated source
asks for. A file that will not read is exit 1 naming the resolved path, the line, and the
message the OS gave. `--emit-typst` returns before that call, so emission reads no image
and works on a document whose figures are absent.
