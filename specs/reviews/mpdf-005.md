# Review record — mpdf-005 (`specs/captions_and_references_spec.md`)

Append-only. One heading per round, newest first.

### Round 4 — Phase 12 only — 2026-09-01 — the consistency lens, resumed — **READY (past the cap, by the human's decision)**

Verdict: `READY`, zero blocking. **Converged at round four**, scoped to the round-3 blocker alone,
with the other two lenses already `READY` at round three. Phase 12's `reviewed` is set to
2026-09-01; `status` was already `accepted`. This is the second time this spec has gone past the
cap, and both times it was a person's call and is logged as one.

**The blocker's fix verified against the files.** The struck close-out item is restored as the
eighth code site, quoted to match `core/tests/golden_test.rs:2754-2756` once the comment wrapping
is flattened, with the split stated — the comment is owed an edit and the assertion is not — and
the replacement reason given against the *inline* half, where `: A line. {#eq:one}` stays prose
because `equation_name` finds a group that is not the whole of its run.

**The instrument generalisation was re-run by the reviewer and reproduces on all three sentences**,
and it found a fourth the close-out did not need to enumerate: the same phrase in
`core/src/emit.rs:Equation`'s doc comment also greps to zero, wrapping at 181-182. Every one of the
zeros is a wrap artefact and every sentence is at the site named.

**The one instrument that failed three times in this episode is the episode's transferable
finding**, and it is now written into the phase rather than only into this record: a line-based
grep over a phrase that may wrap struck a correct close-out item at round 2, produced the round-1
README mis-attribution that round 2 reversed, and under-reports the close-out's own quotations. A
quoted sentence locates a site; it is not itself a search string.

### Round 3 — Phase 12 only — 2026-09-01 — the same three lenses, resumed — **NOT READY (escalated at the cap)**

Verdict: correctness `READY`, exit-gate `READY`, consistency `NOT READY` with one blocker. **Both
round-2 blockers verified resolved against the files by all three.** The cap is reached, so the
phase's `reviewed` is **not** set and the outstanding item goes to the human.

**The one blocking finding is real, is fixed, and is unverified by a reviewer.** The close-out had
claimed that *"the group is a paragraph away from the equation rather than adjacent to it"* is
"nowhere in" `core/tests/golden_test.rs:a_marker_line_after_a_display_equation_is_still_prose`,
and struck the item on that basis. **The sentence is in that test**, as an inline comment wrapping
across two source lines, which is precisely why `grep -rn "paragraph away"` returns nothing from
that file. Confirmed with a multiline scan. The comment is owed an edit and the assertion is not.
The item is restored, the instrument is named, and the same wrap is recorded as the reason
`grep -cF` returns zero for two other sentences this close-out quotes — so a quoted sentence here
locates a site and is not itself a search string.

**The severity split is 2:1 and is recorded rather than resolved by the author.** The two READY
lenses raised the same finding as non-blocking prose, on the ground that it changes neither what is
built nor what the gate catches; the consistency lens called it blocking because the close-out
instructs an implementer not to fix a comment that is stale. Both readings are defensible and the
author did not adjudicate the severity down to avoid the cap.

**One instrument failed three times in this episode and the record says so**, because it is the
transferable finding: a line-based grep over a phrase that may wrap. It struck a correct close-out
item at round 2, it produced the round-1 README mis-attribution that round 2 then reversed, and it
under-reports two of this close-out's own quotations.

Non-blocking folded at the cap: the code-site count, which read four, then seven, then six, and is
eight with the restored item; gate (5)'s residual discrimination, which over-claimed by one variant
(a run-scoped refusal that kept `caption.is_none()` leaves the case green, and gate (4)'s inline row
is what catches run-scoping in every variant); and `rules/pipeline.md`'s split into five passages
that change and one that is checked and does not.

### Round 2 — Phase 12 only — 2026-09-01 — the same three lenses, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY` from all three. **All three converged independently on one blocker, and it was
the author's own round-1 fix.**

**Rescoping the refusal from a run to a paragraph invalidated the case that justified it, and the
measurement was carried over unre-taken.** `core/src/emit.rs:step`'s caption-marker arm sets
`*para = None` before pushing the caption's frame — *"a caption is not a paragraph"* — so `opens` is
false for every run inside a caption and `caption.is_none()` is unreachable. Measured by building
the refusal with no caption test at all: the showcase compiles and `{#tab:kinds}` still leaves the
page. Three claims fell with it — the guard's load-bearing status, gate (5)'s discrimination, and
the census, which had counted groups alone on a *line* and found one where the rule's own unit
finds **zero**. The census moved in the safe direction: the narrowing touches nothing that ships.

**The exit-gate lens found the sharper one.** Gate (3)'s row 6 is the only case in the phase that
reaches the `:::`-opener retirement, and it tests it only if its document carries a live equation
*before* the opener — without one, a bare `::: figure` / `{#eq:one}` errors under every build and
the row passes green with the scope item unbuilt. The document is now written out with that
reasoning attached.

**Round 2 also reversed a round-1 fix**, which is why the record keeps both: round 1 placed a README
site in `## Background`, the author accepted it, and round 2 found `## Background` is sample text
*inside* the fenced example at 256–308. The original attribution was right.

### Round 1 — Phase 12 only — 2026-09-01 — three fresh lenses (correctness/grounding, exit-gate testability, cross-file consistency) — **NOT READY**

Verdict: correctness `NOT READY`, consistency `NOT READY`, exit-gate `READY`. **Round 0, asked once
for this episode: yes.** The observable is the typeset PDF; today a document naming an equation on
the line below typesets the literal `{#eq:one}` and its reference fails, and after this phase that
page carries a numbered equation and a sentence reading "Equation 1". The warrant is an
inconsistency *inside* the shipped dialect — a caption's name already may stand on a continuation
line, and does so in the corpus — plus a silent drop this project's own rules forbid.

**Four blockers.** The `:::` opener retires `*figure` and not `*equation`, so the widened tail test
reaches across a block boundary and the insert lands before `Group.start` — measured, and the
author's own trace corrected the finding's framing: the message is identical before and after, so it
is a stale offset and a wrong message rather than a regression. The refusal, scoped to a whole
*run*, refused `An inline $x + 1$ {#eq:inline}`, which **Phase 4 decided stays prose** and which this
phase's own gate (4) asserted stays prose — the phase contradicting itself. The append-to-insert
change falsified the "every write into a `bufs` frame is an append" invariant stated in both
`core/src/emit.rs:Figure`'s struct doc and `rules/pipeline.md`, neither named. And the close-out
edited `samples/showcase/sections/mathematics.md` without naming `rules/desktop-panes.md`'s 1140
text items or the hand-run six-page gate — resolved by *declining* the edit, since three sentences
are incomplete rather than false and now all take the same call.

**The run-to-paragraph fix introduced its own consequence**, folded rather than left: a tight list
item has no paragraph, so `- {#fig:one}` stays prose where a loose item's and a block quote's are
refused. Fourteen further non-blocking findings folded.

### Round 3 — Phase 11 only — 2026-09-01 — the same three lenses, resumed — **READY**

Verdict: `READY` from all three, zero blocking. **Converged at the cap**, the fifth time this
spec has. Phase 11's `reviewed` is set to 2026-09-01; `status` was already `accepted`.

**Both round-2 blockers verified resolved against the files.** The truncate count was
re-measured independently by the lens that raised it: **four** `.truncate(` calls in
`core/src/emit.rs` — `splice_caption`, `take_member`, `close_group`, `close_abstract` — so
`rules/pipeline.md`'s "this file's two truncates" has been wrong **since Phase 5**, before this
phase touched anything, and a keywords closer makes five. It is named as already-stale rather
than as this phase's arithmetic. The three look-file prose claims are named in a paragraph of
their own, and the recorded call on the sharpest of them is to **correct the reason and not
re-tune the number**: a keywords float makes `template.typ`'s `clearance: 2em` an internal
front-matter gap rather than the front-matter/body boundary its comment records, and which gap
should be largest is a look's own call that no gate here reads.

**The gate lens's blocker closed with a measurement that also validated the fix's shape.**
`join(` appears three times in each look already, inside `#let template`, above both files'
exports — so the whole-file needle passed on an unmodified tree and the slice-scoped one is
clean.

**Two corrections this round ran against the author.** Gate (8) claimed an over-cap rule file
is an error; `spec-lint` emits `RULE_OVER_CAP` as a **warning**, with the comment "setting a cap
does not require rewriting a rule to fit it", so the gate is recast as one-warning-not-two and
the cap raises rest on the numbers. And refusal 6 cannot draw its nine names from
`core/src/emit.rs:describe`, which collapses seven of them into two strings and carries a
doc-comment invariant that would break.

Four non-blocking folded at convergence, three of them internal contradictions the fold itself
had introduced: gate (3) still required a nesting row's opener to stand first, which the
closer-side refusal 5 was chosen to retire; the close-out's opening still carried the severity
gate (8) had just corrected; the ten `rules/pipeline.md` passages had been counted as eleven by
naming one paragraph twice; and refusal 5's blank-line test needed saying that it runs on the
*trimmed* region, since the region ends in two separator newlines and a raw test would refuse
every well-formed block.

### Round 2 — Phase 11 only — 2026-09-01 — the same three lenses, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`. Correctness returned `READY`; the other two returned `NOT READY` with one
blocker each, both newly found rather than re-raised.

