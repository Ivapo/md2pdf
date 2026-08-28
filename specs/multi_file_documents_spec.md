---
id: mpdf-008
title: multi-file-documents
note: >
  A document may be written as several markdown files: a master names its
  sections in the order they are read, `core` joins them into the one stream the
  emitter already walks, and every error, every asset and every anchor learns
  which file it came from.
status: accepted
last_updated: 2026-08-28

phases:
  - name: "Phase 1 — the master reads its sections"
    reviewed: 2026-08-24
    shipped: 2026-08-24
    cut: null
    by: null
  - name: "Phase 2 — a section names its own neighbours"
    reviewed: 2026-08-24
    shipped: 2026-08-24
    cut: null
    by: null
  - name: "Phase 3 — the desktop app opens a project"
    reviewed: 2026-08-24
    shipped: 2026-08-24
    cut: null
    by: null
  - name: "Phase 4 — the document shows its parts"
    reviewed: 2026-08-25
    shipped: 2026-08-25
    cut: 2026-08-28
    by: mpdf-010
  - name: "Phase 5 — a section may name a figure beside the master"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-002, mpdf-003, mpdf-004, mpdf-005, mpdf-006, mpdf-007]
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

### The line is translated at one boundary, not carried through the walk (decision, recorded)

**REWRITTEN 2026-08-24, by round 1, which found the design this section first
carried could not deliver what it promised.** A draft of this spec widened the
error types and left the walk alone, on the reasoning that a location "becomes a
file and a line". Round 1 established that this buys nothing: **the file would
have had no source.** Every line inside the emitter is produced by
`core/src/emit.rs:line_of` from a byte offset into the joined string, and the
carriers between there and the caller are bare integers —
`core/src/emit.rs:Emitted`'s `headings: Vec<usize>` is the sharpest case, since
`core/src/lib.rs:anchors_from` takes `lines: Vec<usize>` and could not know a
file if it wanted one. Filling the field would have meant widening some twenty
internal carriers, and the draft's phases named none of them.

**The answer is that no internal carrier changes at all.** `core` already knows
where each section begins in the joined string, because it did the joining. That
is a map, and a joined line resolves through it by a single comparison:

```
joined line L  →  the last section whose start ≤ L
                  file = that section's path
                  line = L − start + 1
```

**So the translation is one function applied at one boundary**, not a parameter
threaded through the walk. `core/src/emit.rs:line_of` keeps its signature, every
one of the twenty-three `Error` construction sites in `core/src/emit.rs` keeps
writing the joined line it writes today, and `Emitted.headings` stays a
`Vec<usize>`. The five functions that return a location to a caller —
`core/src/lib.rs:md_to_typst`, `core/src/lib.rs:md_to_pdf_with_anchors`,
`core/src/lib.rs:image_paths`, `core/src/lib.rs:bibliography_path` and the new
`section_paths` — relocate on the way out. **`section_paths` relocates to the
identity and always will**, because it reads the master alone and nesting is
refused, so a `SectionRef`'s file is always absent; it is in the list because a
later phase that answered OQ-1 differently would need it there.

**A single-file document has a one-entry map whose only section starts at line
one**, so the translation is the identity and every message is what it is today,
character for character. That is the inertness property, and it is a property of
the arithmetic rather than of a branch an implementer has to remember.

### What a widened message reads, all nine of them (decision, recorded)

**APPENDED 2026-08-24, by round 1**, which found that the phase said the file
would appear and never said where — and that three of the eight variants it then
had already carried a `{path}`, four of the nine now, so a message with a file in
it would carry two paths with nothing to tell them apart.

**A location is one type with one `Display`.** `core/src/lib.rs:Error`'s
line-carrying variants — eight today and nine after this phase — replace `line: usize` with a `Location` carrying an
optional file and a line, which renders as

```
at line 12                              — no file: exactly today's phrase
in sections/method.md at line 4         — with one
```

and every message interpolates that phrase whole:

| variant | message |
|---|---|
| `UnsupportedConstruct` | `unsupported markdown construct '{construct}' {location}` |
| `Frontmatter` | `frontmatter error {location}: {problem}` |
| `Math` | `math error {location}: {problem}` |
| `Name` | `name error {location}: {problem}` |
| `Citation` | `citation error {location}: {problem}` |
| `MissingImage` | `no image file supplied for '{path}' {location}` |
| `MissingBibliography` | `no bibliography file supplied for '{path}' {location}` |
| `ImageFormat` | `image file '{path}' {location} does not hold {format} data` |
| `MissingSection` | `no section file supplied for '{path}' {location}` |

**`MissingSection` is a ninth variant and not a reuse of `MissingImage`**, on
`mpdf-007` §2's recorded argument for adding `MissingBibliography` beside it:
*"the words are the only thing that differs, and they are the whole point"*. It
has to live in `core` rather than in a wrapper, because `web/src/lib.rs:render`
calls `md_to_pdf` directly with a fixed asset array and there is no wrapper
there to catch it.

**The phase's two other refusals are `UnsupportedConstruct`**, which carries a
construct name and needs no problem string, and their sentences are fixed here
so the gate can enumerate them:

```
unsupported markdown construct 'section with its own frontmatter' in sections/two.md at line 1
unsupported markdown construct 'include inside an included section' in sections/two.md at line 8
```

**The two paths never collide, because only one of them is quoted.** An asset is
`'fig.png'` and a source file is bare after `in`, so
`no image file supplied for 'fig.png' in sections/two.md at line 3` reads once
and correctly. With no file the phrase is `at line 3` and each row above is
byte-for-byte the message that ships today — which is what gate (2) asserts
rather than assumes.

**The four types beside `Error` take the same `Location`.**
`core/src/lib.rs:ImageRef`, `core/src/lib.rs:BibliographyRef` and
`core/src/lib.rs:Anchor` each carry a bare `line` today and each is relocated at
the same boundary; `SectionRef` is the fourth and is born with one. **`Anchor` loses `Copy`**, which nothing depends on —
`app/src/document.rs` consumes `rendered.anchors` by `into_iter` and
`web/src/lib.rs` by `iter` — and is recorded because a public type quietly
losing a marker trait is the kind of thing a later reader finds by compile error.

**What this costs, counted rather than estimated, and re-derived by round 2
after a draft of this paragraph got it wrong:** forty-eight sites pattern-match a
line-carrying variant — forty-three in `core/tests/golden_test.rs`, five in
`core/src/frontmatter.rs` — of which seven use `..`. **Forty-two need editing and
six do not**, because `core/tests/golden_test.rs`'s
`Err(Error::UnsupportedConstruct { line, .. })` uses `..` *and* binds `line`, so
it breaks with the exhaustive ones. Each becomes `location.line` where it reads
`line` today.

**Construction is the larger half and a draft of this section undercounted it.**
Thirty-six sites construct a line-carrying variant — twenty-three in
`core/src/emit.rs`, six each in `core/src/frontmatter.rs` and
`core/src/lib.rs`, one in `core/src/math.rs`. **"Nothing inside the walk changes"
is a claim about the *value*, not about the source**: every one of those sites
writes the same joined line it writes today and every one of them changes
syntactically, because the field it names is renamed. That is roughly eighty
mechanical edits before the widened signatures are counted, and the signatures
add about another hundred and five call-site fixups — ninety-seven of them
`md_to_typst` in `core/tests/golden_test.rs` alone — for something near **a
hundred and eighty in total**. Every one is compiler-driven and none changes
behaviour, but the number is what answers "is this one plan-mode pass", so it is
recorded rather than left to be discovered. It is the same size whichever shape
the field takes — an exhaustive pattern breaks on a renamed field and on an
added one alike.

