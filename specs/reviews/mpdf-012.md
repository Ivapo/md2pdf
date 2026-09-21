# Review record — mpdf-012 (`specs/diagrams_mermaid_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — 2026-09-21 — the same three reviewers, resumed with the author's changelog — **READY**

Verdict: `READY` from all three, zero blocking. Converged at round 2 of 3. Each reviewer
checked its own round-1 blockers against the file, not the changelog, and found all
four resolved:
- the MPL-2.0 branch in `identify`, and gate 6's text assertions;
- the rewritten copyleft assertion and its three failure behaviours;
- the six-diagram fixture with its bands;
- `%%{` refused anywhere.

The grounding reviewer reran the author's round-2 harnesses. They reproduced all six
fixture widths and the byte-identical native and `wasm32` PDFs.

**The author's own round-2 fix was wrong, and two reviewers caught it.** The author had
added a note on which diagram catches a mis-set floor. The scope reviewer showed it
overstated the coverage. The exit-gate reviewer then showed the corrected version had
one clause backwards: `press-release` at two columns puts #2 at 7.93 pt in its column,
so it catches a floor **at or below** 7.93 pt, not one between 7.93 and 8. Taken
together, the gate pins the floor to (7.93, 8.46] pt. That is recorded as accepted,
since pinning it to exactly 8 pt needs a diagram within about 1% of a column edge,
which any renderer change would push out of its band.

**Non-blocking, folded in at convergence:**
- the front-matter check compares the trimmed line, since merman's reader accepts
  whitespace and a CRLF around `---`;
- gate 5's runtime check goes through the public `md_to_typst`, because the render
  function is crate-private. The harness is described well enough to rebuild, and every
  import the module declares is supplied as a throwing stub;
- the sequence diagram's exact source is in the gate;
- the band edge is 806.30, not the truncated 806.29;
- the weaker half of the `mpdf-005` argument, kind detection, is dropped. Typst falls
  back to image kind, so placement and scope carry it alone.

**Not changed:** the regenerated crate count will be 365, as the grounding reviewer
walked it, and the spec already derives the notice's count from the regenerated file.

Numbers re-checked this round:
- Typst's millimetre conversion agrees with 72/25.4 exactly, which gives `article` a
  453.543 pt text width and a 217.701 pt column, and `press-release` a 425.197 pt text
  width and a 204.094 pt column at two columns;
- the band edges are 387.0236, 435.4016, 497.6018 and 806.2992 px;
- MPL-2.0 is the only new licence term among the 36 added crates;
- the tree's highest `rust-version` today is 1.92 (the typst crates), so 1.95 is a new
  floor;
- query order in the introspector follows the source even for floats, so gate 2 can
  pair images and figures to diagrams by order.

Set on convergence: `status: accepted`, and `reviewed: 2026-09-21` on Phase 1 and
Phase 2. This was one document-wide episode, since no phase had shipped.

### Round 1 — 2026-09-21 — panel of three fresh reviewers with repo access (grounding, exit-gate testability, scope and cross-file consistency) — **NOT READY**

**Round 0**, answered by the author before round 1, for the one document-wide episode
(no phase has shipped): *Yes.* Both phases produce the project's observable, the typeset
PDF compiled from the author's markdown: a `mermaid` fence that today sets as a listing
of its source becomes the diagram it describes. It is the right observable, because
authors already write Mermaid for GitHub and GitLab, and the spike showed the result
legible at caption size in both looks.

Verdict: `NOT READY` from all three. Four distinct blockers after deduplication, three of
them raised by every reviewer independently.

**B1 — the MPL-2.0 text would never arrive** (all three). The draft said the text
"arrives on regeneration". `tools/third-party-licenses.py:identify` has no Mozilla
branch; run on `cssparser`'s, `cssparser-macros`' and `dtoa-short`'s `LICENSE` it returns
nothing. So all four crates would have landed under "not reproduced", with prose
claiming they ship no licence file, and no gate looked for the text. Fixed: Phase 1
adds the branch, and gate 6 asserts the text is in the regenerated file and in
`--licenses=full`, and that none of the four is listed as unreproduced.

