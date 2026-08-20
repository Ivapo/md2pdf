# Review record — mpdf-006 (`specs/web_demo_spec.md`)

Append-only. One heading per round, newest first.

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
