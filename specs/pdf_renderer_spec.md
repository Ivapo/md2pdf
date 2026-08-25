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
- **Step 1 — does it remove or contradict shipped work?** **Yes, and that is
  the landing.** `mpdf-003` Phase 7 exists only to work around a view this spec
  removes; every line of its mechanism — the transform, the height
  compensation, the settle timer, the re-mint, the caret-page jump a redraw
  costs — is answered by a renderer that recomputes its scale per render.
  Phase 1's own frame goes too. A phase of `mpdf-003` cannot do this: §6.1 step
  1 says a phase never removes what another phase shipped, and a phase that
  removed Phase 7 would read as if Phase 7 had never been built, when in fact
  it was built, used, and taught this project what it needed to know.
- **Steps 2–4 are not reached**, step 1 having matched. The edge is
  `supersedes: [{id: mpdf-003, phases: ["Phase 7 — the page fits the pane it is
  given"]}]`, against which `mpdf-003` sets `cut` and `by: mpdf-009` on that
  phase, keeps `status: accepted`, and has its rollup become `partial` by rule
  4.

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
  markdown line needs a position map through the generated Typst, which
  `mpdf-003` OQ-7 already parks as a `core` question. A later spec.
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

Two files copied into `app/dist/`: `pdf.min.mjs` and `pdf.worker.min.mjs`,
about 1.67 MB together. **No npm at build time, no bundler, no node** — which
is the whole of what `mpdf-003` OQ-2's `withGlobalTauri` bought and what this
spec must not spend. `pdfjs-dist` ships browser-ready ES modules precisely so
they can be loaded from a `<script type="module">`, and **measured in this
window on 2026-08-25: version 6.2.108 imports in 26–31 ms** from
`tauri://`-served static files, with no CSP change and no capability added.

**The files are committed.** The repository refuses to commit `web/pkg/` — a
25.7 MB wasm module — and the reasoning there was that git history is permanent
and the artifact is enormous. This is fifteen times smaller and, unlike that
one, it is *required for the app to build at all*: fetching it in the build
would put a network and a package manager back in a path that is one Cargo
command. The `LICENSE` and `NOTICE` travel beside it, Apache-2.0 requiring
attribution.

### Fit-to-width is a scale, recomputed (decision, recorded)

`scale = paneWidth / page.getViewport({scale: 1}).width`, taken **on every
render**. That is the whole of what `mpdf-003` Phase 7 spent a transform, a
height compensation, a settle timer and a redraw on: a fit that survives a
resize because it is not a state that can go stale, it is an expression
evaluated when the page is drawn.

**The pane's geometry is observed and not inferred** — `mpdf-003` Phase 7's
rule survives its own mechanism, and a `ResizeObserver` on the preview column
is what triggers a re-render. Two bugs came from listing the causes of a resize
and both had one shape.

### Sharpness is the device's pixel ratio, and it is the thing a transform could
not do (decision, recorded)

The canvas backing store is `floor(viewport.width * devicePixelRatio)` by the
same in height, with the context transformed by that ratio and the CSS size
left at the logical one. **Measured: page 1 of the six-page showcase renders in
33–46 ms at `devicePixelRatio` 2.** A CSS transform can only resample a raster
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
  Measured: all six pages of the showcase render in 94–95 ms, about 16 ms each,
  so a document of this size needs no virtualisation and Phase 1 will not build
  any. A thesis will. The threshold is unmeasured and the answer is a page
  budget rather than a document one. *(deferred by evidence)*
- **OQ-2** — what happens to the reader's scroll position across a re-render?
  The current pane cannot restore it and sets the caret's page instead, which
  `mpdf-003` Phase 6 decided and Phase 7 inherited. **A renderer this project
  owns knows the scroll offset**, so for the first time restoring it is
  possible — and "restore where they were" and "follow the author's caret" are
  now two answers that both exist, where before only one did. Phase 1 must
  choose, and the choice is a use question. *(needs-input)*
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

- **Scope:** The iframe and its blob go. In their place a scroll container the
  pane owns, one canvas per page, and a module that opens the bytes and renders
  them.

  **The bytes reach it as they do now.** `current_pdf` already crosses raw as a
  `tauri::ipc::Response`; `pdf.js` takes a `Uint8Array`, so nothing about the
  command, the `revision` guard or the signal changes. The object URL, its
  revocation, and the `#page=N` and `view=FitH` fragments all go, and with them
  every line of `mpdf-003` Phase 7's mechanism.

  **The scale is `paneWidth / naturalWidth`, per render**, and the canvas is
  sized at `devicePixelRatio` with the context transformed to match. A
  `ResizeObserver` on the column re-renders, which is Phase 7's surviving rule.

  **OQ-2 is resolved in this phase and not deferred past it**: a re-render
  either restores the scroll offset it took before it started, or opens on the
  caret's page as `mpdf-003` Phase 6 decided. The recommendation is *both, by
  cause* — an author's own keystroke follows the caret, and a resize restores
  the offset — because the two are now distinguishable, where the old pane had
  one answer for every cause.

  **A render in flight is cancelled before the next begins.** `pdf.js` returns
  a task with `cancel()`, and a pane that re-renders whenever typing stops will
  overlap them.

- **Exit gate:** A document of six pages draws all six; the page's width equals
  the pane's content width at three divider positions and after a window
  resize, within a pixel; the canvas backing store is `devicePixelRatio` times
  the CSS size. `cargo test --workspace` passes unchanged — **`core` and the
  Rust side are untouched by this phase**, which is itself checkable and worth
  checking. The rendering is in the page and `mpdf-003` OQ-10 still applies.

- **Close-out:** `rules/desktop-panes.md`'s "The frame's width" is replaced
  rather than corrected — it describes a mechanism that no longer exists — and
  "The page" loses the blob and the fragment. `mpdf-003` Phase 7 takes `cut`
  and `by: mpdf-009` on this phase shipping, and `mpdf-003` §2's "no JavaScript
  PDF viewer is bundled and none is wanted" takes a dated `CORRECTED` note.
  The README's app section gains a line about what draws the page. One push.

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
