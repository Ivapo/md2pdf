---
title: desktop-geometry
sources:
  - app/dist/index.html
covers: >
  the desktop pane's geometry and what it costs: the three fits the reader
  chooses between and the one expression each derives, the control that offers
  them and the cap that is its own last option, the container every width is
  measured from and the axis a zoomed page may overflow, the gap that tells one
  page from the next, the ring that draws a page's edges and the margins that
  centre one narrower than the pane, the one writer of a page's CSS box and of
  the property that carries its layers, the backing store derived from the
  logical size, the width a gesture carries by CSS size and a rest answers by a
  render, the reader's place held across both, the one comparison that opens a
  gesture and passes a fit change through, the geometry the page observes rather
  than infers, the budget that decides whether the pane draws a document whole
  and the three passes that evaluate it, the layout separated from the raster,
  the pages the reader is near and the observer that reports them, the two terms
  a raster's freshness turns on, the one drawing pass and the rest it re-checks
  before every page, the release that zeroes a canvas and the two sweeps that
  catch one, and the surface the pane publishes for its own gate
max_lines: 340
generated: null
---

# Desktop geometry

How big a page is drawn and who decides, how the pane answers a width that is
moving, where the reader is kept across that, and how much of a document the
pane holds at once.
Its sibling `desktop-panes.md` has the panes themselves — the front end, the
renderer, the wrapper a page is, the panel and the gutter.

## The page's size

**There are three fits and the reader picks one**: `fitMode` is `width | page |
manual`, with `fitScale` beside it under the third. What moves re-derives under
the first two and is held under the third. **Fit-to-width is the default** and
an Open puts it back — an open is not a zoom request — which `clear()` does
with the rest of the pane's state.

**The word is "fit" and never "mode".** `mode`, `decideMode` and `__pane.mode`
are the whole-or-band retention below, a different question about the same
pane, and a second `mode` here would hand a reader the wrong variable at the
first grep.

**The scale is chosen in `layoutPages` and nowhere else, one expression per fit
and all three in `scaleFor`**: `paneWidth / natural.w` under width,
`min(paneWidth / natural.w, paneHeight / natural.h)` under page, and `fitScale`
under manual. Everything downstream — the CSS box, `--total-scale-factor`, the
backing store, the reader's anchor, the budget — follows from the viewport it
produces. **The pane is passed in rather than read there**, so one pass sizes
every page against one measurement of the box.

**The pass stores what it derived, unrounded, in `fitted`.** Recovering it from
the rounded CSS box as `logical.w / natural.w` is a different number and never
equals a fit-page derivation, so every rest would re-lay the document out for
nothing.

**The control is a `<select id="fit">` in the header, and its option list is the
cap.** `Fit width`, `Fit page`, then `50% · 75% · 100% · 125% · 150% · 200% ·
300% · 400%`. Nothing above 400% is offered and no second clamp is written
anywhere: there is no pinch and no wheel, so the control is the whole surface,
and a scale stated twice would be a scale stated once wrongly. **400% is the
last step that fits the budget**: a pinned page costs `7.645·s²` MiB of backing
store at `devicePixelRatio` 2 whatever the pane measures, so above about 410% —
`√(128 / 7.645) = 4.09` — one page alone exceeds 128 MiB, where holding fewer
has stopped being a lever because one is the floor. While a reader holds a zoom
the retained set may reach two pages and 245 MiB, roughly twice the budget:
accepted, because the ladder below measured 4.85 GiB drawn and answering, so
this spends headroom known to exist. The backing store is not lowered to buy it
back — that spends the sharpness the renderer exists for, exactly where a reader
zoomed in to look.

**A fit change is a rest and never a gesture.** Nothing about it is continuous,
so it takes the width rest's path unmodified — set the fit, read the anchor if
none is held, then layout, `applyAnchor`, `refill` — and inherits that path's
deferral behind a running render, its cosmetic catch, and its budget pass. The
anchor is read *before* the layout, which is the whole of what carries the
reader: taken after it, a reader at page 36 fraction 0.499 switching from
fit-width to 200% lands at page 16 fraction 0.684.

**`paneWidth` is `clientWidth` of `#pages`, the scroll container, and not of the
`#preview` column around it.** Six A4 pages make it scroll, and on a machine set
to show classic scrollbars its content box is some 15 px narrower than the
column's — the difference between a page that fits and a page clipped at its
right edge. `#pages` carries `overflow-y: scroll` rather than `auto` so that
width cannot move under the fit: with `auto`, a document sitting just under the
scroll threshold gains a scrollbar the moment the canvases widen, the content
box narrows by the track, and the pages clip.

**The other axis is opened by the fit that can cross it and by no other**: a
`.wide` class puts `overflow-x: auto` on the box while the fit is a pinned scale,
and it is `hidden` otherwise. A zoom draws 2,381 px against a 520 px pane at
400%, where `hidden` would put everything past the right edge out of reach rather
than merely off screen; fit-to-width draws a page exactly `clientWidth` wide and
fit-page one at most that wide, so neither can overflow sideways and neither needs
the axis.

**Opening it unconditionally is a measured mistake, not a hypothetical one**: 21
`ResizeObserver loop completed with undelivered notifications` in one gate run
against none before, every one of them in a drag being scrolled under. The loop
is this box's own — the observer's callback writes page widths, a drag moving
faster than the callback leaves them momentarily wider than the box, `auto`
answers with a horizontal track, and the ~15 px it takes out of `clientHeight` is
a fresh notification for the very observer that caused it. `hidden` absorbed that
transient for as long as no page could legitimately be wider. Under a pinned
scale the loop cannot form, because `fit()` declines to act there and the callback
writes no width at all.

