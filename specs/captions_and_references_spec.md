---
id: mpdf-005
title: captions-and-references
note: >
  Figures, tables, listings and equations gain captions, numbers and
  cross-references: the emitter wraps them in Typst's `figure`, the looks decide
  what a caption and a number look like, and a reference that names one stays
  true when another is inserted above it.
status: draft
last_updated: 2026-08-15

phases:
  - name: "Phase 1 — a captioned figure"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — tables and listings take the same treatment"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — labels and cross-references"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-002, mpdf-004]
reference: >
  Pandoc's `implicit_figures` extension and the `pandoc-crossref` filter are the
  inspiration for the shape, not for the syntax. `implicit_figures` promotes a
  standalone image's alt text to a caption, which `mpdf-002` §1.1 refused and
  this spec does not revisit — alt text is accessibility metadata. Pandoc itself
  stays excluded permanently by `mpdf-001` §1.1. **Typst's `figure` element is
  the mechanism**, and §2 records what it supplies for free and what it does not.
---

# captions-and-references

## 1. Goal

Let a document caption the things it shows, number them, and refer to them by a
name that survives editing. **The observable is unchanged — the typeset PDF that
Typst compiles from the user's markdown — and what widens is what that PDF can
say about its own contents.**

The consumer is the same author. Today they can write this:

```markdown
![The three steps, drawn as boxes](pipeline.svg)

As the diagram above shows, the emitter sits in the middle.
```

and "the diagram above" is the only way to point at it. If a second diagram is
inserted, or the figure floats to the next column, the sentence is quietly
wrong. After this spec the author writes a caption and a name, and points at it:

```markdown
![The three steps, drawn as boxes](pipeline.svg)
: The conversion pipeline. {#fig:pipeline}

As [](#fig:pipeline) shows, the emitter sits in the middle.
```

and the PDF reads *"As Figure 1 shows…"*, with the caption set beneath the
image, both renumbered by Typst whenever anything moves.

**The caption syntax above is illustrative, not decided.** It is the central
design call of this spec and OQ-1 carries it; §2 fixes the mechanism, the seam
and the enforcement, and deliberately does not fix the spelling before a review
round has argued it.

### 1.1 Why this is a new spec and not a phase of an existing one

The methodology's §6.1 is an ordered test, and it is worked in full rather than
stopped at the first step that lands.

- **Step 0 — does this change a decision?** Yes, three. `mpdf-002` §1.1 resolved
  "no captions and no figure numbering". `mpdf-004` §1.2 resolved "no labels, no
  cross-references", which its Phase 3 already split once. And `mpdf-001` §1.1
  parked the subject at the founding.
- **Step 1 — does it remove or contradict shipped work?** **Not as scoped, and
  this is the step that shapes the spec rather than the step that disposes of
  it.** Every one of those three resolutions names its own replacement:
  `mpdf-002` says "a later spec may adopt Typst's `figure` properly" and left the
  hook by name; `mpdf-004`'s OQ-7 says referencing "is a phase to append if the
  answer is yes"; `mpdf-001` chose Typst partly because "it has native math,
  citations, cross-references, and numbering, so later specs do not have to build
  those". Work that names its own successor is not work this removes.

  **One live hazard remains at this step and §2 is built around avoiding it.**
  `mpdf-004` Phase 3 shipped the frontmatter key `equations`, on 2026-08-15. A
  design here that replaced it would contradict shipped work, which step 1 says
  is never a phase and would force `supersedes: [{id: mpdf-004, phases: [...]}]`
  with a `cut` on a phase that shipped the day before. OQ-2 carries the choice,
  and §2's default is additive precisely so this edge stays `null`.
- **Step 2 — is the subject one an existing spec owns?** **No, and this is the
  reason the answer is a new document.** The subject spans four constructs owned
  by three specs: images by `mpdf-002`, tables and code blocks by `mpdf-001`,
  equations by `mpdf-004`. §6.1's closing rule is written for exactly this: "A
  cross-cutting feature still gets its own spec. If the work spans several
  subsystems and its unifying thread is a *goal* rather than a subject … no
  subject spec has standing to remove what another one shipped."
