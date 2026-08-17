---
id: mpdf-005
title: captions-and-references
note: >
  Figures, tables, listings and equations gain captions, numbers and
  cross-references: the emitter wraps them in Typst's `figure`, the looks decide
  what a caption and a number look like, and a reference that names one stays
  true when another is inserted above it.
status: accepted
last_updated: 2026-08-15

phases:
  - name: "Phase 1 — a captioned figure"
    reviewed: 2026-08-15
    shipped: 2026-08-17
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

**The caption half of that sketch is decided** — §2 fixes the `: ` line and the
`{#name}` that may ride it, and OQ-1 records why. **The reference half,
`[](#fig:pipeline)`, is not**: it is the one spelling that reinterprets a
construct `mpdf-001` shipped, and OQ-8 carries it. Phase 1 needs only the
caption, so nothing downstream of that question blocks it.

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

Measured by grep on 2026-08-15 and re-derived by round 1: **the Typst element
`figure` appears nowhere** — not in `core/src/emit.rs`, not in
`core/assets/template.typ` or `core/assets/press-release.typ`, and not in any of
the 18 shipped golden files. (The *word* occurs twice, in a comment about the
Windows path `C:\figure.png` and in `core/src/lib.rs`; the claim is about the
element.) The three constructs reach Typst bare:

- `core/src/emit.rs:image_call` writes `image(path, alt: …)` **without a leading
  `#`**, which `core/src/emit.rs:write_image` adds for the standalone form and
  omits inside a `box` for the inline one.
- `core/src/emit.rs:table_call` writes `#table(…)` — **with** the `#`, unlike
  `image_call`. Recorded because it is the asymmetry Phase 2 trips on: that inner
  `#` has to be dropped before the call can sit inside a `#figure(…)`.
- `core/src/emit.rs:step` writes `#raw(block: true, …)` for a fenced block.

**That bare standalone `#image` is a designed extension point, not an
accident.** `mpdf-002` chose it over `box(image(…))` in order to give "a later
figure treatment its hook: a bare `#image` in its own paragraph is what a future
`show` rule or `figure` wrapper can address, and a box is not." This spec is the
later figure treatment, and it spends that hook.

**Only the standalone form is wrapped.** An inline image inside a sentence stays
a `box(image(…))` and takes no caption, no number and no label — a figure is a
block that floats, and one inside a clause is not a figure.
`core/src/emit.rs:step`'s standalone test already tells the two apart, **and
neither it nor the flush that reads it changes** — an inline image is never
recorded as a splice point, so it can never take a caption. The one thing added
is the record itself, which §2's attachment subsection states exactly.

### The caption is a `: ` line, and it is what makes a figure (decision, recorded)

**A caption is a paragraph of its own, immediately after the construct, opening
`: `.** A name may ride the end of that line as `{#name}`, which Phase 3
implements and Phase 1 refuses as it refuses anything else it does not yet
accept.

```markdown
![The three steps, drawn as boxes](pipeline.svg)

: The conversion pipeline, with the *emitter* in the middle.
```

**The blank line is required, and round 2 is why.** A draft of this section put
the caption on the very next line, which reads better and is what an author
reaches for. Measured against `pulldown-cmark` 0.13.4 under
`core/src/emit.rs:options`: that form is **one paragraph**, not two — the events
are `Start(Paragraph)`, the image, `SoftBreak`, `Text(": …")`, `End(Paragraph)`.
Run through today's emitter it converts to `#box(image(…))` — the *inline* form —
followed by a literal `: The conversion pipeline…`, because
`core/src/emit.rs:step` tests standalone-ness by whether `End(TagEnd::Paragraph)`
is the next event and a `SoftBreak` is not it.

So the prettier spelling is not a syntax choice, it is a change to the
standalone test — **the one discrimination every image in the dialect flows
through**, and the blast radius this phase is scoped to avoid. The blank-line
form leaves that test untouched, which is what keeps gate (2)'s "an uncaptioned
image is byte-for-byte unchanged" provable rather than hopeful. Recorded because
the rejected spelling is the one a later reader will propose.

Chosen over the two alternatives round 1 was asked to weigh. **Alt text is not
reused**: `mpdf-002` §1.1 refused it because alt is accessibility metadata, and
§1.2 keeps that. **A fenced-div wrapper** (`::: figure … :::`) is unambiguous and
generalises, and it is more syntax than the job needs — a caption is one line of
prose, and a three-line wrapper around a one-line construct reads as ceremony.

