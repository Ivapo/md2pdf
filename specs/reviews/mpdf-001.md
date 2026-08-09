# Review record — mpdf-001 (`specs/md_to_pdf_pipeline_spec.md`)

Append-only. One heading per round, newest first.

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
