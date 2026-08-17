# Review record — mpdf-005 (`specs/captions_and_references_spec.md`)

Append-only. One heading per round, newest first.

### Round 4 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY (past the cap, by the human's decision)**

Verdict: `NOT READY`, one blocking. **Run past §7.6's three-round cap on the
human's explicit authorisation**, recorded here because going past the cap is a
decision a person makes.

**Round 3's blocker was accepted in its conclusion and rejected in its premise,
and the reviewer confirmed the rejection against the code.** Rounds 2 and 3 both
claimed a held image call would be lost — at the document's end, then at a
footnote region's end. `core/src/emit.rs:Walk::finish` drains `pending` itself,
under a doc comment that says why it exists, and both walk-ending sites go
through it. **No image is dropped at either site**, and the drain the spec had
promised to add would have fixed a bug that does not exist.

What survives is smaller and real: `finish` destructures `(call, _)` and writes
`write_image(…, false)`, discarding the form. Held one paragraph longer, a
standalone image reaching `finish` is demoted to `#box(image(…))` — visible
rather than silent, and a one-line fix in the one place both sites share. Round
3's conclusion also survives: two sites, and the footnote one covered by nothing,
since `tests/fixtures/footnotes.md` ends its definition with a list.

**The new blocker is the second consecutive one caused by perturbing the flush
timing**, which is a fact about the design rather than about the prose.
`Walk.para` is a buffer *offset*, not a flag —
`core/src/emit.rs:step` computes `opened: *para == Some(top(bufs).len())` against
the length recorded when the paragraph opened. Holding the flush past
`Start(Paragraph)` means the held call is appended *after* `para` was recorded,
so the offsets no longer match and **the second of two consecutive standalone
images is demoted to `#box(image(…))`**. Re-derived by the author against the
shipped binary: both are bare `#image(…)` today. An `awk` sweep of
`tests/fixtures/*.md` and `samples/*.md` finds **no two consecutive standalone
image paragraphs anywhere**, so no gate case reaches the shape.

One non-blocking: gate (3a)'s "a new fixture case" does not say which file, and
adding the footnote-final image to `tests/fixtures/footnotes.md` would move a
shipped golden and fail gate (7). Outstanding at escalation.

**Escalated again rather than folded.** Two rounds running, the blocker came from
the deferral colliding with machinery it was not designed against, which is
evidence about the mechanism; the author's recommendation to the human is to
replace the deferral rather than patch it. `status` stays `draft` and Phase 1's
`reviewed` stays `null`.

### Round 3 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY (escalated at the cap)**

Verdict: `NOT READY`, one blocking, at the loop's three-round cap. Round 2's
blocker was verified resolved in the files — both sketches carry the blank line,
§2 records the rejected spelling with its measurement, the attachment subsection
no longer says "the next event", and gate (3) pins the rejected form so an
implementer who "fixes" it by widening the standalone test fails there.

**The new blocker is the second instance of the fix's own hazard, and it is the
third time this episode that a fix introduced one** — the pattern §7.3 names.
§2 and Phase 1's scope both say `core/src/emit.rs:emit` gains a post-loop drain
for the held call. **`emit` is not the only site.**
`core/src/emit.rs:collect_definitions` handles `End(TagEnd::FootnoteDefinition)`
and `continue`s *before* dispatching to `step`, taking the body with
`std::mem::replace(&mut walk, Walk::new()).finish()` — so a call still held at a
footnote region's end is discarded with the walk and `step` never sees the
closing event.

Reachable today, and re-confirmed by the author against the shipped binary
rather than taken from the changelog: a definition whose last block is a
standalone image emits
`#footnote[Some text.\n\n#image("dot.png", alt: "alt")]`. Under the longer
deferral that image vanishes while `core/src/lib.rs:image_paths` still returns
its `ImageRef`, so the CLI reads a file whose image never reaches the page —
the silent-drop class `mpdf-001` §2 exists to refuse, which §2 line 343 cites by
name for the *first* drain site.

**No gate case can see it**, which is what makes it blocking rather than a note:
(3a) is scoped to a document-final image in `tests/fixtures/images.md`, (2) to
`tests/golden/images.typ`, and `cargo test --workspace` is silent because
`tests/fixtures/footnotes.md`'s definition ends with a list, not an image —
verified at lines 20–21.

Three non-blocking, all outstanding at escalation: OQ-1's resolution line still
records the pre-round-2 wording and the wrong round, which is the same
stale-summary class that produced round 2's blocker; §2's "this needs no new
state" predates the deferral design and is now inaccurate, since the held call
carries its verdict across events; and the consumed caption paragraph leaves two
`'\n'` separators already pushed into the buffer by
`core/src/emit.rs:step`'s paragraph arms, which the implementer must unwind.

**Escalated rather than looped, per §7.6.** `status` stays `draft` and Phase 1's
`reviewed` stays `null`: the date records convergence, and this phase has not
converged. Going past the cap is a decision a person makes.

### Round 2 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`. All four of round 1's blockers resolved, and **one new
blocker the author's own fix introduced** — §7.3's named hazard, and the reason
that step exists.