`: ` is the marker because **it costs nothing in this dialect**. It is Pandoc's
own table-caption spelling, so it is the closest thing to a convention; GFM gives
it no meaning, since definition lists are not in GFM and `pulldown-cmark` with
`core/src/emit.rs:options` parses such a line as an ordinary paragraph. Censused
2026-08-15 across `tests/fixtures/` and `samples/`: **no line anywhere in the
corpus begins with `: `**, which is the same instrument `mpdf-001` Phase 8 used
before claiming a marker.

**A `: ` line is a caption only where a caption can attach**, which is
immediately after a standalone image (Phase 1), a table, a fenced code block
(Phase 2) or a display equation (Phase 3). Everywhere else it stays the ordinary
paragraph it is today. That is what keeps the marker from being a ban: a document
that opens a paragraph with `: ` somewhere else is untouched, and the collision
window is one paragraph in one position, which the census finds empty.

### Why an uncaptioned construct is not wrapped (decision, recorded)

**A standalone image with no caption line stays exactly the bare `#image(…)` it
is today.** Round 1 measured both halves of the alternative, and each is a
failure this project has a rule against:

- An uncaptioned `#figure(image(…))` **prints no number and still consumes the
  counter**, so a captioned figure after an uncaptioned one reads *"Figure 2"*
  with no Figure 1 anywhere on the page.
- `figure` centres its body where a bare block sits flush left — measured as a
  bounding box moving from `xMin=70.87` to `277.48`, plus `figure`'s own gap.

Wrapping unconditionally therefore silently re-lays-out and mis-numbers every
existing document that shows an image. **`mpdf-004` Phase 3 stated the property
this violates** — "no document's typeset output changes unless its author asks" —
and this spec inherits it whole.

So the caption is not decoration on a figure; **the caption is what makes it a
figure.** One rule, and three things fall out of it: no shipped golden file
changes in Phase 1 (OQ-6), `cli/tests/cli_test.rs:emit_typst_reads_no_image`
keeps passing over its uncaptioned `tests/fixtures/figure.md`, and an author who
wants a number gets one by writing the caption that a numbered figure wants
anyway.

### What a caption may contain, and what is refused (decision, recorded)

**The caption's text is walked as inline markdown, not escaped as literal text.**
`*emphasis*`, `` `code` ``, a link and a `$…$` span all work inside one, because
a caption is prose and a caption that alone in the document could not carry
markdown would be the surprise. This needs no new mechanism: the caption's
content is a run of inline events, and `core/src/emit.rs` already walks such a
run into a buffer of its own for a list item, a block quote and a table cell —
the `bufs` stack is the mechanism, and the caption is one more frame on it. (A
footnote definition is *not* a fourth example: it gets a whole separate `Walk` in
`core/src/emit.rs:collect_definitions`, which is a different mechanism and the
one OQ-7 is about.)

Two refusals, each naming the author's line, per `mpdf-004` §2's rule that the
error names what the author typed:

- **A `: ` line carrying no text.** A caption marker with no caption is a
  mistake, and emitting an empty `caption: []` would put a bare *"Figure 1:"* on
  the page.
- **A second `: ` line** immediately after the first. One construct takes one
  caption; the second line is either a mistake or a paragraph the author did not
  mean to mark, and both are better named than guessed.

  **This one does not fall out of §2's three spend conditions and needs state of
  its own**, which is recorded because an implementer will otherwise get silence
  where the gate demands an error: once a caption has spliced, the recorded
  region carries `#figure(…)` rather than the `#image(…)` it recorded, so a
  following `: ` paragraph fails the second condition and would print as prose
  rather than be refused. The walk therefore remembers that the point it just
  spent was spent, which is what turns the second line into the named error gate
  (4) pins.

**Not refused, deliberately: a `: ` line that follows nothing captionable.** It
is ordinary prose, unchanged, per the rule above. Refusing it would break a
document that never asked for a caption, which is the direction this project's
§2 rule does not run in.

### Where the caption attaches, and why it is not `write_image` (decision, recorded)

Round 1 caught this and it is recorded so an implementer does not re-derive it:
**the caption arrives as a later parser event than the image, so
`core/src/emit.rs:write_image` cannot see it.** It is handed a finished call and
a `standalone` flag, and by then the caption has not been parsed.

**The caption looks back at an image already written; it does not hold the image
waiting for a caption.** That direction is the design, and it was chosen after
the other one failed three times.