**B2 — the copyleft claim lives in four places, one of them a shipped test** (all
three). `cli/tests/cli_test.rs:the_notice_states_the_facts_the_table_and_the_font_directory_hold`
asserts that no licence term contains `MPL`. It is `mpdf-001` Phase 14's gate (3), and it
fails on the dependency whatever the notice says, so "the licence tests pass" could have
been met by deleting it. The README's Licence section and `mpdf-001` Phase 14's prose
state the same thing. Fixed:
- §2 lists all four sites.
- The test's replacement assertion is spelled out: copyleft rows other than MPL-2.0
  still fail, MPL-2.0 rows must be named in the notice, and "None is copyleft" must be
  gone.
- §1.1 step 1 now names the Phase 14 narrowing.
- The close-out adds the README Licence section and a dated note on Phase 14.

**B3 — gates 1 and 2 described different fixtures** (all three). Gate 2's roles needed at
least five diagrams, captioned where only a caption can float, in width bands the draft
never stated. `press-release`'s "nothing floats" was true only at one column. Fixed by
naming the fixture: six diagrams with their sources, measured 2026-09-21 under §2's exact
configuration (78.72, 411.68, 467.36, 470, 525.6 and 1140.56 px). Each band is asserted
as a precondition. The expected widths are computed in four configurations from each
look's own literals, within 0.01 pt. The test runs in `core/src/lib.rs`'s `tests` module,
because only there does the introspector reach the compiled document.

**B4 — the directive scan looked only at line starts** (grounding). merman-core's
`directive_blocks_controlled` finds `%%{` anywhere in the input and applies `init` and
`initialize`. So `A --> B %%{init: …}%%`, or a directive inside a `%%` comment, would have
passed the scan and reset the label size the sizing rule depends on. Fixed: `%%{` is
refused anywhere, and gate 3 carries both cases.

**A non-blocking finding promoted to a recorded decision.** The scope reviewer found
that `diagram(…, caption:, name:)` contradicts `mpdf-005`'s *the look contract does not
widen for a caption*, while the draft had said captions were used "unchanged". §2 now
records why a diagram's caption crosses: the look must build the figure to decide its
float, and a figure's kind cannot be detected through a `context` body. It also records
that `mpdf-005`'s reason, that styling needs no argument, still holds. The close-out
adds a dated note on `mpdf-005` and corrects the rules sentence.

**Non-blocking, accepted and folded in:**
- `serde_json` is a second direct dependency, already in the tree.
- The reason for `default-features = false` was wrong: the defaults are the Cytoscape
  layout and math labels, not ELK or the system adapters.
- Determinism belongs to the engine's `RuntimePolicy`, not `SvgEnvironment`.
- ER relationship labels are 14 px, so the rule is stated as keyed to 16 px text.
- The keyword-finding rule is now stated.
- A one-off `wasm32` runtime check under Node joins gate 5, since a build alone is what
  let the rejected renderer through.
- The notice's 50-line cap is stated.
- A determinism assertion and a ` ```Mermaid ` case are added.
- The untestable fifth error row is marked as such.
- Phase 2's ER fixture band is stated.
- The spike's extra `fontFamily` and `state.padding` settings are disclosed.
- The compiled function is described honestly.
- merman's `rust-version = "1.95"` is recorded as a cost.
- `core/tests/messages_test.rs` rows and a `#let diagram(` needle are in scope.
- `take_member` is not changed.
- The fence's line comes from the end event's range.
- The smaller rules and README sites are listed.
- Consistency fixes: the example is five lines; `993.2` is marked illustrative; the §6.1
  citation is corrected; `mpdf-008` added to `related`.

**Resolved by measurement rather than argument.** The scope reviewer asked whether the
native-vs-`wasm32` last-digit SVG differences break `rules/pipeline.md`'s "the same PDF
on every machine". The author compiled both SVGs of each differing diagram, state and
pie, through `md_to_pdf`: the SVGs differ and the PDFs are byte-identical (13,804 and
14,865 bytes). The claim holds for the measured cases, and §2 records it.

**Rejected:** nothing.

Numbers re-checked this round, so a later one can trust them:
- all three reviewers re-derived the column (217.70 pt), the text width (453.54 pt) and
  the gutter (Typst's 4%), and the spike's table values from its viewBoxes;
- the lockfile delta is 388 → 424 packages, and the binary 34,866,432 → 46,548,320
  bytes;
- `rules/pipeline.md` is at 1292 of 1300 lines;
- the syntax-error span 47..63 starts on the block's third line;
- merman's detection order puts `flowchart-elk` before `flowchart-v2`;
- the class SVG contains `Vec&lt;u8&gt;`;
- ER labels are 14 px (relationships) and 16 px (entities).
