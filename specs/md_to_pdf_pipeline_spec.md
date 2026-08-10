---
id: mpdf-001
title: md-to-pdf-pipeline
note: >
  The core .md → .pdf pipeline: pulldown-cmark parses, a hand-written emitter maps
  events to Typst markup, and embedded Typst compiles the PDF, behind a CLI.
status: accepted
last_updated: 2026-08-09

phases:
  - name: "Phase 1 — end-to-end pipeline behind a CLI"
    reviewed: 2026-08-08
    shipped: 2026-08-08
    cut: null
    by: null
  - name: "Phase 2 — frontmatter and column layout"
    reviewed: 2026-08-08
    shipped: 2026-08-08
    cut: null
    by: null
  - name: "Phase 3 — inline constructs"
    reviewed: 2026-08-08
    shipped: 2026-08-08
    cut: null
    by: null
  - name: "Phase 4 — block constructs"
    reviewed: 2026-08-08
    shipped: 2026-08-08
    cut: null
    by: null
  - name: "Phase 5 — links"
    reviewed: 2026-08-08
    shipped: 2026-08-08
    cut: null
    by: null
  - name: "Phase 6 — tables"
    reviewed: 2026-08-09
    shipped: 2026-08-09
    cut: null
    by: null
  - name: "Phase 7 — footnotes"
    reviewed: 2026-08-09
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-002]
reference: >
  Pandoc's md → pdf pipeline is the inspiration. Embedding Pandoc is out of scope:
  it is heavy to embed, it has no lightweight WASM build, and its GPL license
  raises linking questions.
---

# md → pdf pipeline

## 1. Goal

Convert one markdown file into one typeset PDF, fully on the local machine. **The
observable this project produces is the typeset PDF that Typst compiles from the
user's markdown — "Single file in, single PDF out."** The consumer is an author who
writes markdown and wants an article-look PDF without a server, a SaaS, or a LaTeX
toolchain.

The end state for this spec is a CLI:

```console
$ md2pdf paper.md -o paper.pdf
$ md2pdf paper.md --emit-typst        # print the generated Typst source; do not compile
```

over an input like:

```markdown
---
title: A Minimal Example
author: Iva Po
columns: 1        # optional; the default is 2
---

# Introduction

Body text in an article look.
```

The engine is UI-independent. A Tauri desktop app later becomes a second thin
wrapper around the same core crate. It is not a rewrite.

### 1.1 Non-goals

- **No servers, no VPS, no SaaS. Ever.** All work happens on the local machine.
- **No Pandoc.** See `reference` above. This holds permanently, not only for this spec.
- **Not full Pandoc-markdown and not raw Typst syntax.** The input dialect is a
  minimal CommonMark subset that this spec defines.
- Out of scope for this spec, parked for later specs: LaTeX math via `mitex`,
  citations and bibliography, multi-file manifests, a raw-Typst escape hatch,
  git-based collaboration, an agentic authoring flow, the Tauri UI, a WASM browser
  build, and desktop packaging.

## 2. Design

A Cargo workspace with two crates:

- **`core`** — a library crate. It parses markdown, emits Typst markup, and calls
  the Typst compiler in-process.
- **`cli`** — a thin binary. It wraps `core` behind `md2pdf input.md -o output.pdf`.

The pipeline inside `core` is three steps: `pulldown-cmark` parses the markdown
into an event stream; a hand-written emitter walks that stream and maps each event
to Typst markup; the embedded Typst compiler turns the Typst source into a PDF.

Typst is built around a `World` trait. The embedding crate supplies file bytes,
fonts, and the current time; the compiler itself never touches the filesystem or
the network. `core` therefore keeps all OS access out: its API takes a markdown
string and returns Typst source or PDF bytes, and the `cli` crate does the file
I/O. That split is what lets the same `core` compile natively for Tauri and to
`wasm32` for a possible browser build — one codebase, two targets, no rewrite.

`core` depends on two Typst crates: `typst` for the compiler and `typst-pdf`
for the PDF export, versions pinned at implementation. Its `World`
implementation supplies the Typst standard library, the font book, the main
source, the bytes of `template.typ`, the bundled fonts, and the current date.

### Why Typst, not LaTeX or Pandoc (decision, recorded)

Typst is fast, and it embeds as a Rust crate — native for a Tauri build, WASM for a
possible browser build. It has native math, citations, cross-references, and
numbering, so later specs do not have to build those. Pandoc is excluded for weight
and for its GPL license; LaTeX is excluded because it cannot embed.

