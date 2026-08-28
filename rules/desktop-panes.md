---
title: desktop-panes
sources:
  - app/dist/index.html
  - app/tsconfig.json
  - app/typecheck.mjs
  - app/src/document.rs
  - app/src/preview.rs
covers: >
  the desktop app's two panes: the one file the front end is, the text pane and
  the pages the app rasterises the artifact onto, the wrapper a page is and the
  canvas and two layers inside it, the vendored renderer and its worker, the retained document a geometry-only redraw draws from, the
  redraw that moves the reader and the three causes that decide where to, the
  anchor path that opens on the author's own page and the one file it is
  filtered to, the status the page places and never composes, the panel that
  lists the project's files and the two states it keeps apart, the flat entries
  it is drawn from and the folders derived rather than sent, the fold the page
  holds, the inert row and the one button on it, the disk half that is walked
  twice and the missing half that follows the text, the text the reader can
  select and the stream it is read off, the link filter and the destination a click resolves, the scaffolding the
  bundle does not carry and the app supplies, the gutter whose rows are as tall
  as their lines render, the follow
  that keeps the numbers against their lines, the caret's two marks and the
  pane that loses both when it empties, the check that reads this file and the
  two declarations it holds to each other, and the seven defects that check
  does not reach
max_lines: 300
generated: 2026-08-28
---

# Desktop panes

What the window puts on screen, and the geometry it keeps while it does.
`rules/desktop.md` has the crate, the commands, the file I/O, the watch and the
bundle; this file has the two panes those things feed.

## The page

`app/dist/index.html` is the whole front end — one file, inline CSS and JS.

Two panes: a `<textarea>` at 40% of the width, a divider the reader drags, and
`#pages`, a scroll container holding one `div.page` per page of the document.
**The text pane is plain** — no highlighting, no autocomplete, no formatting commands — and every
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

**A page is a wrapper and not a canvas.** `#pages`'s children are `div.page`,
each holding the canvas plus a text layer and an annotation layer. Three boxes
have to share one origin and one size, and only an element holding all three can
be the child `drawPages` indexes and swaps, the prune removes, and the reader's
place is measured against — so `.logical` sits on the wrapper, `size()` writes
that box, and the canvas inside takes `width: 100%; height: 100%`. Layers as
siblings of the canvas *in* `#pages` would break the indexing; layers under a
wrapper that is not itself the child would be dropped by `replaceChild`. A page
removed takes its layers with it, which is the second reason the wrapper is the
child and is why `clear`'s `replaceChildren` disposes them too.

**A page is swapped in only once it holds pixels.** Setting `canvas.width`
clears it, so re-rendering in place would flash empty pages at every rest — and
since the whole page is swapped, its layers ride that rule too and are built
while it is still detached, which nothing in their path needs layout for.

**The text comes off `streamTextContent` and never off `getTextContent`.** That
call is a `for await (… of readableStream)` over its own stream, and
`ReadableStream.prototype[Symbol.asyncIterator]` is `undefined` in WKWebView —
Chromium and Gecko implement async iteration of a readable stream and WebKit does
not, so it rejects with *"undefined is not a function"* from inside the minified
bundle, which names nothing useful. `TextLayer`'s constructor takes the stream
itself and drives it with `getReader()` and `read()`, so handing it
`page.streamTextContent()` obeys the rule by construction rather than by
remembering it. The showcase carries 968 text items across its six pages.

**Only a link carrying an internal destination is rendered**, filtered before the
layer is built. An external one has no element, no `href` and nothing to
activate, so *refused* is something a second person can check rather than a
behaviour observed not to happen — `mpdf-003` §1.1, no servers and no network,
ever. The same filter needs no second rule for a markdown link *with text*
pointing at a figure: it reaches the PDF as a `/URI`, `pdf.js` refuses it on its
protocol, and it arrives with `url` and `dest` both null. The showcase's twenty
internal links are seven cross-references *plus* the footnote marks, their return
arrows and the citation marks, so the filter delivers a document navigable three
ways rather than one.

