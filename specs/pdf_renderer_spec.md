---
id: mpdf-009
title: pdf-renderer
note: >
  The app draws the page itself: `pdf.js` is vendored as two static modules and
  rasterises each page onto a canvas the pane owns, so fit-to-width is a mode
  rather than a transform, the type is sharp at the display's own resolution,
  and the text and links come back with it.
status: draft
last_updated: 2026-08-25

phases:
  - name: "Phase 1 — the page is drawn here"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — the text it can select and the links it can follow"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — the zoom the reader owns"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-003, mpdf-006]
reference: >
  `armquill`, this author's own cloud editor, is where the shape was read from:
  its viewer takes a `fitMode` of `page | width | manual` and derives the render
  scale per page from the container's width, which is the design this spec
  takes. Its collaborative editing, its server-side compile and its file
  explorer are out of scope permanently — `mpdf-001` §2 keeps this app local and
  fetching nothing. Mozilla's `pdf.js` is the renderer itself, Apache-2.0, whose
  licence and NOTICE travel with the vendored files.
---

# pdf renderer

## 1. Goal

**Draw the artifact in a surface this project owns.** The observable is
unchanged and stays `mpdf-001`'s — the typeset PDF that Typst compiles — and
what changes is who rasterises it. Since `mpdf-003` Phase 1 the pane has handed
the bytes to WebKit and let WebKit build a PDF view inside an iframe. That view
is good and it is not ours: it cannot be asked to re-fit, it reports nothing
about where the reader is, it has no zoom this app can reach, and on a webview
that ships no PDF viewer at all it does not exist.

The consumer is the same author. Today they drag the divider and the page they
are reading is held at the right width by a transform over a raster made for a
different one, softening until something re-mints it — `mpdf-003` Phase 7 is
that mechanism, and this spec is the reason it was written knowing it was
temporary.

### 1.1 Why this is a new spec and not a phase of `mpdf-003`

§6.1's ordered test, worked:

- **Step 0 — does this change a decision?** Yes. `mpdf-003` §2 decided that the
  pane "draws the artifact, not a picture of it", that "no JavaScript PDF viewer
  is bundled and none is wanted", and Phase 7 decided that a width that changes
  is answered by a transform. This reverses all three.
- **Step 1 — does it remove or contradict shipped work?** **Yes, and it is
  `mpdf-003` Phase 1 that makes it so** — corrected in round 1, which found the
  draft resting this on Phase 7 instead. Phase 1 shipped on 2026-08-10 and
  decided that the pane hands its bytes to WebKit through a `blob:` URL in an
  iframe, under a constraint it stated as **no bundled JavaScript PDF viewer**.
  This spec bundles one. That is a contradiction of shipped work, step 1
  matches, and a phase of `mpdf-003` is therefore not available however the
  rest of the argument goes.

  **`mpdf-003` Phase 7 is a separate question and is not what makes this a new
  spec.** Its `shipped` and `reviewed` are both `null` and no round for it
  exists in `specs/reviews/mpdf-003.md`; its mechanism is on `main` as
  prototype, labelled so in `app/dist/index.html`. The draft called it shipped
  work, which the record contradicts. What is true of it is narrower and still
  matters: it is the one phase of `mpdf-003` whose *whole subject* this spec
  replaces, which is why it is the phase the edge names.
- **Steps 2–4 are not reached**, step 1 having matched. The edge is
  `supersedes: [{id: mpdf-003, phases: ["Phase 7 — the page fits the pane it is
  given"]}]`, against which `mpdf-003` sets `cut` and `by: mpdf-009` on that
  phase, keeps `status: accepted`, and has its rollup become `partial` by rule
  4 once the cut lands, which is the moment this clause describes. It already
  reads `partial` today for a different reason — Phases 7 and 8 being unshipped
  — and round 2 corrected round 1's correction here.

  **The edge is not in this document's frontmatter yet, and that is
  deliberate.** It is declared when the cut happens — on Phase 1 shipping —
  because the inverse of `supersedes` is a `cut` date on the phase it removes,
  and `mpdf-003` Phase 7's mechanism is on `main` and running while this is a
  draft. Writing the edge now would put a removal date on work that has not
  been removed, and a draft that is withdrawn would have to take it back out of
  a spec it never replaced. §1.1's argument is the record until then; the
  frontmatter follows the code.

