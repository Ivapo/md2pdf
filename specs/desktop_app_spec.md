---
id: mpdf-003
title: desktop-app
note: >
  A macOS desktop app that shows the PDF while you write: a Tauri window wraps
  the same core crate, watches the document and its images, and re-renders.
status: accepted
last_updated: 2026-08-29

phases:
  - name: "Phase 1 — the window, and one compile on screen"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 2 — the watch loop"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 3 — export, and the state the loop needs to show"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 4 — the text pane"
    reviewed: 2026-08-10
    shipped: 2026-08-10
    cut: null
    by: null
  - name: "Phase 5 — an app you can install"
    reviewed: 2026-08-11
    shipped: 2026-08-11
    cut: null
    by: null
  - name: "Phase 6 — the page the author is on"
    reviewed: 2026-08-15
    shipped: 2026-08-15
    cut: null
    by: null
  - name: "Phase 7 — the page fits the pane it is given"
    reviewed: null
    shipped: null
    cut: 2026-08-25
    by: mpdf-009
  - name: "Phase 8 — the text pane shows its lines"
    reviewed: 2026-08-25
    shipped: 2026-08-25
    cut: null
    by: null
  - name: "Phase 9 — what checks the front end"
    reviewed: 2026-08-27
    shipped: 2026-08-27
    cut: null
    by: null
  - name: "Phase 10 — the editor is named Letur"
    reviewed: 2026-08-29
    shipped: 2026-08-29
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-002]
reference: >
  Typst's own web app is the inspiration for the two-pane shape and for
  re-rendering as you type. Its collaboration, its package registry and its
  server are all out of scope permanently: this app runs on the local machine
  and fetches nothing, per mpdf-001 §2.
---

# desktop app

## 1. Goal

Show the PDF while you write it. **The observable is unchanged from `mpdf-001`
— the typeset PDF that Typst compiles from the user's markdown — but it stops
being a file you open and becomes a page you watch.** The consumer is the same
author, who today runs a command, switches to a viewer, reads a page, switches
back, and repeats that loop on every edit.

What Phase 1 alone produces:

```console
$ cargo tauri dev            # a window opens
```

Open `samples/article.md` from the window's Open dialog, and its page is drawn:

```
┌────────────────────────────────────────┐
│ samples/article.md                     │
├────────────────────────────────────────┤
│          A Sample Article              │
│               Iva Po                   │
│                                        │
│  Introduction                          │
│  This file exists so you can see…      │
└────────────────────────────────────────┘
```

Phase 2 makes that page redraw on every save, which is what turns it from a
viewer into a loop. Phase 3 adds the status the loop needs to show — stale,
failed, and how long the compile took — and an export. Phase 4 adds a text pane
on the left, and the app becomes the editor too; it comes last of the four that
change what the app does, because it is the only one the project can do
without. Phase 5 then packages what those four produce.

The engine does not change. `md2pdf-core` already takes a markdown string and
returns PDF bytes, and `mpdf-001` §1 recorded the claim this spec is the test
of: "A Tauri desktop app later becomes a second thin wrapper around the same
core crate. It is not a rewrite."

### 1.1 Non-goals

- **No servers, no network. Ever.** `mpdf-001` §2's decision is inherited
  whole. The app fetches nothing at run time, and any asset it needs is
  bundled at build time.
- **No change to the dialect.** The app converts exactly what `mpdf-001` and
  `mpdf-002` accept, and refuses exactly what they refuse, with the same
  message. A construct the CLI names is a construct this app names.
- **macOS only.** Windows and Linux are future work. Nothing in `core` is
  platform-specific, so a later port is a packaging job rather than a rewrite,
  and saying that here is what keeps this spec from designing for three
  platforms it cannot test.

  > **CORRECTED 2026-08-25, by Phase 7.** The bullet above is kept as it was
  > written, and one clause of it is no longer true: **a port is not a packaging
  > job.** The claim was made about `core`, and about `core` it still holds —
  > pure Rust, its fonts bundled, no platform call anywhere, and the CLI really
  > would travel by being compiled. What the claim did not account for is that
  > **this app's preview pane is not code this project wrote.** It is WebKit's
  > own PDF view, handed a blob and addressed through PDF open parameters, and
  > that view is a property of the webview rather than of the app. Windows runs
  > WebView2, whose PDF view is Chromium's: it exists, but the `view=FitH` and
  > `#page=N` behaviour Phase 6 and Phase 7 both build on is tuned to WebKit's
  > and would need re-establishing. Linux runs WebKitGTK, which ships **no
  > built-in PDF viewer at all** — a frame fed a PDF blob renders nothing — so
  > the pane does not merely behave differently there, it is absent. Both claims
  > are stated from documentation rather than from a build on either platform,
  > and the port is where they get tested.
  >
  > **What the bullet was really about survives**: this spec should not design
  > for platforms it cannot test, and it has not. Read the clause as *`core`
  > forces no rewrite*, which is true, and not as *the app is a recompile away*,
  > which it is not. **The pane is the port**, and OQ-9 carries it.
- **Not the browser build.** `mpdf-001` §1.1 parks a `wasm32` build, and it
  stays parked. It is a different front end with a different file story, so it
  is a later spec rather than a phase here.
- Out of scope, parked: multi-file projects and manifests, a document library
  or recent-files list, printing, PDF export options, theming, collaboration,
  and anything agentic.

  > **CORRECTED 2026-08-28, by `mpdf-010` Phase 1.** The bullet above is kept as
  > it was written, and its first clause is no longer true: **multi-file
  > projects are in scope and shipped.** The app opens a project rather than a
  > file — the folder the opened document belongs to, found by one level of
  > climb — lists what is under it, and lets the author say which file in it
  > compiles.
  >
  > **Manifests are still parked, and that is the half worth keeping.** Nothing
  > is written into the author's own folder. The one fact the app remembers per
  > project lives in its own Application Support directory, keyed by the root's
  > canonical path, and `mpdf-010` §2 records why a dotfile beside the document
  > was refused: it is this clause arriving under another name, and it would put
  > a file into a directory the author may have under version control. **The
  > document library and the recent-files list are still parked too** — the app
  > opens one project at a time, through the dialog or a Finder association,
  > exactly as it opened one document. Printing, export options, theming,
  > collaboration and anything agentic are untouched.

## 2. Design

A third crate joins the workspace beside `core` and `cli`:

- **`app`** — a Tauri binary. It owns file I/O, the window, and the watch loop.
- It calls `md2pdf-core` and nothing else of the project's own code.

`core` gains nothing and changes nothing. That is the falsifiable claim: if
building this app requires an edit to `core/src/lib.rs` beyond what a bug fix
would need, `mpdf-001` §1's "not a rewrite" was wrong, and the review round
should say so rather than let the edit through quietly. Round 1 confirmed the
claim holds for Phase 1: `md_to_pdf`, `image_paths`, `Asset`, `ImageRef` and
`Error` are already public at the crate root, and Phase 1 needs nothing else.

> **CORRECTED 2026-08-15, by Phase 6.** The paragraph above is kept as it was
> written and is no longer true as stated. `core` gained one function —
> `core/src/lib.rs:md_to_pdf_with_anchors`, plus the `Anchor` and `Rendered` it
> returns — and Phase 6's round let the crossing stand. **What the claim was
> really about survived it**: no existing signature changed, `md_to_pdf` became a
> wrapper over the new function rather than a second path, `cli/src` is
> untouched, and five phases had already answered the question the claim was
> asking, which is whether building a window forces a rewrite of a library
> written for a CLI. Read the sentence as *the app forces no rewrite of `core`*,
> which is what the five diffs before it checked and what Phase 6's own gate (5)
> still checks on the half it does not cross.

### Why a third crate, not a mode of the CLI (decision, recorded)

`cli` is a binary with a contract — arguments, exit codes, stderr — that
`cli/tests/cli_test.rs` pins. A window has none of those, and a GUI subcommand
would put a webview dependency into every CLI build. Two binaries over one
library is the shape `mpdf-001` §2 already drew, and this is the second one it
predicted.

### Why the app owns file I/O, exactly as the CLI does (decision, recorded)

`core` holds no OS access by design, so the app reads files itself. It mirrors
`cli/src/main.rs:read_assets`: call `md2pdf_core::image_paths` on the markdown,
resolve each path against the document's parent directory, read the bytes once
per path, and hand back `Asset` values that keep the path the markdown wrote.
Duplicating that logic in a second wrapper is deliberate and small — the
alternative is a shared helper crate for forty lines, and the two wrappers
report errors differently, which is most of what those forty lines do.

### Why the preview shows the PDF itself (decision, recorded)

The pane draws the artifact, not a picture of it. Typst's layout engine decides
the page breaks and produces page frames, and both `typst-pdf` and Typst's SVG
export consume those same frames — so pagination, glyph positions and line
breaking are identical whichever one draws the pane. What differs is everything
the PDF format carries besides the picture: a link is a live annotation rather
than dead ink, text can be selected and searched, and the accessibility tagging
that `mpdf-001` settled in an open question of its own, and put on
`table.header`, is present. The sharpest reason is
the last: `typst-pdf` is a separate crate from the layout engine, so a pane
drawn from frames would show that the *layout* is right, which is not the claim
this project makes. The observable is the PDF.

The constraint that rides with it: **no bundled JavaScript PDF viewer.** This
project has no dependency of that kind and does not gain one for a preview
pane. OQ-1's probe settled that it does not have to: WebKit draws the PDF
itself, and the mechanism is recorded below.

> **CORRECTED 2026-08-25, by `mpdf-009` Phase 1.** The constraint above is kept
> as it was written and no longer holds. `mpdf-009` bundles one — Mozilla's
> `pdf.js`, two vendored ES modules — and §1.1 of that spec lands on *this*
> sentence: bundling a viewer contradicts shipped work, which is why it is a
> new spec rather than a phase here. What the constraint was protecting is
> intact and was never the viewer as such: it was `withGlobalTauri`'s bargain,
> no bundler and no node toolchain, and `pdfjs-dist` ships browser-ready
> modules precisely so that bargain survives. What is genuinely given up is
> named in `mpdf-009` §2: a canvas plus a text layer does not expose a tagged
> PDF's structure to the accessibility tree, and that spec's OQ-3 carries it.

### Why the pane is a `blob:` URL in an iframe (decision, recorded)

The app hands the compiled bytes to the page, the page wraps them in a `Blob`
with type `application/pdf`, and an iframe's `src` is the object URL. WebKit
then instantiates its own PDF document view inside that frame. OQ-1's probe
confirmed each step on this machine, and two properties of this route decided
it over the alternatives.

The first is that it is same-origin. A blob URL inherits the page's origin —
the probe read `blob:tauri://localhost/…` — so the parent page can reach into
the frame, which a custom URI scheme does not allow: the probe found that same
frame served over a `pdf://` scheme loads but exposes a `null`
`contentDocument`, because the scheme is a separate origin. Whatever the later
phases can get from the frame at all is reachable only on the same-origin
route — and Phase 2's round then measured how much that is, which the section
below records: a page can be set, and nothing can be read. This sentence
originally named the scroll offset as the thing the route bought, which the
measurement proved wrong; the route still earns its place, because setting a
page needs it and because reading nothing is a fact the other route could not
even have established.

The second is that the bytes never touch the disk. A temporary file would put
the artifact somewhere the user did not ask for, and would need cleaning up on
a crash.

**The escape hatch, if WebKit ever stops doing this.** The probe was run on one
macOS version, and §1.1 keeps this spec to macOS, so the risk is bounded — but
it is not zero, and a later WebKit or a later platform could take the native
PDF view away. The fallback is then Typst's SVG export, one page per frame,
which is already reachable: `typst-svg` 0.15.1 sits in `Cargo.lock` today as a
transitive dependency of `typst`. Taking it would mean amending this decision
and saying so, because the pane would stop showing the artifact and start
showing a picture of it. It would not mean bundling a JavaScript viewer, which
stays refused either way.

> **CORRECTED 2026-08-25, by `mpdf-009` Phase 1.** The section above is kept as
> it was written and no longer describes the pane. There is no iframe and no
> `blob:` URL: `app/dist/index.html` hands the bytes to a vendored `pdf.js` and
> rasterises each page onto a canvas the pane owns. The last sentence is the
> one most exactly reversed — the escape hatch is taken, and it is the
> JavaScript viewer rather than the SVG export.
>
> **The measurement this section rests on still stands**, and it is why the
> route was right for as long as it was used: the frame *was* same-origin, the
> bytes *never* touched the disk, and WebKit *did* build its own PDF view. The
> reason it was left is not that any of that was wrong but that it was not
> ours: `view=FitH` is read once at load, a resize cannot be answered, and the
> view leaks no position, so a redraw could set the reader's place and never
> restore it. `mpdf-009` §1 argues it in full. The bytes still never touch the
> disk.

### What the pane cannot do: keep the reader's place (decision, recorded)

The frame is same-origin, and a later phase can still reach into it — but not
for this. Phase 2's review round probed the running Phase 1 app, and after a
reader scrolled several pages by hand the parent page saw `scrollTop` 4, which
is the four pixels of slack between the frame's own document and its viewport;
`scrollY` 4; the `<embed>`'s `scrollTop` 0; no enumerable properties on that
element; and no `hashchange`, so the view does not write its page into the
fragment either. **WebKit's PDF view leaks nothing about where the reader is.**

The write direction does work, and was confirmed on the operation Phase 2
performs rather than on a cheaper one: a `#page=N` fragment on a *fresh* blob
URL — new bytes, new object URL, as every recompile produces — is honoured at
load. So the app can put the pane on any page. It cannot learn which page to
ask for.

A re-render therefore returns the reader to the first page. That is a cost this
spec records rather than hides, and it is the price of §2's decision that the
pane draws the artifact: the alternative that would fix it is the SVG fallback,
which stops drawing the artifact. OQ-6 carries the question, and Phase 2 does
not answer it.

> **CORRECTED 2026-08-15, by Phase 6.** The paragraph above is kept as it was
> written and is no longer true of every re-render. The measurement it rests on
> still holds — the view leaks nothing, so the reader's place cannot be *learned*
> — but Phase 6 spends the write direction the paragraph above it already
> records: a redraw following the author's own edit carries `#page=N` for the
> heading above their caret. **A redraw that took a text from disk still returns
> them to the first page**, which is every open, every external reload over a
> clean buffer, and every Finder launch — so the sentence stays true of exactly
> the cases Phases 1, 2 and 5 gated on it. What changed is that the app now
> restores a position it still cannot observe, by following the author rather
> than the reader.

### Why the watch set is the document's own directory (decision, recorded)

A document that changes is not the only thing that should redraw the page. A
figure edited in another program should redraw it too, and `mpdf-002` already
supplies the list: `md2pdf_core::image_paths` returns every path the document
names, in reader order. Emission reads no image bytes, so that list is
available even when the document names a file that does not exist yet.

**The set is one directory rather than that list**, which Phase 2's review round
settled and this section records. Two facts decide it. A watcher cannot register
a path that does not exist — `notify` 8.2.0's macOS backend refuses one in its
own `append_path`, which returns `path_not_found` before it reaches FSEvents at
all — so the very case the list exists to serve, a figure named
minutes before it is drawn, is the case a file-valued set cannot hold. And
`core/src/emit.rs:check_image` refuses a URI scheme, a leading `/`, a `..`
segment and a backslash, so **every path a document can legally name resolves
under the document's own directory**. One recursive watch on that directory
therefore covers the document, every figure it names, every figure it will name,
and every directory not yet created — and it is computable from the document's
path alone, so a document the dialect refuses is still watched and can be fixed
into one that compiles.

