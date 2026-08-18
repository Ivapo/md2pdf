---
id: mpdf-005
title: captions-and-references
note: >
  Figures, tables, listings and equations gain captions, numbers and
  cross-references: the emitter wraps them in Typst's `figure`, the looks decide
  what a caption and a number look like, and a reference that names one stays
  true when another is inserted above it.
status: accepted
last_updated: 2026-08-18

phases:
  - name: "Phase 1 — a captioned figure"
    reviewed: 2026-08-15
    shipped: 2026-08-17
    cut: null
    by: null
  - name: "Phase 2 — tables and listings take the same treatment"
    reviewed: 2026-08-17
    shipped: 2026-08-17
    cut: null
    by: null
  - name: "Phase 3 — labels and cross-references"
    reviewed: 2026-08-18
    shipped: 2026-08-18
    cut: null
    by: null
  - name: "Phase 4 — equations join"
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

**Closed 2026-08-17, on drafting Phase 3: OQ-8 resolved it, and the sketch above
is the spelling — with one narrowing the sketch cannot show.** The *empty text*
is what makes it a reference. `[](#fig:pipeline)` becomes `#ref(<fig:pipeline>)`;
`[the diagram](#fig:pipeline)`, or any link that carries text, stays exactly the
link it is today. That is what keeps this a phase rather than a supersession,
and OQ-8 works §6.1 step 1 in full.

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

**WIDENED 2026-08-17, on shipping Phase 2: "the blank line is required" is one
rule about images and not one rule about the dialect.** Measured against the
built binary for the two constructs Phase 2 added, because the sentence above
was about to be carried into `rules/pipeline.md` over all three. Above a
**table** the blank line is required too, but for GFM's reason rather than this
one's: a non-blank line after the last row is one more row, so `: A caption.`
becomes a cell and no caption attaches — a different symptom from the image's,
which is a literal marker on the page. Above a **code block** it is not required
at all: a fence and an indented block each end at their own syntax, so what
follows one is a paragraph either way and the caption attaches with or without
it. **Nothing in §2 changes** — the syntax is what it was, the standalone test
is still untouched, and every gate held — and the rule file and the README carry
the per-construct wording rather than the image's. Recorded because a later
reader re-deriving the sentence above over a code block would find it false and
have no way to tell an omission from a regression.

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
paragraph it is today.

**CORRECTED 2026-08-17, on drafting Phase 3: the display equation is Phase 4's,
and whether it takes a `: ` line at all is now an open question rather than an
assumption.** The sentence above reads as though an equation's name would ride a
caption line like the other three. It cannot as written: an equation has no
caption, so the line would carry a name and no text, which is the first of §2's
own two refusals. OQ-10 carries the syntax and OQ-4 records why the phase split.
The rest of the sentence stands — the marker is still one paragraph in one
position, and the three constructs it attaches to are the three Phases 1 and 2
shipped. That is what keeps the marker from being a ban: a document
that opens a paragraph with `: ` somewhere else is untouched, and the collision
window is one paragraph in one position, which the census finds empty.

**RESOLVED 2026-08-18, on drafting Phase 4: the equation never joins this list,
and the marker stays a rule about three constructs.** OQ-10 chose the closing
`$$` as the carrier — `$$…$$ {#eq:one}` — so a `: ` line after a display equation
is the ordinary prose Phase 3's gate (7) pinned, permanently rather than for one
phase. The two sentences above are therefore complete as they stand: the marker
attaches after a standalone image, a table and a fenced or indented code block,
and nowhere else in the dialect.

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

**Closed 2026-08-17, with OQ-3's own note: it does not widen.** OQ-2 resolved to
no frontmatter key, so nothing crosses in Phase 2 either, and the two bundled
looks reach a table and a listing with the rules they already carry. The sentence
above is left standing because it was true when written.

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

  **CORRECTED 2026-08-17, on drafting Phase 3: the check is not a pre-pass, and
  the sentence above named the wrong precedent.** A name is declared only on a
  caption line that *attaches*, and what decides attachment is the walk itself —
  so a pre-pass gathering names would have to re-run the walk Phase 1 built. The
  check instead collects declarations as the walk meets them and runs once the
  walk is done, which is what makes a reference above its declaration work. The
  claim this section makes is unchanged and is the one that mattered: the check
  is `core`'s, it is a single pass over declared names, and it exists so the error
  names the author's line. Phase 3's scope carries the ordering consequence.

**WIDENED 2026-08-18, on shipping Phase 3: the refusals here take an error type
of their own, and this section named none.** Every refusal in this spec until now
was an `Error::UnsupportedConstruct`, which reads *"unsupported markdown construct
'…' at line N"* and carries no room for a problem string. Phase 3's five cannot
all fit it — this section requires the character-set refusal to name what it
accepts, and "unsupported markdown construct 'name with a character outside …'"
is that requirement met by cramming. So `core/src/lib.rs:Error` gains a `Name`
variant carrying a line and a problem, on the argument `mpdf-004` added `Math`
under: **the construct is a caption or a link, both of which the dialect
supports, and what the error names is the name inside it.** Nothing about the
seam or the checks changes, and no wrapper does either — `cli`, `app` and `web`
all reach `Error` through `Display` alone.

**And one refusal falls out of two rules that had not met.** §2 refuses a `: `
line carrying no text; Phase 3 makes `{#name}` leave the caption. A line reading
`: {#fig:one}` is therefore both, and the implementation tests emptiness *after*
the group is dropped, so it is refused as a caption with no text rather than
labelling a bare *"Figure 1:"*. That is the older rule reached through the newer
syntax rather than a rule of its own; recorded because the gate did not name it
and a later reader would find the case in the tests with nothing behind it.

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

  **Closed 2026-08-17: it does not move.** OQ-2 resolved to no frontmatter key at
  all, so there is nothing to push across, and Phase 2's own scope says the
  contract stays at five for the second phase running. The clause above is left
  standing because it was true when written; this note is what stops a
  consistency sweep reading it as live.

