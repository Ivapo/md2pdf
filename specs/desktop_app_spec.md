---
id: mpdf-003
title: desktop-app
note: >
  A macOS desktop app that shows the PDF while you write: a Tauri window wraps
  the same core crate, watches the document and its images, and re-renders.
status: draft
last_updated: 2026-08-10

phases:
  - name: "Phase 1 — the window, and one compile on screen"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — the watch loop"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — export, and the state the loop needs to show"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — the text pane"
    reviewed: null
    shipped: null
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

The end state for the early phases:

```console
$ cargo tauri dev            # a window opens
```

Open `samples/article.md` in the window. Keep editing it in your own editor.
Every save redraws the page:

```
┌────────────────────────────────────────┐
│ samples/article.md            ⟳ 41 ms  │
├────────────────────────────────────────┤
│          A Sample Article              │
│               Iva Po                   │
│                                        │
│  Introduction                          │
│  This file exists so you can see…      │
└────────────────────────────────────────┘
```

Phase 4 adds a text pane on the left, and the app becomes the editor too. That
phase is last because it is the only one the project can do without.

The engine does not change. `md2pdf-core` already takes a markdown string and
returns PDF bytes, and `mpdf-001` §2 recorded the claim this spec is the test
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
would need, `mpdf-001` §2's "not a rewrite" was wrong, and the review round
should say so rather than let the edit through quietly.

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
that `mpdf-001`'s OQ-6 put on `table.header` is present. The sharpest reason is
the last: `typst-pdf` is a separate crate from the layout engine, so a pane
drawn from frames would show that the *layout* is right, which is not the claim
this project makes. The observable is the PDF.

The constraint that rides with it: **no bundled JavaScript PDF viewer if it can
be avoided.** This project has no dependency of that kind and should not gain
one for a preview pane. On macOS a Tauri window is a WKWebView, the same engine
that draws a PDF when one is opened in Safari, so the native route may cost
nothing at all. OQ-1 carries the mechanism and the fallback.

### Why the watch set is the document plus every image it names (decision, recorded)

A document that changes is not the only thing that should redraw the page. A
figure edited in another program should redraw it too, and `mpdf-002` already
supplies the list: `md2pdf_core::image_paths` returns every path the document
names, in reader order. Emission reads no image bytes, so that list is
available even when the document names a file that does not exist yet, which
is what makes the watch set computable from a document that will not compile.

### Why a change is a whole re-compile (decision, recorded)

No incremental machinery, no caching, no partial re-layout. Measured on this
machine on 2026-08-10, the release binary converts `samples/press-release.md`
in 0.010 s and `samples/article.md` in 0.042 s, both including process startup,
which the app does not pay. A re-render per save is affordable by two orders of
magnitude, and an incremental path would be a large amount of state to hold
wrong. If a document is ever slow enough to need one, that is a measurement and
a later phase, not a guess made now.

### Why the last good page survives an error (decision, recorded)

An author mid-edit passes through broken states constantly — a half-typed
table, a fence not yet closed. Blanking the pane on each one loses their place
and makes the loop worse than the CLI it replaces. So a failed compile leaves
the last page drawn and shows the error above it, in the same words the CLI
prints: the construct and its line, or the frontmatter key and its line. The
pane also marks itself stale, because a page that silently belongs to older
text would be the flattening `mpdf-001` §2 refuses, one layer up.

### Why the app's logic lives in plain functions (decision, recorded)

Everything that can be decided without a window — the watch set, the debounce,
the asset read, the error string, the stale flag — lives in ordinary Rust
functions with ordinary tests. Only the parts that genuinely need a window go
through Tauri's command layer. A GUI whose logic is reachable only by clicking
has no exit gate but a screenshot, and this project's gates are tests.

## 3. Open questions

- **OQ-1** — how does the pane draw a compiled PDF, and can it be done with no
  bundled viewer? On macOS the window is a WKWebView, which renders PDFs
  natively, but the mechanics are unverified: whether bytes can be handed over
  through a custom protocol, a `blob:` URL or a temporary file; whether an
  `<embed>` or `<iframe>` draws it or only a full-frame navigation does; and
  what the app can control about the viewer's own chrome, zoom and scroll
  position. If no route holds, the fallback is Typst's SVG export per page,
  and §2's decision is amended to record why the artifact could not be shown
  directly. Answerable from code and a probe during review. Blocks Phase 1's
  gate case (1), and OQ-3 depends on the answer.
- **OQ-2** — which Tauri version, and what does the minimal app look like in
  code — the crate layout, the configuration file, the command boundary, and
  what the workspace must carry for `cargo tauri dev` to open a window?
  `mpdf-001`'s OQ-1 is the precedent: the same question about the Typst crates
  was answered by reading them during review. Answerable from code during
  review. Blocks Phase 1.
- **OQ-3** — what holds the scroll position across a re-render? A watch loop
  that returns the reader to page 1 on every save is unusable for a document
  longer than a page, and it is the difference between this app and a shell
  loop that reopens a viewer. The mechanism depends on OQ-1's answer, and the
  honest floor may be that the pane restores an offset rather than a semantic
  position. Design call, with the mechanism answerable from code. Blocks
  Phase 2's gate case (2).
- **OQ-4** — can the watcher follow a path that does not exist yet? A document
  may name `figures/new.svg` minutes before that file is created, and the
  watch set includes it because `image_paths` reads the text rather than the
  disk. The usual answer is to watch the parent directory instead of the file,
  which changes what a change event means and how much the loop filters.
  Answerable from code (the file-watching crate the phase picks). Blocks
  Phase 2's scope.
