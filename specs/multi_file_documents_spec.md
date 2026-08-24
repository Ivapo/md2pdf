---
id: mpdf-008
title: multi-file-documents
note: >
  A document may be written as several markdown files: a master names its
  sections in the order they are read, `core` joins them into the one stream the
  emitter already walks, and every error, every asset and every anchor learns
  which file it came from.
status: draft
last_updated: 2026-08-24

phases:
  - name: "Phase 1 — a location is a file and a line"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — the master reads its sections"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — a section names its own neighbours"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — the desktop app opens a project"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-002, mpdf-003, mpdf-005]
reference: >
  Pandoc's multi-file invocation (`pandoc a.md b.md -o out.pdf`) is the shape
  this rejects: it puts the order in the shell rather than in a file, so the
  document has no written record of what it is made of. LaTeX's `\input` and
  `\include` are the shape this takes — the order lives in a master file that is
  itself a document. Neither's *syntax* is borrowed, and `mpdf-001` §1.1 keeps
  Pandoc excluded permanently. Typst's own `include` is deliberately unused: it
  would put a second file in front of the compiler, where this spec's whole
  design is that the compiler keeps seeing exactly one source.
---

# multi-file-documents

## 1. Goal

Let a long document be written as several files and still convert as one. **The
observable widens for the first time since `mpdf-001` set it**, and §1.1 works
that consequence in full rather than noting it: today it is *"one markdown file
plus the images it names in, single PDF out"*, and after this spec the first
noun is a master file plus the sections it names.

The consumer is the same author, writing something long enough that one file has
stopped being comfortable — a thesis, a report, a manual. Today the only way to
split it is to convert the parts separately and merge the PDFs, which loses the
whole point of `mpdf-005`: a figure number, a cross-reference and a footnote are
document-wide, and three PDFs have three of each starting at one.

After this spec the author writes a master:

```markdown
---
title: A Long Report
figures: sectioned
bibliography: refs.bib
---

[](sections/introduction.md)

[](sections/method.md)

[](sections/results.md)
```

and the sections are ordinary markdown files with no frontmatter of their own. A
figure declared in `introduction.md` is *Figure 1.1*, a `[](#fig:it)` in
`results.md` reads its number, and a footnote defined anywhere lands at the foot
of the column that cites it — because §2's mechanism is to hand the emitter one
stream, which is the stream it already walks.

**The syntax is settled and it is not new syntax.** `mpdf-005` reserved the
empty-text link: `[](#name)` is a cross-reference rather than a link because its
text is empty. This spec reads the other half of that shape — an empty-text link
whose destination is a markdown path — and §2 records the census and the
measurement behind it.

### 1.1 Why this is a new spec and not a phase of an existing one

§6.1 is an ordered test and it is worked in full rather than stopped at the first
step that lands.

- **Step 0 — does this change a decision?** Yes, three, and they are unusually
  explicit. `mpdf-001` §1.1 parks **"multi-file manifests"** in the founding
  non-goals, in the same sentence as math, citations, the Tauri UI and the WASM
  build — every other item on that list has since shipped as a spec of its own.
  `mpdf-005` §1.2 refuses **"references to other markdown files"** and says why:
  *"a document that points into a second document is a project model rather than
  a document model. That is a larger subject and not this one."* `mpdf-003` §1
  parks **"multi-file projects and manifests"** for the desktop app.

- **Step 1 — does it remove or contradict shipped work?** **It removes nothing,
  and it contradicts one thing that is not a spec at all — which is the finding
  that shapes this document.** No phase is un-built and no construct changes
  meaning: §2's inertness rule is that a document with no include marker converts
  to the same bytes it converts to today, on `mpdf-004` Phase 3's property, and
  §4 gates it the way `mpdf-005` Phase 7 gated its own.

  What it does contradict is **the observable, which lives in `CLAUDE.md`'s
  stanza rather than in any spec**: *"One markdown file plus the images it names
  in, single PDF out."* Every phase in this corpus is required to say whether it
  produces that sentence, so a spec that edits the sentence is editing the thing
  seven specs are measured against. **That is a reason to be careful and not a
  reason it cannot be done** — but it is why this cannot be a phase appended to
  `mpdf-001`, whose §1.1 parked the subject and whose own observable is the one
  being widened. OQ-6 carries the wording.

