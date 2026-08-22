---
title: web-demo
sources:
  - web/index.html
  - web/src/lib.rs
  - .github/workflows/pages.yml
covers: >
  the published browser demo: the one page and what it costs, the two exports
  that cross into WebAssembly, the list of what the dialect adds and the marked
  examples that carry it, the byte rule those examples obey and the test that
  enforces it, the CSS that renders a script element, the height model and the
  panes beneath the list, the image limit the page still has, and the build and
  deploy that publish it
max_lines: 120
generated: 2026-08-21
---

# Web demo

The project's front door: `md2pdf-core` compiled to `wasm32-unknown-unknown` and called
from one page, published to GitHub Pages at `https://ivapo.github.io/md2pdf/`. It is the
third front end beside the CLI and the desktop app, and it shares `core`'s API with them
and nothing else. `mpdf-006` owns the directory; `mpdf-003` §1.1 named this a later spec
rather than a phase of its own, and it is that spec.

**The PDF a visitor sees is the PDF the CLI writes.** `web/src/lib.rs:render` calls
`md2pdf_core::md_to_pdf` and maps a failure through `JsError` over the error's own
`Display`, which is the same sentence `cli/src/main.rs` prints after its `error: ` prefix.
A construct outside the dialect is refused here in the words it is refused at the terminal.
`web/src/lib.rs:anchors` is the second export, `mpdf-003` Phase 6's `line:page` pairs
answered in a browser and reported as text, since a page has no caret to follow.
`web/src/lib.rs:start` routes a Rust panic to the console.

## The page

One page, not a landing page plus a player. The text is first in the document and renders
without the module; `<script type="module">` is async by definition, so the module lands
while the reader reads. Two URLs would turn the click that settles the argument into a
navigation that re-pays the download.

**The module is the whole cost and it is large.** Measured 2026-08-15: 25.7 MB raw,
7.8 MB brotli, of which `core/assets/fonts` is 2.5 MB. Nothing in the design is keyed to
either figure; they size the problem. `web/Cargo.toml`'s release profile is tuned for size
over speed — `opt-level = "s"`, `lto`, one codegen unit, `panic = "abort"`.

`#status` is the spike's instrument panel and still reports the boot beside each compile —
the elapsed time, the PDF's size, the anchors, and how the module crossed the wire, read
from `performance.getEntriesByType` rather than asserted. A compile that fails puts the
error there and **leaves the last good page in the pane**, as the desktop app does, because
an author mid-edit passes through broken states constantly. Two `<noscript>` blocks hold
the page honest with scripting off: one hides the status line, which would otherwise read
`booting…` forever above the argument, and one says the panes need JavaScript.

The panes sit beneath the list, so the page scrolls and `main` takes a slice of the
viewport — `clamp(360px, 70svh, 720px)`, in `svh` rather than `dvh` so browser chrome
hiding mid-scroll does not resize the frame and lose the reader's place in the PDF. Below
720px the two panes stack and take the full viewport instead.

## What the page claims, and the test that holds it to the compiler

The page carries three groups and **ten examples**: syntax an ordinary renderer passes
through as text (a caption over a table, a caption over a listing, a `:::` group, a
`{#name}` and the `[](#name)` that points at it, display math), things markdown has no way
to say (the six frontmatter keys, a footnote), and three refusals — raw HTML, a task list,
and a LaTeX command off the accepted list. Twenty-two constructs are supported, so the page
is a chosen few and links out to the README for the rest.

**Each example is one element, and two consumers read it.** A
`<script type="text/markdown" data-example="…" data-expect="ok|error">` holds the source;
the page will read it through `querySelectorAll` and `core/tests/page_examples_test.rs`
reads it through `include_str!`. A `<script>` element holds raw text, so markdown inside one
needs no escaping and a block of a non-JavaScript type is never executed.

**The content is load-bearing bytes: flush left, no leading and no trailing newline.** This
is not tidiness. Measured at two spaces of indent the frontmatter example stops being
frontmatter and reaches the page as a setext heading over prose, and the caption example
keeps its table but emits `: The measurements.` as literal text — and `md_to_pdf` returns
`Ok` for both. A leading newline is the same hazard one line on, moving every refusal's
`at line N` by one. The rule needs no stripping step in either consumer, so the two cannot
drift apart by normalising differently.

`core/tests/page_examples_test.rs` asserts that rule, that the count is exactly ten, that
every example is marked `ok` or `error`, that names are unique and that the message elements
match the refusals; then that each `ok` example compiles and each `error` example's
`to_string()` equals its row's visible `<code data-error-for="…">` text, character for
character. **The checked sentence is the one the reader sees** — an attribute copy would
prove agreement between the compiler and a string nobody reads. The `<code>` scan rests on
something weaker than the `<script>` scan, since parsed markup equals its raw slice only
while the sentence needs no character reference, so a separate assertion refuses a message
carrying `<`, `&` or a newline.

**That test is a workspace test over a file outside the workspace, and that is the point.**
`web/Cargo.toml`'s empty `[workspace]` table detaches the directory, so `cargo build
--workspace` and `cargo test --workspace` behave as they did before it existed. Reading a
file is not membership: `include_str!` compiles the bytes in and `web/` stays out of the
build graph.

**A `<script>` is `display: none` in every UA stylesheet**, and the source column of every
row is one. `script[data-example] { display: block; white-space: pre; overflow-x: auto }`
is what renders it. A visible `<pre>` duplicate beside the element was refused deliberately:
two copies of an example can differ, which is the failure the whole arrangement prevents.
The comparison column beside it is **written by hand, never rendered** — a second markdown
renderer on the one page claiming a single module does the work would be an unaudited
dependency, and no implementation is entitled to be called *ordinary*.

## What the page cannot do

**No image assets cross the boundary.** `web/src/lib.rs:render` passes `&[]`, so an
`![…](path)` a visitor types reaches `core/src/lib.rs:collect` and comes back
`Error::MissingImage`. A browser has no filesystem, so `core/src/lib.rs:image_paths`'
shopping list has nowhere to be read from. The caption examples use a table and a code
block for that reason, which take the same mechanism and the same counter behaviour; only
the image itself goes unshown. The page is not an editor and never gains accounts,
persistence or a share link — `mpdf-001` §1.1 refuses servers permanently, and Pages serves
static files.

## The build and the deploy

`.github/workflows/pages.yml` builds `web/` alone with `wasm-pack build --target web
--release`, on a push to `main` touching `web/**`, `core/**` or the workflow. It assembles
`_site` from **`web/index.html` and `web/pkg/` only**, so anything the page needs must be
inline in that file or added to that step. `wasm-pack`'s own `.gitignore` inside `pkg/` is
deleted before upload, or the artifact would skip the module.

**The module is never committed.** It is ten times the size of this repository and git
history is permanent, so it is built in the job and handed straight to the Pages artifact.
The workspace is not touched by this workflow and `web/` is not one of its members, so
nothing here can change what a phase's exit gate runs — and nothing here runs that gate
either: `cargo test --workspace` is a local check before a push, not part of the publish
path.