**What Phase 7 is kept for.** Its findings are this spec's evidence and are not
superseded with its mechanism: that `view=FitH` is read once at load, that a
redraw cannot restore a reader's place because WebKit leaks none, and that a
pane's geometry must be observed rather than inferred from the events believed
to change it. The third outlives the renderer entirely and this spec inherits
it.

### 1.2 Non-goals

- **Not reverse sync.** A click on the page reporting the source line it came
  from is the feature this most obviously unlocks, and it is a different
  subject: the click is the easy half, and turning a page coordinate into a
  markdown line needs a position map through the generated Typst — the general
  answer `mpdf-003` Phase 6 §2 rejected in favour of the ordinal heading
  correspondence it shipped. Round 1 caught the draft citing `mpdf-003` OQ-7
  for this, which is resolved and asks the opposite direction. A later spec.
- **Not the port.** This removes the reason Windows and Linux are blocked; it
  does not attempt either, and `mpdf-003` OQ-9 stays open until there are
  machines to test on. Nothing here may be designed for a platform this project
  cannot run.
- **Not a viewer.** No thumbnail rail, no presentation mode, no search, no
  printing, no annotation editing. The pane shows one document at one width and
  the reader scrolls it.
- **Not a change to the dialect, the pipeline, or `core`.** `core` gains
  nothing. The bytes this renders are the bytes `md_to_pdf_with_anchors`
  already returns.
- **Not the web demo.** `mpdf-006` publishes its PDF by handing the browser a
  blob, and the browser there is the reader's own. Out of scope; `mpdf-006`
  stays as it is.

## 2. Design

### The renderer is vendored, not built (decision, recorded)

Two files copied into `app/dist/pdfjs/`: `pdf.min.mjs` and `pdf.worker.min.mjs`,
1,717,067 bytes together — 1.72 MB decimal, 1.64 MiB, and round 1 re-derived
both. **No npm at build time, no bundler, no node** — which
is the whole of what `mpdf-003` OQ-2's `withGlobalTauri` bought and what this
spec must not spend. `pdfjs-dist` ships browser-ready ES modules precisely so
they can be loaded from a `<script type="module">`.

**Measured, and re-measured after round 1 found the first attempt taken in the
wrong place.** The draft's 26–31 ms was the *unminified* `pdf.mjs` over
`tauri dev`'s `http://127.0.0.1:1430` static server — not the file this
vendors, and not the scheme the app ships on. Retaken on 2026-08-25 against
`cargo run -p md2pdf-app`, whose assets are embedded by
`tauri::generate_context!` and served over the custom scheme, with the
**minified** build and the worker enabled:

| | |
|---|---|
| origin | `tauri://localhost`, protocol `tauri:` |
| `pdf.min.mjs` imported | 29 ms, version 6.2.108 |
| six-page showcase opened | 59 ms |
| worker | real, not the main-thread fallback |
| page 1 rendered | 42 ms at `devicePixelRatio` 2 |
| all six pages | 94 ms |
| `streamTextContent` | 190 items on page 1 |

No CSP change and no capability added: `app/tauri.conf.json` declares no
`security` key and `app/capabilities/default.json` carries only `core:default`
and the two dialogs. **The scheme was the risk and it is retired**; OQ-5 carries
the one thing that run did surface.

**The files are committed.** The repository refuses to commit `web/pkg/` — a
25.7 MB wasm module — and the reasoning there was that git history is permanent
and the artifact is enormous. This is fifteen times smaller and, unlike that
one, it is *required for the app to build at all*: fetching it in the build
would put a network and a package manager back in a path that is one Cargo
command. **`pdfjs-dist` 6.2.108 ships a `LICENSE` and no `NOTICE`** — round 1 checked,
against a draft that promised both — so the `LICENSE` travels beside the two
modules and `README.md`'s `## Licence` section, which already enumerates the
bundled fonts' licences, gains the Apache-2.0 entry.

### Fit-to-width is a scale, recomputed (decision, recorded)

`scale = paneWidth / page.getViewport({scale: 1}).width`, taken **on every
render**. A fit that survives a resize because it is not a state that can go
stale — it is an expression evaluated when the page is drawn.

**`paneWidth` is `clientWidth` of the scroll container the pages sit in, not of
the column around it.** Round 1 found the draft naming neither. The container is
a child of `#preview`, six A4 pages make it scroll, and on a machine set to
show classic scrollbars its content box is some 15 px narrower than the
column's — fifteen times the tolerance the gate asks for, and the difference
between a page that fits and a page clipped at its right edge. The canvases sit
flush to that container's content box; a phase that wants a gutter between
pages must change this sentence and the gate with it.

