# Review record — mpdf-006 (`specs/web_demo_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 4 only — 2026-08-22 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two non-blocking. Converged inside the cap, on the
episode's second round. The blocker was confirmed resolved **against the file**, and the
reviewer did not check the scheme in principle — it ran the marker round-trip over all
eleven measured `push_html` outputs and got the block back byte-for-byte **11/11**,
`raw-html` included, which is the one output that ends without a trailing newline and
therefore the one "nothing trimmed at either end" had to be unambiguous about. It also
confirmed that none of the eleven blocks holds any of the four refused substrings, so the
guard never false-fires; that no example name contains `--`, which would be illegal inside
an HTML comment; and that the existing scans in `core/tests/page_examples_test.rs` —
`examples()`, `asset()`, both attribute counts and the `data-error-for` scan — still find
exactly what they found before, the 954-byte data URI included.

**The percent-encoding was checked the same way, and one level deeper than asked.** The
rule was run over the real inline bytes — `web/index.html`'s asset is `samples/pipeline.svg`
minus its trailing newline, 509 against 510, and both carry the `stroke="#1e3c82"` that §2's
argument turns on — producing a 954-byte URI that round-trips exactly, holds no raw `#`,
`"`, `<`, `>`, `&`, `'` or space, and **rendered in Chromium off a served page at
`naturalWidth: 320`, `naturalHeight: 72`**, the SVG's own declared size. The reviewer then
checked the trap under the trap: `escape_href` does not double-encode `%`, so even the
event-stream route would have survived — and the phase does not take it, substituting after
`md_to_html` over a destination that occurs exactly once across all eleven outputs.

Every re-keyed literal was re-derived rather than granted: three of eleven drifted columns,
25,342,182 bytes for the module, 623/625 and 153/155 for the two `rules/` caps, and the
`web-demo.md` cap precedent confirmed against `git log` as 120 → 140 → 155.

Both non-blocking findings were folded rather than deferred. `web/Cargo.toml` is named in
scope as a **conditional fourth file**, edited only if the module grew, with the expected
result stated. And the page's "Do not reformat" comment grows to cover the generated
blocks: they are exactly as byte-fragile as the examples it already warns about, and a
reader meeting generated HTML in a hand-edited file has no way to know that from the file.

### Round 1 — Phase 4 only — 2026-08-22 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes, and it is the second phase in
this spec to produce no observable and argue for itself instead. The pane, the buttons and
the compile path are untouched; what it does is put the last unchecked claim on the page
under the test the other column has had since Phase 1. The evidence that it is wanted came
out of drafting it: measured through `pulldown_cmark::html::push_html` over
`core/src/emit.rs:options`, **three of the eleven written columns assert behaviour those
options contradict** — a page built so it cannot claim what the compiler refuses had a
second column claiming what the parser does not do, and nothing would ever have caught it.

Verdict: `NOT READY`, one blocking, nine non-blocking. One generalist reviewer. **No
finding was rejected.**

**The blocker was the same one Phase 1's round 1 caught, one column over.** The phase said
the test would compare "the page's stored block" and named no marker, no key back to
`data-example="<name>"`, and no delimiting rule — and §2's standing justification for plain
string scans, that *each region ends at the next closing tag*, is **provably false for this
region**: the `raw-html` block ends `<div>a raw HTML block</div>` and the `footnote` block
carries a `<div class="footnote-definition">`, so a wrapper closed by `</div>` is
mis-delimited by the very convention that bought the scan. Counting opens against closes is
an HTML parser under another name, which the same paragraph refuses. Resolved with comment
markers keyed to the row's own example name — `push_html` emits none, a comment cannot
nest, and the region between them is exactly the generator's returned string.

Of the nine non-blocking findings, four are worth the record. **The reflex `data:` URI is
broken and the break is invisible to the check**: `escape_href` leaves `#` unencoded and
`pipeline.svg` carries `stroke="#1e3c82"`, so the URI truncates at the fragment and an
equality assertion agrees with itself about bytes that render nothing — the encoding is now
named, and the gate's browser half says *the diagram visible rather than a broken-image
box* and says why that item exists. **"One rule in one place" contradicted "applied in the
generator and in the test alike"** unless the generator *is* the test, which was the reading
that worked and was unstated; saying it also holds the phase to three files. **The drift
count was one and is three** — `math-refusal` carries the same `ENABLE_MATH` drift as
`display-math`, and the footnote row promises a rule this backend draws no `<hr>` for. And
**the prose reversal was scoped to the wrong elements**: the row-level `.says` and `.does`
sentences are `md2pdf`'s own and need no edit, while the opening lede, group 1's heading and
its "the other column prints it" were left asserting the universal the phase forbids.

