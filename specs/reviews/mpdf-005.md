# Review record — mpdf-005 (`specs/captions_and_references_spec.md`)

Append-only. One heading per round, newest first.

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
