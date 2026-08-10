# Review record — mpdf-001 (`specs/md_to_pdf_pipeline_spec.md`)

Append-only. One heading per round, newest first.

### Round 17 — Phase 9 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, two new non-blocking. The
reviewer verified every fix against the working tree rather than the
changelog, and re-derived the sweep the gate is now keyed to: all
thirteen fixtures lack a `template` key, so every import line stays
`template.typ`; all thirteen lack a `date` key, so every second line
gains `date: none`; no `columns` value moves, because the one fixture
naming one wins over the convention and every other resolves to the
article's `2`. "Exactly its second line" holds, and Phase 3's cited
precedent checks out.

Blocker 1 is resolved by OQ-9's inline resolution — the `date` key,
free string, verbatim, no clock — with the contract retroaction
answered rather than dodged: all four arguments on every template,
named on every call, pinned textually by gate case (4). Blocker 2 is
resolved by OQ-10's inline resolution — no template-carried default;
per-template parse-time resolution, explicit value wins — and the old
gate claim is replaced by its honest successor, the thirteen-golden
second-line sweep.

The two new findings, accepted and folded after the verdict: "the
default's one home" was half-true once the convention's site is the
parse-time resolution — the spec now says the schema, never the
template, stays the home; and "its comment stays true" over-claimed
for `TypstWorld::today`, whose "no template uses a date" sentence is
retouched in the same pass while its no-clock substance stands.

On this convergence: `reviewed: 2026-08-10` on Phase 9. `status` was
already `accepted`.

### Round 16 — Phase 9 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 9): the phase
produces the observable — a PDF in a second look, from markdown that
changes one frontmatter line — and it is the right one: §2's styling
decision reserved exactly this mechanism, and the project's own notes
name the press-release format as the wanted feature. The episode
proceeded.

The reviewer's grounding pass opened every citation and confirmed all
but one against the code: the frontmatter's three keys and its error
shapes, `header`'s fixed import line and its names-every-argument
comment, `TypstWorld::lookup`'s two branches and the struct comment
they carry, `emit`'s return type, the template contract as
`template.typ` implements it, `today`'s deliberate `None`, and every
close-out target. The one miss: "all twelve checked-in golden files"
— thirteen exist, `strikethrough.typ` having landed the same day the
phase was appended.

Verdict: `NOT READY` — two blocking findings, three non-blocking. The
author accepted all five, rejected none, deferred none.

Blocker 1: **the phase's central deliverable was unwritable while OQ-9
stood** — the scope said the press-release file's content "OQ-9
blocks", gate case (2) required that file, and the resolution retroacts
on the phase's own template contract, so an implementer could not plan
from the spec alone. Resolved in the round: OQ-9 landed on the `date`
key. Blocker 2: **gate case (1)'s no-golden-change claim was one the
document itself declared undecided while OQ-10 stood** — the scope
stated the all-arguments contract as settled while OQ-10 held it open.
Resolved in the round: OQ-10 landed on no-template-carried-defaults
with the per-template parse-time convention.

The round also supplied two facts the resolutions lean on: `today`
touches only the compile and never the emitted source, so the clock
option's reproducibility break would ship silently; and the
omit-the-argument alternative changes twelve of the thirteen shipped
call lines while dropping the `--emit-typst` property.

Non-blocking, all accepted: the golden census said twelve where
thirteen exist; the fixed set of template names was never enumerated
in one place; gate case (4) did not name its assertion mechanism.

Rejections: none.

### Implementation note — Phase 8 — 2026-08-10 — the census held, and the rule's cap did not

Not a review round. Phase 8 shipped as reviewed, with two things worth
recording.

The corpus census the round-14 reviewer ran held against the build. The three
options went on and **no shipped golden file changed**: the whole suite passes,
79 tests over four targets, and the only new file under `tests/golden/` is the
phase's own. `tests/fixtures/hostile.md`'s lone tilde and lone dollar still
reach the page as themselves, through the escape loop that already pinned them.

The corpus check passed with no gap: the repository's own README and
`samples/article.md` both convert, and the sample now carries a struck phrase
in each spelling. Its own "a ~ tilde" line survives unchanged, as the close-out
said it would — whitespace on both sides means that tilde can neither open nor
close a run — and the by-eye read confirms the struck phrases struck on the
page.