### A section's neighbours are its own (decision, recorded)

**A path inside a section file resolves against that section's directory, not
the master's.** `sections/method.md` naming `figure.png` means
`sections/figure.png`.

This is `mpdf-002` §2's rule applied one level out — *"a document and its images
stop being a folder that travels as one thing"* — and it is what lets a chapter
folder be moved, copied or shared whole. The showcase folder this repo added on
2026-08-23 is the single-file demonstration of the same principle.

**CORRECTED 2026-08-24, by the sample rewrite that followed Phase 3: the
showcase is no longer single-file, so the sentence above now points at the
opposite of what it describes.** It was rewritten as a master and five sections
in `3c12f7f`, later the same day, on the argument that a folder whose own README
calls it *"one document that uses every construct md2pdf supports"* could not be
the one document not using the marker. So it demonstrates this section's rule
rather than standing beside it: its four figures moved into `sections/`, and the
bare `mark.svg` written in `sections/text.md` now means `sections/mark.svg`.
**The example moved and the rule did not** — which is the whole of the
correction, and is why the paragraph above is otherwise untouched. It is kept
because it is what was true when it was written.

**It is Phase 2 and not Phase 1, because it needs Phase 1's map and nothing
else.** Phase 1 ships with every path resolved against the master, which is a
real limitation and a named one rather than a surprise — `README.md` and
`cli/src/main.rs:read_assets` both say so in as many words.

**REWRITTEN 2026-08-24, by Phase 2's round 1, which found that the mechanism
this section first carried could not deliver what it promised.** A draft said
the caller does it: *"an `ImageRef` already carries the file that named it, so
`cli/src/main.rs:read_assets` joins against that file's parent rather than the
input's."* Round 1 established that this silently corrupts the phase's own
headline case, because **the path the author wrote is an identity and not just a
lookup**, in the four places below:

| where | what it keys |
|---|---|
| `core/src/emit.rs:image_call` | the destination written into the Typst source |
| `core/src/lib.rs:collect` | the `supplied` map, the `seen` dedupe, and `file_id` — the world's `FileId` |
| `cli/src/main.rs:read_assets` | the `seen` dedupe, so a repeated path is read once |
| `app/src/document.rs:read_assets_with` | the same dedupe again |

Measured 2026-08-24 against the shipped binary: a master including `one/a.md` and
`two/b.md`, each naming `figure.png`, emits

```
#image("figure.png", alt: "first")
#image("figure.png", alt: "second")
```

— two byte-identical calls with nothing to tell them apart. A caller that
resolved the two against different directories would read the first file, skip
the second as already seen, and set one figure twice. **No error, and nothing on
the page to see** — the silent-corruption class this spec refuses twice already,
landing on the one case the phase exists for: two chapter folders that each hold
a `figure.png`.

**So the emitter writes the path the master would have written.** `core`
prefixes a section's image destination with that section's own directory, at
emission, because that is the one place that knows both the destination and the
file it was written in. `core/src/sections.rs:Sources` already holds the map;
`core/src/emit.rs:emit` and `core/src/emit.rs:collect_definitions` take it beside
the markdown, and the `Tag::Image` arm of `core/src/emit.rs:step` prefixes the
destination before `core/src/emit.rs:check_image` ever sees it. Four
consequences, and the third is the one that makes this the smaller change as
well as the correct one:

- **The path is unique by construction**, so there is no collision to detect and
  none to refuse. `one/figure.png` and `two/figure.png` are two files because
  they are two names.
- **`check_image` and `core/src/emit.rs:portable_path` are unchanged and still
  run on what the walk sees.** A section writing `../x.png` is prefixed to
  `one/../x.png`, which still carries a `..` segment and is still refused — so a
  section cannot reach up out of its own folder, which is what keeps "a folder
  travels as one thing" true at both levels.
- **No caller changes at all.** `cli/src/main.rs:read_assets` goes on joining
  every path against the master's directory and finds `one/figure.png` there,
  and `app/src/document.rs:read_assets_with` inherits the rule rather than
  needing a copy of it — which the caller-side mechanism could not have given,
  and which is why Phase 3 has nothing to add here.
- **A single-file document has a one-entry map whose only file is absent**, so no
  destination is prefixed and every golden is byte-identical. That is Phase 1's
  inertness arithmetic reused, not a second branch.

The cost, recorded rather than discovered: `Error::MissingImage` names
`one/figure.png` where the author wrote `figure.png`. That is the path relative
to the master, which is the frame a caller supplies assets in, and the location
beside it names the author's own file — so the sentence reads *"no image file
supplied for 'one/figure.png' in one/a.md at line 3"*, which is longer than
before and points at exactly one file.

**Images are the only paths a section can name.** A section carries no
frontmatter, so it cannot name a bibliography, and nesting is refused, so it
cannot name a section. This rule therefore has one site and not a class of them.

**CORRECTED 2026-08-24, by Phase 2's implementation, which measured the ordering
this section states and found it launders a path the dialect refuses.** The
sentence above says the prefix goes on *before* `core/src/emit.rs:check_image`
ever sees the destination, and argues it from `..`: `one/../x.png` still carries
a `..` segment and is still refused, which is true and which round 2 confirmed
against the binary. **The case neither round worked is the absolute path.** Read
in `typst-syntax` 0.15.1 `src/path.rs`, `components()` maps a non-leading empty
segment to `Component::Current`, whose `push_component` arm is documented "has no
effect" — so a section writing `/x.png` is prefixed to `one//x.png`, which
`core/src/emit.rs:portable_path` normalises to `/one/x.png` and **accepts**. An
absolute path becomes a relative one with nothing raised: the silent-corruption
class this spec refuses twice already, reintroduced by the fix for the third.

**So the shipped order is reversed: the shape is checked on what the author
wrote, and the prefix is applied after.** Every refusal keeps the words it has
always used — `..`, absolute, a URL, an empty destination — and each still names
the section's own file and line, which is what gate (4) asks for. Nothing slips
through the other way, because a section's directory came from a marker path that
`portable_path` had already accepted, so a destination this check passes is still
portable once it is prefixed. `check_image` and `portable_path` are unchanged, as
this section says; it is only which string reaches them that moved. The paragraph
above is kept because it is what was thought at the time.

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
opens them the way it already opens the other two. **It is
`pub fn section_paths(md: &str) -> Result<Vec<SectionRef>>`**, and
`SectionRef` carries the path the master wrote and the line it wrote it on, as
`core/src/lib.rs:ImageRef` does — named here because round 2 was right that the
phase's one new public function was described and never named. It takes the
master's text alone, per the ordering decision below.

**The order of the two passes is the one thing an implementer will get wrong,
and round 2 caught a draft of this paragraph saying the caller joins.** It does
not, and it must not: `core` builds the map from the boundaries it creates, so a
caller that joined would leave the map with no source — B4 by the back door.