Fit-page is also the fit that reads `clientHeight`, and it never has the track, so
the boundary it derives is never measured against a height a scrollbar has taken.
The horizontal offset across a re-render is whatever element reuse preserves; the
anchor stays *(page, vertical fraction)*.

**A page is separated from the next by 16 CSS pixels, and the container adds
nothing at its own sides.** The gap is the top margin on each page's
wrapper, plus `#pages::after` for the one below the last page — a pseudo-element
rather than a bottom margin, because whether a scroll container's last child's
margin reaches `scrollHeight` is engine-dependent and this ships on WKWebView,
and rather than a spacer element, because `layoutPages` addresses
`pages.children[n - 1]` and every geometry function, and the drainer with them,
assumes each child is a page carrying `.logical` and `.natural`.
**The declaration is `margin: 16px auto 0` and every part of it is a
decision.** `offsetTop` includes an element's top margin, so the reader's place
moves with the gap; `#pages` is a block formatting context by virtue of
`overflow-y: scroll`, so the first page's margin does not collapse out of it;
and no *bottom* margin exists, so no pair of siblings collapses to less than the
constant. It is a length rather than a ratio, because a percentage resolves
against the containing block's *width* and would track the divider. **No padding
at the container's sides, and not because the width is wanted elsewhere**:
`clientWidth` *includes* padding, so side padding would leave `paneWidth`
unchanged, the canvases would be sized past the content box, and they would clip
silently. The gap is chrome and not content, so it is a constant that does not
scale with a gesture or a zoom — `fit()` and `unscale()` know nothing about it.

**The auto sides centre a page narrower than the pane**, which fit-page above
its boundary and a small pinned scale both draw and fit-to-width never could. An
auto margin resolves to zero when the page is at least as wide as the containing
block, so fit-to-width is untouched to the pixel and a zoomed page still
overflows to the right, where it is reachable.

**Each page wears a one-pixel ring on all four edges**, `box-shadow: 0 0 0 1px
var(--edge)`. A shadow rather than a border so that it costs no layout: under
`content-box` a border would make each canvas two pixels taller than the height
`size()` wrote, and `offsetHeight` — which the reader's place is expressed in —
would stop being the raster's own height. `inset` is not the escape either: an
inset shadow paints beneath a replaced element's content, and a canvas is opaque
white where `pdf.js` filled it. The colour is named because a colourless
`box-shadow` resolves to `currentColor`, which inherits `--ink`.

**At fit-width nothing of the ring is visible and nothing changed.** The side
pixels land at x ∈ [−1, 0) and [W, W+1), outside the scrollport, and that fit
keeps `overflow-x: hidden`, whose clip removes them. It would be safe without the
clip — a `box-shadow` is painted ink rather than scrollable overflow, so `auto`
grows no track for one — but the clip is what is actually there. Where `--ground`
shows beside a page, the ring is what draws its edge; that case exists only
because the fits made it.

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

So the two halves of a resize are answered differently, and **only the width
fit has a page that should track a moving pane** — `fit()` returns at once under
the other two, so under fit-page the page keeps its size through a drag and
under manual it keeps the size the reader pinned. Below fit-page's boundary that
fit *is* width-bound, so there a drag holds the page at its old size until the
rest snaps it: the accepted cost of one rule rather than three.

**While a width is changing the canvases are resized by CSS**, their backing stores untouched and
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
when **the scale the fit derives is the scale the layout holds**. **The timer waits for a render already running rather than cancelling
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

**What marks a start is `fitted` itself** — the scale the raster now in the
canvases was made for. A derived scale arriving while it differs opens a
gesture; one arriving while a gesture is open continues it. Nothing keys on
`pointerdown`, because a window drag-resize, a control taking width out of the
column, and the fit control itself have no pointer event to key on. A compile
landing mid-gesture keeps the reader's place and skips the caret jump, for that
compile only.

**One comparison answers four cases, and it was a width until the fits arrived.**
A width is the right thing to compare only while width is the one input. As a
scale: a window resized by its height alone still costs nothing under
fit-to-width; a height-only resize under fit-page *does* re-derive, which a width
comparison could never notice, so the page goes on fitting the pane; a width rest
under a pinned scale derives the same number and costs nothing; and a fit change
passes the rest's own guard on its own, because the derived scale moved while the
width did not. **A null derivation is not a scale that failed to move**: it means
the pane holds no page, and a layout that could not measure the pane leaves
`fitted` null too — skipping there would refuse the render meant to fix it, for
the life of the window.

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

**It is evaluated at the three passes that can move it — an open, a width rest
and a fit change — and never once per document.** A pass after a scroll
re-evaluates nothing, correctly: scrolling moves neither width nor fit. Five
shipped causes move the *width* term: the divider, a window resize, the panel
folding, the gutter, and an open itself, the panel being shown for any document
naming sections.

**The third pass costs nothing to add and is not optional.** It is free because
the fit change rides the width rest's path, which already reaches the decision;
it is required because a scale multiplies a page's cost by its square — 30.6 MiB
at 200% where fit-to-width costs 7.6 — so an implementation that re-derived the
scale without re-deciding what is held would draw a 20-page document whole at
612 MiB on a budget of 128.

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

**The pane publishes `window.__pane`** — its mode, the fit in force and the
scale the layout derived from it, the renders started, what the sizing pass and
the last raster cost, when the observer first delivered, and the canvases made
and released. It ships because the pane's gate is read by a person at a console:
the type check `rules/desktop-panes.md` describes reaches this file's
declarations and reaches none of its geometry.