`rules/pipeline.md` grew past its own 245-line cap, which is raised to 255 in
the same commit as the prose that needed it — the second time this rule has
taken that treatment, after Phase 7 raised it from 205. The growth is a
construct in the dialect list and its inline form, plus the reachability
property this phase establishes over `describe`; the four-line gap paragraph
Phase 7 was obliged to write came out in the same edit.

One fold-in beyond the close-out's letter, named rather than silent: the
sample's constructs list still read "links, tables, and images" and omitted
footnotes, which shipped in Phase 7. It sits three paragraphs above the gap
paragraph this phase deletes, so it was corrected in the same pass.

### Round 15 — Phase 8 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, one new non-blocking. The
reviewer verified every fix against the file and re-ran the probes
itself rather than trusting the author's.

Blocker 1 is resolved by OQ-8's inline resolution — refuse both math
forms — with the scope's math paragraph now unconditional over the
existing reject arm. The reviewer independently re-probed the `\$`
escape, including the motivating case: `the range \$5–$10` stays text,
one backslash on the opener sufficing. Blocker 2 is resolved by the
stated alt-capture disposition, with the false Phase 6 parallel named
in the scope and a gate case pinning the flattening. The three
non-blocking folds were spot-checked in the file, and the reviewer
additionally probed a run of three tildes, which stays text, consistent
with the cited `is_valid_seq` predicate.

The one new finding, accepted and folded after the verdict: the
close-out called the sample's lone tilde "not a delimiter run", but a
run of one is valid — the tilde survives because whitespace flanking
means it can neither open nor close. The sentence now says so.

On this convergence: `reviewed: 2026-08-10` on Phase 8. `status` was
already `accepted`.

### Round 14 — Phase 8 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 8): the phase
produces the observable — a PDF whose struck text is struck, from input
that today prints its tildes as prose — and it is the right one: it
closes the faithfulness gap Phase 7's close-out named in three
artifacts, rather than opening a subject of its own. The episode
proceeded.

The reviewer's grounding pass verified the version pins, re-derived the
"six named, three unreachable" arithmetic over `describe` and `options`,
confirmed the `$` flanking rule and the `is_valid_seq` tilde predicate
against pulldown-cmark 0.13.4's `firstpass.rs` with its own cargo
probes, confirmed `StrikeElem` and the absence of a strike markup form
and of any checkbox element against typst-library and typst-syntax
0.15.1, and ran the corpus census: one unpaired `~` in
`tests/fixtures/hostile.md`, one whitespace-flanked `~` in the sample,
no `~~` pair, no pairable `$` and no task-list bracket outside code
contexts anywhere — so the option flip changes no shipped golden.

Verdict: `NOT READY` — two blocking findings, three non-blocking. The
author accepted all five, rejected none, deferred none.

Blocker 1: **the math scope pre-committed to an answer OQ-8 explicitly
left open** — the scope said "set `ENABLE_MATH`" while OQ-8 held three
answers open, two of which contradict that instruction, so scope and
gate could not both be followed and gate case (2) was unverifiable.
Blocker 2: **strikethrough inside image alt text was unspecified** —
probed: the events arrive between the image's two, where the capture's
reject arm sits, and three observably different implementations all
passed the written gate; the "way Phase 6 dropped its table arms"
parallel was false because a table cannot occur inside alt content and
a strikethrough can.

Non-blocking, all accepted: "their goldens" claimed a golden for the
sample, which has none; the one-tilde form `~struck~` joins the dialect
under `is_valid_seq` unacknowledged; gate case (2) attached "in the
shape OQ-8 lands" to the task-list half, which does not depend on it.

Rejections: none.

### Implementation note — Phase 7 — 2026-08-09 — one arm the scope did not name

Not a review round. Phase 7 shipped as reviewed, with one addition worth
recording and one check worth reporting.

The scope names three footnote error shapes and the code carries four. The
walk of the definitions meets a `Tag::FootnoteDefinition` inside a region it
has already entered — a definition written inside another one — and the
match has to be total there. The probe says the parser hoists such a
definition to a sibling at the top level, so the arm is unreachable; it
returns `footnote definition inside a footnote definition` rather than
panicking, because an unreachable arm that guesses is worse than one that
names what it saw. No test pins it, because no input reaches it.