**What was tried, and why it is recorded rather than deleted.** A draft extended
`Walk.pending` — the deferral `core/src/emit.rs:step` already runs, which holds a
finished image call and settles `standalone` one event late — to hold across
`Start(Paragraph)` as far as the following paragraph's first `Text`, which is the
first event that says whether a caption follows. Rounds 2, 3 and 4 each found a
different construct broken by that hold, and the third is what condemned the
approach rather than the wording:

- `core/src/emit.rs:Walk::finish` drains a held call in the **inline** form —
  `write_image(&mut self.bufs, &call, false)`, the `bool` discarded — which is
  correct today only because a call never survives to it. Held longer, a
  standalone image reaches the page as `#box(image(…))`.
- Both walk-ending sites hit that: `core/src/emit.rs:emit`, and
  `core/src/emit.rs:collect_definitions`, which takes a footnote body with
  `std::mem::replace(&mut walk, Walk::new()).finish()`.
- **`Walk.para` is a buffer offset, not a flag.** `step` computes
  `opened: *para == Some(top(bufs).len())` against the length recorded when the
  paragraph opened. A call flushed *after* that recording appends to the buffer
  and the offsets stop matching — so **the second of two consecutive standalone
  images is demoted to `#box(image(…))`**, measured against the shipped binary,
  where both are bare `#image(…)` today.

Three symptoms, one cause: the flush timing is load-bearing for machinery that
was not written against a longer hold. **So the flush timing does not change at
all.**

**The mechanism is a recorded splice point.** When
`core/src/emit.rs:write_image` writes a standalone call, the walk records where
it began in the buffer that received it. When a paragraph's first `Text` opens
`: `, the caption accumulates in a frame of its own on the `bufs` stack, and at
that paragraph's end the recorded region is rewritten from `#image(…)` to
`#figure(image(…), caption: [ … ])`. `Walk.pending` behaves exactly as it does
today, `Walk::finish` is untouched, `Walk.para` is never read at a moment its
offset has moved, and every one of the three symptoms above is unreachable
rather than fixed.

**The record is verified at use, never invalidated at a distance**, which is the
property that keeps this from becoming its own bookkeeping problem. It rests on
something true of the whole emitter rather than on care: **every write into a
`bufs` frame is an append** — no `truncate`, `insert`, `replace_range`, `drain`
or `remove` touches one anywhere in `core/src/emit.rs` — so a recorded offset
cannot shift while its frame is live. (`Walk.para` is the nearest idiom but not
an exact one: it is compared at use *and* cleared, at `Event::End(TagEnd::Paragraph)`.
What this borrows is the comparison, not the clearing.) A recorded point is spent
only when three things hold: the buffer
frame is the same depth it was written at, the recorded region still carries the
call it recorded, and **everything after that region is separator newlines and
nothing else**. Any content written in between fails the third, so no clearing
has to be scattered across the emitter's arms for a caption to refuse to attach
to the wrong thing.

**CORRECTED 2026-08-17, on shipping Phase 1: the append-only claim now has
exactly one exception, and it is the splice itself.** The paragraph above says
no `truncate` touches a `bufs` frame anywhere in `core/src/emit.rs`, which was
true when it was measured and stopped being true the moment the rewrite it
argues for was written: `core/src/emit.rs:splice_caption` truncates back to the
recorded point and appends the wrapper. **The argument is unchanged and this is
what it was for** — the one non-append write is the one that spends the record,
and it updates the record in the same breath, so every *other* write is still an
append and a live record's offset still cannot shift under it. Recorded because
a later reader re-deriving the measurement will find that `truncate` and needs
to know it is the sanctioned one rather than a hole in the design.
`rules/pipeline.md` carries the corrected wording, since that is the artifact
that tracks the code.

**The separators are the one detail an implementer will otherwise rediscover.**
By the time the caption's first `Text` arrives, `core/src/emit.rs:step`'s
paragraph arms have pushed a `'\n'` closing the image's paragraph and another
opening the caption's, for a paragraph that is about to be consumed rather than
printed. Both are unwound by the splice. Gate (1)'s byte-exact golden pins the
result either way.

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
point, not a defect. **The look contract does not widen for a caption** — OQ-3
resolved that in round 1: a caption needs no argument crossing the seam, only a
`show` rule over an element the emitter emits, which is how both looks already
reach `raw` and `table.cell` without an export. It may still widen in Phase 2, on
OQ-2's answer rather than on captions.

### Why the check is on what the author wrote (decision, recorded)

