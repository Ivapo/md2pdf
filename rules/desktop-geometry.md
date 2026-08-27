---
title: desktop-geometry
sources:
  - app/dist/index.html
covers: >
  the desktop pane's geometry and what it costs: the fit that is an expression
  rather than a state and the container it is measured from, the gap that tells
  one page from the next and the hairline that draws its edges, the one writer of
  a page's CSS box and of the property that carries its layers, the backing store
  derived from the logical size, the width a gesture carries by CSS size and a
  rest answers by a render, the reader's place held across both and what marks a
  gesture's start, the geometry the page observes rather than infers, the budget
  that decides whether the pane draws a document whole and the two passes that
  evaluate it, the layout separated from the raster, the pages the reader is near
  and the observer that reports them, the two terms a raster's freshness turns on,
  the one drawing pass and the rest it re-checks before every page, the release
  that zeroes a canvas and the two sweeps that catch one, and the surface the pane
  publishes for its own gate
max_lines: 240
generated: null
---

# Desktop geometry

How wide a page is drawn, how the pane answers a width that is moving, where the
reader is kept across that, and how much of a document the pane holds at once.
Its sibling `desktop-panes.md` has the panes themselves — the front end, the
renderer, the wrapper a page is, the panel and the gutter.

## The page's width

**The fit is an expression, not a state.** `scale = paneWidth / naturalWidth`,
recomputed on every render, so it cannot go stale — there is no stored fit to
disagree with the pane.

**`paneWidth` is `clientWidth` of `#pages`, the scroll container, and not of the
`#preview` column around it.** Six A4 pages make it scroll, and on a machine set
to show classic scrollbars its content box is some 15 px narrower than the
column's — the difference between a page that fits and a page clipped at its
right edge. `#pages` carries `overflow-y: scroll` rather than `auto` so that
width cannot move under the fit: with `auto`, a document sitting just under the
scroll threshold gains a scrollbar the moment the canvases widen, the content
box narrows by the track, and the pages clip.

**A page is separated from the next by 16 CSS pixels, and from the container's
sides by nothing.** The gap is `margin-top` on each page's
wrapper, plus `#pages::after` for the one below the last page — a pseudo-element
rather than a bottom margin, because whether a scroll container's last child's
margin reaches `scrollHeight` is engine-dependent and this ships on WKWebView,
and rather than a spacer element, because `layoutPages` addresses
`pages.children[n - 1]` and every geometry function, and the drainer with them,
assumes each child is a page carrying `.logical`. **`margin-top` is
the property and that is a decision**: `offsetTop` includes an element's top
margin, so the reader's place moves with the gap; `#pages` is a block formatting
context by virtue of `overflow-y: scroll`, so the first page's margin does not
collapse out of it; and only top margins exist, so no pair of siblings collapses
to less than the constant. It is a length rather than a ratio, because a
percentage resolves against the containing block's *width* and would track the
divider. **Nothing at the sides, and not because the width is wanted
elsewhere**: `clientWidth` *includes* padding, so side padding would leave
`paneWidth` unchanged, the canvases would be sized past the content box, and
they would clip silently. The gap is chrome and not content, so it is a constant
that does not scale with a gesture — `fit()` and `unscale()` know nothing about
it.

**Each page wears a hairline along its top and bottom edges and along neither
side**, `box-shadow: 0 -1px 0 var(--edge), 0 1px 0 var(--edge)`, painted into
the gap. A shadow rather than a border so that it costs no layout: under
`content-box` a border would make each canvas two pixels taller than the height
`size()` wrote, and `offsetHeight` — which the reader's place is expressed in —
would stop being the raster's own height. Top and bottom only because a
fit-to-width canvas meets the scrollport's own side edges, so there is no
background beside a page to separate it from, and an outset ring would spread
outside `overflow-x: hidden`'s clip anyway. `inset` is not the escape either: an
inset shadow paints beneath a replaced element's content, and a canvas is opaque
white where `pdf.js` filled it. The colour is named because a colourless
`box-shadow` resolves to `currentColor`, which inherits `--ink`.

