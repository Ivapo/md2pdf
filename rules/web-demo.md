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
  enforces it, the CSS that renders a script element, the button every row
  carries and the readiness it holds, the status line the compile owns, the
  height model and the panes beneath the list, the one image the page carries
  and the limit that remains, the engines the page has been run in, and the
  build and deploy that publish it
max_lines: 155
generated: 2026-08-22
---

# Web demo

The project's front door: `md2pdf-core` compiled to `wasm32-unknown-unknown` and called
from one page, published to GitHub Pages at `https://ivapo.github.io/md2pdf/`. The third
front end beside the CLI and the desktop app, sharing `core`'s API with them and nothing
else. `mpdf-006` owns the directory; `mpdf-003` §1.1 named this a later spec rather than a
phase of its own, and it is that spec.

**The PDF a visitor sees is the PDF the CLI writes.** `web/src/lib.rs:render` calls
`md2pdf_core::md_to_pdf` with the page's one image in hand, and maps a failure through
`JsError` over the error's own `Display` — the same sentence `cli/src/main.rs` prints after
its `error: ` prefix, so a construct outside the dialect is refused in the words it is
refused at the terminal.
`web/src/lib.rs:anchors` is the second export, `mpdf-003` Phase 6's `line:page` pairs
answered in a browser; **the page no longer calls it**, and it stays because it is that
phase's export rather than this page's. `web/src/lib.rs:start` routes a panic to the console.

## The page

One page, not a landing page plus a player. The text is first in the document and renders
without the module; `<script type="module">` is async by definition, so the module lands
while the reader reads. Two URLs would turn the click that settles the argument into a
navigation that re-pays the download.

**The module is the whole cost and it is large.** Measured 2026-08-15: 25.7 MB raw, 7.8 MB
brotli, of which `core/assets/fonts` is 2.5 MB; nothing in the design is keyed to either
figure. `web/Cargo.toml`'s release profile is tuned for size over speed — `opt-level = "s"`,
`lto`, one codegen unit, `panic = "abort"`.

The panes sit beneath the list, so the page scrolls and `main` takes a slice of the
viewport — `clamp(360px, 70svh, 720px)`, in `svh` rather than `dvh` so browser chrome
hiding mid-scroll does not resize the frame and lose the reader's place in the PDF. Below
720px the two panes stack and take the viewport. One `<noscript>` paragraph says what
needs scripting: every example is on the page either way, compiling one is not.

## One click, one PDF

**Every row carries a `load it` button, and the button is where readiness lives.** All
eleven carry `disabled` in the markup — inert with scripting off rather than promising a
compile the page cannot run — and `await mod.default()` resolving enables them. Nothing
else reports readiness. The button carries no `data-example` attribute of its own:
`core/tests/page_examples_test.rs` asserts the page holds exactly eleven of those.

A click reads its own row's element — `button.closest('.row')`, then
`querySelector('script[data-example]')` — writes that source into the textarea and calls
`compile` itself, because **writing `value` fires no `input` event** and the typing path's
300 ms debounce would never run. Then it scrolls to the panes, which sit below the rows.

**A click empties the pane before it compiles; typing does not**, and the two acts get
different answers deliberately. An author mid-edit passes through broken states constantly
and keeps the last good page — `mpdf-003`'s behaviour, living in `compile`'s catch branch.
A reader who clicks a row captioned *what it refuses, on purpose* has asked to be shown a
refusal, and the previous row's PDF would be the page asserting something false about its
own output. So the **button** owns the clearing, not `compile`: the iframe is hidden and
its blob revoked, and a refusal reached that way leaves it that way.

**The status line is the compile's alone.** `#status` sits inside `main` above both panes,
carrying the sentence a refusal names and nothing when the compile succeeded;
`#status:empty { display: none }`, so a page that compiles shows no strip. The spike's
instrument panel is gone, elapsed time and anchor list with it — but the wire measurement,
read from `performance.getEntriesByType` rather than asserted, **moved to `console.log`**:
it answers one of the three questions the spike exists to ask, and the person asking that
question opens a console where a reader does not.

**Run in two engines on 2026-08-22**, headless over `http://127.0.0.1`: **Chromium
151.0.7922.34 and WebKit 26.5**, identical results — every button inert before the module
resolved and live after, each accepted row drawing a PDF, each refusal emptying the pane
and printing its exact sentence, and a typed refusal keeping the last good page. The asset
channel was checked the same day in **Chromium alone**, a byte array crossing an existing
`wasm-bindgen` boundary being nothing two engines can disagree about; the image row's PDF
came back byte-identical to the one the CLI writes for the same source and file. Whether
the page *states* browser support is still open (`mpdf-006` OQ-5); it makes no such claim.

