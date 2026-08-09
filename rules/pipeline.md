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
max_lines: 95
generated: 2026-08-08
---

# Pipeline

Markdown in, PDF out, in three steps. `pulldown-cmark` parses; `core/src/emit.rs:emit`
walks the event stream and writes Typst markup; the embedded Typst compiler produces the
PDF. `core/src/lib.rs:md_to_typst` returns the source, `core/src/lib.rs:md_to_pdf` the
bytes. Neither touches the filesystem, the clock, or the network; `cli/src/main.rs` does
all file I/O and all terminal output.

## The dialect

Nine things are supported: headings at levels 1–6, paragraph text, soft breaks, emphasis,
strong emphasis, inline code, hard line breaks, thematic breaks, and a leading YAML
frontmatter block. Heading levels map to Typst headings of the same level.

The five inline constructs reach Typst as function calls, not as its own markup.
`#emph[…]` and `#strong[…]`, because Typst's `_…_` and `*…*` are word-boundary sensitive
while CommonMark permits intraword emphasis, so `foo*bar*baz` would either keep literal
underscores or fail to compile. Inline code is `#raw("…")`, its content a string literal
through `core/src/emit.rs:typst_string`. A hard break is a `\` before a newline; the same
`\` before text is an escape sequence instead. A thematic break calls
`core/assets/template.typ:divider`, whose look the emitter decides nothing about.

**Everything else is an error.** `core/src/emit.rs:describe` names the construct, and
`Error::UnsupportedConstruct` carries that name with the 1-based line. The CLI prints it to
stderr and exits 1. Nothing is ever dropped or flattened silently.

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
string literal rather than as markup: the title, the author, and every `#raw` content. A
literal interprets only `\` and `"`; the markup escape inside one would reach the PDF.

Quotation marks in body text are **not** escaped. `core/assets/template.typ` sets
`smartquote(enabled: false)` instead, which is why they still reach the PDF verbatim.

## The template

`core/assets/template.typ:template` owns all styling: page, fonts, heading style, title
block, and column count. The emitter passes the three frontmatter keys and adds no styling
of its own, so a new look is a new `.typ` file. Its own defaults never reach a document.

The title block uses `place(scope: "parent", float: true)`, which lifts it out of the
column grid so it spans the page; Typst supports that scope only together with `float`. A
document with neither `title` nor `author` gets no title block at all.

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