**The pane's geometry is observed and not inferred** — `mpdf-003` Phase 7's
rule survives its own mechanism, and a `ResizeObserver` on that container is
what triggers the re-render. Two bugs came from listing the causes of a resize
and both had one shape.

### A gesture is carried by a transform; a rest is answered by a render (decision, recorded)

**Round 1's sharpest finding, and it reverses part of what the draft claimed.**
The draft deleted "every line of `mpdf-003` Phase 7's mechanism" and left a
`ResizeObserver` wired straight to a full re-render. The divider's drag sets a
flex basis on every `pointermove`, so that observer fires once per frame, and
six pages cost 94 ms against a frame's 16.7 ms — every render cancelled by the
next, nothing completing until the hand comes off, and a cancelled render on a
just-resized canvas leaves it *cleared*, so the literal reading blanks the pane
through the drag. `mpdf-003` Phase 7 rejected redrawing per pointermove for
exactly this reason.

**So the split Phase 7 found is kept, and only its reason changes.** While a
width is changing the canvases are scaled by CSS — their backing stores
untouched, the browser resampling a bitmap it already has, which composites and
holds every frame. When the width rests, they are re-rendered at the new scale,
which is sharp because the raster is made for that width.

What is different is what the rest costs. Phase 7's redraw re-minted a blob and
so **could not restore the reader's place**, because WebKit leaks none — the
whole reason a drag put the reader on the caret's page. A renderer this project
owns re-rasterises into canvases it already positions, so the rest is invisible:
same scroll offset, same page, sharper type. **The mechanism looks like Phase
7's and costs nothing it cost.**

### Sharpness is the device's pixel ratio, and it is the thing a transform could
not do (decision, recorded)

The canvas backing store is `floor(viewport.width * devicePixelRatio)` by the
same in height, with the context transformed by that ratio and the CSS size
left at the logical one. **Measured: page 1 of the six-page showcase renders in
42 ms at `devicePixelRatio` 2**, and all six in 94 ms — the same run as the
table above, so the two figures are one measurement and not two. The draft
quoted 33–46 ms from the dev-server probe; round 1 caught that the per-page and
whole-document numbers were being read as if they described the same thing. A CSS transform can only resample a raster
that was already committed, which is what made `mpdf-003` Phase 7's scaled page
visibly soften and is the report that produced this spec.

### The worker is a second file and is not optional (decision, recorded)

`GlobalWorkerOptions.workerSrc` names `pdf.worker.min.mjs` and parsing and
rasterisation happen off the main thread. **Rasterising on the main thread
would block the window**, and this pane re-renders every time the typing stops,
so a compile would freeze the text the author is typing into. *This has not
been measured with the worker disabled*; it is a design call from where the
work sits, and a round may ask for the measurement.

### `getTextContent` is unusable in this webview (decision, recorded)

**Measured, and this is the trap an implementer would otherwise fall into.**
`pdfjs.PDFPageProxy.getTextContent` is a `for await (… of readableStream)` over
its own stream, and `ReadableStream.prototype[Symbol.asyncIterator]` is
`undefined` in WKWebView — Chromium and Gecko implement async iteration of a
readable stream and WebKit does not. The call rejects with *"undefined is not a
function"* from inside the minified bundle, which names nothing useful.

**`streamTextContent` read by hand works**, and is what Phase 2 uses: take
`getReader()`, loop on `read()` until `done`, collect `value.items`. Measured on
the same page: **190 items, the first of them `"Everything the Dialect
Carries"`**, which is that page's own heading.

### What is given up, stated plainly (decision, recorded)

`mpdf-003` §2 recorded that the frame "draws the artifact itself, not a picture
of it", and named three things that follow: a link stays a live annotation, the
text stays selectable, and the accessibility tagging survives. **The first two
are rebuilt by Phase 2 and the third is genuinely weaker.** WebKit's PDF view
exposes a tagged PDF's structure to the accessibility tree; a canvas plus a
positioned text layer does not, and `pdf.js`'s `getStructTree` is a partial
answer this spec does not pretend closes the gap. That is the price, it is paid
knowingly, and OQ-3 carries what could be done about it.