The list keeps a second job, one layer in: **it is the filter, not the set.** An
event is relevant when its path is the document or one of the paths
`image_paths` returns, and that filter is a plain function over a path and a
list. Watching is what needs a directory; deciding what to redraw for is what
needs the list.

One fact about that filter costs a build, in the same idiom as the icon facts
OQ-2 recorded: **both sides of the comparison must be canonicalized.** `notify`
canonicalizes a path as it registers it, because FSEvents reports the resolved
path, so an event arrives naming `/private/var/folders/…` where the Open dialog
handed the app `/var/folders/…`. Comparing the two as they arrive matches
nothing, and the failure is silent in the worst way — the watcher runs, every
event is filtered out, and the page simply never redraws. On macOS `/tmp` and
`/var` are both symlinks into `/private`, so this is the default case and not
an exotic one.

The limit this decision accepts: a figure that is a symlink pointing out of the
directory is not watched. Its path is legal and resolves inside, and the bytes
it names live somewhere a recursive watch on the tree never sees. A watch
follows the tree, not the targets, and that is recorded here rather than fixed.

### Why a change is a whole re-compile (decision, recorded)

No incremental machinery, no caching, no partial re-layout. Measured on this
machine on 2026-08-10, twenty runs of the release binary each, medians:
`samples/press-release.md` 8.5 ms and `samples/article.md` 28.7 ms, both
including the process spawn that the app does not pay. A re-render per save is
affordable by two orders of magnitude, and an incremental path would be a large
amount of state to hold wrong. If a document is ever slow enough to need one,
that is a measurement and a later phase, not a guess made now.

The numbers are stated with their method because a first pass took one run each
and reported 42 ms for the article, which round 1 could not reproduce; a single
cold run is not a measurement.

### Why the exported file is byte-identical to the CLI's (decision, recorded)

The PDF is a pure function of the markdown text and the asset bytes, and both
wrappers compute the same assets, so the app's export and `md2pdf` on the same
document produce the same file. Phase 3's review round measured it rather than
argued it: `samples/article.md` compiled in five separate processes gave five
identical files, and each matched a `samples/article.pdf` a different build had
produced seven hours earlier; `samples/press-release.md` gave three identical
files across three processes, likewise matching an older build's; a document
naming a PNG gave three identical files, and **the same content under a
different file name gave the same bytes**, so the output does not depend on the
path. The round also read the two asset readers line for line —
`app/src/document.rs:read_assets_with` and `cli/src/main.rs:read_assets` — and
found the same `image_paths` call, the same dedup on the same key, the same
join and the same order.

The method is stated with the numbers for the reason `mpdf-001`'s round stated
it with the timings: a claim of this kind is worth exactly the runs behind it.

**What the claim rests on, so a later reader knows what could take it away:**
`core/src/lib.rs:md_to_pdf` calls `typst_pdf::pdf` with `PdfOptions::default()`,
and nothing in `core`, `cli` or `app` reads a clock or a path into the document.
A Typst release that put a timestamp or a build identifier in that default would
falsify this silently, which is why Phase 3's gate checks it rather than trusts
it.

**Where the check has to live is not where the claim reads.** `md2pdf` is a
binary of the `cli` package, and `CARGO_BIN_EXE_md2pdf` is set only for
integration tests of the package that defines it; `md2pdf-app` declares a
`[[bin]]` and no `[lib]`, so nothing in `app/src/` is importable either. No
single test can hold both ends. Phase 3's gate therefore splits the claim in
two and composes them, and says so.

### Why the last good page survives an error (decision, recorded)

An author mid-edit passes through broken states constantly — a half-typed
table, a fence not yet closed. Blanking the pane on each one loses their place
and makes the loop worse than the CLI it replaces. So a failed compile leaves
the last page drawn and shows the error above it, in the same words the CLI
prints: the construct and its line, or the frontmatter key and its line. The
pane also marks itself stale, because a page that silently belongs to older
text would be the flattening `mpdf-001` §2 refuses, one layer up.

### Why an external change waits for a clean buffer (decision, recorded)

OQ-5's three answers were refuse the reload, take the disk copy, or ask. **The
answer is refuse when the buffer holds unsaved edits, and take the disk copy
when it does not.** The condition is what makes this one rule rather than a
compromise between two.

**Refusing unconditionally would remove a behaviour this project has shipped.**
Phase 2's loop is "save the file in your editor and the page redraws", the
README documents it in those words, and Phase 2's own by-eye gate case read it
at the window. A rule that refused every external change would falsify all
three, for the commonest workflow the app has. §6.1 of the methodology is
explicit that contradicting shipped work is never what a phase does, and here a
design that keeps the behaviour costs one comparison.

Taking the disk copy unconditionally is the answer that loses. It destroys work
the app was asked to hold, which is the flattening `mpdf-001` §2 refuses, one
layer up and with the author's own words rather than a construct. Asking needs a
modal dialog — a UI class this app does not have, and a permission its
capability file does not carry — for a case the two-part rule already decides
without losing anything. That permission is `dialog:allow-message`, which is
where a two-choice prompt lives too: `tauri-plugin-dialog` 2.7.2 registers
exactly `open`, `save` and `message`, and its `ask` and `confirm` both route
through the last of those.

**The app does not merge**, and that is recorded rather than deferred. A
three-way merge is an editor project, and §1.1 keeps this spec from being one.
When the buffer is dirty and the disk has moved, the author resolves it by
saving, which overwrites the disk, or by reopening, which takes it. The app
names which choice each one is and makes neither for them.

**The rule is three strings and two comparisons, and it needs no dirty flag.**
The app holds the buffer and the text as it stood at the last open or save. On
an event naming the document, read the file:

- the file equals the buffer — the app's own save arriving back, or a change
  that changed nothing. Nothing happens;
- the file differs, and the buffer equals the last-saved text — the buffer is
  clean, so nothing can be lost. Take the disk copy and recompile. **This is
  Phase 2's loop, unchanged**;
- the file differs, and so does the buffer — the author has unsaved work. Keep
  it, and report the divergence.

Each of those is a plain function over strings, which is what keeps the rule out
of the window and inside the gate.

**Two limits it accepts, recorded rather than fixed.** An author who keeps
typing between a save and that save's event — the 12 ms delivery plus the
debounce — is in the third outcome by the time it arrives, so the app names a
divergence that was really its own write. It loses nothing, because that
outcome keeps the buffer, and the next save clears it. And an external writer
that happens to write exactly the author's unsaved text takes the first
outcome, which leaves the last-saved text unrefreshed and the buffer reading
dirty for longer than it is. Both err toward refusing, which is the direction
that keeps work.

The mechanism this needs is also what a save needs. **Once a document is open in
the pane, its own path stops triggering a recompile directly and starts
triggering the rule above.** The buffer is what compiles, so a document event no
longer means "the text changed, redraw"; it means "the disk moved, decide", and
a recompile is one of the three outcomes rather than the event's meaning. The
figures keep triggering a recompile directly, because nothing else supplies
them, and the directory watch itself is unchanged.

**So the loop gains a second action, not a narrower filter**, and the difference
matters: a document dropped from the filter would never reach the rule that
depends on it. `app/src/watch.rs:start` takes one filter and one callback today,
so the document's events and the figures' events reach the same code. Phase 4 is
where they stop doing so. Whether that is two callbacks or one callback handed
the path is an implementation choice; what this decision fixes is that a
document event runs the rule rather than a bare recompile.

That is why this phase suppresses no self-write. A suppression would have to win
a race the loop's own measurement says is not winnable — `app/src/watch.rs`
records that a save's first event reached the process 12 ms after the write, so
dropping and restarting the watch around a write leaves a window open.
**Routing the event into the rule removes the race instead of racing it**,
because the first of the three outcomes is exactly the self-write case, and it
is decided by comparing content rather than by winning on timing.

### Why the app's logic lives in plain functions (decision, recorded)

Everything that can be decided without a window — the watch set, the debounce,
the asset read, the error string, the stale flag — lives in ordinary Rust
functions with ordinary tests. Only the parts that genuinely need a window go
through Tauri's command layer. A GUI whose logic is reachable only by clicking
has no exit gate but a screenshot.

This rule is about where logic lives, and it does not claim that every gate in
this spec is a test. One thing genuinely cannot be one: whether the right
pixels reached the glass. That claim is read by a person, exactly as
`mpdf-001`'s Phases 7 and 9 read a PDF by eye. The rule's job is to keep that
list to one item.

## 3. Open questions

- **OQ-1** — ~~how does the pane draw a compiled PDF, and can it be done with no
  bundled viewer? On macOS the window is a WKWebView, which renders PDFs
  natively, but the mechanics are unverified: whether bytes can be handed over
  through a custom protocol, a `blob:` URL or a temporary file; whether an
  `<embed>` or `<iframe>` draws it or only a full-frame navigation does; and
  what the app can control about the viewer's own chrome, zoom and scroll
  position. If no route holds, the fallback is Typst's SVG export per page,
  and §2's decision is amended to record why the artifact could not be shown
  directly. Answerable from code and a probe during review. Blocks Phase 1's
  gate case (1), and OQ-3 depends on the answer.~~ **RESOLVED (2026-08-10), in
  review round 1: a `blob:` URL in an iframe, and no bundled viewer.** A
  throwaway Tauri 2.11.5 app was built and run on macOS 26.5.2, and it
  reported, from inside the webview: `navigator.pdfViewerEnabled` is `true`,
  `navigator.mimeTypes['application/pdf']` is present, and the frame fed a blob
  of the compiled bytes exposes a `contentDocument` whose `contentType` is
  `application/pdf`, whose location is `blob:tauri://localhost/…`, and which
  holds one `<embed>` — WebKit's own PDF document view, built without anything
  the app shipped. Custom-scheme serving also works — a `pdf://` scheme
  returned 200 with the right content type — but that frame's
  `contentDocument` is `null`, because the scheme is a separate origin, which
  is what ruled it out. Landed in §2 as its own decision.

  > **CORRECTED 2026-08-25, by `mpdf-009` Phase 1.** The resolution above is
  > kept as it was written; its second clause, **"and no bundled viewer"**, no
  > longer holds. The probe's findings are all still true of WebKit — this
  > corrects what was *decided*, not what was measured. See §2's own note.

  The residual, recorded rather than hidden: the probe proves WebKit
  *instantiated* its PDF view, not that the pixels are right. Nothing readable
  from JavaScript can prove that, and this machine denies the terminal
  Screen Recording permission, so no screenshot could be taken during the
  round. Phase 1's gate case (1) therefore has to be run where a human can see
  the window, and it says so.
- **OQ-2** — ~~which Tauri version, and what does the minimal app look like in
  code — the crate layout, the configuration file, the command boundary, and
  what the workspace must carry for `cargo tauri dev` to open a window?
  `mpdf-001`'s OQ-1 is the precedent: the same question about the Typst crates
  was answered by reading them during review. Answerable from code during
  review. Blocks Phase 1.~~ **RESOLVED (2026-08-10), in review round 1**, by
  building the throwaway app rather than by reading about it. Tauri 2 —
  `tauri` 2.11.5 and `tauri-build` 2.6.3 resolve today, and `tauri-cli` 2.10.1
  is what runs `cargo tauri dev`; the versions pin at implementation, as
  `mpdf-001` §2 pinned the Typst crates. The crate is seven files: `Cargo.toml`
  naming `tauri` and `tauri-plugin-dialog` with `tauri-build` under
  `[build-dependencies]`; `build.rs` calling `tauri_build::build()`;
  `tauri.conf.json` with `identifier`, `build.frontendDist` and an
  `app.windows` entry; `src/main.rs`; `dist/index.html`; `icons/icon.png`; and
  `capabilities/default.json`.

  Four facts cost a build each and are recorded so the next reader does not pay
  again. `icons/icon.png` is **required** — without it `generate_context!`
  panics at compile time with "failed to open icon", which is a confusing
  failure for a window that has no icon yet. `app.withGlobalTauri: true` puts
  the API on `window.__TAURI__`, so **the frontend needs no bundler and no npm
  toolchain at all** — `frontendDist` is a directory of static files, which
  keeps the whole app one Cargo build. The command boundary is
  `#[tauri::command]` plus `generate_handler!`, and a custom URI scheme is
  `register_uri_scheme_protocol`. And the Open dialog is
  `tauri-plugin-dialog`, which needs `dialog:allow-open` in
  `capabilities/default.json` beside `core:default`; the probe called it and
  the native dialog opened rather than rejecting, which is what confirms the
  permission entry is right.
- **OQ-3** — ~~what holds the scroll position across a re-render? A watch loop
  that returns the reader to page 1 on every save is unusable for a document
  longer than a page, and it is the difference between this app and a shell
  loop that reopens a viewer. OQ-1's resolution is what leaves the question
  answerable at all — the blob frame is same-origin, so the parent can read and
  write inside it, where the custom-scheme frame it ruled out could not be
  touched. What remains open is what WebKit's own PDF view exposes to be saved
  and restored, and the honest floor may be an offset rather than a semantic
  position. Design call, with the mechanism answerable from code. Blocks
  Phase 2's gate case (2).~~ **RESOLVED (2026-08-10), in Phase 2's review
  round 1: nothing
  holds it, and nothing can.** Both halves of the question's own guess were
  wrong. The floor is lower than an offset — the view exposes *nothing*
  readable, measured on the running Phase 1 app and recorded in §2. The ceiling
  is higher than an offset — `#page=N` on a fresh blob URL is honoured at load,
  so a position can be *set* at page granularity. Read is impossible and write
  works, which is the one combination the question did not consider, and it
  leaves the app able to restore a position it cannot observe.

  Phase 2 therefore ships without the property, and §2 records the cost. Gate
  case (2) of the drafted phase, which was keyed to this resolution, is gone
  rather than weakened: a gate keyed to an impossibility is not a gate. **OQ-6
  carries what is left**, because the choice it now poses — amend §2's pane
  decision, or accept the cost permanently — is a different question from the
  one this entry asked, and answering it inside a round that was unblocking
  Phase 2 would have made a recorded decision fall as a side effect.
- **OQ-4** — ~~can the watcher follow a path that does not exist yet? A document
  may name `figures/new.svg` minutes before that file is created, and the
  watch set includes it because `image_paths` reads the text rather than the
  disk. The usual answer is to watch the parent directory instead of the file,
  which changes what a change event means and how much the loop filters.
  Answerable from code (the file-watching crate the phase picks). Blocks
  Phase 2's scope.~~
  **RESOLVED (2026-08-10), in Phase 2's review round 1: it cannot, and the set
  is one directory.** `notify` 8.2.0 is the crate, and its macOS backend refuses
  a path that does not exist: its own `append_path` opens with
  `if !path.exists() { return Err(Error::path_not_found()…) }`, in the crate's
  `src/fsevent.rs` and so in no file of this repository. A file-valued set
  therefore cannot hold the case the question poses.

  The answer is smaller than "the parent directory of each file", and the round
  found why: `core/src/emit.rs:check_image` refuses a URI scheme, a leading
  `/`, a `..` segment and a backslash, so every legal image path already
  resolves under the document's own directory. **One recursive watch on that
  directory is the whole set**, it needs no recomputation when an edit adds or
  drops a figure, and it is computable from the document's path alone — so it
  also dissolves a question this entry did not ask, which is what to watch for
  a document the dialect refuses. Landed in §2 as its own decision. What the
  loop filters is `image_paths`' list, one layer in.