**The gate lens found gate (5)'s look half vacuous.** `BUNDLED_TEMPLATES` needles are
whole-file `contains`, and both looks already carry `join(` three times for the author block —
so the needle passed before the phase was implemented at all, and would pass a look that
hardcoded `terms.at(0) + ", " + terms.at(1)`. Scoping it to the slice at `#let keywords(`
closed it.

**The consistency lens found the close-out short in two places.** `rules/pipeline.md` carries a
*twin* of the code comment the close-out names — "The dispatch is therefore three-state", the
same sentence one file over, read by nothing mechanical — plus two adjacent counts. And **both
look files carry front-matter prose**, which a draft had treated as gaining code rather than as
carrying claims: the two-floats sentence, the `clearance` rationale, and the press release's
"the first thing in the body".

**One round-2 fix introduced an error the author caught by re-measuring in the same pass** —
the pattern the loop warns about. The changelog claimed gate (1) pins the hyphen *unescaped*
inside the brackets; `-` is in `SPECIAL`, so the walk escapes it, and the golden reads
`([cross\-references], [C\# and C\+\+], …)` — one backslash each, unquoted. Corrected in the
file before the reviewers judged it, and both lenses confirmed the corrected bytes are what the
emitter produces.

Nine non-blocking folded, including refusal 5's detection site: both event-level placements
refuse a *well-formed* block — `Start` counting catches the closer's paragraph, `End` counting
the opener's — so the closer is the only site that works, which incidentally made gate (3)'s
five nesting rows reachable unconditionally.

### Round 1 — Phase 11 only — 2026-09-01 — three fresh lenses (correctness/grounding, exit-gate testability, cross-file consistency) — **NOT READY**

Verdict: `NOT READY` from all three. **Round 0, asked once for this episode: yes.** The
observable is the typeset PDF and the phase puts a list of index terms on the first page that no
markdown in this dialect can ask for. The warrant is convention rather than a defect — no page
is currently wrong, which is Phase 9's weaker footing — but the requirement is real in the
venues the article look serves and OQ-20 pre-committed to answering the family question at
exactly this word.

**The blocker all three lenses converged on, and it changed the design: the terms cannot cross
as Typst string literals.** The closer reads a region the walk has *already* markup-escaped, and
`-` is in `SPECIAL` — measured, `cross-references; C# and C++; a_b` reaches the buffer as
`cross\-references; C\# and C\+\+; a\_b`. Quoted, those are not string escapes at all;
escaped a second time they compound. **A hyphenated keyword is the common case.** The terms now
cross as an array of **content**, on `splice_caption`/`close_group`'s precedent rather than
`typst_authors_or_none`'s, with **no escape running in this phase at all** — which also makes
OQ-21's widening a change to one refusal rather than to the crossing.

**And the gate could not have caught it**: the fixture's only special character was a comma,
which nothing escapes. It now carries a hyphen and a `#`, with the exact bytes written into
gate (1) and the two wrong builds it discriminates against named.

**Three more blockers, each found by two lenses or traced by one.** A `: ` line inside keywords
was not refused in the sole-paragraph shape — `attaches` is false, so it fell through to
`escape_into` and was emitted as a term — which became a ninth refusal. "Nothing about the
marker or the dispatch is rewritten" contradicted the code: the dispatch gains a dimension, arms
and a table lookup. And the close-out named none of the code sites where Phase 10 had named
five, none of the user-facing statements of the abstract's old position rule, and not
`samples/showcase/sections/figures.md` — **the same file and the same sentence Phase 10's own
close-out missed**, which is why this round's fold names the pattern rather than just the file.

**Step 1 was one finding short.** The phase narrows a permission — `::: keywords` is a legal
figure-group opener today, on a census that finds the word nowhere — and it also **widens** the
abstract's shipped first-block refusal, which a draft did not say. Refusal 6 was allotted one
gate row for seven constructs against the shipped abstract table's sixteen; it now names nine
spellings, each with a row, and gate (3) went from nineteen rows to twenty-eight.


### Round 3 — Phase 10 only — 2026-08-31 — the same three lenses, resumed — **READY**

Verdict: `READY` from all three, zero blocking. **Converged at the cap**, the fourth time this
spec has. Phase 10's `reviewed` is set to 2026-08-31; `status` was already `accepted`.

