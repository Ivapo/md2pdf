---
title: web-demo
sources:
  - web/index.html
  - web/src/lib.rs
  - core/tests/page_examples_test.rs
  - .github/workflows/pages.yml
covers: >
  the published browser demo: the one page and what it costs, the two exports
  that cross into WebAssembly, the list of what the dialect adds and the marked
  examples that carry it, the byte rule those examples obey and the test that
  enforces it, the CSS that renders a script element, the other column generated
  from the same parse and the markers and substitution that carry it, the button
  every row carries and the readiness it holds, the status line the compile owns, the
  height model and the panes beneath the list, the two files the page carries
  down one attribute and the limit that remains, the engines the page has been
  run in, and the build and deploy that publish it
max_lines: 245
generated: 2026-08-23
---

# Web demo

The project's front door: `md2pdf-core` compiled to `wasm32-unknown-unknown` and called
from one page, published to GitHub Pages at `https://ivapo.github.io/md2pdf/`. The third
front end beside the CLI and the desktop app, sharing `core`'s API with them and nothing
else. `mpdf-006` owns the directory; `mpdf-003` §1.1 named this a later spec rather than a
phase of its own, and it is that spec.

**The PDF a visitor sees is the PDF the CLI writes.** `web/src/lib.rs:render` calls
`md2pdf_core::md_to_pdf` with both of the page's files in hand, and maps a failure through
`JsError` over the error's own `Display` — the same sentence `cli/src/main.rs` prints after
its `error: ` prefix, so a construct outside the dialect is refused in the words it is
refused at the terminal.
`web/src/lib.rs:anchors` is the second export, `mpdf-003` Phase 6's `line:page` pairs
answered in a browser; **the page no longer calls it**, and it stays because it is that
phase's export rather than this page's.

**Both take the two files as four scalars**, `(path, bytes)` twice, over one private
`web/src/lib.rs:assets` so the pair cannot disagree about which scalar is which path. Not
an array: `web/Cargo.toml` carries `wasm-bindgen` and `console_error_panic_hook` and
nothing else, so a `Vec<Vec<u8>>` across that boundary is a new dependency on a page whose
whole cost is its module — and the set is closed at two, `mpdf-006` §1.2 parking a
reader's own files permanently. `anchors` passed `&[]` until `mpdf-007` Phase 4, which
meant it could answer for neither of the page's own rows.
`web/src/lib.rs:start` routes a panic to the console.

## The page

One page, not a landing page plus a player. The text is first in the document and renders
without the module; `<script type="module">` is async by definition, so the module lands
while the reader reads. Two URLs would turn the click that settles the argument into a
navigation that re-pays the download.

**The module is the whole cost and it is large.** Measured 2026-08-15: 25.7 MB raw, 7.8 MB
brotli, of which `core/assets/fonts` is 2.5 MB; nothing in the design is keyed to either
figure. `web/Cargo.toml`'s release profile is tuned for size over speed — `opt-level = "s"`,
`lto`, one codegen unit, `panic = "abort"`.

`web/pkg/md2pdf_web_spike_bg.wasm` was **25,362,143 bytes** on 2026-08-23, against the
**25,342,182** `mpdf-006` Phase 4 recorded: **+19,961**. The record is the requirement,
never a ceiling — and the delta is **not one phase's**. That baseline predates `mpdf-007`
Phase 2, which put `core/src/bibliography.rs` and `hayagriva`'s reader inside the wasm
build; Phase 4 added two `wasm_bindgen` parameters apiece to two exports.

The panes sit beneath the list, so the page scrolls and `main` takes a slice of the
viewport — `clamp(360px, 70svh, 720px)`, in `svh` rather than `dvh` so browser chrome
hiding mid-scroll does not resize the frame and lose the reader's place in the PDF. Below
720px the two panes stack and take the viewport. One `<noscript>` paragraph says what
needs scripting: every example is on the page either way, compiling one is not.

## One click, one PDF