### Why our own dialect via pulldown-cmark (decision, recorded)

Real CommonMark parsing already exists in `pulldown-cmark`. This project writes
only the emitter and a small number of preprocessing passes. That is a scoped,
tractable project, not a Pandoc reimplementation.

### Why a CLI before the Tauri UI (decision, recorded)

A CLI is faster to iterate on — there is no webview reload cycle — and easier to
test, because golden-file tests run against a function call. The `--emit-typst`
flag exists from the start so the emitter output can be inspected directly while
layout and frontmatter bugs are shaken out.

### Why a separate `template.typ` owns all styling (decision, recorded)

A Typst template is a function. The generated document applies it once, with
`#show: template.with(title: ..., ...)`, and everything below that line becomes
the function's document argument. `template.typ` owns all styling and layout: the
page setup, the title block, the column toggle, and the heading style. The
emitter only ever produces two things: the template call with the frontmatter
mapped to arguments, and the translated body content. A new look later — an
IEEE-style template, for example — is a new `.typ` file and a frontmatter
selector, with no change to the parser or the emitter. We write our own minimal
template rather than import a public one: public templates are fixed-format for
one venue, and we need our own column toggle driven by our own frontmatter
schema. The template reaches the compiler as bytes in the `World`'s virtual
filesystem — a local file, not a package import.

### Why the running app never fetches from the network (decision, recorded)

An `#import "@preview/pkg:x.y.z"` line triggers Typst's package resolution: a
local-cache check first, then a network fetch on a cache miss. The fetch, when
it happens, is performed by the embedder's package-resolution glue inside its
`World` — never by the compiler itself. This spec's scope uses no such import,
so no network is involved on any target, and our minimal `World` implements no
package resolution at all. When a later spec depends on a Typst Universe
package — `mitex` included — we vendor it as a bundled asset, fetched once at
build time. The running app stays fully offline and behaves identically on the
native and WASM targets.

### Why fonts are bundled, not discovered (decision, recorded)

The `World` receives fonts as in-memory bytes on every target, and a browser
sandbox has no OS font access at all. So `core` embeds its fonts at compile
time: the files live in `core/assets/fonts/`, and the default family is
Libertinus Serif (OFL license). `template.typ` names that family explicitly. No
target discovers fonts from the OS. The PDF therefore compiles identically on
every machine, which is what makes the Phase 1 exit gate reproducible.

### Why the emitter escapes and rejects, never guesses (decision, recorded)

Two rules keep the dialect honest. First, the emitter escapes every character
that Typst markup mode interprets in a text run — including `#`, `$`, `*`,
`_`, `` ` ``, `@`, `<`, `>`, `[`, `]`, `\`, and a line-leading `=`, `-`, or
`+` — so body text reaches the PDF verbatim: `$5` stays five dollars and never
opens math mode. Second, a construct outside the dialect — a list, emphasis, a
code block, a link — is an error: the CLI exits non-zero and names the first
unsupported construct and its line. Silently dropping or flattening content
would ship a PDF that lies about its source; a named error is the honest
failure, and support arrives construct by construct in later phases or specs.

## 3. Open questions

- **OQ-1** — ~~Which Typst crates does `core` need (compiler, PDF export), and
  what does a minimal `World` implementation look like in code?~~ **RESOLVED
  (2026-08-08):** `core` uses the `typst` crate for the compiler and `typst-pdf`
  for the PDF export. The minimal `World` supplies the standard library, the
  font book, the main source, the bytes of `template.typ`, the bundled fonts,
  and the current date. Landed in §2.
- **OQ-2** — ~~Where do fonts come from for the native CLI — bundled with the
  tool, or discovered on the system?~~ **RESOLVED (2026-08-08):** fonts are
  bundled and embedded at compile time on every target; the default family is
  Libertinus Serif; no OS font discovery anywhere. Landed in §2, "Why fonts are
  bundled, not discovered".
- **OQ-3** — ~~What is the exact frontmatter schema — the key names for `title`,
  `author`, and the column toggle, and the behavior on unknown keys?~~
  **RESOLVED (2026-08-08):** the schema is `title` (string, optional), `author`
  (string, optional), `columns` (`1` or `2`, default `2`). Absent frontmatter is
  valid, and every default applies. An unknown key, or an invalid `columns`
  value, is an error that names the key. Landed in Phase 2's scope; the §1
  example is settled, not provisional.
- **OQ-4** — ~~How does code content reach Typst verbatim — always the
  `#raw(...)` function form, or backtick fences with delimiter counting when
  the content itself contains backticks? One mechanism should serve both
  inline code and code blocks. Design call; blocks Phase 3's hostile gate
  case and Phase 4's fenced-block case.~~ **RESOLVED (2026-08-08):** the
  function form, always — `#raw("…")` inline, `#raw(block: true, lang: …)`
  for Phase 4's blocks. The content travels as a Typst string literal
  through `typst_string`; Phase 4 extends that escape with a newline as
  `\n`, which inline code never needs because CommonMark folds its line
  endings to spaces. No delimiter counting anywhere: one mechanism,
  deterministic for any content, reusing the escape that already exists.
  Landed in Phase 3's and Phase 4's scope.