**The link service is three methods written here, not a port.**
`SimpleLinkService` is not in the bundle, and its route sets a real `href`, which
in this window would navigate the webview off `tauri://localhost` with nothing to
come back with — so the element gets a fragment and the click is answered by
`goToDestination`, which resolves the destination, takes its page index, turns its
own coordinate into a CSS offset through that page's render viewport, and scrolls
the container. **The destination's own coordinate, not the top of its page**:
`[](#fig:halves)` lands 458 pt down page 4, measured, and the showcase's one
cross-page reference lands 570 pt down page 3.

**The bundle carries the two classes and none of their scaffolding**, which lives
in `pdf_viewer.css` and is not vendored. So the app defines `--scale-round-x` and
`--scale-round-y` as `1px` on the page, writes `--total-scale-factor` from
`size()`, and brings rules for what `pdf.js` styles nowhere. A text span is given
percentage `left` and `top`, `--font-height` in *unscaled* px, a `font-family`
and — where they apply — `--scale-x` and `--rotate`; it is given **no `position`
and no `font-size`**. An annotation is a `<section>` with a percentage box, a
`z-index` and no `position`, and the `<a>` inside it gets no box at all. Left
undefined the layers take no size and the text paints raw over the raster; left
unstyled a link has nothing to click. The glyphs a reader sees are the raster's,
so the layer's own text is transparent, and a span's rotation is applied before
its `--scale-x`, that being a correction along the text's own direction.

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

**Every open document draws a panel**, a left column beside the text pane at
`max-width: 40%` listing the project: every file under the root that this
dialect can read, with the one that compiles marked. A lone `.md` naming nothing
draws one too, which is what lets an author build a first section without
leaving the window, and is the visible reversal of the panel `mpdf-008` Phase 4
shipped.

`Status::entries` is a flat `Vec` of `document::Entry` — `{ path, kind,
missing }`, `path` root-relative with `/` separators, `kind` one of `markdown`,
`bibliography` or `image`. **A directory is never an entry**: `parts` derives the
headings and the indent from the path's own segments, a thing a page can do and a
thing a nested node type would make `Status` carry twice. `Status::main` rides
with it, spelled the same way — root-relative and not the bare file *name* it
carried while the panel listed one document's parts, or the page could not match
it to a row. Both cross for the anchors' reason: the status is already fetched on
the path that draws, so the panel costs no command.

**Absent and folded are two states.** `hidden` is exactly *no document is open*,
and it takes the toggle with it; `.collapsed` is a reader who folded the panel,
and the toggle stays so they can get it back. `clear()` sets the first, because
a panel drawn for every document would otherwise hold the previous project's
files across the open that replaces them. **The fold is the page's own**, a
variable reapplied on every status rather than a field in `Preview`: §2's rule is
about state that decides behaviour, and a fold decides nothing but its own
drawing. The store this app now keeps is not a precedent for it — a main is a
decision about the document, a fold is where a scrollbar was.

**A row's body is inert and one button on it is not.** The `main` button appears
on a markdown row on hover and on focus and sets which file compiles; the row
itself does nothing, because the next phase gives the body a meaning — clicking
it puts that file in the pane. So the panel is still rebuilt whole on every
status, and that is still right: **the rows hold no selection.** Which file
compiles lives in Rust and arrives in the status, and the button reads its path
off the DOM at the moment it is clicked. `specs/file_panel_spec.md` OQ-5 carried
only the gesture; the four cases it settles are pinned in `document.rs`'s tests.

**The disk half is stable and only the marked-missing half moves.**
`document::files_under` walks the tree at an open and at a `Change::Tree` event,
and `Preview` holds the answer; `document::merge` adds the sections the master
names that the walk did not find, on every status, off `Preview::sections` and
off no directory. So a half-typed marker moves one row where the shipped section
panel lost all of them — strictly less motion than `mpdf-008` §2 accepted — and
`status()` still reads nothing from the disk.

## The gutter

**`Lines` is a view and changes not one byte of the buffer.** Off, the pane is
the plain textarea. On, it gains numbered rows and marks the caret's line
**twice** — the row's number in the gutter, and a band behind the line itself.
The textarea stays the editor either way, so nothing that assumes one — the
buffer sent to `edit`, the reload that replaces `value`, `caretPage`, the
disabled state, the divider — is forked.