The rest: `§1` and the frontmatter `reference:` needed the dated `CORRECTED` notes §1.2 had
already got; OQ-1's superseded resolution needed striking through per conventions §4; both
`rules/` files sit within two lines of their caps and the close-out said nothing about it;
and the module-size clause was keyed to `web/Cargo.toml`'s 2026-08-15 header rather than to
the file in the tree, with no threshold and no unit — advisory dressed as a gate.

### Round 2 — Phase 3 only — 2026-08-22 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two non-blocking. Converged inside the cap, on the
episode's second round. All three blockers confirmed resolved **against the files**, not
the changelog, and the reviewer checked the mechanisms rather than granting them: that a
`<script>` of a non-JavaScript type is a data block never executed and that `pipeline.svg`
contains no `</script`; that the asset element carries no `data-example`, so both the test's
count assertion and Phase 1's `script[data-example] { display: block }` override ignore it
and the reader is never shown raw SVG; and that `core/src/lib.rs:collect` iterates the
*document's* image list, so an asset the document never names is never touched.

Every literal the fixed gate is keyed to was re-derived: 10 → **11** for both `EXPECTED`
and the `data-example="` count, **8 + 3** for the split, and 510 / 231 bytes for the two
sample images. The reviewer also ran the built CLI over `![alt](pipeline.svg)` plus a
caption — `#figure(image("pipeline.svg", alt: …), caption: […])`, a 7,940-byte PDF — which
confirms **Figure 1** is the right literal for the browser half of the gate and that the
result clears the existing `pdf.len() > 1000` assertion.

**One rejection was offered and declined.** The author's argument for Chromium alone was
put up to be rejected and the reviewer accepted it instead: reading `textContent` off a data
block, encoding it and passing a byte array across the existing `wasm-bindgen` boundary are
universally implemented, and Typst embeds the figure inside the module, so both engines are
handed identical bytes. Phase 2 established the two-engine result and this phase touches no
surface where they could diverge.

Both non-blocking findings were folded rather than deferred: the older §2 block's "a new
`web/src/lib.rs` entry point beside `render`" now says in place that the block below
overrides it, and Phase 3's prose edit names the **opening** `#adds > p.lede`, that selector
matching two elements.

**OQ-3 is untouched and stays open.** It asks whether the phase happens at all and is
`needs-input`; the phase gate turns on `reviewed`, so it did not affect this verdict, and
the phase remains written to be cut with `cut` + `by: mpdf-006` losing nothing shipped.

### Round 1 — Phase 3 only — 2026-08-22 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes. Phase 3 produces the observable
and produces the one instance of it the front door cannot currently show: `md_to_pdf`
already takes assets, so a caption over an *image* — the README's own flagship, and the only
caption construct the page has to substitute around — reaches the pane, alongside a sentence
that explains `Error::MissingImage` before a reader hits it rather than after. Recorded with
the caveat the spec itself carries: OQ-3 is the human's and this loop cannot close it.

Verdict: `NOT READY`, three blocking, seven non-blocking. One generalist reviewer, matching
the two prior episodes. **No finding was rejected.**

The three blockers, and what each one really was:

1. **The phase named an image and never said how its bytes reach the page — and the only
   deploy path publishes two things.** Measured: `.github/workflows/pages.yml` assembles
   `_site` from `cp web/index.html` and `cp -r web/pkg` and nothing else, so a file dropped
   in `web/` beside them is a 404 on the published page **while a locally-served directory
   shows the row working**. That is a gate passing for the wrong reason, on the one claim
   this spec exists to protect, and it would have been found by a visitor rather than by the
   suite. Resolved with a new §2 block: the bytes go **inline in `web/index.html`** in one
   `<script type="image/svg+xml" data-asset="pipeline.svg">`, which is also what keeps the
   neighbouring "`pages.yml` needs no correction" claim true.