The corpus check passed with no gap: the repository's own README and
`samples/article.md` both convert, and the sample now carries a real
footnote. The by-eye read confirmed OQ-7 twice — in the fixture's PDF and in
the sample's, where the reference sits in the right column and its note lands
at the foot of that column rather than of the page.

`rules/pipeline.md` grew past its own 205-line cap, which is raised to 245 in
the same commit as the prose that needed it.

### Round 13 — Phase 7 only — 2026-08-09 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, two new non-blocking. The
reviewer verified every fix against the file, not the changelog.

Blocker 1 is resolved with the fold stated as a design requirement: the
map, citedness and the generated `fn-N` names all run over the parser's
own folded equivalence through the `unicase` crate, which the reviewer
re-confirmed sits in `Cargo.lock` as pulldown-cmark's own dependency.
Blocker 2 is resolved as the third error shape, under the fold, naming
the second definition's line. Blocker 3 is resolved by the
store-then-surface rule: pass 1 never raises, pass 2 surfaces every
error at its document position, and the reviewer walked the adversarial
case — a stored definition error with an earlier body error between
reference and region — and found the rule decides it without a guess.
All three fixes are pinned in the gate: the cased repeat in case (1),
the cased duplicate and the frontmatter-over-later-definition-error
order in case (2). The four non-blocking folds were spot-checked in the
file.

The two new findings, accepted and folded after the verdict: the rule's
shopping-list line carries no ordering claim today, so the close-out now
says it gains the reader-order statement rather than replacing one; and
the scope's OQ-7 sentence, stale beside the resolution, now points at
the resolved answer.

On this convergence: `reviewed: 2026-08-09` on Phase 7. `status` was
already `accepted`.

### Round 12 — Phase 7 only — 2026-08-09 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 7): the phase
produces the observable — a PDF whose footnotes sit at the foot of the
page, from input that today prints `[^1]` as prose — and it is the
right one: the flattening is a shipped faithfulness bug against §2's
escape-and-reject decision, not a missing ornament. The episode
proceeded.

The reviewer's grounding pass ran the empirical probes itself: the
event-stream claims against pulldown-cmark 0.13.4 (definition before
and after, top-level regions, a dangling reference staying literal, an
inner definition hoisted to a sibling), the label and reference forms
against typst-library 0.15.1's `footnote.rs` (the `cast!` to
`FootnoteBody::Reference`, the counter stepping only for non-reference
footnotes), and the repo census (no `[^` run in any fixture, sample or
README; the raw-HTML rejection tests at both levels; the buffer-stack
premise in `emit`). It also answered OQ-7 from `typst-layout` 0.15.1's
composer: footnote insertions are column-scoped, an oversized entry
spills to the next column, and a reference inside a table cell is found
by the recursive frame search — so the no-template-rule claim stands.

Verdict: `NOT READY` — three blocking findings, four non-blocking. The
author accepted all seven, rejected none, deferred none.

Blocker 1: **the parser matches labels under Unicode case folding while
the design keyed by raw spelling** — probed: `[^A]` resolves against
`[^a]:`, so the map would miss on valid input and the uncited-definition
error would fire on a cased pair. Blocker 2: **duplicate definitions for
one label silently vanish content** — both regions arrive, a map keyed
by label keeps one body and drops the rest, and the two-shape error list
implied that was legal. Blocker 3: **error ordering across the two
passes was unadjudicated** — pass 1 raising on a definition's content
would report a later error before an earlier frontmatter or body error,
contradicting §2's first-error guarantee and the shipped precedence
test.

Non-blocking, all accepted: OQ-7 was still open while the phase leaned
on it; the `describe` arms had no stated disposition, and the reference
arm stays reachable through the alt capture; the close-out did not name
the two `image_paths` ordering statements; "two footnote arms"
miscounted the match arms.

Rejections: none.

### Round 11 — Phase 6 only — 2026-08-09 — same reviewer, resumed for the one fold-in — **READY**