**Measured and regained beside it**: the annotation API answers, and
`getOutline` returns **six entries** for the showcase — Typst emits document
bookmarks that nothing in this project currently reads.

## 3. Open questions

- **OQ-1** — does the pane render every page, or the pages near the reader?
  Measured in the same `tauri://` run as §2, and not the retired dev-server
  one: six pages in 94 ms against 42 ms for the first, so the pages after it
  cost around 10 ms each and a document of this size needs no virtualisation and Phase 1 will not build
  any. A thesis will. The threshold is unmeasured and the answer is a page
  budget rather than a document one. *(deferred by evidence)*
- ~~**OQ-2** — what happens to the reader's scroll position across a
  re-render?~~ **RESOLVED 2026-08-25, in round 1**, which found the draft
  calling it resolved in Phase 1 while leaving it `needs-input` here and
  offering a recommendation in place of a decision. **The answer is by cause,
  and the causes are the ones `app/dist/index.html:refresh` already
  distinguishes:**

  1. **A compile that took a text from disk** — an open, or an external change
     over a clean buffer, which `refresh` already knows as `tookReload` — opens
     at **page 1**. Unchanged from `mpdf-003` Phase 6, which argues it: an open
     is not a cursor movement.
  2. **A compile after the author's own edit** opens at **the caret's page**,
     by `caretPage` and the anchors. Unchanged from Phase 6, whose whole
     subject this is.
  3. **A re-render caused by geometry alone** — a divider drag, a window
     resize, a control taking width out of the column — **restores where the
     reader was**, because no new bytes arrived and nothing about the document
     moved. This is the case that could not be served before and is the reason
     the question was open.

  **The restored quantity is a page index and a fraction within that page, not
  a pixel offset.** Round 1 caught the draft naming the raw `scrollTop`: the
  scale changes with the width, so every canvas's height and the container's
  `scrollHeight` change with it, and a pixel offset held across that moves the
  reader — and clamps at the bottom when a pane widens and then narrows. Taken
  before the re-render and reapplied after, an anchor of *(page, fraction)*
  survives both a scale change and a document that gained a page.
- **OQ-5** — does a window that is not frontmost render at all? Raised
  2026-08-25 by the re-measurement, which **stalled before its first render and
  sat for fourteen minutes** in a window that was behind another; the identical
  probe completed in 42 ms when the window was in front. One observation each
  way, unexplained, and not chased because it blocks nothing in Phase 1 —
  WebKit throttles occluded windows, and a preview that only stalls while
  nobody is looking at it may be correct behaviour. **It stops being harmless
  if the pane is still stale when the author comes back**, which is the thing
  to watch for in use. *(deferred by evidence)*
- **OQ-3** — what does a canvas cost a reader using a screen reader, and what
  can be given back? `pdf.js` exposes `getStructTree`, and Typst's own tagging
  is what would feed it. Unmeasured, and nobody has tested this app with a
  screen reader at any point. *(needs-input)*
- **OQ-4** — do `cmaps/`, `standard_fonts/` or `wasm/` ever need to travel?
  They are 1.6 MB, 800 KB and 1.5 MB and are for CJK encodings, non-embedded
  standard fonts and exotic image codecs respectively. Typst embeds its fonts,
  which is why they are omitted — but a document naming a CJK font, or an image
  in a format `pdf.js` decodes in wasm, would be the case that proves it wrong.
  The showcase does not exercise any of the three. *(deferred by evidence)*

## 4. Implementation phases

### Phase 1 — the page is drawn here
*Produces the observable: **yes** — the same PDF Typst compiled, rasterised by
this app instead of by WebKit, and fitted to the pane at every width.*