2. **The phase did not say which caller uses the new export, and the minimal wiring breaks
   its own flagship on the next keystroke.** The page has one compile path reached from two
   places — the 300 ms `input` debounce and Phase 2's buttons. A second export wired into
   the button alone would draw Figure 1 on click and answer the reader's next keystroke with
   `no image file supplied for 'pipeline.svg' at line 1`; wired into `compile`
   unconditionally it leaves `render` an export nothing calls. **The gate discriminated
   between neither.** Resolved by deleting the ambiguity rather than choosing a side:
   **there is no second export — `web/src/lib.rs:render` takes the asset.** One export, one
   call site, both acts covered, every existing citation still true, and
   `core/src/lib.rs:md_to_pdf`'s own contract makes it free, an asset the document never
   names being ignored.
3. **The gate authorised a branch that leaves the new row's claim compiled by nothing.** It
   had offered "the test grows an asset channel of its own **or** the row is excluded by
   name", and the two are not equal: an excluded row is a claim on the page that no test
   compiles — §2's whole failure mode, and the frontmatter's own promise broken in the phase
   that adds the row. Resolved by refusing the exclusion **by name**.

Of the seven non-blocking findings, four are worth the record. **OQ-4 was keyed to a
measurement that could not discriminate**: re-measured, `samples/pipeline.svg` is 510 bytes
and `check.svg` is 231, four orders of magnitude below the module, and no threshold was
ever going to separate them — so OQ-4 is now resolved inline on suitability instead, and the
measurement lives in the spec rather than in a future close-out. **The "fixed name" was a
third copy of a string**; `data-asset`'s value is now *the* path, and its agreement with the
row's `![…](pipeline.svg)` is checked by construction, a mismatch returning `MissingImage`
and failing `every_ok_example_compiles`. **An eleventh row makes a visible sentence false** —
"ten are shown here" — which the test does not cover, so the prose edit is named in scope,
with a recorded decision *not* to assert it: the count is spelled in English and a check
would need a number-word table to say what the structural count already says. And **the gate
had dropped Phase 2's build recipe**, though this is the first phase since the spike to
change `web/src/lib.rs` and therefore the one where a stale `web/pkg/` shows a second person
the old export.

### Round 2 — Phase 2 only — 2026-08-21 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, three non-blocking. Converged inside the cap, on the
episode's second round. Both blockers confirmed resolved **against the files**, not the
changelog.

The reviewer re-derived every literal the fixed gate is keyed to and confirmed each has a
stated method that reproduces it: 10 `class="row"`, 10 `data-example="`, 7 `ok`, 3 `error`,
and 3 `<code data-error-for="…">` naming `raw-html`, `task-list` and `math-refusal`. It
also verified by diff that **the only change Phase 1 made to the module script was its
comment block** — `compile`, `draw`, `boot` and both status writes are byte-identical to
the spike — which is what makes the two blockers below land on Phase 2 rather than on work
already shipped.

One correction the author made to their own fix is recorded because the next round should
be able to trust it: folding the findings in keyed the gate to *seven and three*, and the
first draft claimed `core/tests/page_examples_test.rs` "holds that split". **It does not.**
Read test by test, it pins the total at ten (`EXPECTED`), asserts each `data-expect` is
`ok` or `error`, and asserts set equality between the refusals and the `<code
data-error-for>` elements — no test asserts a cardinality of either subset. The gate now
says so and marks the split as re-derived from the page rather than asserted anywhere.
This is §3's "a fix can introduce a blocker" arriving on schedule.

All three non-blocking findings were folded rather than deferred: `last_updated` bumped to
2026-08-21; the Safari/Chromium result moved out of the review record and into
`rules/web-demo.md`, on the argument that a fact about how the published page behaves is a
`rules/` fact while the review record answers what a review found; and the two loose ends
of the status line's new job named in scope — an empty `#status` still paints its padding
and rule, and the page stops calling `web/src/lib.rs:anchors`, which is `mpdf-003` Phase
6's export and **stays** whatever the page does with it.

### Round 1 — Phase 2 only — 2026-08-21 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes, and it is the phase that
redeems the one before it. Phase 2 produces the observable — a click compiles the row's own
source through `web/src/lib.rs:render` over `core/src/lib.rs:md_to_pdf`, the same bytes the
CLI writes — and Phase 1's own round 0 recorded that its value was *conditional on Phase 2
following*, since a static list is something a README could carry. This is that phase.