**Every row carries a `load it` button, and the button is where readiness lives.** All
twelve carry `disabled` in the markup — inert with scripting off rather than promising a
compile the page cannot run — and `await mod.default()` resolving enables them. Nothing
else reports readiness. The button carries no `data-example` attribute of its own:
`core/tests/page_examples_test.rs` asserts the page holds exactly twelve of those.

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
came back byte-identical to the one the CLI writes for the same source and file. **The page
states no browser support and carries no row for it** (`mpdf-006` OQ-5, resolved
2026-08-22): a browser that cannot run the module already says so itself, the module
script's `catch` writing `failed to start: …` into `#status`.

**The second file was checked the same way on 2026-08-23**, Chromium 151 alone on that
same argument: all twelve buttons live, the citation row's PDF **byte-identical** to the
CLI's for the source and the bibliography extracted from the page itself
(`0bfa9cb3…`, 18,419 bytes, a `[1]` in the body over a *References* list), the image row
unchanged and a refusal still printing its exact sentence. **The typed path was checked
separately**, on a document naming *both* files at once — that is the hazard the single
call site exists against, and a channel open to the button alone would pass every other
check here.

## What the page claims, and the test that holds it to the compiler

The page carries three groups and **twelve examples**: syntax an ordinary renderer passes
through as text (a caption over a table, over a listing and over an image, a `:::` group, a
`{#name}` and the `[](#name)` that points at it, display math), things markdown has no way
to say (the nine frontmatter keys that decide the look, a footnote, a citation and the
reference list it earns), and three refusals — raw HTML, a task list, and a LaTeX command
off the accepted list. Twenty-five constructs are supported, so the page is a chosen few
and links out to the README for the rest.

**The page's own lede is two behind that**, and the gap is logged rather than half-fixed
here: `web/index.html`'s `<p class="lede">` reads "Twenty-three constructs are supported
and twelve are shown here", which was one behind before `mpdf-005` Phase 10 and is two
behind after it. Correcting the number alone would leave the page claiming a count it shows
no example of, and an example is real work under this spec's own gate — every claim on the
page is a snippet the workspace suite compiles — so both the count and an `::: abstract`
row belong to a phase of `mpdf-006` rather than to another spec's close-out.

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

`core/tests/page_examples_test.rs` asserts that rule, a count of exactly twelve, a mark of
`ok` or `error` on each, unique names, two asset elements, and message elements matching the
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
can differ, which is the failure the whole arrangement prevents.

## The other column, and where its markup comes from

**The column beside each example is generated, not written.** It is
`core/src/lib.rs:md_to_html` over that row's own source — the same parser through
`core/src/emit.rs:parser`, options and broken-link callback together, written out by
pulldown-cmark's own HTML backend instead of by
the emitter, so it is not a second renderer but the one the page's whole claim is already
about. It shows what this parse looks like when something other than the emitter sets it
down: the caption marker is lost because nothing but the emitter is looking for it. The
twelve labels read `the same parse, as HTML`. **The bytes are inlined rather than produced
at load**, so no column sits behind the 7.8 MB and a reader with scripting off meets both
halves of every row.

**The blocks sit between `<!--html:NAME-->` and `<!--/html:NAME-->`**, keyed to the row's
own `data-example` value. This is the one region in the page that cannot end at a closing
tag — the `raw-html` block ends `</div>`, the `footnote` block carries a
`<div class="footnote-definition">` — and counting opens against closes is an HTML parser
under another name; a comment cannot nest and `push_html` emits none. **What lies between a
pair is exactly what the generator returned**, nothing trimmed at either end, so the page
and the test compare the same bytes by construction; `raw-html` ends without a trailing
newline. A `div.rendered` wrapper sits outside the markers, uncompared, carrying the type
scale four real tables, two real checkboxes, a heading, a listing and a real `<div>` need.