`mpdf-004` §2 settled this shape once and it applies unchanged: **a construct
outside the dialect is a named error at the point the author wrote it, never a
silent drop and never a Typst identifier they have never seen.**

Two rules follow, and both exist because of the measurement above.

- **A label is scanned, not passed through.** A name must be a closed character
  set, checked in `core` with the author's line, because Typst's own failure
  names `<a-name>` with no line. The name's character set is Phase 3's to fix,
  and its spelling in a reference is OQ-8's; *that* it is checked here rather
  than left to Typst is fixed.
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
something. So OQ-8 must choose a spelling that a citation spec could extend
rather than contradict, and OQ-5 asks whether "referencing" is a framework with two
kinds — which decides whether that later spec carries `extends: mpdf-005` or
`related` alone. That question is asked here and answered there; it is not a
reason to build the second subject now.

### Why `cli/src` and `app/src` are untouched (decision, recorded)

Neither wrapper reads the dialect. `cli/src/main.rs` and `app/src/document.rs`
both hand a markdown string and an asset list to `md2pdf_core::md_to_pdf`, so a
captioned, numbered, cross-referenced document converts through both the moment
`core` supports it. Each phase checks it as a diff, as `mpdf-004`'s three did.

**The claim is about `cli/src` and `app/src`, not about their tests**, and round
1 was right that the looser phrasing overreached. Two shipped assertions sit in
this work's blast radius even though no wrapper source changes, and both are safe
only because of the decision above: `cli/tests/cli_test.rs:emit_typst_reads_no_image`
asserts a bare `#image("dot.png"` over an *uncaptioned* fixture, and
`core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`
asserts pagination over `samples/article.md`, whose images are uncaptioned too.
Wrapping unconditionally would move both. Wrapping only what carries a caption
moves neither, and Phase 1's gate names them.

## 3. Open questions

- **OQ-1** — ~~what is the syntax for a caption and for a name?~~ **RESOLVED
  (2026-08-15), in round 1 and corrected in round 2: the caption is a paragraph
  of its own, separated by a blank line, opening `: `; a name rides its end as
  `{#name}`.** Round 1's resolution said "the line immediately after the
  construct", and round 2 falsified that by running the parser — with no blank
  line the image and the caption are one paragraph joined by a `SoftBreak`, and
  the example converted to the inline form plus literal `: ` text. §2 records the
  choice, the census behind it, and the rejected spelling. Alt text stays refused as a caption source, on
  `mpdf-002` §1.1's ground; a fenced-div wrapper was weighed and is more syntax
  than one line of prose needs.

  **The question split, and only the caption half was Phase 1's.** The reference
  spelling — `[](#fig:pipeline)`, the one shape that reinterprets a construct
  `mpdf-001` shipped — is a separate call, and OQ-8 carries it against Phase 3.
  That split is what unblocks Phase 1 without pre-deciding the harder half.

- **OQ-2** — ~~does `equations: numbered` generalise, and how?~~ **RESOLVED
  (2026-08-17), after Phase 1 shipped and from a measurement against it: it does
  not generalise, and nothing generalises it. There is no frontmatter key for
  figures, tables or listings — the caption is the ask, and the look decides
  whether a kind carries a number.** The draft offered three shapes and its own
  default was sibling keys; the answer is a fourth the draft did not list, and
  **`supersedes` stays `null` for the reason the draft wanted rather than the one
  it named**.

  **The question presumed a symmetry that the Typst defaults do not have, which
  is what dissolves it.** `math.equation` defaults to `numbering: none`, so
  `equations: numbered` exists to turn numbering **on** and its default `plain`
  is the inert one. `figure` defaults to `numbering: "1"`, so a key here would
  turn numbering **off**. Sibling keys would therefore be four keys over one
  value set with *opposite* defaults — `equations: plain` against
  `figures: numbered` — which reads as a symmetry and behaves as a trap.

  **And the off case needs no key, which is the measurement.** Taken 2026-08-17
  against the shipped Phase 1 by adding one line to `core/assets/template.typ`
  and compiling `tests/fixtures/captions.md`:
  `show figure.where(kind: image): set figure(numbering: none)` returns the
  caption to bare prose — no supplement, no number, no separator — **per kind,
  with no argument crossing the seam and no key in the frontmatter.** That is
  OQ-3's resolution applied one construct further along, and it means the look
  can already express everything a key would have carried.

  **So `equations` stops being an inconsistency and becomes the odd case
  explained.** An equation has no caption, so there is no authorial act to read
  the ask from, which is exactly why `mpdf-004` Phase 3 needed a key; a figure
  has one, and writing it *is* the ask. The two answers differ because the two
  constructs differ, not because one of them is a legacy.

  **The consumer that most wants an unnumbered caption is a look, not a
  document.** A press release carrying one photograph almost certainly does not
  want *"Figure 1"* under it, and that is a fact about press releases rather than
  about any one press release — so it belongs in `core/assets/press-release.typ`,
  where this resolution puts it. Whether that look should in fact suppress is a
  look decision left to Phase 2, not settled here: Phase 1 shipped it numbering
  figures, and changing that moves a page.

  What this costs, recorded rather than waved past: an author who wants captions
  without numbers in *one* document edits a look instead of a frontmatter key.
  No such consumer exists yet, and §5's scope discipline refuses the
  pre-abstraction until one does. **If one turns up, the answer is a new
  question and a new phase, not this one reopened** — and it starts from a
  corpus where the key was never added, which is the cheaper place to start.