- **OQ-5** — ~~How do tight and loose markdown lists map to Typst list
  spacing, and does the template own that look through `show` rules on
  `list` and `enum`? Design call; blocks Phase 4.~~ **RESOLVED
  (2026-08-08):** structurally, through Typst's own markup, which draws
  the same distinction markdown does: items that directly follow each
  other make a tight list, spaced by paragraph leading, and items
  separated by a blank line make a loose one, spaced by paragraph
  spacing. The emitter writes adjacent items for a tight markdown list
  and blank-line-separated items for a loose one — pulldown-cmark
  signals looseness by wrapping item content in paragraph events — and
  owns nothing about the look. Typst derives `tight` from exactly that
  adjacency and does not let a `set` rule override it, so tightness is
  not a template matter; every other list property (marker, indent, the
  spacing distances) remains one, and Phase 4 adds no such rule because
  the defaults already render the two forms distinguishably, which the
  gate pins. Landed in Phase 4's scope.
- **OQ-6** — ~~Does the header row of a table get a look of its own, and
  does `template.typ` own it through a `show` rule? Typst's default table
  sets the header row in the same body type as every other row:
  `table.header` carries the distinction semantically — it repeats the
  row across page breaks and tags it for assistive technology — but
  draws nothing differently. Markdown's pipe-table syntax draws the
  header distinction in the source, so a header row that renders
  indistinguishably from the body arguably flattens it — the worry OQ-5
  resolved for tight and loose lists, except there the Typst defaults
  already rendered the two forms distinguishably, and here they do not.
  Design call; blocks Phase 6's gate case (2).~~ **RESOLVED
  (2026-08-08):** yes, and the template owns it. `template.typ` gains
  `show table.cell.where(y: 0): strong`, so the header row is set in
  strong type; a GFM table has exactly one header row and it is always
  the first, so row 0 is the header by construction, and the bundled
  Bold and BoldItalic faces carry the result. The emitter owns nothing
  about the look, per §2's styling decision. Where OQ-5 found the
  defaults already rendering markdown's distinction, here they do not,
  so a rule is the honest floor — without it the PDF flattens a
  distinction the source draws. Because golden files pin emitter output
  only, gate case (2) pins the rule in `template.typ` itself. Landed in
  Phase 6's scope.
