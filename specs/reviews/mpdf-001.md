# Review record — mpdf-001 (`specs/md_to_pdf_pipeline_spec.md`)

Append-only. One heading per round, newest first.

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