- **OQ-3** — ~~does the look contract widen a sixth time?~~ **RESOLVED
  (2026-08-15), in round 1, for Phase 1: no, and nothing is added to the
  contract.** A caption needs no argument crossing the seam. Both looks already
  reach `raw` and `table.cell` with `show` rules and no export —
  `core/assets/template.typ` and `core/assets/press-release.typ` each carry two —
  so `show figure` and `show figure.caption` are reached the same way.
  `mpdf-001` Phase 9's contract, moved to five by `mpdf-004` Phase 3, is
  untouched here. **It may still move in Phase 2**, where OQ-2's frontmatter
  answer could push keys across; that is Phase 2's question, not this one's.

- **OQ-4 — should a reference to an equation read `(1)` or `Equation 1`?** §2
  measured that the page and the reference disagree today, because the format
  lives in `math.equation` and the supplement lives in `ref`. Typst's
  `set ref(supplement: none)` is one candidate and a `show ref` rule keyed to the
  element is another; both are look decisions by §2's seam, which means the
  answer may be "the two looks decide, and they may differ". Answerable from code
  during review. Blocks Phase 3.

- **OQ-5** — ~~is "referencing" a framework with two kinds, or two subjects?~~
  **RESOLVED (2026-08-15), at convergence rather than in a round: two subjects.
  Nothing is reserved, `related` is the whole edge, and this file is not
  renamed.** Recorded as an author's call taken outside the rounds, because it
  gated `status: accepted` and no round had been asked to decide it.

  **§2's own argument for excluding citations is the evidence.** A framework
  under §2 reserves *named kinds sharing a mechanism*; these share none. The
  direction is opposite — inward at content this compile already holds, outward
  at a record the document does not contain. The channel is different, and it is
  the concrete part: a bibliography needs a new asset kind across
  `core/src/lib.rs:Asset`, `core/src/lib.rs:image_paths` and `mpdf-003`'s watch
  loop, where everything here touches `core` alone. And the observable differs —
  typography of existing content against a section the markdown never names. Two
  subjects that share a spelling convention are not one framework with two kinds;
  **a shared convention is honoured by OQ-8 choosing a spelling a citation spec
  could extend, which costs nothing and reserves nothing.**

  So a citation spec, if written, carries `related` rather than
  `extends: mpdf-005`. That is the same reading `mpdf-004` §1.1 worked against
  `mpdf-001` §1.1 and this spec's §1.1 worked against `mpdf-002` §1.1 — a
  non-goals list is not a §2 framework — now applied to this spec's own
  non-goal so it does not claim more standing than it argued for.

- **OQ-6** — ~~does a `figure` wrapper move a shipped golden file, and how
  many?~~ **RESOLVED (2026-08-15), in round 1: none, for Phase 1 — and the
  draft's guess was wrong twice over.** It called this "the second-largest golden
  movement the project would have taken"; round 1 counted, and exactly **one**
  golden carries a standalone `#image(` (`tests/golden/images.typ`) and **one** a
  `#table(` (`tests/golden/table.typ`), with `tests/golden/strikethrough.typ`
  holding only an inline `box(image(` that Phase 1 leaves alone. So the
  unconditional wrap would have moved **one** file, the smallest movement in the
  record rather than the second largest.

  **And §2's answer to B-3 takes it to zero**: no fixture carries a caption line,
  so nothing is wrapped and no shipped golden changes at all. Phase 1 can
  therefore assert "no shipped golden file changed", as `mpdf-004` Phases 1 and 2
  both did. Recorded as a measurement rather than a claim, because the number
  moves the moment a fixture gains a caption.

