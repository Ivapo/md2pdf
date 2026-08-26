---
id: mpdf-009
title: pdf-renderer
note: >
  The app draws the page itself: `pdf.js` is vendored as two static modules and
  rasterises each page onto a canvas the pane owns, so fit-to-width is a mode
  rather than a transform, the type is sharp at the display's own resolution,
  and the text and links come back with it.
status: accepted
last_updated: 2026-08-26

phases:
  - name: "Phase 1 — the page is drawn here"
    reviewed: 2026-08-25
    shipped: 2026-08-25
    cut: null
    by: null
  - name: "Phase 2 — the text it can select and the links it can follow"
    reviewed: 2026-08-25
    shipped: 2026-08-26
    cut: null
    by: null
  - name: "Phase 3 — the zoom the reader owns"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — the pages are told apart"
    reviewed: 2026-08-25
    shipped: 2026-08-25
    cut: null
    by: null
  - name: "Phase 5 — the pane holds what the reader is near"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes:
  - id: mpdf-003
    phases: ["Phase 7 — the page fits the pane it is given"]
superseded_by: null
related: [mpdf-001, mpdf-003, mpdf-006]
reference: >
  `armquill`, this author's own cloud editor, is where the shape was read from:
  its viewer takes a `fitMode` of `page | width | manual` and derives the render
  scale per page from the container's width, which is the design this spec
  takes. Its collaborative editing, its server-side compile and its file
  explorer are out of scope permanently — `mpdf-001` §2 keeps this app local and
  fetching nothing. Mozilla's `pdf.js` is the renderer itself, Apache-2.0, whose
  LICENSE travels with the vendored files.
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

> **CORRECTED 2026-08-25, by Phase 4.** The last sentence above is kept as it
> was written and its first clause is no longer true. **The canvases do not sit
> flush**: `#pages canvas` carries `margin-top: 16px`, `#pages::after` carries
> the same gap below the last page, and each page wears a `var(--edge)`
> hairline on its top and bottom edges — so a reader can see where one page
> ends and the next begins. Its *second* clause is the one that held, and held
> exactly: a phase that wanted a gutter did have to change this sentence and
> the gate with it, and Phase 4 is that phase doing that.
>
> **What is corrected is the gutter and nothing else.** The paragraph's own
> subject — that `paneWidth` is the scroll container's `clientWidth` and not
> the column's — is untouched and stays exactly true, and the phase is built so
> that it does. The gap is vertical only: `clientWidth` *includes* padding, so
> side padding would not reduce `paneWidth` at all, and the canvases would be
> sized past the content box and clipped silently. They still meet that content
> box at its left and right edges, which is why there is no background beside a
> page for a side hairline to separate it from.

**The pane's geometry is observed and not inferred** — `mpdf-003` Phase 7's
rule survives its own mechanism, and a `ResizeObserver` on that container is
what triggers the re-render. Two bugs came from listing the causes of a resize
and both had one shape.

### A gesture is carried by CSS size; a rest is answered by a render (decision, recorded)

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
width is changing the canvases are resized by CSS — their backing stores
untouched, the browser resampling a bitmap it already has. §4 fixes *which*
property does it and argues the choice; the cost is a relayout and a repaint
per step rather than a composite, which is far under the 94 ms a render costs
and is what the gesture exists to avoid. When the width rests, they are re-rendered at the new scale,
which is sharp because the raster is made for that width.

What is different is what the rest costs. Phase 7's redraw re-minted a blob and
so **could not restore the reader's place**, because WebKit leaks none — the
whole reason a drag put the reader on the caret's page. A renderer this project
owns re-rasterises into canvases it already positions, so the rest is
invisible: same page, same place in it, sharper type. **The mechanism looks
like Phase 7's and costs nothing it cost.**

**The gesture moves the reader unless it is held, and round 3 found the spec
silent on it.** Resizing by CSS reflows, so page *k*'s top offset is the sum of
the heights above it and a gesture of factor *s* maps a content offset `T` to
`T·s` — while the browser holds `scrollTop` at `T`. The reader is displaced by
`T(s−1)` continuously through the drag. Re-derived from the app's own default
geometry — a 900 px window, the text pane at 40%, so a ~540 px preview, and A4
at 763.7 px a page — page 5's top is 3055 px and a 20% widen displaces the
reader **611 px**; and WebKit implements no scroll anchoring that would
compensate. **What marks a start, for the causes that have no pointer event
to mark one — a window resize, a control taking width out of the column — is
`fitted` itself**: a width change arriving while `box().width === fitted` opens
a gesture, and one arriving while they already differ continues the gesture
already open. So **OQ-2 case 3's anchor is taken when the gesture starts, not
when the render begins**, and it is reapplied on every step of the gesture as
well as after the render that ends it. Taken at the render, as the draft of
this section had it, the anchor would faithfully preserve a position the drag
had already ruined.

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
Carries"`**, which is that page's own title block. Phase 2's round re-derived the
whole profile — 190, 173, 165, 139, 234 and 67 items, 968 in all.

**The loop above need not be written, and Phase 2's round found why.**
`TextLayer`'s constructor takes a `textContentSource` and branches on
`t instanceof ReadableStream`, driving it with `getReader()` and `read()` — no
async iteration anywhere. Handing it `page.streamTextContent()` obeys this
decision by construction. The hand-written loop stays recorded because it is
what the rule *means*, and because a later reader reaching for
`getTextContent` needs to find this section either way.

### The layers need scaffolding the vendored files do not carry (decision, recorded)

**This is the trap of Phase 2 as `getTextContent` is the trap of its text**,
measured in that phase's round 1. `app/dist/pdfjs/` holds three files, and the
bundle does export `TextLayer` and `AnnotationLayer` — the classes are there.
What is not there is what pdf.js's own viewer wraps around them.

- **`SimpleLinkService` is absent**: the string appears zero times in
  `pdf.min.mjs`. `LinkAnnotationElement.render` reads `this.linkService` and
  calls `addLinkAttributes`, `getDestinationHash` and `goToDestination` on it,
  and the only implementation ships in `pdfjs-dist/web/pdf_viewer.mjs`, which
  this project does not vendor.
- **`--total-scale-factor`, `--scale-round-x` and `--scale-round-y` are read and
  never defined.** `setLayerDimensions` writes
  `round(down, var(--total-scale-factor) * <pageWidth>px, var(--scale-round-x))`,
  and the annotation editor's own styles read the same property. Their
  definitions live in `pdf_viewer.css`, which is not vendored either.
  Undefined, those declarations are invalid at computed-value time, the layer
  takes no size, and the text paints raw over the canvas instead of aligning to
  it.

**The answer is not a third vendored file**, and both gaps turn out to be this
app's own business.

**The app defines the three custom properties, and that is what carries a
gesture.** The split this section already decided for the canvases arrives free:
one property is written wherever a page's CSS box is written, and every layer box
and every span position follows by CSS — no new viewport, no re-render. Phase 2's
draft said the layers are "positioned from the same viewport the canvas rendered
with, so a scale change moves them together", which is true of a **rest**, that
being what produces a new viewport, and false of a **gesture**, which produces
none — the layers would have stayed at the old size through every drag, which is
the drift the sentence claimed to prevent. The custom property is what makes it
true of both. `--scale-round-x` and `--scale-round-y` are `1px`, the same
whole-CSS-pixel rule the canvases already follow.

**`--total-scale-factor` is an absolute scale from unscaled PDF units, and round
2 caught this document saying otherwise.** `setLayerDimensions` multiplies it by
`viewport.rawDims.pageWidth`, and `rawDims` is built as `viewBox[2] - viewBox[0]`
— **unscaled**, 595.28 for the showcase's A4, and independent of the render
scale. So the value is always