- **OQ-5** — ~~what does the editor phase do when the file changes on disk while
  the text pane holds unsaved edits? Every editor answers this and none of the
  answers is free: refuse the reload, take the disk copy, or ask. It does not
  block anything until Phase 4, and stating it here is what stops Phase 4 being
  designed as if the question were not there. Design call. Blocks Phase 4.~~
  **RESOLVED (2026-08-10), across Phase 4's review rounds 1 and 2: refuse the
  reload when the buffer holds unsaved edits, and take the disk copy when it
  does not.** Round 1 answered "refuse", flat; round 2 found that refusing
  unconditionally would falsify Phase 2's shipped loop, its README wording and
  its by-eye gate case, so the answer gained the condition that keeps them true.
  A dirty buffer is kept and the divergence is named, and the author resolves it
  by saving or by reopening; the app makes neither choice for them. Landed in §2
  as its own decision, with the three-outcome rule and the mechanism it needs.

  **That mechanism is a second action, not a narrower filter**, and this
  sentence says so because an earlier draft of it said the opposite. The path
  **stays** in the watch filter while the pane owns it — dropped from the filter
  it would never reach the rule at all — and what changes is that the event runs
  the rule instead of a bare recompile. That is what makes a save produce no
  second compile without a self-write suppression the loop's own 12 ms
  measurement says would be racy.

- **OQ-6** — ~~should the pane keep drawing the artifact, given what OQ-3 cost?
  The reader returns to the first page on every save, and §2 records why: the
  view that draws a real PDF is the view that tells the app nothing. The two
  candidate answers are already written down. Take §2's escape hatch —
  `typst-svg`, one page per frame, already in `Cargo.lock` — and the pane
  becomes ordinary HTML whose scroll offset is read and restored exactly, at
  the price of the live links, the selectable text and the accessibility
  tagging §2 chose the PDF for; or keep the PDF and accept the cost
  permanently, in which case this entry closes as a non-goal rather than an
  answer. A third shape, an app-owned page number restored through `#page=N`,
  was considered in the round and is weak: the number goes stale the moment the
  reader scrolls with a trackpad rather than through the app's own controls.~~

  ~~**This one needs use, not analysis.** The honest input is a person running
  Phases 2 and 3 on a document long enough to scroll, which is why it is not
  answered now and why it blocks neither. Design call. Blocks nothing; it is a
  decision to make before Phase 4 puts a second pane beside this one.~~
  **RESOLVED (2026-08-10), in use rather than in a review round: the pane keeps
  drawing the artifact.** This entry asked for a person running Phases 2 and 3
  on a document long enough to scroll, and that is what answered it, the day
  Phase 3 shipped: `samples/article.md` at three pages, opened in the window,
  scrolled past the first page, then edited in a separate editor and saved. The
  return to page 1 is real friction, and it was judged tolerable. §2's pane
  decision stands, and the escape hatch stays on the record with its price
  rather than being taken.

  **It does not close as a non-goal, and the reason is a shape this entry could
  not have considered.** The third option rejected above — an app-owned page
  number through `#page=N` — was rejected because the number goes stale the
  moment the reader scrolls by hand. That objection is about the *reader's*
  position. Phase 4 puts a text pane in the app, which makes a different thing
  available: follow the *author's cursor*, mapping the line being edited to a
  page and setting `#page=N` on each fresh blob URL. Phase 2's round confirmed
  the write direction on exactly that operation — a fragment on a *new* blob
  URL, honoured at load — and the staleness objection does not reach it,
  because the cursor lives in the app's own editor rather than in a view that
  tells the app nothing. **Phase 4 carries that**, as a design question for its
  own round rather than an answer made here. What is settled is the pane: it
  draws the artifact, and Phase 4 is designed against a PDF view.

- **OQ-7** — ~~should the preview follow the author's cursor, mapping the line
  being edited to a page and setting `#page=N` on each fresh blob URL? OQ-6
  handed the question to Phase 4's round, and **that round put it outside Phase
  4**, for a reason worth stating rather than a matter of size. Nothing in
  `core` maps a source line to a page: `core/src/lib.rs` exports `md_to_typst`,
  `image_paths`, `md_to_pdf`, `Asset`, `ImageRef` and `Error`, and no more. So
  building it needs `core` to gain an output it does not have, which is the one
  thing §2 stakes this spec's falsifiable claim on not needing — a change that
  size is a phase with its own round rather than a rider on the phase that adds
  an editor. What makes it worth keeping open is that the objection which killed
  OQ-6's third shape does not reach it: that number went stale because the
  reader scrolled, and a cursor lives in the app's own text pane. Design call,
  with the mechanism answerable from Typst's own crates. Blocks nothing; it is a
  phase to append if the answer is yes.~~

  ~~**Phase 6 drafts the yes, at heading granularity, and this entry stays open
  until that phase's round converges** — `reviewed` is what closes a question
  this spec left to a round, not the drafting of the phase that answers it.~~
  **RESOLVED (2026-08-15), when Phase 6's round converged at zero blocking in
  round 3: yes, at heading granularity.**
  What the phase settles, and the round checked: the anchor is a heading rather than the
  cursor, because the Nth markdown heading is the Nth compiled one and no source
  map is needed; "answerable from Typst's own crates" turns out to mean **no new
  crate at all**, since the page comes from a `PagedDocument` that
  `core/src/lib.rs:md_to_pdf` already builds and drops; and the `core` output
  this entry predicted is one additive function rather than a change to an
  existing signature. What the phase does **not** settle is whether heading
  granularity is enough, which is a use question of the kind OQ-6 was answered
  by rather than one a round can close.

  **Two things the round added that this entry could not have.** That
  `PagedDocument` is nameable only *inside* the function that builds it, because
  `typst` re-exports `typst-library`, `typst-syntax` and `typst-utils` and not
  `typst-layout` — so "no new crate at all" holds on a condition rather than
  outright, and the extraction staying inline is a constraint rather than a
  preference. And the feature needed a rule this entry never suspected: **the
  pane follows only a redraw that did not replace the text**, because assigning a
  textarea's `value` moves its caret to the end of the control, and without it
  the first keystroke would have broken three shipped gates. An open is not a
  cursor movement.

  **This entry is struck at the phase's close-out rather than in the round that
  converged**, which is where its own sentence puts the job. The review commit
  missed it, and this is the next place that could take it; nothing about the
  answer changed in between.

- **OQ-8** — what does it take to put this app on a machine that is not this
  one? Phase 5's round raised the question by falsifying the gate case that
  assumed the answer. That case asked for a launch on "a machine that has no
  Rust toolchain and no fonts installed", and neither half survives: macOS
  cannot be in the second state, because the system font set is not removable,
  and the first state is checked better and reproducibly by `otool -L` than by
  finding a second machine. What is left is the real question underneath, which
  is **distribution, and it is gated on signing rather than on packaging.**
  Measured during that round: an unsigned bundle carries only the linker's
  ad-hoc signature — `codesign -dv` reports `flags=0x20002(adhoc,linker-signed)`
  with `Sealed Resources=none` — and `spctl -a -vvv -t exec` rejects it with
  "code has no resources but signature indicates they must be present". So
  whether it launches elsewhere turns on how it travelled: a copy over USB or
  `scp` sets no `com.apple.quarantine` attribute and runs, while a `.dmg`
  downloaded or sent through AirDrop sets one and Gatekeeper refuses it until a
  person overrides by hand. **A gate keyed to that would be measuring the
  transfer, not the build**, which is why Phase 5 does not carry one. Answering
  this needs an Apple Developer account, a Developer ID Application
  certificate and notarisation credentials — external input, none of it on this
  machine on 2026-08-11. Blocks nothing; it is a phase to append when the
  credentials exist.

- **OQ-9** — what does the preview pane become on a platform whose webview does
  not draw PDFs? Raised on 2026-08-25 by Phase 7, which established that
  `view=FitH` is read once at load and built a transform around that fact, and
  in doing so made plain how much of this pane is **not code this project
  wrote**. The frame is handed a blob and addressed through PDF open
  parameters; everything between those two things is the webview's own PDF
  view. Windows runs WebView2, whose view is Chromium's — it exists, and the
  open-parameter behaviour Phases 6 and 7 both build on would have to be
  re-established against it rather than assumed. Linux runs WebKitGTK, which
  ships no built-in PDF viewer, so a frame fed a blob renders nothing and the
  pane is absent rather than different. Both are read from documentation, not
  from a build on either platform.

  **The candidate answer is to stop borrowing a viewer and draw the pages in
  the page**: `pdfjs-dist` ships browser-ready ES modules, so vendoring two
  static files keeps `withGlobalTauri`'s whole point — no bundler, no node
  toolchain, one Cargo build. It would also answer three things this pane
  cannot do today: fit-to-width as a *mode* that survives a resize rather than
  a transform standing in for one, a zoom the reader controls, and a click in
  the page that names the source line it came from. What it costs is the
  property Phase 1 recorded — that the pane "draws the artifact itself, not a
  picture of it", with live annotations, selectable text and the tagging that
  rides them. `pdf.js` rebuilds the first two and is weaker at the third.

  **Blocks nothing today and gates every port.** It is a spec of its own rather
  than a phase here: it would supersede this spec's decision about what draws
  the page, and §6.1 step 1 says a phase never removes shipped work.

- **OQ-10** — what checks `app/dist/index.html`? Raised on 2026-08-25 by Phase
  7, which could not write an exit gate that this repository's suite could run.
  OQ-2's `withGlobalTauri` bought the app one Cargo build and no node toolchain,
  and the price recorded there was that there is no JavaScript test harness
  here. That price was small while the page only placed what Rust chose. It is
  larger now: Phases 7 and 8 put the pane's geometry, the fit, the gutter's
  measurement and the caret's mark in that file, and every one of them is
  checked by eye.

  **The sharpest edge is narrower than the whole file, and it is worth naming
  separately: nothing checks that the page and `Status` agree.** `invoke` returns
  an untyped value and the page reads ten fields off it by name; a field renamed
  in Rust breaks the window silently, at runtime, with no console anyone reads.
  Phase 4 of `mpdf-008` adds two more such fields.

  **A candidate answer that costs no toolchain**: `core/tests/page_examples_test.rs`
  already `include_str!`s `web/index.html` and enforces claims about a
  hand-written page from the Rust suite, so the idiom exists in this repository.
  A test in `app/` that reads `dist/index.html`, scrapes its `state.<field>`
  accesses and asserts each against a serialized `Status` would close the
  narrow edge. The wide one — geometry, measurement, layout — is not reachable
  that way and would need a webview driver, which is a different decision with
  OQ-2's price attached to it. Blocks nothing; the narrow half is a phase to
  append.

  > **PART-ANSWERED 2026-08-27, by Phase 9, and the question stays open for the
  > rest.** The narrow edge is closed twice over rather than by the scrape
  > proposed above: a JSDoc `@typedef` the page is type-checked against, and a
  > Rust test comparing that typedef's field list to a serialized `Status`. The
  > pair is tighter than the scrape — **two declarations compared, rather than
  > usage compared against a declaration** — and the scrape is not built.
  >
  > **`tsc` over the shipped file, measured before the phase was drafted: 243
  > errors with the strict flags on, 41 with them off, and none of the 243 a
  > defect.** That is what chose the settings, and it is also the honest size of
  > the win: of the eight defects this file has produced, a type check catches
  > **one** — `doc?.destroy()`, a method `PDFDocumentProxy` does not have,
  > swallowed by optional chaining. **That one was caught in review before it
  > reached `main`**, in the same commit that introduced it, which makes the
  > type check's measured record against shipped defects zero and its record
  > against *written* ones one in eight. The seven it does not reach are
  > `ResizeObserver` feedback, observer-delivery ordering, a dropped forced
  > pass, a stale `deliveryMs`, a timer cancelling a render, a counter advanced
  > for a render that never drew, and an early return leaving a caret band on an
  > empty pane. **Every one is behaviour, and the wide half of this question is
  > the only thing that reaches them.**
  >
  > **What the wide half now has that it did not: evidence that it works.**
  > `mpdf-009` Phase 3 was verified by serving the *shipped* `dist/index.html`
  > over http with `window.__TAURI__` stubbed and driving it in Chromium. It
  > reproduced the app's own literals exactly — `scrollHeight` 53,337, 116.64
  > MiB at a 520 px pane — and, A/B'd against the parent commit, is what
  > identified `overflow-x: auto` as the cause of 21 `ResizeObserver` errors a
  > run. The harness was a scratch file and was thrown away. Committing it is
  > the wide half, and it still carries OQ-2's price: a browser driver is a node
  > toolchain. *(needs-input — the price is a judgement, not a measurement)*

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Phases 1 to 3 build a
previewer, which is usable on its own. Phase 4 makes it an editor, and it is
the one phase this spec could lose without invalidating the rest.

### Phase 1 — the window, and one compile on screen
*Produces the observable: yes — the typeset PDF, on screen, from a document
the user picked.*

- **Scope:** Add the `app` crate to the workspace — package `md2pdf-app`,
  binary `md2pdf-app`, the seven files OQ-2 lists, `members` in the root
  `Cargo.toml` gaining `"app"`. An Open dialog through `tauri-plugin-dialog`,
  filtered to `md`, with the capability entry OQ-2 names; the app opens one
  file at a time. On open, in plain functions per §2: read the document, read
  its assets the way `cli/src/main.rs:read_assets` does, call
  `md2pdf_core::md_to_pdf`, and hand the bytes to the page, which draws them
  per §2's blob decision. The bytes cross the boundary as `tauri::ipc::Response`
  rather than as a returned `Vec<u8>`, which would serialize as a JSON array of
  numbers — a performance choice rather than a correctness one, named here so
  it is not rediscovered. The window titles the open document. No watching, no
  editing, no export, no status chrome — Phases 2 and 3 own those.

  Two failure classes reach the pane, and they are not the same type. A
  compile failure is a `md2pdf_core::Error`, and the pane draws its `Display`
  — the same text the CLI prints after its `error: ` prefix. A file that will
  not read is not an `Error` at all: `cli/src/main.rs:read_assets` builds a
  plain `String` for it, and this app builds the same sentence for the same
  case, so a missing figure names the path and the line here exactly as it does
  at the terminal. Either way the pane shows the message and the app stays
  open. Neither keeps a previous page, because Phase 1 has none to keep.
- **Exit gate:** (1) Opening `samples/article.md` draws its first page, and
  opening `samples/press-release.md` draws its first page in the second look.
  **This case is read by eye by a person at the window** — the precedent is
  `mpdf-001`'s Phases 7 and 9, which read a PDF by eye for a claim no test
  could hold; Phase 6 of that spec faced the same problem and chose a textual
  assertion instead, which is available there and is not here. OQ-1's residual
  is why: nothing readable from JavaScript proves the pixels, and the machine
  this spec was drafted on denies the screenshot. (2) Opening
  `tests/fixtures/unsupported_html.md` draws the message naming the construct
  and line 5, and the app stays open — the same rejection the CLI makes, at a
  third level. (3) Unit tests over the plain functions, three cases, each with
  the directory it needs, because one fixture cannot carry all three: with
  `dot.png` and `figures/mark.svg` both present beside a copy of
  `tests/fixtures/figure.md`, the reader returns two `Asset` values; with
  `figures/` absent — which is how `tests/fixtures/` actually stands — it
  reports `figures/mark.svg` by path and by line 5; and over an inline document
  naming `dot.png` twice, in a directory holding that one file, it returns one
  `Asset` and reads the file once. The last case takes an inline document
  rather than a fixture: `images.md` repeats `dot.png` on lines 3 and 10, but
  names `fig#2.png` on line 7, which no directory holds, so a reader fails
  there before it ever reaches the repeat. (4) `cargo test --workspace`
  still passes and `core` and `cli` are untouched, which is §2's falsifiable
  claim checked as a diff.