- **OQ-7 — what does a caption or a name inside a footnote definition do?**
  **The draft's grounding for this was wrong and round 1 corrected it**, which
  matters because the wrong version names the wrong hazard.
  `core/src/emit.rs:collect_definitions` walks a footnote body **once** and keeps
  the translated body; the document walk then *skips* that region. What is
  discarded is the headings vector, which is why `core/src/lib.rs:anchors_from`
  withdraws every anchor when the counts disagree. So the real question is
  whether a name declared inside a spliced body is visible to a reference outside
  it, and whether a figure inside one takes a number in document order.
  Answerable from code during review. Blocks Phase 3.

- **OQ-8 — how is a reference spelled?** Split out of OQ-1, which resolved its
  caption half. `[](#fig:pipeline)` invents nothing, and that is exactly its
  problem: `mpdf-001` shipped links that keep a `#` fragment intact in a
  destination — `tests/golden/links.typ` pins one — so this **reinterprets shipped
  behaviour** and needs §6.1 step 1 worked before it can be a phase rather than a
  supersession. It may survive that test scoped to a fragment matching a name the
  same document declares, since a document with no names would then behave exactly
  as it does today; that is a judgement for Phase 3's round, not an assumption
  here. The alternative is a spelling of its own, which invents dialect and owes
  nothing to shipped work. Design call. **Blocks Phase 3.**

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. All three produce the
observable. **The order follows §2's measurement** — a caption and a number
arrive together for these constructs, so they are not split into separate phases
the way a reader coming from `mpdf-004` Phase 3 would expect.

### Phase 1 — a captioned figure
*Produces the observable: yes — a PDF with a captioned, numbered figure under an
image, which is what a document that shows something needs in order to talk
about it.*

- **Scope:** One construct only — the standalone image — so that §2's caption
  syntax is tested by a real consumer before three more constructs are committed
  to it.

  In `core/src/emit.rs`, a paragraph opening `: ` immediately after a standalone
  image becomes that image's caption, and the pair is rewritten
  `#figure(image(…), caption: [ … ])`. **The attachment is a splice at a point
  recorded when `core/src/emit.rs:write_image` wrote the standalone call**, per
  §2 — not a longer hold on `Walk.pending`, which rounds 2 to 4 measured breaking
  three separate constructs.

  **Nothing about the existing flush changes**: `Walk.pending` is settled one
  event late exactly as today, `core/src/emit.rs:step`'s standalone test is
  untouched, `core/src/emit.rs:Walk::finish` is untouched, and `Walk.para` is
  never read at a moment its offset has moved. The record is verified where it is
  spent — same frame depth, same recorded call, only separators after it — rather
  than invalidated from the emitter's other arms.

  **An image with no caption line is written exactly as it is today**, per §2's
  uncaptioned decision. The inline form is untouched. Nothing else in the dialect
  gains a caption in this phase: a `: ` line after a table, a fenced block or a
  display span is ordinary prose until Phases 2 and 3, and a `: ` line after
  anything else is ordinary prose permanently.

  The caption's content is walked as inline markdown into a buffer of its own on
  the `bufs` stack. **`{#name}` is not implemented here**; a caption line ending
  in one is refused as this phase refuses any caption it cannot yet write, naming
  the line — Phase 3 is where it becomes a Typst label, and shipping a name that
  silently did nothing is the drop `mpdf-001` §2 refuses.

  Both bundled looks gain a `show figure` rule and a `show figure.caption` rule.
  No look argument is added, per OQ-3. **Neither `core/src/frontmatter.rs` nor
  `core/src/emit.rs:header` changes**, so the look contract stays at
  `mpdf-004` Phase 3's five.