- **OQ-5** — what does the editor phase do when the file changes on disk while
  the text pane holds unsaved edits? Every editor answers this and none of the
  answers is free: refuse the reload, take the disk copy, or ask. It does not
  block anything until Phase 4, and stating it here is what stops Phase 4 being
  designed as if the question were not there. Design call. Blocks Phase 4.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Phases 1 to 3 build a
previewer, which is usable on its own. Phase 4 makes it an editor, and it is
the one phase this spec could lose without invalidating the rest.

### Phase 1 — the window, and one compile on screen
*Produces the observable: yes — the typeset PDF, on screen, from a document
the user picked.*

- **Scope:** Add the `app` crate to the workspace. A Tauri window per OQ-2,
  with an Open command that takes one `.md` file. On open: read the file, read
  its assets the way `cli/src/main.rs:read_assets` does, call
  `md2pdf_core::md_to_pdf`, and draw the result per OQ-1's resolution. A failed
  compile draws the error string — `Error`'s own `Display`, the same text the
  CLI prints after its `error: ` prefix — and nothing else, because there is no
  previous page to keep yet. The window titles the open document. No watching,
  no editing, no export.
- **Exit gate:** (1) Opening `samples/article.md` draws its first page, and
  opening `samples/press-release.md` draws its first page in the second look; a
  screenshot of each is read by eye once, which is what `mpdf-001`'s Phases 6,
  7 and 9 did for a claim no test can hold. (2) Opening
  `tests/fixtures/unsupported_html.md` draws the message naming the construct
  and line 5, and the app stays open — the same rejection the CLI makes, at a
  third level. (3) Unit tests over the plain functions: the asset reader
  returns one `Asset` per distinct path for `tests/fixtures/figure.md`, and
  reports the missing second file by path and line. (4) `cargo test
  --workspace` still passes and `core` and `cli` are untouched, which is §2's
  falsifiable claim checked as a diff.
- **Close-out:** Seed `rules/desktop.md` with all five frontmatter keys the
  methodology's §8.1 requires, `sources` naming the new crate's files. The
  README gains a section for the app. One push.

### Phase 2 — the watch loop
*Produces the observable: yes — the same PDF, redrawn on every save, which is
the whole point of the app.*

- **Scope:** Watch the document and every path `md2pdf_core::image_paths`
  returns, per §2, with OQ-4's answer deciding whether the watcher takes the
  files or their directories. Recompute the watch set after every successful
  parse, because an edit may add or drop an image. Debounce, because one save
  arrives as several filesystem events; the interval is a constant with its
  measurement beside it, not a guess. On each settled change: re-read,
  re-compile, redraw. A failed compile keeps the last good page and shows the
  error above it, and marks the pane stale, per §2.
- **Exit gate:** (1) Editing `samples/article.md` and saving redraws the page
  without any action in the window; verified by hand once and recorded. (2) The
  scroll position survives a re-render, per OQ-3's resolution. (3) Unit tests
  over the plain functions: the watch set for `tests/fixtures/figure.md` holds
  the document and both image paths; two events inside the debounce window
  produce one compile and two outside produce two; a compile error leaves the
  previous page as the current one and sets the stale flag. (4) Replacing an
  image file on disk, with the document untouched, redraws the page.
- **Close-out:** Update `rules/desktop.md` against the code. One push.

### Phase 3 — export, and the state the loop needs to show
*Produces the observable: yes — the PDF written to a file the user names,
which is the artifact the CLI writes.*

- **Scope:** A Save-a-copy command that writes the current PDF bytes where the
  user asks, defaulting to the document's path with a `.pdf` extension, which
  is `cli/src/main.rs:default_output`'s rule. The window shows what the loop
  knows: the open document, whether the page is current or stale, the compile
  time, and the error when there is one. Nothing here re-compiles — export
  writes the bytes the pane is already showing, so the file and the page cannot
  disagree.
- **Exit gate:** (1) Export writes a file starting with the `%PDF` magic bytes,
  byte-identical to what `md2pdf <the same document>` writes — the two wrappers
  agree, which is the claim that keeps the app honest. (2) Export is refused,
  with a message, while the pane is stale. (3) Unit tests over the default
  output path and the state machine that decides current, stale and failed.
- **Close-out:** Update `rules/desktop.md` and the README against the code.
  One push.

### Phase 4 — the text pane
*Produces the observable: yes — the same PDF, redrawn as the author types
rather than as they save.*

- **Scope:** A text pane beside the preview. It holds the document's text,
  writes it to disk on save, and re-renders on a debounce as it changes. The
  external-change rule follows OQ-5's resolution. The pane is a plain text
  editor, not a rich one: no syntax highlighting, no autocomplete, no
  formatting commands. Those are a later phase or a later spec, and naming
  them here is what stops this one growing into an editor project.
- **Exit gate:** (1) Typing in the pane redraws the preview without a save; the
  debounce holds it to one compile per pause. (2) Save writes the file, and the
  watch loop does not then compile the same text a second time. (3) The
  external-change rule behaves as OQ-5 resolved, tested at the plain-function
  level. (4) A document opened, edited, saved and reopened round-trips byte for
  byte.
- **Close-out:** Update `rules/desktop.md` and the README against the code.
  One push.

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
