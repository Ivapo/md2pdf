---
id: mpdf-004
title: math
note: >
  LaTeX math in markdown becomes typeset math in the PDF: mitex converts each
  span in process, a bundled prelude supplies the symbols it emits, and a
  formula that will not convert is a named error.
status: draft
last_updated: 2026-08-11

phases:
  - name: "Phase 1 — inline math on the page"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — display math as its own block"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001]
reference: >
  MiTeX (https://github.com/mitex-rs/mitex) is the converter, taken as a Rust
  crate rather than as the Typst package of the same name. Its `convert_text`
  half — LaTeX documents, not LaTeX formulas — is out of scope permanently:
  this project's document syntax is markdown, and `mpdf-001` §1.1 keeps it
  from becoming a TeX front end.
---

# math

## 1. Goal

Convert a markdown document that carries formulas into a PDF that typesets
them. **The observable is unchanged from `mpdf-001` — the typeset PDF that
Typst compiles from the user's markdown — but the input widens: a document may
now carry `$…$` and `$$…$$` spans, which today make it convert to nothing at
all.**

The consumer is the same author, who writes markdown and today gets this:

```console
$ md2pdf paper.md
error: unsupported markdown construct 'math' at line 12
```

for a document whose line 12 is an ordinary formula:

```markdown
The Gaussian integral is $\int_{-\infty}^{\infty} e^{-x^2}\,dx = \sqrt{\pi}$,
which follows from the polar substitution below.

$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$
```

After Phase 1 the inline span typesets and the document converts. After Phase 2
the display span is set as a block of its own.

**This spec is what `mpdf-001` §1.1 parked.** That section lists "LaTeX math via
`mitex`" among the things held for later specs, and `mpdf-001`'s own OQ-8
resolved to refuse both forms *because* support was parked — "support is parked
for `mitex` by §1.1, so the named error is the whole answer". Phase 8 of that
spec shipped the refusal as "the honest floor meanwhile", the same words it used
for the task list marker. **So this spec takes up a reservation rather than
overturning a decision**, and the methodology's §6.1 step 1 does not apply.

### 1.1 Non-goals

- **Not raw Typst math syntax.** `mpdf-001` §1.1 refuses raw Typst in the input
  dialect, and math is not an exception carved out of it. A `$…$` span holds
  LaTeX and is read as LaTeX, so `$\frac{a}{b}$` is the form that works and
  `$frac(a,b)$` is not the form this dialect promises. A document written in
  Typst math is not an error the parser can see — it is LaTeX that happens to
  convert to something else — and §2 records that limit rather than hiding it.
- **No equation numbering, no labels, no cross-references.** Markdown carries no
  syntax for any of the three, and inventing one is a dialect decision this spec
  is too small to make well. It is also partly a look decision, which `mpdf-001`
  §2 gives to `template.typ` rather than to the emitter.
- **No macro definitions that outlive a span.** `\newcommand` inside one formula
  is whatever `mitex` does with it; a definition in one span visible from the
  next is a document-wide symbol table, which is a different subject.
- **No LaTeX outside math.** `mitex::convert_text` is not called and no LaTeX
  environment beyond the math ones is read. The document syntax stays markdown.
- **No claim about mathematical accessibility.** Typst decides what tagging a
  math block carries, and this spec neither adds to it nor tests it.
- Out of scope, parked: citations and bibliography, which `mpdf-001` §1.1 parks
  separately; `mathml` or any second output form; and a math-aware preview in
  the desktop app beyond what it gets for free.

## 2. Design

`core` gains one dependency and one asset. Nothing else in the project changes:
`mpdf-001` §2's two-wrapper shape means **both front ends get math from this one
piece of work**, because `cli` and `app` each call `md2pdf_core::md_to_pdf` and
neither reads the dialect.

### Why `mitex`, and why the crate rather than the Typst package (decision, recorded)

`mitex` is a TeX-to-Typst converter published in two forms, and this spec takes
the Rust crate — `mitex` 0.2.4, Apache-2.0 — rather than the `@preview/mitex`
Typst package. The package is the wrong shape twice over. It resolves through
Typst's package registry, which is a network fetch, and `mpdf-001` §2's "no
servers, ever" is inherited whole; and it works by loading a WASM plugin at
compile time, which would put a second executable format inside the bundle
`mpdf-003` Phase 5 just made.

The crate has none of that. `mitex::convert_math(&str, Option<CommandSpec>) ->
Result<String, String>` is a pure function over a string, so the conversion
happens in `core`, in process, before Typst sees anything. Measured on this
machine on 2026-08-11 against the crate's own source: `convert_math` is one of
three public entry points, the others being `convert_text` and
`convert_math_no_macro`, and the `Result`'s `Err` arm is a message rather than a
type.

### What `convert_math` actually returns (decision, recorded)

**Bare Typst math markup, to be placed inside `$…$` by the caller** — not a
complete equation, and not anything that could be handed to Typst on its own.
Probed on 2026-08-11 over 26 formulas of the kind a markdown author writes, all
26 converted:

| LaTeX | what `convert_math` returns |
|---|---|
| `\frac{a}{b}` | `frac(a ,b )` |
| `\sum_{i=1}^n` | `sum _(i = 1 )^(n )` |
| `\int_0^1 f(x)\,dx` | `integral _(0 )^(1 ) f \(x \)thin d x` |
| `\alpha\beta\pi` | `alpha beta pi` |
| `\leq \neq \approx` | `<=  !=  approx` |
| `\begin{cases}1&x>0\\0&x\le0\end{cases}` | `cases(1 &x > 0 ,0 &x <= 0 )` |

So the emitter's job is `$` + the conversion + `$`, and the whitespace the
converter leaves is Typst's to ignore.

### Why a bundled prelude, and how small it is (decision, recorded)

**Most of that output is plain Typst, but not all of it.** The same probe found
exactly five identifiers across those 26 cases that Typst's own math module does
not define: `mitexsqrt`, `mitexmathbf`, `mitexunderbrace`, `textmath` and `zws`.
They come from `\sqrt`, `\mathbf`, `\underbrace`, `\text` and the cell separator
inside a matrix — none of them exotic.

Those five are defined by the *Typst* half of MiTeX, which this spec does not
take. So `core` bundles its own prelude: a `.typ` asset defining exactly the
symbols the converter can emit, imported the way the looks already are.
`core/src/lib.rs:file_id` and `TypstWorld::source` already serve bundled `.typ`
files by name, and `core/src/emit.rs:header` already writes an `#import` of the
look the frontmatter chose — so **the mechanism exists and this adds an asset to
it rather than a mechanism**.

**Five is a sample, not the list**, and this section says so rather than
implying otherwise. Twenty-six formulas found five; the complete set is a
property of `mitex-spec` 0.2.4 and has to be read out of it rather than sampled.
OQ-2 carries that, and it blocks Phase 1, because a prelude short of one symbol
fails at Typst compile time on a formula nobody tested — which is the worst
shape a gap can take here.

### Why a formula that will not convert is a named error (decision, recorded)

`mpdf-001` §2's escape-and-reject rule is inherited whole: the tool names what
it cannot handle rather than dropping it or printing it as prose. `convert_math`
returning `Err` therefore becomes a `md2pdf_core::Error` naming the construct
and its line, in the same sentence shape the dialect already prints. Probed:
`\notacommand{x}` returns `Err("error: unknown command: \\notacommand")`.

**No new mechanism carries the line.** `core/src/emit.rs:step` already walks
with `into_offset_iter()` and every existing error reaches
`core/src/emit.rs:line_of` with the event's own range, so a math event's line is
available exactly as a table's or a footnote's is.

### What `mitex` repairs silently, recorded rather than fixed (decision, recorded)

This is the one place where the converter's behaviour and this project's ethos
disagree, and it is written down because a review round should decide it rather
than discover it. **`mitex` does not always refuse malformed input; sometimes it
repairs it.** Probed on 2026-08-11: `\frac{a}{` — an unclosed brace, and not a
formula anyone meant — returns `Ok("frac(a ,zws )")` rather than `Err`. The
document converts, and the PDF shows a fraction with an empty denominator.

That is a flattening of exactly the kind `mpdf-001` §2 refuses, one layer down
and inside a dependency rather than in this project's own code. Three things are
true about it and all three matter. The error class is narrow — an unknown
command *is* refused, which is the commonest real mistake. The alternative is
this project validating LaTeX itself, which is the whole of what taking `mitex`
was meant to avoid. And the failure is visible on the page rather than silent in
the sense §2 fears: a wrong-looking fraction is something an author reading their
own PDF will see. OQ-3 carries the decision.

### Why this changes nothing in `cli` or `app` (decision, recorded)

Neither wrapper reads the dialect. `cli/src/main.rs` and `app/src/document.rs`
both hand a markdown string and an asset list to `md2pdf_core::md_to_pdf`, so a
document with formulas converts through both the moment `core` supports it, and
the desktop app's pane draws typeset math with no change to `mpdf-003`'s crate
at all. **That is the same falsifiable claim `mpdf-003` §2 makes, pointed the
other way**, and this spec's gate checks it as a diff for the same reason.

## 3. Open questions

- **OQ-1** — where does the prelude live, and how does a document import it? Two
  shapes are available and the difference is not cosmetic. A third bundled asset
  — `core/assets/mitex-prelude.typ` beside `template.typ` and
  `press-release.typ` — imported by `core/src/emit.rs:header` beside the look,
  which costs one more `#import` line in every document including those with no
  math. Or the five definitions added to each look's `.typ`, which costs
  duplication across two files today and every look added later, and which makes
  a look's contract wider than the `template` and `divider` exports
  `mpdf-001` Phase 9 fixed. The first looks right and the second is what a
  reader might assume; naming both is what stops it being decided by accident.
  Answerable from code during review. Blocks Phase 1.

- **OQ-2** — what is the *complete* set of symbols `mitex` can emit that Typst
  does not define? §2 records five, found by sampling 26 formulas, and sampling
  is the wrong instrument for a completeness claim: a prelude one symbol short
  fails at Typst compile time, on a formula this project never tested, with an
  error naming a Typst identifier rather than the author's LaTeX. The list is a
  property of `mitex-spec` 0.2.4 and should be read out of it — the crate ships
  its specification as data, which is what makes this answerable rather than
  endless. Answerable from code during review. **Blocks Phase 1**, and it is the
  one open question here that can produce a wrong implementation rather than a
  delayed one.

- **OQ-3** — what does the dialect do about the input `mitex` repairs rather
  than refuses? §2 records `\frac{a}{` converting to a fraction with an empty
  denominator. Three answers are open: accept it, and record that the
  faithfulness rule holds at the construct level rather than inside a formula;
  pre-validate the LaTeX in `core` before handing it over, which is the work
  taking `mitex` avoided and would be a second parser to keep correct; or check
  the *output* for the repair marker — `zws` appears in the repaired form and is
  otherwise a matrix separator, so this is narrower than it sounds but is a
  heuristic rather than a rule. Design call, with the mechanism answerable from
  code. Blocks nothing; it changes Phase 1's scope if the answer is not the
  first.

- **OQ-4** — is a display formula's placement a look decision? An inline span
  sits in its line and needs nothing. A block one has spacing above and below, a
  horizontal alignment, and a decision about whether it breaks across columns —
  and `mpdf-001` §2 gives look decisions to `template.typ` rather than to the
  emitter, which is the rule that kept the emitter out of the table header's
  boldness. If it is a look decision, the two bundled looks each gain a `math`
  export and the emitter names it, which widens the look contract that OQ-1 also
  touches. Design call. Blocks Phase 2.

- **OQ-5** — does `$$…$$` reach the walk as `DisplayMath`, and does a display
  span sitting alone in its paragraph arrive wrapped in a paragraph the emitter
  must not print? `mpdf-002` hit the same shape for images and had to tell a
  standalone image from an inline one; `core/src/emit.rs` already carries that
  distinction for images, so the question is whether it generalises or whether
  math needs its own. Answerable from code during review. Blocks Phase 2.

- **OQ-6** — what happens to a document written in Typst math rather than
  LaTeX? §1.1 says LaTeX is the promise, but `$frac(a,b)$` is not an error the
  parser can raise: it is a LaTeX span whose content happens to be valid-looking
  Typst, and `mitex` will convert it to something. Whether that produces a
  confusing PDF or a clean error is unmeasured, and it decides one sentence of
  the README rather than any code. Answerable by probe during review. Blocks
  nothing.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Both produce the observable,
and the split is between the two things a formula can be rather than between a
mechanism and its use — Phase 1 carries the dependency, the prelude and the
error, and Phase 2 carries only what a block needs beyond a span.

### Phase 1 — inline math on the page
*Produces the observable: yes — a PDF with a typeset formula in its running
text, from a document that today converts to nothing.*

- **Scope:** `core/Cargo.toml` gains `mitex = "0.2.4"`, pinned as every
  dependency here is pinned. In `core/src/emit.rs`, `Event::InlineMath(source)`
  stops falling to the reject arm and becomes `$` + `mitex::convert_math` + `$`.
  **The content does not go through `core/src/emit.rs:escape_into`** — it is
  Typst markup by the time it is written, exactly as a code span's content
  travels as a string rather than as markup, and escaping it would break every
  formula.

  `describe`'s math arm stops being reachable for `InlineMath` and stays for
  `DisplayMath` until Phase 2, which is a state the phase should say out loud
  rather than leave for a reader of `mpdf-001` Phase 8 to trip over: that phase's
  property was that every `describe` arm is reachable, and this spec's Phase 2 is
  where it is true again.

  The prelude is a bundled asset per OQ-1's answer, holding exactly the symbols
  OQ-2's answer names, licensed and attributed as Apache-2.0 alongside the fonts'
  OFL notice.

  An `Err` from the converter becomes a `md2pdf_core::Error` naming the
  construct and its line, through the range `step` already carries.
- **Exit gate:** (1) A golden-file fixture carrying inline math in running text,
  a formula using each of the five symbols §2 measured, and a `\$` escape beside
  them, matches its golden file and compiles to a PDF with the `%PDF` magic
  bytes. **The `\$` case is not decoration**: `mpdf-001`'s OQ-8 made that escape
  the documented exit from math, and this phase must not take it away. (2) A
  formula with an unknown command makes the CLI exit non-zero naming math and
  its line — the reject path survives for the input that deserves it. (3) Every
  symbol OQ-2's answer names is exercised by a test that compiles, because a
  prelude entry no test reaches is one that could be missing. (4) `cargo test
  --workspace` still passes with no shipped golden file changed, and `cli/src`
  and `app/src` are untouched — §2's claim that both wrappers get this for free,
  checked as a diff.
- **Close-out:** Update `rules/pipeline.md`'s dialect section against the code,
  raising `max_lines` in the same pass — its body sits at 279 against a cap of
  280. **Its claim that math is an error stops being true for the inline form
  and is corrected rather than appended to.** The README's math error example
  and its `\$` sentence both change. `mpdf-001` Phase 8's shipped prose gains a
  dated `CORRECTED` note pointing here, per the methodology's §6.1: its math
  sentence is now actively misleading, and a sibling file cannot do that job
  because the reader never gets there. One push.

### Phase 2 — display math as its own block
*Produces the observable: yes — a PDF with a centred display equation, which is
what a formula on its own lines is for.*

- **Scope:** `Event::DisplayMath(source)` becomes a block equation rather than
  an inline one, per OQ-4's answer about where its placement is decided and
  OQ-5's about how it arrives. `describe`'s math arm becomes unreachable, which
  restores `mpdf-001` Phase 8's property in full.
- **Exit gate:** (1) A golden-file fixture with a display formula between two
  paragraphs matches its golden file and compiles, and the golden shows the
  block form rather than an inline one wrapped in a paragraph. (2) A display
  formula and an inline formula in one document each take their own form, which
  is the case a single shared arm would pass and a correct implementation must
  distinguish. (3) `describe` has no reachable math arm, asserted by the same
  means `mpdf-001` Phase 8 used for the arms it made reachable. (4) `cargo test
  --workspace` still passes, and `cli/src` and `app/src` are untouched again.
- **Close-out:** Update `rules/pipeline.md` against the code, and the README's
  math section gains the display form. `mpdf-001` Phase 8's `CORRECTED` note
  from Phase 1 is extended rather than duplicated — one note, both forms. One
  push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-004.md, append-only, one heading per round. See §7 of the
methodology.
-->
