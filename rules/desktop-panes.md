---
title: desktop-panes
sources:
  - app/dist/index.html
  - app/src/document.rs
  - app/src/preview.rs
covers: >
  the desktop app's two panes: the one file the front end is, the text pane and
  the canvases the app rasterises the artifact onto, the vendored renderer and
  its worker, the retained document a geometry-only redraw draws from, the
  redraw that moves the reader and the three causes that decide where to, the
  anchor path that opens on the author's own page and the one file it is
  filtered to, the status the page places and never composes, the panel that
  names the document's parts and the two states it keeps apart, the rows that do
  not load and the invariant they keep, the fold the page holds and the list that
  retains nothing, the width a gesture carries by CSS size and a rest answers by
  a render, the reader's
  place held across both, the geometry the page observes rather than infers,
  the gap that tells one page from the next and the hairline that draws its
  edges, and the gutter whose rows are as tall as their lines render
max_lines: 280
generated: 2026-08-25
---

# Desktop panes

What the window puts on screen, and the geometry it keeps while it does.
`rules/desktop.md` has the crate, the commands, the file I/O, the watch and the
bundle; this file has the two panes those things feed.

## The page

`app/dist/index.html` is the whole front end — one file, inline CSS and JS.

Two panes: a `<textarea>` at 40% of the width, a divider the reader drags, and
`#pages`, a scroll container holding one `<canvas>` per page. **The text pane is
plain** — no highlighting, no autocomplete, no formatting commands — and every
change goes straight to Rust, which holds the buffer.

**The app rasterises the artifact itself.** `pdf.js` is vendored as two static
ES modules under `app/dist/pdfjs/` — `pdf.min.mjs` and `pdf.worker.min.mjs`,
1,717,067 bytes together, with the Apache-2.0 `LICENSE` beside them — loaded
from a `<script type="module">`. `pdfjs-dist` ships browser-ready modules, so
there is still no bundler, no npm at build time and no node. The worker is not
optional: parsing and rasterisation go off the main thread, because the pane
re-renders every time the typing stops and rasterising on the main thread would
freeze the text the author is typing into.

`openPdf` takes the bytes and a page. **`getDocument` transfers the buffer to
the worker**, so the bytes cannot be read twice. **The loading task is retained
between renders** and a re-render caused by geometry alone draws from it and
invokes nothing — which is not an optimisation, since `current_pdf` refuses
while the preview is stale and a pane that re-fetched on a resize could not
re-fit a stale page. What is destroyed on a re-open is the *task*, not the
document proxy, which has no `destroy` of its own; the task owns the worker, and
dropping one undestroyed leaks a thread per compile.

**A canvas is swapped in only once it holds pixels.** Setting `canvas.width`
clears it, so re-rendering in place would flash empty pages at every rest.

**Only a render that drew advances `drawnRevision`.** `openPdf` answers
whether it did, and returns without drawing when a newer render supersedes it or
the pane measures no width. Recording the revision regardless would tell the next
signal that a page nobody drew is already on screen, after which `refresh`
returns at its own guard forever — a blank pane with no error and no way back.

**A re-render moves the reader only where it should, and the cause decides
where.** A compile that took a text from disk opens at page 1; a compile after
the author's own edit opens at the caret's page; a re-render caused by geometry
alone restores where the reader already was, which the blob frame could never do
because WebKit leaked no position to restore.

`refresh` asks for the status first, and for the bytes only when the state is
`current` **and** the status's `revision` is one it has not drawn — so the page
never draws a page it has been told is out of date, and never redraws one the
reader has scrolled for a signal that compiled nothing, which the app's own save
now is. It re-reads the document's text on the `reloaded` count and on nothing
else, so a fetch cannot race a keystroke still in flight.

**It captures whether it took a reload *before* it advances `takenReload`**, and
hands `openPdf` no page on that pass. This is load-bearing rather than tidy:
assigning a textarea's `value` moves its caret to the end of the control, so a
pass that replaced the text would read the document's last line and open it on
its last page. An open, an external reload over a clean buffer and a Finder
launch therefore all still draw page 1. The rule is "took no reload", not "the
author typed" — an external *figure* change carries a fragment though no
keystroke caused it, which is left as it falls, since the caret is still where
the author put it.

`caretPage` is the lookup: the caret's line is the newlines before
`text.selectionStart`, and the target is the last anchor at or below it, or
nothing when there is none. **Counting newlines is not parsing markdown** — the
page owns no knowledge of the dialect — and it is the one quantity both sides
agree on, `selectionStart` being a UTF-16 offset where a line in Rust is counted
from bytes. A caret above the first heading and a document with no headings take
the same branch and ask for no page, which is the page-1 case above. The precision is the document's heading density, and it follows the
*author* rather than the reader.

**The list it walks holds only the headings written in the file the pane holds**,
and `app/src/document.rs:render_with` drops the rest. A line means something in
exactly one buffer, so an anchor from a section is not a worse match but a number
about a document the pane is not showing — and this lookup breaks at the first
anchor past the caret rather than searching for a best one. A pure manifest
therefore yields none and opens at page 1, which is already the no-heading case
above; a master carrying a preface syncs on its own headings, correctly.