- **OQ-4** — ~~should a reference to an equation read `(1)` or `Equation 1`?~~
  **RESOLVED for Phase 3 (2026-08-17), by taking the subject out of it rather
  than by answering it: equations leave Phase 3 and become Phase 4. CLOSED
  (2026-08-18) in both halves on drafting Phase 4 — see the note at the end of
  this entry.**

  **The question turned out to carry two independent unresolved decisions, and a
  phase holding two is one a round cannot converge.** The first is the compile
  prerequisite the widening below found. The second the draft did not notice it
  was missing: **a name rides the end of a caption line, and an equation has no
  caption line to ride.** A bare `: {#eq:one}` is a marker with no text, which is
  §2's own first refusal, so naming an equation needs syntax Phase 1 did not
  choose — and Phase 2's rule, "no new syntax: if this phase needs any, Phase 1
  chose wrongly", applies to Phase 3 unchanged. OQ-10 carries that half.

  **The obvious shortcut is disqualified by measurement rather than by taste.**
  Wrapping the equation in a `figure` so it takes the caption machinery was
  measured 2026-08-17 against the pinned 0.15.1:
  `#figure($ y = 2 $, caption: [A wrapped equation]) <eq:two>` under
  `equations: numbered` puts **two numbers on the page** — the equation's own
  `(2)` at the margin and a `Figure 1.` caption beneath it — and `@eq:two` reads
  **"Figure 1"**, because `figure` infers no math kind from an equation body.
  Under `plain` the same source shows one number and still the wrong supplement.
  So the wrap is wrong in one setting or the other depending on a key the author
  set for an unrelated reason, and it is out before Phase 4 starts.

  Re-measured the same day, confirming Phase 2's round: a labelled `$ x = 1 $`
  with `@eq:one` fails `equations: plain` with ``cannot reference equation
  without numbering`` and compiles under `equations: numbered`, where the page
  reads `(1)` and the reference reads `Equation 1`. **So the disagreement this
  question was originally about is real and visible on one page**, and it is
  Phase 4's to settle along with the shape that makes the default compile.

  What Phase 3 takes from this is a boundary rather than an answer: it labels the
  three `figure` kinds, and a display equation stays exactly what it is today.
  **That is the third of the three shapes Phase 2's round named below**, taken
  for a reason that round did not have: the carrier is missing, not just the
  numbering.

  The original question, kept as the record — its classification and the phase it
  names are both superseded by the resolution above. §2
  measured that the page and the reference disagree today, because the format
  lives in `math.equation` and the supplement lives in `ref`. Typst's
  `set ref(supplement: none)` is one candidate and a `show ref` rule keyed to the
  element is another; both are look decisions by §2's seam, which means the
  answer may be "the two looks decide, and they may differ". Answerable from code
  during review. Blocks Phase 3.

  **WIDENED 2026-08-17, by Phase 2's round 2, and the harder half is new: under
  the default an equation reference does not read anything, because it does not
  compile.** Typst's "cannot reference X without numbering" is generic over the
  element rather than special to `figure`, and both looks set
  `math.equation(numbering: … else { none })` while `mpdf-004` Phase 3 made
  `plain` the frontmatter default — so the default path *is* the unnumbered one.
  Measured against the pinned 0.15.1 by putting a labelled `$ x = 1 $` and a
  `@ref` to it in a look and compiling twice: `equations: plain` fails with
  ``cannot reference equation without numbering``, `equations: numbered`
  compiles.

  **So Phase 3's "a reference becomes `@name` … equations included" would fail
  the compile for every document that did not opt in**, which is a correctness
  claim rather than a typographic one and is why this is recorded against OQ-4
  rather than left to be found late. It also explains why §2's measurement table
  did not catch it: that probe was run "with `mpdf-004` Phase 3's numbering rule
  active", which is the non-default. Three shapes for Phase 3 to weigh — refuse
  an equation reference in a `plain` document, with an error naming the key the
  author would have to set; number equations always and let a look hide the
  number typographically, which contradicts `mpdf-004` Phase 3's shipped default;
  or scope Phase 3's labels to the three `figure` kinds and leave equations to a
  later phase. **Not Phase 2's problem** — Phase 2 ships no reference and touches
  no look.

  **CLOSED 2026-08-18, on drafting Phase 4, in both its halves.**

  **The compile prerequisite: a reference to an equation is refused in `core`
  when the document did not set `equations: numbered`, and the error names the
  key.** That is the first of the three shapes above, taken after the second and
  third were measured out — and it is the one `mpdf-004` had already very nearly
  decided. That key exists for exactly this consumer: `mpdf-004` Phase 3 shipped
  it so that "a document that refers to its own formulas can number them", which
  the README states in those words. Requiring it is that sentence read back
  rather than a new tax.

  **What the shape costs is stated rather than waved past**: an author who wants
  to point at one equation numbers all of them. That is a real cost and it is
  bounded — the alternative was to number all of them *anyway*, which is what the
  second shape does to every document rather than to the ones that ask.

  **The seam-preserving alternative was built and measured before it was
  rejected, and it fails on a property of Typst rather than on taste.** The
  attractive shape was OQ-2's resolution applied one construct further along:
  the emitter writes only the label, and each look numbers a named equation with
  a `show math.equation` rule of its own, so nothing crosses the seam and no
  frontmatter key is consulted. Measured 2026-08-18 against the pinned 0.15.1, in
  four steps, because each one narrows the next:

  - `it.label` inside a `show math.equation` rule **fails the compile** with
    `equation does not have field "label"`.
  - `it.at("label", default: none)` **works**, and reads `<eq:a>`, `none`,
    `<eq:b>` over three equations. So a look *can* tell a named equation from an
    unnamed one.
  - A rule that re-emits the named one as
    `math.equation(block: true, numbering: "(1)", it.body)` **numbers it on the
    page**, and counts only the named ones.
  - **And the reference still fails**, with `cannot reference equation without
    numbering`. A `show` rule changes what is *drawn*; `ref` resolves against the
    element the label was attached to, whose own `numbering` is still `none`.

  **That last line is the whole finding**, and it disposes of every shape in
  which a look rescues a reference. The only mechanisms left that make `ref`
  work are the frontmatter key, which exists, and an element constructed with a
  `numbering` argument — which means either a format string in the emitter, which
  §2's seam refuses, or a sixth thing crossing the look contract, which OQ-3 has
  held shut for three phases. A look export was measured working
  (`#let equation(body) = math.equation(block: true, numbering: "(1)", body)`,
  under which the page numbers and the reference reads *Equation 1*) and is not
  taken: it buys one convenience at the price of a contract every third look
  would have to meet.

  **The typographic half, which is what this question originally asked, is
  resolved by the seam rather than by a rule.** Re-measured 2026-08-18 with the
  function form: under `equations: numbered` a labelled `$ x = 1 $` needs no
  other change at all — the page reads `(1)`, `(2)`, `(3)` and the references
  read *Equation 1* and *Equation 3*. **The disagreement is real, visible on one
  page, and it is each look's own call**, exactly as a caption's supplement and
  separator are: the page format is `math.equation`'s `numbering` and the prose
  supplement is `ref`'s, both look-side, and both bundled looks currently take
  Typst's defaults for the second. A look that wants them to agree sets `ref`'s
  `supplement`. Nothing here is `core`'s, so nothing here blocks a phase.

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