- **Step 2 — is the subject one an existing spec owns?** **No, and three specs
  each own a piece of it, which is the closing rule's own case.** `mpdf-001` owns
  the pipeline and the CLI, `mpdf-002` owns the asset channel this rides,
  `mpdf-003` owns the app that watches the files. §6.1's closing rule is written
  for exactly this: *"A cross-cutting feature still gets its own spec. If the work
  spans several subsystems and its unifying thread is a goal rather than a
  subject … no subject spec has standing to remove what another one shipped."*
  `mpdf-005` §1.2 reached the same conclusion from the other side and called it
  "a larger subject".

- **Step 3 — a named kind under a reserved framework?** No. The three sentences
  above are non-goals lists, not §2 frameworks reserving named kinds — the
  reading `mpdf-004` §1.1 worked and the corpus has now honoured four times. So
  **`extends` stays `null`** and `related` carries the links.

- **Step 4** is therefore the landing: a new spec, `extends: null`.

### 1.2 Non-goals

- **Not a project file, not a build system, and no configuration beyond the
  master's own frontmatter.** The master is a markdown document that happens to
  name others. There is no `.toml`, no target list, no output matrix.
- **Not conditional or parameterised inclusion.** No variables, no `if`, no
  including a file twice with different content. That is a macro system, which
  `mpdf-004` §1.2 refuses on its own ground and which this spec does not reopen
  from a different direction.
- **Not glob patterns or directory inclusion.** `[](sections/*.md)` is not a
  thing. Order is what a master exists to state, and a glob states an ordering
  the filesystem chose. A directory that reordered itself would silently
  reorder the document.
- **Not partial conversion.** There is no "convert only chapter 3". The
  observable is one PDF from the master, and a section is not separately
  convertible while it carries no frontmatter of its own.
- **No second output form and no merge of existing PDFs.** `mpdf-001` §1.1 parks
  both and they stay parked. This joins *markdown*, before the parse, and never
  goes near the compiler's output.
- **Not cross-document references.** A name still lives in one compiled document.
  What widens is what one document is made of, not the ability to point outside
  it — which `mpdf-005` §1.2's second sentence refuses and which this spec, by
  making the sections *one* document, removes the want for rather than grants.

## 2. Design

### The join is markdown, before the parse (decision, recorded)

**`core` concatenates the sections into one markdown string and hands the
existing pipeline exactly what it hands it today.** Not a second parse, not a
merge of two event streams, not a Typst `include`.

This is the decision everything else falls out of, and it was taken because the
alternative is unnecessary rather than because it is easy. Measured 2026-08-24
against the shipped binary, by joining two files by hand and converting the
result:

| written across two files | emitted |
|---|---|
| `![A figure](dot.png)` + `: Declared in part one. {#fig:one}` in the first | `#figure(image("dot.png", …), caption: […]) <fig:one>` |
| `As [](#fig:one) shows…` in the **second** | `As #ref(<fig:one>) shows…` |
| `A claim.[^n]` in the first, `[^n]: The note…` in the **second** | `A claim.#footnote[The note, defined in part two.]<fn-1>` |

**Every document-wide mechanism this project has built already crosses a file
boundary, for free, because it was never written against a file in the first
place.** `core/src/emit.rs:collect_definitions` and `core/src/emit.rs:emit` walk
one event stream; `core/src/emit.rs:check_references` and
`core/src/emit.rs:check_citations` run once after the walk over the names
`core/src/emit.rs:declare` gathered. None of them can tell where the bytes came
from, and none of them needs to. So the figure numbering `mpdf-005` shipped, the
sectioned scheme its Phase 7 added, the footnote two-walk and the citation
namespace all work across sections on the day the join lands, with nothing added
for them.