> `--total-scale-factor` = *the page's current CSS width* ÷ *its unscaled width*

which is the render scale at rest and the render scale times *s* during a
gesture. An earlier draft of this section said `fit()` writes it and `unscale()`
"writes it back to `1`"; **`1` would put a 595 px layer over a 535 px page** on
this app's own default geometry — an 11.3% offset on every span and every
annotation rect, most of the overhang clipped rather than visible. The relative
factor a gesture already computes is wrong by the same ratio at every width.
Written as the one expression above it needs no case analysis, which is the
argument for writing it that way.

**The layers need a stylesheet, and "define three properties" is not one.** Round
2's second catch, and this section had the mechanism right and its surface
wrong — an earlier draft claimed the bundle's own span styles key off
`--total-scale-factor`, and that property appears **zero times** inside
`TextLayer`. What a text span actually receives inline is `left` and `top` as
**percentages**, `--font-height` in unscaled px, `font-family`, and — when they
apply — `--scale-x` and `--rotate`. **There is no `position` and no
`font-size`**, so with no rules of the app's own the percentages resolve against
nothing and the layer renders as a wall of readable text over the raster.
`AnnotationElement._createContainer` is the same shape: percentage `left`, `top`,
`width` and `height`, a `zIndex`, and no `position`; and the `<a>` it appends
gets no box at all, so a link has nothing to click until the app gives it one.

So the app both **defines** three properties and **consumes** three more, in a
short stylesheet of its own — position and font-size for a span, the transform
that `--scale-x` and `--rotate` feed, a box for the annotation's `<a>`, and
transparent text, the glyphs a reader sees being the raster's. That stylesheet is
part of what "no third vendored file" buys and is named here so it is not
mistaken for an omission.

**A gesture does not call `TextLayer.update()`.** It re-measures every span
against a 2D context to recompute `--scale-x`, which is a unitless
measured-to-target ratio that a pure scale change does not disturb. Reaching for
it per gesture step would put a per-frame text measurement into the drag, which
is the cost the whole gesture-versus-rest split exists to avoid.

**The link service is hand-written rather than ported.** An internal destination
is already something this app reaches — it scrolls the pages container, which is
what its anchor path does — and `SimpleLinkService`'s own route sets an `href`
on an `<a>`, which in this window navigates the webview off `tauri://localhost`
with nothing to come back with.

**The annotation layer takes `viewport.clone({ dontFlip: true })`** where the
text layer takes the viewport itself. That is what pdf.js's viewer does, and it
is recorded here for the same reason as everything else in this section.

### An external link is not rendered at all (decision, recorded)

Phase 2's draft said an external link "is refused rather than opened" and that
the decision "belongs to a round". This is that round, and the answer is that
**the annotation is filtered out before the layer is built**: only a link
carrying an internal destination is rendered, so an external one has no element,
no `href` and nothing to activate. "Refused" then means something a second
person can check — there is no `<a>` in the DOM for it — rather than a behaviour
observed not to happen.

**The citation is `mpdf-003` §1.1, "No servers, no network. Ever."**, and not
`mpdf-001` §2, which the draft named and which is about Typst package resolution
and font discovery inside the compiler's `World`. Round 1 caught the
substitution. The app also declares only `core:default` and the two dialogs, and
carries no opener or shell plugin, so opening one was never a step away.

**A third class exists that refuse-or-follow does not name**, and it sits in the
sample the gate runs against. `samples/showcase/sections/text.md:56` writes
`[this one](#fig:pipeline)` — a markdown link *with text*, which the dialect
emits as an ordinary link rather than as a reference. **It reaches the PDF as a
`/URI (#fig:pipeline)` annotation**, and pdf.js rejects it on its *protocol*
rather than for lacking a destination: `_isValidProtocol` accepts `http`,
`https`, `ftp`, `mailto` and `tel` and nothing else, so the annotation arrives
with `url` and `dest` **both `null`**. It is on page 2, and it reads in the markdown almost exactly like a
cross-reference. The filter above covers it without a second rule: no internal
destination, no element.

**Internal is broader than cross-references, which is worth knowing before the
filter is written.** Re-derived from the compiled showcase: its twenty
internal-destination links are seven cross-references *plus* the footnote marks
and their return arrows and the citation marks into the reference list — so the
filter delivers a document that is navigable in three ways, not one, and a gate
that only exercised a cross-reference would under-report what shipped.