Verdict: `READY`, zero blocking findings, nothing newly broken. Round 10
folded one change in, and the loop resumes the reviewer after any fold-in.
The reviewer verified it against the file: gate case (1) places emphasis,
inline code and the link inside body cells, with the mechanism recorded
inline — `show raw` names Libertinus Mono, only its regular face is
bundled, and Typst synthesizes no bold, so `strong` could not carry a
code span in the header row. The narrowing contradicts nothing: the
header row is still exercised through its plain text, where Serif Bold
is bundled, and case (2) pins the template rule independently.

On this convergence: `reviewed: 2026-08-09` on Phase 6.

### Round 10 — Phase 6 only — 2026-08-09 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, one new non-blocking finding. The
reviewer verified every fix against the file, not the changelog. The
blocker is resolved: OQ-6 is struck through and `RESOLVED` inline — the
header row is set in strong type through
`show table.cell.where(y: 0): strong` in `template.typ` — and gate case
(2) is restated as a checkable artifact: a test reads
`core/assets/template.typ` and asserts the row-0 rule, because golden
files pin emitter output only. The resolution's own claims were
re-grounded in typst-library 0.15.1: the selector is the library's own
documented idiom, verbatim in its `table.header` example over bare
content blocks — the exact shape this emitter will produce — `TableCell`
carries a zero-indexed `y`, and both bold faces are bundled. The three
non-blocking fixes were confirmed in the file: the four-test census with
its two fates, and the `ENABLE_TABLES` comment rewrite.

The new finding, accepted: a code span inside a header cell renders at
regular weight, because `show raw` names Libertinus Mono and only its
regular face ships — no compile failure, no gate failure, but the
fixture should keep inline code in body cells knowingly. Folded into
gate case (1); Round 11 confirms it.

### Round 9 — Phase 6 only — 2026-08-08 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 6): the phase
produces the observable — a PDF from documents with pipe tables — and it
is the right one: tables are a common construct in an ordinary article,
the parser already reads them behind `ENABLE_TABLES`, and the widening
continues the Phases 3–5 ladder. The episode proceeded.

The reviewer's grounding pass confirmed every parser and compiler claim
against the pinned sources: pulldown-cmark 0.13.4 pads a short row with
empty cells and drops excess ones per GFM, so every delivered row
carries the header's cell count — with a verified aside, the
`MAX_AUTOCOMPLETED_CELLS` DoS cap, which never changes what the emitter
receives; the column count is the alignment vector's length; an integer
`columns:` casts to that many auto-sized columns in typst-library
0.15.1; `table.header` repeats across page breaks and carries the
accessibility tagging; `align` takes a per-column array including
`auto`. It also re-derived the no-golden-changes and empty-corpus
claims: no fixture, sample or README line holds a pipe table outside
`unsupported_table.md`.

Verdict: `NOT READY` — one blocking finding, three non-blocking. The
author accepted all four, rejected none, deferred none.

The blocker: **gate case (2) was keyed to OQ-6, which was open, and the
gate's pinning mechanism itself depended on the resolution** — if the
answer was a `show` rule in `template.typ`, the golden files, which pin
emitter output only, could not pin the look at all. Resolved: OQ-6 is a
decision now — the header row in strong type, the template owning the
rule — and gate case (2) names its own artifact, a test on
`core/assets/template.typ`.

Non-blocking, all accepted: the test migration under-enumerated its
census relative to Phase 4's practice, and one test degraded silently —
`a_frontmatter_error_wins_over_a_later_construct_error` would keep
passing while ceasing to test precedence over a construct error, which
no suite run can catch because nothing fails; two of the four migrated
tests would duplicate Phase 5's image tests, so they are deletions; and
the comment above `ENABLE_TABLES`, which says tables are outside the
dialect, becomes false and is rewritten.

Rejections: none.

### Round 8 — Phase 5 only — 2026-08-08 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, none newly introduced. The
reviewer verified every fix against the file, not the changelog. Blocker 1:
the empty destination is an error clause now, its grounding recorded inline
and re-verified in the pinned sources — pulldown-cmark 0.13.4 delivers
`dest_url: ""` for `[text]()`, and `typst-library-0.15.1`'s `Url::new`
rejects the empty string — and the generic wording also covers the
reference-definition route to an empty destination. Blocker 2: a non-empty
link title is an error clause now, and the reviewer confirmed the
"non-empty" qualifier is right, because `[x](url "")` delivers `title: ""`
and stays in-dialect. The new material was checked too: the hostile-URL
case is implementable — CommonMark separates a destination from a title by
whitespace, so one bare destination can carry both `#` and `"` — and the
no-golden-changes claim holds because `link` is a standard-library element
with no template export. Gate case (3) leaves the exact error strings to
the implementer, which matches the precision of every shipped phase's
rejection gate.