- **OQ-7** — ~~where does Typst place a footnote whose reference sits inside a
  table cell, and where does it place one in the two-column layout — the
  bottom of the column that holds the reference, or the bottom of the page?
  The two-column answer decides whether `template.typ` needs a rule of its
  own, which OQ-5 and OQ-6 are the two precedents for. Answerable from code
  (`typst-layout` 0.15.1's flush and footnote-placement path) during review.
  Blocks Phase 7's gate case (1) in its look claim only; a misplaced footnote
  would still compile.~~ **RESOLVED (2026-08-09), in review round 1:** the
  bottom of the column that holds the reference. `typst-layout` 0.15.1's
  composer keeps its footnote insertions per column, an entry too tall for
  its column spills into the next, and a reference inside a table cell is
  found by the composer's recursive frame search and placed by the same
  rule. That is the standard two-column article convention, so the phase's
  no-template-rule claim stands on OQ-5's precedent, and gate case (1)'s
  by-eye read confirms the answer rather than supplying it.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Each states the observable it
produces and carries a checkable exit gate.

### Phase 1 — end-to-end pipeline behind a CLI
*Produces the observable: yes — a PDF compiled from one markdown file.*

- **Scope:** Create the Cargo workspace with the `core` and `cli` crates. In
  `core` (`core/src/lib.rs`, `core/src/emit.rs`): parse markdown with
  `pulldown-cmark`; walk the event stream and emit Typst markup for section
  headings and paragraph text only. Markdown heading levels 1–6 map to Typst
  headings of the same level. Escape text runs and reject out-of-dialect
  constructs, per the dialect decision in §2. Strip and ignore a leading
  frontmatter block, with a warning on stderr — Phase 2 parses it. Compile the
  Typst source to a PDF in-process via the `typst` and `typst-pdf` crates, with
  the bundled fonts (§2). Ship a minimal `template.typ`
  (`core/assets/template.typ`) that owns the page setup and the heading style;
  supply it to the compiler as bytes in the `World`'s virtual filesystem. The
  emitter output applies it with `#show: template.with()` and then adds the
  translated body, per the styling decision in §2. Expose two functions:
  `md_to_typst(md: &str) -> Result<String>` and
  `md_to_pdf(md: &str) -> Result<Vec<u8>>`. In `cli` (`cli/src/main.rs`):
  `md2pdf input.md [-o output.pdf]` — `-o` defaults to the input path with its
  extension replaced by `.pdf`; errors go to stderr with exit code 1. The
  `--emit-typst` flag prints the generated Typst source instead of compiling.
  That output targets the `World`'s virtual filesystem; it serves inspection,
  not a standalone external `typst compile`.
- **Exit gate:** Golden-file tests, three cases. (1) A fixture with headings
  and paragraphs produces Typst source that matches a checked-in golden file;
  the CLI run on it writes a PDF that starts with the `%PDF` magic bytes and is
  non-empty; the `--emit-typst` output equals the golden file. (2) A hostile
  fixture whose body text contains `#`, `$`, `*`, `_`, a backtick, `@`, `<`,
  `>`, `[`, `]`, `\`, and a line-leading `=` compiles the same way, and its
  golden file shows every one of them escaped. (3) A fixture with a bullet list
  — an unsupported construct — makes the CLI exit non-zero with a message that
  names the construct and its line.
- **Close-out:** Seed `rules/pipeline.md` with all five frontmatter keys the
  methodology's §8.1 requires, with `sources: [core/src/lib.rs,
  core/src/emit.rs, core/assets/template.typ, cli/src/main.rs]`. One push.

### Phase 2 — frontmatter and column layout
*Produces the observable: yes — a PDF whose layout the frontmatter controls.*

- **Scope:** Parse the YAML frontmatter block in `core/src/frontmatter.rs`,
  with the schema OQ-3 resolved: `title` (string, optional), `author` (string,
  optional), `columns` (`1` or `2`, default `2`). Absent frontmatter is valid,
  and every default applies. When `title` and `author` are absent, the template
  omits the title block. An unknown key, or a `columns` value other than `1` or
  `2`, is an error that names the key. The emitter maps the frontmatter to
  arguments of the template call — `#show: template.with(title: ...,
  author: ..., columns: ...)` — and gains no styling logic of its own. Extend
  `template.typ` to own the title block and the column toggle: a **two-column
  layout by default**, single-column when `columns: 1`. This phase removes
  Phase 1's strip-and-warn behavior.
- **Exit gate:** Golden-file tests, four cases. (1) A fixture whose frontmatter
  has `title` and `author` but no `columns` key produces Typst source that
  matches its golden file — the two-column default — and compiles to a PDF that
  starts with the `%PDF` magic bytes. (2) A fixture with `columns: 1` does the
  same for the single-column layout. (3) The generated Typst source contains
  the title and the author. (4) A fixture with an unknown frontmatter key makes
  the CLI exit non-zero with a message that names the key.
- **Close-out:** Add `core/src/frontmatter.rs` to `rules/pipeline.md`'s
  `sources`, then update the rule against them. One push.

### Phase 3 — inline constructs
*Produces the observable: yes — a PDF from prose that Phase 2 rejects.*

The aim of Phases 3–5 together: **an ordinary markdown article converts
unmodified.** The dialect is the bottleneck — real prose contains emphasis
and inline code, and today one pair of backticks is a fatal error.

- **Scope:** In `core/src/emit.rs`: map five constructs that today reach the
  reject arm. Emphasis wraps its translated content in `#emph[…]` and strong
  emphasis in `#strong[…]` — the function forms, not Typst's `_…_`/`*…*`
  markup, because Typst's delimiters are word-boundary sensitive and
  CommonMark permits intraword emphasis: through `_…_`, `foo*bar*baz` would
  render literal underscores and `*foo*bar` would fail to compile with an
  unnamed error, each breaking the §2 faithfulness decision in its own way.
  Inline code becomes `#raw("…")` per OQ-4's resolution, its content a Typst
  string literal through `typst_string` — never the markup escape — so it
  reaches the PDF verbatim. A hard line break becomes Typst's `\` line
  break, a `\` followed by a newline — a `\` followed directly by text is an
  escape sequence instead. A thematic break becomes a call to `divider`, a
  column-width horizontal rule, which `template.typ` exports beside
  `template`; the emitter names it and owns nothing about its look, per the
  styling decision in §2. The header's import line becomes
  `#import "template.typ": template, divider` on every document, so all four
  checked-in golden files change on that line. The escape rule for plain
  text runs is unchanged: the emitter writes the calls itself, and body text
  inside and around them stays escaped.