The order is: **`section_paths` reads the master's own text alone**, because the markers are in the master and no join is needed to see
them; the caller reads those files; and every later entry point takes the master
*and* the sections, joins them internally, and answers about the joined
document. One extra round trip through `core`, no recursion in the caller, and
one place that ever concatenates.

**So three signatures widen, and round 2 was right that the spec had said they
relocate without saying how.** `core/src/lib.rs:md_to_typst`,
`core/src/lib.rs:image_paths` and `core/src/lib.rs:bibliography_path` each take
`md: &str` today and can no more see a section's bytes than they can read a
file. Each gains the sections beside the markdown, exactly as
`core/src/lib.rs:md_to_pdf` gained `assets` in `mpdf-002` Phase 1 — *"the
existing function gains the parameter"* — and for the same reason. `md_to_pdf`
and `md_to_pdf_with_anchors` already take `&[Asset]` and keep their shapes.

**`core/src/lib.rs:md_to_html` is deliberately untouched.** It is infallible and
returns no location, so it has nothing to relocate; what it renders for a master
that is only a list of markers is OQ-4's question and not this phase's.

## 3. Open questions

- **OQ-1 — may a section include a section?** *(design call)* One level is what
  Phase 1 ships and the question is whether that is the answer or the start. A
  nested include needs a cycle check and a depth bound, and the caller's read
  loop stops being one round trip and becomes a fixpoint. The want is real for a
  book with parts, and the shapes available: refuse nesting permanently and say
  so at the marker; allow it with a cycle check keyed on the resolved path; or
  allow one further level and no more, which is a number nobody can defend.
  **Blocks nothing** — Phase 1 refuses nesting by name either way, so this
  decides whether a later phase relaxes it.

- **OQ-2 — is a section's frontmatter refused, ignored, or is the accumulator
  fixed?** *(design call, answerable from code)* §2 measured two separate
  failures — the setext-heading corruption and the never-cleared `meta` buffer —
  and they have different fixes. Refusing a section that opens `---` is the
  cheapest and names the author's file. Clearing `meta` at
  `Event::Start(Tag::MetadataBlock)` fixes the *shipped* defect independently and
  makes a second block a plain duplicate-key error at the right line. They are
  not alternatives, and round 2 was right that only one of them is this spec's.
  **The accumulator is a code-only fix that needs no spec action at all** — §6.1
  step 0 asks whether a decision changed and none did — so it ships in a commit
  of its own, either side of this phase, and this spec records it rather than
  owning it. What Phase 1 owns is the refusal, which gate (7) already pins.
  **Blocks nothing.**

  **CORRECTED 2026-08-24, by Phase 1's implementation, which measured the fix
  this entry proposed and found it does something else.** Clearing `meta` does
  *not* make a second block "a plain duplicate-key error at the right line": it
  makes each block parse alone, so a key repeated across two blocks stops being a
  duplicate at all and the second block silently replaces the first — where the
  shipped code merges them. Measured on the shipped binary: two blocks naming
  different keys produce a document carrying both. That merge is a decision this
  spec has no standing to change under §6.1 step 0, so the fix that shipped
  **pads** the accumulator instead — each block's text is laid out at the lines
  the author wrote it on, and the never-cleared buffer stays never-cleared. The
  duplicate now reports line 8 rather than line 3 in §2's own example, and
  nothing else moves. The sentence above is kept because it is what was thought
  at the time.

- **OQ-3 — may the master carry prose of its own?** *(design call)* §1's example
  is a pure manifest, but nothing in §2 requires that: the marker is a paragraph,
  so a master could open with a preface and put a page of its own between two
  chapters. Allowing it is free and refusing it is a rule to write. The argument
  for allowing: a title page and a preface are exactly the content that belongs
  to the whole rather than to any section. The argument against: a master that
  is only a table of contents is legible at a glance, and one that is half prose
  is a document you have to read to find the structure of. **Blocks nothing**;
  Phase 1 allows it unless this says otherwise.

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

- **OQ-5 — does the desktop app's scroll sync follow the caret across files? —
  RESOLVED 2026-08-24, by Phase 3's round 1**, which found that leaving it to
  Phase 3's plan-mode pass is the shape this corpus has refused twice in this same
  subsystem — `mpdf-007` Phase 3 and Phase 4 each recorded *"a phase that leaves
  this open is a phase that forces a guess"* — and that the gate keyed to it had
  no referent.

  **The answer: the pane holds one file, and an anchor syncs only when it was
  written in that file.** `app/src/document.rs:render_with` keeps an anchor whose
  `location.file` is `None` and drops the rest. `app/src/document.rs:Anchor` stays
  a line and a page, and `app/dist/index.html:caretPage` is not touched.

  **It is the only one of the three that is true by construction.** A line means
  something in exactly one buffer and the pane holds one, so an anchor from
  another file is not a worse match — it is a number about a document the pane is
  not showing. A pure manifest has no headings of its own, so it yields no anchors
  and the frame opens at page 1, which `caretPage` already documents as its
  no-anchor case; a master carrying a preface — which OQ-3 allows — syncs on its
  own headings and syncs correctly. And a later phase that put a section in the
  pane would change which file is kept, not the rule.

  **The other two are rejected, and the second is rejected on a measurement.**
  *One editor per section* is a second pane and a file switcher, where `mpdf-003`
  built one text pane; that is a phase of its own and not one plan-mode pass.
  *No sync at all in the first cut* is not reachable by doing nothing:
  `app/dist/index.html:caretPage` walks a flat list and breaks at the first
  `anchor.line > line`, and since Phase 1 a heading inside a section carries that
  **section's** own 1-based line — so a master whose caret sits on line 7 is
  matched against anchors numbered 1, 4, 1 and the pane opens on whatever page the
  *last* of them landed on. **Those three numbers are not an argument — they are a
  shipped assertion**, `core/tests/golden_test.rs:an_anchor_names_the_file_its_heading_was_written_in`,
  which pins today's anchors for `tests/fixtures/multi_file.md` as
  `(sections/introduction.md, 1)`, `(sections/method.md, 4)` and
  `(sections/results.md, 1)`. Doing nothing ships confidently wrong sync rather
  than absent sync, which is why the filter is named in Phase 3's scope and gated.

  **CORRECTED in the same pass: this entry's premise was already false when it was
  written.** It says *"`core/src/lib.rs:Anchor` is a line and a page … so an
  anchor needs its file"*, but §2 of this document gave `core`'s `Anchor` a
  `Location` carrying an optional file, Phase 1 shipped it, and §2 records that it
  lost `Copy` in the same change. What is still line-only is
  **`app/src/document.rs:Anchor`**, which is what the app serializes to its page —
  and neither this entry nor Phase 3's draft named it, so resolving the question as
  written would have mis-scoped the work. `core/src/lib.rs:anchors_from`'s silent
  empty vector on a count mismatch is unrelated to sections and is unchanged.

  ~~*(design call)* `core/src/lib.rs:Anchor` is a line and a page, and
  `core/src/lib.rs:anchors_from` returns an empty vector on a count mismatch —
  silently, which `mpdf-005` Phase 7's gate had to assert against. With sections
  the pane holds one file while the PDF holds all of them, so an anchor needs its
  file and the pane needs to know which file it is showing. Whether that is one
  editor per section, a single pane that follows the master, or no sync at all in
  the first cut is Phase 3's central call. **Blocks Phase 3**, and nothing
  earlier.~~