## What the page claims, and the test that holds it to the compiler

The page carries three groups and **eleven examples**: syntax an ordinary renderer passes
through as text (a caption over a table, over a listing and over an image, a `:::` group, a
`{#name}` and the `[](#name)` that points at it, display math), things markdown has no way
to say (the six frontmatter keys, a footnote), and three refusals — raw HTML, a task list,
and a LaTeX command off the accepted list. Twenty-two constructs are supported, so the page
is a chosen few and links out to the README for the rest.

**Each example is one element, and three consumers read it.** A
`<script type="text/markdown" data-example="…" data-expect="ok|error">` holds the source;
the reader sees it, the row's button loads it, and `core/tests/page_examples_test.rs` reads
it through `include_str!`. A `<script>` holds raw text, so markdown inside one needs no
escaping and a block of a non-JavaScript type is never executed.

**The content is load-bearing bytes: flush left, no leading and no trailing newline.** Not
tidiness: measured at two spaces of indent the frontmatter example stops being frontmatter
and reaches the page as a setext heading over prose, and the caption example keeps its
table but emits `: The measurements.` as literal text — and `md_to_pdf` returns `Ok` for
both. A leading newline is the same hazard one line on, moving every refusal's `at line N`
by one. The rule needs no stripping step in any consumer, so they cannot drift apart by
normalising differently.

`core/tests/page_examples_test.rs` asserts that rule, a count of exactly eleven, a mark of
`ok` or `error` on each, unique names, one asset element, and message elements matching the
refusals; then that each `ok` example compiles — **each handed the page's image**, as the
page hands it to every compile — and that each `error` example's `to_string()` equals its
row's visible `<code data-error-for="…">` text, character for character. **The checked
sentence is the one the reader sees** — an attribute copy would prove agreement with a
string nobody reads. The `<code>` scan is weaker than the `<script>` scan, parsed markup
equalling its raw slice only while the sentence needs no character reference, so a separate
assertion refuses a message carrying `<`, `&` or a newline. It does **not** assert the 8/3
split.

**That test is a workspace test over a file outside the workspace, and that is the point.**
`web/Cargo.toml`'s empty `[workspace]` table detaches the directory, so `cargo build
--workspace` and `cargo test --workspace` behave as they did before it existed — reading a
file is not membership.

**A `<script>` is `display: none` in every UA stylesheet**, and the source column of every
row is one. `script[data-example] { display: block; white-space: pre; overflow-x: auto }`
renders it. A visible `<pre>` duplicate was refused deliberately: two copies of an example
can differ, which is the failure the whole arrangement prevents. The comparison column
beside it is **written by hand, never rendered** — a second markdown renderer on the one
page claiming a single module does the work would be an unaudited dependency, and no
implementation is entitled to be called *ordinary*.

## The one image, and the files that stay out

**The page owns one image file and the reader owns none.** `samples/pipeline.svg` is
carried inline in `web/index.html`, in a `<script type="image/svg+xml" data-asset="…">`
whose attribute value *is* the path `md2pdf_core::Asset` is given — one copy of one name,
which the caption row must write identically or fail the suite. `render` hands it to
**every** compile, typed and clicked alike: `md_to_pdf` ignores an asset the document never
names, and a channel open to the button alone would draw Figure 1 on a click and refuse it
on the next keystroke.

**A reader's own files stay parked.** A browser has no filesystem, so
`core/src/lib.rs:image_paths`' shopping list has nowhere to be read from and an
`![…](their-file.svg)` comes back `Error::MissingImage` from `core/src/lib.rs:collect` — a
line above the textarea names the one readable file and that sentence, so a visitor meets
it before the compiler says it. The page is not an editor and never gains accounts,
persistence or a share link: `mpdf-001` §1.1 refuses servers permanently, and Pages serves
static files.

## The build and the deploy

`.github/workflows/pages.yml` builds `web/` alone with `wasm-pack build --target web
--release`, on a push to `main` touching `web/**`, `core/**` or the workflow. It assembles
`_site` from **`web/index.html` and `web/pkg/` only**, so anything the page needs must be
inline in that file or added to that step — which is why the one image is. `wasm-pack`'s
own `.gitignore` inside `pkg/` is deleted before upload, or the artifact would skip the
module.

**The module is never committed** — ten times the size of this repository, and git history
is permanent, so it is built in the job and handed straight to the Pages artifact. The
workspace is untouched and `web/` is not one of its members, so nothing here changes what a
phase's exit gate runs; nor does anything here run it. `cargo test --workspace` is a local
check before a push, not the publish path.
