---
id: mpdf-006
title: web-demo
note: >
  The published browser demo becomes the project's front door: the page says what the
  dialect adds to markdown, every claim it makes is a snippet the workspace suite
  compiles, and one click sets that snippet as a PDF in the reader's own browser.
status: accepted
last_updated: 2026-08-21

phases:
  - name: "Phase 1 — the page says what the dialect adds"
    reviewed: 2026-08-19
    shipped: 2026-08-21
    cut: null
    by: null
  - name: "Phase 2 — every example is one click from a PDF"
    reviewed: 2026-08-21
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — a page-owned image, so the figure examples show one"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-002, mpdf-003, mpdf-005]
reference: >
  Typst's own web app is the two-pane shape `mpdf-003` already named as inspiration,
  and it is the inspiration here for nothing else: it is a hosted editor with accounts
  and projects, and `mpdf-001` §1.1 refuses servers permanently. The comparison this
  page draws — the same source under an ordinary markdown renderer — is the argument
  rather than a borrowing, and no part of such a renderer is adopted.
---

# web-demo

## 1. Goal

**Give the dialect a front door that shows itself.** `web/index.html` has been live at
`https://ivapo.github.io/md2pdf/` since the spike landed, and it is a textarea beside a
PDF pane with no word on the page about why the markdown in that textarea is worth
writing. A visitor who arrives from the README meets a box that makes PDFs and no reason
to prefer it to any other thing that makes PDFs.

**The observable is unchanged — the typeset PDF that Typst compiles from the user's
markdown — and this spec builds no new one.** What it builds is the first place a reader
meets that observable without installing anything: `web/src/lib.rs:render` already calls
`core/src/lib.rs:md_to_pdf`, so the PDF a visitor sees here is the PDF the CLI writes,
from the same crate, byte for byte. §4 holds each phase to that claim rather than to a
page that merely describes it.

The consumer is someone who has never heard of the project. Today they see this:

```
┌──────────────────────────────────────────────────────────┐
│ md2pdf — md2pdf-core compiled to wasm32, converting …    │
├──────────────────────────────────────────────────────────┤
│ booting…                                                 │
├───────────────────────────┬──────────────────────────────┤
│ [ a sample document ]     │  [ the PDF ]                 │
└───────────────────────────┴──────────────────────────────┘
```

After this spec they see the same two panes under an argument, and every claim in that
argument is a button:

```
┌──────────────────────────────────────────────────────────┐
│ md2pdf — one markdown file in, one typeset PDF out.      │
│ Everything below compiles in this page. No server.       │
├──────────────────────────────────────────────────────────┤
│ WHAT THIS ADDS TO MARKDOWN                               │
│                                                          │
│ A caption makes a figure, and numbers it   [ load it ▸ ] │
│   md2pdf                    an ordinary renderer         │
│   ┌─────────────────────┐   ┌─────────────────────────┐  │
│   │ | a | b |           │   │ a table, then a         │  │
│   │ |---|---|           │   │ paragraph reading       │  │
│   │ | 1 | 2 |           │   │ ": The measurements."   │  │
│   │                     │   │                         │  │
│   │ : The measurements. │   │                         │  │
│   └─────────────────────┘   └─────────────────────────┘  │
│   → Table 1, captioned beneath, counted with the rest    │
│ …                                                        │
├───────────────────────────┬──────────────────────────────┤
│ [ the example, loaded ]   │  [ its PDF ]                 │
└───────────────────────────┴──────────────────────────────┘
```

**The argument the page makes is that the difference is visible in one click**, which is
why the comparison is not left at two code blocks: the right-hand column of every row is
what an ordinary renderer does with the identical source, and the button is what settles
it. A reader who does not click still gets the whole list; §4's Phase 1 ships exactly
that, and Phase 2 makes it live.

### 1.1 Why this is a new spec and not a phase of an existing one

The methodology's §6.1 is an ordered test, and it is worked in full. **This is the rare
case where two shipped specs already named the answer.**