- **OQ-6 — what does the observable become, and who edits it? — RESOLVED
  2026-08-24, by the human, in Phase 1's plan-mode pass.** The candidate below
  was taken as written, and it landed in `CLAUDE.md`'s workflow stanza in Phase
  1's close-out. The stanza now reads: *"One markdown file, or a master and the
  sections it names, plus the images they name, single PDF out."* It stays one
  sentence, and it stays falsifiable in both halves — a phase either produces a
  PDF from those inputs or it does not.

  ~~*(needs-input)* `CLAUDE.md`'s stanza says *"One markdown file plus the images
  it names in, single PDF out"*, and every phase in seven specs is measured
  against that sentence. The replacement has to stay one sentence and stay
  falsifiable — *"one markdown file, or a master and the sections it names, plus
  the images they name, single PDF out"* is the obvious candidate and is already
  clumsy. **Blocks Phase 1's close-out**, which is where the stanza would change,
  and it is a question for the human rather than for a review round.~~

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. **All three produce the
observable**, which is a change round 1 forced rather than a property the draft
had: it carried a Phase 1 that produced none, and §2's rewritten location
decision dissolved it — the widening it described had no source for the field it
added, and the translation that replaces it has nothing to do until there are
sections to translate between. The two are one phase because neither is wanted
without the other.

> **CORRECTED 2026-08-25, by Phase 4.** "All three" was true of the three
> phases this section had when it was written, and a fourth has since been
> appended under §6.1 whose header reads *Produces the observable: **no***.
> The sentence is kept as it stood — the claim it was making, that no phase of
> the original three was a scaffolding phase, is untouched — and the count is
> not to be read as a rule the section enforces. Phase 4 argues its own case
> where §1 requires, which is what the rule actually is.

### Phase 1 — the master reads its sections
*Produces the observable: **yes** — a PDF compiled from a master and three
section files, whose figure numbers, cross-references and footnotes run
continuously across all four, and whose errors name the section file the author
wrote them in.*

- **Scope:** `core/src/emit.rs` reads the marker: a paragraph whose entire
  content is one empty-text link whose destination passes
  `core/src/emit.rs:portable_path` and ends `.md`. **The standalone test is read,
  not changed.** A shopping list in `core/src/lib.rs` names the sections in
  order, beside `core/src/lib.rs:image_paths` and
  `core/src/lib.rs:bibliography_path`; `core` joins their text with a blank line
  per §2 and walks the result.

  `core/src/lib.rs:Error`'s eight line-carrying variants and the three types
  beside them take §2's `Location`, and the four functions that return one to a
  caller relocate on the way out through §2's map. **Nothing inside the walk
  changes** — not `core/src/emit.rs:line_of`, not the twenty-three construction
  sites, not `core/src/emit.rs:Emitted`'s `headings: Vec<usize>`.

  **Three signatures widen** — `core/src/lib.rs:md_to_typst`,
  `core/src/lib.rs:image_paths` and `core/src/lib.rs:bibliography_path` each take
  the sections beside the markdown, per §2's ordering decision;
  `core/src/lib.rs:md_to_html` is untouched. **`Error` gains a ninth variant**,
  `MissingSection`, for a marker naming a section the caller did not supply.

  A section that opens a `---` block, and a nested include, are each
  `UnsupportedConstruct` with the construct name §2 fixes, located at the
  section's own file and line.

  **Two sentences belong to the CLI rather than to `Error`, and are named here
  because §2's table cannot reach them.** `cli/src/main.rs` hand-builds
  *"cannot read {} for the bibliography at line {}"* and the image equivalent; a
  section it cannot open needs the third, on the same pattern. And **a section
  whose bytes are not UTF-8 is a new failure mode**, with its precedent already
  set at `core/src/bibliography.rs` for the one other file `core` is handed and
  must read as text. `cli/src/main.rs:read_assets`
  reads the sections before the images, per §2's ordering note. Every path still
  resolves against the master's directory — Phase 2's limitation, named in the
  README rather than left to be discovered. **`cli/src/main.rs:read_assets` and
  `app/src/document.rs:read_assets_with` each build a message that formats an
  `ImageRef` or `BibliographyRef` line by hand**; both become `location.line`.
  **That is not the only edit either wrapper takes, and a draft of this scope
  said it was.** The three widened signatures reach them too:
  `cli/src/main.rs` takes four edits — its `md_to_typst`, `image_paths` and
  `bibliography_path` calls, and the new section pass — and
  `app/src/document.rs` four, at its two `image_paths` and two
  `bibliography_path` calls. All are compiler-driven and none changes
  behaviour.

  **The desktop app's mid-state is named rather than left to be found**, as
  `mpdf-002` Phase 1 named its own: the app supplies no sections until Phase 3,
  so a master opened in it refuses with `MissingSection` naming the first section
  it could not find. That is an honest mid-state and a named one, and it is what
  Phase 3 exists to close.
- **Exit gate:** eight cases. The first three are the phase; the rest are what
  keeps it from moving a document that did not ask.

  (1) **The observable, read by eye, one PDF per look:** a master and three
  sections convert to one PDF, with continuous figure numbering, a
  cross-reference from the third section to a figure in the first, and a
  footnote defined in one section and cited in another.

  (2) **An error inside section three names section three's own file and its own
  line** — the direct answer to §2's measurement, which reports line 11 of a
  document nobody wrote. Asserted on the exact string.

  (3) **The paragraph-merge hazard is pinned:** a section ending in a paragraph
  and the next beginning with one are **two** paragraphs in the output, which is
  the case a `cat` join fails silently.

  (4) **Every message with no file is byte-identical to today's**, enumerated
  rather than gestured at, because round 1 established that "every error shape"
  had no referent in this suite. Two artifacts, and they are different tests.
  **`core/tests/page_examples_test.rs:every_refusal_prints_the_sentence_beside_it`
  passes unchanged** — the repo's only byte-exact `Display` assertion, and it
  covers three rows across two of the nine variants. Beside it, **a new test
  constructs one of each of the nine variants with an absent file and asserts
  its exact sentence**, one row per variant, which is what makes "every error
  shape" enumerable — the nine are listed in §2's table and again in the test.
  **The phase's own two new `UnsupportedConstruct` refusals join that
  enumeration** with the sentences §2 fixes, so nothing this phase adds falls
  outside the gate its own fix built.
  A third set of rows constructs each with a file present and asserts the
  widened sentence, so the `in FILE at line N` phrasing is pinned in both
  directions.

  (5) **`cli/tests/cli_test.rs`'s stderr assertions pass unchanged**, with two
  named exceptions that are deliberately not made byte-exact: the two asserting
  `"os error"` carry text the operating system supplies and the CLI formats
  itself, which is neither `Error`'s `Display` nor reproducible across
  platforms.

  (6) **Inertness:** every shipped golden is unchanged, and a document with no
  marker compiles to identical PDF bytes across the trees either side of the
  phase — the comparison `mpdf-005` Phase 7 used, because a golden pins emitter
  output and cannot pin a join that did not happen.

  (7) **`[](x.md)` inside a sentence, and `[text](x.md)` anywhere, are still the
  links they are today.** A section carrying frontmatter is refused naming the
  file; a nested include is refused naming the file.

  (8) `cargo test --workspace` passes. **`web/src/lib.rs` is checked by hand and
  not by that command** — `Cargo.toml` lists `members = ["core", "cli", "app"]`
  and `web` is deliberately outside it, so the gate says explicitly that `web`
  reads `Error` through `Display` and an anchor through its fields, and compiles
  against the widened types.