**A row is as tall as its line renders, not one line-height.** A textarea
soft-wraps and will not say where, so the heights are measured off a hidden
mirror: the same text, the same font, laid out at the same content width, one
element per logical line. What the browser did to the mirror is what it did to
the textarea. **An empty logical line is mirrored as a zero-width space** — an
empty line still occupies one and an empty element does not, and markdown is
mostly blank lines. The failure without it **accumulates** rather than costing
one row its height: the offsets are a running sum over the measured heights, so
every blank line above a row shifts it by one line more. The rebuild happens
when the text or the width changed and is skipped when neither did, and the
width-change half rides `settle`'s 200 ms timer rather than a `pointermove` or
the `pointerup`, because the measurement is a layout.

**The gutter does not scroll; it follows.** Its box does not scroll itself and
its `scrollTop` is driven from the textarea's, on the textarea's own `scroll`
event and again whenever the rows are rebuilt. Without it the numbers part
company with their lines the moment a document is taller than the pane, which is
every document.

**The caret's line is counted the way `caretPage` counts it**, off
`selectionStart` — a second way of finding it would be a second thing that can
disagree with the page the pane opens on. The band behind that line is the
textarea's **own background**, one colour sized and placed from the measured
row, with `background-attachment: local`, which is what scrolls it with the text
rather than pinning it to the border box — the other half of the contrast above,
and why the two are built differently — and `background-repeat: no-repeat`,
without which the one-row gradient tiles down the whole pane. Drawing it as an
element would mean wrapping the textarea and layering over it, and the divider's
geometry reads the textarea's own box. Only the two rows that change are touched
per caret move.

**An emptied pane loses its band with its rows.** `clear` empties the buffer and
calls `relines`, which returns early there — and the two other things that move
the mark both need a focused, enabled textarea, which a cleared pane is not. So
that early return clears the mark on its way out; without it the previous
document's band stayed painted across an empty, disabled pane until something
opened.

## What checks it, and what that does not reach

**`app/typecheck.mjs` type-checks this file.** It mirrors the page — every line
outside `<script type="module">` blanked, and the one relative `pdfjs` specifier
rewritten to the bare name, which `app/tsconfig.json`'s `paths` answers with the
vendored declarations and a relative specifier cannot be given types for — and
runs TypeScript **5.9.3, pinned**, over the result. Both rules are
line-preserving and the script asserts it, so every diagnostic cites a real
`app/dist/index.html` line. It runs in CI on a push touching the page, the
settings, the declarations or the script, and **never in `cargo test`**: bun is
not a prerequisite of this workspace, and the Rust suite does not acquire one to
check a file it otherwise never opens.

**`strictNullChecks` and `noImplicitAny` are off, and that was measured rather
than assumed.** With them on the shipped page reports **243** errors and not one
is a defect: 112 nullable `getElementById` results, 78 implicit `any`, and the
rest the page's own expando properties. With them off it reported **41**, and
every one of those is an annotation the file now carries — the typedef block at
the top of its script, the DOM subtypes `getElementById` cannot know, `doc` and
`loading`, and the two `AnnotationLayer` call sites whose declarations require
fields `pdf.js` tolerates the absence of at runtime.

**The page and `Status` are held to each other from both sides.** The typedef
block declares `Status`'s ten fields and `Anchor`'s two; the type check binds
the page's reads to those, and `app/src/preview.rs`'s
`the_page_typedefs_name_exactly_the_fields_status_serializes` compares the same
`@property` lists against a serialized `Status` carrying one `Anchor` — which it
must carry, since an empty list puts no `Anchor` in the JSON at all. **Two
declarations compared with each other**, rather than usage compared against a
declaration. A field renamed on either side alone fails one of the two.

**What none of it reaches is most of what has gone wrong in this file.** Of the
eight defects it has produced, a type check catches **one** — a `destroy` that
`PDFDocumentProxy` does not have, swallowed by optional chaining, and caught in
review before it shipped. The seven it misses are `ResizeObserver` feedback
through a box the callback resized, an `IntersectionObserver`'s delivery racing
an animation frame, a forced pass dropped rather than re-armed, a stale
`deliveryMs`, a settle timer cancelling the render that was about to set the
fit, a counter advanced for a render that never drew, and an early return
leaving a caret band on an empty pane. Every one is behaviour, and no type
system sees any of them: `tests/gates/mpdf-009-phase5.js`, pasted into a
console, is still the only thing that does. `specs/desktop_app_spec.md` OQ-10
carries what is left.