**`size()` is the one writer of a page's CSS box and the one writer of
`--total-scale-factor`**, which is what carries the two layers through a gesture
without either of them knowing one is happening: `fit`, `unscale` and the render
all delegate here, so a gesture and a rest agree by construction rather than by
two functions being kept in step. A gesture produces no new viewport, so a layer
positioned from the render's would have stayed at the old size through every
drag. **The value is absolute** — the box's current CSS width over the page's
*unscaled* width, which is what `setLayerDimensions` multiplies
`rawDims.pageWidth` by — because anything relative is wrong by the render scale
at every width: `1` would put a 595 px layer over a 535 px page. **`--scale-round-x`
floors the layer to whole CSS pixels**, so at some pane widths that product lands
an ulp under the integer and the layer measures 1 px narrower than its page —
observed at a 716 px pane and not at 476 or 556, uniform across all six rather
than drifting down them, so a span at the right margin sits up to a pixel left of
its glyphs and one at the left margin sits on them.

**The logical size is whole CSS pixels and the backing store is derived from
it**, so the backing store is exactly `floor(cssWidth × devicePixelRatio)`. The
two are one number in exact arithmetic — `naturalWidth × (paneWidth /
naturalWidth)` is the pane width — but in floating point it lands an ulp under,
and flooring that costs a device pixel the CSS size still claims. Sharpness is
the display's own pixel ratio, and it is the thing a transform over a committed
raster could never do.

So the two halves of a resize are answered differently. **While a width is
changing the canvases are resized by CSS**, their backing stores untouched and
the browser resampling bitmaps it already has. A render costs some 94 ms for six
pages against a frame's 16.7 ms, so a render per `pointermove` would be
cancelled by the next and nothing would complete until the hand came off — and a
cancelled render on a just-resized canvas leaves it cleared, so the literal
reading blanks the pane through the drag. **Which property carries the gesture is
a decision and not a style**: a transform leaves layout alone, so the container's
`scrollHeight` would keep its pre-gesture extent and the reader's anchor would be
read against a stale one. CSS width makes the extent track the gesture. The
factor is read per canvas, because a compile landing mid-gesture swaps its pages
in one at a time and the list then briefly holds rasters from two widths.

**When the width rests the pages are drawn again**, sharp, from the retained
document. It fires on `pointerup` and, through one 200 ms timer, on a window
resize — a drag ends on an event and a window resize does not — and is skipped
when the width did not move, so a window resized by its height alone costs
nothing. **The timer waits for a render already running rather than cancelling
it**: opening a document unhides the pane, which is a resize, which arms the
timer, so it can come due mid-render and cancel the very render about to set the
fit. The count of running renders is a count and not a flag, because a superseded
render overlaps the one superseding it.

**The reader's place is a page index and a fraction within that page, never a
pixel offset.** The scale changes with the width, so every canvas's height and
the container's `scrollHeight` change with it; an offset held across that moves
the reader, and clamps at the bottom when a pane widens and then narrows.

**The anchor is taken when the gesture starts, not when the render begins.**
Resizing by CSS reflows, so a gesture of factor `s` maps a content offset `T` to
`T·s` while the browser holds `scrollTop` at `T` — on the app's default geometry
a 20% widen displaces a reader on page 5 by some 611 px, and WebKit implements no
scroll anchoring that would compensate. It is read before the first `fit` of a
gesture, while the canvases still hold their pre-gesture size, and reapplied on
every step as well as after the render that ends it. Taken at the render it would
faithfully preserve a position the drag had already ruined.

**What marks a start is `fitted` itself** — the width the raster now in the
canvases was made for. A width arriving while it differs opens a gesture; one
arriving while a gesture is open continues it. Nothing keys on `pointerdown`,
because a window drag-resize and a control taking width out of the column have no
pointer event to key on. A compile landing mid-gesture keeps the reader's place
and skips the caret jump, for that compile only.

**The pane's geometry is observed, not inferred from the events believed to
change it.** A `ResizeObserver` on the scroll container, not a list of listeners.
Two bugs came from the other approach and both had one shape — a control added
to the window changed a geometry something else had already measured. The
divider read `clientX`, measured from the window, which matched the pane's own
width until the panel sat to the left of it; and the scaled frame held its
explicit width while the panel and the gutter each took width out of the column
beside it. The divider now measures from the text pane's own left edge, taken
once at the grab. **The rule outlived the renderer it was found in.**