- **Close-out:** `rules/pipeline.md` — the dialect gains the marker, the shopping
  lists become three, and the error section takes §2's nine messages.
  `README.md` gains a multi-file section, and its "one markdown file" framing
  moves. **`CLAUDE.md`'s observable stanza changes**, on OQ-6's answer — the one
  close-out in this corpus that edits the sentence every other phase is measured
  against. One push.

### Phase 2 — a section names its own neighbours
*Produces the observable: **yes** — a PDF whose chapter folder holds its own
figures, so the paths inside that folder survive it being moved. Moving it still
means editing the master's marker, which is the one line that exists to say where
a section is.*

- **Scope:** an image named inside a section resolves against that section's
  directory, and it does so because **the emitter writes it that way**, per §2's
  rewritten decision. `core/src/emit.rs:emit` and
  `core/src/emit.rs:collect_definitions` take `core/src/sections.rs:Sources`
  beside the markdown — both walk the joined string and both produce `ImageRef`s,
  so a rule in one and not the other would lose every image inside a footnote
  definition. `core/src/sections.rs:Sources` gains the lookup that answers a
  joined line with the directory of the file it came from, which is the segment
  it already stores. The `Tag::Image` arm of `core/src/emit.rs:step` prefixes the
  destination there, **before `core/src/emit.rs:check_image` sees it**, so the
  shape rule runs on what the walk will emit and a section's `..` is still
  refused.

  **Three internal signatures widen and a fourth caller moves with them**, all
  compiler-forced and none carrying a decision: `core/src/emit.rs:emit`,
  `core/src/emit.rs:collect_definitions` and `core/src/emit.rs:step` take the
  map, and `core/src/lib.rs:render` — the fourth caller of `emit`, beside the
  three entry points that already hold a `Sources` — passes the one it has.

  **A section with no directory of its own prefixes with nothing.**
  `[](chapter.md)` beside the master must leave `dot.png` as `dot.png`, exactly
  as the master's own images are left. The idiom matters: a naive
  `format!("{dir}/{dest}")` yields `/dot.png`, which
  `core/src/emit.rs:portable_path` refuses as absolute — loud rather than silent,
  but wrong, and gate (3) is what catches it.

  **No caller changes**, which is the sharpest way to say what moved:
  `cli/src/main.rs:read_assets` and `app/src/document.rs:read_assets_with` go on
  joining every path against the master's directory, unedited, and find the file
  there. `core/src/lib.rs:md_to_html` is untouched again — it reads no map.

  **The shipped fixture layout is part of this phase and not a surprise it
  causes.** `tests/fixtures/sections/introduction.md` names `dot.png` and
  `method.md` names `mark.svg`, both of which sit in `tests/fixtures/`; after
  this phase those destinations mean `tests/fixtures/sections/dot.png`. The
  images move down beside the sections that name them, which is the layout the
  phase is *for*, and `cli/tests/cli_test.rs:a_master_and_its_sections_convert`
  copies them there instead of beside the master — the comment it carries today
  says in as many words that this is Phase 2's job.
- **Exit gate:** six cases.

  (1) **The observable, read by eye:** a master and two chapter folders, each
  folder holding its own `figure.png`, convert to one PDF in which **the two
  figures differ**. That is the case the written-path identity made impossible,
  and it is asserted on the source as well as read — two `#image` calls with two
  distinct destinations.

  (2) **A section naming a file that is not beside it names the resolved path,
  not the written one** — `no image file supplied for 'sections/figure.png' in
  sections/method.md at line 3`, asserted on the exact string **at the library
  level**, because that is the only level that reaches it: the CLI fails earlier
  at its own `std::fs::read` and prints the resolved path itself. Round 1 was
  right that the draft's *"names the section's own path and line"* already passes
  on the shipped tree, so it pinned nothing; the resolved path in the first slot
  is what this phase changes.

  (3) **Both shapes that prefix with nothing.** A master naming an image of its
  own still resolves against the master's directory — asserted on a master that
  names one directly, since every section in `tests/fixtures/` names one too and
  its golden does move. And **a section beside the master**, `[](chapter.md)`,
  leaves its `dot.png` as `dot.png` rather than as `/dot.png`, which is what the
  wrong join idiom produces.

  (4) **`..` inside a section is still refused**, naming the shape and the
  section's own file and line — the prefix does not launder it.

  (5) **Inertness, with its one named exception.** Every shipped golden is
  unchanged except `tests/golden/multi_file.typ`, whose two image destinations
  gain `sections/`; that one is re-blessed and the change is named here rather
  than found. Every document that names no section compiles to identical PDF
  bytes across the trees either side of the phase — the comparison Phase 1 used,
  because a golden pins emitter output and cannot pin a resolution that did not
  happen.

  (6) `cargo test --workspace` passes. `web/` is checked by hand as Phase 1's
  gate (8) had it, though nothing here changes a signature it reads.
- **Close-out:** `rules/pipeline.md` in **three** places, named because the
  close-out that said "the asset section" pointed at none of them. One sentence
  goes actively false and takes the correction first: §"Several files" asserts
  *"Every path still resolves against the master's directory, a section's own
  images included"*. §"The CLI" says *"each path joins the parent directory of
  the input file"*, which stays true of the caller and now needs saying beside
  what `core` wrote into the path. §"Images and their files" breaks nothing —
  *"once per path at its first reference"* survives, because the prefix is what
  makes two paths two — and it is where the new rule belongs. `README.md`'s
  §"Several files" loses its *"One limitation, for now"* paragraph and its
  §"Images" path rule gains the second level. One push.

### Phase 3 — the desktop app opens a project
*Produces the observable: **yes** — the app renders a multi-file document and
re-renders when any section changes.*