- **Scope:** Everything here is in **`app/dist/index.html`**, plus two files
  copied into `app/dist/pdfjs/`. No `.rs` file is edited.

  **Vendored first**: `pdf.min.mjs` and `pdf.worker.min.mjs` from `pdfjs-dist`
  6.2.108, with its `LICENSE`, into `app/dist/pdfjs/`. §2 argues the choice; the
  phase is where they arrive, and the draft named them only in §2 — round 1's
  catch.

  **The iframe and its blob go**, and a scroll container the pane owns takes
  their place with one canvas per page. The bytes reach it as they do now:
  `app/src/main.rs:current_pdf` already crosses raw as a `tauri::ipc::Response`,
  which reaches the page as an `ArrayBuffer` — `pdf.js` takes a `Uint8Array`
  over it, and **`getDocument` transfers that buffer to the worker**, so it
  cannot be read twice. `app/dist/index.html:refresh`'s `revision` guard, the
  payload-less `rendered` signal and `Status` are untouched.

  **The `PDFDocumentProxy` is retained between renders**, and a geometry-only
  re-render draws from it without a second `invoke`. This is not an
  optimisation: `current_pdf` refuses while `Preview::is_stale`, so a pane that
  re-fetched on resize could not re-fit a stale page — the one case §2's
  "a fit that cannot go stale" is strongest about. Round 1's catch.

  **The scale is §2's**, read from the scroll container, with the canvas backing
  store at `devicePixelRatio` and the context transformed to match. A
  `ResizeObserver` on that container drives it, per §2's gesture-versus-rest
  split: CSS-scale while the width is moving, re-render when it rests. A render
  in flight is cancelled before the next begins.

  **OQ-2's three cases are built as §3 resolves them**, including the
  *(page, fraction)* anchor for the geometry case.

  **What `mpdf-003` Phase 7 leaves behind is mostly *kept*, and this paragraph
  reads the opposite of the draft's because round 2 found the two irreconcilable.**
  §2's gesture-and-rest split *is* Phase 7's design; a phase that deleted
  Phase 7's parts and then asked for that behaviour would be asking an
  implementer to re-derive five things this spec already knows. What survives,
  by name:

  - **`fitted`** — the width the raster now in the canvases was made for. Kept;
    it is what tells a gesture from a rest.
  - **`box()`** — kept, reading the scroll container per §2 rather than the
    column it sits in.
  - **`fit()`** — kept as the gesture, but it sets each canvas's **CSS `width`
    and `height`**, where today it sets a `transform` and a `flex`. **Which
    property carries the gesture is a decision and not a style**: a transform
    leaves layout alone, so the container's `scrollHeight` would hold its
    pre-gesture extent and OQ-2 case 3's *(page, fraction)* anchor would be
    read against a stale one. CSS width makes the extent track the gesture. It
    is also what makes gate clause 3 discriminating — backing store and CSS
    size diverge during a gesture and agree again at rest, so a rest that never
    came is visible.
  - **`unscale()`** — kept, as what a completed render resets the canvases to.
  - **`settle()`** — kept **whole, both halves, at its 200 ms**, and the
    divider's `pointerup` keeps calling it. Phase 7's finding is untouched by
    this spec: *a drag ends on an event and a window resize does not*, so the
    timer is what gives the second one an ending. What changes is only what its
    first half calls.

  **What goes is the blob and nothing geometric**: `draw`, `remint`, and the
  object URL's minting, revocation, `#page=N` and `view=FitH`. The divider's
  handler keeps its `fit()` per move and loses `pane.style.pointerEvents`,
  which existed to stop WebKit's PDF view swallowing a drag — a reason this
  phase removes with the element it was set on.

  **A re-open destroys the `PDFDocumentProxy` it replaces.** The pane re-opens
  every time the typing stops, and a retained proxy that is dropped without
  `destroy()` leaks its worker-side document.

  **The four states keep working**, and they are named because they ride on the
  element being replaced: `report` toggles the dimming a stale page wears,
  `fail` reads whether a page is drawn to decide whether the error takes the
  whole pane, and `clear` empties it for a second Open.

