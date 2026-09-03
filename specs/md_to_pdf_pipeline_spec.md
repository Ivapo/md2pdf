---
id: mpdf-001
title: md-to-pdf-pipeline
note: >
  The core .md → .pdf pipeline: pulldown-cmark parses, a hand-written emitter maps
  events to Typst markup, and embedded Typst compiles the PDF, behind a CLI.
status: accepted
last_updated: 2026-09-02

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
    shipped: 2026-08-09
    cut: null
    by: null
  - name: "Phase 8 — the constructs the reject arm names but never sees"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 9 — the look the frontmatter chooses"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 10 — the title block gets its air back"
    reviewed: 2026-08-31
    shipped: 2026-08-31
    cut: null
    by: null
  - name: "Phase 11 — several authors, and the affiliation each belongs to"
    reviewed: 2026-08-31
    shipped: 2026-08-31
    cut: null
    by: null
  - name: "Phase 12 — a marker with nothing to point at"
    reviewed: 2026-08-31
    shipped: 2026-08-31
    cut: null
    by: null
  - name: "Phase 13 — the binary carries its own licences"
    reviewed: 2026-09-02
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

### The title block's spacing is too small, measured (decision, recorded)

**APPENDED 2026-08-31**, and **substantially corrected the same day by Phase 10's review
round 1, which falsified the mechanism a draft of this section asserted.** Both states are
written out, because the wrong one is the one a later reader will re-derive.

§2's styling decision above gives `template.typ` the title block, and both bundled looks
have written a spacing value into it since Phase 9. **The value is too small and the lines
overlap** — a defect rather than a taste question, which is said first because everything
else here follows from it.

**Measured 2026-08-31 through the shipped pipeline**, on `mpdf-005` Phases 7–9's own method:
a probe carrying `title`, `author` and `date`, compiled through the CLI and read with
`pdftotext -bbox`, which reports each line's own box rather than the grid `-layout` invents.
Boxes in points, article look:

| line | box top | box bottom | separation from the line above |
|---|---|---|---|
| title, 17pt | 66.63 | 86.01 | |
| author, 11pt | 83.24 | 95.78 | **−2.78** |
| date, 10pt | 94.71 | 106.11 | **−1.07** |

**The boxes overlap**, which is why the author rides up into the title's descenders. The
press-release masthead has the same defect by its own route — its title is a
`block(below: 0.5em)` and its author the block beneath — and measures **−2.03**.

**The cause is not what a draft of this section claimed, and the correction is the whole
reason this phase is a number per look rather than a rewrite.** That draft said the
`v(0.4em, weak: true)` was *discarded* at a paragraph boundary, and prescribed a structural
change — drop the `linebreak()`s, add `set par(spacing: 0pt)`, give each gap an explicit
non-weak `v()`. Round 1 falsified the safety net that claim implied, and re-measuring to
answer it falsified the claim itself. Under the **shipped** shape, varying only the value:

| `v(…, weak: true)` | 0pt | 2pt | 0.4em (= 4pt, shipped) |
|---|---|---|---|
| author box top | 79.24 | 81.24 | 83.24 |

**Weak spacing applies in full, linearly, slope exactly 1, with no threshold and no
collapse.** And the prescribed structure measures **identically at the same values** —
79.24 at `v(0pt)` and 81.24 at `v(2pt)` — so it buys nothing at all. Six points across the
two shapes fit one line, and the round's own independent readings (`0.8em` → +1.22,
`2em` → +16.00 of movement) sit on it.

**So the value is the bug, and the arithmetic says exactly how small it is.** `0.4em`
resolves against the block's **10pt body** and not against the 17pt title — the trap the
shipped value was already in, and `mpdf-005` Phase 9's coupling in a second place — so the
look asks for 4pt where the runs need **6.78pt** to clear zero between title and author and
**5.07pt** between author and date. Those two thresholds are the linear fit above read at
zero, and round 1 derived the second independently as "≈0.51em".

**What the rejected "obvious fix" actually measured, recorded because the draft read it
backwards.** Dropping the `linebreak()`s and making the spacing non-weak measured **+17.62**
and **+12.13**, and the draft took that as the fix overshooting its value. It is not
overshoot: without the `linebreak()`s each run becomes its own paragraph, so Typst's default
paragraph spacing lands *on top of* the value written. That is an argument for **keeping**
the `linebreak()`s, which is what the phase now does. **The sign is accounted for and the
exact size is not**: measured against the structural shape at the same 4pt value the
difference is **+20.4pt** where `par.spacing` defaults to 1.2em = 12pt, so something beyond
the default is in the sum. Recorded rather than chased, because the shape is rejected on the
equivalence measurement above and nothing turns on the remainder.

**Neither look needs a different *shape*, and the two need different *sites*** — measured
rather than assumed, and OQ-12's answer, resolved in review rather than deferred to build.

- **The article look's three runs share one `v` inside a `for` loop**, so one value cannot
  serve two joins that want different amounts. The fix splits the value by index and raises
  both: measured at `1.8em` and `0.9em` the separations become **+11.22** and **+3.93**.
- **The press release's masthead is three sibling blocks**, and the spacing goes on the
  **author block's `above:`** rather than the title block's `below:`. That is not a style
  preference: the title's `below:` also governs the gap between a *title-only* document's
  headline and its `divider`, which the gate forbids moving, and round 1 measured that route
  failing at `1.4em` for exactly that reason. Measured on the author block instead,
  `above: 1.0em` takes the **−2.03** to **+3.47** and leaves a title-only press release
  byte-identical.

**A wrapping headline's internal leading is not this defect and is not touched, and this
is true of both looks rather than only the obvious one.** The press-release look sets
`par(leading: 0.35em)` for a title that wraps, deliberately, and Phase 9 recorded why: two
lines of one headline measure **−2.90** apart. **The article look does the same thing
without a special rule** — a wrapped title measures **−1.92** between its own two lines,
under the shipped look and under the fix alike. Both are negative separations that are
correct, and together they are why the gate below is scoped to the joins *between* the three
keys rather than to every consecutive pair of boxes: the scoping is load-bearing in both
looks, not just where a leading was set by hand.

**The values are the look's**, on the seam every phase since Phase 2 has used, and there is
no frontmatter key: the author says what the title block says and the look says how far
apart it says it. The two bundled looks are free to disagree, and here they must, because
one is a centred float over two columns and the other a flush-left masthead over one.

**`mpdf-004` Phase 3's property is bent, and the bound is named rather than argued away.**
"No document's typeset output changes unless its author asks" cannot hold: every document
that carries a title block moves, and there is no key to condition the fix on — the same
bound `mpdf-005` Phase 6 recorded, reached from the same direction. What moves is a defect,
and an author who did not ask for the fix did not ask for the overlap either.

### Several authors, and the affiliation each of them belongs to (decision, recorded)

**APPENDED 2026-08-31.** `author` has been one free string since Phase 2 and OQ-3 fixed the
schema around it. A document with two authors has had nowhere to put the second.

**The constraint is the parser and it is a deliberate one.**
`core/src/frontmatter.rs:parse` refuses an indented line outright — *"nested keys are not
supported"* — so a YAML list is not available without reopening a decision the schema rests
on. A list is therefore a separator inside one line, and the whole design follows from that.

**`author` takes the list; there is no `authors` key.** `mpdf-005` Phase 8 wrote the rule
down while refusing `numbered` beside `6`: **this schema has no synonyms**. `author` and
`authors` would be exactly that, with the added trap that a document writing the wrong one
gets silence rather than an error. A one-name document is a one-element list, and its page
does not move.

**The separator is `;`.** A comma is what an author reaches for and it is refused:
`author: Po, Iva` is an ordinary way to write one person's name, and a comma-split turns
that person into two silently — the failure this project's §2 rule exists to prevent.
BibTeX's ` and ` was weighed and is the better-known convention; it fails the same way for
a rarer name, and it reads as prose where a list is meant, so a reader who has never seen
this dialect has to be told it is a list. A semicolon tells them.

**The marker rides the name, because nothing else in a flat schema carries the relation.**

```
author: Iva Po^1; Someone Else^2; A Third Person^1,2
affiliation: Anthropic, San Francisco; MIT, Cambridge
```

An affiliation is a *relation* between two lists, not a third string, and one line per key
leaves the marker as the only place to put it. `^1` is chosen over `(1)` because it says on
the page what it means in the source, and because a parenthesis inside a name is commoner
than a caret. `affiliation` is singular for the reason `author` is: one key, several
values, no synonyms.

**What crosses the seam is structure and not typography**, which is the whole of why this
does not widen into a paper template. `core` hands the look a list of names each with its
markers, and a list of affiliations; the look decides that a marker is a superscript
number, that affiliations set smaller and italic, one to a line, and that the authors run
on one line separated by commas. **Measured 2026-08-31**, in a probe with the two lists as
literal defaults on `core/assets/template.typ`: `super()` renders both the single and the
multiple marker — `A Third Person` reading `¹˒²` — with no package, no import, and nothing
added to `core/src/lib.rs:TypstWorld`.

**The call contract changes an argument's type, and that is the real cost of this phase.**
`author` has crossed as a string since Phase 2 and would cross as an array. `mpdf-005`
Phase 8 added a *seventh argument*, which is additive and costs a third look one parameter;
this changes one that exists, so a third look written against the shipped contract breaks.
The mitigation is honest rather than clever: **no third look exists**, the contract is
stated in exactly two places — `rules/pipeline.md` and `README.md` — and the alternative is
worse. That alternative is `core` joining the names back into one string, which puts
"comma or one per line" in the emitter and hands a look a rendered decision it is supposed
to make; that is the seam collapsing, and every phase since Phase 2 has held it.

**Three refusals, each naming the author's own line**, on `mpdf-004` §2's rule that the
error names what the author typed:

- **A marker naming an affiliation the document does not carry.** `^3` under two
  affiliations is a mistake with a silent failure mode — a superscript pointing at nothing —
  and Typst cannot catch it, because by then it is a number in a list.
- **An `affiliation` key with no marker anywhere.** The relation is unstated and no rendering
  of it is a guess worth making.
- **A marker that is not a number**, which is a typo in the one position where a typo would
  otherwise reach the page as text.

**Not refused, deliberately: an author with no marker in a document that has affiliations.**
A paper with three authors from one lab and a fourth from none is a real document, and
refusing it would break something nobody asked to have broken.

**CORRECTED 2026-08-31 by Phase 11's review round 1, which falsified three claims above.
Both states are written out, because the wrong one is what a later reader re-derives.**

**The refusals are four, not three; one of them is scoped rather than absolute; and the
line each names is not the one the sentence above promises.** "Three refusals, each naming
the author's **own** line" is wrong in that last respect as well: refusal 2's fault is in
`affiliation`, so that is the line it names. Phase 11's scope assigns all four.

- **The one-affiliation case relaxes refusal 2 rather than triggering it.** As written above,
  "an `affiliation` key with no marker anywhere" is refused — which makes the commonest real
  paper unwritable. One lab and three authors leaves the author two bad options: write `^1`
  on every name, which is the noise OQ-11 names, or write no marker and be refused. So
  **with exactly one affiliation the markers are optional**: a document that writes none is
  valid and every author belongs to that affiliation. Refusal 2 applies from **two**
  affiliations up, where the relation really is unstated. This is OQ-11's resolution and it
  is a *third* answer, neither of the two that question enumerated.
- **A fourth refusal: an empty name in the list.** `author: Iva Po;` splits to a second,
  empty element, which none of the three above covers and which reaches the look as a
  dangling separator. That is the silent flattening §2's escape-and-reject decision exists
  to forbid, so it is an error naming the line.

**The grammar is specified here rather than left to one clause**, because round 1 worked
real inputs through it and found four places two implementers would diverge:

- **Every element is trimmed**, on both lists and on each marker. `author: A^1; B^2` and
  `A^1;B^2` are the same document, and `^1, 2` and `^1,2` are the same markers — a trim is
  not a guess, and the spaced form is what an author actually types.
- **The name splits from its markers at the *first* `^`.** `A^B^1` is therefore refused by
  refusal 3 naming `B^1`, rather than guessed into a name `A^B`. A `^` inside a name is
  rarer than a typo in the marker, and §2 refuses rather than guesses.
- **A marker is a non-empty run of ASCII digits** after trimming, and indexes `affiliation`
  in written order, from 1.

**`author` crosses as `none` and never as `()` when the document wrote no author**, because
both looks guard the whole block on `if title != none or author != none or date != none` and
an empty array is not `none` — a keyless document would grow a title block. And **a
one-element Typst array needs its trailing comma**: `("Iva Po")` is a parenthesized string,
not an array, so `header` writes `("Iva Po",)`. Both are traps the emitter must be written
against, and neither was stated.

**"The contract is stated in exactly two places" is false**, and it is the mitigation's
load-bearing sentence. It is stated in four prose sites — `rules/pipeline.md` twice,
`README.md`, and both `core/assets/*.typ` header comments — and asserted mechanically by
`core/tests/golden_test.rs:every_bundled_template_meets_the_call_contract`. The mitigation's
substance survives, because **no third look exists** and that was always the real argument;
the count does not, and the undercount is what generated the close-out's missed sites. **The
honest count is five locations across four files** — round 2 caught this correction making
the same class of error it was written to fix.

**CORRECTED 2026-08-31 by Phase 12, after the shipped refusal was read by its first user as
saying an affiliation is always required.** Both states are written out, because the wrong
one is what a later reader re-derives.

**Refusal 1 above is stated absolutely and should not be.** "A marker naming an affiliation
the document does not carry" refuses `author: Iva Po^1` in a document with **no
`affiliation` key at all** — a state an author reaches by commenting the key out while
drafting, which is an ordinary move, and by writing the markers before the key. It stopped
the build over markers that state nothing.

**What the first user read it as is a fact about this bullet and not about the message**,
and round 1 caught a draft of this block blaming the wrong one. The shipped sentence had
already been reworded the same day to name both exits — *"add that key, or drop the
markers"* — and it still produced the reading "so an affiliation is always required",
because the bullet above is what the schema actually says and the bullet is absolute. A
message cannot talk an author out of a rule.

**The refusal now begins at one affiliation. With none, every marker is dropped and the
document compiles.** The case is not that dropping is harmless — it is which failure each
answer leaves *visible*.

- **The failure refusal 1 was written against is invisible**: `^3` among two real
  affiliations sets a superscript that points at nothing, in a byline where the other
  superscripts point at something, and no reader scans a byline for that. **That case is
  untouched from one affiliation up, `^0` with it.** At zero, `^0` is cleared like every
  other marker — round 1 caught a draft of this bullet claiming `^0` was untouched
  everywhere, which the scope below contradicts and the scope is right.
- **The failure a drop permits is self-evident**: an author who writes markers meaning to
  add the affiliations later, and forgets, gets a PDF with **no affiliation block anywhere**.
  That is not a defect a reader has to hunt for; it is the whole feature missing from the
  page. The cost is real and it is accepted, because it announces itself.

**This bends §2's escape-and-reject decision, and round 1 falsified the bound a draft of
this block named.** That draft said a marker "is not content: it is one end of a
**relation**, and at zero affiliations the relation has no other end." **The relation
argument proves too much, and this dialect already ships the counter-example.** A citation
key in a document with no `bibliography` is one end of a relation whose other end does not
exist, and `core/src/emit.rs:check_citations` **refuses** it rather than dropping it — for
the flattening reason. A separator that licenses dropping the marker licenses dropping the
citation, and nothing in this project wants that.

**The separator that actually holds is what the drop leaves on the page.** Dropping a
marker removes no glyph a reader could misread: `Iva Po^1` sets as `Iva Po`, which is a
name, correctly typeset, and nothing on the page is wrong. A citation is refused because its fallback puts
**wrong glyphs** in front of a reader — the escaped `\[\@smith2020\]` sitting in the
sentence — which is the thing the refusal prevents rather than something it causes. Round 2
caught a draft of this paragraph stating that mechanism backwards, as a hole in the sentence
and as brackets reaching the page. **§2's rule is about what a silent drop puts in front of a
reader, and here it puts nothing.** A later phase reaching for this
precedent has to answer that question and not the relation one.

**It is OQ-11's answer taken one step down in one respect and reversed in another, and both
halves are written out.** OQ-11 made the marker optional at exactly one affiliation, because
there the relation is redundant; at zero it does not exist, so a schema that lets an author
omit a marker where it says nothing and refuses one where it says nothing is answering the
same question two ways. **But OQ-11 also said a marker written anyway "is honoured and
reaches the page, because the author wrote it", and at zero this phase does not honour it** —
it cannot, there being nothing to point at. The continuity carries the optional half and
breaks the honouring half, and round 1 caught a draft claiming only the first.

**`core` clears the markers rather than the look ignoring them, and that is load-bearing.**
What crosses the seam is the truth about the document: a document with no affiliations has
authors with no markers, so `--emit-typst` shows `markers: ()` and neither look changes by a
character. A look asked to ignore markers when `affiliation` is `none` would be a second
place the rule lives, and the two would drift.

**Still four refusals in this document's telling, and now two of them are scoped rather than
one.** Refusal 2 begins at two affiliations, per OQ-11; refusal 1 begins at one. Refusals 3
and 4 — a marker that is not a number, and an empty element in either list — are
unconditional, because both are faults in what the author *typed* rather than in a relation
that may not exist. Both fire inside the line loop, before either scoped one can run.

**The shipped code carries a fifth that this document has never counted, and round 1 found
it.** `author: ^1` is an entry that is not empty and has no name, and
`core/src/frontmatter.rs` raises *"key 'author' has an entry with no name before its '^'"*
for it. That is a real refusal with a test of its own, landed by Phase 11 and covered by
neither its refusal 4 nor its count. It is recorded here rather than renumbered: the tally
has been wrong since Phase 11 shipped, this phase does not touch that arm, and a
renumbering would make every earlier sentence in this section read against a list it never
described.

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

- **OQ-8** — ~~does the dialect refuse both math forms, and what does refusing
  them cost prose that converts correctly today? `Options::ENABLE_MATH` is what
  makes math reach the walk at all, and §1.1 parks LaTeX math via `mitex` for a
  later spec, so this phase's answer is a named error rather than support. The
  cost is the open part. pulldown-cmark 0.13.4 opens a math span at a `$` whose
  next byte is not ASCII whitespace and closes one at a `$` whose previous byte
  is not — the flanking test in its `firstpass.rs` scanner — so `it costs $5 or
  $10` stays text while `the range $5–$10` becomes math, and under a reject arm
  that second document stops converting where today it converts and prints
  exactly what its author wrote. Three answers are open: refuse both forms and
  accept the cost; refuse the display form only, which no prose produces by
  accident, and leave the inline form flattened with the gap still named; or
  leave the option off, which keeps `describe` claiming a rejection it cannot
  perform. Answerable from code for the mechanism, a design call for the cost.
  Blocks Phase 8's math half and its gate case (2).~~ **RESOLVED
  (2026-08-10), in review round 1: refuse both forms.** Each alternative
  preserves a flattening lie — the display-only answer keeps `$x$` printing
  literally while `describe` claims math is refused, and leaving the option
  off keeps all three unreachable arms and forfeits the property the phase
  exists to establish. §2's escape-and-reject decision names the error as
  the honest failure, and support is parked for `mitex` by §1.1, so the
  named error is the whole answer. The cost is accepted because it is
  bounded and has a one-character exit, both probe-verified against
  pulldown-cmark 0.13.4: `\$` suppresses the span and the dollar reaches
  the page as itself, the flanking rule already keeps `it costs $5 or $10`
  as text, and the corpus census finds no file that trips the rule. A
  document refused here gets an error naming math and its line, and the
  README's close-out documents the `\$` escape beside it. Landed in Phase
  8's scope.

- **OQ-9** — ~~does the press-release look carry a dateline, and where does that
  date come from? `core/src/lib.rs:TypstWorld::today` returns `None`
  deliberately: reading an OS clock would give `core` the OS access it exists
  to avoid, and it would make one document compile to two different PDFs on
  two machines, which is what the golden-file gate rests on. Its own comment
  records that no template needed a date. A press release conventionally
  carries one. Three answers are open: a fifth frontmatter key, `date`, an
  optional string the author writes, which leaves the reproducibility decision
  whole and costs one more key; the look omits the dateline, which ships a
  press-release template that a press release cannot use; or `today` starts
  answering, which contradicts a recorded decision and makes every golden
  compile-dependent on the day it ran. Design call, with the mechanism
  answerable from code. Blocks Phase 9's second template and its gate case
  (2).~~ **RESOLVED (2026-08-10), in review round 1: a fifth key, `date` —
  an optional free string, typeset verbatim.** The author writes the
  dateline; no clock is read anywhere, `today` stays `None`, and the
  reproducibility decision stands whole. The round added the fact that
  sharpens the clock option's rejection: `today` touches only the compile,
  never the emitted source, so its break would ship silently — byte-stable
  goldens over a PDF that differs by machine. Omitting the dateline ships a
  press-release look a press release cannot use. The key rides the existing
  `Frontmatter` → `header` → template-argument path; every bundled template
  accepts `date` and renders it when present — the article look beneath its
  author line, the press-release look as its dateline — because a key the
  author wrote that reaches no page is the vanishing §2 refuses. `header`
  names it on every call like the other three, which changes the second line
  of every shipped golden; Phase 3's import-line sweep is the precedent, and
  gate case (1) pins this one. Landed in Phase 9's scope.

- **OQ-10** — ~~may a template carry its own default for `columns`, and what
  would that cost? A press release is single-column by convention, and under
  Phase 9 as scoped its author must write `columns: 1` or get two columns,
  because `core/src/emit.rs:header` names every argument on every call and the
  template's own default therefore never applies. Letting the template decide
  means omitting the argument when the frontmatter left it out, which changes
  the second line of all twelve checked-in golden files and drops the property
  that `--emit-typst` shows the layout a document actually gets. Design call,
  and the blast radius is already counted. Blocks Phase 9's gate case (1),
  whose "no shipped golden file changes" claim is false under one answer.~~
  **RESOLVED (2026-08-10), in review round 1: no — a template carries no
  default that ever applies; the schema's `columns` default becomes
  per-template instead.** The schema is the shipped default's home, per
  `core/src/frontmatter.rs:Frontmatter::default`'s own comment, and the
  schema it stays — never the template, though within the file the
  convention's site is the parse-time resolution rather than the struct:
  an absent `columns` resolves at parse time to the selected look's
  convention — `2` for `article`, `1` for `press-release` — so `header`
  still names the argument on every call and `--emit-typst` keeps showing
  the layout a document actually gets. The Typst-side alternative — omit the
  argument and let the template's own default apply — would change the call
  line of the shipped goldens *and* drop that property, buying nothing the
  parse-time answer does not give. The round corrected the census: thirteen
  shipped goldens, not twelve, `strikethrough.typ` having landed the same
  day this phase was appended; `single_column.md` alone names `columns`
  explicitly. A press-release author therefore writes nothing and gets a
  single column, which is the point of choosing a look. Landed in Phase 9's
  scope.