- **OQ-7** — ~~what does a caption or a name inside a footnote definition do?~~
  **RESOLVED (2026-08-17), by three measurements: a caption inside a definition
  already works, a name declared inside one is visible to a reference anywhere in
  the document, and a figure inside one takes its number at the *reference* site
  rather than the definition's. Nothing is refused — the hazard is not Typst's,
  it is that `core` would not see the name.**

  Measured against the shipped Phase 2 binary and the pinned 0.15.1:

  - **The emitter, already.** A definition whose next block is a standalone image
    with a `: ` line beneath it emits
    `#footnote[A note.\n\n#figure(image(…), caption: […])]<fn-1>`. Phase 1's
    splice runs inside `collect_definitions` because that walk is the same
    `core/src/emit.rs:step`, so a caption in a definition needed nothing and got
    nothing.
  - **Typst.** A `<fig:note>` written inside `#footnote[…]` resolves to a
    `@fig:note` in the body: it compiles, and it reads "Figure 2".
  - **The number.** With one figure before the note, one inside it and one after,
    the page reads Figure 1, Figure 2 — the one in the note, set at the foot of
    the column — and Figure 3. **The counter follows the reference site**, which
    is where the emitter splices the content and where the reader meets it, so
    document order is the order a reader sees.

  **So the answer is an implementation constraint, not a refusal.**
  `core/src/emit.rs:emit` skips a definition's region, so a name declared inside
  one is met only by the walk that is thrown away. A collection that ran on the
  document walk alone would find no such name and would refuse a reference Typst
  resolves perfectly well — a false error over valid input, which is worse than
  the labelless message it was built to replace. The names travel with the body
  the way `core/src/emit.rs:Definitions` already carries its images and its math
  flag, and Phase 3's gate has a case for it.

  The original question, kept as the record.
  **The draft's grounding for it was wrong and round 1 corrected it**, which
  matters because the wrong version names the wrong hazard.
  `core/src/emit.rs:collect_definitions` walks a footnote body **once** and keeps
  the translated body; the document walk then *skips* that region. What is
  discarded is the headings vector, which is why `core/src/lib.rs:anchors_from`
  withdraws every anchor when the counts disagree. So the real question is
  whether a name declared inside a spliced body is visible to a reference outside
  it, and whether a figure inside one takes a number in document order.
  Answerable from code during review. Blocks Phase 3.

- **OQ-8** — ~~how is a reference spelled?~~ **RESOLVED (2026-08-17):
  `[](#name)` — a link with *no text* whose destination is `#` followed by a name
  the document declares. It becomes `#ref(<name>)`. A link that has text is
  untouched, whatever its destination.** §6.1 step 1 is worked below and
  `supersedes` stays `null`.

  **CORRECTED 2026-08-17, by Phase 3's round 1: the Typst side is `#ref(<name>)`
  and not `@name`.** The resolution originally wrote the marker form. Measured on
  the round's finding: `[](#fig:one)s` — an ordinary plural — emits `@fig:ones`,
  which `core`'s own check passes and Typst then fails on a label the author never
  typed, and `@fig:` drops its trailing colon where `#ref(<fig:>)` does not. The
  markdown spelling this question answers is unchanged; what changed is what the
  emitter writes for it, on `mpdf-001`'s own argument for the function forms.

  **Step 1 — does it remove or contradict shipped work? No, and the empty text is
  the whole reason.** Measured 2026-08-17: `[](#fig:one)` emits today
  `#link("#fig:one")[]` — a link with no content, which compiles and **puts
  nothing on the page**. That is the shape this claims, and a shape whose shipped
  behaviour is content reaching no page is the silent drop `mpdf-001` §2 exists to
  refuse, not a behaviour a document can be relying on. Every *visible* link is
  untouched: `[some words](#fig:one)` still emits
  `#link("#fig:one")[some words]`, and `tests/golden/links.typ`'s hostile URL
  keeps its `#frag` for two independent reasons — its text is non-empty and its
  destination is absolute. Censused the same day across `tests/fixtures/`,
  `samples/` and the README: **no link in the corpus has empty text, and none has
  a `#` destination at all**; the two `[](` hits are `![](dot.png)`, an image with
  empty alt that the image arm serves.

  **The draft's own suggested scoping is rejected, and this is the part worth
  recording.** Scoping by *declaredness* — a `#` destination is a reference only
  where the document declares that name — fails twice. It returns `[](#typo)` to
  being an invisible link, which is the drop again, and it makes
  `[Introduction](#introduction)`, the ordinary markdown anchor idiom, change
  meaning in a document that happens to declare a name. Scoping by the empty text
  changes only the degenerate shape and leaves every anchor idiom exactly as it is.

  **A spelling of its own is rejected on the escape rule.** A bare `@name` in
  prose would need a scan inside `core/src/emit.rs:escape_into`'s territory —
  `@` is in `SPECIAL` and every one in body text is escaped today — and it would
  collide with every handle and address an author writes as prose. `[](…)` needs
  no scan at all: pulldown-cmark already resolves it into one `Tag::Link` with the
  destination in hand, which is the same argument `mpdf-001` used for links.

  **And it pays the debt §2 named.** A citation spec, if one is ever written,
  extends this carrier — `[](@key)`, `[](#cite:key)` — rather than inventing a
  second way to point at something.

  **Deliberately not in it: the author's own words over a reference.**
  `#link(<name>)[the pipeline diagram]` was measured working on the same day and
  would be one arm, and §5's scope discipline refuses it until a consumer asks.
  The seam this spec rests on is that the author supplies the caption's words and
  the look supplies the supplement and the number, and §1's promise is exactly
  "As [](#fig:pipeline) shows" reading "As Figure 1 shows".