## What the pane holds

**A canvas costs 5.8 MiB at the default pane and nothing bounded that**: 71 pages
held 414 MiB, and a longer document more. The pane's width is inside the figure —
20 pages cost 116 MiB at 520 px and 211 at 700 — so **the budget is 128 MiB,
chosen and not derived**, admitting 21 pages at the default window and 15 at 619.
What would ground it is the length at which WKWebView jettisons its content
process, which nobody has measured. Under it a document is drawn whole and
nothing a reader sees changes; over it the pane holds the pages the reader is
near and lets the rest go.

**It is evaluated at the two passes that can move it, an open and a width rest,
and never once per document.** Five shipped causes move the pane's width — the
divider, a window resize, the panel folding, the gutter, and an open itself, the
panel being shown for any document naming sections. A pass after a scroll
re-evaluates nothing, correctly: scrolling moves no width.

**The layout is separated from the raster.** `layoutPages` sizes every page from
its own viewport and draws none, so the child list, `scrollHeight`, every
`offsetTop`, the gap, the fit and the reader's place are right over the whole
document from the first frame — the extent exact from layout alone, 53,337 px for
71 pages at 520. It commits in one mutation, a per-page append growing
`scrollHeight` across frames and giving the observer a delivery per page, and it
reconciles, so the elements the reader's place is measured against survive a
compile.

**Which pages the reader is near is observed and not computed**: an
`IntersectionObserver` on `#pages` at `rootMargin: '100% 0px'`, one scrollport
either side, a percentage so a resize needs no rebuild — three pages at either end
and five or six through the middle. It is established afresh after every sizing
pass, and disconnected by `clear` and by the pane going back to holding
everything. Not per drawing pass, which would deliver and never draw; and a
compile that *grew* the document would otherwise leave an observer blind to the
new trailing pages, so a page just written could never be drawn.

**A raster belongs to the document that drew it and to the width it was drawn
for, and either stale counts as absent.** The document term is a counter of its
own and not `renderSeq`, which every width rest bumps. Generation alone loses the
author's own edit; width alone leaves every page soft after a rest, which is the
pane's own *comes back sharp when you let go* regressed by a test that reads
correctly. The width term needs no property: a backing store is `floor(logical.w
× devicePixelRatio)` by construction, so `canvas.width` against that expression
is the test, and it survives a change of display.

**One drawing pass at a time, and the rest is re-checked before every page.** A
flag serialises them, naming one function not being enough to stop two
invocations overlapping. Checked only at entry the rest does nothing under a
throw — the re-entries refused while the running pass draws every page the reader
is flying past — costing forty-three renders where three will do. The rest is
120 ms beside the width's 200, chosen and not measured. An open and a width rest
force past it at entry *and* at every page, their own `scrollTop` write firing a
scroll event that would otherwise stop them at their second page; a programmatic
scroll cannot be told from a reader's by a flag, so following a link waits out one
rest.

**The order is the layout, the reader's place, then the drawing** — waiting for
the observer's first delivery, raced against three animation frames. **One frame
is not enough and the difference is visible**: a delivery is computed after the
animation frame callbacks of its own rendering turn and handed over in a task,
while the continuation of an awaited frame is a microtask of that same callback,
so a single frame always resumes first. The forced pass then reads an empty set,
draws nothing, and releases the whole document to bare paper until the next scroll
rest. **The race is what keeps it from hanging**: awaiting the delivery alone
never returns if the observer is disconnected before it delivers. Drawing before
positioning would spend the pass at `scrollTop` 0 and jump the reader onto a page
with nothing on it.

**A page is released by zeroing its canvas**, which is what gives the memory back,
and its two layers go with it, so a selection reaches only what the pane holds and
a link outside it is not there to click. The pass sweeps after every page drawn
and once more as it ends, a page dropped while its own render is in flight having
no canvas to release at that moment.

**The pane publishes `window.__pane`** — its mode, the renders started, what the
sizing pass and the last raster cost, when the observer first delivered, and the
canvases made and released. It ships because nothing in this repository reaches
this file, so the pane's gate is read by a person at a console.