- **Step 3 — a named kind under a reserved framework?** No. `mpdf-002` §1.1 and
  `mpdf-001` §1.1 are non-goals lists, not §2 frameworks reserving named kinds —
  the same reading `mpdf-004` §1.1 worked and the corpus has now honoured three
  times. So **`extends` stays `null`** and `related` carries the links.
- **Step 4** is therefore the landing: a new spec, `extends: null`.

### 1.2 Non-goals

- **Not citations, and not a bibliography.** This is the boundary a reader will
  most want argued, so §2 argues it rather than asserting it. Parked by
  `mpdf-001` §1.1 and left parked here.
- **Not references to other markdown files.** `mpdf-001`'s observable is "one
  markdown file plus the images it names in, single PDF out", and a document that
  points into a second document is a project model rather than a document model.
  That is a larger subject and not this one.
- **No numbering scheme beyond one counter per kind.** Not `1.1` per section, not
  per-chapter restarts, not `A.1` appendices. Typst supports all three through
  `figure`'s own `numbering` argument, so each is a later phase or a later spec
  and none needs a new mechanism here.
- **No list of figures and no table of contents.** Typst's `outline` produces
  both from the same elements this spec starts emitting, which makes them cheap
  later and is a reason to leave them out now rather than a reason to include
  them: neither is a *reference*, and §5's scope discipline refuses the
  pre-abstraction.
- **No caption on a construct the dialect does not carry.** Block quotes, lists
  and headings get none. Headings are already numbered by Typst if a look asks,
  and both bundled looks currently decline.
- **No change to what alt text means.** It stays `image`'s `alt:` argument and
  accessibility metadata, per `mpdf-002` §1.1. A caption is a separate,
  author-visible string, and §2 records why the two are not merged.

## 2. Design

### What Typst supplies for free, measured rather than assumed (decision, recorded)

Measured 2026-08-15 against the Typst 0.15.1 that `core/Cargo.toml` pins, through
`core/src/lib.rs:TypstWorld` and this repo's own `core/assets/template.typ`, with
`mpdf-004` Phase 3's numbering rule active. The probe source wrapped one image,
one table and one raw block in `figure` with captions, labelled all three plus a
display equation, and referenced all four:

| written | typeset |
|---|---|
| `#figure(image("dot.png"), caption: [A diagram])` | **Figure 1: A diagram** |
| `#figure(table(...), caption: [A table])` | **Table 1: A table** |
| `#figure(raw(..., lang: "rust"), caption: [Some code])` | **Listing 1: Some code** |
| `$ a^2 + b^2 = c^2 $ <eq:one>` | the equation, then **(1)** |
| `@fig:one @tab:one @lst:one @eq:one` | **Figure 1  Table 1  Listing 1  Equation 1** |

**Four independent counters, four supplements, and the kind inferred from the
body — none of it configured.** `figure` numbers by default; nothing in the probe
turned numbering on. That single fact reshapes the phase order below, because for
these three constructs a caption and a number arrive together and it is
*suppressing* the number that costs work, which is the opposite of the equation
case `mpdf-004` Phase 3 shipped.

**Two asymmetries are recorded because they are what an implementer trips on.**

*The equation reads `(1)` on the page and `Equation 1` in a reference.* The
caption-side format comes from `math.equation`'s `numbering`, which `mpdf-004`
Phase 3 put in each look; the reference-side supplement comes from `ref`. They
are two settings and a look that changes one has not changed the other. Whether
that is a defect to fix or a convention to keep is OQ-4.

*An unresolved reference is a compile error, not a silent drop.* Measured the
same day: `@nosuchthing` fails with ``label `<nosuchthing>` does not exist in the
document``. **That is the good half and the bad half at once.** Good, because the
`mpdf-001` §2 faithfulness failure — a reference that vanishes — cannot happen,
and `core` therefore needs no symbol table of its own to be *correct*. Bad,
because the message carries no line number and names a Typst label rather than
what the author typed, which is precisely the gap `mpdf-004`'s OQ-3 was resolved
to close. §2's scan rule below is the answer.