- **OQ-11** — ~~with exactly one affiliation, does the marker appear at all? Most
  published templates drop it: one affiliation over any number of authors needs no
  relation, and a lone `¹` on every name is noise that says nothing. Against that, a
  document that gains a second affiliation would then see every marker appear at once,
  which is a page moving for a reason its author did not write. Two places could answer
  it and they are not equivalent — the **look**, which would make it typography and let
  the two bundled looks disagree, or the **schema**, which would make it a fact about the
  document and refuse a marker the single-affiliation case does not need. Design call.
  Blocks nothing in Phase 10; blocks Phase 11's gate case for the one-affiliation
  document, which cannot state what it reads.~~ **RESOLVED (2026-08-31), in Phase 11's
  review round 1: the schema answers, and it answers by making the marker optional rather
  than by refusing it — a third answer, neither of the two enumerated above.** With
  exactly one affiliation a document may write no marker at all, and every author belongs
  to that affiliation; a marker written anyway is honoured and reaches the page, because
  the author wrote it. The question presupposed that the schema's only move was a refusal,
  and the round found that reading makes the commonest real paper — one lab, several
  authors — unwritable: §2's refusal 2 would reject the unmarked form, leaving only the
  lone `¹` this question calls noise. The **look** answer was rejected for a reason the
  question did not carry: a look suppressing a marker decides a *fact* about the document
  (this relation is redundant) rather than a typographic one, and the two bundled looks
  disagreeing would mean one document reading as two. **The question's own worry — that a
  second affiliation would make every marker appear at once — does not arise under these
  rules, and round 2 caught a draft of this resolution accepting a consequence its rules do
  not produce.** A document with one affiliation and no markers that gains a second is
  *refused* by refusal 2, not silently re-rendered: the build stops and names the line until
  the author writes the markers the new relation needs. The page cannot move behind the
  author's back, which is a stronger answer than the one the question was willing to settle
  for. Landed in §2's correction and in Phase 11's scope, which carries the
  one-affiliation gate case this question said it blocked.
- **OQ-12** — ~~do the two looks need the *same* structural fix for the spacing, or only
  the same property? The article look's title block is a centred run inside one `align`;
  the press release's is three sibling `block`s in a masthead, and the measured **−2.03**
  has a different cause. One fix may not serve both, and a phase that assumes it does would
  ship a look whose numbers landed by luck. Measurement, and Phase 10's scope carries it.~~
  **RESOLVED (2026-08-31), in Phase 10's review round 1: neither look needs a *structural*
  fix, and the two need different *sites*.** The question presupposed a structural change
  the round measured to be a no-op — weak spacing applies linearly with no collapse, so the
  shipped shape and the prescribed one are identical at equal values. What is left is a
  value per join, and the sites differ: the article's three runs share one `v` in a `for`
  loop, so the value splits by index; the press release's spacing goes on the **author
  block's `above:`**, because the title block's `below:` also governs a title-only
  document's gap to its `divider`, and moving it fails the phase's own byte-identity case.
  Landed in Phase 10's scope, with §2 carrying both measurements.
- **OQ-13** — does the affiliation list belong to the title block alone, or does a later
  spec want it anywhere else? A footer, a running header and a first-page footnote are all
  places published documents put one, and `mpdf-003`'s desktop app has a footer design
  parked. Nothing in Phase 11 turns on the answer — the data crosses the seam either way
  and a second site is a `show` rule over content the look already has — but it decides
  whether the key is named for the title block or for the document, and a key named for
  the wrong one is a rename later. Design call, and cheap to hold: the name chosen in
  Phase 11 is `affiliation`, which is neutral between the two.

- **OQ-14** — does `core` need a warning channel at all? Phase 12 drops a marker that points
  at nothing and says so nowhere, because the alternative on offer was an error and the
  author had already been stopped by one. A third answer exists — warn on stderr and
  continue — and it was weighed and not taken, for a reason that is about the crate rather
  than about markers: `core`'s API returns a `Result` and has no second channel, so a warning
  would be a new return shape on `md_to_typst` and `md_to_pdf`, threaded through `cli`, the
  desktop app and the WASM build, for one message. Phase 1 shipped a stderr warning for a
  stripped frontmatter block and Phase 2 removed it, so the precedent runs both ways.
  **Nothing in Phase 12 turns on the answer** — a warning is additive to a document that
  already compiles — but a second construct wanting one would make this a real question, and
  the first is the cheapest place to notice that. Design call, cheap to hold.
- **OQ-15** — what keeps `THIRD-PARTY-LICENSES.md` current? *(design call, raised by Phase
  13 and deliberately not answered by it)* The file is generated by
  `tools/third-party-licenses.py` from the resolve graph, and Phase 13 compiles it into the
  binary — so the binary cannot drift from the repository, and nothing yet stops the
  repository drifting from `Cargo.lock`. **Regenerating it needs the registry sources
  unpacked**, because the licence texts are read out of the crates themselves rather than
  copied into the tool, and a CI runner that has only fetched crates does not have them.
  Three answers are open. Split the check in two, and assert in CI only the half that is
  cheap — the crate *list* against `cargo metadata`, which needs no sources — leaving the
  texts to a deliberate local run. Run the generator in `test.yml` behind a `cargo fetch`
  and whatever unpacks the sources, paying the time on every push. Or accept the drift as
  bounded and say so: the generator walks the resolve rather than an enabled feature set,
  so it over-lists by eight crates today, and a stale file that over-lists is wrong in the
  direction that costs a reader nothing. **Nothing in Phase 13 turns on the answer** —
  every clause of its gate reads the committed file, whatever that file says — and the
  question becomes real the first time a dependency is added or removed. Named here so
  that phase meets it as a recorded question rather than a discovery.

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

  **CORRECTED 2026-08-18, by `mpdf-005` Phase 3: "the first one in document
  order is the one reported" now has exactly one exception, and the sentence
  above states it without qualification.** A cross-reference to a name the
  document does not declare is checked *after* the walk, because a reference may
  precede its declaration and a pre-pass that found declarations would have to
  re-run the walk that decides them. The walk still aborts at the first construct
  error, so a document carrying a bad reference on line 3 and a raw HTML block on
  line 5 reports the HTML. **That is the whole of it** — every other error, in
  either pass's territory, still surfaces at its document position, and the
  frontmatter precedence is untouched. The original is kept because the claim it
  makes about *this* phase's two passes is unchanged and still true;
  `rules/pipeline.md` carries the corrected wording, since that is the artifact
  that tracks the code.

  **WIDENED 2026-08-18, by `mpdf-005` Phase 4: the exception is still one check
  and it now refuses two things.** That same after-the-walk pass also refuses a
  reference to a *display equation* in a document that did not set
  `equations: numbered`, because Typst fails the whole compile on one and its
  message names neither line nor key. The exception the note above records is
  unchanged in kind — one check, run late, over references — and the earliest
  line is the error across both classes together. Recorded because the note
  enumerates its one class by name, and a later reader meeting the second in the
  code would have no way to tell an omission from a regression.

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

### Phase 8 — the constructs the reject arm names but never sees
*Produces the observable: yes — a PDF whose struck text is struck, from
documents that today print their tildes as prose.*

Appended 2026-08-10, after Phase 7 shipped, per the methodology's §6.1: the
dialect is this spec's subject, and this phase closes the gap Phase 7's
close-out was obliged to name rather than opening a subject of its own.

The motivation is Phase 7's, and the criterion is sharper than "what is
missing". `core/src/emit.rs:describe` names six constructs it can reject, and
three of them can never arrive: strikethrough, a task list marker and math each
need a parser option, and none of those three options is set. So `~~struck~~`,
`- [ ] a` and `$x$` reach the walk as ordinary text, the escape rule escapes
their markers, and the PDF prints them as prose while the code claims it
refuses them. That is one shipped instance of the §2 faithfulness failure per
unreachable arm. When this phase ships, every arm in `describe` is reachable,
which is the property that keeps that function honest — and the property a
later reader can check in one sitting.

- **Scope:** In `core/src/emit.rs`: set `Options::ENABLE_STRIKETHROUGH`,
  `Options::ENABLE_TASKLISTS` and `Options::ENABLE_MATH` in `options`, the one
  builder both walks read, since a difference between them would be a
  difference in what the document means.

  Strikethrough joins the dialect. `Tag::Strikethrough` wraps its translated
  content in `#strike[…]`, and Phase 3's delimiter argument does not arise
  here: Typst has no markup form for a strike, so the function form is the only
  form there is. The parser accepts a delimiter run of one tilde as well as
  two — verified against pulldown-cmark 0.13.4's `firstpass.rs`, whose
  `is_valid_seq` admits both — so `~struck~` is strikethrough under this
  dialect, not prose, and the close-out's documentation says so. Inside an
  image's alt text, strikethrough joins `AltCapture`'s wrapper arm beside
  emphasis, strong and link: the wrapper contributes nothing and its inner
  text arrives, which is CommonMark's plain-text reading of alt. That
  disposition is what lets `describe` drop its two strikethrough arms
  honestly — unlike Phase 6's table arms, a strikethrough can occur inside
  alt content, so dropping the arms without the capture change would turn an
  in-dialect construct into a generic "markdown construct" error there.

  A task list marker becomes an error that names the construct and its line.
  Typst has no checkbox element, and a marker drawn as a character would be a
  look decision, which §2 gives to `template.typ` rather than to the emitter.
  Support is a later phase or a later spec; the named error is the honest floor
  meanwhile, and it is strictly better than the brackets the PDF prints today.
  `Event::TaskListMarker` keeps its `describe` arm, which the option makes
  reachable.

  Math becomes an error, both forms, per OQ-8's resolution: `InlineMath` and
  `DisplayMath` fall to the reject arm that already exists, which names the
  construct through their `describe` arm and the line through the event's
  range. No new mechanism is needed. The accepted cost and its `\$` exit are
  recorded in OQ-8; the README documents the escape at close-out.

  > **CORRECTED 2026-08-14, by `mpdf-004` Phase 1.** The paragraph above and
  > OQ-8's resolution are the record of what was decided in August 2026, and
  > they are kept. Half of it is no longer true of the code: **the inline form
  > `$x$` is in the dialect now**, scanned against a closed list of LaTeX
  > commands and converted by `mitex`, and only `$$x$$` still falls to the
  > reject arm. That is what keeps `describe`'s math arm reachable until
  > `mpdf-004` Phase 2 takes the display form too. The `\$` escape is
  > untouched and still the way to keep a dollar as prose. See
  > `specs/math_spec.md`; `rules/pipeline.md` is what tracks the code.
  >
  > **CORRECTED 2026-08-15, by `mpdf-004` Phase 2.** The other half has gone
  > too: **`$$x$$` is in the dialect now**, set as Typst's block equation, and
  > `describe` no longer names math at all. This phase's property — every arm
  > `describe` names is reachable — is restored by that arm being dropped
  > rather than kept alive, which is the half of this phase's own precedent
  > that applied. A second stamp rather than a moved one, because the first
  > records when the inline half stopped being true.

- **Exit gate:** Golden-file tests, three cases, plus the full existing suite,
  which the option change touches — no shipped golden file changes, because
  the corpus census finds no `~~` pair, no pairable `$` and no task-list
  bracket outside code contexts in any fixture. `tests/fixtures/hostile.md`
  carries a lone unpaired `~`, and its golden plus the escape-loop test in
  `core/tests/golden_test.rs` are what pin that an unpaired tilde still
  reaches the page as itself; `samples/article.md`'s "a ~ tilde" line has no
  golden and rides the corpus check, which proves it converts, not how.
  (1) A fixture with strikethrough alone, strikethrough spelled with one
  tilde, strikethrough nested inside emphasis, strikethrough around a link,
  and a strikethrough inside an image's alt text matches its golden file,
  shows the `#strike[…]` form and the alt flattened to its inner text, and
  compiles to a PDF with the `%PDF` magic bytes. The same fixture pins what
  stays text beside it: a `~~` inside inline code, which the string escape
  carries verbatim, and a `\$…\$` pair, whose backslash escapes keep it
  prose under `ENABLE_MATH`. (2) Inline math, display math and a task list
  marker each make the CLI exit non-zero naming the construct and its line —
  math through the existing reject arm per OQ-8's resolution, the marker
  through its own `describe` arm. (3) A raw HTML block still exits non-zero
  naming the construct and its line at both levels — rejection survives the
  widening.
