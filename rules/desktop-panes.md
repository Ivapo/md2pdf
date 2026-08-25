---
title: desktop-panes
sources:
  - app/dist/index.html
  - app/src/document.rs
  - app/src/preview.rs
covers: >
  the desktop app's two panes: the one file the front end is, the text pane and
  the blob frame that draws the artifact, the redraw that moves the reader and
  the page it chooses, the anchor path that opens the frame on the author's own
  page and the one file it is filtered to, the status the page places and never
  composes, the panel that names the document's parts and the two states it
  keeps apart, the width the frame is held at through a change and redrawn at
  after one, the geometry the page observes rather than infers, and the gutter
  whose rows are as tall as their lines render
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
the frame. **The text pane is plain** — no highlighting, no autocomplete, no
formatting commands — and every change goes straight to Rust, which holds the
buffer. The frame stops taking pointer events for the length of a drag, because
WebKit's PDF view swallows them otherwise and the divider would stick the moment
the pointer crossed the page.

It draws the artifact, not a picture of it: the bytes go into a `Blob` of type
`application/pdf`, and an iframe's `src` is the object URL, so WebKit builds its
own PDF document view inside the frame. No JavaScript PDF viewer is bundled and
none is wanted. The route is same-origin — a blob URL inherits the page's origin,
where a custom URI scheme does not — and the bytes never touch the disk. The
previous object URL is released once the frame has left it. **`draw` takes a page
as well as the bytes** and puts a `#page=N` fragment on the object URL it just
minted; the fragment goes on the frame's `src` and not on the URL it keeps, so
what it revokes next time is the object URL itself.

**A re-render moves the reader, and the app chooses where to.** WebKit's PDF view
leaks nothing about where the reader was, so a position can never be *learned* —
but `#page=N` on a **fresh** blob URL is honoured at load, so one can be set, and
a redraw following the author's own edit is opened on the heading above their
caret. A redraw that took a text from disk opens on page 1, as every redraw did
before Phase 6.

`refresh` asks for the status first, and for the bytes only when the state is
`current` **and** the status's `revision` is one it has not drawn — so the page
never draws a frame it has been told is out of date, and never redraws one the
reader has scrolled for a signal that compiled nothing, which the app's own save
now is. It re-reads the document's text on the `reloaded` count and on nothing
else, so a fetch cannot race a keystroke still in flight.

**It captures whether it took a reload *before* it advances `takenReload`**, and
hands `draw` no page on that pass. This is load-bearing rather than tidy:
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
the same branch and ask for no page, which is where an unfragmented frame opens
anyway. The precision is the document's heading density, and it follows the
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

**A document that names sections says so in a panel**, listed above the text
pane: the master first, then the sections in the order the master reads them,
with the one the pane is holding marked. A document that names none draws no
panel at all, so a single-file window is what it was.

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
the panel, and the toggle stays so they can get it back.

## The frame's width

**The page opens fitted**: `view=FitH` rides the fragment beside `page=N`, both
read at load. **WebKit reads it once** — the view keeps the scale it computed
and does not reflow when the frame resizes, and no script reaches inside a
native PDF view to ask it to.

So the two halves of a resize are answered differently. **While a width is
changing the frame is scaled**: it keeps the pixel width its page was fitted to
and a transform maps it onto the width the pane now has, which is fit-to-width
by construction — scale by `s` and the page inside is `s` times as wide. It
composites, so it holds every frame of a drag, and **the reader keeps their
place because nothing reloaded**. The frame is laid out `h / s` tall so it is
exactly `h` after scaling, and `flex` goes to `none` with it, since in a column
`flex` sizes the height being set.

**When the width settles the frame is drawn again**, because a scaled page is a
page rendered for a different width and softens far enough from `1`. A fresh
object URL at the settled width is fitted by the loader and rendered sharp. It
fires on `pointerup` and, through one 200 ms timer, on a window resize — a drag
ends on an event and a window resize does not — and is skipped when the width
did not move, so a window resized by its height alone re-fits without redrawing.
**A redraw lands on the caret's page**, by the rule above: a fresh blob can set
a place and never restore one.

**The pane's geometry is observed, not inferred from the events believed to
change it.** A `ResizeObserver` on the preview column, not a list of listeners.
Two bugs came from the other approach and both had one shape — a control added
to the window changed a geometry something else had already measured. The
divider read `clientX`, measured from the window, which matched the pane's own
width until the panel sat to the left of it; and the scaled frame held its
explicit width while the panel and the gutter each took width out of the column
beside it. The divider now measures from the text pane's own left edge, taken
once at the grab.

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