On this convergence: `reviewed: 2026-08-08` on Phase 5. No phase of this
spec remains unreviewed.

### Round 7 — Phase 5 only — 2026-08-08 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 5): the phase produces
the observable — a PDF whose links resolve — and it is the right one: links
are the last family in the Phases 3–5 ladder, whose stated aim is an
ordinary markdown article converting unmodified, and real prose carries
links. The episode proceeded.

The reviewer's grounding pass confirmed the phase's parser claims against
the pinned sources: reference links arrive with `dest_url` already
resolved, an unresolved reference produces no Link event at all (it stays
literal text, which the existing escape handles), and an email autolink
delivers the bare address — the `mailto:` prefix is pulldown's HTML
renderer's work, so prepending it is correctly emitter scope.

Verdict: `NOT READY` — two blocking findings, five non-blocking. The
author accepted all seven, rejected none, deferred none.

The blockers. First: **an empty link destination, legal CommonMark,
produced the pipeline's first unnamed input-dependent compile error** —
`[text]()` delivers `dest_url: ""`, `#link("")` fails Typst's compile
naming neither construct nor line, breaking the guarantee that generated
source always compiles. Resolved: an empty destination is an error naming
the construct and its line, pinned by a new gate case. Second: **the link
`title` field was unspecified**, and the spec's own recorded policy made
both readings defensible — a silent drop flattens content, an error was
nowhere stated. Resolved: a non-empty title is an error, pinned by the
same gate case.

Non-blocking, all accepted: an email autolink in gate case (1), since the
`mailto:` prepend was the one scoped behavior with a branch and no gate;
a hostile URL carrying `#` and `"`, so the golden shows the string escape
doing the work the markup escape must not; the corpus check's vacuity
named — the README holds no link construct outside code fences — and the
sample gaining a real link and an email autolink to fix it; the sample
named in the close-out, as Phase 4's close-out was interpreted in
practice; and two wording tightenings — the reference forms all arrive as
the same `Tag::Link`, and the gate now states the no-golden-changes claim
with its reason.

Rejections: none.

### Round 6 — Phase 4 only — 2026-08-08 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, none newly introduced. The reviewer
verified every fix against the file, not the changelog, and re-grounded the
new clauses in the pinned sources: pulldown-cmark 0.13.4 reports the final
line's terminator as part of a code block's content, and Typst's
`split_newlines` keeps the trailing empty segment, so the phantom line was
real and the strip clause removes it. The clause is bounded — "one … when
present" — so an authored trailing blank line survives and an empty block is
covered. The test-migration census was re-verified complete at three, and
`a_frontmatter_error_wins_over_a_later_construct_error` correctly stays: its
frontmatter error still precedes construct handling after the widening. The
gate's embedded claim — no shipped golden file changes — was re-derived: no
existing fixture contains a pipe, no existing string literal contains a
newline, and Phase 4 adds nothing to the import line.

On this convergence: `reviewed: 2026-08-08` on Phase 4. Phase 5 is a
separate episode and remains unreviewed.

### Round 5 — Phase 4 only — 2026-08-08 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 4): the phase produces
the observable — a PDF from documents with lists, code blocks and quotes,
which the shipped dialect rejects and which the §1 aim, an ordinary markdown
article converting unmodified, needs most. The episode proceeded.

Before the round, the author resolved OQ-5 as a recorded decision, grounded
in the pinned sources rather than memory: tight and loose lists map
structurally through Typst's own markup adjacency rule, verified in
typst-library 0.15.1 — `tight` is derived from blank-line separation, and no
`set` rule overrides it — and in a pulldown-cmark 0.13.4 probe, which shows
loose items wrapping their content in paragraph events. The same probe
caught a grounding gap the author folded in: without `Options::ENABLE_TABLES`
a pipe table parses as paragraph text, so the phase's own rejection gate was
unimplementable as written. The scope now names the option and the reason.