- **Close-out:** Update `rules/pipeline.md`'s dialect section, the README and
  `samples/article.md` against the code. This phase deletes prose where the
  others added it: the gap paragraph Phase 7 was obliged to write — in the
  rule, in the README and in the sample, three artifacts saying the same thing
  — goes away, because nothing flattens any more, and the README's "Almost
  every other construct is an error" returns to the sentence it corrected. The
  README's documentation of strikethrough names the one-tilde form, and its
  math error gains the `\$` escape beside it, so the author refused a math
  span is told the one-character way out. The sample gains a struck phrase,
  which is what keeps the corpus check from passing vacuously; no corpus file
  carries one today. The sample's own "a ~ tilde" line survives unchanged —
  flanked by whitespace on both sides, that tilde can neither open nor close
  a run — which the corpus check now exercises under the new options. The corpus check repeats: the repository's
  own README and the sample both convert without error, or the gap is named
  in the review record. One push.

### Phase 9 — the look the frontmatter chooses
*Produces the observable: yes — a PDF in a second look, from markdown that
changes one frontmatter line and nothing else.*

Appended 2026-08-10, after Phase 8 shipped, per the methodology's §6.1: the
subject is the frontmatter schema and the styling decision, and this spec owns
both. §2's styling decision already reserved the mechanism — "a new look later
— an IEEE-style template, for example — is a new `.typ` file and a frontmatter
selector, with no change to the parser or the emitter" — so this phase builds
what that sentence promised rather than deciding anything new about where
styling lives.

Every phase from 3 to 8 widened the dialect. This one widens nothing in the
dialect: the same markdown converts, and the frontmatter says what it looks
like.

- **Scope:** In `core/src/frontmatter.rs`: two new keys. `template` takes a
  name from a fixed set of two — `article`, the default, which is the shipped
  look and keeps the filename `template.typ`, and `press-release`, which names
  `press-release.typ` — so every document written before this phase converts
  unchanged and the key stays optional like the rest. A name outside the set
  is an error that names the key and lists the accepted names, the way
  `columns` refuses a value outside `1` and `2`. `date`, per OQ-9's
  resolution, is an optional free string, typeset verbatim — no parsing, no
  formatting, no clock; `core/src/lib.rs:TypstWorld::today` stays `None` —
  its no-clock argument stands, and its "no template uses a date" sentence is
  retouched in the same pass, since every template now typesets an
  author-written one. Per OQ-10's resolution, an absent `columns` now
  resolves at parse time to the selected look's convention — `2` for
  `article`, `1` for `press-release` — so the schema, never the template,
  stays the home of every default, and `Frontmatter::default`'s own comment
  moves with the convention; an explicit `columns` wins over it either way.

  In `core/assets/`: `template.typ` keeps its filename and becomes the
  `article` look — renaming it would rewrite the import line in all thirteen
  checked-in golden files for no gain — and gains a date line beneath the
  author when `date` is present. `press-release.typ` joins it: single-column
  by convention, a dateline where press releases carry one, and the same
  `divider`. Every template exports `template` and `divider`, and its
  `template` accepts `title`, `author`, `columns` and `date`, because
  `core/src/emit.rs:header` names all four arguments on every call; a
  template missing one would fail the compile with an error naming neither
  the document nor the key. That contract is what a third look has to meet,
  and it is stated here rather than discovered later.

  In `core/src/emit.rs:header`: the import line names the selected file rather
  than a fixed one, so `#import "press-release.typ": template, divider` is what
  a press release gets. A fixed name would make two documents in two looks emit
  byte-identical source, and `--emit-typst` exists to show what a document
  compiles to; source that cannot say which look it takes is the flattening §2
  refuses. The call gains `date`, named on every call like the other three
  arguments, so all thirteen shipped golden files change on exactly their
  second line, gaining `date: none` — the deliberate one-line sweep Phase 3
  performed on the import line, and the gate pins it the same way. The import
  line of every shipped golden is untouched, because the default look keeps
  its filename.

  In `core/src/lib.rs`: `TypstWorld` binds **every** bundled template, not only
  the selected one, so `core/src/emit.rs:emit` keeps its return type and
  `md_to_pdf` keeps its signature. The alternative — plumbing the selected name
  out of the walk so the world binds one file — buys a smaller virtual
  filesystem and nothing else: the templates are compile-time constants either
  way, the dialect has no syntax for a raw Typst import, and only the emitter
  ever writes one. `TypstWorld::lookup` gains a table where it has two branches
  today, and the struct's doc comment stops saying two source files.

- **Exit gate:** Golden-file tests, four cases, plus the full existing suite,
  which the `header` and `Frontmatter` changes touch. Every shipped golden
  changes on exactly its second line, gaining `date: none` and nothing else —
  the equality tests over all thirteen regenerated goldens are what pin the
  sweep, as they pinned Phase 3's. (1) A fixture whose frontmatter carries no
  `template` key matches its golden — the `article` default, the import line
  unchanged, `date: none` named — and compiles to a PDF with the `%PDF` magic
  bytes; a second fixture with a `date` and no `template` key shows the date
  in a string literal on the call and its compiled PDF is read by eye once,
  confirming the article look renders the line. (2) A fixture with
  `template: press-release`, a `date`, and no `columns` key matches its own
  golden — the import line naming `press-release.typ`, `columns: 1` from the
  per-template convention, the date in a string literal — and compiles to a
  PDF with the `%PDF` magic bytes. That PDF is read by eye once, confirming
  the second look differs from the first on the page and carries its dateline
  — a golden pins emitter output and cannot pin a look, so the observable
  needs an artifact of its own, as OQ-6's header rule did in Phase 6. (3) A
  fixture whose `template` value sits outside the set makes the CLI exit
  non-zero with a message that names the key and lists the accepted names.
  (4) A test reads each bundled template's source and asserts it exports both
  `template` and `divider` and names all four arguments — the textual
  assertion Phase 6's template test already models, because no golden pins
  the file's side of the contract.
- **Close-out:** Update `rules/pipeline.md`'s frontmatter section, its template
  section and its world section against the code — the world stops holding two
  source files, and the frontmatter grows to five keys. The README's
  frontmatter documentation gains both new keys, the list of looks, and the
  per-template `columns` convention. `samples/`
  gains a second document in the press-release look rather than converting the
  existing one, which would lose the coverage the article sample carries. The
  corpus check repeats over both samples and the README: all three convert
  without error, or the gap is named in the review record. One push.

### Phase 10 — the title block gets its air back
*Produces the observable: yes — a PDF whose author and date stand clear of the title
instead of overlapping it, in both bundled looks.*

**Drafted 2026-08-31**, on §2's spacing measurement above, and appended per §6.1 step 2.
The ordered test lands on step 2 and the steps above it are worked rather than skipped.

- **Step 0 — a decision, not only code?** Yes. §2's styling decision gives the look the
  title block, and both looks have carried a spacing value since Phase 9. What changes is
  the value each look asks for — which is a look's own decision, taken here because the one
  it has been asking for lands the three lines on top of one another.
- **Step 1 — does it remove or contradict shipped work? No, and the measurement is what
  says so.** No phase is removed and no capability un-built: the same three keys reach the
  same block in the same order. What moves is where the lines sit. **`mpdf-004` Phase 3's
  property is bent** — every document carrying a title block moves — and §2 carries the
  bound: there is no key to condition the fix on, which is `mpdf-005` Phase 6's bound
  reached from the same direction, and what moves is a defect rather than a preference.
- **Step 2 — the subject.** The title block, which §2 assigns to the look and this spec has
  owned since Phase 2. So a phase, not a spec.

- **Scope: a value per join in the two look files, and deliberately not a rewrite.**

  `core/assets/template.typ`'s title block **keeps its `linebreak()`s and keeps
  `weak: true`**, and changes what it asks for: one `v(0.4em, …)` shared by both joins
  becomes a value chosen per join — `1.8em` between title and author and `0.9em` between
  author and date, measured at **+11.22** and **+3.93**. **The structural rewrite a draft of
  this phase prescribed is not in scope and would be a no-op**, per §2: the two shapes
  measure identically at equal values, and dropping the `linebreak()`s adds Typst's
  paragraph spacing on top of whatever the look asks for.

  `core/assets/press-release.typ` puts `above: 1.0em` on the **author block**, measured at
  **+3.47**. **Not on the title block's `below:`** — that value also governs a title-only
  document's gap to its `divider`, which gate (3) forbids moving, and round 1 measured that
  route failing at `1.4em` for exactly that reason. This is OQ-12's resolution rather than
  an open question the implementer inherits.

  **Every number above is a look's own call and none is a contract.** They are recorded so a
  second person can reproduce the gate, not so a third look must match them; the thresholds
  that matter — 6.78pt, 5.07pt, and zero for the press release — are in §2.

  **No `core/src` file changes and no golden moves.** A golden pins emitter output, and the
  emitter writes no spacing — the seam §2 drew in Phase 2 and every look-only phase since
  has used. `cli/src` and `app/src` are untouched. **The look call contract is unchanged at
  seven**: nothing new crosses it, because a `v()` is a look talking to itself.

- **Exit gate:** five cases, and the first is the phase. **The probe is named**, because
  round 1 was right that a by-eye read and a measurement over two different scratch
  documents are two verdicts rather than one.

  **The probe**, used by cases (1) and (2), and written out so a second person reproduces it
  rather than invents it: a document whose frontmatter carries `title`, `author` and `date`
  and nothing else, whose title is long enough to wrap in the press-release look at 20pt,
  followed by one `#` heading and one short paragraph. It is rendered twice — once in the
  default look and once in a copy carrying `template: press-release` — the two-PDF read
  Phase 9 gate case (2) established and every look-only phase since has used.

  (1) **Read by eye, one PDF per look.** The author stands clear of the title and the date
  clear of the author, in both looks, and the press release's wrapping headline still sets
  tight against itself.

  (2) **The overlap is gone, as arithmetic rather than a reading.** In one `pdftotext -bbox`
  run per look over that same probe, **each join between two of the three keys is separated
  by a positive number**. That is title→author and author→date in the article look, and
  date→title and title→author in the press release, whose masthead puts its date first —
  **four joins, not three**, which round 2 caught the enumeration dropping. §2 measured
  **−2.78**, **−1.07** and **−2.03** today and **+11.22**, **+3.93** and **+3.47** under the
  values the scope names; the press release's date→title join is **+2.58** and is not
  touched by this phase, since the fix sits on the author block below it.

  **The pairs are the joins between keys and not every consecutive pair of boxes**, which
  round 1 caught and which is the difference between a gate a correct fix passes and one it
  cannot. A press-release headline that wraps sets its own two lines **−2.90** apart under
  `par(leading: 0.35em)`, deliberately and since Phase 9; a gate reading every consecutive
  pair would fail a correct fix and invite an implementer to undo that leading.

  **This case catches a value raised on one join and not the other, and it is honest that it
  catches nothing subtler.** §2 killed the discriminator a draft of this gate claimed — that
  a value raised without a structural change "still collapses to nothing" — by measuring the
  two shapes identical. The fix is a number, so the gate is the measurement.

  (3) **A document that carries only `title` is byte-identical, in both looks**, compared as
  a PDF hash against the same document built from `HEAD`. **The document is named for the
  reason the probe above is** — round 2 caught this case publishing two hashes a second
  person could not reproduce: frontmatter carrying `title` and nothing else, then one `#`
  heading and one short paragraph, rendered in each look. **This is the case that bounds the
  press-release fix and the reason its spacing sits on the author block**: the title block's
  own `below:` governs a title-only document's gap to its `divider`, and round 1 measured
  that route moving the hash at `1.4em`. Measured under the scope's values, both looks
  unchanged: `54d9134121…` for the article and `ceeb79ea11…` for the press release, before
  and after.

  (4) **`cargo test --workspace` passes with no golden re-blessed and no `core/src` file
  touched** — `git diff --stat` naming the two `.typ` files and the close-out's prose, and
  nothing else. **The three shipped geometric assertions in the blast radius are named**, the
  last on round 1's catch: `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`,
  which pins pagination over `samples/article.md`;
  `core/tests/long_document_test.rs:the_fixtures_are_the_lengths_phase_5_measures_against`,
  which pins `tests/fixtures/long.md` at exactly 71 pages **and a cross-reference on page 64
  at fraction 0.620 ± 0.01**, over a document carrying all three keys — the tightest such
  assertion in the suite and the one a reader would most want named; and the showcase's six
  pages, pinned by the hand-run `tests/gates/mpdf-009-phase5.js` rather than by the suite, so
  it is measured and reported even though no gate here can fail on it.

  (5) **Nothing in the suite pins this fix afterwards, and that is stated rather than
  papered over.** Every look-only phase before this one left a `BUNDLED_TEMPLATES` needle;
  this one leaves none, because what it changes is a **value**, and a value is not a needle
  in this corpus — the same rule that keeps a caption's separator, a group's gutter and a
  listing's inset length off the needle lists. The regression round 1 feared, an edit
  restoring `linebreak()` and weak spacing, is now known to be **harmless**: §2 measures the
  two shapes identical. The regression that would matter is someone lowering the value, which
  no needle can see and which case (2) catches only while it is run. **The gate case is
  therefore to record this in `rules/pipeline.md`** — that the title block's spacing is
  unpinned and why — so the next person to touch it is told rather than left to find out.