Round 1's B-2 was resolved by choosing a caption syntax: a `: ` line after the
construct. **The reviewer falsified it by running the parser rather than by
arguing.** With no blank line that is *one* paragraph, not two — the events are
`Start(Paragraph)`, the image, `SoftBreak`, `Text(": …")`, `End(Paragraph)` — so
`core/src/emit.rs:step`'s standalone test, which looks for `End(TagEnd::Paragraph)`
as the next event, answers false. The spec's own flagship example therefore
converted to the **inline** `#box(image(…))` plus a literal `: The conversion
pipeline…`, and the document contradicted itself: the syntax prose specified the
no-blank-line form while the attachment mechanism specified the blank-line form.

Resolved toward the blank line — the caption is a paragraph of its own — because
the prettier spelling is not a syntax choice but a change to the standalone test,
the one discrimination every image in the dialect flows through, and that is the
blast radius Phase 1 is scoped to avoid. The author reproduced the measurement
against the shipped binary before choosing.

Two non-blocking, both accepted: a longer deferral needs a drain, because
`core/src/emit.rs:emit` has no post-loop flush and `tests/fixtures/images.md`
ends with a standalone image (this became round 3's blocker at its second site);
and gate (6)'s caption needles were moved off
`every_bundled_template_meets_the_call_contract`, which is named for the
five-argument contract OQ-3 had just resolved does not widen.

### Round 1 — Phase 1 only — 2026-08-15 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — a new spec with no shipped phase, per §7.0).** *Does
this phase produce the observable, and is it the right one?* Yes: the typeset
PDF, with a captioned, numbered figure under an image. **The risk recorded rather
than dismissed:** the conversation that prompted this spec asked about *labels*,
and captions arrived because Typst's `figure` carries both and numbers by
default — so Phase 1 delivers the carrier while the function that motivated the
request lands in Phase 3, which is the shape `mpdf-004` Phase 3's round 0 flagged
about numbering without referencing.

**Scope note.** §7 says a spec with no shipped phase takes one document-wide
round; this episode was scoped to Phase 1 at the human's explicit instruction,
which `loops/review-spec.md` honours. Phases 2 and 3 are deliberately sketches
pending OQ-1 and OQ-2, so a document-wide round would have rediscovered what the
draft already says.

Verdict: `NOT READY`. Four blocking, eight non-blocking, **all twelve accepted;
no rejections in this episode.**

**Two blockers were the phase not being buildable at all.** Phase 1 carried no
exit gate — "to be written when OQ-1 and OQ-3 resolve" — which §3 requires of
every phase and §7 rule 2 names as blocking by definition; and OQ-1, the caption
syntax, was Phase 1's entire input surface and the spec itself said it blocked
the phase. The gate became eight cases; OQ-1 was split, its caption half resolved
and its reference half moved to OQ-8 against Phase 3, which is what unblocked
Phase 1 without pre-deciding the harder question.

**The third was measured, and it inverted the phase's blast radius.** Wrapping
every standalone image was undefined against wrapping only captioned ones, and
the reviewer measured both costs of the former: an uncaptioned `#figure` prints
no number but **still consumes the counter**, so the next captioned figure reads
"Figure 2" with no Figure 1 on the page, and `figure` centres its body where a
bare block sits flush left — a bounding box moving from `xMin=70.87` to
`277.48`. That violates `mpdf-004` Phase 3's stated property, "no document's
typeset output changes unless its author asks". Resolved: the caption is what
makes it a figure. The fourth was the caption content model — whether caption
text is walked as markdown, what an empty caption does, what a `: ` line
following nothing does — none of which any OQ carried.

**The round re-derived every number and corrected one.** OQ-6 had guessed "the
second-largest golden movement after `mpdf-004` Phase 3's seventeen"; the count
is **one** golden with a standalone `#image(` and **one** with a `#table(`, so
the unconditional wrap would have been the *smallest* movement in the record, and
with the third blocker resolved it is **zero** — no fixture carries a caption
line, so Phase 1 can assert "no shipped golden file changed". OQ-3 was answered
from the code in the same round: no sixth look argument, since both looks already
reach `raw` and `table.cell` with `show` rules and no export.

Grounding confirmed rather than corrected, so a later round need not re-verify:
18 golden files, and 17 at `d6d9edc^`, corroborating `mpdf-004` Phase 3's
"seventeen"; §2's measured Typst table reproduced exactly against a probe crate
built on the pinned 0.15.1, including the labelless
``label `<nosuchthing>` does not exist in the document``; `mpdf-002` §1.1's
non-goal and its "bare `#image` is the hook" sentence verbatim; and §1.1's §6.1
working sound on all four steps, so `extends: null` and `supersedes: null` stand.

Five smaller corrections were accepted and folded: `table_call` writes `#table(`
**with** the `#` where `image_call` omits it, which is the asymmetry Phase 2
trips on; the caption attaches at the `pending` flush in `core/src/emit.rs:step`
and not in `write_image`, which cannot see a later event; "`figure` appears
nowhere" narrowed to the Typst *element*; the "changes nothing in `cli` or `app`"
claim narrowed to `cli/src` and `app/src`, with the two tests in the blast radius
named; and OQ-7's grounding corrected —
`core/src/emit.rs:collect_definitions` walks a footnote body **once** and keeps
it, the document walk skips the region, and what is discarded is the headings
vector.