- **OQ-9 — should a look ever suppress a kind's numbering, given what it costs?**
  Raised by Phase 2's round 1, which found the question live in the gate with
  nothing carrying it — §4's own failure mode, an unresolved question with no
  phase attached. OQ-2 resolved that suppression *belongs* to a look, and that
  stands. What was not known then is the price, measured 2026-08-17 against the
  pinned 0.15.1: **a figure with `numbering: none` cannot be referenced at all**,
  and Typst fails the compile with `cannot reference figure without numbering`.
  So a look that suppresses a kind breaks every Phase 3 cross-reference to that
  kind, in documents whose authors chose neither.

  That is not a reason it can never happen — it is a reason it is a decision with
  a consequence rather than a styling preference. The real want behind it is
  narrow and legitimate: a press release with one photograph does not want
  *"Figure 1"* under it, and it has nothing to cross-reference either. Three
  shapes if it is ever taken up: suppress per kind in one look and accept that
  references to that kind are a compile error there; keep numbering and let the
  look hide the supplement and number typographically, which keeps `ref` working
  and is the only one of the three that costs nothing; or a frontmatter key,
  which OQ-2 refused and which this does not reopen. **The middle one is the
  draft's default if the question is ever asked.** Design call, and one with no
  consumer yet. Blocks nothing — Phase 2 suppresses nothing and Phase 3 needs no
  answer to work.

- **OQ-10** — ~~how is an equation named, given that it has no caption line to
  carry one?~~ **RESOLVED (2026-08-18), on drafting Phase 4: the name rides the
  closing `$$`.**

  ```markdown
  $$
  a^2 + b^2 = c^2
  $$ {#eq:one}
  ```

  becomes `$ a^2 + b^2 = c^2 $ <eq:one>`, and the reference is Phase 3's
  `[](#eq:one)` unchanged.

  **The carrier was chosen by measurement rather than by taste, and the
  measurement is that it needs no mechanism at all.** Taken 2026-08-18 against
  the pinned 0.15.1 and the shipped Phase 3 binary: `$$…$$ {#eq:one}` reaches the
  walk as an `Event::DisplayMath` followed by an `Event::Text` of `" {#eq:one}"`,
  **in the same paragraph and as the very next event**. So the label is appended
  where the equation was just written — no recorded point, no truncate, no splice.
  That is strictly less machinery than Phase 1 needed for a caption, and the
  reason is structural: a caption is a *later paragraph* and a name on a fence is
  the *next event*.

  **The name-only `: ` line is rejected, and Phase 3 is what rejects it.** That
  shape was the draft's first candidate, and shipping Phase 3 made it expensive
  rather than merely inelegant: `: {#eq:one}` is a marker carrying a name and no
  words, which Phase 3 refuses as a caption with no text — a refusal with a test
  behind it. Carving an exception out of it for one construct would make the same
  three characters an error after an image and a declaration after an equation,
  with nothing on the page to tell an author which they had written. It would
  also reverse what Phase 3's gate (7) pinned four commits ago.

  **The `figure` wrapper stays disqualified**, on OQ-4's own measurement: it
  double-numbers under `equations: numbered` and mis-supplements the reference
  under both settings.

  Two narrowings the sketch cannot show, both of them Phase 4's scope to state.
  **The group must be the whole of the text run**, so `$$…$$ {#eq:one} and more`
  is the prose it is today — the same discipline that keeps `: ` a marker in one
  position rather than a ban. And **an inline `$…$` takes no name**, because
  Typst numbers the block form alone and a name on a thing that cannot number is
  a name that cannot be pointed at.

- **OQ-11 — should the two adjacency shapes `#ref(<name>)` still has be closed,
  and how?** Raised by Phase 3's round 2, which measured the function form rather
  than assuming it had none. `[](#fig:one)(a)` emits `#ref(<fig:one>)(a)`, which
  Typst parses as a chained call, and `[](#fig:one).Then` as a field access; both
  fail the compile with a message naming no line. **Both reasons this is a residue rather than a
  regression are measured, and neither is that one form fails louder than the
  other** — the marker's plural failed at the compile too. They are that one
  space defuses both, and that both are strictly narrower than `@name`'s, which
  swallowed any adjacent alphanumeric: `escape_into` already escapes `-`, `_` and
  `[`, and neither `(` nor `.` follows a reference in ordinary prose, since a full
  stop takes a space after it. What is left needs an adjacency prose does not
  produce. Two shapes if it is ever taken up: a terminator written
  after the call, which would have to be measured not to change the spacing on the
  page; or a refusal in `core` when the character after the link is `(` or a `.`
  followed by a letter, which names the author's line and costs the page nothing.
  Design call, and one with no consumer. **Blocks nothing** — Phase 3 pins the
  ordinary shape in gate (1a) and names these two here.


## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. All three produce the
observable. **The order follows §2's measurement** — a caption and a number
arrive together for these constructs, so they are not split into separate phases
the way a reader coming from `mpdf-004` Phase 3 would expect.