### Nothing is wrapped today, and `mpdf-002` left the hook (decision, recorded)

Measured by grep on 2026-08-15: **`figure` appears nowhere** — not in
`core/src/emit.rs`, not in `core/assets/template.typ` or
`core/assets/press-release.typ`, and not in any of the 18 shipped golden files.
The three constructs reach Typst bare:

- `core/src/emit.rs:image_call` writes `image(path, alt: …)`, and
  `core/src/emit.rs:write_image` decides standalone against inline.
- `core/src/emit.rs:table_call` writes `table(…)`.
- `core/src/emit.rs:step` writes `#raw(block: true, …)` for a fenced block.

**That bare standalone `#image` is a designed extension point, not an
accident.** `mpdf-002` chose it over `box(image(…))` in order to give "a later
figure treatment its hook: a bare `#image` in its own paragraph is what a future
`show` rule or `figure` wrapper can address, and a box is not." This spec is the
later figure treatment, and it spends that hook.

**Only the standalone form is wrapped.** An inline image inside a sentence stays
a `box(image(…))` and takes no caption, no number and no label — a figure is a
block that floats, and one inside a clause is not a figure. `write_image` already
tells the two apart, so this needs no new state.

### Why the caption is the author's and the format is the look's (decision, recorded)

This is `mpdf-004` Phase 3's seam, reused deliberately rather than reinvented:
**the author supplies the words and asks for the treatment; the look decides what
it looks like.** The author writes the caption text and the name; the look owns
the supplement, the type, whether the caption sits above or below, the separator,
and the numbering format. So `core/src/emit.rs` writes no `"Figure"`, no `":"`
and no `"1"` — the rule `mpdf-001` §2 set when it kept the emitter out of the
table header's boldness, applied one construct further along.

What that costs is that both bundled looks gain a `show figure` rule and a
`show figure.caption` rule of their own, and the two may disagree — which is the
point, not a defect. Whether the *look contract* widens again is OQ-3, and the
answer is not obviously yes: a caption needs no argument crossing the seam, only
a `show` rule over an element the emitter emits, which is how both looks already
reach `raw` and `table.cell` without an export.

### Why the check is on what the author wrote (decision, recorded)

`mpdf-004` §2 settled this shape once and it applies unchanged: **a construct
outside the dialect is a named error at the point the author wrote it, never a
silent drop and never a Typst identifier they have never seen.**

Two rules follow, and both exist because of the measurement above.

- **A label is scanned, not passed through.** A name must be a closed character
  set, checked in `core` with the author's line, because Typst's own failure
  names `<a-name>` with no line. The set and its shape are OQ-1's business, but
  *that* it is checked here is fixed.
- **A reference to a name the document does not declare is refused by `core`,
  before Typst sees it.** This is the one place `core` does need to collect
  labels — not to make references work, which Typst does, but to make the *error*
  name the author's own line. That is a single pass over the declared names, not
  a symbol table with scoping rules, and `core/src/emit.rs:collect_definitions`
  already establishes the precedent of a pre-pass that gathers names before the
  main walk.

### Why citations are not in this spec (decision, recorded)

Raised directly during drafting, and recorded rather than left implicit, because
the two subjects look adjacent and are not.

**They share a shape and nothing else.** Both give a thing an identity and point
at it, and Typst supplies both natively — `cite` and `bibliography` beside `ref`
and `label`. That is the whole of the overlap.

The differences are structural, and each one lands somewhere different:

- **Direction.** A cross-reference points *inward*, at content the same document
  already contains and the same compile already has. A citation points *outward*,
  at a record that is not in the document at all.
- **A new channel, in three crates.** A `.bib` is a new asset kind.
  `core/src/lib.rs:Asset` carries one image file each, `core/src/lib.rs:image_paths`
  is the shopping list the CLI reads from disk, and `mpdf-003`'s watch loop
  watches the document and the files that list names. A bibliography would have to
  join all three. **Everything in this spec touches `core` alone** — the claim
  `mpdf-003` §2 makes pointed the other way, which `mpdf-004`'s phases each
  checked as a diff and this spec's phases will too.
