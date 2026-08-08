---
title: pipeline
sources:
  - core/src/lib.rs
  - core/src/emit.rs
  - core/assets/template.typ
  - cli/src/main.rs
covers: >
  the markdown-to-PDF pipeline: the supported dialect, the escape rule, the
  rejection rule, the Typst world and its bundled fonts, and the CLI contract
max_lines: 60
generated: 2026-08-08
---

# Pipeline

Markdown in, PDF out, in three steps. `pulldown-cmark` parses; `core/src/emit.rs:emit`
walks the event stream and writes Typst markup; the embedded Typst compiler produces the
PDF. `core/src/lib.rs:md_to_typst` returns the source and `core/src/lib.rs:md_to_pdf`
returns the bytes. Both take a markdown string, and neither touches the filesystem, the
clock, or the network. `cli/src/main.rs` does all file I/O and all terminal output.

## The dialect

Four things are supported: headings at levels 1–6, paragraph text, soft breaks, and a
leading YAML frontmatter block, which is recognised and ignored. Markdown heading levels
map to Typst headings of the same level.

**Everything else is an error.** `core/src/emit.rs:describe` names the construct, and
`Error::UnsupportedConstruct` carries that name with the 1-based line. The CLI prints it to
stderr and exits 1. Nothing is ever dropped or flattened silently.

The emitter recognises the frontmatter block through
`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` rather than stripping it from the input. That
is what keeps every reported line number true to the user's file.

## The escape rule

`core/src/emit.rs:escape_into` backslash-escapes every occurrence of
`\ # $ * _ ` @ < > [ ] ~ - + = /`. The spec names all of these except `~`, a non-breaking
space in Typst, and `/`, which opens a `//` line comment.

One rule depends on position: a `.` is escaped when every character before it on the
output line is a digit, because `2. text` at a line start opens a Typst enumeration.
`core/src/emit.rs:line_is_all_digits` is that test.

Quotation marks are **not** escaped. `core/assets/template.typ` sets
`smartquote(enabled: false)` instead, which is why they still reach the PDF verbatim.

## The world

`core/src/lib.rs:TypstWorld` holds exactly two files, `main.typ` and `template.typ`, both
under `VirtualRoot::Project`. It implements no package resolution, so no import can reach
the network on any target.

Fonts are embedded with `include_bytes!` from `core/assets/fonts/`: Libertinus Serif
Regular and Bold, under the OFL. No target discovers fonts from the OS, so the same
markdown compiles to the same PDF on every machine.

`World::today` returns `None`. The spec's §2 lists the current date among what the world
supplies, but reading an OS clock would break the no-OS-access rule in the same section
and would make the PDF differ between machines. No template uses a date.

## The CLI

`md2pdf input.md [-o output.pdf] [--emit-typst]`. Without `-o` the PDF lands at the input
path with a `.pdf` extension. `--emit-typst` prints the Typst source and ignores `-o`; that
output names `template.typ`, which exists only inside the world, so it serves inspection
and not a standalone `typst compile`.

`cli/src/main.rs:starts_with_frontmatter` warns on stderr when the input opens with a
frontmatter block. The warning lives there, not in `core`, because `core` must reach
`wasm32`, where stderr has no destination.
