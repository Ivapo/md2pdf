# mpdf-009 — review record

Append-only. Newest round first. One heading per round.

### Round 4 — Phase 1 only — 2026-08-25 — panel of three, resumed — **READY (converged)**

**Past the loop's three-round cap, by the human's explicit decision** at the
escalation. All three lenses READY, zero blocking. `status` set to `accepted`
and Phase 1's `reviewed` set to 2026-08-25; Phases 2 and 3 remain `null` and
are not cleared to build.

Round 3's two blockers resolved, both in the shape their raisers prescribed.
**The gesture's drift**: the *(page, fraction)* anchor is now taken when the
gesture *starts* and reapplied on every step, not taken at the render, where it
would have preserved a displaced position faithfully and restored nothing.
§2, §3 and gate clause 4 were brought into agreement, and clause 4 now reads
the anchor "before the drag was started", which is what makes it able to fail.
**Gate clause 8** is re-keyed to the drawn canvas showing the edit rather than
to the header's word, `current`/`stale` being set in Rust from the compile
alone on a thread occlusion does not stop — the old clause passed on the
defect it existed to find.

**The drift figure was re-derived independently** from the app's default
geometry — a 900 px window, the text pane at 40%, A4 at 763.7 px a page — giving
611 px of displacement on page 5 for a 20% widen, against the draft's "some
600 px". The spec now carries the derivation.

Non-blocking folded in on convergence: what marks a gesture's *start* for a
cause with no pointer event is now stated — `box().width === fitted` opens one,
a difference continues one — which also closes the window-resize hole in clause
4; clause 4 runs again for a window drag-resize; and a compile landing
mid-gesture is decided to keep the reader's place and skip case 2's caret jump
for that compile, on the grounds that it never moves the page out from under a
hand that is on it.

**The pattern across four rounds, recorded because it is the finding about this
phase rather than in it: every round's blockers were introduced by the previous
round's fix.** Eleven blocking findings, all accepted, none rejected — seven
latent in the draft, four created by fixing them. It is a mechanism whose parts
are tightly coupled enough that each correction shifts load onto the next, and
a fifth round would not have been surprising.

One author error worth its own line: the `NOTICE` correction was reported as
folded in when an unguarded string replace had matched nothing across a line
break. A reviewer caught it, not the author. Every edit since has been
assert-guarded.

### Round 3 — Phase 1 only — 2026-08-25 — panel of three, resumed — **NOT READY (escalated at the cap)**

Two of three READY; the grounding and exit-gate lenses each returned one
blocking finding, **both newly introduced by round 2's own fixes**. §7 rule 6
caps the loop at three rounds, so it stops here and the human decides. Nothing
was set: `status` stays `draft` and Phase 1's `reviewed` stays `null`.

Round 2's blocker confirmed resolved by its raiser, against the code: `fitted`,
`box()`, `fit()`, `unscale()` and `settle()` are all kept by name, `settle`
whole at 200 ms with the divider's `pointerup` still calling it, and only
`draw`, `remint` and the object URL's minting, revocation and fragments go —
a coherent cut, `settle`'s first half being the only caller of `remint`. Gate
clause 3 was confirmed genuinely discriminating: at rest `cssWidth ==
paneWidth` and the backing store is `floor(paneWidth × dpr)` so they agree,
while during a gesture they diverge.

**Outstanding blocker 1 — the gesture moves the reader, and nothing governs
it.** Switching `fit()` from a transform to CSS `width`/`height` is right, but
it relocates round 1's pixel-offset problem out of the re-render and into the
gesture. `fit()` now reflows, so a gesture of factor *s* maps content offset
`T` to `T·s` while the browser holds `scrollTop` at `T` — the reader is
displaced by `T(s−1)` continuously, about 600 px on page 5 of the showcase for
a 20% widen, and WebKit implements no scroll anchoring that would compensate.
OQ-2 case 3 names the drag as a cause that "restores where the reader was",
but its capture point is "taken before the re-render", which under a 200 ms
settle is *after* the drag displaced them; the anchor then faithfully preserves
the displaced position. Gate clause 4 inherits the ambiguity — read "before the
drag" it fails a correct implementation of §4, read "after the drag" it passes
trivially. Prescribed fix: fix the capture point at **gesture start**, held
through the gesture and reapplied at the rest render, with clause 4 saying
"before the drag".

**Outstanding blocker 2 — gate clause 8 passes on the defect it exists for.**
Clause 8 was added in round 2 to sample OQ-5, and it is keyed to `current` /
`stale`. Those are compile-status words set in Rust from the compile alone;
the watch loop compiles on its own thread with no webview involvement, so in
OQ-5's scenario the header reads `current` whether or not a canvas ever
rasterised. A pane stalled behind another window shows the previous page under
a header saying `current`, and the clause passes. Its own text claims it "is
the only clause that can see it", which makes the closure void rather than
partial. Prescribed fix: key it to the drawn page — *the page on screen shows
the edited section* — not to the status word.

Non-blocking left standing, agreed across lenses: §2's `(decision, recorded)`
heading still reads "A gesture is carried by a **transform**" while §4 argues
against exactly that, and a heading is the surface a later pass cites; the
frontmatter's `reference:` still promises a `NOTICE` that `pdfjs-dist` 6.2.108
does not ship — **reported as folded in and it was not, an unguarded string
replace that matched nothing**; `unscale()` "resets the canvases to" inverts
for a canvas, which clears to its backing-store size, so the CSS size must be
*written* to the logical one; and `fitted`'s only writer today is the deleted
`draw()`, so the new render must be named as its writer.