**What it costs, recorded rather than hidden**: a reader who sees a link cannot
follow it, and the page gives no sign of which links are live. The showcase
carries five external annotations and they are **not all on one page** — four on
page 2, being `https://typst.app` three ways and `mailto:you@example.com`, and a
fifth inside a footnote that lands on page 5. Round 2 caught an enumeration here
that named four and called them five. The sixth `/URI` above is on page 2 too.
OQ-6 carries what could be given back.

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

  > **MEASURED 2026-08-26, by OQ-7's probe, and the threshold this asked for now
  > exists.** At a 619 px pane and `devicePixelRatio` 2, a 71-page 62,000-word
  > document costs **1.25 s** to draw and holds **587 MB** of canvas backing
  > store — 17.6 ms and 8.3 MB a page, against 50 MB for the six-page showcase.
  > **Memory is what binds, and by a wide margin**: the time is still tolerable
  > at a size the memory is not, so a budget expressed in pages is really one
  > expressed in megabytes, and the pane's width is a term in it. The question
  > stays open because what to *build* about it — near-page rendering, a released
  > backing store, a lower ratio off screen — is a phase and not a number.
  >
  > **What the open felt like, and what it was not.** Observed the same day: 5–7 s
  > the first time that document opened and 1–2 s on every compile after.
  > **Almost none of it is the pane, and the attribution nearly went the wrong
  > way.** That document compiles in **7.12 s** through `target/debug/md2pdf` and
  > **0.24 s** through `target/release/md2pdf` — Typst is some 30× slower
  > unoptimised — against 0.71 s and 0.08 s for the showcase. So the first wait is
  > the compiler in a debug build, which `cargo tauri dev` is and which every
  > figure taken at that window therefore is; the 1–2 s after it is Typst's own
  > incremental cache; and a release build would open this document in about
  > 1.5 s, of which **the pane's 1.25 s is the larger half**. The renderer is what
  > binds on time once the compiler is optimised, and memory binds before either.
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

  **The anchor is taken when the gesture starts and held throughout**, not
  taken when the render begins — round 3's catch, and §2's "A gesture is
  carried by CSS size" argues it: a CSS resize reflows, so the reader drifts
  through the drag itself unless the anchor is reapplied on every step. A
  capture at the re-render would preserve the displaced position faithfully and
  restore nothing.

  **A compile landing mid-gesture keeps the reader's place and skips case 2's
  caret jump**, for that compile only. It takes a keystroke's debounce expiring
  in the instant between grabbing the divider and letting go, so it is rare;
  both answers are defensible and this is the one that never moves the page out
  from under a hand that is on it. The next compile after the gesture ends
  follows case 2 as usual.

  **The restored quantity is a page index and a fraction within that page, not
  a pixel offset.** Round 1 caught the draft naming the raw `scrollTop`: the
  scale changes with the width, so every canvas's height and the container's
  `scrollHeight` change with it, and a pixel offset held across that moves the
  reader — and clamps at the bottom when a pane widens and then narrows. An
  anchor of *(page, fraction)* survives both a scale change and a document that
  gained a page.
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
- **OQ-6** — what is given back to a reader who clicks a link out to the web?
  Raised 2026-08-25 by Phase 2's round 1, which forced the refusal to be decided
  rather than deferred. §2 settles that an external annotation is not rendered,
  which is the honest floor and leaves the reader with no sign that the link was
  ever live. Three shapes were named and none costed: mark the link visibly
  inert, so the page distinguishes a dead link from ordinary text; copy the URL
  to the clipboard on a click, which fetches nothing; or open the system browser,
  which needs a Tauri opener plugin, a capability entry, and a decision against
  `mpdf-003` §1.1 that this project has never wanted to make. **The first two are
  compatible with every decision this project has recorded** and the third is
  not, which is most of the answer. Design call. Blocks nothing. *(needs-input —
  it wants a reader's judgement in use, as `mpdf-003` OQ-6 was answered)*
- ~~**OQ-7** — what do the two layers cost per render, and is there a page
  budget?~~ **RESOLVED 2026-08-26**, by the probe this question deferred to, run
  in the window against the shipped Phase 2 on the day it shipped. Two documents,
  seven passes each, at a 619 px pane and `devicePixelRatio` 2:

  | | the showcase | a 62,000-word document |
  |---|---|---|
  | pages | 6 | 71 |
  | text items | 968 | 6,509 |
  | raster alone | 131 ms | 1,250 ms |
  | raster and both layers | 169 ms | 1,251 ms |
  | median ms a page | raster 18, text 5, annotations 4 | raster 12, text 4, annotations 0 |

  **The text layer costs about 4 ms per 100 text items, and the two documents
  agree on that independently** — 3.9 ms per 100 at the showcase's 161 items a
  page, 4.3 ms per 100 at the other's 93. The annotation layer costs 0–4 ms a
  page and **did not scale with the twenty links** the showcase carries, so what
  it measures is the `getAnnotations` round-trip rather than the elements.

  **The two totals at 71 pages are not the figure to read**, and saying so is
  half the answer: 1,250 against 1,251 ms is worker-bound throughput rather than
  a free layer, and it is the per-page medians that separate the phases — 4 ms of
  16. A reader taking those totals at face value would conclude the layers cost
  nothing, which the six-page run contradicts.

  **So no page budget is forced by the layers**, which is what was asked. They
  are a surcharge of about a third on a raster costing 12–22 ms a page, and the
  rest they were feared to lengthen went from 131 ms to 169 ms over six pages.

  **The budget that is forced is OQ-1's, and it is memory rather than time** —
  the one thing this question did not anticipate. The pane draws every page and
  retains it, a canvas at this width and ratio is 8.3 MB, and the 71-page
  document therefore holds **587 MB**. Time crosses 500 ms at 28 pages and 1 s at
  56; memory crosses 1 GB at about 120. Logged against OQ-1, which is where the
  decision belongs.
- **OQ-8** — at what document length does the pane as Phase 1 built it actually
  *fail*, and on what machine? Raised 2026-08-26 with Phase 5, whose argument
  needs it and does not have it. What is measured is that the cost is linear and
  unbounded at 8.3 MB a page, and that 71 pages and 587 MB are survivable — the
  window drew them and stayed responsive. What is **not** measured is the point
  at which WKWebView jettisons the content process, which is the difference
  between a phase that prevents a crash and one that prevents a number from
  growing. Answerable by a probe: compile documents at 150, 300 and 600 pages and
  open each. **It calibrates Phase 5's urgency and does not block it**, because
  Phase 3's explicit zoom multiplies the same quantity by the square of the scale
  whatever the answer turns out to be. *(deferred by evidence)*

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
    it is what tells a gesture from a rest. **The render writes it**, `draw()`
    being its only writer today and going with the blob.
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
  - **`unscale()`** — kept, but it **writes** the CSS size to the logical one
    rather than clearing it. Clearing is what it does today, and a canvas with
    no CSS size lays out at its backing store — at dpr 2 that is a page twice
    the pane's width. §2's "the CSS size left at the logical one" is the rule;
    this is the function that keeps it.
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
     keystroke on the caret's page, and a divider drag on the same page and
     fraction **the reader was at before the drag was started** — read then,
     not after, which is what makes the clause test anything. The reader must
     also not drift *during* the drag, which is the same anchor doing the same
     job and is why one clause covers both. **Run again for a window
     drag-resize**, which reflows by the same arithmetic and is the path with
     no pointer event to mark a start — an implementation keyed to
     `pointerdown` rather than to the width moving drifts the reader there and
     passes every other clause.
  5. The page is not blank or flickering *during* a divider drag, only after —
     §2's gesture rule, and the thing no clause of the draft's gate reached.
  6. Every control in the header is toggled and the page follows each: the
     `Sections` panel and `Lines`, which are the two that caused the bugs
     "geometry is observed" exists for.
  7. The empty, failed and stale states are each entered once, and a second
     document is opened over the first.
  8. **A section is edited in another editor while the md2pdf window is behind
     it**, and the window is then brought forward: **the page on screen shows
     the edit**. Keyed to the drawn canvas and deliberately not to the header's
     word — round 3's catch, and the clause passed on its own defect before it.
     `current` and `stale` are set in Rust from the compile alone, on the watch
     loop's own thread, which window occlusion does not stop; a pane that never
     rasterised a pixel still reads `current`. This is `README.md`'s own "save
     the file in another program and the page redraws too", which is by
     construction the not-frontmost condition OQ-5 describes.
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
*Produces the observable: **yes**, and in the same weak class Phase 4 argued
rather than asserted — the draft of this line asserted it, which round 1 caught.
No byte of the compiled PDF changes here. What changes is that two things the PDF
genuinely carries, its link annotations and its text, stop being unreachable to
the reader looking at it. That is Phase 4's argument exactly: the artifact was on
screen and a property of the artifact was not.*

**Why it is worth building.** `mpdf-003` §2 chose to draw a real PDF for three
named properties — a link stays a live annotation, the text stays selectable, and
the accessibility tagging survives — and Phase 1 of this spec removed all three
when it took WebKit's view away. §2's "What is given up" assigns the first two
here. **So this closes a regression against six shipped phases rather than adding
a capability**, which is a different argument from Phase 4's and a stronger one.

- **Scope:** All of it is in **`app/dist/index.html`**. No `.rs` file is edited
  and **nothing new is vendored** — §2's scaffolding decision is what makes that
  hold.

  **The DOM shape is the decision the rest depends on, and it moves a shipped
  invariant.** `#pages`'s children are canvases today, and five shipped functions
  turn on it — **seven sites across six functions**, the prune living inside
  `drawPages`, and round 2's sweep found two that the first draft of this list
  missed: `drawPages` indexes `pages.children[n - 1]`, calls
  `pages.replaceChild`, and **prunes with
  `while (pages.children.length > numPages) pages.lastElementChild.remove()`**;
  `fit` and `unscale` iterate `pages.children` and dereference `canvas.logical`;
  `readAnchor` and `applyAnchor` read `offsetTop` and `offsetHeight` off each
  child; and **`size(canvas, scale)` is the only writer of a CSS box in the
  file**, which `fit`, `unscale` and `drawPages` all delegate to. Phase 4 wrote
  that invariant down as its own reason for a pseudo-element rather than a spacer
  element, and this phase is where it is renegotiated deliberately rather than
  tripped over.

  **A page becomes a positioned wrapper and the canvas moves inside it**, with
  the text layer and the annotation layer beside it **inside that wrapper**:
  `.logical` is set on the
  wrapper, `replaceChild` swaps the wrapper, the prune removes a wrapper and
  takes its layers with it, and the anchor reads the wrapper's box. **All seven
  sites keep their shape and change only what they address.** Layers as
  siblings of the canvas *inside `#pages`* break the indexing; layers inside a
  wrapper that is not itself the child leave `replaceChild` swapping the wrapper
  away and dropping them. Neither is available, and saying so is what stops an
  implementer trying one.

  **`size()` sizes the wrapper, and the canvas inside it takes `width: 100%;
  height: 100%`.** Round 2's catch: with `.logical` on the wrapper, `size()`
  writes the wrapper's box and nothing writes the canvas's — and a canvas with no
  CSS size lays out at its backing store, which at `devicePixelRatio` 2 is a page
  twice the pane's width. That is the exact failure Phase 1 argued `unscale()`
  into existence to prevent, arriving by a different door.

  **`size()` is also where `--total-scale-factor` is written**, as §2's one
  expression — the box's new CSS width over the page's unscaled width, the
  unscaled width being kept beside `.logical` at render. Writing it there rather
  than in `fit()` and `unscale()` is what makes a gesture and a rest agree
  without either knowing about the layers: both already delegate here.
  **`--scale-round-x` and `--scale-round-y` are `1px`**, restated here rather
  than left under the pointer to §2 because undefined they take the layer's size
  down with them, which is clause 5's failure by a second route.

  **The app's own stylesheet for the layers**, per §2 — a span's `position` and
  `font-size`, the transform `--scale-x` and `--rotate` feed, a box for the
  annotation's `<a>`, and transparent text. `pdf.js` sets none of these, and
  without them the text layer renders as readable text over the raster.

  **`margin-top: 16px` and the `var(--edge)` hairline move to the wrapper with
  it.** `#pages canvas` still matches a canvas nested one deep, so left alone the
  gap would move *inside* the page: a layer pinned to the wrapper's origin would
  sit 16 px above its glyphs, and Phase 4's `offsetTop` arithmetic would read `0`
  where it shipped `16`. Phase 4's clause is therefore re-run in this gate.

  **The text comes off `streamTextContent` and never off `getTextContent`**, per
  §2 — and `TextLayer`'s constructor takes the `ReadableStream` directly, so the
  loop §2 describes is what the class already does and is not written again here.

  **The annotation layer takes `viewport.clone({ dontFlip: true })`** where the
  text layer takes the viewport itself, per §2.

  **Only a link with an internal destination is rendered**, per §2's own
  decision, and following one scrolls the pages container through the app's
  existing anchor path rather than through a `linkService` port. The destination
  is honoured at its own coordinate, not at the top of its page.

  **The layers follow a gesture by `--total-scale-factor` and a rest by being
  rebuilt**, per §2 — the property as `size()` writes it above, and a render
  rebuilding both layers from the viewport it drew with.

  **The layers are built while the wrapper is still detached, and swapped in
  with it.** Phase 1 made "a canvas is swapped in only once it holds pixels" an
  invariant against flashing empty pages, and under wrappers the whole wrapper is
  what is swapped, so the invariant now covers the layers too. Nothing in the
  layer path needs layout to do its work — `TextLayer` measures against an
  offscreen 2D context and `setLayerDimensions` only writes styles — so building
  detached is available, and it is chosen rather than left open. It puts the
  `streamTextContent` await inside the per-page loop, where every await already
  re-checks that a newer render has not superseded this one.

  **The four states keep working**, named as Phase 1 named them because they ride
  on the elements being changed. A **stale** page keeps its text selectable and
  its links live under `report`'s dimming — it is old, not broken, and a reader
  who can no longer select the page they are looking at would read it as the
  latter. `fail` still reads whether a page is drawn. `clear`'s
  `pages.replaceChildren()` disposes both layers with the wrappers, because they
  are inside them — which is the second reason the wrapper is the child.

  **A link followed inside `settle`'s 200 ms window is undone**, because
  `rerender()` ends with `applyAnchor` on the anchor captured when the gesture
  opened. **Accepted as it falls rather than fixed**: it is the same shape as the
  compile-landing-mid-gesture case §3 resolves in prose, the window is 200 ms
  wide, and the anchor exists to protect the reader's place against exactly the
  gesture that is still open.

- **Exit gate:** Run against **`samples/showcase/showcase.md`**, six pages.
  Manual, per `mpdf-003` OQ-10, and every clause names what a second person
  reads. Clauses 1, 3 and 4 are keyed to literals Phase 2's round 1 re-derived
  from the compiled PDF and the vendored bundle.

  1. **Select all of page 1 and copy it**: the text begins `Everything the
     Dialect Carries`, which is that page's own title block. §2 measures the page
     at 190 text items.
  2. **The selection lands on the glyphs at three divider positions and after a
     window drag-resize** — select one word mid-paragraph on page 4 and confirm
     the highlight covers that word and not its neighbour. **This is the clause
     `--total-scale-factor` exists for**: layers left at the render's viewport
     drift through every drag and fail here, and pass clause 1.
  3. **`[](#fig:halves)` on page 4 scrolls to Figure 4.2, which sits 458 pt down
     an 841.89 pt page** — not to the top of page 4. Re-derived from the
     compiled PDF during this phase's round; an implementation that scrolls to
     the destination *page* rather than to its coordinate would pass by eye
     without it. **And `[](#tab:kinds)` on page 4 scrolls back to Table 3.1 on
     page 3**, the document's only cross-page reference and the one that proves
     the destination's page was resolved at all. **Then one footnote mark and
     one citation mark**, per §2 — the filter admits twenty internal links here
     and only seven of them are cross-references.
  4. **No external link is followable.** The Web Inspector shows no `<a>` for
     any of page 2's four external annotations — `https://typst.app` three ways
     and `mailto:you@example.com` — and a click where each sits does nothing and
     navigates nowhere. **The fifth is inside a footnote and lands on page 5**,
     which is the one a filter applied to the wrong page would miss; round 2
     caught this clause looking for all five in one place. **The same at `[this one](#fig:pipeline)`, also page 2**,
     which is §2's third class: it carries a `/URI` and is refused on protocol,
     so an implementer who filters on "has a URL" rather than "has an internal
     destination" renders it and fails only here.
  5. **No text is painted over the canvas**: the glyphs the reader sees are the
     raster's and the layer above them is invisible. **Both scaffolding failures
     land here first** — a missing stylesheet renders the text layer as a wall of
     readable text, and a `--total-scale-factor` left at `1` puts a 595 px layer
     over a 535 px page — and both are easy to mistake for a font bug.
  6. **Both layers are present on all six pages**, and the gap between pages
     still measures 16 px — Phase 4's clause 1 re-run against the wrapper this
     phase moves the margin to.
  7. **A compile, a `clear` and a second Open each leave exactly one text layer
     and one annotation layer per page**, counted in the Inspector. This is the
     leak a layer parked outside the wrapper produces, and nothing else in the
     gate sees it.
  8. **A stale page's text still selects and its links still follow** — enter the
     state by saving a document that will not compile.
  9. `cargo test --workspace` passes unchanged — no `.rs` file is edited, which
     is itself the check.

- **Close-out:** `rules/desktop-panes.md` gains the two layers, §2's scaffolding
  and the link filter. **It sits at 301 of its declared 310 body lines**, so this
  close-out does the arithmetic `mpdf-003` Phase 7's did and reaches the other
  answer: that file had a second subject to shed and this one does not — the
  panes, their geometry, the gutter and now the layers are one subject — so the
  cap rises in the same pass rather than the file being split again.
  `rules/desktop.md` is untouched, and the reason is this phase's own "nothing
  new is vendored": it counts the app's files and names the vendored renderer,
  and neither moves.

  **The stylesheet's own comments are a third place the invariant lives, and they
  go false with it.** The block above `#pages canvas` says `margin-top` sits on
  `#pages canvas`; `#pages::after`'s says `fit`, `unscale`, `readAnchor` and
  `applyAnchor` "assume each child is a canvas carrying `.logical`"; and
  `#pages`'s own says `position: relative` makes each *canvas's* `offsetTop` a
  measurement from that box. All three are corrected in the pass that moves the
  wrapper in. This is the miss Phase 4's round 1 caught against its own draft,
  recurring one phase later, which is why it is written into the close-out rather
  than left to be noticed.

  **`README.md`'s app section gains a sentence, and that is argued rather than
  waived.** Phases 3 and 4 each say "the README gains nothing" with a reason; the
  reason does not carry here, because a reader who clicks a link in the pane and
  sees nothing happen has met a behaviour, and behaviours are what that section
  documents. It says the text selects, that a cross-reference jumps to what it
  names, and that a link out to the web does not open. One push.

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

### Phase 4 — the pages are told apart
*Produces the observable: **yes**, and it is the weakest yes in this document,
so it is argued rather than asserted. Nothing about the compiled PDF changes —
this phase touches no byte of it. What changes is that the reader can see where
one of its pages ends and the next begins. Phase 1 shipped, correctly by its
own design, six A4 pages as one uninterrupted white strip, which is not what a
six-page document looks like; the artifact was on screen and its pagination,
which is a property of the artifact, was not.*

**Appended after Phase 1 shipped**, per §6.1's ordered test worked in full.
Step 0 matches: §2 decided this and the decision changes. **Step 1's
phase-removing bullets do not match** — nothing Phase 1 delivered is removed,
no phase is cut, the vendored renderer, the recomputed fit, the
gesture-and-rest split and the reader's anchor all stand exactly as shipped,
and a `## 0.` closing note with a `cut` on Phase 1 would read as if Phase 1 had
never been built. **Step 1's prose bullet does match**, and the close-out
discharges it: §2's flush sentence is shipped prose that becomes actively
misleading, so it takes a dated `CORRECTED` note in place. Round 1's second
lens caught the draft asserting step 1 did not match at all while its own
close-out performed one of step 1's bullets. Step 2 then matches and is the
mechanism: `mpdf-009` owns how the pane draws the page, its rollup is `partial`
rather than `abandoned`, so this is a phase appended to it, numbered after the
last and renumbering nothing. **It has its own review round and is not cleared
to build until that converges.**

**It may ship before Phases 2 and 3.** §3 permits an out-of-order `shipped` and
the reason belongs in the review record rather than asserted here, so the
argument is logged there; what this phase claims is only that it depends on
nothing but shipped Phase 1.

- **Scope:** **`app/dist/index.html`, and inside it the stylesheet alone.** No
  `.rs` file, no change to the fit, no change to `renderPages`, no new
  `pdf.js` API. If this phase's diff reaches the script, something has been
  misunderstood.

  **The gap is 16 CSS pixels**, and the number is a length rather than a ratio
  — which is the whole point, and worth saying beside a gate clause that exists
  to forbid a percentage. Sixteen is large enough to read as a separation at a
  glance against a page some 535 px wide, and small enough not to spend scroll
  distance the reader wants; being a length, it stays exactly that at a narrow
  pane, where a proportion would collapse. It is carried by
  **`margin-top` on `#pages canvas`** and by **`#pages::after`** — which needs
  `content: ''` and `display: block` to generate a box at all, spelled out
  because every other declaration in this phase is — for the one below the last
  page. The property is
  named because the argument depends on it: `offsetTop` includes an element's
  top margin, `#pages` is a block formatting context by virtue of
  `overflow-y: scroll` so the first canvas's margin does not collapse out of
  it, and only top margins exist so no pair of siblings collapses to less than
  the constant. **The trailing gap is a pseudo-element and not a margin**
  because whether a scroll container's last child's bottom margin reaches
  `scrollHeight` is engine-dependent, and this ships on WKWebView where round 1
  could only probe Chromium. **It must not be a spacer element either**:
  `drawPages` indexes `pages.children[n - 1]` and prunes with
  `pages.lastElementChild.remove()`, and `fit`, `unscale`, `readAnchor` and
  `applyAnchor` all assume every child is a canvas carrying `.logical`. A
  pseudo-element is in the flow and not in `children`, which is exactly the
  pair of properties wanted.

  **The separation is vertical only, and that is the decision this phase turns
  on.** A gap between pages, and **no padding at the sides**. Round 1 corrected
  the draft's reason, which was weaker than the fact: `clientWidth` *includes*
  padding, so side padding would not reduce `paneWidth` at all — `box()` would
  keep reporting the padding-box width, the canvases would be sized past the
  content box, and they would be clipped silently. Beyond that it would spend
  the width fit-to-width exists to give: the app's default preview is some
  535 px, and an author writing to a page width wants that width. **A vertical
  gap costs the reader nothing they want**, only scroll distance they were
  passing through anyway.

  **The hairline is on the top and bottom edges only, and this is a design
  answer rather than a concession.** Round 1 found both blockers here, from two
  lenses and two independent probes. A fit-to-width canvas is exactly
  `pages.clientWidth` wide, so it meets the scrollport's own left and right
  edges: **there is no background beside a page to separate it from**, and a
  side hairline would be a line drawn along the edge of the viewport, carrying
  no information. It is also unpaintable — `#pages` ships `overflow-x: hidden`,
  and any outset ring spreads to x ∈ [−1, 0) and [W, W+1), both outside the
  clip. The draft asked for "all four sides" and no implementation could have
  passed it.

  **`inset` is not the escape, and is recorded so a later round does not
  propose it.** An inset shadow paints beneath a replaced element's content,
  and a canvas's bitmap is opaque white where `pdf.js` filled it, so an inset
  ring would be invisible.

  **The hairline is `box-shadow: 0 -1px 0 var(--edge), 0 1px 0 var(--edge)`**,
  painted into the gap above and below each page. `--edge` is named because
  `box-shadow` with no colour resolves to `currentColor`, which inherits
  `--ink` — an ink-coloured ring in light and a near-white one on white paper
  in dark. It is `box-shadow` rather than `border-top`/`border-bottom` so that
  it costs no layout: under `content-box` a border would make each canvas two
  pixels taller than the height `size()` wrote, and `offsetHeight` — which the
  anchor is expressed in — would stop being the raster's own height. It is
  wanted at all because in the light palette a white page sits on `--ground` at
  `#f4f4f2`, a separation the eye has to look for.

  **The gap is a constant and does not scale with a gesture or a zoom.** It is
  chrome and not content: a gap that grew with the page would read as part of
  the document at a large scale and vanish at a small one. `fit()` and
  `unscale()` therefore keep writing canvas sizes and know nothing about it.

  **The reader's anchor needs no change, checked against the shipped code
  rather than assumed.** `readAnchor` and `applyAnchor` are written in terms of
  `offsetTop` and `offsetHeight`; `offsetTop` measures to the border box and so
  moves with the gap, `offsetHeight` does not include the margin, and the pair
  round-trip. A reader parked *in* a gap yields a negative fraction against the
  page below, which `applyAnchor` reproduces — **not exactly across a scale
  change**, which the draft claimed: the negative fraction is re-multiplied by
  the new `offsetHeight`, so a reader *d* px into a gap lands *d·s* px into it
  after a gesture of factor *s*, and the browser clamps to 0 in the gap above
  page 1. The error is bounded by the constant above and is not visible, but
  it is bounded rather than absent.

- **Exit gate:** Run against **`samples/showcase/showcase.md`**, six pages
  (`/Count 6` on the compiled page tree). Read in the Web Inspector on a
  `cargo tauri dev` build, which is Phase 1 clause 3's instrument and is named
  here because clauses 2–4 re-run its clauses.

  1. The six pages are visibly separate, and the gaps are **each 16 px measured
     from the rendered geometry rather than from the declarations**: for the
     container `pages`, `children[0].offsetTop` is 16; for every adjacent pair,
     `children[k + 1].offsetTop − (children[k].offsetTop +
     children[k].offsetHeight)` is 16; and `scrollHeight − (last.offsetTop +
     last.offsetHeight)` is 16. **Reading the declarations instead would pass a
     `margin: 16px 0` written for `margin-top: 16px`**, which the first two
     expressions and a declaration read alike wave through. It reads **32 on any
     engine**: with the pseudo-element present the last canvas's bottom margin is
     not a last-child margin at all — it collapses with the `::after`'s zero top
     margin and contributes a real 16 px of advance ahead of the pseudo-element's
     own — so the trailing-margin question the `::after` was chosen to retire
     does not arise here either. Each page carries a hairline along its **top and
     bottom** edges and along neither side.
  2. **Phase 1's gate clause 2, re-run as that clause is written**: with the
     machine set to always-show scrollbars, no page is clipped at its right
     edge at three divider positions and after a window resize. Phase 1 records
     that a width-*equality* check passes by construction and is not what is
     tested; round 1 caught the draft adding one back and calling it the
     discriminator, which it is not.
  3. **Phase 1's gate clause 3, re-run as written**: the canvas backing store
     is `floor(cssWidth × devicePixelRatio)` to the device pixel, after each of
     clause 2's positions and after a window resize. The gap must not have
     reached the fit.
  4. **Phase 1's gate clause 4, re-run in all three of OQ-2's causes**, because
     this phase changes what `offsetTop` returns and all three consume it:
     - an **open** lands at `scrollTop` 0, which now shows the 16 px gap above
       page 1 rather than page 1 flush to the pane's top edge. **That is the
       expected result**, recorded here so it is not read as a defect; case 1
       is `pages.scrollTop = 0` in the script and this phase does not touch the
       script.
     - a **keystroke** lands on the caret's page at its own top edge, below
       that page's gap.
     - a **divider drag** and a **window drag-resize** each leave the reader on
       the same page and fraction they were at before the gesture, and they do
       not drift during it.
  5. The gap does not scale: **clause 1's three measurements each read 16 px at
     three divider widths**. The failure this guards against is a percentage
     margin, which resolves against the containing block's *width* and so tracks
     the divider — at a narrow pane the same declaration renders 8–9 px, and the
     pairs alternate rather than landing on a plausible integer. **Take all three
     at a settled width**, never mid-gesture: `fit()` writes fractional heights
     while a width is moving, and the rounded differences can read 15 or 17 with
     the margin still exactly 16.
  6. In **both palettes**, the computed `box-shadow` colour resolves to
     `--edge` — `rgb(213, 213, 207)` in light and `rgb(54, 59, 71)` in dark —
     and each page's top and bottom edges are discernible against `--ground`.
  7. `cargo test --workspace` passes unchanged, **no `.rs` file edited**, which
     is itself the check.

  Clauses 1–6 are manual, per `mpdf-003` OQ-10.

- **Close-out:** **§2's sentence "The canvases sit flush to that container's
  content box" takes a dated `CORRECTED` note** — a decision statement that
  stops being true, which §6.1's **second** further rule ("A correction is
  about claims, not pointers") sends to a note in place rather than to a
  sibling file. Round 1 caught the draft citing it as the third. §2's clause
  about `paneWidth` being the scroll container's `clientWidth` is **not**
  corrected and must be left standing: this phase is built so that it stays
  exactly true.

  **The same claim exists in three places and the draft named two.**
  `rules/desktop-panes.md`'s "The page's width" loses "no gutter between pages
  and none at the sides" and gains the gap, the hairline and the reason the
  hairline is a shadow; its `covers:` gains them too, since §8.1 makes `covers`
  the regeneration target and a fact the next `/sync-rules` cannot aim at is a
  fact it will drop. And **the stylesheet comment above `#pages canvas` in
  `app/dist/index.html` says it a third time** — inside the very block this
  phase edits.

  **`README.md` gains nothing** — it describes what the app does for a writer,
  and a reader who can see a page boundary has not gained a feature to be told
  about. One push.

### Phase 5 — the pane holds what the reader is near
*Produces the observable: **yes**, and the claim is narrower than this phase's
first draft made it. **At the app's default window a document the pane can
already show is rendered whole and nothing changes at all** — that is a decision
below, not an accident, and gate clause 1 checks it rather than assuming it. What
changes is for a document past the budget: its pages arrive without waiting for
every page to be rasterised, and it is held at a bounded cost rather than an
unbounded one. Round 1 rejected the draft's "some 100 ms" arithmetic — the sizing
pass precedes the first paint and the draft omitted it — so this phase claims no
latency figure it has not measured, and gate clause 9 is where the figure comes
from.*

**Appended after Phases 1, 2 and 4 shipped**, per §6.1's ordered test worked in
full, and **round 1 corrected the working itself**.

- **Step 0 matches**: OQ-1 decided, with a reason, that Phase 1 would build no
  virtualisation, and OQ-7's measurement changes that decision.
- **Step 1's phase-removing bullets do not match.** Nothing Phases 1, 2 or 4
  delivered is removed, no phase is cut, and every mechanism they shipped keeps
  working *because* Phase 2 made the page a wrapper and the canvas a detail
  inside it.
- **Step 1's prose bullet DOES match, and the draft asserted it did not.** §2's
  "What is given up, stated plainly" records that `mpdf-003` chose a real PDF
  view for three properties and that "the first two are rebuilt by Phase 2" — a
  live link and selectable text. **Unqualified, that stops being true here**: a
  page outside the band has no text layer to select and no annotation layer to
  click, which this phase's own "two costs" paragraph says four paragraphs later.
  The draft claimed no shipped prose went misleading while its own scope
  falsified it — **the exact shape Phase 4's round 1 caught, recurring two phases
  later**, which is why the close-out now carries the dated `CORRECTED` note that
  discharges it.
- **The claim that every page is rendered lives in five places, not one**, and
  the draft's audit named only OQ-1. It is also in OQ-7's resolution ("The pane
  draws every page and retains it"), in the frontmatter `note` ("rasterises each
  page onto a canvas"), in Phase 1's scope ("one canvas per page"), in
  `drawPages`'s own doc comment, and in the `#pages .page` stylesheet comment.
  **Only §2's sentence takes a `CORRECTED` note**: a resolved OQ is a record of
  what was measured on a date and §6.1 forbids rewriting it, `note` is
  description rather than a decision, and the two in-file comments are `rules`-
  class facts that the close-out corrects in place as Phase 2's did.
- **Step 2 then matches and is the mechanism**: `mpdf-009` owns how the pane
  draws the page, its rollup is `partial` rather than `abandoned`, so this is a
  phase appended to it, numbered after the last and renumbering nothing. **It has
  its own review round and is not cleared to build until that converges.**

**It is numbered after Phase 3 and is intended to ship before it**, which §3
permits when building an earlier phase *measures* something that reorders them —
OQ-7's probe, run during Phase 2, is that measurement. The reason belongs in the
review record and is short enough to state here too: **Phase 3 hands the reader a
knob that multiplies this phase's quantity by the square of the scale.** A canvas
is 8.3 MB at fit-to-width on a 619 px pane and 33 MB at 200%.

**And the argument does not depend on Phase 3, which round 1 was right to press
on.** Phase 3 is `reviewed: null` with no round of its own; if it were cut, what
would remain is a measured 587 MB at 71 pages growing linearly and without bound,
which is sufficient on its own. The zoom makes it urgent; it does not make it
true.

**What is measured and what is not, stated before the design rather than after.**
Measured, by OQ-7's probe at a 619 px pane and `devicePixelRatio` 2: 8.3 MB of
canvas per page, 587 MB for 71 pages, 12–22 ms to rasterise one, 6,509 text items
across 71 pages. **Not measured, and round 1 caught the draft asserting it
anyway: what `getPage` costs.** OQ-7's table records raster, text-layer and
annotation-layer medians and no page-fetch figure at all; the draft's "some 5 ms
a page" was that table's *text-layer* median read as something else, and the
355 ms sizing budget derived from it is withdrawn. The sizing pass is a worker
round trip per page whose cost is unknown, gate clause 9 measures it, and **a
phase that finds it dominating has learned something no probe has yet shown.**
**Also not measured: the length at which the present pane actually fails.** 71
pages was survivable. OQ-8 carries it and this phase does not pretend to its
answer.

**One smaller design was considered and rejected, recorded so a later round does
not re-raise it** (§5): *render the band first and the rest in the background,
releasing nothing.* It buys the same first-paint win with no placeholders, no
decoupled box and neither of the two costs below — and it does not bound
anything, which is this phase's subject. It is the right answer to a latency
question and the wrong one to a memory question.

- **Scope:** **`app/dist/index.html`**, plus **one `#[ignore]`d test that writes
  the gate's long document** — round 1 found six of eight clauses keyed to an
  artifact that exists nowhere in the repo and has no recipe, which is a gate a
  second person cannot run. Nothing is vendored and `core` gains nothing.

  **A page's box is separated from its raster, and that is the decision the rest
  depends on.** Every page of the document gets a wrapper sized from its
  viewport — which `page.getViewport` yields without rasterising — so `#pages`'s
  child list, its `scrollHeight`, every `offsetTop` and `offsetHeight`, the 16 px
  gap, the hairline, the fit and the reader's anchor work over the **whole**
  document from the first frame. **This is Phase 2's wrapper being paid for**: the
  canvas is already a detail inside a box something else owns, so releasing it
  disturbs nothing that measures the page.

  **The box must not be allowed to depend on the raster**, which is what forbids
  the cheaper implementation. A wrapper taking its size from a canvas it no longer
  has would collapse, `scrollHeight` would shrink as the reader moved, and
  `applyAnchor` would restore a fraction of a height that no longer means
  anything. **A placeholder therefore carries `.logical`, `.natural` and `.view`
  exactly as a rendered page does** — `size()` divides by `natural.w` and writes
  `--total-scale-factor: NaN` without it, and `linkService.goToDestination`
  dereferences `wrapper.view`, so a cross-page link into an unrendered page throws.
  Round 1's catch, and all three are set by the sizing pass rather than by the
  render.

  **A budget decides whether to virtualise at all, and the draft had no such
  rule.** Round 1 re-derived the draft's premise and found it false: at the app's
  own default window the band reaches some 3,300 px and the showcase's six pages
  are 5,346 px of content, so a *correct* implementation would have left two pages
  blank and failed the very clauses the draft told it to re-run. So:

  > **If the whole document's raster fits the budget, every page is rendered, and
  > the pane behaves exactly as Phase 1 shipped it.** Otherwise only the band is.

  **The budget is 128 MB**, and the number is derived rather than chosen. A page
  costs about `5.66 × dpr² × paneWidth²` bytes for A4 — 6.1 MB at the default
  window's ~520 px pane at `devicePixelRatio` 2, 8.3 MB at OQ-7's 619 px, both
  matching the probe. 128 MB therefore admits the six-page showcase at 37 MB with
  room to spare, admits some twenty pages at the default window, and refuses the
  71-page document at 587 MB by a factor of four and a half.

  **The band is the floor, and when the two conflict the floor wins.** The band is
  the pages intersecting the scrollport plus one scrollport above and below. On a
  very wide pane a single page can cost 50 MB and the band alone can exceed the
  budget; a pane must draw what the reader is looking at, so the budget yields.
  **Retained bytes are therefore `min(whole document, max(band, budget))`**, which
  is the expression gate clause 2 evaluates from the tester's own geometry rather
  than a literal that only holds at one window size — round 1's finding against
  the draft's bare "100 MB", raised independently by two of the three lenses.

  **The band is observed, not computed from `scrollTop`.** An
  `IntersectionObserver` rooted on `#pages` with `rootMargin: '100% 0px'` reports
  which pages are near. This is `mpdf-003` Phase 7's rule — *the pane's geometry is
  observed and not inferred from the events believed to change it* — reaching the
  one axis it had not been applied to; a scroll listener computing intersections by
  hand is the same bug in a third place. **The margin is a percentage and not a
  pixel count**, because a percentage resolves against the root rect's own height
  and so needs no rebuilding when the window resizes, where a px margin would need
  the `ResizeObserver` to rebuild the observer and nothing today does.

  **The order at open is layout, then position, then render**, and round 1 found
  the draft silent on it with a bad consequence. `openPdf` today renders and *then*
  scrolls, so an observer cannot report a band before the scroll exists — on a long
  document every compile at the caret's page would swap in a blank wrapper and fill
  it a frame later. All three of OQ-2's cases know where the reader goes before any
  pixel is needed, so the sizing pass runs first, the scroll position is applied
  second, and the band is rasterised third. **The reader's own page is never a
  placeholder.**

  **How a band render composes with the shipped generation guard, named site by
  site** — round 1's finding that the draft restructured this file's most
  load-bearing function without naming it, where Phase 2 enumerated seven sites
  across six functions. `renderSeq` keeps its meaning: **the generation of the
  wanted document and geometry, not of an individual page.**

  - **`drawPages`** splits into a sizing pass and a band pass. It keeps its `seq`
    parameter, its per-await generation check, its prune and its closing
    `unscale(); fitted = paneWidth`.
  - **A scroll-triggered band render does not touch `renderSeq` and does not call
    `cancelRenders`.** It is work *inside* the current generation, not a new one.
    Bumping the sequence would abort a compile's sweep — after which `openPdf`
    returns false, `drawnRevision` never advances and the compile is redone — and
    calling `cancelRenders` would kill that sweep's page mid-render.
  - **Each page's task still joins `inflight`**, so a real supersession — a new
    document, a rest at a new width, `clear` — still cancels it through the one
    path that already does that.
  - **`renderPages` still brackets it**, so `rendering > 0` holds while a band
    render is in flight and `rerender` defers to it and re-arms `settle`, which is
    the behaviour Phase 1 argued into existence and this must not lose.
  - **`openPdf`, `clear`, `rerender` and the `ResizeObserver` keep their shape**;
    what changes is that `openPdf` orders the three passes above and `clear`
    disconnects the observer.

  **A page that has left the band before its render begins is skipped**, checked
  at the top of each page's turn in the band pass — which is what stops a fast
  scroll through seventy pages from enqueuing seventy renders.

  **The wrapper list is reconciled, not replaced, and the observer's lifecycle
  rides on that.** An `IntersectionObserver` holds a **strong** reference to every
  target, so a wrapper dropped without `unobserve` is retained with its backing
  store — a leak that gate clause 2's sum over `#pages` cannot see, which round 1
  caught. So the sizing pass reuses the wrappers already there, `unobserve`s and
  removes only the surplus when a document loses pages, and `clear` calls
  `disconnect()`. **The canvas is what is swapped inside a stable wrapper**, which
  is where Phase 1's *a canvas is swapped in only once it holds pixels* now lives,
  unchanged in meaning.

  **Leaving the band frees the backing store and takes both layers with it.**
  `canvas.width = canvas.height = 0` is what returns the memory — the vendored
  bundle uses that same idiom in four places for the same purpose — and it happens
  **before** the element is detached, never after, because a detached canvas is
  what the gate cannot measure. The text layer and the annotation layer go with it.

  **A placeholder is page-shaped, which needs a declaration.** `#pages .page`
  has no background of its own today and the white a reader sees is the canvas's,
  so an empty wrapper would render as `--ground` between two hairlines — a void
  rather than a page. It gains a paper-coloured background, which a rendered
  canvas then covers.

  **The gesture-and-rest split is untouched.** A width that is moving is answered
  by CSS, the band is re-rendered at the rest by `rerender`, and no band render
  runs during a gesture.

  **Two costs are recorded rather than hidden.** A selection cannot span pages
  that are not rendered, so *select all* reaches the band and not the document,
  and an out-of-band link is not clickable — `pdf.js`'s own viewer has the same
  property, and it is §2's ledger that this falsifies, which the close-out
  discharges. And a reader who jumps far sees a placeholder before they see the
  page.

- **Exit gate:** Run against **`samples/showcase/showcase.md`** (six pages,
  `/Count 6`) **and `tests/fixtures/long.md`**, written by this phase's
  `#[ignore]`d generator and compiling to **`/Count 71`** — both literals
  independently checkable on the compiled page tree. Read in the Web Inspector on
  a `cargo tauri dev` build. **Clauses keyed to the default window state so**,
  because every byte figure in this document is geometry-dependent and round 1
  found the draft's ceiling reproducible at no stated size.

  1. **At the default window the showcase is unchanged in every respect**, which
     the budget rule makes true rather than assumed: six pages at that geometry
     cost some 37 MB against a 128 MB budget, so every page renders and **Phase
     2's clauses 1, 2, 3, 5, 6, 7 and 8 and Phase 4's clause 1 re-run and pass
     verbatim** — six text layers, six annotation layers, twenty links, gaps of
     16 px, and the selection landing on the glyphs at three divider positions.
     Clause 2 of Phase 2 is in this list because this phase creates a new moment
     at which a layer is built, and clause 8 because a stale page is one of the
     four states that ride on these elements.
  2. **On `long.md` the retained backing store equals what the rule says.** Sum
     `Σ canvas.width × canvas.height × 4` over `#pages` and compare it to
     `min(71 × perPage, max(band, 128 MB))`, both computed from the tester's own
     `pages.clientWidth`, `pages.clientHeight` and `devicePixelRatio` — **not to a
     literal**. Take it at four moments: on open, after scrolling to the last
     page, after scrolling back to the first, and after a divider drag. The
     unbounded pane measures 587 MB at OQ-7's geometry, so the comparison
     discriminates by more than a factor of four wherever it is taken.
  3. **No canvas is retained outside `#pages`.** In the Web Inspector's memory
     timeline, open `long.md`, scroll from the first page to the last and back,
     force a compile ten times, then `clear` by opening the showcase: **the canvas
     allocation returns to the showcase's own footprint and does not step upward
     per compile.** This is the clause that sees the leak clause 2 structurally
     cannot — a wrapper dropped without `unobserve`, or a canvas detached before
     it was zeroed.
  4. **The scroll extent is right before anything beyond the band has rendered.**
     With `pages.scrollHeight` logged from a `requestAnimationFrame` armed before
     the open, its first value equals its value once every page has been visited.
     A hand reading is too slow for this and the clause says so.
  5. **The reader does not move when a placeholder fills in.** Park mid-document,
     record *(page, fraction)* with the app's own anchor arithmetic, wait for the
     band, and read it again — **not `scrollTop`**, which WebKit holds constant
     while content reflows above the reader and which therefore reports a pass in
     its own failure case. Round 1's catch.
  6. **OQ-2's three causes still land** where §3 resolves them, on `long.md`: an
     open at page 1, a keystroke at the caret's page **with that page drawn and
     not a placeholder**, and a divider drag and a window drag-resize at the same
     page and fraction without drift during either.
  7. **A jump to the last page draws it**, and a fast scroll does not thrash.
     Scroll from page 1 to page 71 in one throw and stop: page 71 and its
     neighbours are drawn, and the count of `render` tasks started over the throw
     is under twice the band's page count — logged by a counter, because "does not
     enqueue seventy renders" is otherwise unobservable.
  8. **Phase 1's clauses 2 and 3 re-run on `long.md`, scoped**: no page clipped at
     three divider positions with scrollbars shown, and the backing store exactly
     `floor(cssWidth × devicePixelRatio)` **for every page that has one**.
     **Phase 1's clause 5 is re-run only on the showcase**, where every page is
     rendered — on `long.md` a blank page is this phase's design and that clause's
     failure, and round 1 found the draft demanding both.
  9. **The sizing pass is measured, not assumed.** Time `getPage` across all 71
     pages and report it; the phase asserts no figure, and this clause exists to
     produce one. **It fails only if the total exceeds 1 s**, which is the point at
     which a document would open slower than Phase 1 draws it whole.
  10. `cargo test --workspace` passes unchanged. **The only `.rs` change is the
      `#[ignore]`d generator**, which the suite does not run — the pattern
      `core/tests/page_examples_test.rs:bless_the_generated_blocks` already
      establishes.

- **Close-out:** **§2's "the first two are rebuilt by Phase 2" takes a dated
  `CORRECTED` note in place**, per §6.1's step-1 prose bullet and Phase 4's
  precedent in the same section: they are rebuilt for a page in the band and for
  no other, and a reader of that ledger must not be left thinking otherwise.
  **OQ-1 is resolved by this phase** — it asked whether the pane renders every page
  or the pages near the reader, and this is the answer — inline per §4, with the
  note it already carries left standing.

  `rules/desktop-panes.md` gains the budget, the band, what a page outside it is,
  and the observer that reports it; its `covers:` gains them. **The cap is named
  now rather than deferred to the diff**, which round 1 asked for: the file sits at
  374 of `max_lines: 375`, and it **rises to 420**. If the pass does not fit in
  420, the geometry — the fit, the gesture and rest, the reader's place, the gap,
  and now the band — leaves for `rules/desktop-geometry.md` with its own `sources`,
  `covers` and index entry, and the panes file keeps the panes.

  **The in-file comments are the third place the invariant lives**, and Phase 2's
  close-out recorded that as a recurring miss: `drawPages`'s doc comment says
  "Draw every page of the retained document", the `#pages .page` comment describes
  a wrapper that always holds a raster, and both are corrected in the pass that
  changes them. `rules/desktop.md` is untouched — nothing is vendored and its file
  count does not move.

  **`README.md` gains a sentence**, on Phase 2's argument rather than Phases 3 and
  4's waiver: a reader who scrolls a long document fast and watches a page arrive
  has met a behaviour. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-009.md, append-only, one heading per round. See §7 of the
methodology.
-->