- **Close-out:** Seed `rules/desktop.md` with all five frontmatter keys the
  methodology's §8.1 requires, `sources` naming the new crate's files. The
  README gains a section for the app. One push.

### Phase 2 — the watch loop
*Produces the observable: yes — the same PDF, redrawn on every save, which is
the whole point of the app.*

- **Scope:** Watch the open document's own directory, recursively, per §2's
  decision — one watch, not a list of files. `notify = "8.2.0"` is the crate,
  pinned as `app/Cargo.toml` pins the others, and it is the workspace's first
  new dependency since Phase 1. The debounce is this app's own rather than
  `notify-debouncer-full`'s, so that the constant below is one this project
  measured and one its own test pins.

  On each event, filter before anything else: the path is relevant when it is
  the document, or one of the paths `md2pdf_core::image_paths` returns resolved
  against the document's directory. **Both sides are canonicalized before the
  comparison**, for the reason §2 records — the event arrives resolved and the
  dialog's path does not. That list is recomputed after each successful
  *parse*, meaning emission and not the compile: `image_paths` succeeds on a
  document whose figures are all missing, which is exactly what keeps the list
  available while the compile fails. Everything else under the directory is
  dropped, which is what a directory-valued watch buys and pays for.

  Then debounce, because one save arrives as several filesystem events. **The
  interval is measured, not guessed**, by the method §2 used for the compile
  timings: count the events one save produces in each of three editors that
  write differently — an in-place write, a write-then-rename, and an atomic
  replace — twenty saves each, and state the medians beside the constant.

  On each settled change: re-read, re-compile, redraw. **The compile happens
  once, in the loop, in Rust.** The page's invoke returns bytes already
  compiled and never triggers a compile of its own, so one save is one compile
  and no second read can race the first. Rust emits a signal carrying no
  payload, and the page invokes for the current bytes; a `window.emit` carrying
  them would serialize them as a JSON array of numbers, one per byte, which is
  the cost §2's IPC decision already refused. Phase 1's `open_document` both
  compiles and returns in one call, and this phase separates those, because the
  loop now compiles with nobody asking.

  A failed compile keeps the last good page, shows the error above it, and
  marks the pane stale, per §2. **The app therefore gains state Phase 1 did not
  give it**: the last good PDF bytes and a stale flag, held in Rust, written by
  the loop because the loop is what compiled, and read by Phase 3's export so
  that the file and the page cannot disagree. Phase 2 sets the flag and draws
  the error; the chrome that spells "stale" as a word in the window is Phase
  3's, and this phase adds none.

  The reader's place is not kept. §2 records the measurement that decided it and
  OQ-6 carries what is left; nothing in this phase attempts it.
- **Exit gate:** (1) Unit tests over the plain functions: the watch set for a
  document is that document's directory; the filter admits `figure.md`'s own
  path and both `dot.png` and `figures/mark.svg`, rejects a sibling
  `notes.txt`, and **admits an event path that differs from the document's only
  by `/var` against `/private/var`**, which is the canonicalization §2 records;
  two events inside the debounce window produce one compile and two outside
  produce two; a compile error leaves the previous bytes as the current ones
  and sets the stale flag. (2) A test that drives the real watcher, in a
  scratch directory holding a copy of `samples/article.md` and its two figures:
  rewriting the document produces a recompile within a bounded wait, and
  replacing `pipeline.svg` with different bytes — the document untouched —
  produces another. **That directory goes under `std::env::temp_dir()`**, whose
  real path differs from itself on macOS, so this case fails loudly on a
  canonicalization bug rather than passing under some directory that happens
  not to be symlinked. Counting compiles needs a seam, and
  `app/src/document.rs:read_assets_with` is the one Phase 1 built for the same
  reason. This case proves the wiring, not the count — the count is (1)'s
  debounce test, which needs no filesystem and cannot flake on one — and its
  wait is bounded generously, because FSEvents' own coalescing sits under the
  debounce. (3) A document naming `figures/new.svg`, which does not exist, is
  watched anyway, and creating that file afterwards produces a recompile that
  now succeeds. This is the case OQ-4 was about and the one a file-valued watch
  set could not have held; no other case reaches it. (4) Opening a second
  document, in a different directory, moves the watch: a change to the first no
  longer redraws and a change to the second does. An implementer who sets the
  watcher up once rather than per document passes every case above and fails
  this one. (5) **Read by eye once, at the window:** editing
  `samples/article.md` in an editor and saving redraws the page, with no action
  taken in the window. This is the one claim no test holds — that the right
  pixels reached the glass — and §2's rule is that the list stays at one item,
  which is why the draft's other three window checks are tests above. (6)
  `cargo test --workspace` still passes and `core` and `cli` are untouched:
  §2's falsifiable claim, checked again at the phase that adds a dependency and
  new state and is therefore the likeliest to leak.
- **Close-out:** Update `rules/desktop.md` against the code, **raising its
  `max_lines` in the same pass** — its body sits at exactly its cap of 80
  today, so a watch loop cannot be documented without it. The README's app
  section says the app "does not yet watch the file", which this phase makes
  false, so it changes too. One push.

### Phase 3 — export, and the state the loop needs to show
*Produces the observable: yes — the PDF written to a file the user names,
which is the artifact the CLI writes.*

- **Scope:** A Save-a-copy command that writes the current PDF bytes where the
  user asks, defaulting to the document's path with a `.pdf` extension, which
  is `cli/src/main.rs:default_output`'s rule. It follows Phase 1's dialog
  pattern — a `File` menu item emits to the page, the page opens
  `tauri-plugin-dialog`'s save dialog and invokes with the path — so **the
  capability file gains `dialog:allow-save` beside `dialog:allow-open`**, which
  the plugin ships and which OQ-2's idiom says to name here rather than
  rediscover at a build. Nothing here re-compiles: export writes the bytes the
  pane is already showing, so the file and the page cannot disagree.

  The window then shows what the loop knows: the state, the compile time and
  the error. **The open document is not a fourth thing to draw** — it is already
  the window's title, from Phase 1, and this phase does not draw it twice.

  **The state machine has four states and the app holds one bit today.**
  `app/src/preview.rs:Preview` sets `stale` and `error` together and clears them
  together, so those two fields carry one bit between them. The four are:
  *empty* — no document has been opened, which is the state the app launches
  into and holds until the first Open; *current* — the last compile succeeded;
  *stale* — it failed and an older page is still drawn; *failed* — it failed
  with no page to keep, which is the open that never compiled.

  **What separates *stale* from *failed* is `Preview::pdf().is_some()`, not
  anything `app/src/main.rs:current_pdf` does.** That command reads
  `if preview.is_stale()`, and a failed compile always sets `stale`, so *stale*
  and *failed* both take its first branch; its second `Err` is reachable only
  when the flag is clear and there are no bytes, which is `Preview::default()`
  and so is *empty* alone. Phase 2's round 2 caught the same class of mistake
  one layer down, and this sentence is written out because the first draft of
  this phase attributed the split to the wrong branch.

  **Two things the state needs do not exist yet and this phase adds them**: the
  compile duration, which nothing in `Preview` or `app/src/document.rs:Render`
  carries, measured around the compile in `Preview::compile`; and an accessor
  for the open document, which is private today and which the export's default
  path needs. The status crosses to the page as its own command beside
  `current_pdf`, invoked on the same payload-less `rendered` signal — a second
  command rather than a widened return, because the bytes cross as raw
  `tauri::ipc::Response` and a status does not.

  **The status line is a value a plain function computes**, per §2, and the page
  renders it. That is what keeps this phase's gate to tests: a window that
  formats its own status would be checkable only by eye, and §2's rule is that
  the list of by-eye claims stays at one item, which Phases 1 and 2 have spent.

  The limit this phase accepts, recorded rather than fixed: **"stale" answers
  "did the last compile fail", not "does the page match the file on disk".**
  For the debounce interval plus the compile — 100 ms by
  `app/src/watch.rs:DEBOUNCE`, plus the 8.5 ms to 28.7 ms §2 records, and those
  medians include a process spawn the app does not pay, so the real window is
  smaller than the sum — a saved document leaves the pane reading *current*
  while its bytes belong to the older text, and an export in that window writes
  those older bytes. The promise export makes is that the file and the *page*
  agree, which it keeps; the file and the *disk* is a different claim and this
  phase does not make it.
- **Exit gate:** (1) The byte-identity claim, split in two because §2 records
  that no single test can hold both ends, and composed. **Both halves take
  `samples/article.md` and the two figures it names**, copied into a scratch
  directory; `tests/fixtures/figure.md` is the trap to avoid, because its
  `figures/mark.svg` is absent, so both sides would fail rather than agree.

  **(1a), in `app`:** the export writes the bytes `Preview::pdf()` holds, byte
  for byte, triggers no second compile, and defaults its path to
  `input.with_extension("pdf")`, which is `cli/src/main.rs:default_output`'s
  rule. It also asserts those bytes against an in-test `md2pdf_core::md_to_pdf`
  call over assets the test reads itself. **That last assertion is the middle
  leg of the composition and it is not optional**: without it the two halves
  meet only through §2's line-for-line reading of
  `app/src/document.rs:read_assets_with` against `cli/src/main.rs:read_assets`,
  and a later divergence in either reader would pass both halves while the
  wrappers silently disagreed.

  **(1b), in `cli/tests/cli_test.rs`:** the `md2pdf` binary's output for that
  same document is byte-identical to `md2pdf_core::md_to_pdf` called in process
  with the assets read the same way. `cli/Cargo.toml` already carries both what
  this needs — the `[[bin]]` that sets `CARGO_BIN_EXE_md2pdf`, which
  `cli/tests/cli_test.rs:BIN` uses today, and `md2pdf-core` as a dependency.
  It writes into a scratch directory rather than beside the sample, because
  export's default path and `md2pdf`'s default path are the same path and one
  would overwrite the other.

  **Both halves are tests; no case in this phase is read by eye.** The cost of
  that choice, recorded rather than hidden: the export's user path — menu item,
  page, save dialog, invoke — is then exercised by nothing, exactly as Phase 1's
  Open path is. It is the trade §2's one-item rule buys, and it is named here so
  it is a decision rather than an oversight.

  (2) **Export is refused, with a message, unless the pane is current** — which
  is two refusals to test, not one: while the pane is stale, where the bytes
  exist but are known to belong to older text, and while no document is open,
  where there are no bytes at all. The second is not the first —
  `Preview::default()` has `stale` clear and no bytes — and an implementer who
  tests only the stale refusal leaves the launch state to panic or to write
  nothing silently. (3) Unit tests over the status a plain function
  computes, **one per state, so four**: empty names no document and no time,
  current names the compile time, stale names the error and keeps the page, and
  failed names the error and has no page. (4) `cargo test --workspace` still
  passes, and `core/src` and `cli/src` are untouched — §2's falsifiable claim,
  at the phase whose gate is the first to reach into `cli` at all. `cli/tests/`
  gains (1b) and nothing else; a shared `default_output`, or any edit that makes
  one crate's binary reachable from the other, is what this case is watching
  for.
- **Close-out:** Update `rules/desktop.md` against the code, **raising its
  `max_lines` again in the same pass** — its body sits at 151 against a cap of
  155, which does not hold an export command, a dialog and a capability, a
  status value and a new timing field. The README's app section gains the
  export. One push.

### Phase 4 — the text pane
*Produces the observable: yes — the same PDF, redrawn as the author types
rather than as they save.*

- **Scope:** A text pane beside the preview in `app/dist/index.html`, holding
  the open document's text. It is a plain text editor, not a rich one: no
  syntax highlighting, no autocomplete, no formatting commands. Those are a
  later phase or a later spec, and naming them here is what stops this one
  growing into an editor project.

  **The pane's text becomes what compiles, and that is the change this phase
  really makes.** Every compile path in the tree reads the disk today:
  `app/src/preview.rs:Preview::compile` calls `app/src/document.rs:render`,
  which calls `app/src/document.rs:render_with`, which opens with
  `std::fs::read_to_string`. This phase splits that — the markdown string
  becomes a parameter and the disk read moves out to the caller — and `Preview`
  gains the text it last compiled. **`core` needs nothing**:
  `core/src/lib.rs:md_to_pdf` already takes a `&str`, which is the same claim
  Phase 1 checked and the reason this phase is not a rewrite either. The image
  list `app/src/preview.rs:Session::classifier` feeds to
  `app/src/watch.rs:classify` follows the buffer, for the same reason
  everything else does: the buffer is the document now. (Those two were
  `Session::filter` and `is_relevant` when this phase was drafted and reviewed;
  the pointers are corrected here because the implementation renamed them, which
  §6.1 allows in the section the phase itself touches.)

  Save writes the buffer to the open document's path, from a `File` menu item
  at `CmdOrCtrl+S` and a button beside the two the header carries.
  `app/src/main.rs:menu` reserved that accelerator in Phase 3 and says so, so
  this phase spends it rather than choosing it.

  **The buffer lives in Rust, beside the state the loop already writes.** The
  pane sends its text on each change through a command of its own — one more
  beside Phase 3's four — and `Preview` holds it, along with the text as it
  stood at the last open or save, which OQ-5's rule needs. Keystrokes therefore
  cross the IPC boundary and the debounce is Rust's, which is what lets gate (1)
  test it at all; a debounce in the page would be logic reachable only by
  clicking, which §2 refuses.

  The traffic the other way follows Phase 3's precedent rather than inventing
  one: the replacement text an external change is taken as, and the divergence
  report a refused one produces, both reach the page the way the status does —
  through a command answering a payload-less signal, with every word chosen in
  Rust. **A divergence is not `stale`**, because nothing failed to compile, and
  whether it joins `app/src/preview.rs:Status` or sits beside it is an
  implementation choice the gate does not turn on: gate (3) asserts the rule's
  own return.

  **While the pane owns the document, its own path stops triggering a recompile
  directly and starts triggering OQ-5's rule**, per §2's decision, and that is
  what makes a save produce no second compile. **The path stays in the filter**
  — dropped from it, an event would never reach the rule — and the figures keep
  triggering a recompile directly. The directory watch is unchanged.
  `app/src/watch.rs:start` carries one filter and one callback today, so this
  phase is where the document's events and the figures' events learn to reach
  different code. Nothing suppresses a self-write, because §2 records that the
  rule's first outcome is the self-write case and that suppressing one by timing
  would be racy.

  The external-change rule is OQ-5's resolution, recorded in §2 as three
  outcomes over three strings: the file equal to the buffer does nothing, the
  file differing under a clean buffer is taken and recompiled, and the file
  differing under a dirty one is refused and reported.

  **The typing debounce is its own constant, not `app/src/watch.rs:DEBOUNCE`.**
  That one is 100 ms measured against FSEvents' batching, which is not what this
  gates. **This one is measured too**, by the method §2 used for the compile
  timings and Phase 2 used for its own interval: time twenty compiles of
  `samples/article.md` and twenty of `samples/press-release.md` through the
  pane's own path — in process, with no spawn, unlike §2's published figures —
  and state the medians beside the constant, as `DEBOUNCE`'s doc comment states
  its own.

  Following the author's cursor is **not in this phase**. OQ-7 carries it and
  records why it is a phase of its own rather than a rider on this one.