- **Exit gate:** (1) A new fixture carrying a captioned standalone image matches
  its golden file and compiles to a PDF with the `%PDF` magic bytes. **Its caption
  carries `*emphasis*` and a `` `code` `` span**, and the golden shows them as
  Typst markup rather than escaped text — the case that pins §2's "walked as
  inline markdown" against the cheaper implementation that writes the caption as a
  string literal, which would pass a gate whose caption was plain prose.

  (2) **An uncaptioned standalone image is byte-for-byte unchanged.**
  `tests/golden/images.typ` gains no `figure`, and
  `cli/tests/cli_test.rs:emit_typst_reads_no_image` still passes over its
  uncaptioned `tests/fixtures/figure.md`. **This is the case that holds §2's
  central decision**: an implementer who wraps unconditionally passes (1) and
  fails only here, and round 1 measured what that costs — an uncaptioned `figure`
  consumes the counter, so the next captioned one reads "Figure 2" with no Figure
  1 on the page, and the body is centred where a bare block sits flush left.

  (3) An inline image inside a sentence is unchanged: its golden still shows
  `box(image(`, and it takes no caption from a `: ` paragraph beneath it. **The
  same case pins the blank line**: an image with `: …` on the *very next line* is
  one paragraph joined by a `SoftBreak`, so it stays the inline form and the `: `
  reaches the page as text — the measured behaviour §2 records, asserted so that
  an implementer who "fixes" it by widening the standalone test fails here.

  (3a) **The three shapes that broke the rejected design keep the standalone
  form**, each an uncaptioned image and so each unchanged from today. They are
  named individually because §2 records that each was measured breaking, and
  because a future implementer who reaches back for the deferral fails here
  rather than shipping:

  - **two consecutive standalone image paragraphs** — both bare `#image(…)`,
    which an `awk` sweep confirms no fixture or sample carries today, so no other
    case reaches the shape;
  - **a standalone image as a footnote definition's last block** — the shape
    `core/src/emit.rs:collect_definitions` ends a walk on, and one
    `tests/fixtures/footnotes.md` does not have, since its definition ends with a
    list;
  - **a standalone image as the document's last block** — already covered from
    the other side by gates (2) and (7), since `tests/golden/images.typ` ends
    with one, and included here so the three sit together.

  **These go in the new fixture and its new golden, not into
  `tests/fixtures/footnotes.md` or `tests/fixtures/images.md`** — editing either
  would move a shipped golden and fail gate (7), which is the same reason
  `mpdf-004` Phase 2 wrote a fixture of its own rather than extending `math.md`.

  (4) **Both refusals, each naming its line**: a `: ` line with no text after it,
  and a second `: ` line beneath the first. And the case that stops the rule being
  a ban — **a `: ` paragraph that follows nothing captionable reaches the page as
  ordinary prose**, unchanged and with no error. An implementer who refuses `: `
  everywhere passes the first half and fails the second.

  (5) A caption line ending in `{#name}` is refused with its line, per the scope.

  (6) **Both looks carry the two rules**, asserted over their sources by a test
  of its own walking `core/tests/golden_test.rs:BUNDLED_TEMPLATES`, with two
  needles per look: `show figure` and `figure.caption`. **Not** by extending
  `every_bundled_template_meets_the_call_contract`, which is named for the
  five-argument contract that OQ-3 resolved does *not* widen here — hanging a
  caption assertion off it would leave a test whose name stopped describing it.
  The caption's *position* — above or below — is deliberately not a needle,
  because it is each look's own call, per §2's seam.

  (7) `cargo test --workspace` passes with **no shipped golden file changed** —
  the property OQ-6 measured, which holds only because uncaptioned constructs are
  not wrapped — and `cli/src` and `app/src` untouched, checked as a diff.
  `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`
  passes unchanged, since `samples/article.md`'s images carry no captions.

  (8) **Read by eye, one PDF per look — two documents.** A golden pins emitter
  output and cannot see a caption's type, its position, or the word "Figure",
  all of which live inside the look. `mpdf-001` Phase 9 read two PDFs for exactly
  this reason and `mpdf-004` Phase 3 reached for it again. Each document is read
  for three things: the caption sets beneath the image, it reads "Figure 1", and
  the emphasis in the caption is italic — which is (1)'s claim seen where it is
  visible rather than in the source.

### Phase 2 — tables and listings take the same treatment
*Produces the observable: yes — a PDF whose tables and code blocks carry the same
captions and their own counters.*