- **Exit gate:** Golden-file tests, three cases, plus the full existing
  golden suite, which the import-line change touches. (1) A fixture with
  emphasis, strong emphasis, inline code, a hard break, and a thematic break
  produces Typst source matching its golden file and compiles to a PDF with
  the `%PDF` magic bytes. (2) A hostile fixture whose inline code contains a
  backtick, a `#`, a `$`, and a `\` shows in its golden file the `#raw`
  string literal with only the `\` string-escaped — the markup escape never
  applied. (3) A bullet list still exits non-zero naming the construct —
  rejection survives the widening.
- **Close-out:** Update `rules/pipeline.md`'s dialect section and the
  README's "What the markdown may contain" against the code. One push.

### Phase 4 — block constructs
*Produces the observable: yes — a PDF from documents with lists, code
blocks, and quotes.*

- **Scope:** In `core/src/emit.rs`: bullet lists become `- ` items and
  ordered lists `N. ` items, keeping a non-`1` start number and nesting by
  indentation; a tight list's items sit on adjacent lines and a loose
  list's items are separated by a blank line, per OQ-5's resolution.
  Fenced and indented code blocks become block-level raw content, encoded
  per OQ-4's mechanism, with one trailing newline stripped from the
  block's content when present: pulldown-cmark reports the final line's
  terminator as part of the content, and a string literal that kept it
  would typeset a phantom empty line after every code block. A fence's
  language tag — the first word of its info string — is carried through
  for Typst's highlighting; an indented block, or an empty info string,
  yields no `lang` argument. Block quotes become Typst's `quote`, block
  form; as with lists under OQ-5, Phase 4 adds no quote rule to the
  template — the default look stands, and any future styling is a
  `template.typ` rule per §2's styling decision. List items, not lists
  alone, stop being errors. The Phase 1 fixture
  `tests/fixtures/unsupported_list.md` is deleted, and every test keyed
  to a list rejection — the two on that fixture, and the inline list in
  `line_numbers_survive_a_frontmatter_block` — moves to a construct this
  spec still excludes: a table. The parser gains `Options::ENABLE_TABLES`
  for exactly that rejection: without it, pulldown-cmark reads a pipe
  table as paragraph text, so the pipes would reach the PDF as prose and
  the reject arm would never see the construct it is meant to name.
- **Exit gate:** Golden-file tests, three cases, plus the full existing
  suite, which the option change and the test migration touch — no
  shipped golden file changes, because no existing fixture contains a
  pipe table. (1) A fixture with a nested bullet list, an ordered list
  starting at 3, a loose item holding two paragraphs, a fenced code block
  with a language tag, an indented code block, and a block quote matches
  its golden file and compiles to a PDF with the `%PDF` magic bytes.
  (2) A tight list
  and a loose list of the same items produce Typst source that differs
  exactly in the blank lines between the loose items — Typst derives
  `tight` from that adjacency, per OQ-5 — pinned by the golden file, and
  both compile. (3) A fixture with a table makes the CLI exit non-zero
  naming the construct and its line.
- **Close-out:** Update `rules/pipeline.md`'s dialect section and the README
  against the code. One push.

### Phase 5 — links
*Produces the observable: yes — a PDF whose links resolve.*

- **Scope:** In `core/src/emit.rs`: inline links, reference links (already
  resolved by `pulldown-cmark` — the full, collapsed and shortcut forms
  all arrive as the same `Tag::Link`, so there is no `link_type`
  distinction to draw), and autolinks become `#link("url")[…]`, the URL
  through the string-literal escape that `typst_string` already
  implements, the link text translated as normal inline content. An email
  autolink's destination arrives as the bare address, so the emitter
  prepends `mailto:`. Two link shapes are errors that name the construct
  and its line, per the §2 escape-and-reject decision: an empty
  destination — `[text]()` is legal CommonMark, and `#link("")` fails
  Typst's compile with an error naming neither construct nor line, the
  first input-dependent break of the guarantee that generated source
  always compiles — and a non-empty link title, which neither Typst's
  `link` nor the PDF can carry, and which dropping silently would
  flatten. Images stay errors: they need file access, and the `World`
  holds exactly two files by design — a later spec's subject, not a
  construct.