**A fourth was appended 2026-08-17**, on drafting Phase 3 and per §6.1 step 2:
the equation left Phase 3 for one of its own, because it needs a naming syntax
the caption line cannot carry and a numbering answer the shipped default fails.
All four produce the observable, and the sentence above still describes the
three that carry a caption.

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

- **Scope:** `core/src/emit.rs:step`'s `Event::End(TagEnd::Table)` and
  `Event::End(TagEnd::CodeBlock)` arms take Phase 1's wrapper. **The arms, not
  `core/src/emit.rs:table_call`** — that is a pure formatter returning a string
  and can record neither a buffer offset nor a frame depth, so whether it keeps
  its `#` is an implementer's choice and the record and the splice land in the
  arm either way. No new syntax: if this phase needs any, Phase 1 chose wrongly.

  **No frontmatter key lands here, and that is OQ-2's resolution rather than an
  omission.** This is the phase that first has more than one counter, and it
  switches none of them: the caption is the ask, and each look decides per kind
  with a `show figure.where(kind: …)` rule. So `core/src/frontmatter.rs` is
  untouched, `core/src/emit.rs:header` is untouched, and the look contract stays
  at `mpdf-004` Phase 3's five for the second phase running — the widening OQ-3
  left open for this phase does not happen.

  **Neither look file changes, and both keep all three kinds numbered.** Round 1
  was right that the draft left this unmade while gate (8) demanded it, so it is
  made here rather than delegated: `core/assets/template.typ` and
  `core/assets/press-release.typ` are untouched by this phase, and their
  kind-agnostic `show figure` and `figure.caption` rules cover a table and a
  listing the moment the emitter emits one.

  **Measured 2026-08-17, and the measurement is what decides it rather than
  taste:** a figure whose numbering a look has suppressed **cannot be referenced
  at all** — Typst fails the compile with `cannot reference figure without
  numbering`, confirmed against the pinned 0.15.1 by putting a suppressed,
  labelled figure and a `@ref` to it in a look and isolating the cause to the
  suppression alone. So a look that suppressed a kind would make **every Phase 3
  cross-reference to that kind fail to compile**, in a document whose author
  chose neither the look's suppression nor the failure. OQ-2 is unchanged as a
  mechanism — the look is still where suppression belongs — but exercising it
  costs something that lands two phases away, so nothing is suppressed until
  there is a reason bigger than that cost. A press release that wants an
  unnumbered photograph is a real want and it is now a question with a price,
  which is what OQ-9 carries.

  Two asymmetries follow that Phase 1 did not have to face, and an implementer
  trips on both. `table_call` writes `#table(` **with** the `#` where
  `image_call` omits it, so the inner `#` has to go before the call can sit
  inside a `#figure(…)`. And **the separators are owned differently**: for a
  standalone image the leading `'\n'` comes from the `Event::Start(Tag::Paragraph)`
  arm, so the recorded `start` lands after it on its own, where for these two the
  *same arm* pushes the `'\n'` and then the call — so the record must start after
  that newline, or `core/src/emit.rs:splice_caption`'s truncate eats the block
  separator and glues `#figure(` onto the line above.

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
  is the first of the scope's two asymmetries, and the sharper of them to
  diagnose; it fails the *compile* rather than a comparison, since a `#` inside a
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

  (3) **The three kinds number independently.** **The same fixture as (1)**,
  which therefore carries a captioned image as well as the table and the block —
  one document rather than three, because the property is that the counters do
  not share, and only one document can show that. It reads *"Figure 1"*,
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
  fixture of their own rather than into Phase 1's — and `cli/src`, `app/src` and
  both look files untouched, checked as a diff.
  `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`
  passes unchanged: `samples/article.md` carries a table, a fenced block **and an
  indented one**, all three captionable after this phase and all three
  uncaptioned, so none is wrapped and its pagination cannot move.
  `samples/press-release.md` carries a table on the same terms — **checked by
  hand rather than by that test**, because no Rust test compiles that sample, so
  naming it here protects nothing on its own.

  (8) **Read by eye, one PDF per look — two documents**: the new fixture in the
  article look, and a scratch copy carrying `template: press-release`, which is
  how Phase 1 read its two. Three things on each, and the second and third are
  new rather than inherited:

  - **the three supplements and the three counters each at 1** — case (3) seen
    where it is visible, and true of *both* documents, since neither look
    suppresses a kind;
  - **the wrapped code block is centred**, where every uncaptioned code block in
    the corpus sits flush left. §2 measured that `figure` centres its body for an
    image and it applies unchanged to a `raw` block, so this is new behaviour
    rather than a regression — gate (2) protects every uncaptioned block — but it
    is the most visible thing this phase does to a page and it would otherwise
    ship unremarked;
  - **the caption reads under the block it names**, not under the one after it,
    and each block keeps the space around it that an uncaptioned one has. Round 2
    was right that this is *not* the separator asymmetry seen from the page —
    that bug glues `#figure(` to the line above, and `figure` being a block
    element, Typst breaks it out anyway, so its symptom is spacing at most.
    Gate (1)'s golden and gate (5) net both byte-exactly; this read is here
    because a page is where a reader would notice either, not because it is the
    only instrument that can.

### Phase 3 — labels and cross-references
*Produces the observable: yes — a PDF whose "as Figure 1 shows" is still true
after a figure is inserted above it, which is the whole reason to number
anything.*