Verdict: `NOT READY`, two blocking, nine non-blocking. One generalist reviewer.

**Both blockers are one class of problem: the phase was written before Phase 1 existed, and
two of its sentences describe a page that was never built.** Phase 1 shipped the same day
this round ran, and its scope deliberately kept the module script's behaviour — so the
prose Phase 2 rested on had been overtaken without anyone editing it. This is the failure
mode a scoped round on a spec with a freshly shipped phase exists to catch, and it is worth
recording as a pattern rather than as two findings.

1. **"The same place `#status` stops reporting the boot" names a code site that does not
   exist, and §2's status-line decision was claimed by no phase.** Measured: `compile`
   appends the `boot` string to *both* status writes, success and failure, so the line
   never stops reporting the boot. §2 had recorded that the line "becomes the compile's own
   line… with readiness carried by the buttons instead" but named no phase, so under one
   reading a recorded decision shipped in no phase at all, and under the other an
   implementer had to guess whether the instrument panel survived. Resolved with a new §2
   block assigning it to Phase 2 and saying what becomes of the telemetry: the line carries
   the compile alone, and **the wire measurement moves to `console.log` rather than being
   deleted**, because it is the live answer to one of the three questions the spike exists
   to ask.
2. **"The refusal rows … show the error where a PDF would be — that path already works"
   contradicts the code.** Measured: the catch branch writes `#status`, which sits *above*
   the panes, and never calls `draw` — so the previous example's PDF stays in the pane. A
   reader clicking the raw-HTML row would have got a refusal's sentence over a rendered page
   from the row before it: **a page asserting something false about its own output, which is
   the failure this spec exists to prevent, reached from the other direction.** Resolved
   without deleting the shipped behaviour, because the argument for it is sound and is about
   a different act: **typing and clicking are different acts and get different answers.** An
   author mid-edit keeps the last good page — that is `mpdf-003`'s behaviour and Phase 2
   does not touch it. A reader who clicks *load it* on a row captioned "what it refuses, on
   purpose" has asked to be shown a refusal, so the button clears the pane before it
   compiles. **The button owns that, not `compile`**, which keeps the two acts apart in the
   code as well as in the argument.

The nine non-blocking findings were all accepted and none rejected. The three worth
recording: the gate said "each row's button … produces a PDF" of ten rows when three of
them are refusals; **a button carrying a `data-example` attribute of its own would fail
`core/tests/page_examples_test.rs`**, which asserts the page holds exactly ten of that
attribute, so the trap is named in scope with its reason; and writing `textarea.value`
fires no `input` event, so the handler must call `compile` itself rather than rely on the
300 ms debounce the typing path uses. The gate also gained the build recipe a second person
needs — `wasm-pack build --target web --release` and serving over HTTP, since `web/pkg/` is
gitignored and an ES-module import fails from `file://` — which Phase 1's no-JS check did
not need. **OQ-5 was narrowed rather than closed**: running the check in two browsers
produces the evidence and does not settle whether the page states support, and a support
row is explicitly out of Phase 2's scope.

### Round 2 — Phase 1 only — 2026-08-19 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two non-blocking. Converged inside the cap, on the
episode's second round.

All three blockers confirmed resolved **against the files and in a browser**, not
against the changelog. The reviewer re-ran the built CLI over all ten examples in
their new byte-rule form and loaded the `data-example` / `data-error-for` / CSS
shape in Chromium:

- The "same bytes by construction" claim was checked rather than granted. The
  element's `textContent` came back `"| a | b |\n|---|---|\n| 1 | 2 |\n\n: The
  measurements."`, with no leading or trailing newline, byte-identical to the slice
  an `include_str!` scan takes between `>` and `</script`.
- The CSS override was measured: computed `display: block`, `white-space: pre`,
  rendered box 1184×75, and `document.body.innerText` carrying the source — so the
  source column is rendered text rather than JavaScript-injected, which is what the
  no-JS half of the gate asserts.
- The ten examples were compiled. The seven `ok` rows return `Ok`; the three
  refusals return `unsupported markdown construct 'raw HTML block' at line 3`,
  `unsupported markdown construct 'task list marker' at line 1`, and `math error at
  line 1: unsupported command '\includegraphics'`. **Those `at line N` values are
  well-defined only because of the byte rule**, which is round 1's blocker paying
  for itself.