- **Exit gate:** Golden-file tests, three cases; no shipped golden file
  changes, because `link` is a standard-library name and the import line
  is untouched. (1) A fixture with an inline link, a reference link, an
  autolink, and an email autolink matches its golden file — every URL in
  a string literal, the `mailto:` prefix present, the link text escaped —
  and compiles to a PDF with the `%PDF` magic bytes. At least one URL is
  hostile, carrying a `#` and a `"`, so the golden shows the string
  escape doing the work the markup escape must not. (2) An image makes
  the CLI exit non-zero naming the construct and its line. (3) A link
  with an empty destination, and a link with a title, are each an error
  naming the construct and its line.
- **Close-out:** Update `rules/pipeline.md`'s dialect section, the README
  and `samples/article.md` against the code; the sample gains a real link
  and an email autolink, which is what keeps the corpus check from
  passing vacuously — the README holds no link construct outside code
  fences, so on its own it would demonstrate nothing about this phase. The
  corpus check closes the ladder: the repository's own README and the
  sample both convert without error, or the gap is named in the review
  record. One push.

### Phase 6 — tables
*Produces the observable: yes — a PDF from documents with pipe tables.*

Appended 2026-08-08, after Phase 5 shipped, per the methodology's §6.1: the
dialect is this spec's subject, and a construct migrating from the reject
set to the supported set is the same widening Phases 3–5 performed.

- **Scope:** In `core/src/emit.rs`: GFM pipe tables become Typst's `table`.
  `Tag::Table(alignments)` opens a frame beside `ListFrame`; the column
  count is the alignment vector's length, and every row arrives with
  exactly that many cells — pulldown-cmark pads a short row with empty
  cells and drops excess ones, following GFM, so the emitter never counts
  cells itself. The call is `#table(columns: N, align: (…),
  table.header([…], …), […], …)`: an integer `columns` yields N auto-sized
  columns; the header row travels as `table.header`, which is what repeats
  it across page breaks and carries the accessibility tagging; each cell
  is a content block whose content is translated as normal inline content
  — a GFM cell holds inline content only, so emphasis, code and links
  inside cells arrive through the arms that already exist, each cell opens
  a buffer on the existing stack, and the markup escape on cell text is
  what keeps a `]` in a cell from closing its block. Alignments map
  `None → auto`, `Left → left`, `Center → center`, `Right → right`; when
  every column is `None`, the `align` argument is omitted. Per OQ-6's
  resolution, `template.typ` gains `show table.cell.where(y: 0): strong`,
  the header row in strong type — the emitter owns nothing about the
  look, per §2's styling decision. The rejection moves: `describe` drops
  its table arms, and the four tests keyed to a table rejection resolve
  in two ways. The two on `tests/fixtures/unsupported_table.md` are
  deleted — Phase 5's image tests already assert the same shape at both
  levels — and that fixture with them. The inline pipe tables in
  `line_numbers_survive_a_frontmatter_block` and
  `a_frontmatter_error_wins_over_a_later_construct_error` become images,
  which keeps the second testing frontmatter precedence over a construct
  error rather than passing vacuously once a table stops being one.
  `Options::ENABLE_TABLES` stays on, now serving support rather than
  rejection, and the comment above it — which says tables are outside
  the dialect — is rewritten to say so.