### Round 2 — Phase 1 only — 2026-08-25 — panel of three, resumed — **NOT READY**

Two READY, one NOT READY. All seven of round 1's blocking findings confirmed
resolved against the files, including independent re-derivation of `settle`'s
call graph, `relines`' five call sites, and 454,669 + 1,262,398 = 1,717,067 B.

**Blocker, newly introduced by round 1's own fix:** §2's new "a gesture is
carried by a transform; a rest is answered by a render" decision re-adopted
`mpdf-003` Phase 7's five-part design in full, while §4 still deleted all five
parts by name and offered a `ResizeObserver` in their place — which has no
notion of moving versus rest, that being precisely what `settle` supplies.
§2 and §4 described different phases. Resolved by inverting §4's paragraph:
Phase 7's mechanism is mostly *kept*, each part named, with the gesture carried
by CSS `width`/`height` rather than a transform — argued from the consequence
that a transform leaves layout alone, so the container's extent and the
*(page, fraction)* anchor would be read stale.

Also folded in: gate clause 3 widened to run after a window resize, closing a
hole where a rest ending only on `pointerup` leaves a resized pane soft and
every other clause passes; clause 8 added for OQ-5; `PDFDocumentProxy`
destroyed on re-open; OQ-1 re-derived from the `tauri://` run; §2's path
corrected to `app/dist/pdfjs/`. Round 1's rule-4 → rule-6 "correction" was
itself corrected back: the clause describes the post-cut moment, where rule 4
matches first.

### Round 1 — Phase 1 only — 2026-08-25 — panel of three, fresh — **NOT READY**

**Round 0 (this episode): yes, with a named risk.** Phase 1 produces the
observable — the PDF Typst compiled, on screen, fitted to the pane at every
width — and the observable itself is untouched; only who rasterises it changes.
The challenge is that the pane works today and this spends 1.72 MB of vendored
JavaScript and three phases to replace it; the answer is that the fit-and-
sharpness problem was reported twice in use, its current answer is a workaround
the spec calls temporary, and the same change unblocks two platforms that are
otherwise dead. **The risk that could make it wrong is OQ-3**: the
accessibility regression is real, admitted and unmeasured.

Panel of three — correctness/grounding, scope/YAGNI, exit-gate testability.
All three NOT READY. Seven deduped blocking findings, **all accepted, none
rejected**:

1. §1.1 landed §6.1 step 1 on `mpdf-003` Phase 7 as "shipped work". Phase 7 is
   `shipped: null, reviewed: null` with no round in `mpdf-003`'s record; its
   code is on `main` as prototype. Re-landed on Phase 1's shipped **no bundled
   JavaScript PDF viewer** constraint, with Phase 7 reframed as what the edge
   *names* rather than what makes this a new spec.
2. OQ-2 was `needs-input` in §3 and "resolved in this phase" in §4, with a
   recommendation in place of a decision and no gate clause either way.
   Resolved into three causes keyed to what `refresh` already distinguishes.
3. The specified restore was a raw `scrollTop`, which is wrong precisely when
   the scale changed — the case it was assigned to. Now *(page, fraction)*.
4. "Every line of Phase 7's mechanism goes" would have taken `relines()` with
   `settle()` — the only width-change path that rebuilds Phase 8's line gutter
   — silently, with nothing in the repository able to see it.
5. What the reader sees *during* a width change was unspecified, and the naive
   reading blanks the pane: a `ResizeObserver` firing per frame against 94 ms
   of work, each render cancelled by the next, and a cancelled render on a
   just-resized canvas leaves it cleared.
6. The enabling measurement was taken over `tauri dev`'s
   `http://127.0.0.1:1430` with the **unminified** build — not the file
   vendored, not the scheme shipped. **Re-measured, not reworded**, against
   `cargo run -p md2pdf-app` at `tauri://localhost`, minified, worker real:
   import 29 ms, open 59 ms, page 1 42 ms at dpr 2, six pages 94 ms, 190 text
   items. OQ-5 records that one earlier run stalled fourteen minutes before its
   first render in a non-frontmost window.
7. "The pane's content width" named no element, and the width clause passed by
   construction under `scale = paneWidth / naturalWidth`. Pinned to the scroll
   container's content box, with the clause rewritten to test the ~15 px
   always-show-scrollbars case that can actually fail.

**Rejections worth recording.** Deferring the `CORRECTED` notes and the `cut`
to Phase 2, on the grounds that Phase 1 alone leaves the pane worse than
today: rejected, because the notes describe what Phase 1 itself makes true — a
JavaScript viewer *is* bundled the moment it ships — so deferring them leaves
the corpus stating something false for the interval. And the two Rust doc
comments citing `app/dist/index.html:caretPage`: no action, since OQ-2 case 2
keeps `caretPage` and its anchors.

**Numbers re-derived independently by the panel**: `samples/showcase/showcase.md`
compiles to `/Count 6`; its outline carries six entries; `pdf.min.mjs` +
`pdf.worker.min.mjs` = 1,717,067 B (1.72 MB / 1.64 MiB); `pdfjs-dist` 6.2.108
ships a `LICENSE` and no `NOTICE`; `cargo test --workspace` green on a clean
tree.