- **Step 0 — does this change a decision, or only the code?** A decision, and the code
  says so in three places. `web/Cargo.toml` opens "A spike, not a front end… It makes no
  design claim — the browser front end is a spec nobody has written". `web/src/lib.rs`'s
  module doc calls the missing image channel "a design question for the spec that
  eventually owns it". `web/index.html`'s script comment says that file story "belongs to
  a spec nobody has written". Promoting a spike to a front door is the decision those
  three comments were holding open, and this spec is the one they were held open for.
- **Step 1 — does it remove or contradict shipped work?** **No, and both of the specs it
  touches name their own successor.** `mpdf-001` §1.1 parks "a WASM browser build" for
  later specs. `mpdf-003` §1.1 goes further and names the shape of the successor: "**Not
  the browser build.** `mpdf-001` §1.1 parks a `wasm32` build, and it stays parked. It is
  a different front end with a different file story, so it is a later spec rather than a
  phase here." Work that says *a later spec* is not work a later spec removes.
- **Step 2 — is the subject one an existing spec owns?** **No, and both candidates
  disown it by name.** `mpdf-001` owns the pipeline and the CLI; `mpdf-003` owns the
  desktop app and the sentence above. A third front end is neither.
- **Step 3 — is it a named kind under a framework an existing spec reserved?** **No, and
  this is the step worth being careful at**, because "front ends" reads like a framework
  with three kinds. It is not one. A reserved framework under §2 is a shared design with
  a declared contract its siblings implement — `core`'s call contract is that shape and
  the looks are its kinds. Nothing reserves a *front end* framework: `cli/src/main.rs`
  and the Tauri app share `core`'s API and nothing else, having no common surface, no
  common file story and no common lifecycle. So `extends` stays `null`.
- **Step 4 — a new spec.** With the corpus's own parking notes as the argument.

**The subject is the page, not the prose on it.** A reader could object that a page
explaining the dialect is documentation, and documentation is a close-out obligation
under §6 rather than a subject with a spec. That objection is right about the words and
wrong about the artifact: this page carries a 25.7 MB WebAssembly module, a deploy
workflow, a browser-support claim, an image story two specs parked, and a failure mode —
a page asserting something the compiler refuses — that no README has. §2 is about those,
and the prose is the easy half.

### 1.2 Non-goals

- **No server, no accounts, no persistence.** `mpdf-001` §1.1 refuses servers
  permanently, and Pages serves static files. Nothing here is a step toward one.
- **No editor.** The textarea stays a textarea. Syntax highlighting, a file tree, a
  share link and anything that would make this a hosted editor are out; `mpdf-003` is
  where authoring lives.
- **The user's own image files stay parked.** Phase 3 bundles *one page-owned* image so
  the figure examples can show the flagship case. A browser has no filesystem, and the
  file story `mpdf-001` §1.1 named — a user's own images reaching the compiler — is not
  opened here. §2 records the line between the two.
- **The page is not the documentation.** The README stays the reference a reader is sent
  to; the page carries a chosen few of the dialect's constructs and links out. The count —
  **twenty-two supported constructs** — is `rules/pipeline.md`'s, which enumerates them.
- **No new dialect, and no change to `core`'s behaviour.** Every example is markdown the
  shipped compiler already accepts. Phase 1 adds one Rust *test* and edits one Rust *doc
  comment*, and neither changes what the compiler does; no phase here touches `core/src`.
- **Not a second look.** How the demo's PDF is styled is `template.typ`'s, unchanged.

## 2. Design

### One page, and its text does not wait on the module (decision, recorded)

`web/Cargo.toml` records the measurement: **25.7 MB raw, 7.8 MB brotli, on 2026-08-15**,
of which 2.5 MB is fonts — `core/assets/fonts` measures that exactly. The raw figure is
the one to treat as approximate: the module built that day and left in the tree at
`web/pkg/md2pdf_web_spike_bg.wasm` is 25,316,809 bytes, so the header rounds a decimal
25.3 MB up. Nothing in this spec is keyed to either number; they size the problem and
that is all. That is the cost of the pane, and today the page has nothing else to show
while it is paid — `#status` reads "booting…" and both panes are empty.

