---
id: mpdf-003
title: desktop-app
note: >
  A macOS desktop app that shows the PDF while you write: a Tauri window wraps
  the same core crate, watches the document and its images, and re-renders.
status: accepted
last_updated: 2026-08-10

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
    reviewed: null
    shipped: null
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
- **Not the browser build.** `mpdf-001` §1.1 parks a `wasm32` build, and it
  stays parked. It is a different front end with a different file story, so it
  is a later spec rather than a phase here.
- Out of scope, parked: multi-file projects and manifests, a document library
  or recent-files list, printing, PDF export options, theming, collaboration,
  and anything agentic.

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

- **OQ-7** — should the preview follow the author's cursor, mapping the line
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
  phase to append if the answer is yes.

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

- **Scope:** A macOS bundle: an `.app` with its icon, its identifier and its
  document association for `.md`, and a `.dmg` to carry it. Signing and
  notarisation if the phase can be run with credentials to hand; if it cannot,
  the phase ships the unsigned bundle and names the gap rather than pretending
  to close it.
- **Exit gate:** (1) The built `.app` launches on a machine that has no Rust
  toolchain and no fonts installed, opens `samples/article.md`, and draws the
  page — which is also the last check that the bundled fonts really are
  bundled. (2) Opening a `.md` file from Finder launches the app on that
  document. (3) The build is one documented command.
- **Close-out:** Update `rules/desktop.md` and the README's install section
  against the code. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-003.md, append-only, one heading per round. See §7 of the
methodology.
-->