- **Exit gate:** Golden-file tests, three cases, plus the full existing
  suite, which the `describe` change and the test migration touch; no
  shipped golden file changes, because `table` and `table.header` are
  standard-library names and the import line is untouched. (1) A fixture
  whose table carries a default, a left, a center and a right column,
  emphasis, inline code and a link inside body cells — the inline code
  stays out of the header row, because `show raw` names Libertinus Mono,
  only its regular face is bundled, and Typst synthesizes no bold, so
  `strong` could not carry a code span there — an escaped pipe in a
  cell, and one body row a cell short matches its golden file — the
  short row padded with an empty cell — and compiles to a PDF with the
  `%PDF` magic bytes. (2) The golden shows the header row inside
  `table.header(…)`, and the header rule is pinned where it lives: a
  test reads `core/assets/template.typ` and asserts the `show` rule on
  row 0 is present — golden files pin emitter output only, so the
  template's side of OQ-6 needs its own artifact — and case (1)'s
  compile exercises the rule. (3) An image still makes the CLI exit
  non-zero naming the construct and its line — rejection survives the
  widening, through the migrated tests.
- **Close-out:** Update `rules/pipeline.md`'s dialect section, the README
  and `samples/article.md` against the code; the sample gains a real
  table, which is what keeps the corpus check from passing vacuously —
  neither corpus file holds a pipe table today. The corpus check repeats:
  the repository's own README and the sample both convert without error,
  or the gap is named in the review record. One push.

### Phase 7 — footnotes
*Produces the observable: yes — a PDF whose footnotes sit at the foot of the
page, from documents that today print their bracket syntax as prose.*

Appended 2026-08-09, after Phase 6 shipped and after `mpdf-002` shipped, per
the methodology's §6.1: the dialect is this spec's subject, and a construct
migrating from the reject set to the supported set is the same widening
Phases 3–6 performed. Footnotes are not citations; §1.1 parks citations and
a bibliography for a later spec, and this phase does not touch them.

The motivation is sharper than the earlier widenings, because a footnote is
not rejected today — it is flattened. `Options::ENABLE_FOOTNOTES` is not set,
so `[^1]` never reaches the reject arm and `core/src/emit.rs:describe`'s
footnote arms are unreachable. The parser reads the reference and the
definition as ordinary text, the escape rule escapes the brackets, and the
PDF prints `[^1]` and `[^1]: The source.` as prose. That is the §2
faithfulness failure the escape-and-reject decision exists to prevent, and it
is a shipped bug against a shipped decision rather than a missing feature.
Strikethrough and math flatten the same way and stay out of scope here; the
close-out names that gap.