- **A different observable.** Numbering and captions change the typography of
  content the document already has. A bibliography *adds a section* that is not in
  the markdown anywhere, which is a larger promise.
- **Scope discipline.** §5 refuses pre-abstraction before there are real
  consumers. Folding a second, larger subject into a spec that has shipped none of
  its own phases is that failure exactly.

**One thing is owed to the citation spec that does not yet exist, and it is
cheap: the addressing convention.** If this spec spells a name one way and a
citation spec spells it another, the dialect grows two ways to point at
something. So OQ-1 must choose a syntax that a citation spec could extend rather
than contradict, and OQ-5 asks whether "referencing" is a framework with two
kinds — which decides whether that later spec carries `extends: mpdf-005` or
`related` alone. That question is asked here and answered there; it is not a
reason to build the second subject now.

### Why this changes nothing in `cli` or `app` (decision, recorded)

Neither wrapper reads the dialect. `cli/src/main.rs` and `app/src/document.rs`
both hand a markdown string and an asset list to `md2pdf_core::md_to_pdf`, so a
captioned, numbered, cross-referenced document converts through both the moment
`core` supports it. Each phase checks it as a diff, as `mpdf-004`'s three did.

## 3. Open questions

- **OQ-1 — what is the syntax for a caption and for a name?** **The central
  design call, and the one thing markdown gives no help with.** Alt text is the
  only caption-shaped field in the whole of markdown and `mpdf-002` §1.1 refused
  to repurpose it, on the ground that alt is accessibility metadata — a reason
  this spec accepts and its §1.2 restates. Tables, code blocks and equations have
  nothing at all. The shapes available:

  a **Pandoc-style attributes** — `{#fig:pipeline}` after the construct, with the
  caption on an adjacent line. Generalises to all four constructs uniformly, is
  familiar to anyone who has used `pandoc-crossref`, and is unambiguously new
  syntax rather than a reinterpretation of old.

  b **A marked adjacent paragraph** — a line beginning `: ` beneath the
  construct, as the §1 sketch draws it. Less invented-looking, but `: ` at the
  start of a line is a definition-list marker in several dialects, so it claims
  syntax a later phase might want.

  c **Markdown's own link syntax for the reference half** — `[](#fig:pipeline)`.
  Attractive because it invents nothing, but `mpdf-001` shipped links that keep a
  `#` fragment intact in the destination, so this **reinterprets shipped
  behaviour** and needs §6.1 step 1 worked before it can be a phase rather than a
  supersession. It may survive that test if it is scoped to a fragment matching a
  name the same document declares — a document with no names then behaves
  exactly as it does today — but that is a judgement for a review round, not an
  assumption for a draft.

  Design call. **Blocks Phase 1**, and everything downstream of it.

- **OQ-2 — does `equations: numbered` generalise, and how?** `mpdf-004` Phase 3
  shipped that key on 2026-08-15, and its name presumes equations are a special
  case. Three shapes: **sibling keys** — `figures:`, `tables:`, `listings:` —
  which is purely additive, needs no supersession, and keeps every document
  written today working, at the cost of four keys where one concept exists; a
  single `numbering:` key taking a list, which is tidier and **contradicts a
  shipped key**, forcing `supersedes` and a `cut` on a phase that shipped the day
  before; or both, which is two ways to say one thing and the worst of the three.
  **The draft's default is sibling keys**, which is why `supersedes` is `null` in
  the frontmatter. Design call. Blocks Phase 2.

- **OQ-3 — does the look contract widen a sixth time?** `mpdf-001` Phase 9 fixed
  it at four named arguments and `mpdf-004` Phase 3 moved it to five, at the cost
  of every shipped golden file. A caption may need nothing — a `show figure`
  rule reaches the element without an export, the way both looks already reach
  `raw` and `table.cell`. But OQ-2's answer may push one or three more arguments
  across. Answerable from code during review, once OQ-2 lands. Blocks Phase 1.