- **Scope:** **Narrowed 2026-08-17, on drafting: the three `figure` kinds, not
  four.** The draft said "all four kinds, equations included", and OQ-4's
  resolution takes the equation out — naming one needs a syntax that does not
  exist and referencing one fails the compile under the shipped default, which
  are two unresolved decisions, and a phase that holds two is one a round cannot
  converge. Phase 4 carries it.

  **A caption line's trailing `{#name}` becomes a Typst label on the figure that
  caption makes**: `#figure(…, caption: [ … ]) <name>`, written into the markup
  buffer the call already sits in. **`Figure.written` takes the label with it**,
  or Phase 1's second-caption refusal silently stops firing:
  `core/src/emit.rs:Figure::live` compares the recorded region against `written`,
  so a label appended without updating it fails the content check and a following
  `: ` paragraph prints as prose where the dialect names an error. The gate has a
  case, because Phase 2's refusal cases carry no name and would all still pass. Phase 1 refuses that group with "caption with
  a name" and `core/src/emit.rs:splice_caption` is where the refusal lives, so it
  is where the implementation replaces it — the name is already parsed out of
  what the author typed, because the markup escape has turned its `#` into `\#`
  by then.

  **The name is a closed character set with a position rule, checked in `core`**:
  every character an ASCII letter, a digit, `-`, `_`, `:` or `.`; **the first
  character not `:` and not `.`**; non-empty; and not `fn-` followed by digits.
  The set is closed rather than "whatever Typst accepts" because §2 requires the
  error to name what it accepts, and each of the three clauses is a measurement
  rather than a preference:

  - `fig:one`, `fig-two`, `fig_three`, `fig.four` and `fig5` are all labels, and
    all five round-trip. Measured 2026-08-17.
  - **A name opening with `:` or `.` is not a label at all.** Measured the same
    day, and it is the reason the position rule exists: Typst's markup enters a
    label only where the character after `<` continues an identifier, so
    `#figure(…) <:foo>` **typesets the literal text `<:foo>` on the page** and
    raises nothing. That is the silent drop §2 and `mpdf-001` §2 exist to refuse,
    reached through a name the dialect would otherwise have accepted, so `core`
    refuses it with the author's line.
  - **`fn-N` is a namespace the emitter already owns.**
    `core/src/emit.rs:step`'s `Event::FootnoteReference` arm writes `]<fn-1>`,
    and those names are generated rather than declared, so the duplicate check
    below would never see the collision — Typst would, with
    ``label `<fn-1>` occurs multiple times in the document`` and no line. One
    reserved prefix is cheaper than teaching the check about a second namespace.

  **The prefix is otherwise the author's convention and the dialect neither
  requires nor reads it:** the kind comes from the body Typst was handed, so
  `{#pipeline}` on an image is a figure and so is `{#tab:pipeline}` on one.

  **The `{#name}` group leaves the caption**, which is not a substring removal
  of what stands in the buffer: `core/src/emit.rs:escape_into` has turned
  `{#fig-two}` into `{\#fig\-two}` by then, since `#`, `-` and `_` are all in
  `SPECIAL`. The name is read from `Caption.text`, which is the unescaped copy
  Phase 1 keeps for exactly this test, and the *caption* is the buffer content
  with the group's own span dropped and the space before it trimmed.

  **A reference is `[](#name)`, per OQ-8, and becomes `#ref(<name>)` — the
  function form, not Typst's `@name` marker.** That is `mpdf-001`'s own argument
  for `#emph[…]` over `_…_`, applied one construct further along: the markup form
  is boundary-sensitive where the function form is **very nearly not** — OQ-11
  records the two shapes that survive it, both loud and both narrower than the one
  they replace. Measured 2026-08-17, and both halves are why this is the spelling
  rather than a preference:

  - `[](#fig:one)s` — an ordinary plural after a reference — emits `@fig:ones`
    under the marker form, which `core`'s own check passes, since `fig:one` is
    declared, and which Typst then fails on a label the author never typed, with
    no line. Measured with a stand-in name: `@figAs` fails with
    ``label `<figAs>` does not exist in the document``, and the plural above names
    `<fig:ones>` the same way.
  - `@fig:` silently drops its trailing colon and references `fig`, where
    `#ref(<fig:>)` resolves to `fig:`. Every shape that is a label at all was
    measured round-tripping through `#ref(<…>)` and landing on the right figure,
    `fig:`, `fig.` and `fig..one` included.

  **The link arm cannot decide this at `Event::Start(Tag::Link)`**, and the
  reason is the one Phase 1 recorded for the image: the emptiness of a link's
  text is a later event. The Start arm writes `#link(…)[` today, so a reference
  opens a frame on the `bufs` stack the way a table cell already does, and
  `Event::End(TagEnd::Link)` pops it and writes either the reference or the link
  the emitter writes now. **The Start arm's two existing refusals still run
  first and are untouched**, so `[](#name "a title")` is a link with a title, as
  it is today. And **empty means empty**: `[ ](#name)`, with a space, is a link
  with text and stays the `#link(…)[ ]` it is today, because the discriminator has
  to be statable and `is_empty()` is, where "text that renders to nothing" is not.

  **Two whole-document checks, and they run at different times for a reason.** A
  repeated name is refused where the second one stands, because that is
  backward-looking and the walk already knows. A reference to an undeclared name
  is refused **after the walk**, because a reference may precede its declaration
  and no pre-pass is needed to emit correctly — the emitter can always write
  `#ref(<name>)`, since Typst is what resolves it, and `core`'s check exists only
  to name the author's line instead of Typst's labelless message. **Where several
  references are undeclared, the one on the earliest line is the error**, which is
  stated because the obvious container is a set and "the first" out of one varies
  between runs — the same determinism this project holds everywhere else.

  **The cost is recorded rather than waved past, and it falsifies a sentence two
  other artifacts state without qualification.** `rules/pipeline.md` says the
  first error in document order is the one reported, and `mpdf-001` §4's Phase 7
  states it in full — "every error, in either pass's territory, surfaces at its
  document position, the first one in document order is the one reported". A
  check that needs the whole document cannot honour that: the walk aborts at the
  first construct error, so a document carrying a bad reference on line 3 and a
  raw HTML block on line 5 reports the HTML. **That is the whole of the
  exception** — every other error keeps its position — and the phase's
  reconciliation owes two artifacts: `rules/pipeline.md`, which tracks the code
  and is corrected outright, and `mpdf-001` §4's Phase 7, whose sentence takes a
  dated note where it stands, per §6.1's rule for shipped prose that a later
  phase makes misleading. A
  declaration pre-pass would keep the ordering and is refused for its price: a
  name is declared only on a caption line that *attaches*, so finding one means
  re-running the walk that decides attachment, which is the mechanism Phase 1
  built and this would duplicate.

  **A name declared inside a footnote definition is a declared name**, per OQ-7.
  `core/src/emit.rs:emit` skips a definition's region, so those names are met only
  by `core/src/emit.rs:collect_definitions`, and they travel with the body the way
  `core/src/emit.rs:Definitions` already carries its images and its math flag. A
  reference written inside a definition travels the same way.

  **Neither look changes and the look contract stays at five for the third phase
  running.** `ref` supplies its own supplement, and both looks already reach
  `figure` and `figure.caption` with `show` rules of their own. Neither
  `core/src/frontmatter.rs` nor `core/src/emit.rs:header` changes, and nothing
  about `equations` is touched.