- **Close-out: one line owed and nothing corrected, with the census here rather than left
  to the implementer to repeat.** `rules/pipeline.md`'s template section states the block's
  *mechanism* — the article's `place(scope: "parent", float: true)`, the date beneath the
  author, the press release's dateline above a flush-left title over a `divider` — and the
  README's look table says the same in a row each, as does `web/index.html`'s "A centred
  title block". **Not one of them states a distance**, which is what this phase changes, so
  every sentence in all three survives the fix verbatim. **What is owed is the line gate (5)
  names**: `rules/pipeline.md` gains the note that the title block's spacing is a value no
  needle pins, so the next person to touch it is told rather than left to find out.
  **That file has two lines of headroom** — 1008 body lines against its own
  `max_lines: 1010`, which `.spec-lint.yaml` enforces — so the note plus its blank line
  lands it at exactly the cap. **The close-out raises `max_lines` to 1020 rather than
  trimming**, and the reason is that a rule sitting at exactly its cap is one the next phase
  cannot touch without an unrelated edit; `mpdf-005` Phase 9 already trimmed this file twice
  to stay under it.
  `samples/` moves on the page and that is the point; no sample's prose describes its own
  title block, and `samples/*.pdf` is gitignored so no committed artifact goes stale.
  **Neither index moves** — the rollup stays `partial` while Phase 11 is unshipped — so the
  spec's `shipped` date is the rest of the paperwork. One push.

### Phase 11 — several authors, and the affiliation each belongs to
*Produces the observable: yes — a PDF whose title block carries three authors on one line
with superscript markers, and the two affiliations those markers point at beneath them.*

**Drafted 2026-08-31**, on §2's authors decision above, and appended per §6.1 step 2.

- **Step 0 — a decision, not only code?** Yes, two. OQ-3 fixed the frontmatter schema with
  `author` as one free string, and §2's styling decision fixed what the look does with it.
- **Step 1 — does it remove or contradict shipped work?** **No shipped document's page
  moves, one shipped *contract* does, and every shipped golden's call line moves — which is
  the honest answer rather than the comfortable one.** A document writing `author: Iva Po`
  is a one-element list and compiles to the same page. But `author` crosses the seam as an
  array where it crossed as a string **and** `affiliation` is added beside it, so the look
  contract Phase 9 stated both changes an argument's type and gains one. §2 carries the
  argument and the mitigation: no third look exists, and the alternative collapses the seam.
  **All twenty-nine shipped goldens are re-blessed**, not seven — `core/src/emit.rs:header`
  is one unconditional `format!` that names every argument on every call, so an eighth
  argument lands on the second line of every one of them, exactly as OQ-9's `date` moved all
  thirteen and `mpdf-005` Phase 8's seventh argument moved twenty-eight. **Round 1 caught a
  draft of this step claiming seven**, which is the count of goldens whose `author` value
  *also* changes; the gate now names both counts so a wrong one is visible.

- **Step 2 — the subject.** The frontmatter schema and the title block, which Phase 9 says
  in its own words this spec owns both of. So a phase, not a spec.

- **Scope: the schema, the call, and both looks.**

  `core/src/frontmatter.rs` — `author` becomes a list of names each carrying its markers and
  a new `affiliation` key takes a list, both under **the grammar §2's correction
  specifies**: split on `;`, every element trimmed, the name splitting from its markers at
  the **first** `^`, markers split on `,` and each trimmed, a marker a non-empty run of
  ASCII digits indexing `affiliation` in written order from 1. **Four refusals**: a marker
  naming an affiliation the document does not carry; an `affiliation` key with no marker
  anywhere **and two or more affiliations**, since OQ-11 makes the marker optional at exactly
  one; a marker that is not a number; and **an empty element in *either* list**, which round
  2 widened from the author list alone — `affiliation: MIT;` leaves a blank second
  affiliation that `^2` would then point at without tripping the first refusal. **An author
  with no marker in a document that has affiliations is not refused**, per §2.

  **Which line each refusal names is assigned here, by this phase** — §2 says only that a
  refusal names a line, and round 2 caught a draft crediting it with more and then
  contradicting itself two paragraphs later. **Each refusal names the line of the key the
  author would have to edit**: refusals 1, 3 and 4 name the `author` line when the fault is
  in `author` and the `affiliation` line when it is in `affiliation`; refusal 2 names the
  `affiliation` line, since that is the key whose relation is unstated.

  Refusals 1 and 2 are **cross-key checks that can only run after the line loop**, because
  `affiliation` may sit below `author` — the shape `columns` and `template` already use. So
  `Frontmatter` gains line-carrying state for **both** keys, the way `bibliography` already
  keeps a `Location` — round 2 caught a draft provisioning it for `author` alone.

  `core/src/emit.rs:header` writes both as Typst arrays, **with the trailing comma a
  one-element array needs and `none` rather than `()` for an absent author** — both traps
  §2's correction records. **`core/src/lib.rs:Error` needs no new variant**: all four
  refusals are `Frontmatter { location, problem }`, which every other schema refusal uses.

  `core/assets/template.typ` and `core/assets/press-release.typ` each render the two lists
  for themselves: the marker as a superscript, the authors on one line, and **the
  affiliations directly beneath the authors — in both looks**, each look picking its own
  size, separator and emphasis. **Round 2 caught a draft of this line saying "between the
  authors and the date", which is unsatisfiable in the press release**: that masthead runs
  date, title, author, `divider`, so its date is above the headline and there is no position
  between the two. "Beneath the authors" is the one instruction true of both. **Measured in §2 to need no package**, and round 1 reproduced that measurement
  independently. Each look also picks the join spacing for the run it adds — Phase 10 tuned
  that block and `rules/pipeline.md` records that no needle pins its values.

  **An absent `affiliation` crosses as `none`**, the same byte every other optional key
  reaches through `core/src/emit.rs:typst_string_or_none` — though not through that function,
  which renders a string literal from an `Option<&str>`; a list-valued key needs an
  array-or-`none` helper beside it. Stated because it is a byte in all twenty-nine re-blessed
  goldens.

  **`core/src/sections.rs` and `core/src/bibliography.rs` are untouched**, and so are
  `cli/src` and `app/src` — neither wrapper reads the schema, verified by round 1 as a grep
  rather than assumed. **The one thing that could break `app/src` is the sample sweep**, and
  the close-out disposes of it.

- **Exit gate:** eight cases. **Round 1 rewrote four of them**, because two contradicted
  each other, one asserted a count a correct build cannot produce, and nothing checked in
  pinned the phase's own output.

  **The fixture is named and checked in**, which every construct- or key-adding phase in
  this spec has shipped and a draft of this phase omitted: `tests/fixtures/authors.md`
  carries `title`, `author`, `affiliation` and `date` — the full list gate (2) needs, since
  the press release's first join is its dateline — with three authors and two affiliations,
  the third author in both, **one name containing a comma** — which is what pins §2's sharpest call, that a comma is not a
  separator and `Po, Iva` stays one person — and a marker written `^1, 2` with the space, so
  the trim is pinned where an author would actually type it.

  (1) **`tests/fixtures/authors.md` matches `tests/golden/authors.typ`** and compiles to a
  PDF with the `%PDF` magic bytes. The golden shows both Typst arrays, the trailing comma
  where one is needed, the comma'd name intact as one element, and `^1, 2` reduced to two
  markers. **This is the case a draft of this phase had no artifact for at all.**

  (2) **Read by eye, one PDF per look**, over that same fixture: the markers point at the
  right affiliations, the multiple marker reads as one superscript, and — because this phase
  inserts a run into the block Phase 10 tuned — **every join in the block is separated by a
  positive number**, in one `pdftotext -bbox` run per look, the method Phase 10's gate case
  (2) established. **Each look has its own join list and they are enumerated separately**,
  which is the enumeration failure Phase 10's round 2 caught and round 2 caught again here:
  the article's are title→authors, authors→affiliations and affiliations→date; the press
  release's are date→title, title→authors and authors→affiliations, its masthead putting the
  date first. Three each, and as in Phase 10 they are the joins **between** keys — a wrapping
  headline sets its own lines tighter, deliberately, in both looks.

  (3) **The one-affiliation document, which OQ-11 said this gate could not state.** Two
  authors and one `affiliation`, **written with no marker**, is valid, compiles, and puts no
  marker on the page; the same document with `^1` on both names is also valid and does print
  them. Two fixtures or one fixture and one inline document, in both looks.

  (4) **Every refusal names its own line**, as four tests — the fourth covering **both**
  shapes of the empty element, in `author` and in `affiliation` —
  over `Error::Frontmatter`, asserting `location.line` and `problem` — the shape
  `core/src/frontmatter.rs:errors_name_the_key_and_the_line` already uses, which asserts
  both. **Round 1 recorded that this file's tests assert the problem alone and round 2
  falsified it**; some do, that one does not, and the correction is kept visible because a
  false claim about the code is what a later reader would re-derive.

  (5) **A one-name document's compiled PDF is byte-identical against `HEAD`, and its
  `--emit-typst` changes on the call line alone.** This replaces a draft case claiming the
  emitted source was byte-identical too, which **a correct implementation cannot satisfy**:
  `tests/golden/frontmatter.typ` is a one-name document and is one of the seven whose
  `author` value changes. §2's survivable wording is "a one-name document is a one-element
  list, and *its page* does not move", and that is what this case reads. The document is
  named: `tests/fixtures/frontmatter.md` for the article look and a copy of it carrying
  `template: press-release` for the other, the two-PDF form Phase 10's gate case (3) used.

  (6) **All twenty-nine goldens are re-blessed, and the two counts are both named.** Each
  changes on its `template.with` line alone; **the seven carrying a non-`none` author change
  in two ways** — the array form and the new argument — **and the other twenty-two in one**,
  the new argument only. Checked as a diff naming the files, the way Phase 9 and `mpdf-005`
  Phase 8 checked theirs.

  (7) **Both looks carry the contract, by two needles rather than one**, per the doctrine
  `rules/pipeline.md` records — "the parameter alone is satisfied by a look that takes it
  and ignores it". So `affiliation` the parameter, and the `super()` call that renders a
  marker. `every_bundled_template_meets_the_call_contract` gains `affiliation` beside the
  seven it names today.

  (8) **`cargo test --workspace` passes**, and **the three shipped assertions over the
  `template.with` line are named in the radius** rather than discovered:
  `core/tests/golden_test.rs:the_generated_source_carries_the_title_and_the_author`,
  `:absent_frontmatter_gets_every_default` — the only literal copy of the whole call line
  outside the goldens — and `:a_key_repeated_across_two_frontmatter_blocks_names_its_own_line`.
  **`spec-lint` runs too**, which `cargo test` does not: the close-out edits a rule with five
  lines of headroom.

