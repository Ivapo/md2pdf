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
    reviewed: 2026-08-26
    shipped: 2026-08-26
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

> **CORRECTED 2026-08-26, by Phase 5.** The sentence above is kept as it was
> written and its first clause is no longer true without a qualification.
> **The first two are rebuilt by Phase 2 only inside the budget.** For a document
> past 128 MiB the pane holds the pages the reader is near and releases the rest,
> and a released page has neither layer — so a selection cannot span pages that
> are not drawn, and a link on one of them is not there to click. What a reader
> can *see* is inside that set by construction, which is why the loss is bounded
> and why it is recorded here rather than treated as a reversal. Under the budget
> nothing changes and the sentence stands exactly as written.

**Measured and regained beside it**: the annotation API answers, and
`getOutline` returns **six entries** for the showcase — Typst emits document
bookmarks that nothing in this project currently reads.

## 3. Open questions

- ~~**OQ-1** — does the pane render every page, or the pages near the reader?~~
  **RESOLVED 2026-08-26, by Phase 5**, which answers *the pages near the reader,
  and only once a budget says it must*. The threshold this question asked for is
  a quantity and not a page count, exactly as the measurement below concluded:
  `whole ≤ 128 MiB` of canvas backing store, evaluated at an open and at a width
  rest, with the pane's width a term in it. Under it the document is drawn whole
  and nothing a reader sees changes; over it the pane retains the pages
  intersecting the scrollport plus one scrollport either side. **128 MiB is
  chosen and not derived**, and OQ-8 is still what would ground it. The
  question's own record follows, unaltered.

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
- **OQ-9** — is 120 ms the right rest for a scroll? Raised 2026-08-26 with
  Phase 5, whose prototype established that the rest is *needed* — without it a
  throw of 41 frames starts 43 renders against 5 — and could not establish its
  length. Too short and an ordinary slow scroll renders pages the reader is
  passing; too long and a reader who stops waits for the page under them. It sits
  beside the width's 200 ms and was chosen to be shorter because a scroll stops
  more often than a drag does. **Nothing measured it against a reader**, which is
  the only instrument that can. Blocks nothing. *(needs-input — it wants a
  reader's judgement in use, as `mpdf-003` OQ-6 was answered)*
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

  > **MEASURED 2026-08-26, by the probe this asked for, and the cliff was not
  > reached.** A ladder of 142, 284, 426, 568 and 852 pages, each opened twice at
  > a 520 px pane and `devicePixelRatio` 2 — once with the budget disabled, which
  > is the pane as Phase 1 built it, and once as Phase 5 ships it. The instrument
  > is Playwright's WebKit rather than WKWebView in the window, so these are the
  > engine's numbers and not the app's.
  >
  > | pages | uncapped | capped |
  > |---|---|---|
  > | 142 | 828 MiB, 3.9 s | 17 MiB, 1.7 s |
  > | 284 | 1,656 MiB, 7.3 s | 17 MiB, 1.7 s |
  > | 426 | 2,484 MiB, 12.3 s | 17 MiB, 1.7 s |
  > | 568 | 3,313 MiB, 18.7 s | 17 MiB, 1.7 s |
  > | 852 | **4,969 MiB, 36.5 s** | **17 MiB, 1.7 s** |
  >
  > **Uncapped, nothing failed.** 852 pages holding 4.85 GiB of backing store drew
  > and stayed answering. **So the question's own dichotomy is settled even though
  > its number is not**: 128 MiB is preventing a quantity from growing, not a
  > crash anybody has seen, exactly as §2 claimed and at least up to 4.85 GiB.
  > **Capped, both columns are flat** — three canvases, 17 MiB and 1.7 s at every
  > length on the ladder — which is the property that matters: retention and the
  > cost of an open stop being functions of how long the document is.
  >
  > **The ladder stopped at 852 rather than finding the cliff**, and the reason is
  > recorded rather than hidden: the next rung is 8.3 GiB against 7.8 GiB free on
  > the machine to hand, so it would have measured that machine's swap rather than
  > the engine's limit. The length at which WKWebView gives up is still unmeasured
  > and this question stays open for it.
  >
  > **What it does settle for Phase 3, which is why it was run now.** The budget
  > bounds *whether* the pane draws a document whole; **nothing bounds the retained
  > set itself**, and its cost is quadratic in the scale — re-derived here and
  > confirmed against two measured widths, 5.832 MiB a page at 520 px and 10.574 at
  > 700, a ratio of 1.813 against (700/520)² = 1.813. So a six-page retention costs
  > 35 MiB at fit-to-width and **560 MiB at 400%**, on a budget of 128. A zoom that
  > re-derives the scale without re-deciding what is held would put the pane back
  > where Phase 5 found it. Phase 3 must either bound the retained set or evaluate
  > the budget against the zoomed cost, and its review round is where that is
  > argued.

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

**Redrafted 2026-08-26, from a prototype, before its review round.** The phase was
scoped in twelve lines before Phase 5 existed, and Phase 5 put a budget under the
pane that those twelve lines do not mention. Rather than let a review round
discover that — which is how Phase 5 came to cost five rounds and sixty-one
findings — the three modes were built onto the shipped front end and driven, and
every quantity below is a reading. `reviewed` stays `null`; this is the draft the
round will argue with.

- **Scope:** Fit-to-width, fit-page, and an explicit scale, as the three modes
  `armquill`'s viewer names. The mode is the pane's state, and a width that moves
  re-derives the scale under the first two and leaves it alone under the third.

  **Fit-to-width stays the default**, because it is what the pane has always
  approximated and what an author writing to a page width wants.

  **The scale is chosen in `layoutPages` and nowhere else.** It already computes
  `paneWidth / naturalWidth` per page; the mode decides which expression it uses,
  and everything downstream — the CSS box, `--total-scale-factor`, the backing
  store, the anchor, the budget — follows from the viewport it produces. Measured:
  the three modes are one expression each and no other function needs to know the
  mode, with two exceptions the prototype found and which are named below.

  **A gesture must not resize a page the reader pinned a size to.** `fit()` scales
  every page to the pane on every `pointermove`, which is right under the two
  derived modes and wrong under a pinned one — the reader asked for 400%, not for
  whatever 400% becomes when the divider moves. It returns early under the pinned
  mode. **Measured: a pane dragged from 520 to 700 at 400% left the page at
  2,381 px, the retained set at 244.7 MiB and the reader on page 36 fraction
  0.500.**

  **A page wider than the pane must be reachable, and today it is not.** `#pages`
  ships `overflow-x: hidden`, so at any scale above fit-to-width the page is
  clipped at the pane's right edge with no way to reach the rest of it. It becomes
  `overflow-x: auto`. **Measured: at 400% the container reports `scrollWidth`
  2,381 against a `clientWidth` of 520, and scrolling fully right reaches the
  page's own edge; at fit-to-width nothing overflows and the pane still measures
  520, so the change costs the default nothing.**

  **Phase 4's hairline argument is touched and must be re-argued rather than
  assumed.** It reasoned that a side hairline was unpaintable because an outset
  ring would spread outside `overflow-x: hidden`'s clip. Under `auto` that clip is
  gone, so the reason changes even though the conclusion may not: at fit-to-width
  a page still meets the scrollport's own side edges, so there is still no
  background beside it to separate it from — but a ring would now produce a pixel
  of horizontal overflow rather than being clipped, which is a different objection
  and belongs in this phase's own prose.

  **A mode change must carry the reader, and the anchor as shipped will not.**
  `anchor` is gesture-scoped: it is opened by a width arriving that differs from
  `fitted` and closed by the render that ends the gesture, and a mode change is
  neither. **Measured, and this is the prototype's plainest failure: a reader at
  page 36 fraction 0.499 who switched from fit-to-width to 200% landed on page 16
  fraction 0.684.** A mode change reads the anchor before the layout and applies
  it after, exactly as a width rest does.

  **What a scale costs, and the decision this phase turns on.** Phase 5 bounds
  *whether* the pane draws a document whole; nothing bounds the retained set, and
  its cost is quadratic in the scale. Measured on a 71-page document at a 520 px
  pane and `devicePixelRatio` 2:

  | scale | one page | pages held | retained |
  |---|---|---|---|
  | 100% | 7.6 MiB | 4 | 30.6 MiB |
  | 200% | 30.6 MiB | 3 | 91.8 MiB |
  | 300% | 68.8 MiB | 2 | **137.7 MiB** |
  | 400% | 122.4 MiB | 2 | **244.7 MiB** |
  | 500% | **191.1 MiB** | 1 | 191.1 MiB |

  **The retained set crosses the 128 MiB budget at about 290%, and above about
  410% a single page exceeds it on its own** — at which point holding fewer pages
  is no longer a lever, because one is the floor. So a zoom that only re-derives
  the scale puts the pane back exactly where Phase 5 found it, and **this phase
  must decide what to do about it.** Three shapes, none costed, and the round is
  where one is chosen:

  - **Cap the scale** at whatever keeps one page inside the budget — simple,
    checkable, and it takes a capability away from the reader for a reason they
    cannot see.
  - **Lower the backing store when zoomed** — render at less than
    `devicePixelRatio` above some scale, which spends the sharpness Phase 1 exists
    for, and spends it exactly where a reader zoomed in to look closely.
  - **Re-derive the budget against the zoomed cost and accept a larger one**,
    which is honest only if OQ-8 ever finds the ceiling it is still looking for.

  **What is not in this phase.** No pinch gesture and no scroll-wheel zoom — the
  mode is set by a control, and a gesture is a separate subject with its own
  device questions. No per-page scale. No rotation.

- **Exit gate:** Run against **`tests/fixtures/long.md`** (71 pages) for the
  retention clauses and **`samples/showcase/showcase.md`** for the rest.

  1. Each mode holds across a divider drag and a window resize: under the two
     derived modes the scale re-derives, and under the pinned one the page keeps
     its size to the pixel.
  2. **Fit-page shows a whole page, read at a pane wider than 745 px.** Below
     that it is not a distinct mode and the clause tests nothing: a page fits the
     *width* constraint first, so at the app's default 520 px pane fit-page and
     fit-to-width produce the identical 520 × 735 box — measured. At a 900 px pane
     they separate, 745 × 1054 against 900 × 1273.
  3. The pinned scale survives a re-render after a compile, and a divider drag
     leaves the page's CSS width unchanged to the pixel.
  4. **A page wider than the pane is reachable**: at 400% the reader can scroll to
     its right edge, and at fit-to-width the container does not overflow and the
     pane still measures what it measured before this phase.
  5. **A mode change leaves the reader on the same page and fraction**, ±0.01,
     in every direction between the three modes.
  6. **Phase 5's clauses 2, 3, 9, 10.1 and 13 re-run at a pinned scale**, which is
     where this phase can break them: retention is the mode's own set, nothing is
     retained outside `#pages`, layers track canvases, the rest never empties the
     pane, and a compile lands the reader on a drawn page.
  7. **Whatever §4 decides about the budget is gated here**, and the clause is
     written when the decision is made rather than now.
  8. **Phase 4's clause 1 re-run**: the gap reads 16 px throughout at every mode
     and scale, the gap being chrome and not content.
  9. `cargo test --workspace` passes unchanged — no `.rs` file is edited.

  Clauses 1–8 are manual, per `mpdf-003` OQ-10, and the pasteable script Phase 5
  shipped its gate as is the instrument.

- **Close-out:** `rules/desktop-geometry.md` gains the three modes, the scale
  expression each derives, the overflow the pane now allows and whatever §4
  settles about the budget. **`rules/desktop-panes.md` is touched too** if Phase
  4's hairline reasoning changes, that prose having stayed with the pages rather
  than the geometry. The README gains nothing — a zoom is not what the app is
  for. One push.

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
*Produces the observable: **yes**, and the claim is now stated for the case it is
true of. **A document under the budget is rendered whole and nothing a reader sees
changes.** For a document past it, what improves is not the first page — Phase 1
appends each page as it is drawn, so page 1 has always arrived after page 1's own
render, measured at 42 ms — but **the page the reader is taken to**, which today
waits for all 71 because `applyAnchor` runs after the loop. That wait is 1,182 ms
in the harness. Two earlier drafts of this sentence compared against the wrong
baseline; round 3 caught the third.*

**Written from a prototype and revised from a second one.** Rounds 1 and 2
returned twenty-three blocking findings, round 3 eighteen more, and the pattern
each time was a design being settled in prose. So the mechanism was built and run
against the real vendored `pdf.js`, a 71-page document and a 20-page document
sitting near the budget. **Every quantity below is a reading, and four decisions
below exist because the prototype failed without them.** The harness is Chromium;
the gate closes that, and clause 12 says how.

**Appended after Phases 1, 2 and 4 shipped**, per §6.1's ordered test. Step 0
matches: OQ-1 decided that Phase 1 would build no virtualisation and OQ-7's
measurement reverses it. Step 1's phase-removing bullets do not match — nothing
shipped is removed and no phase is cut. **Step 1's prose bullet matches once**:
§2's "the first two are rebuilt by Phase 2" stops being true outside the band and
takes a dated `CORRECTED` note. **Five other places carry a related claim and none
takes a note**, each for its own reason: OQ-7's resolution and Phase 1's scope are
history §6.1 forbids rewriting; Phase 2's opening paragraph restates §2's ledger
and is history for the same reason; the frontmatter `note` has nowhere to put a
dated note and is amended with `last_updated` instead; and the three in-file
comments are `rules`-class facts the close-out corrects in place. Round 3 found
the previous draft counting four and listing five, and missing two. Step 2 is the
mechanism: a phase appended to the spec that owns the subject.

**Numbered after Phase 3, intended to ship before it**, per §3 — OQ-7's probe is
the measurement that reorders them. Phase 3's zoom multiplies this quantity by the
square of the scale. **The argument does not need Phase 3**: without it, 414 MiB
measured at 71 pages, growing without bound, still stands.

- **Scope:** **`app/dist/index.html`**, plus **one `#[ignore]`d generator and one
  non-ignored test that pins what it writes**, both in `core/tests/`.

  **A page's box is separated from its raster.** Every page gets a wrapper sized
  from its viewport, so the child list, `scrollHeight`, every `offsetTop`, the
  16 px gap, the fit and the anchor work over the whole document from the first
  frame. **Measured: the extent is exact from layout alone** — 53,337 before any
  raster and 53,337 after all 71 pages, at a 520 px pane.

  **The sizing pass commits in one mutation.** A per-page append grows
  `scrollHeight` across frames and gives the observer 71 deliveries instead of
  one. **Measured: 3.4 ms for 71 pages.**

  **A placeholder carries `.logical`, `.natural`, `.view` and its page number.**
  **Measured: following a destination into unrendered page 65 lands at fraction
  0.525 and does not throw.**

  ### The six rules the mechanism turns on

  Four carry observed-failure provenance — the harness broke without 1, 3, 4
  and 5. Rule 2 is round 3's reasoning confirmed by
  measurement, and rule 6 is composition with the shipped file, which the harness
  cannot test. Round 5 asked for the provenance to be told apart, having caught
  rule 6 claiming an observation the harness observably lacks.

  **1. A canvas belongs to the document generation *and* the width that drew it.
  Either stale counts as absent.** **The generation is a document counter and not
  `renderSeq`** — round 4 caught the previous draft saying only "the generation",
  and `renderSeq` is what an implementer reaches for. It cannot be `renderSeq`:
  `rerender` bumps that at every width rest, which would invalidate every canvas
  and make the bug below impossible. The width term needs no new property —
  `canvas.width === Math.floor(wrapper.logical.w * devicePixelRatio)` is the
  test, and it is what clause 10 measures; the harness carried a `forWidth`
  property instead, which the spec drops deliberately, the derived test being the
  one that survives a display change. Generation alone loses nothing at a
  recompile — that much a reader would notice — but **width alone was the bug
  that shipped in the harness: after a rest at a new width, whole-mode pages kept
  a canvas made for the old one, six pages at a 1120 backing store where 1400 was
  wanted, every one of them soft.** That is Phase 1's own *comes back sharp when
  you let go*, and its gate clause 3, regressed by a freshness test that looked
  correct. With width in the test: **the round trip 520 → 700 → 520 returns to
  116.64 MiB, sharp, exactly where it opened.**

  **2. The budget is evaluated at the two passes that can move it — an open and a
  width rest — and never once per document.** A scroll-rest drain is a pass and
  re-evaluates nothing, correctly: scrolling moves no width. `whole ≤ 128 MiB`
  is a function of pane width, and five shipped causes move it — the divider, a
  window resize, the panel folding, the gutter, **and an open itself**, since
  `parts()` shows the panel for any document naming sections and `report` runs
  before the bytes are fetched, so the width is already correct when the pass
  evaluates. **Measured on a 20-page document: whole at a 520 px pane (20
  canvases, 116.64 MiB), band at 700 (3 canvases, 31.72 MiB), whole again back at
  520, sharp throughout, with the boundary at a 545 px pane.** Crossing to band
  establishes the observer and releases what falls outside; crossing to whole
  disconnects it and fills the rest in. **128 MiB is chosen, not derived** — it
  admits 21 pages at the default window and 15 at 619 px, and OQ-8 is what would
  ground it.

  **3. A `draining` flag serialises every pass, and the open-and-rest path forces
  past the scroll rest.** Round 4 caught the previous draft recording the
  prototype's *measurement* where its *rule* belonged: "one drainer" names one
  function, which does not stop two invocations overlapping. **The flag does** —
  a pass entered while one is running returns immediately. And `force` suppresses
  the rest check **at entry and at every per-page check**, not only at entry:
  suppressed at entry alone, the open's own `scrollTop` write stops the forced
  pass at its second page.

  `applyAnchor` and `goToDestination` write `scrollTop`, and **a programmatic
  scroll fires a scroll event that cannot be told from a reader's by a flag** —
  the prototype tried a microtask-cleared flag and the event outran it every
  time. Following a link therefore waits out the rest like any other scroll,
  which is accepted rather than fixed: it costs one rest and it keeps the rule to
  one sentence. **Measured: zero concurrent passes across every experiment,
  including a resize deliberately overlapped with a scroll.**

  **4. The rest is re-checked before each page, not at the top of the pass.** At
  entry only it does nothing: a 41-frame throw refused re-entry 39 times while
  the running pass rendered 40 pages anyway. **Measured: 43 renders become 5.**
  The rest is **120 ms** beside the width's 200 ms and is chosen, not measured;
  OQ-9 carries it.

  **5. After every rendered page — and once more as the pass ends — the pass
  releases each page holding a canvas that is no longer wanted.** The harness
  carries both sweeps; the previous draft recorded one, and described only
  "leaving the band frees the backing store", which round 4 showed leaks: a page
  dropped from the wanted set while its render is in flight has no canvas to
  release at that moment, and the callback reports only what changed, so nothing
  revisits it. The per-page sweep collects it; the closing sweep collects a band
  that shrank with nothing left to render.

  **6. The drainer does not go through `renderPages` and does not touch
  `rendering`.** `rerender` defers while `rendering > 0`; a band pass inside that
  bracket would re-arm the 200 ms timer for as long as it ran. **It must swallow
  what it fails at, catching `RenderingCancelledException` as the shipped loop
  does — and this is a rule for the app that the harness observably does not
  follow**: the harness has no catch on its drain path and lets a cancelled
  task's rejection propagate unhandled, which round 5 caught this rule claiming
  as an observation. A timer-fired drain has no awaiter to reject into;
  `rerender`'s "cosmetic, so it swallows what it fails at" is the precedent.

  ### The rest of the mechanism

  **The band** is the pages intersecting the scrollport plus one scrollport either
  side, from an `IntersectionObserver` rooted on `#pages` with
  `rootMargin: '100% 0px'` — observed rather than computed, which is `mpdf-003`
  Phase 7's rule on the one axis it had not reached, and a percentage so a resize
  needs no rebuild. **Measured: 3 pages at either end, 5 to 6 in the middle
  depending on alignment.** The callback **maintains** the wanted set rather than
  rebuilding it, since after the first delivery it carries only what changed.

  **The order at open is layout, position, then band**, and **the band pass waits
  one animation frame after `observe()` rather than awaiting a delivery promise** —
  which is what the prototype does, and round 4 showed why it matters: a promise
  that resolves only in the observer's callback never settles if that observer is
  disconnected first, stranding `rendering` above zero and deferring every future
  width rest for the life of the window. A frame is enough because the delivery
  lands inside it. **The frame-wait belongs to the width rest as much as to the
  open** — the harness performs observe → one frame → forced drain on both paths,
  and round 5 caught the previous draft stating it for the open only: without the
  frame, a rest's forced pass runs against a just-cleared wanted set, finds
  nothing, and the real work rides an un-forced drain the reader's own scrolling
  defers — defeating rule 3's promise for exactly the path it names. **Measured: 0.4–6.8 ms, before the first frame**, and
  the reader's own page is drawn rather than a placeholder. **Measured at a
  recompile mid-document: 5 renders, every canvas current, the reader's page
  drawn.**

  > **CORRECTED 2026-08-26, after Phase 5 shipped**, by a code review of the
  > commit that shipped it. The paragraph above is kept as it was written and its
  > central claim is false. **A frame is not enough, and "the delivery lands
  > inside it" is the wrong way round**: an observer's records are computed
  > *after* the animation frame callbacks of the same rendering turn and handed
  > over in a task of their own, while the continuation of an awaited
  > `requestAnimationFrame` is a microtask of that very callback. The
  > continuation therefore always resumes first, and the forced pass reads a
  > `wanted` set that is still empty — draws nothing, and its closing sweep
  > releases every page in the document. **Measured on the shipped build, both
  > engines: eight consecutive frames of bare paper at every width rest in band
  > mode**, ending only when the 120 ms rest fired an unforced pass.
  >
  > What ships instead is the delivery raced against three frames, which keeps
  > the hang this paragraph correctly feared — an observer disconnected before it
  > delivers never settles its promise — while no longer reading the set before
  > it exists. `sweep` also refuses an empty set while pages are present, so the
  > failure is unreachable rather than merely unlikely.
  >
  > **The harness had this defect too, and five review rounds and this phase's
  > own gate all missed it**, which is the finding worth keeping: every clause of
  > that gate samples after the pane settles, and this is a defect that is only
  > visible while it has not.

  **A page's canvas and both layers are built detached and swapped in one
  `replaceChildren(fragment)`**, the outgoing canvas zeroed before it is dropped.
  Appending — which `drawLayers` does today, safe only because Phase 2's wrapper is
  new each pass — doubles both layers under a wrapper that persists. **Measured
  over six enter-and-leave cycles and across recompiles: canvases, text layers and
  annotation layers stay equal.** **Release is `replaceChildren()` after the zero**,
  so the layers go with the canvas; *select all* reaching only the band depends on
  it.

  **The observer is re-established — disconnected and created afresh over the
  reconciled wrapper list — after the sizing pass, at an open and at every width
  rest, while the mode is band; and disconnected by `clear` and when the mode
  crosses to whole.** Round 5 caught the previous draft saying "on every pass",
  which in this section's own vocabulary means a *drain* pass — and re-establishing
  per drain is an establish/deliver loop that renders nothing. **This is also a
  deliberate divergence from the harness, which re-establishes only at a mode
  crossing and at a resize, and that omission is a live bug in it**: a band-mode
  recompile that *grew* the document reuses the old observer, whose target list
  does not include the new trailing wrappers, so a page the author just wrote can
  never render. Re-establishing after every sizing pass is what closes it, and a
  fresh `observe()` over the reconciled list is also what drops the surplus with
  the old observer, no `unobserve` by hand. It holds strong references to its
  targets, and **the prototype reproduced that by omission too**: an open that left
  the previous document's observer attached had it release a page out of the next
  document.

  **The wrapper list is reconciled** — a compile reuses the elements already
  there and removes only the surplus, the observer's re-establishment dropping the
  old registrations with the old observer. The one-shot commit is per structural
  change, not per pass.

  **A band render's task joins `inflight`**, so `cancelRenders` reaches it, and
  every await re-checks `renderSeq` as every loop in this file already does.

  **The pane exposes a small debug surface, and that is a decision rather than an
  oversight.** `mpdf-003` OQ-10 records that nothing in this repository reaches
  this file, so its gate is read by a person; three of the quantities that gate
  needs — renders started, the sizing pass's duration, the observer's first
  delivery — live inside a `<script type="module">` and are unreachable from the
  console. So the module publishes `window.__pane`, carrying `mode`,
  `renders`, `sizingMs`, `deliveryMs`, `renderMs`, `made` and `released` — the
  durations in milliseconds — `sizingMs` the whole sizing pass from first
  `getPage` to the commit, `deliveryMs` from `observe()` to the first callback,
  `renderMs` the last band page's raster alone — and `made`/`released` counting
  canvases created and zeroed.
  Round 4 added the final three: without `renderMs` one of the engine re-takes had
  no instrument, and `made`/`released` replace a memory-timeline reading whose
  category Safari does not present. It ships, and without it half this gate has no
  instrument.

  **A placeholder is paper-coloured.** `#pages .page` has no background today and
  the white a reader sees is the canvas's, so an empty wrapper paints `--ground` —
  tolerable in light and a dark rectangle where paper belongs in dark. The
  prototype carries a `--paper` fill and the previous draft dropped it.

  **Two costs, recorded.** A selection cannot span pages that are not rendered and
  an out-of-band link is not clickable. And a reader who jumps far sees a
  placeholder for one delivery plus one render.

- **Exit gate:** **Preconditions, and round 4 found the previous draft's block
  unrunnable.** `#pages` ships `hidden` and only `openPdf` clears it, so a check
  pasted "before anything else" reads `clientWidth === 0`. And the pane is not one
  number: `#files` is shown for any document naming sections, so the **showcase**
  sits at roughly 305 px while `long.md` and `near.md` — **single-file by
  construction, which is why the generator writes them that way** — sit at 520.

  So: open `long.md`, then assert **`devicePixelRatio === 2`**,
  **`pages.clientWidth === 520`** (which subsumes the always-show-scrollbars
  setting, since overlay scrollbars give 535) and **`pages.clientHeight` between
  1001 and 1126**, outside which the band is not 3 at the ends and 5–6 in the
  middle and every retention literal below is void. The band is also computable
  directly — the pages intersecting `[scrollTop − H, scrollTop + 2H]` — and a
  tester who finds the height outside that range should read it that way instead.

  Fixtures: **`samples/showcase/showcase.md`** (`/Count 6`), **`tests/fixtures/long.md`**
  and **`tests/fixtures/near.md`**, the last two written by the `#[ignore]`d
  generator and **pinned by one non-ignored test asserting `/Type/Pages/Count 71`
  and `/Type/Pages/Count 20`** — keyed to `/Type/Pages` because outline nodes carry
  `/Count` too. **That test also resolves the long cross-reference out of the
  compiled bytes and asserts its page and coordinate**, the technique Phase 2's
  round 1 used to derive "458 pt down an 841.89 pt page"; **clause 8's literals are
  then read from what it pins rather than from this document**, which round 4
  showed cannot supply them. The pinning test compiles a 62,000-word document at
  every `cargo test --workspace` — **7.12 s in a debug build, measured by OQ-1** —
  and that cost is accepted here rather than discovered later.

  `window.__pane` carries **`mode`, `renders`, `sizingMs`, `deliveryMs`,
  `renderMs`, `made` and `released`**; the last three are round 4's additions,
  `renderMs` because clause 15's third reading had no instrument and `made`/
  `released` because clause 3's did not exist.

  1. **The showcase is unchanged**, at its own geometry: `#files` shown, pane ~305,
     whole mode either way. **Phase 2's clauses 1–8 and Phase 4's clause 1 re-run
     and pass verbatim**, plus **Phase 1's clause 7** — the empty, failed and stale
     states, and a second document opened over the first — which this phase changes
     what `clear` must do.
  2. **Retention equals `whole ≤ budget ? whole : band`.** On `long.md`, summing
     `Σ canvas.width × canvas.height × 4` over **`#pages canvas`**: **17.5 MiB at
     the top, 29.16–34.99 mid-document, 17.5 at the last page**, inclusive ranges
     because the band is 3 at the ends and 5–6 in the middle. Against 414 MiB whole.
  3. **Nothing is retained outside `#pages`.** After scrolling `long.md` end to end
     and ten compiles: **`__pane.made − __pane.released` equals the count of
     canvases in `#pages`**. Exact, and it does not depend on the platform's memory
     accounting — round 4 found the previous draft naming a "canvas allocation" line
     that Safari's memory timeline does not present, which was clause 2's blind
     instrument reproduced in its replacement.
  4. **The sizing pass commits once.** A `MutationObserver` on `#pages`
     (`childList`) armed from the empty state: **one record, `addedNodes.length`
     71**, not 71 records.
  5. **The extent is exact from layout alone**: `scrollHeight` **53,337** when the
     sizing pass resolves and after every page has been visited.
  6. **The reader does not move when canvases are released.** Park mid-document,
     record *(page, fraction)*, scroll away and back, read it again: same page,
     `|Δfraction| < 0.01`. **Keyed to the release direction**, since a placeholder
     is already at its final size and filling one in reflows nothing — round 4
     found the fill direction near-vacuous against clause 5.
  7. **A throw does not thrash.** A scripted ramp — 40 `requestAnimationFrame`
     steps of `scrollHeight / 40` — then stop and poll `__pane.renders` until it
     settles. It advances **by at most 8** from the end of the open's own band,
     against 43 for a rest checked only at the top of the pass.
  8. **A cross-reference into an unrendered page lands at its coordinate**, the page
     and fraction being the ones the pinning test asserts, ±0.01, and no error
     reaches the console.
  9. **Layers track canvases across release, re-entry and recompile.** Six round
     trips, each allowed to settle, **and ten compiles**: every page with a canvas
     has exactly one text layer and one annotation layer, every page without has
     neither.
  10. **A width gesture on `long.md`, at 400, 520 and 700 px**, set with
      `text.style.flexBasis` so they are exact and all above the **289 px** boundary
      below which a correct implementation is in whole mode. After each rest every
      page holding a canvas satisfies `floor(cssWidth × devicePixelRatio)`, the
      reader holds *(page, fraction)*, `__pane.mode` is `band`, and no error reaches
      the console — `.natural` missing throws inside the `ResizeObserver` on the
      first move. **This is the clause the width half of freshness fails.**
  11. **The budget is crossed in both directions, read at the top of the document**
      — round 4 found the previous draft naming no position, where mid-document at
      700 px gives 4–5 pages rather than 3. On `near.md`: **whole at 520 px, 20
      canvases, 116.64 MiB; band at 700 px, 3 canvases, 31.72 MiB; whole again at
      520.** An implementation deciding once per open reads 20 canvases and
      211.5 MiB at 700.
  12. **An edit reaches the screen.** Scroll `long.md` to mid-document, edit the
      text under the reader, and after the compile: **the page under the reader
      shows the edit** — Phase 1's clause 8 keyed to the drawn canvas — `__pane.renders`
      advances by the band and not by 71, `#pages.children.length` is still 71 and
      the same element objects (held across the compile), and clause 9's equality
      holds. **This is the clause the generation half of freshness fails**, and
      without it an implementation that renders only pages holding no canvas passes
      everything and loses the author's work.
  13. **An open lands the reader on a drawn page.** Recompile at mid-document and
      assert the landing page holds a canvas at the first frame after the open
      resolves. **This is the phase's headline claim** and the previous draft gated
      it nowhere: an implementation leaving `openPdf`'s positioning where it is
      drains the band for `scrollTop` 0, jumps the reader, and shows a placeholder
      until the rest fires — reaching an identical end state.
  14. **Two paths do not overlap.** Drag the divider while scrolling: after the
      rest, `__pane.renders` advanced by at most the band, layers equal canvases,
      and every canvas satisfies clause 10's expression.
  15. **The engine-dependent readings are re-taken.** `__pane.deliveryMs` (Chromium
      0.4–6.8), `__pane.sizingMs` (3.4, failing above 250) and `__pane.renderMs`.
      **The render comparison is not Chromium-versus-WebKit**: OQ-7's 12 ms a page
      was measured *in the window*, at a **619 px** pane, so at 520 the same
      implementation predicts about **8.5 ms** and the comparison is against that.
      **A disagreement on these three is a finding. On every other clause a
      disagreement is a failure.**
  16. `cargo test --workspace` passes, with the one new non-ignored pinning test and
      the generator `#[ignore]`d.

- **Close-out:** **§2's "the first two are rebuilt by Phase 2" takes a dated
  `CORRECTED` note**; the frontmatter `note` is amended with `last_updated`.
  **OQ-1 is resolved by this phase**, inline per §4. OQ-9 already carries the
  120 ms rest.

  `rules/desktop-panes.md` gains the budget, the band, the two freshness terms,
  the observer and `window.__pane` — **and loses three sentences this phase and Phase 2 falsified**:
  "a scroll container holding one `<canvas>` per page", "The gap is `margin-top` on
  each canvas" and "each child is a canvas carrying `.logical`", the last two
  already stale since Phase 2. The cap rises from 375 to **420**, budgeting for
  rewrites and not only additions; if the pass does not fit, the geometry leaves
  for `rules/desktop-geometry.md` with all five §8.1 keys and `generated: null`.
  **The rule must not call this a "band"** — that word already means the caret
  line's highlight there, and `--band` is its colour in the stylesheet.

  **Five in-file comments go false, and round 4 found the last two**:
  `cancelRenders`'s "a task cancelled here rejects inside `renderPages`" stops
  being true once a band task joins `inflight`, and the `rendering` comment turns
  on the drainer staying outside that bracket. The other three are `drawPages`'s
  "Draw every page of the retained document", the `#pages .page` block, and **`drawLayers`'s "built into
  the page while it is still detached"**, which a fragment and a persistent wrapper
  both change. Round 3 found the third; this record has caught a missed third copy
  four times now.

  **`README.md` is corrected, not extended.** Its app section says "Select and
  copy from it as you would in any PDF reader", which *select all* on a long
  document falsifies; the cross-reference half survives, since a link the reader
  can see is inside the band. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-009.md, append-only, one heading per round. See §7 of the
methodology.
-->