- **Exit gate:** **Every case is a test, and none is read by eye.** §2 caps the
  by-eye list at one item and Phases 1 and 2 spent it; there is also no
  JavaScript test harness in this repository, and OQ-2's `withGlobalTauri`
  decision — no bundler, no npm toolchain, no `package.json` — is what keeps it
  that way. So the logic goes in plain Rust functions and the page renders,
  exactly as Phase 3 did. The cost, recorded rather than hidden: typing in the
  pane itself is exercised by nothing, as Phase 1's Open path and Phase 3's
  export path are.

  (1) A compile of text that is not on disk: the pane's string compiles, and the
  file beside it need never have held that text. Two touches inside the debounce
  window produce one compile and two outside produce two, over the shape
  `app/src/watch.rs:Debounce` already provides — time as a parameter, so the
  case needs no clock and no filesystem and cannot flake.

  (2) Save writes the buffer to the document's path, and the watch loop then
  compiles **no** second time, counted through the seam
  `app/src/preview.rs:counted` already provides, in a scratch directory under
  `std::env::temp_dir()` for the reason Phase 2 gives. **A figure changed in
  that same directory still compiles**, and that half is what proves the filter
  narrowed rather than stopped — an implementer who drops the watch entirely
  passes the first half and fails this one.

  (3) The external-change rule, at the plain-function level, **one case per
  outcome, so three**: a document rewritten with text equal to the buffer does
  nothing at all, because that is the app's own save arriving back; a document
  rewritten under a **clean** buffer is taken and recompiled, which is Phase 2's
  shipped loop and the case an unconditional refusal would have broken; and a
  document rewritten under a **dirty** buffer leaves the buffer, the compile and
  the preview untouched and reports the divergence. An implementer who tests
  only the third ships a pane that stops redrawing on an external save.

  (4) A document opened, edited, saved and reopened round-trips **byte for byte
  against the buffer at save**, not against the original, which an edit has
  already made unequal. Both halves are asserted: the file the save wrote equals
  the string the pane held, and reopening yields that same string. The hazard it
  is aimed at is a text pane normalising CRLF to LF, or dropping a trailing
  newline.

  (5) `cargo test --workspace` still passes and `core/src` and `cli/src` are
  untouched — §2's falsifiable claim, at the phase with the strongest pull in
  the spec toward a `core` edit, which is what OQ-7 exists to hold back.
- **Close-out:** Update `rules/desktop.md` against the code, **raising its
  `max_lines` again in the same pass** — its body sits at 206 against a cap of
  210. **Three of its claims stop being true as written and are corrected rather
  than appended to**: that the compile reads the document from disk; that the
  export writes "the file `md2pdf` writes, byte for byte, for the same
  document", which now holds only while the buffer and the file agree; and that
  the app "recompiles on every save", which now holds for an external save only
  while the buffer is clean. Phase 3's recorded limit on "stale" widens with
  them — that window was the debounce plus the compile, and an unsaved buffer
  has no bound at all. The README's app section gains the text pane and the
  save, says that the PDF follows the pane rather than the file, and qualifies
  its "**Save the file and the page redraws**" with the one case where it no
  longer does. One push.

### Phase 5 — an app you can install
*Produces the observable: yes — the same PDF, from an app launched from the
Applications folder rather than from a build command.*

- **Scope:** A macOS bundle — an `.app` and a `.dmg` to carry it — and the one
  piece of Rust that a bundle makes reachable and a build command does not: the
  document association for `.md`, and the code that answers it.

  **The configuration is `app/tauri.conf.json`, and its `bundle.active` is
  `false` today**, which is the key the whole phase turns on. It becomes `true`,
  `bundle.targets` names `app` and `dmg`, and `cargo tauri build` is the one
  documented command. The two obvious commands disagree under the current value
  and that costs a build to discover, so it is recorded here in OQ-2's idiom:
  with `active` false, `cargo tauri build` skips bundling while the standalone
  `cargo tauri bundle` bundles anyway. `identifier` is already
  `dev.md2pdf.desktop` and does not change.

  **`bundle.fileAssociations` is the association**, and it emits
  `CFBundleDocumentTypes` into `Info.plist`: `ext` maps to
  `CFBundleTypeExtensions`, `name` to `CFBundleTypeName`, `role` to
  `CFBundleTypeRole`, `rank` to `LSHandlerRank`. It emits no
  `LSItemContentTypes` unless asked, and modern LaunchServices prefers a UTI, so
  if the extension alone does not take, the key is `contentTypes` with
  `net.daringfireball.markdown`.

  **Spell it `contentTypes` and not `content-types`**, which cost a build to
  learn and is recorded so it costs no more. `tauri-utils` declares the field as
  `content_types` with `#[serde(alias = "content-types")]`, so the hyphenated
  spelling reads correctly — but the CLI never gets that far. It validates the
  file against the **generated JSON Schema** first, a serde alias does not appear
  in a schema, and `deny_unknown_fields` becomes `additionalProperties: false`
  there, so `cargo tauri` 2.10.1 stops with `Additional properties are not
  allowed ('content-types' was unexpected)`. The camel-case spelling passes and
  emits `LSItemContentTypes`. That key exists in the generation the installed
  CLI parses with, which is not a given: **`tauri-cli` pins at 2.10.1 here**, as
  OQ-2's idiom says to pin, and it parses this file with `tauri-utils` 2.8.3
  while `generate_context!` uses 2.9.3. `FileAssociation` is
  `deny_unknown_fields` in both, so a key that exists only in the newer
  generation is rejected by the CLI that is installed.

  **The association only launches the app; it hands the process nothing.** That
  is the work this phase adds to `app/src/main.rs:main`, which ends
  `.run(tauri::generate_context!())` today and surfaces no run events at all.
  It becomes `.build(tauri::generate_context!())?.run(|handle, event| …)`, and
  the event is `tauri::RunEvent::Opened { urls }` — macOS-only, fed by tao's
  `application:openURLs:`. **A bundled app is handed its document by that event
  and not in `argv`**, so nothing here reads `std::env::args`.

  Two details of that payload, in the idiom this section already uses for the
  keys that cost a build. The URLs are `file://`, and the path comes from
  `Url::to_file_path` and not from `url::Url::path`, which leaves a space
  percent-encoded — a document named `my doc.md` arrives as `my%20doc.md` and
  opens as nothing. `tauri` re-exports `Url`, so this adds no dependency to a
  crate that pins every one it has. And `urls` is a `Vec`, because Finder
  delivers a multiple selection as one event: **the app takes the first and
  ignores the rest**, which is Phase 1's "one file at a time" rather than a new
  decision.

  A cold launch reaches the process for a reason worth stating, because the
  obvious guess is wrong: tao's `AppState::open_urls` calls
  `handle_nonuser_event`, which **drops** an event when no callback is set
  rather than queueing it — the queue in that file is a different path, reached
  by `queue_event`, which this does not use. What saves the cold case is
  ordering. Tao installs the callback before `NSApp.run()`, so AppKit cannot
  deliver `application:openURLs:` before there is somewhere for it to go.

  **The open reaches the page rather than bypassing it, and the page's own
  `clear()` is why.** The draft of this phase said the page needed no change at
  all, and that was wrong in exactly the case gate (2) tests.
  `app/src/preview.rs:Session::open` rebuilds the preview from
  `Preview::default()`, so `revision` and `reloaded` restart at 0 for every
  document, while the page's `drawnRevision` and `takenReload` are reset only in
  `app/dist/index.html:clear()` — which the dialog path calls before it invokes
  and a path straight into Rust never reaches. Open a second document that way
  and Rust returns to `revision 1`, `reloaded 1`, which the page already holds
  from the first document: both panes keep showing the old one under a new
  title. The collision is exact rather than racy.

  So the run event **emits to the window the way the menu items do** — the
  idiom `app/src/main.rs` already uses for `OPEN`, `SAVE` and `EXPORT`, and the
  page already listens with `core:default`'s own `core:event:allow-listen`, so
  no capability is added. `app/src/main.rs:open_document` keeps its signature,
  keeps being `async`, and is not refactored, which also keeps Phase 1's
  recorded reason for that `async` — the compile stays on the runtime's pool
  rather than on the thread that draws the window.

  **The signal carries no payload and the page invokes for the path**, which is
  `RENDERED`'s own shape one command over, and it is what removes the launch
  race rather than betting on it. The run event stores the path in a managed
  slot and emits an `opened` signal, named beside `OPEN`, `SAVE`, `EXPORT` and
  `RENDERED`; the page takes that slot through a command of its own — a ninth,
  `pending_open`, returning the path or nothing — at startup beside its existing
  `refresh()`, and again on every signal; and the take clears the slot. A
  cold open that arrived before the page's listener existed is then collected by
  the startup take, a warm one by the listener, and whichever runs second gets
  nothing and does nothing — so the document cannot open twice. `open_document`
  is invoked with the path exactly as the dialog invokes it, after the same
  `clear()`, which is what keeps one open path in the page and the counters
  honest.

  What makes that sound is an ordering in `app/dist/index.html` that is
  load-bearing and does not look it: the script registers its `listen` calls and
  *then* calls `refresh()` as its last statement, so the take at startup goes
  after the listener rather than before it. **The limit it accepts, recorded
  rather than fixed**: `listen` completes over IPC, so an event landing between
  the startup take and the listener's actual registration would sit in the slot
  until the next signal. The source ordering makes that window practically
  unreachable, and the cost if it is ever reached is a document that opens late
  rather than one that opens wrong.

  **Opening a second document from Finder does what opening one from the dialog
  does today**: it replaces the buffer, and unsaved edits in the pane are lost.
  That is not a new decision and this phase does not make one — §2's
  external-change rule offers "open the file again to take it" as the way out of
  a divergence, and that way out works only because reopening replaces. Saying
  so here stops an implementer inventing a prompt for a permission this app's
  capability file does not carry.

  **The icon is the placeholder already in the tree, and this phase designs
  none.** `bundle.icon` stays `["icons/icon.png"]`, and the bundler synthesises
  `md2pdf.icns` from that lone 512×512 PNG — one `ic09` entry, 19,582 bytes,
  measured on a probe bundle. The Dock upscales it, which is what a placeholder
  should look like.

  **`cargo tauri icon` is deliberately not run**, and an earlier draft of this
  paragraph said it was. It writes 52 files across eight subdirectories,
  including iOS, Android and Windows assets that §1.1 puts out of scope, and its
  own `icons/icon.icns` — 12 entries, 74,735 bytes — is never read while
  `bundle.icon` names a PNG, so the command would change nothing about the
  `.app`. New artwork is a later job, and `rules/desktop.md`'s claim that "Phase
  5 owns the real icon" is corrected rather than met.

  **A bundle gets its own identity for privacy consent, and the watch loop
  depends on one.** Under `cargo tauri dev` the process inherits the terminal's
  grants; as `dev.md2pdf.desktop` it does not, and `app/src/watch.rs:start`
  watches a whole directory recursively. A document under `~/Documents`,
  `~/Desktop` or `~/Downloads` can therefore compile once through the open panel
  and then stop redrawing, which is the silent-failure class §2 already records
  for canonicalization, reached by a different route. The gate covers it.

  **Signing and notarisation are `bundle.macOS.signingIdentity` and
  `hardenedRuntime`, and neither can run here.** Measured on this machine on
  2026-08-11: `security find-identity -v -p codesigning` reports `0 valid
  identities found`, and `xcrun notarytool` holds no stored credentials. So the
  unsigned branch is the one that runs, and **"names the gap" means three
  concrete places** rather than a gesture: a sentence in the README's Install
  section, a line in `rules/desktop.md`, and OQ-8, which carries what an
  unsigned bundle cannot do.
- **Exit gate:** (1) **The bundle is self-contained and correctly described**,
  checked by shell over the built `.app` with no window involved.
  `otool -L Contents/MacOS/md2pdf-app` names only `/usr/lib` and
  `/System/Library` — no path under the build tree, under `~/.cargo`, or under a
  package manager's prefix. **The binary keeps the crate's name**: `productName`
  renames the `.app` and not what is inside it, so `CFBundleExecutable` is
  `md2pdf-app` while the bundle is `md2pdf.app`.
  `Contents/Info.plist` carries `CFBundleIdentifier`
  `dev.md2pdf.desktop` and a `CFBundleDocumentTypes` entry whose
  `CFBundleTypeExtensions` holds `md`. `Contents/Resources/` holds
  `md2pdf.icns` and **nothing else** — which is one assertion doing two jobs,
  since it is both the icon branch the scope chose and the absence of any font
  file.

  That last one is the fonts claim in the only form packaging can falsify, and
  the draft of this phase had it wrong. `core/src/lib.rs` embeds all five faces
  with `include_bytes!` and the Typst world exposes those alone, so "the fonts
  ship inside the binary" is a compile-time fact that no launch tests and no
  bundle can break. What a bundle *can* do is grow a `Resources/` font that
  somebody added on the theory that it was needed — so the case asserts the
  absence, and this phase adds nothing to `bundle.resources`.

  `codesign -dv` and `spctl -a -t exec` are run and their output is **recorded
  rather than asserted**: the unsigned branch fails `spctl` by design, and a
  gate that asserted a pass would be keyed to credentials this phase has already
  said it does not have.

  (2) **Read by eye once, at the window**, in one session on this machine, with
  the `.app` in `/Applications`, and three observations in it.

  **First**, launched from Finder, it opens `samples/article.md` **through its
  own Open dialog** and draws the page. That is the bundle running away from
  `cargo`, and it deliberately does not go through the association, so a failure
  here is the bundle and a failure below is LaunchServices.

  **Second**, a `.md` double-clicked in Finder launches it on that document from
  cold, and a second `.md` opened the same way while it is already running
  switches to that document — the case the counters above break, so it is the
  half that must not be skipped. **The observation names a step it cannot skip
  either:** the emitted entry ranks `LSHandlerRank` as `Default`, so any machine
  with an editor already registered for `.md` keeps that editor. Set this app as
  the handler first, through Finder's Get Info → Open With → Change All. A
  ranking that loses to an installed editor is not a broken association, and
  `rank: "Owner"` is the wrong fix for it.

  **Third**, a document under `~/Documents`, edited and saved in another editor,
  redraws the page — the consent check the scope names. **Its precondition is
  the first launch of this identity**, because consent is sticky and an operator
  who already granted it, or who holds Full Disk Access, sees a redraw and
  learns nothing. What the case is watching for is the negative: consent
  refused, the open panel still handing over the file, the page compiling once,
  and the watch then silently never firing again.

  **This phase takes a second by-eye item, and that is a departure §2 has to be
  argued past rather than waved past.** §2 keeps the list to one and Phases 1 and
  2 spent it; Phases 3 and 4 took none, each paying for it by leaving a user path
  exercised by nothing. The argument is that the artifact here exists only
  outside the harness: no Rust test can launch a `.app`, and the claim is not
  "the right pixels reached the glass" but "the bundle runs at all away from
  `cargo`", which is the entire subject of the phase. The rule's spirit is kept
  by pushing everything checkable into (1) and (3) and holding the by-eye part to
  one session.

  (3) **The build is one command** — `cargo tauri build`, documented in the
  README's Install section — and it produces `target/release/bundle/macos/`
  holding `md2pdf.app` and `target/release/bundle/dmg/` holding a `.dmg` named
  for the version and the architecture. The gate names those two paths so a
  second person has something to look for rather than a claim to believe.

  (4) `cargo test --workspace` still passes, and `core/src` and `cli/src` are
  untouched — §2's falsifiable claim, at the phase that rewrites the app's entry
  point and is the first to change how the binary starts.
- **Close-out:** Update `rules/desktop.md` against the code, **raising its
  `max_lines` again in the same pass** — its body sits at 303 against a cap of
  307, which is four lines and does not hold a bundle, an association, a Finder
  open and a signing gap. **Four of its claims stop being true and are corrected
  rather than appended to**: that "there is no installable bundle: that is Phase
  5"; that "Phase 5 owns the real icon", which this phase declines; and both
  places it counts the commands — it "registers eight" and "Each of the eight
  commands is a wrapper over a plain function" — which the ninth command and the
  `opened` signal falsify together. The
  README's Install section gains the bundle command beside the `cargo build`
  it documents today, and the app section's closing sentence — "the window is
  still built from source; an installable `.app` comes later" — is corrected.
  One push.