Both non-blocking findings were folded rather than deferred:

- **The two scans do not rest on the same thing.** A `<script>` is raw text, so
  scanning to `</script` is what an HTML parser does; a `<code>` is parsed markup,
  so its raw slice equals its rendered text only while the sentence needs no
  character reference. True of all three of Phase 1's refusal messages, and a later
  one carrying `<` or `&` fails loudly rather than passing wrongly. §2 now says so
  instead of implying one guarantee over both halves.
- **The repo's own fenced-code fixture violates the new byte rule.**
  `tests/fixtures/captioned_blocks.md` indents its `println!` by four spaces, and it
  is the first thing an implementer reaches for. The spec's claim that none of the
  ten needs an indented line is true, but the two code-carrying rows have to be
  written with a body-less function on purpose. §2 now names that.

No finding was rejected in either round.

### Round 1 — Phase 1 only — 2026-08-19 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Phase 1 produces **no**
observable, and §4 argues that explicitly rather than assuming it: it ships the rows,
the written comparison and the harness that puts every example under `cargo test`,
while Phase 2 — one phase later — is what turns a row into a compiled PDF. Judged the
right thing to build, with the caveat recorded in the phase itself: its value is
conditional on Phase 2 following, since a static list is something a README could
carry, and what earns the phase its place is the test that stops the page from ever
claiming markdown the compiler refuses.

Verdict: `NOT READY`, three blocking, eight non-blocking. One generalist reviewer
rather than a panel — 387 lines, blast radius one HTML file plus one new test.

The three blockers:

1. **The stated extraction method silently degraded accepted examples, and the gate
   could not see it.** The spec said only "plain string operations… the content ends
   at the next `</script`", with no word on indentation or the leading newline.
   Measured: at two spaces of indent the six-key frontmatter example stops being
   frontmatter and reaches the page as a setext heading over prose, and the caption
   example keeps its table but emits `: The measurements.` as literal text — and
   `md_to_pdf` returns **`Ok`** for both. The page would have asserted "a caption
   makes a figure and numbers it" while the PDF showed exactly what its own "ordinary
   renderer" column described, with the gate green. **This is the failure §2 exists to
   prevent, reached through §2's own method.** Resolved with a byte rule — content
   flush left, no leading and no trailing newline — chosen over "strip one leading
   newline" so that the two consumers cannot drift by normalising differently, plus
   three assertions in the test that enforce it.
2. **No marking scheme, and nowhere for a refusal's expected message.** The test could
   not be written without inventing both. Resolved with `data-expect="ok"|"error"` and
   a visible `<code data-error-for="…">` element, and with the rule that **the checked
   sentence is the one the reader sees** — an attribute copy would let the gate prove
   agreement with a string nobody reads.
3. **A `<script>` element does not render**, so with JavaScript disabled the md2pdf
   half of every comparison row was invisible — contradicting both §1's sketch and the
   phase's own gate. Three resolutions existed and the spec picked none. Resolved with
   `script[data-example] { display: block; white-space: pre; }`, refusing the `<pre>`
   duplicate by name because two copies of an example can differ.

The eight non-blocking findings were all accepted. The one worth recording: **§1.1
mis-cited `.github/workflows/pages.yml`** as the file saying the image story "belongs
to a spec nobody has written". It does not — that sentence is `web/index.html`'s, and
`pages.yml` contains no paragraph on the subject at all. The author had built Phase 3's
close-out on correcting text that does not exist; it now corrects `web/src/lib.rs`'s
module doc, and the spec states that `pages.yml` needs no correction.

Numbers re-measured this round, so later rounds can trust them: `core/assets/fonts` is
exactly 2.5 MB; `core/src/frontmatter.rs` takes exactly the six keys; the eight group
refusal shapes are eight, counted by distinct message; `core/src/emit.rs:check_image`
refuses seven or eight destination shapes depending on whether the extension gate counts
as one or two, so the spec now carries no number there; and the module in the tree is
25,316,809 bytes against `web/Cargo.toml`'s 25.7 MB header, from the same 2026-08-15
build. Nothing in the spec is keyed to the last of those.