**What that buys is the reason to prefer it over Typst's own `include`.**
`core/src/lib.rs:TypstWorld` serves one `main.typ`, and putting a second Typst
file in front of the compiler would mean names, footnotes and counters resolving
in Typst's model rather than in the emitter's — a second mechanism doing what
the first already does, and one whose errors name Typst spans the author has
never seen. The compiler keeps seeing exactly one source.

### The join is two newlines, and a naive one silently merges blocks (decision, recorded)

**Sections are joined with a blank line between them, never end-to-end.**
Measured 2026-08-24, and this is recorded because `cat a.md b.md` is what an
implementer reaches for and it is wrong:

- A file ending `Last line of part one.` joined directly to one beginning
  `First line of part two.` is **one paragraph**, the two sentences separated by
  a soft break. No error, nothing on the page to see, and the author's two
  sections have become one paragraph.
- Re-measured with `\n\n` between them: two paragraphs, as written.

The hazard is not limited to paragraphs — a table's last row followed by a
caption line, or any two blocks whose boundary CommonMark decides by a blank
line, has the same shape. **The blank line is therefore part of the join and not
a tidiness rule**, and §4 gates it on the paragraph case because that is the one
that is silent.

### A section carries no frontmatter, and this is refused rather than ignored (decision, recorded)

**A section file that opens with a `---` block is an error naming the file.** Two
measurements, taken 2026-08-24, and each is a different silent corruption:

- **A `---` that follows a paragraph is a setext heading underline, not a
  delimiter.** Joining a first section ending in `Text.` to a second beginning
  `---` / `title: Section two` / `---` produced `== Text.` and
  `== title: Section two` — two level-2 headings, one of them made of the
  author's own YAML. Nothing raised.
- **A `---` block that does follow a blank line is merged into the master's
  frontmatter, and the line it reports is wrong.**
  `core/src/emit.rs:Walk` accumulates metadata text with `meta.push_str` and
  **never clears it**, while `meta_offset` is pinned by `get_or_insert` to the
  *first* block's offset. So the second `End(TagEnd::MetadataBlock)` re-parses
  block one and block two concatenated, against block one's starting line. A
  master saying `title: Master` and a section saying `title: Section` produced
  `frontmatter error at line 4: duplicate key 'title'` — where line 4 is the
  master's closing `---` and the offending key is on line 11.

**The second is a latent defect in shipped code and not a multi-file problem**,
reachable today by any single document carrying two `---` blocks. It is recorded
here because this spec is what makes the shape common rather than exotic, and
because an implementer who leaves it alone ships an error message that points at
the wrong line in the wrong file. OQ-2 carries whether the fix is refusal at the
section boundary, clearing the buffer, or both.

**The master is the only file with a frontmatter block, and that is a feature.**
It is what makes the document's title, look, column count, numbering scheme and
bibliography *one* set of answers rather than a merge with precedence rules
nobody asked for.

### An empty-text link naming a markdown file is an include (decision, recorded)

**The marker is a paragraph whose entire content is `[](path.md)`.** No new
syntax is invented, and this is the argument rather than a convenience.

`mpdf-005` §1 reserved the shape and stated the rule that makes this
possible: *"The empty text is what makes it a reference."* It claimed one
destination — `#name` — and left every other empty-text destination alone. This
spec claims a second: **a destination that `core/src/emit.rs:portable_path`
accepts and that ends `.md`.** Every link that carries text is untouched
whatever its destination, exactly as it has been since `mpdf-005`, so
`[the method](sections/method.md)` stays the link it is today.

**What is being claimed is currently a construct that reaches the page as
nothing**, which is why claiming it costs no document anything. Measured
2026-08-24: `[](sections/intro.md)` emits `#link("sections/intro.md")[]` — a
link with no content, which typesets as nothing at all. That is the `mpdf-001`
§2 faithfulness failure in miniature: the author wrote something and the page
shows nothing. Reinterpreting it removes a silent drop rather than taking a
meaning away.