- **Scope:** `core/src/emit.rs:table_call` and the fenced-block arm of
  `core/src/emit.rs:step` take Phase 1's wrapper. No new syntax: if this phase
  needs any, Phase 1 chose wrongly.

  **No frontmatter key lands here, and that is OQ-2's resolution rather than an
  omission.** This is the phase that first has more than one counter, and it
  switches none of them: the caption is the ask, and each look decides per kind
  with a `show figure.where(kind: …)` rule. So `core/src/frontmatter.rs` is
  untouched, `core/src/emit.rs:header` is untouched, and the look contract stays
  at `mpdf-004` Phase 3's five for the second phase running — the widening OQ-3
  left open for this phase does not happen.

  Two things follow that Phase 1 did not have to face. `table_call` writes
  `#table(` **with** the `#` where `image_call` omits it, so the inner `#` has to
  go before the call can sit inside a `#figure(…)`. And with three kinds
  numbering independently, whether the press-release look should suppress any of
  them is a live look decision — Phase 1 shipped it numbering figures, and
  changing that moves a page.

  **The code-block arm is one arm.** `core/src/emit.rs:step` handles fenced and
  indented blocks in the same `Event::End(TagEnd::CodeBlock)`, differing only in
  whether a `lang` argument is written, and both reach Typst as `raw(block:
  true, …)`. So both take a caption. §2's list says "a fenced code block", and
  this is that sentence read against the code rather than narrowed: splitting
  them would make a `: ` line a caption after one kind of block and prose after
  another, with nothing on the page to tell an author which they had written.
- **Exit gate:** eight cases, and the shape is Phase 1's because the mechanism
  is Phase 1's. What is new is that there are now three recordable constructs
  where there was one, and three counters where there was one.

  (1) **A new fixture carrying a captioned table and a captioned code block**
  matches its golden and compiles to a PDF with the `%PDF` magic bytes. The
  golden shows `#figure(table(` and `#figure(raw(` — **with no inner `#`**. That
  is the asymmetry §2 recorded and the one thing in this phase an implementer
  trips on; it fails the *compile* rather than a comparison, since a `#` inside a
  code context is a syntax error, so the case exists to make that failure legible
  rather than to discover it. One of the two captions carries `*emphasis*`, which
  holds Phase 1's "walked as inline markdown" over the constructs it did not
  cover.

  (2) **An uncaptioned table and an uncaptioned code block are byte-for-byte
  unchanged**, which is Phase 1's central decision applied to two more
  constructs: the caption is what makes a figure. Measured 2026-08-17, the blast
  radius is **one golden each** — `tests/golden/table.typ` carries the only
  `#table(` in the corpus and `tests/golden/blocks.typ` the only
  `#raw(block: true`. Neither gains a `figure`.

  (3) **The three kinds number independently.** One document with a captioned
  image, a captioned table and a captioned code block reads *"Figure 1"*,
  *"Table 1"* and *"Listing 1"* — not 1, 2 and 3. **This is the case the phase
  exists for and the one no golden can see**: the emitter writes no supplement
  and no counter, so the source is identical whether Typst keeps one counter or
  three. It is read by eye, with (8).

  (4) **Phase 1's three refusals hold over both new constructs**, each naming its
  line: a `: ` line with no text, a second `: ` line, and a caption ending
  `{#name}`, after a table and after a code block. They run through the code
  Phase 1 shipped, so this is a regression net rather than new behaviour — and it
  is cheap, which is the argument for having it rather than assuming it.

  (5) **A caption reaches the construct above it and no other.** A table, then a
  code block, then a caption: the caption attaches to the code block and the
  table stays bare. **New in this phase**, because Phase 1 had one recordable
  construct and could not express the case. It pins that the record is the last
  one written rather than the last one of its kind.

  (6) **The look contract does not widen, asserted as a diff.**
  `core/src/frontmatter.rs` and `core/src/emit.rs:header` are untouched and
  `every_bundled_template_meets_the_call_contract` is unchanged at five
  arguments. **This is OQ-2's resolution held as a test rather than as prose** —
  the phase that was drafted to add frontmatter keys adds none, and an
  implementer who reaches for one fails here.

  (7) `cargo test --workspace` passes with **no shipped golden file changed** —
  `tests/golden/captions.typ` included, which is why the new cases go in a
  fixture of their own rather than into Phase 1's — and `cli/src` and `app/src`
  untouched, checked as a diff.
  `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`
  passes unchanged: `samples/article.md` carries **both** a table and a fenced
  block and captions neither, so neither is wrapped and its pagination cannot
  move. `samples/press-release.md` carries a table on the same terms.

  (8) **Read by eye, one PDF per look — two documents**, for (3) and for the
  decision this phase has to take. Each is read for the three supplements and the
  three counters at 1. **The press-release look's own call lands here**: a press
  release carrying one photograph is the case OQ-2 named as belonging to a look,
  and Phase 1 shipped that look numbering figures. Suppressing any kind there is
  a page that moves, so it is decided in this phase's round and read here, not
  assumed either way.

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