- **Exit gate:** nine cases.

  (1) **A new fixture carrying a named image, a named table and a named listing,
  and a reference to each**, matches its golden and compiles to a PDF with the
  `%PDF` magic bytes. The golden shows `<fig:pipeline>` immediately after the
  `#figure(…)` call it names, `#ref(<fig:pipeline>)` where the reference stands,
  and **no `#link(` for that reference at all**. It also carries the named
  figure's caption **as an exact string** — `caption: [The conversion pipeline.]`
  under a fixture line reading `: The conversion pipeline. {#fig:pipeline}` —
  because the group leaves the caption text and an implementation that leaves it
  there passes every other case here, the golden it writes itself included. The
  assertion is positive rather than a "no `{` anywhere" sweep: this same fixture
  carries a named listing, and a `raw` string routinely holds a brace.

  (1a) **A reference in the shape ordinary prose puts it in**: one ending a
  sentence, so the emitted `#ref(<fig:pipeline>)` is followed by a full stop and a
  space. It compiles. This is the common case OQ-11's two survivors sit either
  side of, and it costs one fixture sentence to pin.

  (2) **The property the phase exists for, and the one no golden can see: a
  reference that stays true across an insertion.** Two documents differing only by
  a captioned figure inserted above the referenced one; the same source line reads
  *"Figure 1"* in one and *"Figure 2"* in the other. The emitted Typst for that
  line is **byte-identical in both**, which is the point rather than a limitation
  — the number is Typst's — and is why this is read by eye with (9).

  (3) **Five refusals, each naming the author's line**: a reference to a name the
  document does not declare; a name declared twice, refused where the second
  stands; a name carrying a character outside the set, the error listing the set;
  **a name opening with `:` or `.`**, which is the position rule and the case that
  would otherwise typeset `<:foo>` on the page and raise nothing at all; and a
  name matching the reserved `fn-N`. Each is `core`'s rather than Typst's for a
  measured reason — on 2026-08-17 an undeclared reference failed with ``label
  `<nosuchthing>` does not exist in the document``, a repeated one with ``label
  `<dup>` occurs multiple times in the document``, and the reserved collision with
  that same message: all naming a Typst label the author never typed, none
  carrying a line. **Where two references are undeclared, the error names the
  earlier line**, which is asserted rather than assumed because the obvious
  container is a set.

  (3a) **Phase 1's second-caption refusal still fires when the first caption
  carried a name.** Phase 2's cases for it carry none, so an implementation that
  appends the label without carrying it into `core/src/emit.rs:Figure`'s `written`
  passes `cargo test --workspace` and prints the second `: ` paragraph as prose
  where the dialect names an error.

  (4) **The prefix is not a kind.** `{#pipeline}` on an image and
  `{#tab:pipeline}` on an image both work, and both produce a figure. This is the
  case that stops an implementer inventing a prefix rule the dialect does not
  have, and it costs one fixture paragraph.

  (5) **Every shipped link shape is unchanged**, asserted over the shipped
  golden: `tests/golden/links.typ` does not move and
  `core/tests/golden_test.rs:the_links_golden_carries_each_form` passes unchanged.
  And the two cases that hold OQ-8's scoping, both in a document that declares
  `fig:pipeline`: **`[some words](#fig:pipeline)` is still
  `#link("#fig:pipeline")[some words]`**, and **`[ ](#fig:pipeline)`, whose text
  is one space, is still a link too** — empty means empty. An implementer who
  reinterprets every `#` destination passes (1) and fails on the first; one who
  reaches for `trim()` fails on the second.

  (6) **A name declared inside a footnote definition is visible to a reference
  outside it** — OQ-7's measurement held as a test. An implementation that
  collects names on the document walk alone passes every case above and refuses
  this one, because that walk skips the region.

  (7) **No equation is named and none is referenced.** A `: ` line after a display
  equation is the ordinary paragraph it is today, and the `equations` key is
  untouched. **This is what keeps the phase off OQ-4's compile failure**: an
  implementer who extends labels to equations makes every document that did not
  write `equations: numbered` fail to compile, which is a correctness regression
  rather than a typographic one.

  (8) `cargo test --workspace` passes with **no shipped golden file changed**, and
  `cli/src`, `app/src`, `core/src/frontmatter.rs`, `core/src/emit.rs:header` and
  both look files untouched, checked as a diff.
  `core/tests/golden_test.rs:every_bundled_template_meets_the_call_contract` is
  unchanged at five arguments.

  (9) **Read by eye, one PDF per look, plus (2)'s pair.** Each reference reads its
  supplement and its number — *Figure 1*, *Table 1*, *Listing 1* — in running
  prose rather than as a bare number or a gap, and (2)'s pair shows the same
  source line reading 1 in one document and 2 in the other.

### Phase 4 — equations join
*Produces the observable: yes — a PDF whose "as equation (1) shows" is still true
after an equation is inserted above it, which is the same promise Phase 3 made
for the three `figure` kinds and the one construct it could not keep it for.*