A front door that spends 7.8 MB before its first sentence is a bad front door, and the
obvious fix — a text-only landing page that links to a separate `/play` — is the wrong
one: it puts the argument and the proof on two URLs, so the click that settles the
argument becomes a navigation that reloads and re-pays. **So: one page, the text first in
the document and rendered without the module, the module fetched as it already is —
`<script type="module">` is async by definition — and the example buttons inert until it
resolves.** The reader reads the list while the module lands, which is exactly the time
they will spend reading it.

**The status line changes job.** Today it is the spike's instrument panel and reports the
boot. It becomes the compile's own line — the error a refusal names, and nothing when the
compile succeeded — with readiness carried by the buttons instead, which is where a
reader will look for it. *(Which phase owns that, and what becomes of the measurement it
stops printing, is settled below under "A refusal clicked is not a refusal typed": it is
Phase 2's, and Phase 1 left the line alone.)*

### What the page claims is what the compiler does, and one test holds them together (decision, recorded)

**The failure this design exists to prevent is a page that shows markdown the compiler
refuses.** It is the failure most likely to happen and least likely to be noticed: the
dialect refuses raw HTML and a task list wholesale through `core/src/emit.rs:describe`,
refuses the image destination shapes `core/src/emit.rs:check_image` lists, refuses eight
group shapes, and refuses every LaTeX command off `core/src/math.rs:COMMANDS`. A snippet
typed into a landing page by hand is one edit away from any of them, and the page would
still look right.

So the examples are not typed into prose. **Each is one element in `web/index.html`:**

```html
<script type="text/markdown" data-example="caption-table" data-expect="ok">| a | b |
|---|---|
| 1 | 2 |

: The measurements.</script>
```

**and two consumers read that same element** — the page, through
`document.querySelectorAll`, and a test, through `include_str!`. The element's content is
raw text to an HTML parser, so markdown inside one needs no escaping, and a `<script>`
block of a non-JavaScript type is not executed.

**The content begins immediately after the `>` and ends immediately before the
`</script>`, flush left, with no leading and no trailing newline.** That is a rule about
bytes and it is load-bearing, not tidiness: **an indented example is a different
document, and one that still compiles.** Measured against the built CLI on 2026-08-19, at
two spaces of indent the frontmatter example stops being frontmatter and reaches the page
as a setext heading over prose, and the caption example keeps its table but emits
`: The measurements.` as literal text instead of making Table 1 — and `md_to_pdf` returns
`Ok` for both. A gate that asked only "does it compile" would pass while the page's own
comparison column described what the reader was looking at. A single leading newline is
the same hazard one line further on: it moves every refusal's `at line N` by one, and the
refusal assertion below is keyed to that number.

The rule needs no stripping step in either consumer, which is why it is written this way
rather than as "strip one leading newline": with nothing to strip, `textContent` in the
page and the extracted slice in the test are the same bytes by construction, and the two
cannot drift apart by implementing the same normalisation differently.

**The test enforces the rule rather than asking for it.** It asserts, per example, that
the content's first character is not whitespace, that it neither begins nor ends with a
newline, and that no line within it begins with a space or a tab — so an editor or an
HTML formatter that re-indents the file fails the suite instead of quietly changing what
the page claims. The last of those three constrains which examples the page may carry,
deliberately: none of Phase 1's ten needs an indented line, and one that did would have to
argue for a narrower check. **The two rows carrying code are where that bites**, and the
fixture an implementer reaches for first is the one that breaks it —
`tests/fixtures/captioned_blocks.md` indents its `println!` by four spaces. Those rows
take a body-less function instead, which is not a workaround: the row exists to show a
caption attaching to a listing, and the listing's contents are not the subject.

**`data-expect` is the mark, and a refusal's expected sentence lives in the visible
prose.** `data-expect="ok"` asserts `core/src/lib.rs:md_to_pdf` returns `Ok`.
`data-expect="error"` asserts it returns `Err`, and that the error's `to_string()` equals
the text of the row's `<code data-error-for="…">` element, character for character. **The
checked sentence is the one the reader sees**, not a copy of it in an attribute: an
attribute would let the gate prove agreement between the compiler and a string nobody
reads while the printed prose said something else, which voids the only argument this
phase makes for its own existence.

The test is `core/tests/golden_test.rs`'s neighbour and reads the page the way that file
already reads its fixtures — `include_str!("../../web/index.html")`, the same relative
hop as `include_str!("../../tests/fixtures/basic.md")`. It scans for the markers with
plain string operations: **no HTML parser and no JSON dependency enters the workspace for
this**, because every marker is a fixed literal and each region ends at the next closing
tag. **Those two scans do not rest on the same thing, and the difference is worth
knowing.** A `<script>` holds raw text, so scanning to `</script` is exactly what an HTML
parser would do. A `<code>` holds parsed markup, so its raw slice equals its rendered text
only while the sentence inside needs no character reference — true of all three of Phase
1's, which are plain ASCII. A later message carrying a `<` or an `&` would fail the suite
loudly rather than pass wrongly, so the convention is safe to hold and worth stating
rather than discovering.

**That test is a workspace test over a file outside the workspace, and that is the point.**
`web/` is deliberately not a workspace member — `web/Cargo.toml`'s empty `[workspace]`
table detaches it so that `cargo test --workspace` behaves as it did before the directory
existed. Reading a file is not membership: `include_str!` compiles the bytes in, `web/`
stays out of the build graph, and the exit gate every phase already runs is what catches
a page that lies.

### The source column is that same element, made visible by CSS (decision, recorded)

**A `<script>` element is `display: none` in every UA stylesheet**, so the element above
is invisible as it stands — and the left half of every comparison row, the md2pdf source
in §1's sketch, is exactly what it holds. Left there, the page's argument is half missing
for a reader with JavaScript disabled, which Phase 1's own exit gate refuses.

Three resolutions exist and they are not equivalent. A visible `<pre>` duplicate beside
the script element is the reflex, and it is the one to refuse: it puts every example on
the page twice, so the copy the reader reads and the copy the test checks can differ,
which is the failure this whole section exists to prevent. Accepting an invisible source
gives up the comparison. **So: CSS.** `script[data-example] { display: block; white-space:
pre; }` overrides the UA rule, and the element renders its own text as the source block —
one copy, read by the reader, the page and the test alike.

**The technique is unusual enough to be verified rather than assumed**, which is why
Phase 1's gate loads the page with JavaScript disabled and reads the list: an override
that did not take is visible immediately, and it is visible to a second person following
the gate.

### The comparison column is written, not rendered (decision, recorded)

Each row shows the same source twice: what `md2pdf` does with it, and what an ordinary
markdown renderer does with it. The second could be produced live by vendoring a
JavaScript markdown renderer into the page.

**It is written by hand instead**, for two reasons and one of them is fatal. The fatal
one: the page's whole claim is that one WebAssembly module compiled from this repository
is doing the work, and shipping a second renderer to argue against would put an
unaudited, unversioned dependency on the one page making that claim. The other: there is
no such thing as *an ordinary renderer* — GitHub, CommonMark and a static-site generator
disagree, so a live column would assert one implementation's behaviour as universal. A
written column can say **what every renderer with no notion of the construct does**, which
is the honest claim and the one that holds: it passes the marker through as text.

The column therefore describes rather than renders — *"a table, then a paragraph reading
`: The measurements.`"* — and its wording is reviewed as prose. Where a claim is
contested, the row narrows it to CommonMark plus GFM, which is the dialect's own baseline
through `core/src/emit.rs:options`.

### Three kinds of difference, and the refusals are one of them (decision, recorded)

The list is grouped, because the differences are not all the same *sort* of difference
and a flat list would imply they are.

1. **Syntax an ordinary renderer passes through as text.** The strongest rows, because
   the comparison is visible without a PDF: the `: ` caption line that makes a Figure, a
   Table or a Listing and numbers it (`core/src/emit.rs:caption_marker`); the `:::` group
   that makes several of them one figure; the `{#name}` on a caption and the `[](#name)`
   that points at it; and `$…$` / `$$…$$` math converted through `core/src/math.rs:convert`.
2. **Things markdown has no way to say at all.** The frontmatter's six keys, read by
   `core/src/frontmatter.rs` — `title`, `author`, `date`, `template`, `columns`,
   `equations` — the two looks, one or two columns, `equations: numbered`, and footnotes
   that land at the foot of the *page* rather than the end of the document, which is
   Typst's placement and not a renderer's.
3. **What it refuses, on purpose.** Raw HTML and a task list, named with their line by
   `core/src/emit.rs:describe`; a LaTeX command off the list, named by `Error::Math`. A
   showcase that hides its refusals is selling something, and the refusal *is* the
   feature — `web/src/lib.rs:render` maps the error through the same `Display` the CLI
   prints, so the sentence in the page is the sentence at the terminal. This is the group
   the test in §2 checks hardest, because its rows assert an exact message.

**Twenty-two constructs are supported and the page shows nowhere near that many.** The
rows are chosen for what a reader cannot get elsewhere; ordinary emphasis, lists and
links appear only inside other examples. The README is the complete reference and every
group links to it.

### A refusal clicked is not a refusal typed, and the status line is Phase 2's (decision, recorded)

Two of the sentences above were written before Phase 1 existed and describe a page that
was never built. Phase 1 shipped on 2026-08-21 and changed neither, deliberately — its
scope kept the module script's behaviour — so both land on **Phase 2**, which is the phase
that adds the buttons and therefore the phase §2's readiness argument was always about.
This section settles them, and Phase 2's scope and gate below carry them.

**The status line's job change is Phase 2's, and the boot measurement is not deleted with
it.** "It becomes the compile's own line… with readiness carried by the buttons instead"
above names no phase, and today `web/index.html`'s `compile` appends the `boot` string to
*every* status write, success and failure alike — so **there is no place where `#status`
stops reporting the boot**, and a phase keyed to that place is keyed to nothing. Phase 2
makes it true: the line carries the compile alone — the sentence a refusal names, and
nothing at all when the compile succeeded — and readiness moves to the buttons, which are
inert until the module resolves. The instrument panel is the spike's, and a visitor has no
use for `anchors 7:1 14:1 22:1`; but the wire measurement is the live answer to one of the
three questions the spike exists to ask, so **it moves to `console.log` rather than being
removed** — the person asking that question opens a console and the reader does not.

**A refusal shows where the PDF would be, and only when it was asked for.** The claim that
"that path already works" is false as shipped: `compile`'s catch branch writes `#status` —
which sits *above* the panes — and never calls `draw`, so **the previous example's PDF
stays in the pane**. A reader who clicks the raw-HTML row would get the refusal's sentence
above a rendered page from the row before it, which is a page asserting something false
about its own output: the exact failure §2 exists to prevent, arrived at from the other
direction.

The fix is not to delete the shipped behaviour, because the argument for it is sound and
is about a different act. **Typing and clicking are different acts and get different
answers.** An author mid-edit passes through broken states constantly and wants the last
good page kept — that is `mpdf-003`'s behaviour, it is why the comment in `compile` is
there, and Phase 2 does not touch it. A reader who clicks *load it* on a row captioned
"what it refuses, on purpose" has asked to be shown a refusal, and answering with the
previous row's PDF answers a question nobody asked. So a click clears the pane before it
compiles, and a refusal reached that way leaves it clear with the sentence standing where
the PDF would be. **The button owns that, not `compile`** — which is what keeps the two
acts apart in the code as well as in the argument.

### No image crosses the boundary yet, and what that costs the examples (decision, recorded)

`web/src/lib.rs:render` calls `md_to_pdf(markdown, &[])` — **an empty asset slice**, so an
`![…](path)` in the textarea today reaches `core/src/lib.rs:collect` and comes back
`Error::MissingImage`. That is the spike's stated limit, and it lands on this spec
directly: **the flagship caption example in the README is an image, and the demo cannot
run it.**

Phases 1 and 2 take the constraint rather than fight it. A caption attaches to three
constructs, and two of them need no file at all: the rows use a **table** and a **fenced
code block**, which produce *Table 1* and *Listing 1* through the same mechanism and the
same counter behaviour. The group row uses two of those as members. Nothing in the
caption story goes unshown except the image itself.

Phase 3 then closes it with the narrowest possible opening: **one image, owned by the
page, not by the reader.** The page carries the bytes it already can — `samples/` holds
`pipeline.svg` and `check.svg` — and passes them under a fixed name through a new
`web/src/lib.rs` entry point beside `render`. **The line this does not cross is the one
`mpdf-001` §1.1 parked:** a *user's own* files reaching the compiler needs a picker, a
persistence story and a shopping list read from `core/src/lib.rs:image_paths`, and none of
that is here. A reader who types their own `![…]` still gets `Error::MissingImage`, and
after Phase 3 the page says so beside the box rather than letting them find out.

### The spike's disclaimers come down, and the one property that stays (decision, recorded)

Three comments say `web/` makes no design claim, and after Phase 1 that is false. They are
corrected in place — `web/Cargo.toml`'s header, `web/src/lib.rs`'s module doc and
`web/index.html`'s script comment — to name this spec as the one that owns the directory.
`web/src/lib.rs`'s note that the image channel is "a design question for the spec that
eventually owns it" is corrected in the phase that answers it, which is Phase 3 and not
before. `.github/workflows/pages.yml` needs no correction: it documents the build and
says nothing this spec makes false.

**One property is load-bearing and does not move: `web/` stays out of the workspace.**
The empty `[workspace]` table in `web/Cargo.toml` is what keeps `cargo build --workspace`
and `cargo test --workspace` — every phase's exit gate, in this spec and in the four
before it — from acquiring a five-minute wasm build. Owning the directory changes what it
is for, not what it costs the suite.

## 3. Open questions

- **OQ-1 — does the comparison column render live?** *(design call)* **RESOLVED
  2026-08-19: no, written by hand.** The argument is above: a second renderer on the one
  page claiming a single module does the work, and no implementation entitled to be
  called *ordinary*.
- **OQ-2 — one page or a landing page plus a `/play`?** *(design call)* **RESOLVED
  2026-08-19: one page, text first, module async.** Two URLs turn the click that settles
  the argument into a navigation that re-pays 7.8 MB.
- **OQ-3 — does Phase 3 happen at all?** *(needs-input)* It opens the narrowest slice of
  a story two specs parked, and Phases 1 and 2 stand without it — the caption rows work
  over a table and a listing. Left open deliberately: the phase is written so it can be
  cut with `cut` + `by: mpdf-006` and lose nothing already shipped.
- **OQ-4 — which sample image, if Phase 3 runs?** *(deferred by evidence)* `pipeline.svg`
  is the README's own flagship and an SVG costs bytes measured in kilobytes against a
  7.8 MB module, but neither file has been measured in the page. Phase 3 measures before
  choosing.
- **OQ-5 — does the page state browser support?** *(design call, open)* The spike exists
  partly to answer whether Safari agrees with Chromium, and the answer is not written
  down anywhere this spec can cite. Phase 1 makes no claim on the page; whoever runs the
  check records it and a row may follow.

## 4. Implementation phases

Strictly sequential. Phase 2 needs the elements Phase 1 introduces; Phase 3 needs the
button Phase 2 wires.

### Phase 1 — the page says what the dialect adds

*Produces the observable: **no**, and it is argued rather than assumed.* This phase ships
prose, code blocks and a written comparison; the PDF pane still works exactly as it does
today, but nothing in this phase compiles anything new. It is the one phase in the spec
whose output a reader could get from a README. **It earns its place as the phase that
puts the examples under test** — the `data-example` elements and the test that checks them
land here, so every later phase inherits a page that cannot claim what the compiler
refuses. Phase 2 is what makes the page produce the observable, and it is one phase away.

- **Scope:** four files. `web/index.html` carries the work; `core/tests/page_examples_test.rs`
  is new; `web/Cargo.toml`'s header comment and `web/src/lib.rs`'s module doc are the two
  spike disclaimers outside the page, corrected to name `mpdf-006` (the third is in the
  page itself). Add the three groups of §2 with their rows; each row is a heading, one
  sentence, a `data-example` script element holding the source under §2's byte rule, a
  written "an ordinary renderer" column, and one sentence on what the PDF does — and, on a
  refusal row, the `<code data-error-for="…">` element holding the sentence the compiler
  prints. **Ten examples**, and they are the ones §2 names: caption over a table, caption
  over a fenced block, a `:::` group over two members, `{#name}` + `[](#name)`, `$$…$$`,
  the six frontmatter keys, a footnote, and three refusals. The list sits above the
  existing two panes. **The page's height model changes and that is inside this scope**:
  `body` is `height: 100dvh` over `grid-template-rows: auto auto 1fr` today, which gives
  the panes the viewport and leaves a long section above them nowhere to go, so the page
  becomes one that scrolls with the panes sized beneath the list. The textarea, the iframe
  and the module script keep their behaviour; what changes around them is layout.
- **Exit gate:** `cargo test --workspace` passes, including a new
  `core/tests/page_examples_test.rs` that `include_str!`s `web/index.html` and, for every
  `data-example` element: asserts the content obeys §2's byte rule — first character not
  whitespace, no leading or trailing newline, no line beginning with a space or a tab —
  then asserts `data-expect="ok"` returns `Ok` from `core/src/lib.rs:md_to_pdf`, and
  `data-expect="error"` returns `Err` whose `to_string()` equals the text of the matching
  `<code data-error-for="…">` element, character for character. **The count of examples
  found is asserted to be exactly 10**, so an element that stops matching the marker fails
  the suite rather than silently leaving it. Separately: the page loads with JavaScript
  disabled and the whole list — both columns of every row — is readable, which is what
  checks §2's CSS override.
- **Close-out:** no `rules/` file today covers `web/`; this phase seeds
  `rules/web-demo.md` declaring `web/index.html`, `web/src/lib.rs` and
  `.github/workflows/pages.yml` as its sources. README gains a line pointing at the demo.
  One push; commits as the work wants.

### Phase 2 — every example is one click from a PDF

*Produces the observable: **yes**.* A click sets the example into the textarea and the
existing pipeline compiles it to a PDF in the pane — `web/src/lib.rs:render` over
`core/src/lib.rs:md_to_pdf`, the same bytes the CLI writes.

- **Scope:** `web/index.html` only, and no Rust. Ten rows, so ten buttons. Each reads its
  own row's element — `row.querySelector('script[data-example]')`, and **the button must
  not carry a `data-example` attribute of its own**: `core/tests/page_examples_test.rs`
  asserts the page holds exactly ten of that attribute, so a button spelled that way fails
  the suite. Writing `textarea.value` **fires no `input` event**, so the handler calls
  `compile` itself rather than relying on the 300 ms debounce the typing path uses. The
  pane is scrolled to on click, since the rows sit above it.
  - **Readiness is the buttons'.** They carry `disabled` in the markup and are enabled
    once `await mod.default()` resolves — which is also where `#status` takes up its new
    job, per §2: the compile's line alone, nothing on success, and the boot measurement
    moved to `console.log`.
  - **A click clears the pane before it compiles**, so a refusal leaves it clear with the
    sentence standing where the PDF would be. `compile`'s own catch branch keeps the last
    good page and is not touched — §2 records why the two acts differ.
  - Phase 1's `<noscript>` line — "The two panes at the foot of this page need JavaScript.
    Everything above them does not" — stops being true when the buttons land, and is
    corrected in the same phase.
  - Button placement and label are the phase's call; §1's sketch shows `[ load it ▸ ]` on
    the row's heading line, and `.row > h3` has no slot for one today.
  - Two consequences of the status line's new job, both the implementer's to settle and
    neither needing Rust: an empty `#status` still paints its padding and its bottom rule,
    and the page stops calling `web/src/lib.rs:anchors` — which is `mpdf-003` Phase 6's
    export answered in a browser, so **the export stays** whatever the page does with it.
- **Exit gate:** `cargo test --workspace` still passes — including
  `core/tests/page_examples_test.rs`, which the button markup can break and which is the
  cheapest signal that it did. In a browser: **before the module resolves every button is
  inert, and after it resolves every button is live** — the one new stateful behaviour, and
  the one a screenshot cannot show. Then each of the **seven** `ok` rows loads its source
  and draws a PDF, and each of the **three** refusal rows empties the pane and prints the
  exact sentence beside it. (Ten rows; `data-expect` is what says which is which, and
  `core/tests/page_examples_test.rs` pins the total at ten and ties every refusal to the
  sentence printed beside it — it does not pin the seven-and-three split, so that count is
  re-derived from the page rather than asserted anywhere.)
  - **The gate needs a build, and the recipe is not in the repo's prose.** `web/pkg/` is
    gitignored and the module is never committed, so a second person runs
    `wasm-pack build --target web --release` in `web/` and serves the directory over HTTP —
    an ES-module import fails from `file://`, which is why Phase 1's no-JS check needed no
    module and this one does.
  - Run in Chromium **and** Safari. That produces the evidence OQ-5 wants and does not
    settle it: whether the *page states* browser support is a design call, and adding a row
    for it is **out of this phase's scope**.
- **Close-out:** `rules/web-demo.md` regenerated, and **the Safari/Chromium result lands
  in it** — Phase 2's gate is the only thing that produces that fact, and a measurement
  nothing records is one the next phase re-runs. It goes there rather than in the review
  record because it is a fact about how the published page behaves, which is what a
  `rules/` file is for; the review record answers what a review found. One push.

### Phase 3 — a page-owned image, so the figure examples show one

*Produces the observable: **yes**.* The caption-over-an-image case — the README's own
flagship — reaches the pane for the first time. **This phase is cuttable** (OQ-3): nothing
in Phases 1 or 2 depends on it.

- **Scope:** a new entry point in `web/src/lib.rs` beside `render`, taking the page's
  fixed asset bytes and calling `md_to_pdf` with a one-element slice instead of `&[]`;
  the page carries one image from `samples/`, measured first (OQ-4); one new row using it;
  and a sentence beside the textarea saying a reader's own `![…]` cannot be read here, so
  the `Error::MissingImage` a visitor may hit is explained before they hit it. `core` is
  not touched — `md_to_pdf` already takes assets.
- **Exit gate:** `cargo test --workspace` passes with the new row under the Phase 1 test,
  which needs the asset to reach `md_to_pdf` there too — so the test grows an asset
  channel of its own or the row is excluded by name, and the phase picks one deliberately
  rather than by accident. In a browser, the row produces a PDF with the image and its
  numbered caption.
- **Close-out:** `rules/web-demo.md` regenerated; `web/src/lib.rs`'s module doc corrected
  in place, since this is the phase that answers the question it parks. One push.