Censused 2026-08-24 across `tests/fixtures/`, `samples/`, `README.md`, `rules/`
and `web/index.html`, the instrument `mpdf-005` used before claiming `: ` and
`:::`: **no line anywhere begins with `@include`, `!include`, `{{`, `<<`, `+++`
or `===`**, and `mpdf-005` §2's own census established that no link in the
corpus has empty text. So every candidate was free, and the one chosen is free
*and* already reserved.

**It must be the whole paragraph, because an include is a block and a link is
inline.** A marker inside a sentence would splice headings and tables into the
middle of a clause. The discrimination already exists and is not modified:
`core/src/emit.rs:step` tests standalone-ness by whether the paragraph's end is
the next event, which is the same test `core/src/emit.rs:write_image` reads to
tell a standalone image from a boxed one. **`mpdf-005` §2 calls that test "the
one discrimination every image in the dialect flows through" and scoped a whole
phase around not touching it — this spec reads it and does not touch it either.**
An `[](x.md)` that is not alone in its paragraph stays the link it is today.

Two alternatives were weighed and both are worse for the same reason.

- **A frontmatter key listing the files.** Measured: the hand-written YAML
  subset in `core/src/frontmatter.rs:parse` refuses an indented list with
  *"nested keys are not supported"*, and reads `[a.md, b.md]` as the literal
  string it is. So a list-valued key means widening a subset that has been
  deliberately narrow since `mpdf-001`, and it separates the order of the
  sections from the text they sit in — a master could then say nothing of its
  own between two chapters.
- **A new marker**, `{{path}}` or `!include path`. The census says all of them
  are free, and free is not the same as earned: this dialect already has a
  reserved shape that means "point at that thing", and a second one would be two
  ways to say one kind of thing.

### Every message names the file the author wrote it in (decision, recorded)

**This is the cost of the spec, and it is not the join.** Measured 2026-08-24: a
`\undefinedcmd` on **line 4** of a second section reports

```
error: math error at line 11: unsupported command '\undefinedcmd'
```

— line 11 of the joined document, a file that exists nowhere, and no file named
at all. The author is sent to a line they cannot find in a document they did not
write.

**That is unacceptable here specifically**, because "an error names the
construct at the line the author wrote it on" is not a nicety in this corpus:
`mpdf-001` §2 set it, `mpdf-002` §2 applied it to a path, `mpdf-004` §2 restated
it for a LaTeX command, `mpdf-005` §2 built a whole names pass around it — *"not
to make references work, which Typst does, but to make the error name the
author's own line"* — and `mpdf-007` §2 did it again for a citation key. A spec
that shipped the join without this would falsify the one rule every other spec
in the corpus states.

**So a location becomes a file and a line, everywhere one is carried today.**
`core/src/lib.rs:Error` carries a bare `line: usize` in eight of its eleven
variants — `UnsupportedConstruct`, `Frontmatter`, `Math`, `Name`, `Citation`,
`MissingImage`, `MissingBibliography` and `ImageFormat` — and three public types
carry one beside it: `core/src/lib.rs:ImageRef`, `core/src/lib.rs:BibliographyRef`
and `core/src/lib.rs:Anchor`.

**The file is optional and its absence is the master**, which is what keeps a
single-file document's messages byte-identical to today's. A message names a
file only when there is more than one file to name. That is `mpdf-004` Phase 3's
property — *"no document's output changes unless its author asks"* — carried
into the error text, where a golden cannot see it and a test therefore must.

**Phase 1 does this alone and produces no observable**, and §4 argues that
rather than assuming it. The argument is Probe D above: a phase that shipped the
join first would ship the lie, and the lie is the kind that is discovered by an
author rather than by a test.

### A section's neighbours are its own (decision, recorded)