### Phase 6 — the page the author is on
*Produces the observable: yes — the same PDF, opened at the page the author is
editing rather than at page 1.*

Appended 2026-08-15, after Phase 5 shipped, per the methodology's §6.1: the
preview pane is this spec's subject, and OQ-7 reserved this item by name as "a
phase to append if the answer is yes."

- **Scope:** The pane opens on the page the caret is in. **The anchor is a
  heading**, and everything below follows from that choice rather than from the
  feature.

  **Why a heading, and not the cursor.** The general answer is a map from
  markdown offset to generated-Typst offset, fed to `typst-ide`'s own
  editor-sync function; §2's rejected shape below records why that is a later
  phase. What survives cheaply is an ordinal correspondence that already holds:
  **the Nth heading in the markdown is the Nth heading in the compiled
  document**, so nothing has to be mapped and — the load-bearing half —
  **nothing new is written into the generated Typst source**. An anchor the
  emitter *emits*, a `#metadata` marker before each block or a label per
  paragraph, would rewrite all 17 shipped golden files.

  **A draft of this paragraph called that an invariant every phase has held, and
  it is not one** — `mpdf-001`'s look phase moved thirteen goldens on their
  second line, gaining `date: none`, and said so in its own commit message. The
  distinction that survives is sharper than the false claim: those goldens moved
  because **what the document compiles to** changed, which is the thing a golden
  file exists to pin. An anchor marker moves all 17 for a reason that has nothing
  to do with what the document says, and this phase's own gate (5) can hold "no
  shipped golden file changes" only because it emits nothing. Headings are
  already in the output; that is the whole reason they are the anchor.

  **`core` gains one function, and this is the phase that crosses §2's
  falsifiable claim.** That claim is "`core` gains nothing and changes nothing",
  checked as a diff by all five shipped phases, and it is crossed here rather
  than slipped past. The argument for letting it: the claim was about whether
  *building the app* forced a rewrite of a library written for a CLI, and five
  phases answered no. An additive export that leaves every existing signature
  alone is not that rewrite — but it is the first thing to cross the line, so
  the round should say whether the line moved or the claim was always narrower
  than its wording. **`cli/src/main.rs` keeps calling `md_to_pdf` and is
  untouched**, which is how that stays checkable.

  The shape:

  ```rust
  pub struct Anchor { pub line: usize, pub page: usize }
  pub struct Rendered { pub pdf: Vec<u8>, pub anchors: Vec<Anchor> }
  pub fn md_to_pdf_with_anchors(md: &str, assets: &[Asset]) -> Result<Rendered>
  ```

  **`md_to_pdf` becomes a wrapper over it** rather than a second path —
  `md_to_pdf_with_anchors(md, assets).map(|r| r.pdf)` — for the reason
  `core/src/emit.rs:options` gives for one options builder: two paths over the
  same input that could disagree eventually do.

  **Where each half comes from.** The line: `core/src/emit.rs:Walk` gains a
  `headings: Vec<usize>`, pushed at the `Tag::Heading` arm from
  `line_of(md, range.start)` — the range `core/src/emit.rs:emit` already walks
  with through `into_offset_iter()`, and the one every error in that file
  already reports. The page: `core/src/lib.rs:md_to_pdf` already builds the
  `PagedDocument` that `typst_pdf::pdf` consumes and then drops it. Measured
  against the Typst 0.15.1 that `core/Cargo.toml` pins — `PagedDocument::
  introspector` answers `query(&HeadingElem::ELEM.select())` with the headings in
  document order, and `position(location)` returns a
  `PagedPosition { page: NonZeroUsize, point: Point }`. Two mechanical details,
  recorded so they cost no build: `Selector::Elem` takes
  `(Element, Option<SmallVec<…>>)` rather than a type, which is why the idiom is
  `Element::select`; and two traits have to be in scope where nothing at the call
  site looks like it needs them — `typst::introspection::Introspector` for
  `query`, and `typst::foundations::NativeElement` for the `ELEM` const — while
  `position`, being inherent, needs neither.

  **So the page comes from a value the function already holds, and no dependency
  is added — but that second half holds only while the type stays unnamed.**
  `typst` re-exports `typst-library`, `typst-syntax` and `typst-utils`, and not
  `typst-layout`, so `PagedDocument` and `PagedIntrospector` are unnameable from
  `core` today. Method calls on the value `typst::compile` infers are fine; a
  helper written `fn anchors(doc: &PagedDocument, …)` is not, and would force
  `typst-layout` into a workspace that pins every dependency it has. **The
  extraction therefore stays inline in `md_to_pdf_with_anchors`**, which is a
  constraint on the implementation rather than a preference about it.

  **The zip is by ordinal, and it is guarded rather than trusted.** Two facts
  make the correspondence hold today and one keeps it honest tomorrow. Measured:
  neither `core/assets/template.typ` nor `core/assets/press-release.typ` emits a
  `heading` element of its own — both set the title with `text(size: …, weight:
  "bold", title)` — so no bundled look contributes a heading the walk never saw.
  And a heading inside a footnote definition is walked by
  `core/src/emit.rs:collect_definitions`, whose `Walk` is discarded and whose
  content is spliced in at the *reference*, so its line would name the wrong
  place.

  **If the two counts differ, the function returns no anchors at all** and the
  pane behaves exactly as it does today, which makes a third look that set its
  title as a heading fail visibly rather than silently mis-scroll.

  **What the guard does not buy, stated because a draft of this overstated it:**
  a count check catches one extra or one missing, not one of each, which is the
  shape a footnote heading plus a heading that never materialises would produce
  together. And the failure it guards is a **mis-scroll, not a wrong document** —
  weaker than the class `mpdf-001` §2 refuses, since no byte of the PDF is
  affected. The guard is worth its line for the common case and is not claimed to
  be exhaustive.

  **The app carries it and decides nothing.**
  `app/src/document.rs:render_with` calls the new function and
  `app/src/document.rs:Render` gains the anchors beside `pdf`;
  `app/src/preview.rs:Preview` holds them with the bytes it already holds; and
  `app/src/preview.rs:Status` — already `Serialize`, and already fetched by
  `app/dist/index.html:refresh` immediately before it draws — carries them to
  the page. **No new Tauri command**, because that fetch is already on the path
  that draws.

  The page's own work is inside `app/dist/index.html:draw`: the caret's line is
  `text.value.slice(0, text.selectionStart).split('\n').length`, the target is
  the last anchor whose `line` is at or below it, and the fragment goes on the
  **fresh blob URL `draw` already mints on every compile**. `#page=N` on a *new*
  blob URL is the operation Phase 2's round confirmed and OQ-6 records; nothing
  here reuses a URL, which is the case that does not work.

  **The pane follows only a redraw that did not replace the text, and this rule
  is the phase rather than a detail of it.** Round 1 found that without it the
  feature breaks two shipped gates on its first keystroke. `app/dist/index.html:
  refresh` assigns `text.value = await invoke('document_text')` whenever
  `state.reloaded !== takenReload`, and **assigning `value` moves a textarea's
  caret to the end of the control** — normative WHATWG behaviour, confirmed in a
  browser during that round. `refresh` then falls through to `draw` in the same
  pass, so the formula above would read the *last* line of the document and open
  `samples/article.md` on page 3. That contradicts Phase 1's shipped gate case
  (1), "Opening `samples/article.md` draws its first page", and §1's usage
  sketch; it fires again on Phase 2's external-change reload over a clean buffer
  and on Phase 5's Finder open.

  The signal is already on the page and needs nothing new: the pass that
  replaced the text is exactly the pass that must not follow. `refresh` captures
  whether it took a reload **before** it updates `takenReload`, and hands `draw`
  a target of `none` on that pass — so an open, an external reload and a Finder
  launch all draw page 1 exactly as they do today, and only a redraw following
  the author's own edit carries a fragment. **An open is not a cursor
  movement**, which is the whole of the reasoning.

  **The rule is "took no reload", not "the author typed", and round 2 found one
  case where those differ**: an external *figure* change recompiles without
  replacing the text, so it carries a fragment though no keystroke caused it.
  That is left as it falls rather than special-cased. The caret is still where
  the author put it, the page it names is still the page they were working on,
  and adding a second suppression would be guessing at an intent the app cannot
  see — which is the same reasoning that keeps the display arm position-blind in
  `mpdf-004`. The mechanism sentence is the specification; the slogan is prose.

  **Counting newlines is not parsing markdown**, and the distinction is the
  point rather than pedantry: the page owns no dialect knowledge and must not
  start owning any. It also happens to be the one quantity the two sides agree
  on, since `selectionStart` is a UTF-16 offset where a Rust line is derived
  from bytes.

  **What it costs, named here so the round can reject it rather than discover
  it.** The precision is the document's heading density: a caret above the first
  heading gets page 1, and a section that runs ten pages puts the author at its
  top and no closer. And it follows the *author*, not the reader — an author who
  scrolls the pane to page 5 while the caret sits on page 3 is returned to page
  3 by the next compile. OQ-6's staleness objection does not reach that, because
  the caret lives in the app's own pane, but it is a behaviour to judge in use
  rather than an obvious win.

  **The shape not taken (decision, recorded).** `typst-ide::jump_from_cursor`
  maps a cursor in Typst source to a page *and* a point, which is what an editor
  sync actually wants. It is rejected here on two counts, neither of them size
  alone. The caret is in markdown against a **generated** source, so it needs a
  map the emitter does not build — and building one is not "record two offsets
  per event": `core/src/emit.rs:step` writes into a buffer stack, and
  `core/src/emit.rs:prefixed` re-indents a list item's or a block quote's buffer
  when it closes, invalidating every output offset recorded inside it after the
  fact. And `typst-ide` is not in `Cargo.lock`, so it is a new dependency in a
  workspace that pins every one it has. **It stays available**: if heading
  granularity proves too coarse in use, that is the phase which follows, and
  what it needs is the source map rather than a different anchor.
- **Exit gate:** (1) A document whose headings sit on known lines produces
  anchors naming exactly those lines, in order, with non-decreasing pages — a
  `core` test, since this is where a wrong answer originates. (2)
  `samples/article.md`, which is three pages, produces an anchor whose page is
  greater than 1 for its last heading. **That is the case that proves the
  feature rather than the plumbing**: a gate met only by one-page documents
  would pass on an implementation that always answered 1. (3) A document with no
  headings produces no anchors, and one whose caret sits above the first heading
  resolves to page 1 — the two shapes the lookup has to handle without a special
  case in the page. (4) **The count guard returns no anchors**, tested at both
  levels. The zip is its own function over the two collections — the walked lines
  and the queried positions — so the mismatch needs no contrived document; and
  the mismatch **is** reachable from markdown, which a draft of this gate left as
  homework and round 1 answered: `[^1]: # A heading in a note` emits
  `#footnote[= A heading in a note]`, a heading `collect_definitions` walked into
  a discarded `Walk` and the document walk never counted. That document is the
  second half of this case, and it is a real one rather than a constructed one.
  (5) `md_to_pdf`
  returns what it returned before: `cargo test --workspace` passes, **no shipped
  golden file changes**, and `cli/src` is untouched, which is §2's falsifiable
  claim checked as a diff on the half of it this phase does not cross. (6) **The
  observable, by eye, because no test reaches it**: open `samples/article.md` in
  the app, put the caret in a section that falls on a later page, edit, and the
  pane redraws on that page instead of page 1. OQ-6 was resolved in use on this
  same sample, which is the precedent for judging this the same way. (7) **The
  three shipped paths that must still open on page 1**, which gate (6) cannot
  reach because it instructs the operator to move the caret first: opening
  `samples/article.md` draws its first page, an external save over a clean buffer
  redraws on page 1, and a Finder open of the same file draws its first page —
  the last of which **takes Phase 5's gate (2) precondition with it**, since the
  emitted `LSHandlerRank` is `Default` and any machine with an editor already
  registered for `.md` keeps that editor; set this app as the handler through
  Get Info → Open With → Change All first, or the case observes the wrong
  program. **Phase 1's gate case (1) is one of the three and it is named here
  rather than left to `cargo test`**, because the caret rule above is the only
  thing keeping it true and a regression in it is invisible on any one-page
  document.

  **This phase reads two cases by eye rather than §2's one, and takes the
  departure deliberately** — the precedent is Phase 5, which argued its own. (6)
  is the observable and (7) is the regression that no test reaches, and they are
  the same window in the same sitting: there is no JS harness, for the reason
  Phase 4 recorded, so a rule that must hold *in the page* has nowhere else to be
  checked. Every other case in this phase is an ordinary test.
- **Close-out:** `rules/desktop.md` gains the anchor path — the new `core` call,
  what `Status` now carries, and what `draw` does with it — and
  `rules/pipeline.md` gains the new export beside `md_to_pdf`. **Expect to raise
  both caps in the same pass**: `desktop.md` sits at 390 against 394 and
  `pipeline.md` at 338 against 340, so neither has room for a paragraph.

  **Two paragraphs of §2 get a dated `CORRECTED` note each**, per §6.1's remedy
  for shipped prose that is now misleading — a note beside the text, original
  kept, because this spec is `accepted` and append-only and rewriting either
  paragraph would rewrite history. The first is the falsifiable claim, "`core`
  gains nothing and changes nothing", if the round lets the crossing stand. The
  second is the one round 1 found and a draft of this close-out missed: §2's
  "**A re-render therefore returns the reader to the first page**", which is the
  measurement the pane's whole design rests on and which this phase makes false
  for the author's own edits. **The same sentence is repeated as a comment in
  `app/dist/index.html:draw`**, which the implementation touches anyway, so it is
  corrected there in the same pass. The README's app section gains one sentence.
  One push.

### Phase 7 — the page fits the pane it is given
*Produces the observable: **no** — the PDF is byte-for-byte the one Typst
already compiled. The argument for building it anyway is below.*

Appended 2026-08-25, after Phase 6 shipped, per §6.1 step 2: the preview pane is
this spec's subject, which is the same ground Phase 6 took and for the same
reason. Nothing here is about multi-file documents; a document of one file gets
every line of it.

**Why a phase that produces no observable.** §1 made the observable stop being a
file you open and start being *a page you watch*. A page drawn at whatever scale
WebKit last chose, half of it past the right edge of the pane, is not being
watched — it is the same artifact and not the same document to a reader. The
phase changes no byte of the PDF and changes whether the PDF can be read at the
width the reader gave it.