- **Scope, REWRITTEN 2026-08-24 by round 1**, which found that the draft named
  the wrong file for the watch change, cited the wrong precedent for the read, and
  deferred its own central design call into the plan-mode pass it was meant to
  make possible. OQ-5 is now resolved, and the four decisions below are its
  consequences plus the three the reviewer found unmade.

  **The read is a pass of its own, on this spec's own CLI shape and not
  `mpdf-007` Phase 3's.** That phase added a file the walk already named, so it
  fell out of the existing list; a section must be read *before* either shopping
  list can answer anything, which is the ordering §2 records. So
  `app/src/document.rs` gains `read_sections_with` beside
  `app/src/document.rs:read_assets_with`, and the latter takes the sections as a
  parameter and seeds `seen` with them — exactly as `cli/src/main.rs:read_assets`
  takes `read_sections`' result. **One closure serves both passes**: `read` is an
  `impl FnMut`, so the sections pass takes `&mut read` and the assets pass takes it
  by value. A second closure, or a second read of the same file, breaks the one
  property that seam exists to check —
  `app/src/document.rs`'s `a_path_named_twice_is_read_once`.

  **The watch list is `Render::assets`, and `app/src/watch.rs` is not edited.**
  `app/src/watch.rs:root` is one *recursive* watch on the document's own
  directory, computed from the path alone, and `sections/` already sits under it;
  `app/src/watch.rs:classify` filters on the `assets` slice it is handed, which
  `app/src/document.rs:render_with` builds and `app/src/preview.rs:Preview::compile`
  copies. So what changes is the list. **A section arrives as `Change::Asset`**,
  never `Change::Document`: `app/src/preview.rs:Session::on_change` runs
  `Preview::reload`'s external-change rule on `changed.document`, and that rule is
  about the buffer the pane holds, which is never a section.

  **The section paths go into that list first and unconditionally, and this is the
  decision an implementer would get wrong.** `core/src/lib.rs:section_paths` cannot
  fail — it returns `Ok` over `core/src/emit.rs:includes`' bare `Vec` — where
  `core/src/lib.rs:image_paths` and `core/src/lib.rs:bibliography_path` both can,
  and after this phase both fail with `MissingSection` for a section that does not
  exist yet. `Preview::compile` replaces `self.assets` only when the list is
  `Some`, so a list built the way it is built today would stay empty, `classify`
  would drop the section's creation event, and the app would never recover. The
  rule: the list is `Some(sections ++ bibliography ++ images)` when the shopping
  lists answer, `Some(sections)` when they do not and the master names any, and
  `None` otherwise. **Each branch earns its own sentence, because round 2 caught a
  draft giving all three the middle one's.** The first is today's behaviour
  character for character for a document that names no section — an empty section
  list prepended to the same vector in the same order. The third is what keeps a
  transient out-of-dialect edit from dropping the images the app already knows
  about, since `Preview::compile` keeps the previous list on `None`. The second
  *replaces* the list with a shorter one, so a multi-file document with a missing
  section stops watching its figures until that section returns — **a real
  behaviour change, for multi-file documents only, and the trade is deliberate**:
  recovering the section beats watching figures through a window in which nothing
  compiles anyway.

  **The anchors are OQ-5's answer and one filter**: `render_with` keeps an anchor
  whose `location.file` is `None` and drops the rest. Nothing else moves —
  `app/src/document.rs:Anchor` stays a line and a page, and
  `app/dist/index.html:caretPage` is untouched.

  **The third hand-built sentence is the app's to owe.**
  `cli/src/main.rs:read_sections` prints *"cannot read {} for the section {}: {e}"*
  and `render_with`'s own doc comment records that a file which will not read is no
  `Error` at all and that this app owes the CLI the sentence. The section pass
  builds the third, beside the image and bibliography ones `read_assets_with`
  already builds.
- **Exit gate:** eight cases, matching the density Phases 1 and 2 converged at.
  **Seven are automated and one needs the window.** `app` is a workspace member and
  its tests live in the bin target, so all of them run under
  `cargo test --workspace`; `app/src/preview.rs` already ships the harness —
  `counted()`, `wait_for()`, `scratch_dir()` — and
  `a_bibliography_that_does_not_exist_yet_is_watched_and_then_compiles` is the
  direct template for (5). The fixture is in the tree:
  `tests/fixtures/multi_file.md` beside `tests/fixtures/sections/`.

  (1) **The observable, read by eye, once:** opening that fixture in the window
  renders all four files as one PDF.

  (2) **Editing a section recompiles.** Phrased as the precedent phrases it —
  *produces a recompile within a bounded wait* — and not as a latency: there are
  **two** debounces, `app/src/watch.rs:DEBOUNCE` at 100 ms for the filesystem path
  and `app/src/watch.rs:TYPING_DEBOUNCE` at 300 ms for the pane, neither is
  asserted end to end anywhere, and `app/src/preview.rs:wait_for`'s own doc comment
  calls its bound *"a bound on wiring, not a measurement"*.

  (3) **An error in a section reaches `session.preview().error()` naming that
  section's file and line**, on the exact string, which is `Error`'s `Display` and
  therefore the `in sections/… at line N` phrase Phase 1 shipped.

  (4) **The anchors, both directions.** A render of `tests/fixtures/multi_file.md`
  yields **no** anchors, because every heading in it was written in a section and
  the master is a pure manifest; a master carrying a heading of its own yields
  exactly that one, at the master's own line. An unasserted absence is what
  `core/src/lib.rs:anchors_from` already punishes silently.

  (5) **A section that does not exist yet is watched and then compiles** — the case
  `mpdf-003` Phase 2 gate (3) and `mpdf-007` Phase 3 each pinned for their own
  channel, and the one the unconditional list above exists for.

  (6) **Each file is read exactly once across both passes**, on a counted closure —
  the existing property extended to the channel that added a second pass over the
  same directory.

  (7) **Inertness:** a single-file document opened in the app produces the same PDF
  bytes and the same anchors as it did before the phase. `render_with` now calls
  `section_paths` and threads a section array on *every* compile, so this is the
  third time this document has written that arithmetic and the third time it is
  asserted rather than assumed.

  (8) `cargo test --workspace` passes.
- **Close-out:** `rules/desktop.md` — **the filter's list, not the watch set**,
  which is the noun round 2 corrected: nothing about what is watched changes, and
  what goes stale is §"The watch loop"'s *"the one list `image_paths` and
  `bibliography_path` fill is what it filters against"*, which after this phase
  holds three kinds. The file story and the anchor rule go with it. **`max_lines: 455` against 449 body lines, so the cap moves with the
  section.** `README.md` §"The desktop app" loses its *"One exception, for now: it
  does not yet open a document written in several files"* paragraph and its *"The
  app opens one file at a time."* sentence; the mid-state comment inside
  `app/src/document.rs:read_assets_with` names this phase as its closer and goes
  with them. `CLAUDE.md`'s stanza is unchanged — OQ-6 landed it in Phase 1. One
  push.

### Phase 4 — the document shows its parts
*Produces the observable: **no** — the PDF is unchanged. The argument is below.*

Appended 2026-08-25, after Phase 3 shipped, per §6.1 step 2. **The subject is
this spec's and not `mpdf-003`'s**, and the test is that a document of one file
gets nothing from this phase: no panel is drawn, no byte of the window changes.
It is a phase about what a document made of several files looks like, which is
this spec's whole subject, and Phase 3 already took the ground where that meets
the app.

**Why a phase that produces no observable.** This spec widened the first noun of
the observable from a file to *a master and the sections it names*, and Phase 3
taught the app to open one. Nothing lets the author see what the document is
made of. The master's list is the document's structure — its parts, in the order
they are read — and the app was the only front end that could not show it: the
CLI's author has the master open in an editor, and this app's author has it open
in the pane and still cannot see the list without hunting for the markers.