**Drafted 2026-08-18, on OQ-10 and OQ-4 being taken.** Both were design calls
rather than code questions, which is why this phase sat deferred through three
shipped ones; both are now closed, and each closed with a measurement behind it.

- **Scope:** the display equation becomes a fourth declaring construct, and
  **nothing else in Phase 3's machinery changes**. The name set, the position
  rule, the reserved `fn-` prefix, `core/src/emit.rs:declare`'s duplicate check,
  `core/src/emit.rs:check_references` and the reference spelling `[](#name)` are
  all Phase 3's and are reused rather than extended.

  **A name rides the closing `$$`**, per OQ-10: `$$…$$ {#eq:one}` becomes
  `$ … $ <eq:one>`. `core/src/emit.rs:step`'s `Event::DisplayMath` arm records
  that it has just written one, and if the **very next event** is an
  `Event::Text` that is **entirely** a `{#name}` group, the arm writes ` <name>`
  and consumes that run. Measured 2026-08-18: the parser delivers exactly those
  two events, adjacent and in one paragraph.

  **This is less machinery than Phase 1 built, not more**, and the asymmetry is
  worth stating because a reader arriving from Phases 1–3 will expect a splice.
  A caption is a *later paragraph*, which is why it needed a recorded point, a
  truncate and the three liveness checks. A name on a fence is the *next event*,
  written where the equation was just written — so **no `Figure` record is made,
  `core/src/emit.rs:splice_caption` is untouched, and the append-only property
  the whole design rests on is not spent a second time.** An implementer who
  reaches for the caption machinery here has built the harder thing.

  **The group must be the whole of the text run.** `$$…$$ {#eq:one} and more` is
  the prose it is today, which is the same discipline that keeps `: ` a marker in
  one position rather than a ban on a line an author may already have written.
  **An inline `$…$` takes no name**, because Typst numbers the block form alone
  and a name on a thing that cannot number can never be pointed at — the inline
  arm is untouched, exactly as the inline image is.

  **A reference to an equation is refused where the document did not set
  `equations: numbered`**, per OQ-4, naming the line and naming the key. This is
  the phase's one genuinely new refusal and the one it exists to get right:
  under the shipped default both looks set `math.equation(numbering: none)`, and
  Typst then fails the whole compile with `cannot reference equation without
  numbering` — a message naming neither line nor key. So `core/src/emit.rs:Names`
  records, beside each declared name, whether the construct that declared it was
  an equation, and the after-the-walk check reads `Walk.front` to decide. The
  check stays a single pass over declared names.

  **Naming is not refused there, only referencing.** Measured 2026-08-18: a
  labelled, unnumbered equation compiles perfectly well as long as nothing points
  at it. Refusing the name would break a document that names an equation before
  it points at one, which is the direction §2's rule does not run in.

  **Neither look changes and the look contract stays at five for the fourth phase
  running.** `core/src/frontmatter.rs` is untouched — `equations` is read, not
  extended — and so is `core/src/emit.rs:header`. `cli/src` and `app/src` are
  untouched, as they have been throughout, and each phase checks it as a diff.
- **Exit gate:** nine cases. Six are Phase 3's shape over a fourth construct and
  are cheap; the two that carry this phase are (2) and (3).

  (1) **A new fixture carrying `equations: numbered`, a named equation and a
  reference to it**, matching its golden and compiling to a PDF with the `%PDF`
  magic bytes. The golden shows `$ … $ <eq:one>` — **the label immediately after
  the closing `$`, with the name group gone from the page** — and
  `#ref(<eq:one>)` where the reference stands.

  (2) **A document with `equations: plain` and an equation reference in it is
  refused, naming the line and naming the key.** **This is the case the phase
  exists for**, and the one a phase built against `equations: numbered` alone
  would ship broken: it is the *default* path, so the failure it prevents is what
  every author who did not read the frontmatter documentation would hit first.
  Measured 2026-08-18 as `cannot reference equation without numbering`, carrying
  no line and no key.

  (3) **A name on an equation in a `plain` document is not refused**, and its
  golden shows the label. The pair (2) and (3) is what pins that the refusal is
  on the reference and not on the name; an implementer who refuses both passes
  (2) and fails here.

  (4) **An unnamed display equation is byte-for-byte unchanged.**
  `tests/golden/display_math.typ` and `tests/golden/numbered_equations.typ` gain
  no label, which is `mpdf-004` Phase 3's property inherited whole: no document's
  typeset output changes unless its author asks. The blast radius is those two
  goldens and no others.

  (5) **The group must be the whole run, and an inline span takes no name.**
  `$$…$$ {#eq:one} and more` reaches the page as prose with its marker intact,
  and `$x$ {#eq:one}` does the same. Both are the cases that stop the rule
  becoming a ban on a brace after a formula.

  (6) **A `: ` line after a display equation is still ordinary prose**, which is
  Phase 3's gate (7) held rather than reversed — and the case that fails for an
  implementer who reached for the caption line as the carrier after all.

  (7) **Phase 3's name rules hold over the fourth construct**, each naming the
  author's line: a character outside the set, a name opening with `:` or `.`, the
  reserved `fn-N`, a name declared twice — once across an equation and a figure,
  since the two share one namespace and a document has one set of names.

  (8) `cargo test --workspace` passes with **no shipped golden file changed**,
  and `cli/src`, `app/src`, `core/src/frontmatter.rs`, `core/src/emit.rs:header`
  and both look files untouched, checked as a diff.
  `core/tests/golden_test.rs:every_bundled_template_meets_the_call_contract` is
  unchanged at five arguments.

  (9) **Read by eye, one PDF per look — two documents.** The page reads `(1)` and
  the reference reads *Equation 1*, which is the asymmetry OQ-4 originally asked
  about, seen where it is visible and left to the looks by §2's seam. And the
  property the phase exists for, on a pair as Phase 3 read its own: two documents
  differing only by an equation inserted above the referenced one, where the same
  source line reads *Equation 1* in one and *Equation 2* in the other.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-005.md, append-only, one heading per round. See §7 of the
methodology.
-->