Verdict: `NOT READY` — one blocking finding, six non-blocking. The author
accepted all seven, rejected none, deferred none.

The blocker: **OQ-4's mechanism, applied to what pulldown-cmark actually
delivers, typesets a phantom empty line after every code block, and the
gate would pin it rather than catch it.** The parser reports the final
line's terminator as part of a fenced or indented block's content, and
Typst's function-form `raw` keeps a trailing empty segment — the trimming
that drops it belongs to Typst's backtick markup, not to the function call.
Resolved: the scope strips one trailing newline from the block's content,
when present, with the reason recorded inline.

Non-blocking, all accepted: a no-quote-rule clause mirroring OQ-5's
pattern; the language tag pinned to the first word of the info string, with
no `lang` argument for an indented block or an empty info string; the test
migration enumerated in full — the fixture is deleted, and the inline list
in `line_numbers_survive_a_frontmatter_block` moves too; the full existing
suite added to the gate, with the no-golden-changes claim and its reason;
the pipes reach the PDF as prose, not "escaped prose", since `|` is not in
`SPECIAL`; and a loose item holding two paragraphs added to gate case (1),
so continuation indentation is exercised and pinned.

Rejections: none.

### Implementation note — Phase 3 — 2026-08-08 — the font bundle widened during the build

Not a review round. Phase 3's build found §2's font decision under-specified,
and the fix changed what the spec describes, so it is recorded here rather
than left only in the code.

"Why fonts are bundled, not discovered" names Libertinus Serif as the default
family but does not say which faces, and the shipped bundle carried two:
Regular and Bold. Typst renders the closest match it finds and synthesises
nothing. So `#emph[…]` compiled cleanly and reached the page identical to body
text, and `#raw("…")` fell back to the serif, because Typst's own default for
`raw` names a family this binary does not carry. The emitted Typst was correct
and the gate's `%PDF` assertion passed on both counts — but the observable,
the typeset PDF, did not show emphasis at all. That is the failure the same
section's faithfulness decision exists to prevent, reached through the fonts
rather than through the emitter.

Resolved during the build, with the author's approval: the bundle now carries
five faces, all from one Libertinus release so their metrics agree — Serif
Regular, Bold, Italic and BoldItalic, plus Libertinus Mono, which
`template.typ` names in a `show raw` rule. Regular and Bold were replaced from
that same release rather than left at their earlier provenance. The inline
fixture gained a `***both at once***` clause, so no bundled face goes
unexercised.

This widens §2's decision. §2 is append-only and therefore unchanged;
`rules/pipeline.md` carries the current state. A later spec that revisits
fonts starts here.

### Round 4 — Phase 3 only — 2026-08-08 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, none newly introduced. The reviewer
verified both round-3 blockers against the file, not the changelog, and
re-derived the supporting claims from sources: `typst_string` escapes exactly
`\` and `"`, so gate case (2)'s literal is reproducible from the named
function; pulldown-cmark 0.13.4's `make_code_span` folds every `\r`/`\n` in a
code span to a space, so inline code never needs the newline escape OQ-4
defers to Phase 4; `tests/golden/` holds exactly the four files the scope
says change on the import line; and the `[`/`]` entries in `SPECIAL` are what
keep escaped body text from terminating an emitter-written content block
early. The three non-blocking fixes were spot-checked and confirmed landed.

On this convergence: `reviewed: 2026-08-08` on Phase 3. Phases 4 and 5 are
separate episodes and remain unreviewed.

### Round 3 — Phase 3 only — 2026-08-08 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the appended Phase 3): the phase produces
the observable — a PDF from prose the shipped dialect rejects, which is the
consumer's real input. The episode proceeded.

Verdict: `NOT READY` — two blocking findings, three non-blocking. The author
accepted all five, rejected none, deferred none.

Blockers, and how each was resolved:

1. **OQ-4 unresolved while Phase 3's scope and gate were keyed to it** — the
   same pattern round 1 blocked on OQ-1/2/3. Resolved as a recorded
   decision: the `#raw(...)` function form always, content as a Typst string
   literal through `typst_string`, no delimiter counting; gate case (2)
   rewritten to name the reproducible literal.