- **Scope:** `app/src/document.rs`, `app/src/preview.rs` and
  `app/dist/index.html`. No new command, and `core` gains nothing.

  **`Render` gains `sections`, and it is the list that was already there.**
  `app/src/document.rs:render_with` computes `named` before either shopping list
  can be asked anything — that is Phase 3's ordering, and it does not change.
  What changes is that the list is kept rather than only folded into
  `Render::assets`. **It is a plain `Vec` where `assets` is an `Option`**: a
  panel draws what the text names, and `assets`' `None` says something about a
  watch filter that a panel cannot draw.

  **It reaches the page through `Status`, beside the anchors and for their
  reason** — the status is already fetched on the path that draws, so this needs
  no command of its own. `Status` also gains `master`, the file *name* of the
  document the pane holds: the panel lists the master above the sections it
  names, and the master is the one row this page could not otherwise name. The
  name and not the path, because the panel is a list of one document's parts and
  where that document sits is the window title's business.

  **Taken whether or not the compile succeeded**, as `assets` is and for the
  same recorded reason: it is read off the text rather than the page, so a
  document that will not compile still names its sections.

  **The list tracks the text exactly, and this reverses what the draft of this
  phase decided.** The draft had it keep its last non-empty answer while the
  master named none, by analogy with `Render::assets`' third branch. **Round 1
  found the analogy unsound and the code already right.** `assets` is `None`
  when `image_paths` *fails* — a transient out-of-dialect edit — where
  `core/src/lib.rs:section_paths` **cannot fail**, so an empty section list is
  never a failure to answer. It is the answer: this text names no marker. Under
  keep-last a master whose markers are genuinely deleted would hold a phantom
  panel for the life of the open document, because no later compile could ever
  restore the empty list. `app/src/preview.rs:Preview::compile` therefore
  assigns unconditionally, as it already does.

  **The cost is a panel that flickers while a marker is being typed**, since a
  half-written `[](sections/…)` names nothing. That is accepted here as the
  price of a panel that is never lying. If it proves unbearable in use the
  answer is to damp the panel's *redraw* — not to retain a list the text has
  stopped naming.

  **The panel is a left column, before the text pane**, at `max-width: 40%`,
  listing the master first and then the sections in master order, with the row
  the pane is holding marked. A single-file document draws none of it.
  Placement is load-bearing rather than taste: `mpdf-003` Phase 7 records two
  geometry bugs caused by this panel taking width from the row, and the
  divider's grab point is computed from the text pane's own left edge because
  of it.

  **A section is named as the master writes it** — `sections/method.md`, not
  `method.md`. It is the master's own words, it is what the author would edit,
  and two sections of the same name in different folders are two rows that must
  not read alike.

  **Absent and folded are two states, and the header carries a `Sections`
  toggle.** `hidden` is a document that names no section, and it takes the
  toggle with it — a dead control on a single-file window is worse than none.
  `.collapsed` is a reader who folded the panel away, and the toggle stays,
  because it is the only thing that brings it back.

  **The fold lives in the page and is not persisted, which is this phase's call
  to make** — `app/dist/index.html` says so in as many words, and round 1
  found the phase had left it unanswered. §2's rule is that state lives in Rust
  where a test can reach it, and the rule is about state that *decides
  behaviour*; a fold decides nothing but its own drawing, and nothing else
  reads it. Moving it to `Preview` would buy nothing either: both die with the
  process, this app persists nothing to disk, and a fold already survives an
  Open where it lives now.

  **The rows do not load, and this phase decides that rather than deferring
  it.** Phase 3 recorded that **the pane holds exactly one file — the master**,
  and four things turn on it: `render_with` keeps only the anchors whose
  location names no file, `Session::on_change` runs the external-change rule on
  the buffer the pane holds and never on a section, `save` writes to the
  document's own path, and the join reads every section off the disk. A row
  that loaded a section would invert the first, fork the second, redirect the
  third and require the fourth to take one file's text from memory. **That is a
  phase, not a detail of this one**, and this phase's contribution is to state
  the invariant it keeps and name the four things a later one must answer.

- **Non-goals:** Not a file browser. It lists the parts the *master names*, in
  the order it names them — not the directory the master sits in, not the files
  it does not name, and nothing is created, renamed or deleted. A panel over the
  directory is a different object with a different job and would need a project
  concept this project has never defined; `mpdf-003` §1.1 parks it.

- **Exit gate:** Two halves, because this phase has two.

  In the suite: `render_with` over a master naming N sections returns those N
  paths in `Render::sections`, in the order the master reads them, and returns
  them for a master whose sections are missing from the disk; a document naming
  none returns an empty list.

  And **at `Preview::compile`, not at `render_with`** — round 2's catch, the
  distinction being the whole point of the clause: a master whose markers are
  then deleted leaves `Preview`'s own list empty on the next compile, and the
  panel loses the rows. `render_with` is stateless, so at *that* surface a
  deleted marker and a document that never had one are the same call and the
  clause would pass under either decision. It is the retained state that the
  reversed decision above is about, and the only place keep-last could have
  lived.

  By eye, against `samples/showcase/showcase.md` and `samples/article.md` —
  `mpdf-003` OQ-10 records that nothing in this repository reaches the page,
  and the phase's visible deliverable cannot go ungated for it:

  1. The master and its five sections are listed, in the order the master reads
     them, with the master's row marked as the one the pane holds.
  2. **`samples/article.md` draws no panel and no toggle**, and the window is
     otherwise the one Phase 3 shipped. This is the phase's own opening claim
     and the reason it belongs to `mpdf-008`.
  3. The `Sections` toggle folds the panel and brings it back, and the toggle
     itself is absent for `article.md`.
  4. A marker is deleted from the master: the row leaves the panel on the next
     compile rather than persisting.

- **Close-out:** **`rules/desktop.md`** gains `Render::sections` and the two
  `Status` fields, which its `Render` and `Status` paragraphs do not yet
  mention; its claim that the pane holds exactly one file stays true and is
  worth saying so beside them. **`rules/desktop-panes.md` is verified against
  the code and regenerated where it disagrees**, not written afresh — it
  already carries a `## The panel` section written from the same prototype, and
  round 1 found this close-out describing it as a file some other phase would
  create. `README.md`'s app section describes a window of two panes and names
  neither the third region nor the `Sections` control, so it gains a sentence.
  **Its own push**: nothing here depends on another phase's.

### Phase 5 — a section may name a figure beside the master
*Produces the observable: **yes** — a document whose section names
`../figures/plot.svg` compiles with that figure in it, where today it refuses
with `image with a '..' path segment` and there is no legal way to write it.*

**§6.1, worked:** step 0 — a decision, since what a path written in a section may
be is §2's own rule. Step 1 — it removes nothing: every document that compiles
today still compiles, and the change is a refusal becoming legal. Step 2 — the
subject is a section's paths, which **this spec owns**. So: an appended phase,
which is what this is.