- **Scope:** The page is held at the width of the pane, through a change and
  after it.

  **The page opens fitted.** `view=FitH` is the PDF open parameter for
  fit-to-width and rides the fragment beside Phase 6's `page=N`, both read at
  load. A page opened at the author's own page and at a width they must then set
  by hand is half an answer.

  **WebKit reads it once, and that is the fact the rest follows from.**
  Measured in use on this machine: the view keeps the scale it computed at load
  and does not reflow when the frame around it resizes. No script reaches inside
  a native PDF view to ask it to — the pane's whole design, since Phase 1, is
  that the view is WebKit's and not this project's. OQ-9 carries what that
  costs beyond this phase.

  **So a width that is changing is answered by scaling the frame, not by drawing
  it again.** The frame keeps the pixel width its page was fitted to, and a
  transform maps it onto the width the pane now has: scale by `s` and the page
  inside is `s` times as wide, which is fit-to-width by construction rather than
  by arithmetic anyone has to trust. It composites, so it holds every frame of a
  drag. **The reader keeps their place, because nothing reloaded.** The frame is
  laid out `h / s` tall so that it is exactly `h` after scaling, or the page's
  viewport would leave a band under it.

  **And a width that has settled is answered by drawing it again.** A scaled
  page is a page rendered for a different width, and far enough from `1` the
  type visibly softens — reported in use, which is how the split between the two
  halves was found. A fresh object URL at the settled width is fitted by the
  loader and rendered sharp. It fires on `pointerup` and, through one 200ms
  timer, on a window resize: **a drag ends on an event and a window resize does
  not**, and the timer is what gives the second one an ending. Skipped when the
  width did not move, so a window resized by its height alone re-fits without
  redrawing.

  **A redraw lands on the caret's page, and this phase does not introduce that
  rule — Phase 6 did.** WebKit leaks nothing about where the reader was, so a
  fresh blob can only *set* a place, never restore one, and Phase 6 answered
  that with the author's caret. What is new is that a **reader** now meets it: a
  reader on page 12 who drags the divider is put back on the caret's page. The
  redraw is deliberately at rest rather than during the drag so that the gesture
  itself never moves them.

  **The pane's geometry is observed, not inferred from the events believed to
  change it.** A `ResizeObserver` on the preview column, not a list of
  listeners. This is stated as a rule because **two bugs in one session came from
  the other approach**, and both had the same shape — a control added to the
  window changed a geometry that something else had already measured. The
  divider's grab point read `clientX`, which is measured from the window, and
  matched the pane's own width right up until the section panel sat to the left
  of it. The scaled frame held an explicit pixel width and kept it while the
  panel and the line gutter each took width out of the column beside it. Neither
  was a bug in the code that broke; both were bugs in the assumption that the
  causes of a resize can be enumerated.

- **Rejected:** *redrawing on every pointermove.* It reloads the frame, which
  cannot be done sixty times a second and could not restore the reader's place
  if it could. *Not redrawing at all, and letting the next compile re-mint.* True
  that in an app which recompiles when typing stops the soft state is usually
  transient, and false for a reader who never types — measured, and the reason
  the settled redraw exists.

- **Exit gate:** **This phase's behaviour lives entirely in
  `app/dist/index.html`, which no test in this repository reaches**, and saying
  so is part of the gate rather than an excuse for not having one. The suite's
  contribution is that `cargo test --workspace` still passes and that the page's
  script parses. The behaviour itself is a named manual check, run on a master
  and on a single-file document: open a document and confirm the page arrives
  fitted; drag the divider and confirm the page stays fitted through the drag
  and sharpens when the hand comes off; toggle every control in the header and
  confirm the page follows each one; resize the window by its height alone and
  confirm the page does not jump. **The absence of a reachable test here is the
  phase's own finding**, and OQ-10 below is where it goes.

- **Close-out:** `rules/desktop.md` is at 497 of its declared 500 body lines and
  cannot hold this. The rule is split rather than uncapped — `rules/desktop.md`
  keeps the crate, the commands, the file I/O and the watch, and a new
  `rules/desktop-panes.md` takes the two panes and their geometry, with its own
  `sources` and `covers`. Raising the cap defers the same decision at a file
  that is already at the length its cap was chosen to prevent. The README's app
  section is untouched: nothing here changes what the app is for. One push.

### Phase 8 — the text pane shows its lines
*Produces the observable: **no** — nothing here reaches the PDF at all.*

Appended 2026-08-25, per §6.1 step 2: the text pane is this spec's subject and
Phase 4 is the phase that made it, whose "a plain textarea and nothing more"
this phase is careful not to overturn.