`report` places the status: the line in the header, the message in a bar above
the pane, the divergence in a bar of its own, and the dimming a stale page wears
— with no page under it the message takes the whole pane instead. **Every word it
places was chosen in Rust**, so the page composes none of it and the four states
are checked by tests rather than by eye. It survives a failure, because an author
mid-edit passes
through broken states constantly and blanking the pane would lose their place.
`fail` is for the refusals that are not a compile status — an open that will not
read, an export the pane cannot serve — and the next status replaces what it
wrote.

## The panel

**A document that names sections says so in a panel**, a left column beside the
text pane at `max-width: 40%`: the master first, then the sections in the order
the master reads them, with the one the pane is holding marked. A section is
named as the master writes it — `sections/method.md`, not `method.md` — because
two of that name in different folders must not read alike. A document that names
none draws no panel at all, so a single-file window is what it was.

The list is `Render::sections`, which is the list `render_with` already computed
for the watch — `named`, taken before either shopping list can be asked
anything. **It is a plain `Vec` where `Render::assets` is an `Option`**, because
`assets` is `None` exactly when the caller must keep the list it has, which is a
sentence about a watch filter and not a thing a panel can draw. It crosses in
`Status`, beside the anchors and for their reason: the status is already fetched
on the path that draws, so it costs no command. `Status::master` rides with it,
carrying the file *name* of the document the pane holds — the one row the page
could not otherwise name, and a name rather than a path because where the
document sits is the title's business.

**Absent and folded are two states.** `hidden` is a document that names no
section, and it takes the toggle with it; `.collapsed` is a reader who folded
the panel, and the toggle stays so they can get it back. **The fold is the
page's own**, a variable reapplied on every status rather than a field in
`Preview`: §2's rule is about state that decides behaviour, and a fold decides
nothing but its own drawing.

**The rows do not load, and four things turn on it.** Loading one would break
*the pane holds exactly one file*: `render_with` keeps only the anchors whose
location names no file, `Session::on_change` runs the external-change rule on
the buffer the pane holds, `save` writes to the document's own path, and the
join reads every section off the disk. So the rows are a list and are not
dressed as buttons, and the panel is rebuilt whole on every status — right only
while they hold no selection.

**The list tracks the text exactly and retains nothing.** `section_paths` cannot
fail, so an empty list is the answer rather than a failure to answer, and
`Preview::compile` assigns unconditionally where it keeps the asset list on
`None` — a master whose markers are deleted loses the rows on the next compile.
The cost is a flicker while a marker is half-typed; the remedy would be to damp
the redraw, never to retain a list the text has stopped naming.

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
sides by nothing.** The gap is `margin-top` on each canvas, plus `#pages::after`
for the one below the last page — a pseudo-element rather than a bottom margin,
because whether a scroll container's last child's margin reaches `scrollHeight`
is engine-dependent and this ships on WKWebView, and rather than a spacer
element, because `drawPages` indexes `pages.children[n - 1]` and every geometry
function assumes each child is a canvas carrying `.logical`. **`margin-top` is
the property and that is a decision**: `offsetTop` includes an element's top
margin, so the reader's place moves with the gap; `#pages` is a block formatting
context by virtue of `overflow-y: scroll`, so the first canvas's margin does not
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

## The gutter

**`Lines` is a view and changes not one byte of the buffer.** Off, the pane is
the plain textarea. On, it gains numbered rows and a mark on the caret's line.
The textarea stays the editor either way, so nothing that assumes one — the
buffer sent to `edit`, the reload that replaces `value`, `caretPage`, the
disabled state, the divider — is forked.

**A row is as tall as its line renders, not one line-height.** A textarea
soft-wraps and will not say where, so the heights are measured off a hidden
mirror: the same text, the same font, laid out at the same content width, one
element per logical line. What the browser did to the mirror is what it did to
the textarea. It is rebuilt when the text or the width changed and skipped when
neither did, and taken at a drag's end rather than per pointermove, because the
measurement is a layout.

**The caret's line is counted the way `caretPage` counts it**, off
`selectionStart` — a second way of finding it would be a second thing that can
disagree with the page the frame opens on. The band behind that line is the
textarea's **own background**, one colour sized and placed from the measured
row, with `background-attachment: local`, which is what scrolls it with the text
rather than pinning it to the border box. Drawing it as an element would mean
wrapping the textarea and layering over it, and the divider's geometry reads the
textarea's own box. Only the two rows that change are touched per caret move.

## What nothing checks

**No test in this repository reaches this file.** `withGlobalTauri` bought one
Cargo build and no node toolchain, and the price is that everything above is
checked by eye. The narrowest edge is that nothing checks the page and `Status`
agree: `invoke` returns an untyped value, the page reads a dozen fields off it
by name, and a field renamed in Rust breaks the window silently at runtime.
`specs/desktop_app_spec.md` OQ-10 carries it.