**A path inside a section file resolves against that section's directory, not
the master's.** `sections/method.md` naming `figure.png` means
`sections/figure.png`.

This is `mpdf-002` §2's rule applied one level out — *"a document and its images
stop being a folder that travels as one thing"* — and it is what lets a chapter
folder be moved, copied or shared whole. The showcase folder this repo added on
2026-08-23 is the single-file demonstration of the same principle.

**It is Phase 3 and not Phase 2, because it needs Phase 1's mechanism and
nothing else.** `cli/src/main.rs:read_assets` resolves every path against one
directory — the input file's parent — so a section's own directory is knowable
only once an `ImageRef` says which file named it, which is exactly the widening
Phase 1 makes. Phase 2 therefore ships with every path resolved against the
master, which is a real limitation and a named one rather than a surprise.

### `core` stays OS-free, and a section rides the channel that exists (decision, recorded)

**No new type, and no filesystem in `core`.** `mpdf-001` §2's split — `core`
takes strings and returns bytes, the caller does all I/O — is what lets one
crate compile natively and to `wasm32`, and a spec that read a file in `core`
would end that.

So a section arrives as `core/src/lib.rs:Asset`, the named blob an image already
rides and that `mpdf-007` reused for the bibliography on exactly this argument:
*"It rides the same `Asset` type an image does — a named blob, with nothing
image-specific in it — which is why this channel needed no new type."* Two
reuses make it the pattern rather than the exception.

`core` gains the shopping list beside the two it has —
`core/src/lib.rs:image_paths` and `core/src/lib.rs:bibliography_path` — naming
the sections in the order the master reads them, so `cli/src/main.rs:read_assets`
opens them the way it already opens the other two.

**The order of the two passes is the one thing an implementer will get wrong.**
The section list has to be gathered *before* the images are, because a section's
images are not knowable until the section's text is in hand. So the caller reads
sections first, joins, and only then asks for the image list — one extra round
trip through `core`, and no recursion in the caller.

## 3. Open questions

- **OQ-1 — may a section include a section?** *(design call)* One level is what
  Phase 2 ships and the question is whether that is the answer or the start. A
  nested include needs a cycle check and a depth bound, and the caller's read
  loop stops being one round trip and becomes a fixpoint. The want is real for a
  book with parts, and the shapes available: refuse nesting permanently and say
  so at the marker; allow it with a cycle check keyed on the resolved path; or
  allow one further level and no more, which is a number nobody can defend.
  **Blocks nothing** — Phase 2 refuses nesting by name either way, so this
  decides whether a later phase relaxes it.

- **OQ-2 — is a section's frontmatter refused, ignored, or is the accumulator
  fixed?** *(design call, answerable from code)* §2 measured two separate
  failures — the setext-heading corruption and the never-cleared `meta` buffer —
  and they have different fixes. Refusing a section that opens `---` is the
  cheapest and names the author's file. Clearing `meta` at
  `Event::Start(Tag::MetadataBlock)` fixes the *shipped* defect independently and
  makes a second block a plain duplicate-key error at the right line. They are
  not alternatives and the question is whether Phase 2 does both. **Blocks
  Phase 2's gate**, which must name which failure it pins.

- **OQ-3 — may the master carry prose of its own?** *(design call)* §1's example
  is a pure manifest, but nothing in §2 requires that: the marker is a paragraph,
  so a master could open with a preface and put a page of its own between two
  chapters. Allowing it is free and refusing it is a rule to write. The argument
  for allowing: a title page and a preface are exactly the content that belongs
  to the whole rather than to any section. The argument against: a master that
  is only a table of contents is legible at a glance, and one that is half prose
  is a document you have to read to find the structure of. **Blocks nothing**;
  Phase 2 allows it unless this says otherwise.