**Why a phase that produces no observable.** The app's errors name lines. A line
number the author cannot see is a message that names a place they then have to
count to. The CLI prints the line into a terminal that numbers nothing, but its
author has the file open in an editor that does; this pane *is* the editor, and
had no numbers of its own. (`web/index.html` is a third front end with the same
gap and no such excuse — out of scope here, and `mpdf-006`'s to close.)

- **Scope:** All of it is in **`app/dist/index.html`**. A `Lines` control in the
  header; off, the pane is exactly Phase 4's textarea; on, it gains a gutter
  beside the text and marks the caret's line **twice** — the row's number in the
  gutter, and a band behind the line itself.

  **The textarea stays the editor**, and that is the decision the rest depends
  on. A second editor would fork every seam that assumes a `<textarea>` — the
  buffer sent to `edit`, the reload that replaces `value`, `caretPage`'s
  `selectionStart`, the disabled state, and the divider's geometry — and this
  repository's tests reach none of them. The gutter is drawn beside the pane and
  changes nothing about it.

  **A row is as tall as its line renders, not one line-height.** A textarea
  soft-wraps and will not say where, so a gutter that gave every number one
  line-height would drift further out of true with every wrapped paragraph until
  the numbers meant nothing. The heights are measured off a hidden mirror: the
  same text, the same font, laid out at the same content width, one element per
  logical line. **What the browser did to the mirror is what it did to the
  textarea.** **An empty logical line is mirrored as a zero-width space**: an
  empty line still occupies one and an empty element does not, and markdown is
  mostly blank lines. Without it the error **accumulates** — the rows are a
  running sum over the measured heights, so every blank line above a row
  shifts it by one line more.

  Rebuilt when the text or the width changed and skipped when neither did.
  **The width-change rebuild rides `settle`'s 200 ms timer**, not a
  `pointermove` and not the `pointerup` itself — the measurement is a layout,
  and a layout per frame for numbers nobody reads mid-drag is waste. That hook
  is why `mpdf-009` Phase 1 keeps `settle` whole when it removes the rest of
  what that timer once served.

  **An emptied pane loses its band as well as its rows.** The path that clears
  the document rebuilds the gutter and returns early, and the two other things
  that move the mark both need a focused, enabled textarea — which a cleared
  pane is not. Round 1 found the shipped code leaving the previous document's
  band painted across an empty one until something opened; the early return
  clears the mark on its way out.

  **The gutter does not scroll; it follows.** It sits in a box that does not
  scroll itself and its `scrollTop` is driven from the textarea's, on the
  textarea's own `scroll` event and again whenever the rows are rebuilt.
  Without it the numbers part company with their lines the moment a document is
  taller than the pane, which is every document. **The band needs none of
  this** — `background-attachment: local` scrolls it with the text — and that
  contrast is the reason the two are built differently.

  **The caret's line is counted the way `caretPage` counts it**, off
  `selectionStart`. A second way of finding that line would be a second thing
  that can disagree with the page Phase 6 opens the frame on, and the
  disagreement would be invisible until it was not.

  **The band behind that line is the textarea's own background** — one colour,
  sized and placed from the measured row, with `background-attachment: local`,
  which is what scrolls it with the text instead of pinning it to the border
  box, and **`background-repeat: no-repeat`, without which the one-row gradient
  tiles down the whole pane**. Drawing it as an element would mean wrapping the
  textarea, making it transparent and layering over it — and the divider's
  geometry reads the textarea's own box, so the wrapper would move it.

- **Rejected:** *a code editor component.* CodeMirror 6 needs a bundler, which
  is what `withGlobalTauri` exists to avoid, and it would overturn Phase 4's
  recorded "no highlighting, no autocomplete, no formatting commands" for a
  dialect small enough that highlighting earns much less here than in LaTeX.

- **Exit gate:** **Every clause here is checked by eye, and that is a departure
  this phase argues rather than borrows.** §2's rule is that the app's logic
  lives where a test can reach it, and all of this lives in
  `app/dist/index.html`, which nothing in this repository reads —
  OQ-2's `withGlobalTauri` bought one Cargo build and no node toolchain, and
  this is the bill. OQ-10 carries the general problem. What the suite
  contributes is that `cargo test --workspace` still passes, no `.rs` file
  being touched.

  With `Lines` on, against `samples/showcase/showcase.md`:

  1. **Narrow the pane until a paragraph wraps**, then type into it: the
     numbers stay against their lines. The narrowing is the point — the
     showcase's longest line is 82 characters, so at a wide enough window
     nothing wraps and the clause would pass without testing anything.
  2. **Scroll the pane**: the numbers stay against their lines. Nothing else in
     this gate can see the gutter's scroll-follow, and a document taller than
     the pane is every document.
  3. **Drag the divider**, which rewraps the text: the numbers are against
     their lines when it rests, and the gutter is *not* rebuilt on every
     pointermove — the rows change once, not per frame.
  4. **A document with blank lines between paragraphs** numbers correctly to
     the last line, which is the zero-width-space rule and fails by one row per
     blank line without it.
  5. Move the caret by arrow, click and drag: **both** marks follow — the
     gutter's row and the band.
  6. Toggle `Lines` off: the pane is Phase 4's textarea again, band and all.
  7. **Close the document with `Lines` on**: the gutter empties and the band
     goes with it. Round 1's find, and it was a live defect rather than a
     hypothetical one.

- **Close-out:** **`rules/desktop-panes.md`** — **verified against the code and
  regenerated where it disagrees**, not written afresh: its `## The gutter`
  section and its `covers` already name this, the rule having been written from
  the same prototype. The verb matters because the code and the rule are both
  already on `main`, and "gains the gutter" would read as an instruction to
  duplicate a section that exists. Not `rules/desktop.md`, which no longer
  documents a pane at all and whose `covers` would not admit one; the draft of
  this close-out said otherwise and round 1 caught it.

  **Its own push, and not Phase 7's.** The draft said "one push, with Phase 7";
  `mpdf-009` is `accepted` and its Phase 1 cuts Phase 7 on shipping, and §7's
  gate forbids implementing a phase whose `cut` is set — so a joint push cannot
  happen, and nothing in this phase's reconciliation may depend on Phase 7's.

  `README.md`'s app section names the window's controls, so it gains the
  `Lines` toggle in one sentence — a visible control the reader will otherwise
  meet undocumented.

### Phase 9 — what checks the front end
*Produces the observable: **no**, and the argument is OQ-10's.* Nothing here
reaches the PDF. What it reaches is the file that decides where the reader
lands, how much of a document the pane holds, and what the page reads off Rust
— 752 lines of code inside a 2,566-line file, whose **only checking mechanism is
a person at a console**. Eight defects have shipped or nearly shipped in it;
seven were found by eye or by review, one of them the day this was written. A
phase that produces no observable is worth its place when it is the first thing
that can tell the observable has broken.

Appended 2026-08-27, per §6.1 step 2: OQ-10 raised this subject twice, from
Phase 7 and from Phase 8, and this spec owns it.

- **Scope:** **`app/dist/index.html`** (annotations only), **`app/tsconfig.json`**,
  **`app/types/pdfjs/`** (vendored declarations), a mirror script,
  **`app/src/preview.rs`** (a test function in the `#[cfg(test)] mod tests` it
  already carries, and nothing else),
  **`app/Cargo.toml`** (one dev-dependency), and a CI workflow. **No shipped
  behaviour changes: no `.rs` file's runtime path is touched and no line of the
  front end's code moves** — only comments, and a test module that does not
  exist outside `cfg(test)`.

  **The measurement comes first, because it chooses the settings.** Under the
  exact method below — TypeScript **5.9.3**, `app/tsconfig.json`, the vendored
  declarations — `tsc` over the shipped file reports **243 errors with
  `strictNullChecks` and `noImplicitAny` on, and 41 with them off**, and **not
  one of the 243 is a defect**. They are annotation gaps: 112 `getElementById`
  results that are nullable, 78 implicit `any` (35 parameters and 43 variables),
  and the rest the page's own expando properties. **A check whose first run
  reports 243 non-bugs is a check nobody runs**, which is why the settings are a
  decision with an argument rather than a default.

  **The version is pinned and the settings live in `app/tsconfig.json`, not in
  flags.** A bare `bunx tsc` resolves to whatever is current — 7.x today, whose
  modern defaults happen to give the same 243, while 5.9.3 given no
  `target`/`module`/`lib` gives 274 and different diagnostics. Everything else in
  this crate is pinned on a stated policy (`app/Cargo.toml`: "Every version is
  pinned"), and an unpinned checker makes the gate drift on someone else's
  release schedule.

  **So the loose settings, and the claim is that they lose nothing this project
  has been bitten by.** All three classes that have cost something still fail
  with both flags off, each verified on its own against the shipped file:

  | class | code | the real instance |
  |---|---|---|
  | block-scoped use before declaration | `TS2448` | `mpdf-009` Phase 3, caught by reading during the build |
  | a method the type does not have | `TS2339` | `doc?.destroy()` — `PDFDocumentProxy` has none, and optional chaining swallows it |
  | a field the page reads and Rust does not send | `TS2551`/`TS2339` | OQ-10's own sharpest edge |

  **The middle row is conditional, and the condition is part of the build.**
  `doc` is `let doc = null`, so with `strictNullChecks` off it infers `any` and
  `doc?.destroy()` raises nothing however good the declarations are — **measured,
  not assumed**. It fires only once `doc` and `loading` carry
  `@type {import('pdfjs-dist').PDFDocumentProxy | null}` and its loading-task
  counterpart, which costs two further annotation gaps to clear. An implementer
  who writes the annotations to satisfy the gate rather than the gate to check
  the annotations gets a check that passes for the wrong reason; the clause below
  is written to make that visible.

  **The declarations are vendored from `pdfjs-dist` 6.2.108 and are not
  hand-written.** A hand-written shim for a minified third party is the drift
  hazard this phase solves carefully for `Status` and would be reintroducing one
  file over — and it cannot work anyway: **TypeScript ignores `declare module`
  for a *relative* specifier**, so the `./pdfjs/pdf.min.mjs` import cannot be
  shimmed at all. The `types/` tree from the same tarball as the two vendored
  `.mjs` files, at the same pinned version, with the same Apache-2.0 LICENSE
  already travelling, is the faithful answer.

  **They live at `app/types/pdfjs/` and must not live under `app/dist/`.**
  `generate_context!` walks `frontendDist` recursively with no allowlist, so a
  directory placed there is embedded in the shipped binary — 824 KB of
  declarations no runtime reads. `app/dist/pdfjs/` continues to hold exactly the
  two `.mjs` files it holds today. **Only one of those two is imported as a
  module**; `pdf.worker.min.mjs` is reached through `new URL(…, import.meta.url)`,
  a string `tsc` never resolves.

  **The mirror does two things, and both are stated because a generator with an
  unstated rule is a generator nobody can reproduce.** It replaces every line
  outside `<script type="module">` with an empty one, and it rewrites the single
  relative `pdfjs` specifier to the bare name `pdfjs-dist`, which `paths`
  resolves and a relative specifier cannot be given types for. Both are
  line-preserving, so the mirror is the same length as the HTML and every error
  cites a real `app/dist/index.html` line. **Measured: 2,566 lines each, line
  1,737 identical.** It is generated outside `app/dist/` — inside it, `pdfjs/`
  resolves relatively and the count goes from 41 into the thousands — and never
  committed.

  **The file does not split, and the reason is narrower than the draft claimed.**
  Moving the script to `dist/app.js` would *not* make `frontendDist` a build
  output: `app/dist/pdfjs/` is already two committed static modules the page
  imports, and `rules/desktop.md` says so. The reason that survives is that **the
  mirror is required anyway** for the specifier rewrite, so a split buys nothing
  and costs the `sources` of `rules/desktop.md`, `rules/desktop-panes.md` and
  `rules/desktop-geometry.md`.

  **The annotations are the work.** The page wrapper's `logical`, `natural`,
  `view` and `number`; `gen` on a canvas; `__pane` and `__TAURI__` on `window`;
  the DOM subtypes `getElementById` cannot know; the nullable timers `tsc` infers
  as `null` from their initialiser; `doc` and `loading` as above; the two
  `AnnotationLayer` call sites, whose vendored declarations require fields
  pdf.js tolerates the absence of at runtime; one `new Promise()` that needs a
  JSDoc hint; and **`Status` and `Anchor`, which are the point**. They sit in one
  block at the top rather than scattered above functions that already carry a
  paragraph of argument — this file's comments are arguments and JSDoc is
  annotation, and interleaving them would cost the arguments their shape.

  **The Rust half is what stops the typedef being a lie**, and it is a test
  function in the `#[cfg(test)] mod tests` that `app/src/preview.rs` already
  carries — 45 tests run there today. It cannot be
  `app/tests/`: the `app` package declares only `[[bin]]`, so there is no lib
  target for an integration test to link, and `main.rs`'s `mod preview` is
  private — the `core/tests/page_examples_test.rs` precedent does not transfer,
  because `core` has a `[lib]`. A unit test in the binary's own module runs under
  `cargo test` and reaches `Status` directly. It serializes a `Status` holding
  one `Anchor` and compares the JSON keys of both against the field lists the
  typedefs declare, `include_str!`ing the front end the way that precedent does
  read a file. `serde_json` is a dev-dependency and already in `Cargo.lock` via
  `tauri`, so it adds no crate.

  **Where the type check runs, and where it must not.** **Not `cargo test`** —
  bun is not a prerequisite of this workspace, and making the Rust suite depend
  on one would charge OQ-2's price to everyone who builds the app in order to
  check a file the suite does not otherwise touch. It runs in CI, in a second
  workflow beside `pages.yml`, which builds only `web/` and must stay that way.
  **The Rust half does run in `cargo test`**, costing nothing.

  **What this does not reach, named here rather than discovered later.** Of the
  eight defects this file has produced, a type check catches **one**. The seven
  it misses are `ResizeObserver` feedback through a box the callback resized, an
  `IntersectionObserver`'s delivery racing an animation frame, a forced pass
  dropped rather than re-armed, a stale `deliveryMs`, a settle timer cancelling
  the render that was about to set the fit, a counter advanced for a render that
  never drew, and an early return leaving a caret band on an empty pane. Every
  one is behaviour, and no type system sees any of them. **This phase does not
  answer OQ-10's wide half and must not be read as answering it.**

- **Exit gate:**

  1. `tsc` over the mirror reports **zero** errors, at the pinned version and the
     committed `app/tsconfig.json`.
  2. **Each of the three classes fails on purpose, one at a time.** A read of
     `fitMode` above its declaration; a `.destroy()` on the document proxy; a
     `state.sectons`. Each is reported, at an `app/dist/index.html` line number
     that is the real one. **A check that has never failed is not known to
     work** — and the middle one is the clause that catches an implementer who
     annotated `doc` as `any` to reach clause 1, because it cannot fire if they
     did.
  3. The mirror is the same line count as the HTML, and a line chosen from the
     middle of the prose reads identically in both.
  4. `cargo test --workspace` passes, and **the new test fails when a field is
     renamed in Rust and the typedef is not** — demonstrated for `Status` *and*
     for `Anchor`, which needs a `Status` holding one to reach at all.
  5. **No behaviour changed**: `mpdf-009` Phase 3's gate re-runs to the same
     numbers, and `app/dist/` still holds exactly the files it holds today.
  6. CI runs clause 1 on a push touching `app/dist/index.html`, `app/tsconfig.json`
     or `app/types/**`, and does not run the wasm build.

- **Close-out:** **`rules/desktop.md`** — its `## The crate` section states the
  committed file count and the exact dependency list, and this phase moves both;
  it also declares `app/dist/index.html` among its sources.
  **`rules/desktop-panes.md`** gains what checks this file and what that check
  does not reach — it is the rule whose subject is "the one file the front end
  is". `rules/desktop-geometry.md` declares the same source but documents only
  geometry, which does not move: **named and excused rather than silently
  skipped**. **OQ-10 takes a dated note recording its narrow half answered and
  its wide half open**, rather than a strike-through: the geometry, the fit, the
  observer lifecycles and the reader's place are all still checked by a person,
  which is what the question asked about.

  **`README.md`: none needed** — its app section documents behaviours a reader
  meets, and this changes none. **`CLAUDE.md`: none needed.** One push.

### Phase 10 — the editor is named Letur
*Produces the observable: **no**, and the argument is that this phase changes
what the app is called and nothing about what it makes.* The PDF is byte-identical
across it and the gate below says so. What it buys is a name for the thing that is
not the converter. **`md2pdf` names an engine** — the library, the command, the
dialect, this repository — and it has been serving double duty as the name of a
window that is an editor. The two have become separable products: one is a command
you run over a file and are done with, the other is an application you keep open
all day. A phase that produces no observable earns its place here because **the
name is the one part of an application its user says out loud**, and because
nothing else in this corpus would record why the window stopped sharing the
engine's.

Appended 2026-08-29, per §6.1 step 2: the app's identity is this spec's subject,
and its packaging has been since Phase 5.

There is a second reason to do it now rather than later. **After an open, the
window title is the document's own file name** (`app/src/document.rs:title`), which
is right — the author is editing `method.md`, not "the app" — but it means the
product name is on screen only until the first `⌘O` and nowhere afterwards. **A
footer bar carrying the name is the intended next phase and is not yet written**;
this one is its prerequisite either way, because a brand cell must not say a name
the bundle does not carry.

- **Scope:** **`app/tauri.conf.json`** (`productName`, `identifier`, the window's
  launch `title`), **`app/Cargo.toml`** (`[package] name`, `[[bin]] name`,
  `description`), **`app/src/main.rs`** (the module doc, and the submenu title in
  `app/src/main.rs:menu`), **`app/src/document.rs`** (one string:
  `app/src/document.rs:scratch_dir`'s temp-directory prefix, which is inside
  `#[cfg(test)]`), **`app/dist/index.html`** (the `<title>`, and nothing else in
  that file), **`README.md`**, and **`rules/desktop.md`** and
  **`rules/desktop-project.md`**. `Cargo.lock` changes too and is not edited by
  hand: cargo rewrites the `md2pdf-app` package entry on the first build.

  **No logic changes.** Every `.rs` edit is a string — a doc comment, a menu
  label, a test's scratch-directory prefix — and no function's body moves. That is
  a narrower claim than "no behaviour changes", which would be false; see the
  store, below.

  **`md2pdf` stays the engine's name, everywhere it already is one.** The `core`
  and `cli` packages, the `md2pdf` binary the CLI installs, every `md2pdf_core::`
  path in the app's own source, the dialect, and this repository keep it. The
  rename is exactly one package wide. **The CLI is the deliverable most of this
  corpus is about** and renaming it would rename the observable's producer, which
  nobody asked for and which every spec here cites by name.

  **The crate's directory stays `app/`.** Three `rules/` files declare their
  sources by path under it — `app/dist/index.html`, `app/src/preview.rs`,
  `app/Cargo.toml` — and four specs and their review records cite dozens more.
  Moving the directory would rewrite all of them to say nothing new, and
  `spec-lint`'s citation check would be the thing that found each one. **The
  package is named `letur`; the folder it lives in is just where the app is.**

  **The identifier becomes `dev.letur.desktop`, and the shape is a decision.**
  `dev.letur.app` was the obvious reading of the name and is refused: `tauri-cli`
  2.10.1 — the version this crate pins — warns *"The bundle identifier … ends with
  `.app`. This is not recommended because it conflicts with the application bundle
  extension on macOS"*, verbatim, on every bundle build. It is a `log::warn!` and
  would not fail the build, which is exactly why it would become a warning nobody
  reads. `.desktop` is what `dev.md2pdf.desktop` already used, so the convention is
  inherited rather than invented.

  **The identifier move has three costs, and only the first is cosmetic.**

  The first is LaunchServices: macOS keys the `.md` document association off the
  identifier — `bundle.fileAssociations` in the same file is otherwise untouched —
  and **keeps the old identifier registered for as long as an old bundle exists on
  disk**, so an author who installed the app before this phase sees both in "Open
  With" until they delete it.

  The second is the store. **`app/src/main.rs` resolves the app's data directory
  through `app.path().app_data_dir()`, which Tauri names from the identifier**, and
  `app/src/document.rs:store_file` puts `projects.json` in it. So
  `~/Library/Application Support/dev.md2pdf.desktop/projects.json` becomes
  `…/dev.letur.desktop/projects.json`, and **every project's remembered main file
  is orphaned** — `mpdf-010` Phase 1's one remembered fact — its scope's
  clause 4 — for every root the author has set one on.

  **It is not migrated, and that is the decision rather than an oversight.** What
  is lost is one string per project, and the app is not broken by losing it: with
  nothing remembered it falls back to the discovery rule `mpdf-010` §2 already
  specifies, which is the state a first launch is in — a case `read_store` is
  already written to treat as ordinary. A one-shot migration path is permanent code
  answering a one-time event on the machines of an app at version 0.1.0 that has
  never been distributed. The author re-picks a main in each project they had set
  one in, and **the old directory is left in place rather than deleted**, so
  anything it holds is recoverable by hand.

  **The third cost is consent, and this project has already been bitten by it
  once.** macOS keys TCC grants to the bundle identifier, so **`dev.letur.desktop`
  is a stranger to every grant `dev.md2pdf.desktop` was given**.
  `rules/desktop.md`'s `## The bundle` records what that costs here:
  `app/src/watch.rs:start` watches a whole directory recursively, so a document
  under `~/Documents`, `~/Desktop` or `~/Downloads` **can compile once through the
  open panel and then stop redrawing, silently** — the app looks like it is
  working and is not. Phase 5's gate case (2) third observation was written for
  exactly this and states its precondition as "the first launch of this identity",
  which **this phase re-creates for every installed copy**. That is not an
  argument for keeping the old identifier: consent is a prompt the author answers
  once, against a name they chose. It is an argument for re-running that
  observation, which clause 7 does — and it is named here because a redraw that
  stops is the one failure in this phase that would not look like one.

  **The binary is renamed with its package**, so the bundle becomes
  `Letur.app/Contents/MacOS/letur`. Phase 5 gated on `Contents/MacOS/md2pdf-app`
  inside `md2pdf.app` and made a point of the two names differing; that clause is
  now `letur` inside `Letur.app` and the gate below re-runs it under the new names.
  **Phase 5's text is not edited** — it is the record of what shipped then, per
  §6.1's first further rule.

  **Case is a decision rather than a typo.** `Letur` — capitalised — is the
  product: `productName`, the launch title, the README. `letur` is the package, the
  binary, and the identifier's last component but one. Finder shows the first;
  `cargo` and `otool` show the second.

  **The application menu's own title is not this phase's to set.** macOS takes it
  from the bundle, so the `SubmenuBuilder::new(app, "md2pdf")` string in
  `app/src/main.rs:menu` is not what the menu displays; it is renamed for the
  source's own consistency, and the gate below reads the menu off a built bundle
  rather than crediting that edit for it.

  **The name means something, and the meaning is the reason it was chosen.**
  Icelandic *letur* is type, typeface, lettering — what the engine turns the
  author's markdown into. It was checked for collisions before adoption: free on
  crates.io, npm and PyPI, no GitHub repository of the name, nothing of the name
  on the App Store, and `letur.app`, `letur.dev` and `letur.io` all unregistered.
  The known collisions are a village in Albacete and a Spanish food brand named
  after it, neither in software.

- **Exit gate:**

  1. `cargo test --workspace` passes, with the same test count as before the phase.
     **The PDF does not move**: `core`'s byte-comparison tests and the app's own
     export test are in that run and are the check.
  2. `cargo build --release -p letur` succeeds, and **the engine is untouched**:
     `cargo run -p md2pdf-cli -- samples/article.md` still writes
     `samples/article.pdf`, from a binary still called `md2pdf`.
  3. `cargo tauri build` produces `target/release/bundle/macos/Letur.app`, and
     `otool -L Letur.app/Contents/MacOS/letur` names only `/usr/lib` and
     `/System/Library`, which is Phase 5's clause re-run under the new names.
     **The build logs no bundle-identifier warning.**
  4. `plutil -p Letur.app/Contents/Info.plist` reports `CFBundleIdentifier` of
     `dev.letur.desktop`, `CFBundleName` of `Letur`, `CFBundleExecutable` of
     `letur`, and a `CFBundleDocumentTypes` entry for `md` with
     `net.daringfireball.markdown` — **the association survived the identifier
     change**, which nothing else in the build would report.
  5. **Read from that built bundle**, not from `cargo tauri dev`: a launch with no
     document titles the window `Letur` and the application menu reads `Letur`;
     after `⌘O` the title is the document's file name, unchanged from today.
  6. **The store relocates and the app survives it.** Opening a project whose main
     was remembered under the old identifier compiles the file the discovery rule
     picks, with no error in the window;
     `~/Library/Application Support/dev.letur.desktop/projects.json` appears once a
     main is set again, and the `dev.md2pdf.desktop` directory is still there,
     untouched.
  7. **Consent is re-granted, because the identity is new.** Phase 5's gate case
     (2) third observation, re-run under `dev.letur.desktop`: a document under
     `~/Documents`, edited and saved in another editor, redraws the page. Its
     precondition is met by construction — this *is* the first launch of this
     identity — and **a page that draws once through the open panel and then stops
     is the negative this clause exists to catch**, not a pass.
  8. `grep -rn "md2pdf-app\|md2pdf\.app" --include="*.rs" --include="*.toml"
     --include="*.json" --include="*.yml" --include="*.md" --include="Cargo.lock" .`
     — **both spellings, the hyphen and the dot, and `Cargo.lock` named explicitly
     because none of the other patterns match it** — returns hits in `specs/` only,
     which is history and does not move, and none in `rules/`, `README.md`, `app/`
     or `Cargo.lock`.

- **Close-out:** **`rules/desktop.md`** — it names `md2pdf-app` in its opening
  sentence, in `## The export` and in `## The bundle` (where it states the bundle
  is `md2pdf.app` and `CFBundleExecutable` is `md2pdf-app`), and it declares
  `app/Cargo.toml` among its sources, so `/sync-rules` reaches all of it — **except one literal**:
  `## The bundle` says `Contents/Resources/` holds `md2pdf.icns`, which the bundler
  derives from `productName` and which therefore becomes `Letur.icns`. No declared
  source states the bundler's naming rule, so that word is corrected by hand beside
  the regeneration, the same class of miss as `desktop-project.md` below. **`##
  The document association` is the section to read while doing this**: it is where
  the identifier's effect on `CFBundleDocumentTypes` is written down.

  **`rules/desktop-project.md` is corrected by hand, and cannot be regenerated
  into truth.** It states that `projects.json` lives under the directory Tauri's
  resolver gives `dev.md2pdf.desktop`, which this phase makes false — but its
  declared sources are `app/src/document.rs` and `app/src/preview.rs`, and the
  identifier string is in neither, so `/sync-rules` would leave the stale name
  standing. Correcting it is part of the phase, not of the tool.

  **`rules/desktop-panes.md` and `rules/desktop-geometry.md`: none needed** — both
  declare sources under `app/`, but neither names the package, the product, the
  bundle or the front end's `<title>`: verified by grep, and named and excused
  rather than silently skipped.

  **`README.md`**: two sections, not one. `## Install` states the bundle path
  `target/release/bundle/macos/md2pdf.app`, and `## The desktop app` carries the
  name and the `cargo run --release -p …` invocation. **`CLAUDE.md`: none needed**
  — its stanza names the observable and the `mpdf-` id prefix, and this phase
  changes neither; the repository and the engine keep their name. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-003.md, append-only, one heading per round. See §7 of the
methodology.
-->