- **Exit gate:** Run against **`samples/showcase/showcase.md`**, which the CLI
  compiles to six pages — checkable independently as `/Count 6` on the compiled
  PDF's page tree, so the literal survives the sample gaining a page.

  1. All six pages draw, in order.
  2. **With the machine set to always-show scrollbars**, no page is clipped at
     its right edge at three divider positions and after a window resize. This
     is the clause that has content: `scale = paneWidth / naturalWidth` makes
     any width-equality check pass by construction, so what is tested is that
     `paneWidth` was read from the scroll container's content box and not the
     column's. Round 1's catch.
  3. The canvas backing store is `floor(cssWidth × devicePixelRatio)` **to the
     device pixel**, checked in the Web Inspector — **after each of clause 2's
     positions, and after a window resize**. Round 2's catch: a rest that ends
     only on `pointerup` leaves a window-resized pane CSS-scaled and soft until
     the next compile, and every other clause passes on it. The instrument is a
     `cargo tauri dev` build, whose origin does not bear on a backing store.
  4. Each of OQ-2's three causes lands where §3 says: an open on page 1, a
     keystroke on the caret's page, a divider drag on the same page and
     fraction the reader was at.
  5. The page is not blank or flickering *during* a divider drag, only after —
     §2's gesture rule, and the thing no clause of the draft's gate reached.
  6. Every control in the header is toggled and the page follows each: the
     `Sections` panel and `Lines`, which are the two that caused the bugs
     "geometry is observed" exists for.
  7. The empty, failed and stale states are each entered once, and a second
     document is opened over the first.
  8. **A section is edited in another editor while the md2pdf window is behind
     it**, and the window is then brought forward: the page is current, not
     stale. This is `README.md`'s own "save the file in another program and the
     page redraws too", which is by construction the not-frontmost condition
     OQ-5 describes — one line, and it is the only clause that can see it.
  9. `cargo test --workspace` passes unchanged — **no `.rs` file is edited by
     this phase**, which is itself the check. The binary's embedded assets do
     change, `generate_context!` embedding `frontendDist`.

  Clauses 1–8 are manual: `mpdf-003` OQ-10 records that nothing in this
  repository reaches this file, and this phase does not change that.

- **Close-out:** `rules/desktop-panes.md`'s "The frame's width" is replaced
  rather than corrected — it describes a mechanism that no longer exists — and
  "The page" loses the blob, the fragment and the `pointer-events` workaround.
  **`rules/desktop.md` changes too**, which the draft missed: it counts the
  app's files, calls `frontendDist` a directory of static files, and says no
  policy blocks the `blob:` frame. `mpdf-003` §2's "Why the pane is a `blob:`
  URL in an iframe" and its **no bundled JavaScript PDF viewer** constraint each
  take a dated `CORRECTED` note, and `mpdf-003` Phase 7 takes `cut` and
  `by: mpdf-009` — this spec's `supersedes` edge being written in the same pass,
  per §1.1. In `README.md`, the sentence "**The pane is a real PDF view and
  tells the app nothing about where you scrolled it, so it follows your cursor
  instead**" is replaced rather than supplemented — OQ-2 reverses exactly the
  thing it states — and `## Licence` gains the Apache-2.0 entry. One push.

### Phase 2 — the text it can select and the links it can follow
*Produces the observable: **yes** — the same page, with the text selectable and
the links live, which is what it was before Phase 1 took the frame away.*

- **Scope:** A text layer and an annotation layer over each canvas.

  **The text comes off `streamTextContent` and never off `getTextContent`**,
  per §2: the convenience call is broken in this webview and the stream read by
  hand is not. This is the phase's one non-obvious fact and the reason it is
  recorded in §2 rather than discovered here.

  **The layers are positioned from the same viewport the canvas rendered
  with**, so a scale change moves them together or the selection drifts off the
  glyphs.

  Links are `pdf.js`'s annotation layer, filtered to the link subtype: an
  internal destination scrolls the container, an external one is refused
  rather than opened, per `mpdf-001` §2 — this app fetches nothing and opening
  a browser is not fetching, but it is the same decision and belongs to a
  round.

- **Exit gate:** Selecting a paragraph on a rendered page yields its text in
  document order; a cross-reference in the showcase, which `mpdf-005` makes a
  live link, scrolls the container to the figure it names. Both are manual —
  `mpdf-003` OQ-10.

- **Close-out:** `rules/desktop-panes.md` gains the two layers and the
  `getTextContent` trap, which is the kind of fact a rule exists to hold. One
  push.

### Phase 3 — the zoom the reader owns
*Produces the observable: **yes** — the same page at the size the reader asked
for.*

- **Scope:** Fit-to-width, fit-page, and an explicit scale, as the three modes
  `armquill`'s viewer names. The mode is the pane's state, and a resize
  re-derives the scale under the first two and leaves it alone under the third.

  **Fit-to-width stays the default**, because it is what the pane has always
  approximated and what an author writing to a page width wants.

- **Exit gate:** Each mode holds across a divider drag and a window resize;
  fit-page shows a whole page at the pane's shorter dimension; the explicit
  scale survives a re-render after a compile.

- **Close-out:** `rules/desktop-panes.md`'s width section gains the modes. The
  README gains nothing — a zoom is not what the app is for. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-009.md, append-only, one heading per round. See §7 of the
methodology.
-->