**The round-2 blocker verified resolved by all three independently, each against the code
rather than the changelog.** The correctness lens closed the discriminator question
exhaustively: **exactly two `step` call sites exist crate-wide** — `Mode::Definition` inside
`collect_definitions` and `Mode::Document` inside `emit` — and **exactly four `Walk::new()`
sites**, all inside those same two functions, so a fresh walk is reachable only under
`Mode::Definition` and no third path creates one. It further established that the two modes
*partition* the stream rather than overlapping (`Notes::enter` either raises or sets
`skipping`, and `step`'s first guard then returns early to `End(FootnoteDefinition)`), and that
the three conditions are **complete** — a GFM table cell holds inline content only, so a list
item and a block quote are the only other frame pushers a `:::` paragraph can stand in. It
looked specifically for a fourth hole and found none.

**Two facts the author had not claimed, both verified and folded.** `Notes::enter` re-raises a
definition's stored error, so refusal 3 fired inside a footnote still carries the opener's own
line; and `sections::assemble` joins before `emit` runs, so a section file is walked in
`Mode::Document` and gate (7) is untouched by the new condition.

**The exit-gate lens answered the round's targeted question and the answer changed a
rationale rather than a rule.** Gate (3)'s three refusal-3 rows are **not** necessarily
distinguishable by `construct` string, and it does not matter: they discriminate by
*behaviour*, because a two-condition implementation fails the footnote row **by compiling**.
The phase's stated reason — that the three conditions fail independently — was one condition
short of true, the first two both failing `bufs.len() == 1`, and is corrected.

Nine non-blocking folded at convergence. The sharpest: gate (3) carried **two** totals in one
sentence and **neither was right** — the arithmetic is fourteen rows and is now spelled out,
this being the third round running in which a count in this phase moved without its sentence
following. The round-2 fold had also introduced **two bare `file:line` citations**, which §5
forbids outright ("a line number is not a citation"); both are replaced by the symbolic
anchors already beside them. Refusal 3's round-2 title read as a superset of refusal 2's and
is retitled to the disjoint case it is. `core/src/emit.rs:Marker`'s doc comment joins the code
sites as a fifth — quoted by the phase's own scope and made incomplete by the three-state
dispatch. `rules/pipeline.md`'s "Two caption shapes are errors" is **deliberately left at two**,
the call named rather than left implicit, on the same judgement that leaves the citation
channel's count alone. And the footnote gate row's definition **must be cited**, or
`Notes::enter`'s uncited-definition refusal fires first and the row asserts the wrong string.

### Round 2 — Phase 10 only — 2026-08-31 — the same three lenses, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`. Correctness returned `READY`; the other two returned `NOT READY` with
**the same single blocker, found independently**, and **it was the round-1 fix defeating
itself** — the pattern the loop warns about, and the second episode running in this corpus.

**The blocker: the first-block test admitted an abstract inside a footnote definition.** Round
1 closed a frame hole by specifying the test as content **and** `bufs.len() == 1`, argued from
list items and block quotes. `core/src/emit.rs:collect_definitions` runs every definition
through a fresh `Walk::new()`, so inside one `top(bufs)` is empty *and* `bufs.len() == 1` — the
fix written to close the frame hole opened the definition hole. An abstract there emits
`#abstract[…]` inside `#footnote[…]`, which the look hoists to the page top: the
source-order/page-order divergence the rule exists to prevent.

**It compounds through the import flag**, which is what made it a compile failure rather than
only a layout one. `walk.math` is lifted onto `Body` and ORed back at the reference arm as
`*math |= body.math`; an abstract flag has no such path, so left unrefused **and**
unpropagated the emitted `#abstract[…]` reaches the compile with no `abstract` in the import
list — a failure naming a Typst identifier the author never wrote, which is the labelless
failure §2 forbids.

**The fix cost one clause**, the exit-gate lens having observed that `step` is already handed
`Mode::Definition` at the `collect_definitions` call site and already matches on it. The test
became three conditions; gate (3) grew to fourteen rows. **Refusing was taken over
propagating, and the alternative is recorded** because it is the one an implementer reaches
for: propagation would make a document's import line depend on whether a footnote was
*referenced*.

Fourteen non-blocking folded, including one **all three lenses caught** — the close-out still
argued its cap raise from "six refusals" after the list grew to eight.

### Round 1 — Phase 10 only — 2026-08-31 — three fresh lenses (correctness/grounding, exit-gate testability, cross-file consistency) — **NOT READY**

Verdict: `NOT READY` from all three. **Round 0, asked once for this episode: yes, and it is the
right thing to build.** The phase produces the observable directly — a first page carrying an
abstract set across the full page width above two columns, a shape no markdown in this dialect
can ask for at all, which is Phase 5's "shape rather than treatment" test. An abstract is the
most standard element of a paper this dialect cannot express, and OQ-20 refuses to build the
framework around it. The recorded reservation: it lands in a spec named
*captions-and-references* and an abstract is neither, so §6.1 step 2 is argued in the phase
rather than assumed.

**Five deduped blocking findings.**

1. **The label spelled as a `heading` element silently withdraws every anchor.**
   `core/src/lib.rs:anchors_from` returns an empty vector on any mismatch between walked and
   typeset headings, and `render` derives the count by querying `HeadingElem`. A heading-shaped
   label is counted, mismatches, and takes the desktop pane's scroll sync with it — with
   nothing on the page to show. It would also take a section number and restart
   `figures: sectioned`, re-entering through the look the exact defect the phase refuses a
   heading-shaped abstract to avoid. **`core/assets/template.typ`'s `show bibliography` rule
   already records this trap and already chose styled text for it.** Two lenses reached it from
   opposite ends — one from `anchors_from`, one by building the wrong look and measuring
   `1 Abstract` / `2 Introduction` / `Table 2.1` / no anchors. Fixed by pinning the label to
   `text()` and by a new gate case asserting all three at once.
2. **Gate (5) was falsified by a correct build.** Measured, a five-line abstract gives boxes of
   362.83, 362.83, 71.29, 362.83 and 205.37 against body lines of 217.70 — so "the abstract's
   line boxes are wider than a body line's" is false for two of five. Rewritten to
   widest-against-widest. The fixture's prose was also unspecified, so a correct build with
   short sentences would have failed the gate; it now pins the property, on `mpdf-001` Phase
   10's precedent.
3. **Refusal 6 reused `Walk::unclosed`'s literal**, reporting *"figure group the document never
   closes"* for an unclosed `::: abstract` — the same defect the phase fixes one sentence later
   for a different message. All three lenses raised it. `unclosed` gains a second literal and
   `escaped_frame` moves with it.
4. **The close-out missed `README.md`'s "A word after the opener is yours, and `md2pdf` does
   not read it"** — the user-facing twin of the §2 sentence the phase corrects — and pointed at
   a "constructs table" that does not exist.
5. **Four documents assert the showcase uses *every* construct**, all false the moment the
   dialect gains one the showcase lacks; the fourth sits above `app/src/preview.rs`'s pinned
   `[14, 29, 68]`, so its rewording must be line-count-neutral.

**Every number the phase was keyed to re-derived exactly**, by two lenses independently: 217.70
and 362.83 (595.2756 − 2×70.866 = 453.54 text width, ×0.8, and less a 4% gutter halved), thirty
goldens, `rules/pipeline.md` at 1080/1080, `Table 2.1` reproduced, every cited symbol and
literal verified. Two were corrected: the title block runs y 66.60–**153.22**, not to 142,
which was the date line's *yMin* read as the block's bottom; and the stacking claim was
re-measured at the shape that ships — `#abstract[…]` issued from the body rather than injected
into the look — where it holds.

**Refusals went six to eight**, two of them round-1 catches: an abstract in a list item or
block quote (a fresh frame, so a content-only test reads it as first), and a **standalone
image**, which *is* a paragraph at event level and so slipped the block rule — it would set a
live `Figure` and let a following `: ` line splice a figure inside the float.

**OQ-19's own risk argument was inverted.** Of its three examples only the list is a block; a
display equation and a citation are inline events inside a paragraph, so the stated refusal
already permitted them. Both are now refused by name, and the question is about widening again.

**One deferral, recorded rather than accepted.** `web/index.html`'s "Twenty-three constructs
are supported" goes to `mpdf-006` as a logged gap with its line named, per §6's "or the gap
logged": that page's claims must be compiled snippets under that spec's gate, and correcting
the count without the snippet leaves it asserting a construct nothing compiles.
`rules/web-demo.md`'s identical count **is** fixed here, a rules file tracking the code.

**The fold introduced a defect of its own** — renumbering the gate broke two cross-references —
which is the "a fix can introduce a blocker" pattern; both were caught before round 2 opened,
and round 2 then found the larger instance of the same pattern.

### Round 3 — Phase 9 only — 2026-08-31 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, **zero newly found**, one courtesy non-blocking folded at
convergence. **Converged at the cap**, the third time this spec has, and again because the
round that would have escalated is the one that cleared.

All three round-2 findings confirmed **against the files**, and both arithmetic asks
answered by counting rather than by assent: the close-out's list has exactly six items under
"six claims", the round-1 version listed exactly three, and all six quoted strings were
matched against `rules/pipeline.md` on whitespace-normalised text, since every one of them
wraps across lines in the file. Gate (2)'s citation now resolves, and the reviewer
additionally established the claim behind it rather than taking it: the twins' edges really
are identical in the 4-and-5 configuration, because the caption's `pad` does not touch the
body.

**The courtesy note, folded rather than deferred, and it carried a second defect the round
did not see.** `core/tests/golden_test.rs:BUNDLED_TEMPLATES`' doc comment says "Four tests
read these"; five do today and six will after gate (4). Pre-existing staleness this phase
does not create, corrected because the implementer is in that header anyway. The author then
found that the same sentence names "the three **Phase 9** cases" meaning `mpdf-001`'s Phase
9 — a bare phase number, in the one file this spec's own Phase 9 edits, which is §3's
"one numbering per spec" hazard reached across two documents. The spec id goes in.
**Folded at convergence without a fourth round**, per §7.5's "fold in worthwhile
non-blocking refinements": both edits are one-line and neither touches a gate.

**Converged.** Phase 9's `reviewed` is set to 2026-08-31; `status` was already `accepted`.

### Round 2 — Phase 9 only — 2026-08-31 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, three non-blocking, all three folded. **All three were
defects the author's own round-1 folds introduced**, which is §7.3's named hazard and the
third consecutive episode in this spec's record to hit it.

1. **Gate (2) cited §2 for a number §2 did not carry.** The fold wrote "§2 records a
   configuration where identical edges reported as 4 and 5"; that configuration was the
   *reviewer's*, and the string appeared nowhere in the document. Fixed at the §2 end rather
   than by dropping the citation, because the measurement is real and belongs in the record.
2. **§2 contradicted itself one paragraph apart.** The fold claimed "three relations, every
   one of them an equality", while its own bullet recorded the third as a *move* from column
   25 to 27. The third is a **negative** — the image-first group's listing stays centred —
   and §2 now says so, with the note that no gate is keyed to it and OQ-18 owns it.
3. **A sixth stale claim in `rules/pipeline.md`.** Line 898's "The fifth is the only
   `.where(kind: …)` *rule* in either look" becomes false the moment
   `show figure.caption.where(kind: raw)` lands. It is a distinct claim rather than something
   the rule-count clause covers, so the close-out names it and counts six.

**Both of the author's direct asks were answered by measurement.** The narrowed needle
`figure.where(kind: raw): set align(left)` is byte-identical in both looks
(`core/assets/template.typ:143`, `core/assets/press-release.typ:131`), occurs exactly once
per file against the old selector's twice, and was simulated in both directions: with Phase
6's rule deleted the **old** needle still passes in both looks and the new one fails in both.
And gate (2) did not become unfalsifiable — its ±1 tiebreak resolves against the rendered
page, which is stricter than the grid, and cannot absorb a real failure, since the two
figure-level spellings land the captioned listing 5 and 11 columns off its twin.

### Round 1 — Phase 9 only — 2026-08-31 — fresh general-purpose reviewer with repo access — **READY**

**Round 0 (this episode — one appended phase):** *does this produce the observable, and is it
the right one?* **Yes to the first** — a PDF whose block code stands 2em off the prose's
edge. **The second is recorded rather than glossed**, because it is the weakest warrant this
spec has run a phase on: it was asked for directly, which is the precedent `mpdf-006` OQ-3
and `mpdf-007` OQ-6 set, but it is the first phase here whose want is *legibility* rather
than a defect — and Phase 6's own round 0 praised that phase for "taking something off the
page rather than adding a capability nobody asked for", which this puts back. §2 states it
plainly rather than dressing it up.

Verdict: `READY`, **zero blocking**, four non-blocking, all four folded, none rejected.

**The round reproduced the phase end to end rather than trusting its measurements** — built
the tree, installed the rules in both looks, compiled a probe carrying all six shapes, and
read it with `pdftotext -layout`. Every load-bearing claim reproduced: the twins moving
together 0 → 4 in both looks; the second rule being necessary, the caption sitting at 0 under
a block at 4 without it; **both** figure-level spellings reintroducing Phase 6's defect
(`pad` → 5 against 0, `set block(inset:)` → 11 against 0), so gate (2) catches each; Phase
6's rule not being subsumed, its removal re-centring the captioned listing to 13/16 against a
twin at 4; and gate (5) holding, the whole workspace suite green with both rules installed,
`the_articles_last_heading_is_not_on_the_first_page` included.

**The sharpest finding is a defect in shipped work rather than in the draft, and it had been
unable to fail for eight days.** Gate (3) leaned on
`core/tests/golden_test.rs:every_bundled_template_places_a_listing` to show Phase 6's rule
survives. Its needle is `figure.where(kind: raw)` — and **Phase 7's counter reset writes
`counter(figure.where(kind: raw)).update(0)` into both looks**, so the string occurs twice
per file and the test passes with Phase 6's `set align(left)` deleted outright, while its own
doc comment still calls it "the first `.where(kind: …)` rule either look carries". The phase
now repairs it: the needle narrows to the whole rule. Taken as in scope deliberately — the
gate leans on the assertion, and an assertion that cannot fail is not one.

The other three, all folded: **§2's absolute columns are probe-specific and not
reproducible**, since `pdftotext -layout` re-grids per document — the reviewer's own run read
the list item at 2 → 8 where the author's read 2 → 6 — so §2 is rekeyed to relations and gate
(2) gained the quantisation guard; **the two `2em`s are the same number in different units**,
the first resolving against `raw`'s size and the second against the caption's, coinciding
only because each bundled look sizes them identically (9/9, 9.5/9.5), so the contract a third
look inherits is "the caption sits on the block's edge" rather than "both numbers are 2em";
and the close-out **under-counted `rules/pipeline.md`**, missing line 446's uncaptioned-wrap
argument and line 451's "exactly once, over a listing's alignment".

Grounding confirmed rather than corrected, so a later round need not re-verify: every
`file:symbol` the phase cites resolves; both looks carry Phase 6's rule; the call contract is
seven; neither new needle exists in either look today, so gate (4)'s needles are fresh;
`samples/showcase/sections/figures.md:19` is the only sample prose stating the edge, and the
only line-pinned showcase test pins `showcase.md` and `sections/mathematics.md`, not
`figures.md`; and `rules/pipeline.md` sits at 993 against `max_lines: 1010`, so unlike Phase
8 no cap move is needed.

### Round 3 — Phase 8 only — 2026-08-31 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, one cosmetic non-blocking, folded. **Converged at three
rounds**, the cap, with the third spent verifying fixes rather than on new substance.

All three round-2 findings confirmed **against the files**. The reviewer additionally
re-grounded the two clauses the fixes touched: the emitter does write
`#figure(…, caption: …)` for images, tables, listings and groups, and its `Event::DisplayMath`
arm writes `format!("$ {markup} $")` and wraps nothing — so §4's narrowed "figures, tables
and listings" is true and the sentence excluding the equation is grounded rather than
asserted. `grep` confirms no "three hashes" survives anywhere in the document.

The one cosmetic finding, folded: **OQ-17 opened "leaves two things it forces" and then
called the showcase "the third"** — a count the round-2 edit staled and this round caught.
Corrected to three.

### Round 2 — Phase 8 only — 2026-08-31 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, three non-blocking, all three folded. All three were
defects the *author's own round-1 fixes* introduced, which is the pattern §7.3 warns about
and the reason the loop re-reviews after folding rather than stopping at the first READY.

1. **Gate (5) still said "the three hashes" after §2 was corrected to two.** The
   re-measurement did not propagate one clause. Corrected at both ends, with the reason
   carried: two and not three because `flat` is the resolved default.
2. **§4's new preamble paragraph claimed the emitter wraps equations in `figure`** — false,
   and contradicted by this spec's own OQ-16 and by `rules/pipeline.md`'s "The two keys do
   not meet on the page". Narrowed to figures, tables and listings, with the exclusion
   stated rather than left implicit.
3. **The showcase gap was one document wider than the close-out described.**
   `samples/showcase/showcase.md:27` carries "## The frontmatter, all eight keys" and
   "carries every key there is, and all eight are optional" — the document a reader is
   pointed at, not a README about it. OQ-17 now records four claims, not two.

**A number the author re-measured, and the correction it forced.** Round 1 asked for §2's
inertness fixtures to be specified so their hashes reproduce. Specifying them exposed that
the third fixture measured on 2026-08-31 carried **four** headings where the other two
carried five, so `8e885f46…` was a hash of a different document. The probe was re-run on
three fixtures sharing one body: the true record is **two** hashes, `013e44a5…` for
`figures: sectioned` and `ee54df7f…` for *both* `figures: flat` and the no-frontmatter
document, which coincide because `flat` is the resolved default — that being
`core/tests/golden_test.rs:the_two_forms_of_the_default_compile_to_the_same_bytes`' property
appearing in the measurement. `8e885f46…` is struck from the document.

### Round 1 — Phase 8 only — 2026-08-31 — fresh general-purpose reviewer with repo access — **READY**

**Round 0 (this episode — one appended phase):** *does this produce the observable, and is
it the right one?* **Yes.** Phase 8 produces a PDF whose headings read `1 First` and
`1.1 Background` — a change to the typeset page, not to internal machinery. It is the right
one: §1.2 records heading numbering as a thing both looks merely *decline*, Phase 7's own
gate carved it out ("separates this phase from one that merely switched heading numbering
on"), and the phase was requested rather than inferred.

Verdict: `READY`, **zero blocking**, eight non-blocking, seven folded and one rejected.

**Every number the phase is keyed to was re-derived independently and held**: the 28
goldens carrying a `template.with` line; the look contract at six moving to seven;
`rules/pipeline.md` saying "six" in exactly three places (732, 812, 858) with the other
`six`/`sixth` hits being `show`-rule ordinals and font faces; the anchors count of **5** for
the specified fixture; `ENABLE_HEADING_ATTRIBUTES` absent from the six extensions
`core/src/emit.rs:parser` enables; and all five README fragments verbatim. The reviewer also
established that `core/tests/golden_test.rs:absent_frontmatter_gets_every_default` is the
**only** literal copy of the call line outside the goldens — line 1634 uses a `starts_with`
prefix and is unaffected — which is what makes the phase's "three sets of assertions" claim
complete rather than merely plausible.

The seven folded, in the author's words:

1. **`web/index.html:416` "Seven frontmatter keys decide the look" becomes eight** — the
   demo page is documentation this project keeps current (`mpdf-007` Phase 4 added line
   477's "An eighth frontmatter key" in the phase that added that key), and
   `core/tests/page_examples_test.rs` asserts nothing about the count, so only the close-out
   catches it.
2. **The showcase, and the trap in fixing it.** `samples/showcase/showcase.md` has a
   ten-line frontmatter with its first heading at line 12, and
   `app/src/preview.rs:the_anchors_are_the_headings_of_whichever_file_the_pane_holds` pins
   `[12, 27, 54]`. Adding the ninth key there breaks a test in the one file the phase says
   it does not touch, failing gate (6). The close-out now **makes the call explicitly**:
   the showcase is not changed and its "all eight" claims are reworded, with OQ-17 carrying
   it forward.
3. **`README.md`'s `## Styling` carries two sentences that move, not one** — the contract
   prose, and "the two questions a look cannot answer on its own", which becomes three.
4. **`rules/pipeline.md` needs a `max_lines` bump** (955 of 960 before a ninth key and a
   seventh parameter), and two wordings move: line 732 carries *both* "Seven of them reach
   the look" and "names all six arguments", and **line 754 argues against this phase in as
   many words** — "`figures`' third name, not a ninth key" — which the tree now falsifies.
5. **Gate (5)'s fixtures were unspecified, so its hashes were not reproducible.** Specifying
   them is what exposed the four-heading fixture recorded under Round 2.
6. **§4's preamble records every prior appended phase and stopped at the seventh.** An
   eighth paragraph was added.
7. **Gate (4)'s rule needle was satisfiable by a look that hardcoded its depth.** Changed
   from `n.pos().len() <=` to **`int(headings)`**, the one fragment that cannot be present
   unless the look reads the key, with the weaker fragment's rejection recorded.

**Rejected, with the reason recorded:** *§1.2's "both bundled looks currently decline" is
made false and no in-place note is planned.* Phase 7 reopened a §1.2 non-goal without
annotating §1.2 in place, so a note here would depart from the corpus rather than match it;
§2's decision quotes the sentence and states exactly what moves, and the spec is
append-only. The reviewer had flagged it only so the call would be made deliberately.


### Round 2 — Phase 7 only — 2026-08-23 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, four non-blocking, all four folded. Converged in two
rounds. All four round-1 blockers confirmed **against the files** rather than the
changelog, and both numbers the fixes newly keyed the gate to were re-derived
independently: the anchors count of 3 — `core/src/emit.rs`'s heading arm pushes a line
before it looks at `level` at all, and `core/src/lib.rs` queries `HeadingElem::ELEM.select()`
with no level filter — and the 26 goldens, all 26 carrying a `template.with` line.

The four folded:

1. **Gate (5)'s first compile could not use gate (1)'s fixture**, which carries
   `figures: sectioned` — an unknown key on the pre-phase tree, refused by
   `core/src/frontmatter.rs`. Both compiles are now stated to be of the document with *no*
   key, which is the only form both trees can read.
2. **Gate (1)'s fixture did not exercise the level-1 scoping it was widened for.** With both
   tables ahead of the `##`, an *unscoped* `show heading:` also reads `Table 1.1` /
   `Table 1.2`. The `##` now sits **between** the two tables, and the fixture says why.
3. **"Carry §2's three rules verbatim" did not say rule 1 *replaces*** each look's existing
   `set heading(numbering: none)`. Added before it, the old line wins and the scheme
   silently no-ops back to `Table 0.1`.
4. **A quote was attributed one spec too late.** "A golden pins emitter output and cannot
   pin a look" is `mpdf-001` Phase 9's, which `mpdf-004` Phase 3 was quoting.

### Round 1 — Phase 7 only — 2026-08-23 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes, with a caveat recorded rather
than buried. It produces the observable — a PDF whose first section holds *Table 1.1* and
*Figure 1.1* and whose second holds *Table 2.1* — and it was **asked for**, which is the
precedent `mpdf-006` OQ-3 and `mpdf-007` OQ-6 both set for this question. The caveat: both
bundled looks are short-form, and sectioned figure numbering earns its keep in long
chaptered documents, so the phase's value rests on this dialect being used for papers —
which its headings, footnotes, math and bibliography support and which nothing in the
corpus measures.

Verdict: `NOT READY`, four blocking, eight non-blocking. One generalist reviewer, matching
every prior episode in this corpus. **No finding was rejected.**

**The largest finding is that the phase's central rule set was never written down.** §2
showed the probe's *effects* — "a figure numbered off it reads `Table 0.1`" — and never the
`set figure(numbering: …)` rule keyed to `counter(heading).get().first()` that produces the
format the gate is keyed to. The phase also counted its own rules two ways, "three rules"
in the scope against "both new rules" in §2. §2 now carries all three as a `typst` block.

**The sharpest finding is that the inertness gate could not fail for the reason it named.**
Gate (5) asserted a sibling of
`core/tests/golden_test.rs:the_two_forms_of_the_default_compile_to_the_same_bytes` — but
`core/src/emit.rs:header` writes the *resolved* default on every call, so "no key" and "the
default spelled out" emit byte-identical Typst and therefore identical PDFs **whatever the
looks contain**. The claim "an implementer who writes the three rules unconditionally
passes (1) and fails here" was false: they pass both. The in-suite form is abandoned for a
cross-tree byte comparison, measured 2026-08-23 at `448e98a8…` — the shipped look against a
look carrying all three rules with `figures: "flat"`, byte-identical.

The other two blocking, and what each really was:

1. **The counter reset was under-scoped twice, and the gate was shaped so it could see
   neither.** It named only `kind: table`, so an image would take the advancing prefix with
   no restart — `Figure 2.3`; and it matched *every* heading level, so a `##` restarted the
   table counter inside its own section. Both fixed and re-measured together: `Table 1.1`,
   `Table 1.2`, `Figure 1.1`, `Table 2.1`.
2. **"The same conditional the `equations` rules already sit in" contradicted the shipped
   looks.** There is no conditional — `equations` is a ternary inside a `set` argument, and
   both look files carry a comment recording that a `set` inside a scoped `if` "would
   compile, emit a valid PDF, and put no number on any page". Two of the three rules are
   `show` rules, for which that form does not exist; the branch moves inside the closure.

Notable among the non-blocking: `show heading: it => it.body` — the draft's way of hiding
the number — leaves both looks' `show heading: set block(…)` nothing to apply to, so a
sectioned document's headings would have lost their spacing and read as paragraphs. That
was replaced by `(..n) => none`, which advances the counter and keeps a heading a heading,
and it made the draft's unrecorded rule-ordering question moot. Also folded:
`core/tests/golden_test.rs:absent_frontmatter_gets_every_default` pins the call line as a
literal and moves with the sixth argument; `README.md`'s `## Styling` states the five-
parameter contract in prose, so it is three edits and not a clause; gate (4)'s two needles
are named; the new fixture and its golden are in scope, making the tree 27 with the 26 the
count *before* it; and OQ-16 was narrowed, the key settled in §2 as `figures` taking
`flat`/`sectioned` so no gate is phrased in a placeholder.

**One precondition finding, outside the phase.** `spec-lint` failed the corpus before round
1 on two citations in `specs/citations_spec.md` that `mpdf-007` Phase 4's own renames had
broken. Repaired under §6.1's pointer rule in a commit of its own; the loop's round 1 ran
against a clean linter.

### Round 3 — Phase 6 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, nothing newly found. Converged **inside the cap**,
which is the first time in this spec's record that a phase has — Phases 4 and 5
each converged on the third round because the round that would have escalated was
the one that cleared, and this one had zero blocking findings from round 1 onward.

Both round-2 points confirmed in the files. The reviewer also declined the one
thing the changelog offered to escalate: a first-member clause in `README.md` and
`rules/pipeline.md` "would add information, not repair a falsehood", since neither
document asserts anything about a mixed group, so §6.1's rule that a phase corrects
the prose it makes misleading does not reach them. Recorded because the author
raised it as a possible blocker and the answer is a boundary worth keeping.

### Round 2 — Phase 6 only — 2026-08-18 — same reviewer, resumed — **READY**

Verdict: `READY`, zero blocking, one non-blocking — and **the non-blocking one
falsified a claim the author had just written into the spec**, which is the round's
whole value.

Fix 5 of round 1 had bounded the inherited-property argument with "the property was
written against the emitter". It was not. `mpdf-004` Phase 3 states it as "no
document's typeset output changes unless its author asks, **because the default name
is the one that numbers nothing**", and that phase's own scope changes
`core/assets/template.typ` and `core/assets/press-release.typ` as well as the
emitter. So a look **has** moved under this property before, and kept it by having a
key to condition on that left its new rule inert until the author set
`equations: numbered`. The bound is now the narrower true one: this phase has no such
key and cannot get one, because OQ-2 refused a frontmatter key for the three `figure`
kinds and OQ-9 priced the look-side per-kind alternative. Folded in as a correction
with the falsified draft quoted, rather than swapped silently.

Also folded: gate (3) said `tests/fixtures/groups.md` needs "no new document at all,
in either look", where reading the second look still wants gate (1)'s
`template: press-release` copy — now "no new **content**".

The round verified the other four round-1 fixes in the files and endorsed the one
judgement call among them: qualifying rather than dropping `rules/pipeline.md`'s
"`figure` centres its body where a bare block sits flush left" preserves two readings
a deletion would lose — the clause stays true for `image` and `table` in every look,
and for `raw` in any look without the new rule.

### Round 1 — Phase 6 only — 2026-08-18 — fresh clean-room reviewer with repo access — **READY**

Verdict: `READY`, zero blocking, five non-blocking — all five accepted and folded,
none rejected, none deferred.

**Round 0 — is this the right thing to build at all? Yes.** It produces the
observable directly: a PDF whose captioned code block stands flush left, where today
the same code sits in two places depending only on whether it carries a number. It is
the right one because the defect was found by reading a real page rather than
inferred, and the fix takes something off the page rather than adding a capability
nobody asked for.

**The round reproduced the phase end to end rather than trusting its measurements** —
copied the tree, added the rule to both looks, rebuilt, and rendered before/after in
both looks plus a two-column variant. Every load-bearing claim reproduced: the
captioned/uncaptioned split before, the return to the prose's left edge after, image
and table still centred, a listings group left and an images group centred, the suite
green with no golden and no `core/src` file moving.

The sharpest finding was a measurement the author had generalised from one order:
**a mixed group takes its first member's kind**, not "Figure". Image-then-listing
reads *Figure* and stays centred; listing-then-image reads *Listing* and goes left,
dragging its image with it. Re-measured by the author in both orders before folding.
Nothing in the build turns on it — no gate covers a mixed group — but "a mixed group
stays centred" is false half the time and was about to become the sentence a later
reader carried away.

The other four, all folded: "the emitter has never written an alignment" overreached,
since `core/src/emit.rs:table_call` writes `align:` from the GFM delimiter row — the
claim the seam needs is that it writes no *page* alignment for a block; the
reconciliation named one stale sentence in `rules/pipeline.md` where three go stale;
gates (1)–(3) named no documents, where Phase 2 gate (8) had named its, and
`tests/fixtures/captioned_blocks.md` has a captioned listing with no uncaptioned twin
so a scratch document is needed either way, while `tests/fixtures/groups.md` already
carries gate (3) whole; and §6.1's step-1 argument did not name the inherited
`mpdf-004` property it bends, which round 2 then corrected again.

### Round 3 — Phase 5 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Converged **at the cap** for the second time in
this spec's record, and for the same structural reason Phase 4 did: the round that
would have escalated is the one that cleared.

Round 2's blocker confirmed resolved — the two clauses that assigned the tight div
opposite outcomes are gone, and the reviewer found nothing else in the phase
reading on it either way. It also answered both questions the changelog asked, and
one answer removed a doubt the author had planted about their own gate: **`- :::` /
image / `:::` at column zero is a shape the parser really produces**, the closer
landing one frame shallower than the opener, so gate (6)'s third placement has
something to fire on rather than being a case over an impossible input.

Four non-blocking, all folded, and **one of them is the sharpest finding of the
episode even though it was not classified blocking.** `core/src/emit.rs:Figure::live`
tests `bufs.len() == self.depth`, and depth is not frame identity: `- :::` / image /
`- :::` puts both delimiters at **depth 2 in different frames**, so a depth-only
implementation accepts the pair and truncates a frame it never opened — and all
three of gate (6)'s stated placements pass under exactly that bug. `Figure` is safe
from it only because its content check catches the rest, which a group has no
equivalent of. The scope now says "the same frame, not merely the same depth" with
the measurement behind it, and gate (6) carries a fourth placement whose whole job
is to fail that implementation.

The other three: the backslash escape does not reach the reservation — `\:::` arrives
with first text `:::` because pulldown-cmark folds the escape into the same run, so
an author escaping the marker is refused on their own line, which is recorded as the
safe direction rather than fixed; §2's REOPENED bullet needs a dated clause at
close-out, since it records what the tight div emits *today* and this phase changes
it, which is the situation §2's own 2026-08-17 note was written for; and the gate
preamble was still crediting round 1 with two cases the later rounds rewrote again.

### Round 2 — Phase 5 only — 2026-08-18 — same reviewer, resumed — **NOT READY**

Verdict: `NOT READY`, one blocking — **newly introduced by round 1's own fix**, the
second time this corpus has recorded the loop doing that to itself, and this time it
was predicted in the changelog and happened anyway.

The three round-1 blockers all confirmed resolved, one of them under test: the
reviewer injected the OQ-13 rule into both looks and ran `cargo test --workspace` —
129 passing, `every_bundled_template_meets_the_call_contract` still at five, the
article's pagination unmoved — then compiled a two-image group through the
press-release look and read *"Figure 1 — Two images"* with that look's own separator.
So "the looks move and the contract does not" is measured rather than argued.

**The blocker: the reservation's carve-out and its new refusal assigned the tight div
opposite outcomes.** `:::` / image / `:::` with no blank lines is **one paragraph**, so
it both begins `:::` while being no valid opener (an error) and stands as a `:::`
"among other text in a paragraph" (untouched) — and the two guesses differ on exactly
the spelling §2 records an author reaching for first. Resolved by stating the position
exactly — the reservation reaches a `:::` that is **the first text of its paragraph**
and nothing else — and by taking the consequence rather than leaving it implicit: the
tight div becomes a **named error**, which is the strongest thing the reservation buys,
since that shape fails silently today.

Five non-blocking, all accepted: a sentence round 1's fix was supposed to have killed
was still standing four paragraphs from the paragraph that disowned it; the `columns`
paragraph still announced OQ-13 as blocking the phase five paragraphs above OQ-13's
resolution; **gate (8)'s needle was unnamed and the obvious one passes vacuously** —
both looks already carry `show figure: set block(…)`, so the gate now names
`set grid(gutter:` and says it is a test of its own rather than an extension of one
whose name would stop describing it; the reserved position left one guess, a group
opened inside a list item, which the scope now closes; and gate (7) said "fenced code
block" where `core/src/emit.rs:step` serves both block kinds from one arm.

### Round 1 — Phase 5 only — 2026-08-18 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0, asked once for this episode and answered by the author against §1: **yes.**
The phase produces the observable, and produces a page the dialect cannot express at
all — two figures side by side under one caption and one number, where `: ` attaches
only to the construct immediately above it. It is the right one because the group
takes one caption, one number and one name, which makes it a captioning want with
arrangement as the means, and because it has a stated consumer. **Recorded rather than
glossed: the justification shifted during drafting.** The motivation that raised it —
source legibility — was falsified by measurement, since a div written tight is one
paragraph and blank lines stay structural inside it, so the environment is *more*
vertical space than the marker. The phase now rests on the capability the requester
had listed as a "later we could even", and that is the kind of shift round 0 exists to
notice.

Verdict: `NOT READY`, three blocking and seven non-blocking. All ten adjudicated, all
ten accepted, none rejected or deferred.

**Blocker 1: the marker rule contradicted itself.** The scope opened a group on the
first `:::` met and refused an unclosed one, while the same scope and gate (7) demanded
a lone `:::` reach the page as prose — under the opener rule there is no such thing,
so gates (6) and (7) could not both pass. The reviewer was right that the `: ` analogy
does not carry: `: ` has an inert position and a group can begin anywhere. Resolved by
making the rule honest rather than patching the gate — `:::` at the start of a
paragraph is **reserved**, the first marker in this dialect that is, licensed by the
census and bounded by being positional.

**Blocker 2: OQ-13 was code-answerable, said it blocked the phase, and the scope and
gate asserted the opposite of its only viable answer.** The reviewer measured both
halves: Typst's default grid gutter is **0**, so two members touch, and a look-side
`show figure: set grid(gutter: …)` separates them while crossing nothing. So both look
files change and the contract stays at five — OQ-3's answer one construct further along
— and the phase stopped hedging its own scope and gate on an open question.

**Blocker 3: an empty group reached Typst.** `:::` / `: A caption.` / `:::` satisfies
every stated rule and emits `grid(columns: 0)`, measured failing with `number must be
positive`, naming no line and no construct the author would recognise — the labelless
failure §2's check-what-the-author-wrote decision exists to prevent, reached from
markdown every other rule accepts. A sixth refusal, taking the list from five to seven
with round 2's addition.

Seven non-blocking, all folded. The `Figure`/`body` reuse claim was **confirmed sound**
— `body` holds the bare call for all three constructs and is stale only after
`captioned`, which members never are — but the reviewer measured that the group shape
*already works today* and splices the caption onto the **second image**, so the closer
must suppress a shipped path rather than extend one; that, the caption being held past
`End(TagEnd::Paragraph)`, and the buffer removal are now in the scope. §2's append-only
claim gains its second exception and its "nowhere else in the dialect" gains one
position, both as dated notes. §4's preamble was stale at four phases. Near-miss
markers (`::::`, `::: two words`) were undecided and are now errors. And the
unclosed-group check had no stated home, with groups inside footnote definitions
unaddressed — it now fires where either walk ends.

The reviewer also re-derived every Typst measurement §2 keys the phase to, against the
pinned 0.15.1: a `grid` of images reads **Figure 1**, of tables **Table 1**, of `raw`
**Listing 1**, all with no `kind:` argument; the group takes **one** count; the
reference resolves to the group; two full-size images in `grid(columns: 2)` scale to
fit rather than overflow. The `:::` census and the tight-div measurement both reproduce
exactly, and `Walk.para` and the standalone test are genuinely untouched inside a group.

### Round 3 — Phase 4 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Converged at the cap, which is worth noting: this
episode used all three rounds, and the round that would have escalated is the one
that cleared.

Round 2's blocker confirmed resolved, and the reviewer answered the two questions
the changelog asked it. **`caption.is_some()` is sufficient, for a stronger reason
than the author gave.** `core/src/emit.rs:caption_name` has exactly one caller,
`core/src/emit.rs:splice_caption`, so a `{#…}` group has meaning in exactly one
place in the dialect — and a display span plus a group was run through all five
frame pushes (item, block quote, table cell, caption, link) and through alt text.
All are prose today except the caption, so the new rule reassigns nothing
anywhere else; alt text never reaches the display arm at all, because the alt
capture sits above the main match and handles `DisplayMath` itself. **And the
guard being a `Walk` field rather than a depth test is what covers the one shape
a depth test would miss: a display span nested inside a link inside a caption.**

**Gate (5a) pins its property rather than passing for free**, which is the failure
gate (5) had in round 1. The no-guard implementation genuinely diverges — it emits
`caption: [See $ x = 1 $ <fig:one>]` with nothing after the closing paren, where
the shipped binary emits `#figure(…, caption: [See $ x = 1 $]) <fig:one>`, and
`[](#fig:one)` then resolves to the equation rather than the figure. The case
fails a wrong implementation on the bytes and on the reference.

Two non-blocking trivia, both folded rather than deferred, because §7's own bar is
that a gate keyed to a literal no stated method reproduces passes for the wrong
reason. The scope printed `: See $$x$$ {#fig:one}` beside the literal
`caption: [See $ x = 1 $]`, which came from a probe using `$$x = 1$$`; the source
line is corrected in both places and the literal re-derived against the built
binary. And the gate preamble's arithmetic was off by one — it said "six" of ten
plus three named — so it no longer counts.

### Round 2 — Phase 4 only — 2026-08-18 — same reviewer, resumed — **NOT READY**

Verdict: `NOT READY`, one blocking finding — **newly introduced by round 1's own
fold**, which is the pattern the loop warns about and the first time this corpus
has recorded it happening.

Round 1's blocker confirmed resolved: gate (4) names all three goldens, the
reviewer re-swept and found those three and no others, and gates (4) and (6) now
agree. The reviewer also confirmed the dropped trailing-separator allowance does
real work — `$$\nw = 4\n$$\n{#eq:nextline}` emits `$ w = 4 $\n{\#eq:nextline}`,
so the `SoftBreak`'s newline puts the recorded span off the end of the frame and
the group stays prose. Adjacency is enforced without a "next event" flag.

**The blocker: the scope claimed a display span inside a caption was refused by
the record's own liveness test, and it is not.** The caption's marker arm pushes
its `bufs` frame *before* anything later in that paragraph is written, so the
span records at that deeper frame and is spent at the same deeper frame — the
depth check passes — and nothing stands between the span and the adjacent text,
so the content check passes too. Both hold, the label is spent on the equation,
and the figure loses the name it has today while the run never reaches
`Caption.text`. Measured against the shipped Phase 3 binary:
`: See $$x = 1$$ {#fig:one}` beneath an image emits
`#figure(image(…), caption: [See $ x = 1 $]) <fig:one>` today, so **the name is
the figure's** and the phase as scoped would have moved it silently. The reviewer
noted this is unlike Phase 3's link-frame argument, which holds only because the
`Figure` record is made one frame *shallower* than where a marker can fire.

Resolved by naming an actual guard — the display arm records nothing while a
caption is open — by correcting the false claim in place rather than deleting it,
since it is the reasoning a later reader will repeat, and by adding gate (5a).

### Round 1 — Phase 4 only — 2026-08-18 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0, asked once for this episode and answered by the author against §1:
**yes.** Phase 4 produces the observable — a PDF whose "as Equation 1 shows" stays
true after an equation is inserted above it — which is §1's promise for everything
a document shows, over the one construct Phase 3 could not keep it for. It is the
right one because `mpdf-004` Phase 3 already tells authors they may number their
formulas so a document can refer to them, while leaving them no way to point at
one; this closes the spec's own stated goal rather than opening a new want.

Verdict: `NOT READY`, one blocking and nine non-blocking. All ten adjudicated,
nine accepted, one remedy rejected, one recorded as a limit.

**The blocker: gate (4)'s blast radius was wrong, and the file it omitted was the
load-bearing one.** The draft said two shipped goldens carry an unnamed display
equation. Three do — `tests/golden/display_math.typ`,
`tests/golden/numbered_equations.typ` and `tests/golden/cross_references.typ` —
and the third is the one Phase 3 shipped four commits earlier, which is **the only
shipped golden where a display equation stands next to a `: ` marker paragraph**,
the exact interaction this phase touches. It is therefore what the phase's own
gate (6) rests on, so the gate excluded the file its neighbour depended on.
`samples/article.md` carries two further display equations and no golden; it is
protected by `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`.

Non-blocking, all accepted and folded: gate (5) tested only trailing text, which
anything shaped like `core/src/emit.rs:caption_name` refuses anyway, so the case
passed for the wrong reason — the leading-text shape `$$…$$ see {#eq:lead}` was
measured emitting prose today and would take a label under that reuse, and gate
(5) now carries both sides. The scope's blanket "nothing else in Phase 3's
machinery changes" contradicted its own later sentence and is replaced by explicit
reused/extended lists. "Entirely a `{#name}` group" was falsified by the phase's
own measured literal, which carries a leading space. `core/src/emit.rs:caption_name`
was never cited despite being the symbol an implementer must reach for, and its
contract — group at the *end* — is not this phase's. The ordering between the two
after-the-walk refusals was unstated. Reconciliation was unnamed, and one of its
three artifacts is not free: `samples/article.md` says in its own prose that there
is no way to label a formula, and that sample's pagination is pinned by a test.

**One remedy rejected.** The reviewer proposed the `pending` slot for the
adjacency problem. Not taken: rounds 2 to 4 of Phase 1 each measured a different
construct broken by holding a call across an event, and nothing here needs a hold,
because the equation is already written when the name arrives. The accepted shape
is a record the display arm leaves and the Text arm spends where it is live —
`core/src/emit.rs:Figure::live`'s argument with the trailing-separator allowance
dropped.

**One limit recorded rather than resolved.** The reviewer could not independently
re-derive three Typst-side literals: no `typst` binary is installed and the
dialect admits no raw-Typst injection. The author measured them by temporarily
injecting probe Typst into `core/assets/template.typ` and compiling through the
CLI, then reverting — the technique earlier phases used, since a look file is
arbitrary Typst. Under `equations: numbered` a labelled `$ x = 1 $` needs no other
change: the page read `(1)`, `(2)`, `(3)` over three equations and the references
read *Equation 1* and *Equation 3*. Under the `plain` default the same source
failed with `cannot reference equation without numbering` verbatim. `it.label` in
a `show math.equation` rule failed with `equation does not have field "label"`,
while `it.at("label", default: none)` returned `<eq:a>`, `none`, `<eq:b>` — which
is what let OQ-4 measure, and reject, the seam-preserving alternative.

The reviewer independently confirmed both looks answer the key with a **`set`**
rule (`core/assets/template.typ` and `core/assets/press-release.typ`), which is
the code-side premise that makes OQ-4's rejection of the `show`-rule shape sound,
and re-derived OQ-10's parser measurement and the empty collision census for `{`
after a `$$`.

### Round 3 — Phase 3 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Run because §7.4 makes folding *any* change a
trigger, and two of round 2's three folds touched the gate — the loop's own
warning is that a fix can introduce a blocker.

Both gate literals were re-derived against the shipped binary rather than read.
A fixture line `: The conversion pipeline.` already emits
`caption: [The conversion pipeline.]` verbatim today — the trailing `.` is
unescaped because `line_is_all_digits` is false, and `splice_caption` trims — so
once the phase drops the `{#name}` group the gate's exact string is what the
stated method produces. **Gate (1a) was confirmed to be a real assertion rather
than a restatement of (1)'s compile**: the failure it catches is the fixture
never carrying a sentence-final reference at all.

OQ-11's classification was re-derived and holds, including that both its shapes
are genuinely reachable from markdown — `[](#fig:pipeline)(top)` emits `…[](top)`
today, which becomes the chained call.

One non-blocking prose note, accepted and folded: OQ-11 had claimed its two
survivors are "loud where the marker form's plural was silent", which does not
hold — the plural failed at the compile too. What actually distinguishes them is
that they need an adjacency ordinary prose does not produce, and that is what the
entry now says. The two reasons carrying the argument, both measured, are
unchanged.

**Converged.** Phase 3's `reviewed` is set to 2026-08-18; `status` was already
`accepted`.

### Round 2 — Phase 3 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round two. Round 1's blocker was verified
resolved in the files rather than in the changelog, and the reviewer **strengthened
the fix rather than merely confirming it**: reading `typst-syntax-0.15.1`'s lexer,
entry to a label is gated on `is_id_continue` in *markup* and in *code* alike, so
within the declared set "first character not `:` and not `.`" leaves
`{letters, digits, -, _}` — every one `is_id_continue`. The rule is not merely
sufficient, it is **exactly equivalent to what both lexers require**.

**The round's best catch is what it checked about the `#ref` switch rather than
what it found wrong with it.** `typst-eval-0.15.1` evaluates `ast::Ref` to
`RefElem::new(target)` — the same element `ref()` constructs — so §2's measurement
table, OQ-7's footnote measurement and OQ-4's and OQ-9's compile failures, every
one of them taken with `@`, still underwrite the phase after the spelling changed.
A switch that had quietly invalidated three earlier measurements would have been
the expensive kind of correct.

It also closed the reserved-name question completely: `core/src/emit.rs` writes
labels in exactly two places, both `fn-{number}`, and neither look emits one at
all — so `fn-` and digits is the whole reservation rather than a guess at it.

Three non-blocking, all accepted and folded. The scope claimed the function form
is not boundary-sensitive, and two shapes survive it — `#ref(<x>)(a)` parses as a
chained call and `#ref(<x>).Then` as a field access — so the claim is now "very
nearly not" and **OQ-11** carries the residue with its price. Gate (1)'s "no `{`
in any caption" needle was unwritable in this repo's negative-needle idiom,
because the same fixture carries a named listing whose `raw` string holds braces;
it is now a positive assertion of the exact caption string. And two literals in
the newly folded text did not reproduce: a Typst message quoted from a stand-in
probe, and a citation of `mpdf-001` §2 where the sentence actually lives in that
spec's §4 Phase 7 — checked at `specs/md_to_pdf_pipeline_spec.md` line 676, inside
the block opening at 627.

### Round 1 — Phase 3 only — 2026-08-17 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — one phase, per §7.0).** *Does this phase produce the
observable, and is it the right one?* Yes, and this is the phase whose observable
is the *motivating* one: §1's whole argument is that "the diagram above" goes
quietly wrong when something is inserted, and a reference Typst renumbers is the
fix — Phases 1 and 2 built the carrier. **The risk recorded rather than
dismissed:** the narrowing takes equations out of what §1 promised, which is a
real reduction; §1's own sketch is a figure reference, and Phase 4 carries the
remainder with named blockers rather than dropping it.

Verdict: `NOT READY`. One blocking, eight non-blocking; all nine accepted, none
rejected.

**The blocker was that the declared name character set is not closed under the
label→reference round-trip, and the author's re-measurement changed the shape of
the fix.** The reviewer named two halves. The first reproduced exactly: a name
opening with `:` or `.` is **not a label at all**, and `#figure(…) <:foo>`
typesets the literal text `<:foo>` on the page raising nothing — the silent drop
§2 exists to refuse, reached through a name the dialect would have accepted. The
second — that a trailing `.` or `:` is silently dropped from a reference — was
measured to be a property of the **`@name` marker only**, not of labels. So
instead of the trailing rule the reviewer proposed, the phase now writes
**`#ref(<name>)`, the function form**, on `mpdf-001`'s own argument for `#emph[…]`
over `_…_`: every shape that is a label at all round-trips through `#ref(<…>)`
onto the right figure, `fig:`, `fig.` and `fig..one` included. That single change
also closed the reviewer's adjacency finding outright — `@figAs` fails with
``label `<figAs>` does not exist in the document``, where a call cannot swallow
what follows it.

One correction to the round's own report, recorded so a later reader does not
re-derive it: `_foo` **is** a valid label. The author's first probe failed on it
because the probe's caption `[_foo]` opened emphasis — a defect in the instrument,
not in Typst.

The eight non-blocking, all folded: emptiness is not knowable at
`Event::Start(Tag::Link)`, so a reference opens a `bufs` frame and settles at the
`End` arm, with the Start arm's two existing refusals untouched; the `{#name}`
group must leave the caption and is not a substring removal, since `escape_into`
has made it `{\#fig\-two}`; a named splice must carry the label into
`Figure.written` or Phase 1's second-caption refusal silently stops firing, which
Phase 2's unnamed cases cannot catch — now gate (3a); `fn-N` is a namespace the
emitter already owns, now a reserved name; the post-walk check falsifies a
document-order property two other artifacts state without qualification, now
stated as the one exception with both artifacts named for reconciliation and §2's
"pre-pass" sentence corrected in place; the undeclared reference reported is the
one on the earliest line, because the obvious container is a set; and
`[ ](#name)` is decided — empty means empty, `is_empty()` and not `trim()`.

Grounding confirmed rather than corrected, so a later round need not re-verify:
every `file:symbol` the phase cites is real and says what the phase says, both
looks leave `ref` and `figure(numbering:)` alone, and **OQ-8's §6.1 step-1 working
survives** — the census holds, no consumer of `#`-fragment links exists in `app/`,
`cli/` or `web/`, and `[](#fig:one)` really does emit `#link("#fig:one")[]` and
put nothing on the page.

### Round 2 — Phase 2 only — 2026-08-17 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round two. **Converged**, and Phase 2's
`reviewed` is set to 2026-08-17. `status` was already `accepted`.

Round 1's blocker was verified resolved in the files rather than in the
changelog. The reviewer re-derived the claim the fix rests on — that both looks'
caption rules are kind-agnostic — and confirmed it: `set figure.caption(…)`,
`show figure: set block(…)` and `show figure.caption: set text(…)` in each file,
not one `.where(kind: …)` among them, and Typst's `figure` default
`numbering: "1"` overridden nowhere. So a captioned table and a captioned
listing are styled and numbered the day the emitter emits one, with zero look
edits, and gate (7)'s diff check makes that enforceable rather than trusted.

**The reviewer also named the property that makes the fix safe, which the author
had not:** the numbering measurement is load-bearing as a *reason* and not as a
*dependency*. If it were wrong, Phase 2's work would be unchanged, because
nothing in the phase needs a look edit either way. The decision is keyed to the
right artifact.

Five non-blocking, all accepted; none rejected this round.

**One is a Phase 3 correctness finding that this round's own measurement
surfaced, and it is the round's best catch.** Typst's "cannot reference X
without numbering" is generic over the element, not special to `figure`. Both
looks set `math.equation(numbering: … else { none })` and `mpdf-004` Phase 3
made `plain` the frontmatter default — so **the default path is the unnumbered
one**, and Phase 3's "a reference becomes `@name` … equations included" would
fail the compile for every document that did not opt in. Measured by the author
before folding it in, twice against the pinned 0.15.1: `equations: plain` fails
with ``cannot reference equation without numbering``; `equations: numbered`
compiles. It also explains why §2's table missed it — that probe ran with
`mpdf-004` Phase 3's numbering rule active, which is the non-default. Recorded
against OQ-4 with three shapes for Phase 3 to weigh. Not Phase 2's problem:
Phase 2 ships no reference and touches no look.

The other four: gate (8)'s third bullet claimed to catch the separator asymmetry
and does not — that bug glues `#figure(` to the line above and Typst breaks the
block out anyway, so the symptom is spacing, and gate (1)'s golden and gate (5)
net it byte-exactly; the read stays, its rationale is corrected. Gate (1) still
said "the one thing an implementer trips on" after the scope moved to two. OQ-9
carried no §4 classification. And `last_updated` was three revisions stale.

### Round 1 — Phase 2 only — 2026-08-17 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase, per §7.0).** *Does this phase
produce the observable, and is it the right one?* Yes: the typeset PDF, with
tables and code blocks carrying captions and their own counters. **The risk
recorded rather than dismissed:** the conversation that prompted this spec asked
about *labels*, and Phase 2 widens the carrier to two more constructs while the
motivating function still lands in Phase 3 — the same flag Phase 1's round 0
raised. The ordering is right rather than accidental: Phase 3 makes `@name` read
"Table 1", which needs a table to be a figure at all, so Phase 2 is a
prerequisite rather than a detour; and Phase 3 is blocked by OQ-4, OQ-7 and OQ-8
while Phase 2 was unblocked by OQ-2's resolution the same day.

Verdict: `NOT READY`. One blocking, eight non-blocking; seven accepted, one
rejected.

**The blocker was the phase not being self-contained on its look half, and it
was the author's own defect twice over.** Phase 2's scope named `emit.rs` and
three things it leaves untouched but never said whether either `.typ` file
changes, while gate (8) handed the phase an unmade design call outright —
"decided in this phase's round". No `OQ-N` carried it, which is §4's named
failure mode. **The two halves of gate (8) could not both hold**: it demanded
"three supplements and three counters at 1" while leaving open a suppression
that would falsify exactly that on the press-release PDF. OQ-2's own text pulled
both ways — "it belongs in `press-release.typ`, where this resolution puts it"
against "whether that look should suppress is left to Phase 2".

**Resolved by measurement rather than by taste, and the measurement inverted the
question.** A figure whose numbering a look has suppressed **cannot be
referenced at all**: Typst fails with ``cannot reference figure without
numbering``, isolated by removing only the suppression and changing nothing
else. So suppression in a look breaks every Phase 3 cross-reference to that
kind, in documents whose authors chose neither. Neither look changes and both
keep all three kinds numbered; gate (7) asserts it as a diff; **OQ-9** now
carries the parked want with its price, and records that the cheapest of its
three shapes is a look hiding the supplement typographically while numbering
stays on, which leaves `ref` working.

**Grounding confirmed rather than corrected, so a later round need not
re-verify.** The reviewer built and ran the shipped binary to settle the one
question reasoning alone would not: after a table, a fenced block and an
indented block alike, a following image-only paragraph still emits the
*standalone* form — the same `*para == Some(top(bufs).len())` predicate the
caption branch tests — and the separator is exactly `\n\n` in every case. So
`Figure::live`'s three conditions and `splice_caption`'s truncate carry over
unchanged, and Phase 1's mechanism does extend as the phase assumes. Every
number re-derived: one golden each for `#table(` and `#raw(block: true`;
`samples/article.md` carries a table, a fenced block **and an indented one**,
captioning none; `samples/press-release.md` carries an uncaptioned table;
`tests/fixtures/captions.md` carries neither construct, so Phase 1's golden
cannot move.

Six non-blocking accepted and folded: the separator-ownership asymmetry, which
Phase 1 never met and which would otherwise glue `#figure(` to the line above;
`table_call` named as the site where a pure formatter can record neither offset
nor depth, so the scope now names both `End` arms; "the fenced-block arm"
contradicting the paragraph below it; gate (7) understating the article's three
constructs and over-claiming `samples/press-release.md`, which no Rust test
compiles; **the centring read**, since a wrapped `raw` block is centred where
every code block in the corpus sits flush left and no gate could see it; and the
gate (1)/(3) fixture ambiguity, now one fixture carrying all three kinds. Two
stale forward-looking clauses in OQ-3 and §2 took dated closing notes.

**One rejected**, recorded so it is not re-raised: the refusal message reads
"second caption for one figure" over a table and a code block. A captioned table
*is* a Typst `figure`, so the message names the element the emitter writes; gate
(4) asks for the construct and the line, both unchanged; and the string is
asserted verbatim by `core/tests/golden_test.rs:each_caption_refusal_names_its_construct_and_its_line`,
so changing it moves shipped work for wording. If it is worth changing, it is
worth changing for all kinds at once, which is not this phase.

### Round 5 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round five — **two rounds past §7.6's cap, each
authorised by the human as its own decision.**

Round 4's blocker was not patched. **Three rounds had found three different
constructs broken by one cause** — holding `Walk.pending` a paragraph longer than
the machinery around it was written for: `Walk::finish`'s hardcoded inline form,
the second `finish()` site in `core/src/emit.rs:collect_definitions`, and
`Walk.para`'s offset. That is evidence about the design, so the mechanism was
replaced rather than the symptom fixed, and the rejected deferral is recorded in
§2 with all three failures so a later reader does not propose it again.

**The replacement looks back instead of holding forward**: `write_image` records
where a standalone call began, and a `: ` paragraph splices the recorded region
into `#figure(…)`. The reviewer tested the load-bearing claim — that the flush
timing does not change *at all*, making the three symptoms unreachable rather
than fixed — and confirmed it at each of the three sites.

**It also verified the property the look-behind rests on, which the author had
asserted:** every write into a `bufs` frame in `core/src/emit.rs` is an append —
no `truncate`, `insert`, `replace_range`, `drain` or `remove` touches one — so a
recorded offset cannot shift while its frame is live. That is what makes "verify
at use" sound rather than optimistic, and it is why gates (2) and (7) are now
structurally true instead of hopeful: the record is inert unless a `: ` paragraph
appears, and the census found none in the corpus.

Three non-blocking, all accepted. The `Walk.para` analogy overstated — `para` is
cleared at `Event::End(TagEnd::Paragraph)` as well as compared at use, so §2 now
borrows the comparison and says so. The second-`: `-line refusal does **not**
fall out of the three spend conditions, because a spliced region carries
`#figure(…)` and would fail the content check into silence rather than the error
gate (4) demands; §2 now names the state that closes it. And OQ-5 was the one
item left between the verdict and a plannable phase.

**OQ-5 resolved at convergence rather than in a round**, recorded as an author's
call taken outside the loop because it gated `status: accepted` and no round had
been asked to decide it: **two subjects, not a framework.** §2's own argument for
excluding citations is the evidence — opposite direction, a new asset channel
across three crates against `core` alone, and a different observable. A shared
spelling convention is not a framework; OQ-8 honours it at no cost.

**Converged.** `status` set to `accepted` and Phase 1's `reviewed` set to
2026-08-15. Phases 2 and 3 stay `null` and are not cleared to build: OQ-2 blocks
Phase 2, and OQ-4, OQ-7 and OQ-8 block Phase 3.

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