2. **The mandated `_…_`/`*…*` markup breaks on CommonMark intraword
   emphasis** — verified against the Typst 0.15.1 lexer: `foo*bar*baz`
   would render literal underscores (a PDF that lies about its source), and
   `*foo*bar` would fail to compile with an unnamed error. Resolved: the
   scope mandates the function forms `#emph[…]`/`#strong[…]`, with both
   failure modes recorded inline as the reason.

Non-blocking, all accepted: the import line becomes
`#import "template.typ": template, divider` on every document, all four
golden files named as changing, and the full suite added to the gate; a
descriptive clause pins `divider` as a column-width horizontal rule; the `\`
line break gains the escape-sequence trap — `\` before a newline, never
before text.

Rejections: none.

### Round 2 — 2026-08-08 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings. The reviewer verified every fix
against the file, not the changelog. All five round-1 blockers confirmed
resolved; all seven non-blocking fixes spot-checked and confirmed landed. The
consistency re-sweep re-derived the literals: the column default (`2`) is
stated in four places and agrees in all; the gate fixtures are consistent with
the schema and the error policy; the frontmatter, `specs/INDEX.md`, and the
derived rollup agree. Two observations, recorded as non-blocking and needing no
spec change: the escape list is non-exhaustive by design ("including"), so the
golden files pin the implementer's choice for the rest; and whether Phase 1's
strip-and-warn writes to stderr from `core` or through `cli` is left to the
implementer, which the gates do not test.

On this convergence: `status: accepted`; `reviewed: 2026-08-08` on Phase 1 and
Phase 2 — the document-wide round covered both.

### Round 1 — 2026-08-08 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this document-wide episode): both phases produce the
observable — Phase 1 a PDF from one markdown file through the CLI, Phase 2 the
same PDF with the frontmatter-controlled article look. The answer was yes; the
episode proceeded.

Verdict: `NOT READY` — five blocking findings, six non-blocking. The author
accepted all eleven, rejected none, deferred none.

Blockers, and how each was resolved:

1. **OQ-2 (font sourcing) unresolved, blocked Phase 1** — the gate was not
   reproducible while the font source was open. Resolved as a recorded
   decision: fonts bundled and embedded at compile time (`core/assets/fonts/`),
   default family Libertinus Serif (OFL), no OS discovery on any target.
2. **OQ-1 (Typst crate list and `World` shape) unresolved, blocked Phase 1** —
   the spec deferred its own research to "this phase's review", which was this
   round. Resolved: `typst` + `typst-pdf`, versions pinned at implementation;
   the `World` supplies the standard library, font book, main source,
   `template.typ` bytes, bundled fonts, and current date.
3. **OQ-3 (frontmatter schema) unresolved, blocked Phase 2** — the §1 example
   read as provisional, and missing-key behavior was undefined. Resolved:
   `title`/`author` optional strings, `columns` `1|2` default `2`, absent
   frontmatter valid, unknown key or invalid `columns` an error naming the
   key. Gate fixtures pinned: the default fixture omits `columns`.
4. **Escaping of Typst-significant characters unspecified** — `$5` would open
   math mode; a friendly fixture would pass for the wrong reason. Resolved: a
   normative escape rule in §2, plus a hostile-fixture gate case whose golden
   file shows each listed character escaped.
5. **Out-of-dialect construct policy undefined** — two implementers would ship
   observably different tools. Resolved: a recorded decision — an unsupported
   construct is an error; the CLI exits non-zero naming the construct and its
   line — plus a bullet-list gate case.

Non-blocking, all accepted: Phase 1 strips and warns on a leading frontmatter
block, and Phase 2 removes that behavior; the two `core` functions are named
(`md_to_typst`, `md_to_pdf`); both close-outs name complete rule `sources` and
all five §8.1 keys; `--emit-typst` output declared inspection-only; the network
fetch attributed to the embedder's package-resolution glue, never the compiler;
the CLI contract pinned (`-o` optional, `.pdf`-substitution default, stderr,
exit code 1); heading levels 1–6 map to Typst headings of the same level.

Rejections: none.