- **Scope:** In `core/src/emit.rs`: set `Options::ENABLE_FOOTNOTES`, and map
  the two events it produces. The parser resolves a reference against its
  definition across the whole document before the walk begins — verified
  against pulldown-cmark 0.13.4 at drafting — with three consequences the
  design leans on: a definition may sit before or after the references to it;
  a reference with no definition produces no event at all and stays literal
  text, exactly as an unresolved reference link does; and the match runs
  under Unicode case folding — the parser keys its label map by `UniCase` —
  while the events carry each label's original spelling, so `[^A]` cites a
  definition written `[^a]:`. Every `Event::FootnoteReference` that arrives
  therefore has a definition somewhere in the document, no error shape is
  needed for a dangling reference, and everything below that touches a label
  — the map, citedness, the generated names — runs over the parser's own
  folded equivalence, through the `unicase` crate the parser itself uses,
  already in the tree as its dependency. Keying by the raw spelling would
  miss on valid input and misreport a cased pair as uncited.

  A definition that arrives after its reference is what the mechanism must
  answer, because Typst's `footnote` takes its content at the reference site.
  The emitter therefore walks the event stream **twice**. Pass 1 enters only
  the `Tag::FootnoteDefinition` regions, translates each one through the arms
  that already exist, and collects the set of folded labels the document
  references; it stores results and never raises — each region lands in the
  map as its content string or as the first error its translation produced.
  Pass 2 emits the document: the frontmatter parses where it always has, a
  body construct errors where it stands, and when the walk reaches a
  definition's region it surfaces that definition's stored error — or the
  uncited-definition error below — before skipping it. So every error, in
  either pass's territory, surfaces at its document position, the first one
  in document order is the one reported, and the shipped precedence — a
  frontmatter error wins over a later construct error — holds unchanged. A
  definition's content opens a buffer on the existing stack, as a list item
  and a table cell already do, so block content inside a definition — a
  second paragraph, a list, a code block — arrives through the arms that
  serve it everywhere else.

  The first reference to a label emits `#footnote[…]<fn-N>` and every later
  reference to that same label emits `#footnote(<fn-N>)`, which is Typst's
  own documented form for pointing at a footnote that already exists; `N`
  counts folded labels by first use. The user's own label text never reaches
  the output: a markdown label may hold any characters and a Typst label may
  not, and generating the name removes the escaping question rather than
  answering it. No collision is possible, because the dialect has no syntax
  for a Typst label and the markup escape covers `<` and `>` in body text.
  The emitter writes no numbers at all — Typst numbers footnotes in
  placement order, which is the order GFM numbers them in.

  Three shapes are errors that name the construct and the line, per §2's
  escape-and-reject decision. A **definition that no reference cites** would
  reach no page, and content that vanishes is the failure that decision
  exists to prevent; GFM drops it, and this dialect does not drop. The error
  names the definition's own line. A **second definition for a label already
  defined** — under the fold, so `[^A]:` repeats `[^a]:` — is refused the
  way the frontmatter refuses a repeated key: the parser resolves every
  reference to one body, the map would keep one and lose the rest, and a
  choice between bodies is a guess the dialect does not make. The error
  names the second definition's line. A **footnote reference inside a
  footnote definition** is refused for now: the probe confirms an inner
  definition arrives as a sibling at the top level rather than nested, so
  resolving one would mean a recursive substitution with a cycle check, for
  a construct real articles do not carry. Rejecting it keeps pass 1 free of
  recursion and makes a cycle unreachable.

  `describe` changes shape the way Phase 6 changed it: the two
  footnote-definition arms drop, because pass 1 and pass 2 handle the
  construct. The `FootnoteReference` arm stays, because one place still
  rejects it — an image's alt text, whose capture flattens to a plain
  string, and a footnote cannot render inside one.

  `image_paths`' list gains a definition's images at the point of that
  definition's first reference, not at the position the definition is
  written, so the list still runs in the order a reader meets the images.
  Every definition is cited, because an uncited one is an error, so no asset
  is ever demanded for content that will not render.

  `template.typ` gains no rule, on OQ-5's precedent rather than OQ-6's: the
  Typst default already sets a footnote apart, with a superscript marker, a
  separator, and the note at the foot. OQ-7's resolution confirms the claim
  for the two-column layout — the note lands at the foot of the column that
  holds its reference.

- **Exit gate:** Golden-file tests, three cases, plus the full existing
  suite, which the option change touches; no shipped golden file changes,
  because `footnote` is a standard-library name, the import line is
  untouched, and no checked-in fixture holds a `[^` run — verified at
  drafting. (1) A fixture carrying a definition that follows its reference, a
  definition that precedes one, a single label referenced twice — the second
  reference spelled in a different case, which pins the fold — and a
  definition holding emphasis, inline code, a second paragraph and a list
  matches its golden file — the first use `#footnote[…]<fn-1>`, the repeat
  `#footnote(<fn-1>)`, no user label text anywhere in the output, and no
  definition left in the body — and compiles to a PDF with the `%PDF` magic
  bytes. The PDF is read by eye once, confirming OQ-7's code-resolved answer
  on the page. (2) Each error shape names its construct and its line: a
  definition no reference cites, a second definition for a label already
  defined — its two spellings differing in case, pinning the fold on the
  error path too — and a footnote reference inside a definition. A
  frontmatter error still wins over a definition error later in the
  document, pinning the pass-2 error order. (3) A raw HTML block still exits
  non-zero naming the construct and its line at both levels — rejection
  survives the widening.
- **Close-out:** Update `rules/pipeline.md`'s dialect section, the README and
  `samples/article.md` against the code; the sample gains a real footnote,
  which is what keeps the corpus check from passing vacuously — no corpus
  file carries one today. Two claims are corrected rather than extended, in
  the rule and in the README both: "everything else is an error" is not true
  of strikethrough and math, which flatten to escaped text the way footnotes
  do today, and the rule's own list names them as errors. Naming that gap is
  this phase's obligation; closing it is not, and it wants a phase of its own
  or a plain bug fix, since the decision it implements is already shipped.
  Two ordering statements move with the `image_paths` change: the doc
  comment on `core/src/lib.rs:image_paths` says document order and becomes
  reader order — a definition's images at its first reference — and the
  rule's shopping-list line, which claims no order today, gains the same
  statement. The corpus check repeats: the
  repository's own README and the sample both convert without error, or the
  gap is named in the review record. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-001.md, append-only, one heading per round. See §7 of the
methodology.
-->