- **OQ-4 — should a reference to an equation read `(1)` or `Equation 1`?** §2
  measured that the page and the reference disagree today, because the format
  lives in `math.equation` and the supplement lives in `ref`. Typst's
  `set ref(supplement: none)` is one candidate and a `show ref` rule keyed to the
  element is another; both are look decisions by §2's seam, which means the
  answer may be "the two looks decide, and they may differ". Answerable from code
  during review. Blocks Phase 3.

- **OQ-5 — is "referencing" a framework with two kinds, or two subjects?** If
  internal cross-references and external citations are two kinds under one
  framework, a citation spec later carries `extends: mpdf-005` and this spec must
  reserve the namespace per §2 — which also means renaming this file before it is
  accepted. If they are two subjects, `related` is the whole edge and nothing is
  reserved. **This is asked now because §2's namespace reservation cannot be
  applied retroactively without renaming an accepted spec.** Design call. Blocks
  nothing in the code; blocks `status: accepted`.

- **OQ-6 — does a `figure` wrapper move a shipped golden file, and how many?**
  Every fixture carrying a standalone image or a table changes shape, which is
  the second-largest golden movement the project would have taken after
  `mpdf-004` Phase 3's seventeen. Answerable from code during review by building
  the minimal change and counting, which is the method `mpdf-004` Phase 3's round
  1 used to correct a number the author had asserted. Blocks Phase 1's gate,
  which cannot say "no shipped golden changes" or "these change" until it is
  counted.

- **OQ-7 — what does a caption inside a footnote definition do?** Footnote
  bodies are walked twice by `core/src/emit.rs:collect_definitions` and the first
  walk is discarded, which is why `mpdf-003` Phase 6 had to withdraw its heading
  anchors when the counts disagreed. A label declared in a discarded walk is the
  same hazard. Answerable from code during review. Blocks Phase 3.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. All three produce the
observable. **The order follows §2's measurement** — a caption and a number
arrive together for these constructs, so they are not split into separate phases
the way a reader coming from `mpdf-004` Phase 3 would expect.

### Phase 1 — a captioned figure
*Produces the observable: yes — a PDF with a captioned, numbered figure under an
image, which is what a document that shows something needs in order to talk
about it.*

- **Scope:** One construct only — the standalone image — so that OQ-1's syntax is
  tested by a real consumer before three more constructs are committed to it.
  `core/src/emit.rs:write_image` wraps the standalone form in `figure`; the inline
  form is untouched. Both bundled looks gain a `show figure` rule. The caption
  scan and its named error land here, with the author's line.
- **Exit gate:** to be written when OQ-1 and OQ-3 resolve. It must include the
  golden movement OQ-6 counts, a refusal naming a malformed caption and its line,
  and one PDF read by eye per look — `mpdf-001` Phase 9's answer for anything
  whose observable lives inside a look, which `mpdf-004` Phase 3 had to reach for
  again.

### Phase 2 — tables and listings take the same treatment
*Produces the observable: yes — a PDF whose tables and code blocks carry the same
captions and their own counters.*

- **Scope:** `core/src/emit.rs:table_call` and the fenced-block arm of
  `core/src/emit.rs:step` take Phase 1's wrapper. No new syntax: if this phase
  needs any, Phase 1 chose wrongly. OQ-2's frontmatter answer lands here, because
  this is the phase that first has more than one counter to switch.
- **Exit gate:** to be written.

### Phase 3 — labels and cross-references
*Produces the observable: yes — a PDF whose "as Figure 1 shows" is still true
after a figure is inserted above it, which is the whole reason to number
anything.*

- **Scope:** Names become Typst labels on all four kinds, equations included, and
  a reference becomes `@name`. `core` collects declared names in a pre-pass so a
  reference to an undeclared one is refused with the author's line rather than
  Typst's labelless message. OQ-4's equation-supplement answer lands here, and
  OQ-7's footnote hazard is settled before, not during.
- **Exit gate:** to be written. It must include a document whose reference stays
  correct across an insertion — the property the phase exists for, and one no
  golden file can see.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-005.md, append-only, one heading per round. See §7 of the
methodology.
-->