- **OQ-4 — what does the browser demo do with a document that names sections?**
  *(needs-input)* `mpdf-006` made the page the project's front door and
  `mpdf-007` Phase 4 taught it to carry two files down one attribute. A third
  channel is the same shape again, but a browser has no directory to resolve
  `sections/method.md` against, and the page's examples are single snippets by
  construction. The honest answers are: the page does not demonstrate this at
  all, and says so; or an example carries its sections inline the way it carries
  its bibliography. **Blocks nothing in this spec** — no phase here touches
  `web/` — but it decides whether `mpdf-006`'s "every claim the page makes is a
  snippet the suite compiles" still holds over the whole dialect.

- **OQ-5 — does the desktop app's scroll sync follow the caret across files?**
  *(design call)* `core/src/lib.rs:Anchor` is a line and a page, and
  `core/src/lib.rs:anchors_from` returns an empty vector on a count mismatch —
  silently, which `mpdf-005` Phase 7's gate had to assert against. With sections
  the pane holds one file while the PDF holds all of them, so an anchor needs its
  file and the pane needs to know which file it is showing. Whether that is one
  editor per section, a single pane that follows the master, or no sync at all in
  the first cut is Phase 4's central call. **Blocks Phase 4**, and nothing
  earlier.

- **OQ-6 — what does the observable become, and who edits it?**
  *(needs-input)* `CLAUDE.md`'s stanza says *"One markdown file plus the images
  it names in, single PDF out"*, and every phase in seven specs is measured
  against that sentence. The replacement has to stay one sentence and stay
  falsifiable — *"one markdown file, or a master and the sections it names, plus
  the images they name, single PDF out"* is the obvious candidate and is already
  clumsy. **Blocks Phase 2's close-out**, which is where the stanza would change,
  and it is a question for the human rather than for a review round.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Phase 1 is the only one that
produces no observable and it is argued for below rather than assumed. The order
is set by §2's error decision: the location widening comes first, because a join
shipped ahead of it is a join that lies about where a mistake is.

### Phase 1 — a location is a file and a line
*Produces the observable: **no**, and this is the argument. Nothing an author can
see changes: no new syntax is accepted, no document converts differently, and
every message on every single-file document is byte-for-byte what it is today.
What it buys is that Phase 2 cannot ship the measurement §2 recorded — an error
in a second file naming line 11 of a document nobody wrote. `mpdf-002` Phase 1
set the precedent for a phase that lands a channel before the construct that
uses it; this is narrower still, being a widening whose whole content is that it
changes nothing yet.*

- **Scope:** `core/src/lib.rs:Error`'s eight line-carrying variants gain an
  optional file, as do `core/src/lib.rs:ImageRef`,
  `core/src/lib.rs:BibliographyRef` and `core/src/lib.rs:Anchor`. The absent file
  is the master, and `Display` omits it entirely — so every message a
  single-file document can produce is unchanged, character for character.
  `core/src/emit.rs:line_of` keeps its signature; what changes is what the
  callers wrap around it. `cli/src/main.rs` and `app/src` are read for
  compilation only: neither formats an error itself, both reaching `Error`
  through `Display`, which is the property `mpdf-005` §2 recorded and this phase
  depends on.
- **Exit gate:** (1) Every error shape in the existing suite produces a message
  **byte-identical** to the one it produces today, asserted as string equality
  rather than as a `contains`. (2) `cargo test --workspace` passes with **no
  golden file changed** — the emitter's output is not in this phase's blast
  radius. (3) A unit case constructs each widened variant *with* a file and
  asserts the file appears in the message. (4) `cli/src` and `app/src` diffs are
  empty but for whatever the type change forces, checked as a diff.
- **Close-out:** `rules/pipeline.md`'s error section, which states the shape of
  every message. No README change: nothing an author can see has moved. One push.

### Phase 2 — the master reads its sections
*Produces the observable: **yes** — a PDF compiled from a master and three
section files, whose figure numbers, cross-references and footnotes run
continuously across all four.*