**One substitution, and exactly one**: an image destination equal to the page's
`data-asset` name becomes a `data:` URI over those same bytes, **percent-encoded over an
explicit set — every byte outside ASCII letters, digits and `-._~`**. The set is named
because the reflex is broken and the break is invisible to an equality check —
`pulldown_cmark`'s `escape_href` leaves `#` unencoded and the SVG carries
`stroke="#1e3c82"`, so a raw URI truncates at the fragment and renders nothing while both
sides agree about the broken bytes. Measured 2026-08-22: 509 bytes to a 954-byte URI that
round-trips exactly, the diagram loading at its declared `320×72`.

**The generator is the test** — `core/tests/page_examples_test.rs:generated` produces a
block, `every_generated_block_is_the_parsers_own_html` compares all twelve against the page,
and `bless_the_generated_blocks`, `#[ignore]`d so `cargo test --workspace` skips it, writes
them in; a generator of its own would implement the substitution twice. Three assertions
guard it: the marker counts, that the image is named once across the twelve outputs and
survives nowhere after substitution, and that no block holds `<script`, `data-example="`,
`data-asset="` or `<!--` — the first three handing the file's own scans a phantom element,
the fourth ending the delimiter early. Checked 2026-08-22 in headless Chromium **with
JavaScript disabled**: every column rendered, the diagram visible rather than a
broken-image box, the frontmatter row showing no keys at all.

## The two files, and the ones that stay out

**The page owns two files and the reader owns none**: `samples/pipeline.svg`, and the
bibliography the citation row names. Both are carried inline in `web/index.html`, and
**both ride one attribute**. `data-asset`'s value *is* the path `md2pdf_core::Asset` is
given — as true of a `.yml` as of an `.svg` — so a second attribute would be a second
mechanism for something the page already has one word for. It is one copy of one name
apiece, which the caption row and the citation row must write identically or fail the
suite.

**The `type` is the discriminator, and it was already one.** The image is a
`<script type="image/svg+xml" data-asset="…">` and the bibliography a
`<script type="application/yaml" data-asset="…">` — non-JavaScript types, so neither is
executed and neither needs escaping. The page's module selects on the pair and
`core/tests/page_examples_test.rs` scans for it, so neither depends on document order, and
it is what keeps the `data:` URI substitution keyed to the image alone. Both obey the
examples' byte rule at the ends — no leading and no trailing newline — but only the
bibliography's *inner* indentation is load-bearing: the SVG's bytes reach Typst's image
loader, which does not read them as structure, where a YAML reader does.

`render` hands **both** to **every** compile, typed and clicked alike: `md_to_pdf` ignores
an asset the document never names, and a channel open to the button alone would draw
Figure 1 on a click and refuse it on the next keystroke.

**A reader's own files stay parked.** A browser has no filesystem, so
`core/src/lib.rs:image_paths`' shopping list and `core/src/lib.rs:bibliography_path`'s
second half both have nowhere to be read from — an `![…](their-file.svg)` comes back
`Error::MissingImage` from `core/src/lib.rs:collect` and a `bibliography:` of their own
comes back `Error::MissingBibliography` from the same place. A line above the textarea
names the two readable files and that first sentence, so a visitor meets it before the
compiler says it. The page is not an editor and never gains accounts, persistence or a
share link: `mpdf-001` §1.1 refuses servers permanently, and Pages serves static files.

## The build and the deploy

`.github/workflows/pages.yml` builds `web/` alone with `wasm-pack build --target web
--release`, on a push to `main` touching `web/**`, `core/**` or the workflow. It assembles
`_site` from **`web/index.html` and `web/pkg/` only**, so anything the page needs must be
inline in that file or added to that step — which is why both of the page's files are. `wasm-pack`'s
own `.gitignore` inside `pkg/` is deleted before upload, or the artifact would skip the
module.

**The module is never committed** — ten times the size of this repository, and git history
is permanent, so it is built in the job and handed straight to the Pages artifact. The
workspace is untouched and `web/` is not one of its members, so nothing here changes what a
phase's exit gate runs; nor does anything here run it. `cargo test --workspace` is a local
check before a push, not the publish path.