- **Close-out**, named line by line because this phase's prose radius is wider than its code
  radius, and **round 1 found six sites a draft of it missed**.

  `rules/pipeline.md`: the frontmatter section's key count, **and the second count in the
  same passage** — "Eight of them reach the look: `core/src/emit.rs:header` always names all
  seven arguments and the selected file" — which is the line `mpdf-005`'s round caught
  carrying two counts; the template section's call-contract argument list; and the sentence
  **"The date sets beneath the author"**, which stops being the whole order once an
  affiliation sits between them. **The sentence is quoted rather than cited by line number**,
  per §5: a line number is not a citation, and a draft of this close-out carried one.
  **The file has five lines of headroom against `max_lines: 1020`** and this phase adds a
  key, a five-clause grammar, four refusals with the line each names, an eighth argument and
  the affiliation run in both looks. So the close-out **raises the cap to 1070** rather than
  trimming — the number is named because round 2 caught "raises the cap" being uncheckable,
  and it is calibrated on `mpdf-005` Phase 8, which needed 960 → 1010 for one key plus one
  parameter. The rule's `covers` line gains a clause for the new key, on that same
  precedent; frontmatter lines are free against `max_lines`.

  `README.md`: the frontmatter documentation, the key count, the `## Styling` contract
  sentence — "`md2pdf` names all seven on every call" — and the look table's `article` row,
  "the date under the author", for the ordering reason above.

  **Three code sites state a count or the order and are in the radius**:
  `core/src/frontmatter.rs`'s module doc comment ("the schema is nine keys"), both
  `core/assets/*.typ` header comments (the article's "That is the contract a third look has
  to meet" and the press release's "the same seven the article look takes"), and
  `core/assets/template.typ`'s "The date sits under the author, set smaller." **A fifth
  states the contract count and round 3 found it**: `core/tests/golden_test.rs`'s doc comment
  on `every_bundled_template_meets_the_call_contract` — "`header` names all seven arguments
  on every call" — three lines above the code gate (7) already edits, which also makes §2's
  corrected tally one short.

  `web/index.html`: the page states the schema, so it gains the key — **and carries an
  ordinal, "A ninth frontmatter key names a bibliography"**, which a close-out stopping at
  the key list would leave stale. `rules/web-demo.md` records that the page's examples are
  compiled, so a new key with no example is a choice the close-out names — **and if it names
  the example, three more things move**: `core/tests/page_examples_test.rs`'s
  `EXPECTED: usize = 12`, that file's `the_page_carries_twelve_examples` in its *name*, and
  `rules/web-demo.md`'s "three groups and twelve examples". Round 2 found all three, and they
  retire an assumption the corpus was carrying — that this test asserts nothing about the
  count. **That file carries "twelve" on eight lines and only one is the sentence above**, so
  the arm that adds an example sweeps the word rather than the line. `rules/web-demo.md`'s
  "the eight frontmatter keys that decide the look" moves with the page's own heading either
  way.

  **`samples/` is the widest stale surface and the close-out makes an explicit call rather
  than sweeping blindly.** `samples/article.md` already carries five claims falsified by the
  *ninth* key — "four of the eight keys", "all eight are optional", "names all eight at
  once", "A key outside those eight", "The three keys this file leaves out" — a
  pre-existing miss this phase inherits and **corrects, since it is editing the same
  sentences** — **including its "A date key joins them there, set beneath the author line"**,
  the ordering sentence's last instance, which round 2 found four lines above the counts and
  in the same paragraph. `README.md`'s "under all nine frontmatter keys",
  `samples/showcase/README.md`'s "all nine frontmatter keys", and
  `samples/showcase/showcase.md`'s heading "The frontmatter, all nine keys", its "every key
  there is, and all nine are optional" and its "`title`, `author` and `date` become the block
  at the top of the first page" — the last a content claim rather than a count, which the
  others do not cover. Adding the tenth key to the showcase would shift its frontmatter by a
  line and break
  `app/src/preview.rs:the_anchors_are_the_headings_of_whichever_file_the_pane_holds`, which
  pins `[13, 28, 60]` — **colliding with this phase's own "`app/src` untouched"**. Those are
  heading *line numbers*, so the real constraint is **no line added or removed above line 60
  of `showcase.md`**, which round 2 sharpened from "its frontmatter is not extended" and
  which the prose corrections above must respect. So the showcase's *prose* is corrected
  within that bound and its frontmatter is not extended; the two-author
  document gate case (1) needs is a **new sample**, which is also what keeps the corpus
  check from passing vacuously. That trap is the one `mpdf-005` Phase 8 hit and made a call
  on; this phase inherits it and makes the same one.

  **`specs/INDEX.md` and `rules/INDEX.md` regenerated**, never hand-edited — the spec's
  rollup goes `partial` → `done` as the last unshipped phase lands. One push.

### Phase 12 — a marker with nothing to point at
*Produces the observable: yes — a PDF from a document that today does not compile at all,
its byline setting the names their author wrote without the markers that point nowhere.*

**Drafted 2026-08-31**, on §2's correction above, and appended per §6.1 step 2. The ordered
test lands on step 2 and the steps above it are worked rather than skipped.

- **Step 0 — a decision, not only code?** Yes. Refusal 1's scope is a schema decision, taken
  in Phase 11 and stated absolutely there. What changes is where it begins.
- **Step 1 — does it remove or contradict shipped work?** **It narrows a shipped refusal,
  and the distinction from removal is that nothing built is un-built.** Phase 11 stands whole:
  every marker that pointed at a real affiliation still does, and three of the four refusals
  are untouched. **The prose is a different matter and is corrected in place**, per §6.1's
  third sub-case: §2's absolute statement of refusal 1 is actively misleading now, so it
  carries a dated correction beside it rather than a note in a sibling file the reader never
  reaches. **The precedent is this phase's own predecessor** — Phase 11 relaxed refusal 2 at
  one affiliation by exactly this pair of moves, a phase plus a `CORRECTED` block in §2.
- **Step 2 — the subject.** The frontmatter schema, which this spec has owned since Phase 2
  and OQ-3. So a phase, not a spec.

- **Scope: one condition and one clearing pass, in one function.**

  The cross-key check that `core/src/frontmatter.rs` runs after its line loop gains a zero
  arm. Where the document carries **no** affiliation, every author's `markers` is emptied
  and the block is valid; where it carries one or more, refusal 1 is exactly what it is
  today, `^0` included. The function now resolves as well as refuses, so it takes
  `&mut Frontmatter` and **is renamed from `check_affiliations` to `resolve_affiliations`**.

  **The new name is given rather than left to the implementer, because a rename moves a
  `file:symbol` citation and gate (5) runs `spec-lint`, which errors on a symbol that is not
  there.** Round 1 caught this: `rules/pipeline.md` cites the old name, the close-out below
  now names that citation, and this phase's own scope deliberately cites the *file* rather
  than the symbol so that nothing in an append-only document goes stale the moment the
  rename lands.

  **The message added the same day is deleted with the case it named** — the arm whose
  literal is `key 'author' marks a name '^{marker}' and the document names no
  'affiliation'; add that key, or drop the markers`, quoted as the source spells it because
  round 1 caught a draft quoting a rendered instance that greps to nothing. The two rows
  keyed to it in `core/src/frontmatter.rs:every_affiliation_refusal_names_its_own_line` —
  the ones matching `names no 'affiliation'` and `drop the markers` — go with it. The
  marker-past-the-end message stays, since there the fix is a number.

  **No `.typ` file changes and no golden moves.** `core` clears the markers, so what the
  looks receive is a document with no markers and no affiliations, which is a shape both
  already render — the seam §2's correction argues for. No shipped golden carries a marker
  without an affiliation, so none moves; the call contract stays at eight.

  **`core/src/emit.rs` is untouched**, and so are `cli/src` and `app/src`.

- **Exit gate:** five cases. **Round 1 rewrote all five**, because one named an input a
  correct build cannot refuse, one enumerated a diff a correct build cannot produce, one read
  a property out of a dump that does not carry it, and two were underspecified against this
  spec's own precedents — round 2 caught a draft of this sentence claiming three.

  **The fixture is written out rather than described**, on Phase 10's rule that a second
  person reproduces a probe rather than inventing it. `tests/fixtures/orphan_markers.md`
  carries `title: Two Authors, No Lab` and `author: Iva Po^1; Someone Else^2`, no
  `affiliation` line and no other key, then one `#` heading and one short paragraph.
  **No name in it carries a digit of its own**, which is what makes gate (2) readable.

  (1) **The document compiles, where today it exits non-zero.** Its `--emit-typst` shows
  `markers: ()` on **every** author and `affiliation: none`, and its compile starts with the
  `%PDF` magic bytes. **The assertion is checked in, and the file it lands in is named:
  `core/tests/golden_test.rs`**, beside `ONE_AFFILIATION_MD` and on the shape
  `one_affiliation_carries_its_markers_or_leaves_them_out` already uses — an `include_str!`
  constant and an inline assertion over `md_to_typst`, not a golden file, since a golden
  would pin the whole document where what this phase changes is one argument. Round 1 caught
  that the phase named no file and that gate (5) then forbade the only one that works.

  (2) **The byline carries the names and nothing else, in both looks.** One `pdftotext` run
  per look over that fixture — the second look a copy of the fixture carrying
  `template: press-release`, the substitution Phase 11 gate (5) spelled out and this one
  inherits. **The assertion is mechanical: the byline line reads exactly
  `Iva Po, Someone Else`, with no digit anywhere on it.** A superscript reaches a
  `pdftotext` dump as a bare ASCII digit glued to the name it rides — today's
  `tests/fixtures/authors.md` dumps its **article** byline as
  `Po, Iva1, Someone Else2, A Third Person1,2`, and its press-release byline the same but for
  a space before the last marker — measured, not explained, since the gate does not rest on
  why — so "no digit" is the readable form of "no superscript" in both, and it is readable only because the
  fixture's names carry none. **Phase 10 and Phase 11's `-bbox` arithmetic is deliberately
  not inherited**: a box dump reports where a run sits, not whether it is a superscript.
  What is inherited from Phase 9 gate (2) is the two-PDF shape and nothing else.

  **This case is what catches a clearing pass that empties only the first author**, since
  both looks set `super()` per author with no guard on `affiliation`, so a surviving `^2`
  would reach the byline as a digit.

  (3) **Every refusal that survives still fires and names its line**, as one test over
  `Error::Frontmatter` asserting `location.line` and `problem`, on
  `core/src/frontmatter.rs:errors_name_the_key_and_the_line`'s shape. **The boundary this
  phase moves is at zero and not at one**, which a draft of this case had wrong: `^1` under
  one affiliation is valid today and stays valid, and
  `core/src/frontmatter.rs:one_affiliation_makes_the_marker_optional` already asserts it. The
  rows are: `^2` under **one** affiliation, which is refusal 1 at the new boundary and the
  case that fails an implementation narrowing to `count <= 1`; `^3` under two; `^0` under
  two; a marker that is not a number, asserted under an affiliation **and** under none; an
  empty element in the **author** list under no affiliation, and one in the `affiliation`
  list — which cannot be asserted "under none", since an empty element there presupposes the
  key, and round 2 caught a draft claiming otherwise; **an entry with no name before its
  `^`**, which §2 records as the fifth refusal this document had never counted and which is
  enumerated here rather than left out, since omitting it right after promoting it would read
  as an oversight; and refusal 2 at two affiliations with no marker anywhere. All of these
  but refusals 1 and 2 fire in the line loop before any relation is known, and none may
  become conditional.

  (4) **A document that carries an affiliation does not move.** `tests/fixtures/authors.md`
  compiles to a PDF byte-identical against `HEAD`, in both looks, compared as a hash — the
  form Phase 10 gate case (3) and Phase 11 gate case (5) both used, with the press-release
  copy made the way gate (2) above makes its own. **The hashes are published, because Phase
  10's round 2 caught a case a second person could not reproduce**, and **the digest is named
  because round 2 caught that publishing one without it is the same defect**: under `shasum`
  with no flags, which is **SHA-1**, `98103556cc…` for the article and `57a161d3e9…` for the
  press release, measured 2026-08-31 on the shipped build, before and after. A second person
  reaching for `shasum -a 256` gets a different pair and would read a match as a failure.

  (5) **`cargo test --workspace` passes and `spec-lint` exits zero with no error**, the
  second because `cargo test` does not run it and this phase renames a cited symbol. **The
  one warning it prints is inherited and is named here so an implementer does not chase it**:
  `rules/desktop-geometry.md` carries `RULE_SOURCES_WITHOUT_GENERATED`, pre-existing and
  untouched by this phase. **The diff names
  `core/src/frontmatter.rs`, `core/tests/golden_test.rs`, `tests/fixtures/orphan_markers.md`,
  the close-out's prose, this spec and the two regenerated indices — and nothing else**; in
  particular no `.typ` file and no golden, which is what says the clearing happened in `core`
  rather than in a look. Round 1 caught the earlier enumeration excluding the test file gate
  (1) needs, the spec itself and the indices the close-out's own last paragraph regenerates.

  **The shipped assertion in the blast radius is named rather than discovered**, as Phase 10
  gate (4) and Phase 11 gate (8) each named theirs:
  `core/tests/golden_test.rs:one_affiliation_carries_its_markers_or_leaves_them_out`, which
  is the one test that fails an implementation over-narrowing refusal 1 to `count <= 1`.

- **Close-out**, named line by line, since this phase's prose radius is again wider than its
  code radius. **Round 1 found eight sites a draft of it missed**, four of them inside the
  one file the phase edits — the same undercount Phase 11's close-out recorded, recurring.

  `rules/pipeline.md`, three passages: the refusals paragraph, whose first sentence states
  refusal 1 absolutely **and which carries the one `check_affiliations` citation outside this
  file that the rename moves**; the site is named without its path deliberately, because a
  `file:symbol` citation of a symbol this phase deletes would fail gate (5)'s own `spec-lint`
  clause in an append-only document — round 2 caught a draft of this line doing exactly that,
  four paragraphs after the scope explained why it must not; the sentence **"With exactly one affiliation the markers are
  optional"**, now the middle of a three-way answer rather than the whole of it; and the
  looks paragraph beginning **"Whether a marker reaches the page is the look's, read off the
  data rather than off a key"**, which after this phase is incomplete in the direction §2
  calls load-bearing — at zero affiliations the suppression is `core` clearing the data, not
  the look reading it. The file has **eleven lines of headroom against `max_lines: 1080`**,
  re-derived by all three round-1 lenses, and this phase adds one scoped condition to each of
  three passages, so **no cap change is owed** — stated because the two phases before this
  one both owed one.

  `README.md`, two sentences in one paragraph: the refusal list — "So is a marker pointing at
  an affiliation you did not write" — and **"With exactly **one** affiliation you may leave
  the markers out and every author belongs to it"** one line above it, which frames the whole
  as a two-step ladder this phase makes three.

  `samples/authors.md`: its "What you may leave out" section and its own copy of the refusal
  list. **This sample is the close-out's real work**, because it is the file that answers the
  question this phase came from.

  **`samples/showcase/showcase.md` carries the same sentence and is named with its trap.**
  Its "With exactly one affiliation you may leave the markers out and every author belongs to
  it" is the third instance. **The correction must be line-count-neutral above line 68**:
  `app/src/preview.rs:the_anchors_are_the_headings_of_whichever_file_the_pane_holds` pins
  `[14, 29, 68]`, and a line added above the last of those collides with this phase's own
  "`app/src` untouched" and with gate (5)'s file set. Phase 11's close-out named this trap at
  line 60; the showcase has since grown and the bound moved with it.

  **Four code sites state the refusal absolutely or describe what becomes of a marker, all in
  `core/src/frontmatter.rs`**, where a draft named one: the comment above the marker check;
  the renamed function's own doc comment, which says it is "the two refusals that read both
  list keys at once" and is now two refusals and a clearing pass; **`fn author`'s doc
  comment, which is the one that becomes false rather than merely stale** — it says a
  saturated digit run is refused as "the marker naming an affiliation the document does not
  carry", which at zero affiliations it no longer is, and it names the pre-rename symbol; and
  `Author`'s own doc comment, whose "empty where the author wrote none — which the schema
  permits at exactly one affiliation" now enumerates one of two ways `markers` comes to be
  empty. A fifth sits in the test module, on
  `every_affiliation_refusal_names_its_own_line`, stating the four refusals absolutely.

  `core/tests/golden_test.rs`: the doc comment on
  `one_affiliation_carries_its_markers_or_leaves_them_out`, which opens with the same
  one-affiliation sentence. The file is in the diff for gate (1) regardless.

  **`web/index.html` states no refusal about markers** and does not move — checked by two
  round-1 lenses rather than assumed. **`core/assets/*.typ` are untouched**, which a draft
  argued from the false premise that they state nothing about refusals.
  `core/assets/template.typ` uses the word twice — about a `show` rule and about the
  title-block guard — `core/assets/press-release.typ` not at all, and neither instance is
  about markers; round 2 corrected the attribution to both files. **Their `marked` comments
  are a knowing carry-forward**: each says the markers are optional "at exactly one
  affiliation", which after this phase enumerates one of two ways `markers` comes to be
  empty — the same class as `Author`'s doc comment above. They stay, because gate (5)'s "no
  `.typ` file touched" is what says the clearing happened in `core` rather than in a look,
  and that is worth more than the sentence.

  **`specs/INDEX.md` and `rules/INDEX.md` regenerated**, never hand-edited — the spec's
  rollup goes `done` → `partial` as this phase is appended and back to `done` when it lands.
  One push.

### Phase 13 — the binary carries its own licences

*Produces the observable: **no**, and the gate is that it does not move.* This phase adds
a flag that prints text and touches no path a PDF travels; **gate (6)** is the PDF not
moving, which is what a phase in this position owes. (That read "gate (4)" until round 2:
the round-1 fold renumbered five cases to seven and this one reference did not travel with
them — the phase's own §3 discharge pointing at the wrong clause.)

**The argument for producing none, stated at the strength the facts actually support.** A
draft of this line said the alternative was "an observable someone may not lawfully
redistribute", and round 1's scope lens measured that claim too strong: `.github/workflows/`
holds `test.yml` and nothing else, and **this repository distributes no compiled binary at
all** — no release workflow, no bottle, no bundle. Both documented install paths build on
the reader's own machine, and since the licence fix of 2026-09-02 both leave `LICENSE` and
`THIRD-PARTY-LICENSES.md` on disk beside the source. **So everyone who produces a binary
today already holds the text**, and the consumer this phase serves is the third party who
receives a *hand-copied* one — real, conventional to serve, and currently hypothetical for
this project. That is the honest scope of the benefit and it is written here rather than
inflated, per §5. **What is not hypothetical is the `core` half**: `include_str!` cannot
reach `core/assets/fonts/` from a published `md2pdf-cli` archive, so the CLI's own
requirement forces a public const in `core` whatever anyone downstream ever does with it.

**Drafted 2026-09-02**, after `mpdf-011` Phase 3 published the crates and a licence audit
found the gap. Appended per §6.1 step 2; the steps above it are worked rather than skipped.

- **Step 0 — a decision, or only the code?** A decision. What the CLI's surface is, and
  whether the binary discharges an attribution obligation on its own or delegates it to a
  README. **The repository half was code and shipped as one** — `LICENSE` and
  `THIRD-PARTY-LICENSES.md` now travel in both packages — and no spec was owed for it,
  because a declared `license = "MIT"` shipping without the MIT text is a defect and not a
  decision. What is left over is the part a fix could not settle: a flag.
- **Step 1 — does it remove or contradict shipped work?** No. Phase 1 set the CLI at
  `md2pdf input.md [-o output.pdf]` and introduced `--emit-typst`; a third optional flag
  extends it again. **One shipped statement does narrow**, and it is a grammar rather than
  a claim: `input` stops being unconditionally required. That is not a contradiction to
  correct in place, because no prose in this spec states the requirement as a decision — it
  states the command's shape, which gains an alternative.
- **Step 2 — is its subject one an existing spec owns?** **Yes, and this one.** The CLI
  contract has been this spec's since Phase 1, and `rules/pipeline.md`'s `covers:` ends with
  it. The rollup is `done` rather than `abandoned`, so a phase is the mechanism. **The one
  rival a reader would name is disposed of rather than left implicit**: `mpdf-011` shipped
  the licence *artifacts*, but its subject is the split and the publication, its rollup is
  `done`, and its §1.2 pushes distribution away outright — *"Not signing, packaging or
  distribution."*
- **Step 3 is not reached.**

- **Scope: one export in `core`, one flag in `cli`, and no third place.**

  **`core/src/lib.rs` gains a public const carrying the two font licences**, beside the
  `include_bytes!` block that already embeds the faces they cover. The name is given here
  rather than left to the implementer because it is **public API of a published crate**, so
  choosing it is a decision and not an implementer's call, and the close-out below names the
  `rules/pipeline.md` sentence that must cite it.

      pub const FONT_LICENSES: &[(&str, &str)]

  Two entries, `("OFL.txt", …)` and `("GUST-FONT-LICENSE.txt", …)`, each `include_str!` of
  the file already sitting in `core/assets/fonts/`. **It belongs to `core` because the CLI
  cannot reach those files**: a published `md2pdf-cli` archive holds **nothing of `core`'s**
  — its own sources, tests, manifests, README and the two root licence files, and no
  `assets/` — so `include_str!("../../core/assets/…")` resolves in a checkout and fails on
  the registry. That the fonts are `core`'s makes it
  *right*; that the CLI cannot see them makes it *forced*. **An earlier draft justified the
  width by pointing at `mpdf-003` OQ-8 and round 1 read that OQ**: it asks what it takes to
  run the app on another machine — toolchain, fonts, signing — and says nothing about licence
  text, so the citation is withdrawn rather than repaired. Any embedder still gets the
  const; that is a consequence, not the reason.

  **`cli/src/main.rs:Args` gains `--licenses`**, and `cli/src/main.rs:run` gains an arm that
  prints and returns before it reads a file, on `--emit-typst`'s exact shape.

  **`input` becomes `Option<PathBuf>` *and* carries `#[arg(required_unless_present =
  "licenses")]`. Both, not either.** A draft of this bullet prescribed the attribute alone
  and forbade the `Option`, and **all three round-1 lenses independently built that shape
  against this workspace's clap 4.6.6 and measured it failing**: the derive infers
  `required(true)` from a non-`Option` field, which collides with the attribute, so a
  **debug** build — the one `cargo test` runs — panics on every invocation with *"Argument
  input: `required` conflicts with `required_unless*`"*, exit 101, and a **release** build
  lets `required` win and refuses `--licenses` with exit 2. Adding `required = false` does
  not rescue it, because `clap_derive` generates `ok_or_else(… MissingRequiredArgument …)`
  for any non-`Option` field and the absent value still errors.

  **The rejected rationale is recorded rather than quietly replaced**, because it was wrong
  in its shape and not only in its answer: it framed the two as alternatives trading clap's
  own bare-invocation message against a message this phase would have to author. They are
  complements. `Option` is what lets the field hold "absent"; `required_unless_present` is
  what keeps clap enforcing the requirement and printing its own message when the flag is
  missing — measured, bare `md2pdf` still exits 2 on clap's own text. **So the "acquire no
  message" goal survives intact and attaches to the attribute**, and nothing about it ever
  argued against the `Option`.

  **What it prints, pinned exactly, because gate (2) calls itself a byte comparison.**
  Four parts, joined by `"\n\n"`:

  1. `include_str!` of `cli/LICENSE`;
  2. and 3. one part per `FONT_LICENSES` entry, each rendered `"{filename}\n{text}"`;
  4. `include_str!` of `cli/THIRD-PARTY-LICENSES.md`.

  A draft left this as "under its filename" while the gate said "the four files
  concatenated", which round 1 caught from two lenses as two different byte streams — and
  the trap is specific: the implementer writes both the arm and the test, so the gate would
  have pinned whatever was guessed rather than what was decided.

  **Both `cli/` sources are the symlinks the licence fix of 2026-09-02 put there** —
  `db69839`, a code-only fix commit and **not** `mpdf-011` Phase 3's close-out, which a draft
  misattributed and which its own step 0 above already contradicts. Cargo resolves a symlink
  into the archive as an ordinary file, at 1264 and 37041 bytes.

  **Where to see that, because two rounds were spent on the provenance.** In
  `md2pdf-core-0.1.1.crate`, which sits in the local registry cache, or in
  `md2pdf-cli-0.1.1.crate` fetched from `static.crates.io`; both were published after the
  symlinks landed. **The trap is `target/package/md2pdf-cli-0.1.0.crate`**, which does hold
  both files and is where the original measurement came from — it is a *local repackage*
  carrying the version number of a published archive that holds neither, since 0.1.0 went
  out before the fix. A draft cited `0.1.0` and round 1 called it wrong; round 2 found the
  correction naming a `0.1.1` archive not present on the machine. Both were pointing at the
  same true mechanism through an artifact that could not be opened, so the route is written
  out rather than the version number simply being edited again.

  **Nothing is generated at run time and nothing is read from disk**, so there is no path on
  which the flag can fail. Four `include_str!`s make the text a compile-time constant.

  **`core/src/emit.rs`, the three `.typ` files and every golden are untouched.**

- **Exit gate:** seven cases. **Round 1 added two of them** — one for the `core` half, which
  no clause read, and one for the phase's own title claim.

  (1) **The flag runs without a document and the document runs without the flag.**
  `md2pdf --licenses` exits 0 with no positional argument; `md2pdf --licenses extra.md`
  exits 0 and ignores it, as `--emit-typst` ignores `-o`; a bare `md2pdf` still exits
  non-zero from clap's own message; and `cli/tests/cli_test.rs:emit_typst_prints_the_golden_file`
  and `cli/tests/cli_test.rs:the_o_flag_writes_a_pdf` both still pass. **The `extra.md`
  clause depends on that path not existing** — no such file is in the tree today, and
  against one that exists the clause proves only that parsing succeeded rather than that the
  arm returned before reading. Named so a later implementer does not helpfully create it.

  (2) **What it prints is what the repository holds, byte for byte.** One test asserts
  stdout equals the four parts assembled as the scope pins them, each read from disk by the
  test rather than compiled in — so a reordering, a truncation, a dropped section or a
  missing filename line fails.

  (3) **The text a reader needs is actually in it**, which case (2) cannot say, since two
  wrong files assembled correctly pass it. Five literals must appear in stdout:
  `MIT License`, `SIL OPEN FONT LICENSE`, `Apache License`, `` | `typst` | 0.15.1 |
  Apache-2.0 | `` and **`of the GUST Font License`**. The fifth was added in round 1: the
  other four discriminate three of the four files, and the maths font's licence — the second
  of `FONT_LICENSES`' two entries — was named by no clause, so a list with both entries
  pointing at `OFL.txt` and a test mirroring it passed everything. Verified unique: one hit
  in `core/assets/fonts/GUST-FONT-LICENSE.txt`, zero in the other three.

  (4) **`core` carries the const, and the CLI reads it *through* the const.** Two
  instruments, because round 2 found the first does not reach the second half of that
  sentence. A test in `core` asserts `FONT_LICENSES` has exactly two entries keyed
  `"OFL.txt"` and `"GUST-FONT-LICENSE.txt"`, each non-empty. **And `cargo package -p
  md2pdf-cli` succeeds**, which builds the CLI from its own archive and so fails any
  `include_str!` reaching into `core/assets/` — a path that resolves in a checkout and does
  not exist on the registry.

  **Round 1's gate lens found that cases (1)–(3) and (5)–(7) read only `cli`**, so an
  implementation putting all four `include_str!`s in `cli/src/main.rs` — the one shape the
  scope forbids — passed the whole gate unchanged. **Round 2 then found the residue**: the
  `core` test alone admits an implementation that writes the const *and* separately reaches
  into `core/assets/` from `cli`, which passes in a checkout and breaks for every consumer.
  The packaging run is what closes it, and it closes case (5)'s stated limit in the same
  stroke.

  (5) **The binary carries the text away from the repository.** `target/release/md2pdf`
  copied to a scratch directory outside the tree, run there as `--licenses` with the working
  directory elsewhere, produces the same stdout as case (2). Round 1 added this: every other
  clause runs inside the workspace, so an implementation reading `cli/LICENSE` at run time
  passed them all while breaking the sentence this phase is named for. **Its limit, named
  rather than left to be found:** it catches a *cwd-relative* read, which is the realistic
  wrong shape, and not a `concat!(env!("CARGO_MANIFEST_DIR"), …)` one, since the repository
  still exists on the machine. Case (4)'s packaging run is what reaches that.

  (6) **The page does not move.** `target/release/md2pdf` over `samples/showcase/showcase.md`,
  hashed with `shasum -a 256` before this phase's commit and after, identical, **`-o` into a
  scratch path** since `cli/src/main.rs:default_output` writes beside its input. One document
  is enough here, unlike `mpdf-011` Phase 2's three, because this phase adds a branch that
  returns before the pipeline is entered rather than editing anything inside it.

  (7) **`cargo test --workspace` passes and `spec-lint` exits zero with no error** — run as
  `/Users/ivapo/dev/main/spec-driven-dev/bin/spec-lint .`, the path given because the tool is
  on no `PATH` here and Phase 12's gate inherited the same omission. Its warnings are the
  four kinds `mpdf-011` Phase 2's gate 5 names and are inherited; today it reads 0 errors and
  62 warnings, every one `CIT_UNRESOLVED_PATH`.

  **This phase cites `core/src/lib.rs` and `cli/src/main.rs` as files wherever it means
  something it creates**, never as `file:symbol` — and the const's name is given in a code
  block above rather than in a citation for exactly that reason. A draft of this clause did
  cite the symbol and argued that the resulting `CIT_SYMBOL_ABSENT` was "correct rather than
  a finding" before the code landed. **That argument is wrong and the draft was measured
  failing**: this document is append-only and accepted, so the error would stand in the
  corpus from the moment the phase is written until the moment it ships, failing this same
  clause for every *other* phase in the meantime and failing `/review-spec`'s own run on the
  round that is supposed to judge it. Phase 12 wrote the rule from the other direction, for a
  symbol it deletes; it holds identically for one not yet created.

  **What this gate deliberately does not assert: that `THIRD-PARTY-LICENSES.md` is current.**
  It is generated by `tools/third-party-licenses.py` from the resolve graph, and regenerating
  it needs the registry sources unpacked on the machine — which a CI runner that has only
  fetched crates does not have. So the committed file could in principle lag a dependency
  change, and no clause here would see it. **Named rather than papered over**, and the honest
  bound is that it is a superset by construction: the generator walks the resolve rather than
  an enabled feature set, so it over-lists today by **eight crate names, or nine name-version
  pairs** — round 1 re-derived both and the two figures differ because `vello_common` appears
  at two versions, of which `cargo tree -e normal` reaches one. A file that over-lists is the
  safe direction for the failure this phase is about. **OQ-15 holds the three answers that
  would actually close it**, and this phase takes none of them, because every clause above
  reads the committed file whatever that file says.

- **Close-out.** **Three passages in `rules/pipeline.md`, not two** — round 1 found the third
  and it is the one that goes *false* rather than merely incomplete:

  1. `## The CLI`'s command-grammar line `md2pdf input.md [-o output.pdf] [--emit-typst]`,
     which gains the flag and the condition on `input`.
  2. `## The CLI`'s licence-artifacts paragraph, which after this phase names a fourth reader
     of the same four files.
  3. **The API paragraph's fourth sentence — `core/src/emit.rs:IMAGE_EXTENSIONS` "is the
     crate's one non-function export" — which `pub const FONT_LICENSES` makes untrue.**
     (Quoted verbatim, so the target is unambiguous; a draft called it the paragraph's
     opening sentence, which opens on `md_to_typst` instead.) This
     is the site that must carry the citation of the new const in `core/src/lib.rs` that
     the scope's naming argument promises — **written here as a file and a name rather than
     as a `file:symbol` citation, for the reason the gate gives two screens up**, and a
     draft of this list named neither the sentence nor any home for it. The fold that added
     this paragraph spelled it as a citation and `spec-lint` reported `CIT_SYMBOL_ABSENT`
     on the spot: *"a fix can introduce a blocker"* — `loops/review-spec.md`, in the section
     implementing §7.3 — caught by the phase's own rule inside the pass that wrote it. `rules/` is the artifact that must track the code, so following
     that draft would have shipped it asserting something false. **`core/src/lib.rs`'s own
     doc comment survives untouched**: it says `IMAGE_EXTENSIONS` is "the crate's first
     re-export", and `FONT_LICENSES` is not a re-export.

  **`rules/pipeline.md` is at 1259 of `max_lines: 1265`** — six lines of headroom,
  re-derived three ways in round 1 after one lens read 1260; `spec-lint` prints `1259/1265`
  and that is authoritative. Three passages against six lines is tight, so **re-derive it
  rather than trusting this number, and move the cap with the edit** rather than shaving the
  prose, as the two phases before this one did.

  `README.md`, two sites: `## Licence`, one sentence — it names the shipped files and after
  this phase the binary can print them, which is the half a reader holding only a binary
  needs told; and **`## Use`**, which shows the three invocations and would silently become
  incomplete.

  **`CLAUDE.md`: none needed** — it carries the workflow stanza and the observable sentence,
  and this phase moves neither. **Status artifact: none needed** — `.spec-lint.yaml` has
  `status_artifacts: {}`, so none exists. Both stated because §3's reconciliation step asks
  for them by name and a draft was silent on both.

  **`core/src/lib.rs`'s module doc is deliberately not touched.** It describes the pipeline,
  and `FONT_LICENSES` is beside the pipeline rather than in it — the same call `mpdf-011`
  Phase 2 made when it left the four `app/` doc comments standing.

  **`specs/INDEX.md` and `rules/INDEX.md` regenerated**, never hand-edited — this spec's
  rollup goes `done` → `partial` as this phase is appended and back to `done` when it lands.
  One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-001.md, append-only, one heading per round. See §7 of the
methodology.
-->