- **Scope:** `core/src/emit.rs` reads the marker: a paragraph whose entire
  content is one empty-text link whose destination passes
  `core/src/emit.rs:portable_path` and ends `.md`. **The standalone test is read,
  not changed.** A new `core/src/lib.rs` shopping list names the sections in
  order; `core` joins their text with a blank line per §2 and walks the result.
  A section that opens a `---` block is refused by name, per OQ-2's answer, and
  a nested include is refused by name per OQ-1. `cli/src/main.rs:read_assets`
  reads the sections before the images, per §2's ordering note. Every path still
  resolves against the master's directory — Phase 3's limitation, named in the
  README rather than left to be discovered.
- **Exit gate:** (1) The observable, read by eye: a master and three sections
  convert to one PDF, in both bundled looks, with continuous numbering and a
  cross-reference from the third section to a figure in the first. (2) The
  paragraph-merge hazard is pinned: a section ending in a paragraph and the next
  beginning with one are **two** paragraphs in the output, which is the case a
  `cat` join fails silently. (3) An error inside section three names **section
  three's own file and its own line** — the direct answer to §2's Probe D, and
  the case Phase 1 exists for. (4) A section carrying frontmatter is refused
  naming the file; a nested include is refused naming the file. (5) **Inertness**:
  every shipped golden is unchanged and a document with no marker compiles to
  identical PDF bytes across the trees either side of the phase, the comparison
  `mpdf-005` Phase 7 used, because a golden cannot see a look and cannot see a
  join that did not happen. (6) `[](x.md)` inside a sentence, and
  `[text](x.md)` anywhere, are still the links they are today.
- **Close-out:** `rules/pipeline.md` — the dialect gains the marker, and the
  shopping lists become three. `README.md` gains a multi-file section, and its
  "one markdown file" framing moves. **`CLAUDE.md`'s observable stanza changes**,
  on OQ-6's answer — the one close-out in this corpus that edits the sentence
  every other phase is measured against. One push.

### Phase 3 — a section names its own neighbours
*Produces the observable: **yes** — a PDF whose chapter folder holds its own
figures, so the folder can be moved without editing a path.*

- **Scope:** a path named inside a section resolves against that section's
  directory. The mechanism is Phase 1's: `core/src/lib.rs:ImageRef` already says
  which file named it, so `cli/src/main.rs:read_assets` joins against that file's
  parent rather than the input's. `core/src/emit.rs:portable_path`'s shape rule
  is unchanged and still refuses `..`, so a section cannot reach up out of its
  own folder — which is what keeps "a folder travels as one thing" true at both
  levels.
- **Exit gate:** (1) A section in `sections/` naming `figure.png` finds
  `sections/figure.png`, read as a converted PDF. (2) A missing one names the
  section's own path and line. (3) A master naming an image of its own still
  resolves against the master's directory, unchanged. (4) `..` in a section is
  still refused. (5) No shipped golden changes.
- **Close-out:** `rules/pipeline.md`'s asset section; `README.md`'s path rule
  gains the second level. One push.

### Phase 4 — the desktop app opens a project
*Produces the observable: **yes** — the app renders a multi-file document and
re-renders when any section changes.*

- **Scope:** `app/src/document.rs` supplies the sections beside the images, the
  way `mpdf-007` Phase 4 taught it to supply the bibliography.
  `app/src/watch.rs` watches every section as well as the master and its assets.
  The scroll-sync answer is OQ-5's and is scoped here rather than assumed; the
  minimum that ships is that a multi-file document renders and re-renders, with
  sync left explicitly at whatever OQ-5 chooses.
- **Exit gate:** (1) Opening a master renders the whole document. (2) Editing a
  section re-renders, within the debounce `mpdf-003` already gates. (3) An error
  in a section reaches the app's error pane naming that section's file and line.
  (4) Whatever OQ-5 decides for anchors is asserted, including "no sync across
  files" if that is the answer — an unasserted absence is what
  `core/src/lib.rs:anchors_from` already punishes silently.
- **Close-out:** `rules/desktop.md` — the watch set and the file story. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-008.md, append-only, one heading per round. See §7 of the
methodology.
-->