- **Scope:** `core/src/emit.rs:portable_path` refuses any `..` segment outright,
  before `typst_syntax::VirtualPath::new` sees the path. **That check was correct
  when it was written and this spec is what made it wrong.** With one file,
  *"contains `..`"* and *"escapes the document's folder"* were the same
  statement. §2's section-relative rule separated them: a destination written in
  `sections/method.md` is prefixed with `sections/` by
  `core/src/sections.rs:Sources::resolve`, so `../figures/plot.svg` becomes
  `sections/../figures/plot.svg`, which is `figures/plot.svg` — inside the
  master's own folder, escaping nothing, and refused anyway.

  **The fix is to split the check by what each shape is about, and NOT to flip
  the order.** An earlier draft proposed checking the resolved path instead of
  the written one; review found that reverses a decision §2 made deliberately,
  and it is recorded here so it is not proposed again.
  `core/src/emit.rs:step`'s `Tag::Image` arm checks *before* it resolves because
  `Sources::resolve` is `format!("{directory}/{dest}")` with no guard — so
  `/x.png` in a section becomes `sections//x.png`, and `typst-syntax` 0.15.1 maps
  a non-leading empty segment to `Component::Current` and ignores it. Checking
  after the prefix would therefore **launder an absolute path into a relative
  one**, which is exactly what
  `core/tests/golden_test.rs:the_prefix_launders_no_path_the_dialect_refuses`
  exists to prevent, and it would turn `![alt]()` in a section into
  `image with no file extension` because `dest.is_empty()` would no longer hold.

  **So the shapes divide, and each is checked where it means something:**

  - **A property of what the author wrote** — a scheme, a leading `/`, a
    backslash, an empty destination — stays checked on the written destination,
    before the prefix, exactly as today. That is what the laundering test
    protects and none of it changes.
  - **Escaping the root** is the one shape that is a property of *where the path
    lands*, and it is the only one that moves. It is checked on the resolved path
    by `VirtualPath::new`, whose `Segments::push_component` pops a segment for a
    parent component and returns `PathError::Escapes` only when there is nothing
    left to pop — the rule this dialect wants, stated once by the layer that owns
    the virtual root.

  **The backslash stops being inferred from an error.** `portable_path` ends
  `VirtualPath::new(dest).map_err(|_| PathShape::Backslash)` today, and once
  `Escapes` can arrive there it would be reported as *"a backslash in its path"*.
  The written-shape check tests `dest.contains('\\')` directly instead, which
  is what makes the two distinguishable at all.

  **`core/src/emit.rs:PathShape` gains `Escapes` and keeps `DotDot` for
  nothing**, so the variant goes. Both renderings are user-facing and asserted
  byte-exactly today, so both are written here rather than left to the
  implementer: `PathShape::image` reads **`a path that leaves the document's
  folder`** and `PathShape::key` reads **`a path that leaves the document's
  folder`** — the same words in both, because unlike the other three shapes this
  one is about the destination's *effect* and not its spelling, and there is no
  second phrasing to earn.

  **Normalisation settles the identity, and it must cover both branches of
  `resolve`.** `core/src/lib.rs:collect` keys `supplied`, `seen` and the world's
  `FileId` on the resolved path, so `figures/plot.svg` named by the master and
  `../figures/plot.svg` named by a section must arrive as **one** key — the
  failure the `Tag::Image` arm's comment warns about. `Sources::resolve` has a
  `"" => dest.to_string()` branch for the master's own paths, and normalising
  only the prefixed branch would leave `figures/../plot.svg` in a master
  un-normalised while a section's equivalent normalised — the same identity
  failure in the other direction. Both branches normalise. **`resolve` returns
  `String` infallibly and keeps doing so**: a path that will not normalise
  (`sections/../../escape.png`) falls through unchanged, so `check_image` is
  still what refuses it and still names the author's line.

  **`portable_path` has three callers, not two.** The `bibliography` frontmatter
  key reads it, and there the widening is **a rule change and not only a message
  change**: a master writing `figures/../refs.bib` goes from refused to accepted,
  which is the same widening this phase intends and is stated rather than
  discovered. `core/src/emit.rs:lone_markdown_link` is the third, deciding by
  `portable_path(dest).is_err()` whether an empty-text link is an include marker
  — so `[](sub/../one.md)` becomes a marker where today it is a plain link. That
  follows from the rule and is accepted; the gate pins it either way.

  **How `portable_path` decomposes is left to the implementer — one function
  called twice, or two — but one constraint is not free to discover:**
  `core/src/frontmatter.rs:parse` holds no `Sources`, so the escape check cannot
  move wholly into the `Tag::Image` arm. Both arrangements satisfy the gate
  identically.

  **Two message-selection edges carry today's order rather than a new one.**
  Where an escape and a bad extension both apply — `../../a.bmp` — the shape is
  named first, as it is today. And `![alt](figures/..)` moves from
  `image with a '..' path segment` to `image with no file extension`, which is
  nonsense input reaching a different sentence and is recorded so it is not read
  as a defect. **Marker paths are not normalised where image destinations are**,
  so `sub/../one.md` stays as written in `SectionRef.path` and in
  `Location.file` — the author's own spelling is what a message should name, and
  clause 7 pins the behaviour either way.

  **No caller of `image_paths` changes.** `core/src/lib.rs:image_paths` already
  returns `Sources::resolve`'s output; `cli/src/main.rs:read_assets` and
  `app/src/document.rs:read_assets_with` already join it onto the master's
  directory; and `app/src/watch.rs:classify` already compares against
  `root(document).join(asset)`, `root` being the master's own parent. A figure
  beside the master rather than beside the section is watched, read and supplied
  by the machinery that exists.

- **Exit gate:** In the workspace suite:
  1. A master naming `sections/one.md`, that section naming
     `../figures/plot.svg`, and `figures/plot.svg` on disk: `md_to_pdf` returns
     bytes, and `--emit-typst` writes the path `figures/plot.svg`.
  2. A section naming `../../escape.png` — climbing past the master's own folder
     — is refused as `image with a path that leaves the document's folder`,
     naming the section file and the line, per §2's widened messages.
  3. The master naming `../x.png` is refused with the same sentence.
  4. **The laundering test still passes, with one row rewritten.**
     `core/tests/golden_test.rs:the_prefix_launders_no_path_the_dialect_refuses`
     keeps its `/x.png` and `https://` rows **byte-identical** — that is the
     clause that proves the order was not flipped — and its `../x.png` row moves
     from a refusal to a compile. `![alt]()` in a section is still
     `image with an empty destination`.
  5. **The identity holds across both spellings.** A master naming
     `figures/plot.svg` and a section naming `../figures/plot.svg`:
     `image_paths` returns **two entries naming the same string** — it
     deduplicates nothing, by its own documented contract, and the caller's
     `seen` set is what supplies one asset — and `--emit-typst` writes that
     string in both places. A master naming `figures/../plot.svg` normalises to
     `plot.svg`, which is the `""`-branch clause.
  6. `../refs.bib` in the master's frontmatter is refused as
     `key 'bibliography' takes a path beside the document, not a path that leaves
     the document's folder`; `figures/../refs.bib` is accepted.
  7. `[](sub/../one.md)` is read as an include marker.
  8. **The corpus check Phase 1 used, re-run:** every document in this tree
     compiles to a byte-identical PDF and byte-identical Typst source either side
     of the commit. Nothing that was legal changes; only something that was
     refused becomes legal.
  9. `cargo test --workspace` passes, as Phases 1, 2 and 3 each closed with —
     load-bearing here, since this phase moves several shipped byte-exact
     assertions.

- **Close-out:** **`rules/pipeline.md`** — the seven refused destination shapes,
  `portable_path`'s rule, the section-prefix passage, **and the bibliography-key
  passage**, which states the `..` rule a fourth time. **`rules/desktop.md`**'s
  watch-loop section argues the whole watch set from *"`check_image` refuses … a
  `..` segment"*; the conclusion survives and the premise does not, so the
  argument is restated. **`README.md`** carries the sentence this phase falsifies
  in so many words — *"a path with a `..` segment, which escapes the document's
  own folder — a section's included, so a section cannot reach up out of the
  folder it sits in"* — and it is rewritten, not annotated, the README being
  user-facing documentation rather than a record. **Its own push**; nothing here
  depends on another phase's, and `mpdf-010` does not depend on this one.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-008.md, append-only, one heading per round. See §7 of the
methodology.
-->
