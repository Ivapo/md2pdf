---
id: mpdf-004
title: math
note: >
  LaTeX math in markdown becomes typeset math in the PDF: the dialect allows a
  closed list of LaTeX commands, mitex converts them in process, and a command
  outside the list is an error naming the command and its line.
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
  from becoming a TeX front end. **Its Typst half is out of scope too**, and
  §2 records what that costs and how the cost is bounded.
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

### 1.1 Why this is a spec and not a phase of `mpdf-001`

The methodology's §6.1 is an ordered test, and it is worked here in full rather
than stopped at the first step that disposes of an objection.

- **Step 0 — does this change a decision?** Yes. `mpdf-001`'s OQ-8 resolved
  "refuse both forms" on 2026-08-10.
- **Step 1 — does it remove or contradict shipped work?** No. That resolution
  refuses math *because* support was parked — "support is parked for `mitex` by
  §1.1, so the named error is the whole answer" — and Phase 8 shipped the
  refusal as "the honest floor meanwhile", the words it also used for the task
  list marker. Work that names its own replacement is not work this removes.
  **The prose still goes stale**, which is a different instrument: Phase 1's
  close-out puts a dated `CORRECTED` note beside it, per §6.1's third rule.
- **Step 2 — is the subject one an existing spec owns?** This is the real
  question, and the honest answer is that `mpdf-001` owns *the dialect* while
  §1.1 of it reserves *this item* for a later spec by name. The reservation
  decides it, and the corpus has honoured that reading twice: `mpdf-003` took
  "the Tauri UI" and "desktop packaging" off the same list as its own spec.
- **Steps 3 and 4.** Step 3 does not apply: `mpdf-001` §1.1 is a non-goals list,
  not a §2 framework reserving named kinds, so **`extends` stays `null`** and
  `related: [mpdf-001]` carries the link — which is exactly what `mpdf-002` and
  `mpdf-003` both do. Step 4 is therefore the landing: a new spec, `extends:
  null`.

### 1.2 Non-goals

- **Not raw Typst math syntax.** `mpdf-001` §1.1 refuses raw Typst in the input
  dialect. A `$…$` span holds LaTeX and is read as LaTeX, so `$\frac{a}{b}$` is
  the form that works. **A document written in Typst math is not an error the
  parser can see**, and this is measured rather than assumed: `$frac(a,b)$`
  converts to `f r a c \(a \,b \)`, which sets as letter-spaced garbage with no
  error at all. §2 records why that stays a documentation problem.
- **No equation numbering, no labels, no cross-references.** Markdown carries no
  syntax for any of the three. `\label{…}` is refused rather than ignored, per
  §2's subset rule.
- **No macro definitions that outlive a span.** A definition in one span visible
  from the next is a document-wide symbol table, which is a different subject.
- **No LaTeX outside math.** `mitex::convert_text` is not called.
- **No claim about mathematical accessibility.** Typst decides what tagging a
  math block carries; this spec neither adds to it nor tests it.
- **Not every formula MiTeX can convert.** §2's subset rule is the whole of what
  this dialect promises, and the gap is named at the point of refusal.
- Out of scope, parked: citations and bibliography, which `mpdf-001` §1.1 parks
  separately; any second output form; and a math-aware preview in the desktop
  app beyond what it gets for free.

## 2. Design

`core` gains one direct dependency, one bundled asset, and one check. Nothing
else in the project changes.

### Why `mitex`, and why only its Rust half (decision, recorded)

`mitex` 0.2.4 (Apache-2.0) is published in two halves: a Rust crate that
converts TeX to Typst markup, and a `@preview/mitex` Typst package that defines
the symbols the crate's output uses. **This spec takes the first and not the
second**, and §2 below is mostly about what that costs.

The package is the wrong shape twice over. It resolves through Typst's package
registry, which is a network fetch, and `mpdf-001` §2's "no servers, ever" is
inherited whole; and it loads a WASM plugin, which would put a second executable
format inside the bundle `mpdf-003` Phase 5 ships. The crate has neither:
`mitex::convert_math(&str, Option<CommandSpec>) -> Result<String, String>` is a
pure function over a string, so conversion happens in `core`, in process, before
Typst sees anything.

**One direct dependency is 25 transitive ones** — `mitex-parser`, `-lexer`,
`-glob`, `-spec`, `-spec-gen`, `rowan`, `rkyv` and its derives, `logos`, `ena`,
`fxhash`, `ahash`, `countme`, `text-size`, and build-dependencies `which`,
`anyhow` and `serde_json`. That is stated because this project pins every crate
it has and its §2 says "no servers, ever": `mitex-spec-gen`'s build script
prefers a `typst`-CLI path and falls back to `copy_prebuilt`, and round 1 built
it with no network and no warning. It is a note rather than a hazard, and a
phase that finds otherwise should say so.

### What `convert_math` returns, and what it leans on (decision, recorded)

**Bare Typst math markup, to be placed inside `$…$` by the caller.** Measured
against `mitex` 0.2.4, and reproduced byte-for-byte by round 1:

| LaTeX | what `convert_math` returns |
|---|---|
| `\frac{a}{b}` | `frac(a ,b )` |
| `\sum_{i=1}^n` | `sum _(i = 1 )^(n )` |
| `\int_0^1 f(x)\,dx` | `integral _(0 )^(1 ) f \(x \)thin d x` |
| `\alpha\beta\pi` | `alpha beta pi` |
| `\leq \neq \approx` | `<=  !=  approx` |
| `\begin{cases}1&x>0\\0&x\le0\end{cases}` | `cases(1 &x > 0 ,0 &x <= 0 )` |

**That output is not all plain Typst, and the gap is far larger than a draft of
this section claimed.** That draft said "exactly five identifiers", inferred
from 26 sampled formulas. Rounds 1 and 2 refuted it by enumeration rather than
by sampling: `mitex-spec-gen` 0.2.4's `DEFAULT_SPEC` emits **611 distinct heads,
of which 599 are valid Typst identifiers — all multi-character — and 168 of
those are absent from `typst::Library::default()`'s global, `math` and `sym`
scopes** in Typst 0.15.1. Among the 168: the whole matrix family (`matrix`,
`pmatrix`, `bmatrix`, `smallmatrix`), `aligned`, `substack`, `operatorname`,
`stackrel`, `tfrac`, the `xrightarrow` family, the `text*` family, and 24
`mitex*` names. The 12 that are not identifiers are `.`, `...` and ten Unicode
literals.

**Two classes escape an alias-only scan of the spec data**, which is why an
earlier count missed them: `zws`, which the converter writes directly at nine
sites and which the spec data never names; and nine environments that carry no
alias at all (`matrix`, `pmatrix`, `bmatrix`, `Bmatrix`, `vmatrix`, `Vmatrix`,
`smallmatrix`, `itemize`, `enumerate`), whose names pass through unchanged.

**A sample cannot support a completeness claim**, and 168 rather than five is
what ruled out bundling a prelude that covers everything MiTeX can emit.

### Why the dialect bounds the subset, rather than shipping MiTeX's prelude (decision, recorded)

Two answers were available once the 180 was known.

**Vendor the Typst half.** Copy `@preview/mitex`'s definitions into
`core/assets/`. Rejected on three counts: those definitions are not in any crate
this project can depend on, so they arrive by fetching a file from a package
repository and committing it — a supply step this project has never taken; the
result is ~180 definitions of another project's semantics that `core` would then
own the correctness of, version-locked to a Rust crate that ships separately;
and it would not fix the failures below anyway, which are semantic rather than
lexical.

**Bound the subset, and enforce it on the input.** This is the answer, and it is
`mpdf-001` §2's own rule applied one layer down: *the dialect defines what it
accepts, and everything else is a named error.* The bundled prelude is then
small, hand-written by this project, and **its own** — which dissolves the
licensing contradiction a draft of this section carried, where `core` was said
to bundle its own prelude and then attribute it to Apache-2.0.

### Why the check is on the LaTeX in, not the Typst out (decision, recorded)

A draft of this section checked the *converted output* — every multi-character
identifier had to resolve in Typst's math scope or the prelude. **Round 2
falsified it, and the failure is worth recording because it is what fixes the
instrument.** `$\begin{itemize}\item a\end{itemize}$` converts to `"\n-  a "`,
which has no multi-character identifier, no `#` escape, and is not empty. It
passed every arm of that check, and Typst then reads `-` as an ordinary
operator, so the list structure is dropped and the content sets as `− a`. That
is the silent flattening `mpdf-001` §2 exists to prevent, surviving the check
introduced to stop it.

The same round found the rule was not even well-defined. Deciding which tokens
in Typst markup are identifiers needs a parser: `\text{hello world}` converts to
`#textmath[hello world];`, whose `hello` and `world` are content-block text
rather than identifiers, so a naive scan refuses a construct this dialect
intends to support. Named-argument keys (`arg0:`, `level:`), string-literal
contents and content blocks all need the same distinction.

**So the dialect checks what the author wrote.** Before conversion, the emitter
scans the LaTeX source of the span and refuses anything not on a closed list.
Three properties follow, and each is one the output check could not offer:

- **It is decidable with a scanner, not a parser.** The grammar is small and
  stated below.
- **The error names what the author typed.** A refusal says `\includegraphics`,
  not some Typst identifier the author has never seen — which is what dissolved
  a separate open question about the wording of the message.
- **It catches all three of round 1's semantic escapes and round 2's fourth**,
  because `\includegraphics`, `\label`, `itemize` and `\footnote` are simply not
  on the list. It also catches `\newcommand`, which §1.2 makes a non-goal, for
  the same reason and with no special case.

### The scan, precisely (decision, recorded)

Stated here rather than left to an implementer, because it is the phase's one
new mechanism:

- A **control sequence** is `\` followed by one or more ASCII letters, longest
  match. Checked against the allowed-command list.
- A **control symbol** is `\` followed by exactly one non-letter character.
  Checked against its own short list — `\\`, `\,`, `\;`, `\:`, `\!`, and the
  escapes `\{ \} \% \& \_ \# \$`.
- An **environment** is the name inside `\begin{name}` and `\end{name}`, checked
  against the allowed-environment list. `\begin` and `\end` are consumed by this
  rule and are not separately allowed as commands.
- Anything else is refused, naming it and its line.

**The limit it accepts, recorded rather than fixed: the scan does not model
`%` comments**, so a command inside one is still checked and can be refused
though it would never have been converted. That errs toward refusing, which is
the direction that keeps the dialect honest, and `\%` is a control symbol so an
escaped percent does not start a comment.

### Which commands are allowed, and why the list is not written here (decision, recorded)

The list is chosen editorially — the constructs an author of technical prose
actually reaches for — and Phase 1 states the initial one. **What this section
fixes is the criterion and the enforcement, not the membership**, and that is
deliberate: a list written into a spec is a claim nothing checks, while the gate
below cannot be wrong about it.

The prelude follows from the list rather than being chosen beside it. The
derivation is mechanical, and an implementer runs it rather than guessing:
convert each allowed command, collect the multi-character heads of the output,
subtract what Typst's global, `math` and `sym` scopes define, and define exactly
the remainder. From the earlier probe that remainder is known to include
`mitexsqrt` (`\sqrt`), `textmath` (`\text`), `zws` (matrix separators),
`mitexmathbf` (`\mathbf`) and the `matrix` family — but **the spec does not
assert the set**, because Phase 1's gate requires one compiling fixture per
allowed command, so a missing prelude entry fails the gate rather than shipping.
That is the same reasoning `mpdf-001` used for `describe`: a claim is worth
having only where something reaches every arm of it.

### Why the import is conditional (decision, recorded)

The prelude is imported only by a document that has math. This is not a
performance choice — it is what keeps a shipped gate honest.
`core/src/emit.rs:header` writes the two lines every generated document opens
with, and **all 15 shipped golden files begin with exactly those two lines**, so
an unconditional third import would rewrite every one of them and a phase
claiming "no shipped golden file changed" could not also add it.

The mechanism is available because of the order `core/src/emit.rs:emit` already
runs in: it completes the walk, and only then calls `header(&walk.front)`. So
whether the document contained math is known before the header is written.

### Why the Typst world needs a change, and which one (decision, recorded)

**A third bundled `.typ` is not served today**, and a draft of this section was
wrong to call this "an asset added to a mechanism that exists".
`core/src/lib.rs:TypstWorld` builds its sources by iterating
`frontmatter::Template::ALL`, a closed two-variant enum, and its lookup searches
only the main source and those templates — so a prelude that is not a `Template`
variant returns `FileError::NotFound` and the compile fails.

`TypstWorld` therefore gains a binding of its own. **It is not a new `Template`
variant**, which is the tempting cheap fix and the wrong one: `Template` is what
`core/src/frontmatter.rs:Template::from_name` resolves the `template:` key
against, so a prelude variant would become selectable as a document look.

### Why this changes nothing in `cli` or `app` (decision, recorded)

Neither wrapper reads the dialect. `cli/src/main.rs` and `app/src/document.rs`
both hand a markdown string and an asset list to `md2pdf_core::md_to_pdf`, so a
document with formulas converts through both the moment `core` supports it, and
the desktop app's pane draws typeset math with no change to `mpdf-003`'s crate.
**That is `mpdf-003` §2's falsifiable claim pointed the other way**, and both
phases check it as a diff.

## 3. Open questions

- **OQ-1** — ~~how does the emitter learn what Typst's math scope defines?~~
  **RESOLVED (2026-08-11), in round 2's redesign: it does not need to.** The
  question existed only for the output check, which asked whether a converted
  identifier resolves. §2's scan reads the LaTeX instead, against a list this
  project holds, so nothing is looked up in Typst's scopes at run time. The
  scopes are still consulted once — by the implementer, deriving the prelude —
  and that derivation is checked by Phase 1's gate rather than by the emitter.

- **OQ-2** — ~~which symbols does the bundled prelude define, and by what
  criterion?~~ **RESOLVED (2026-08-11), in round 2's redesign: the prelude is
  derived from the allowed-command list, and the gate is what proves it
  complete.** The question assumed the prelude was chosen independently. It is
  not: §2 fixes the derivation — convert each allowed command, take the
  multi-character heads, subtract Typst's global, `math` and `sym` scopes,
  define the remainder — and Phase 1's gate requires one compiling fixture per
  allowed command, so an entry missed by the derivation fails the gate.

- **OQ-3** — ~~what does a refusal *say*?~~ **RESOLVED (2026-08-11), by the same
  redesign: it names the LaTeX the author typed.** The question was a real one
  while the check ran on converted output, where the only thing to name was a
  Typst identifier the author had never seen. Checking the input removes the
  gap: the scan refuses a specific control sequence or environment name, and
  that is what the message carries, with its line.

- **OQ-4** — is a display formula's placement a look decision? An inline span
  needs nothing. A block one has spacing, alignment, and a decision about
  breaking across columns — and `mpdf-001` §2 gives look decisions to
  `template.typ`, the rule that kept the emitter out of the table header's
  boldness. If it is one, both bundled looks gain an export and the emitter names
  it, which widens the look contract `mpdf-001` Phase 9 fixed. Design call.
  Blocks Phase 2.

- **OQ-5** — does a display span alone in its paragraph arrive wrapped in a
  paragraph the emitter must not print? `mpdf-002` hit this shape for images and
  `core/src/emit.rs` already tells a standalone image from an inline one, so the
  question is whether that generalises or whether math needs its own.
  Answerable from code during review. Blocks Phase 2.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Both produce the observable.

### Phase 1 — inline math on the page
*Produces the observable: yes — a PDF with a typeset formula in its running
text, from a document that today converts to nothing.*

- **Scope:** `core/Cargo.toml` gains `mitex = "0.2.4"`, pinned as everything
  here is pinned.

  In `core/src/emit.rs`, `Event::InlineMath(source)` stops falling to the reject
  arm and becomes `$` + `mitex::convert_math` + `$`. **The content does not go
  through `core/src/emit.rs:escape_into`** — it is Typst markup by the time it is
  written, exactly as a code span's content travels as a string, and escaping it
  would break every formula.

  **The input scan is the substance of this phase**, not a guard on it. It runs
  before conversion, over the span's LaTeX, exactly as §2 specifies it, and a
  refusal is a `md2pdf_core::Error` naming the offending control sequence or
  environment and its line — carried by the range that `core/src/emit.rs:emit`
  and `collect_definitions` already walk with through `into_offset_iter()`.

  **The initial allowed list**, which later phases may grow one fixture at a
  time: the Greek letters in both cases; the relations `\leq \geq \neq \approx
  \equiv \sim \propto \to \gets \mapsto`; the operators `\pm \mp \times \div
  \cdot \ast \star \circ`; the set and logic commands `\in \notin \subset
  \subseteq \supset \cup \cap \setminus \emptyset \forall \exists \neg \land
  \lor`; the large operators `\sum \prod \int \oint \lim`; the constants
  `\infty \partial \nabla \ldots \cdots \dots`; the structural commands `\frac
  \sqrt \binom \text \left \right`; the accents `\hat \bar \vec \tilde \dot
  \overline \underline`; the font commands `\mathbb \mathbf \mathrm \mathcal
  \mathit`; the named operators `\sin \cos \tan \log \ln \exp \min \max \det
  \gcd`; the control symbols §2 lists; and the environments `matrix pmatrix
  bmatrix vmatrix cases aligned`.

  **The prelude is derived from that list by §2's procedure**, not chosen
  beside it, and gate (1) is what proves the derivation complete.

  **`core/src/lib.rs:TypstWorld` gains a binding for the prelude**, not a
  `Template` variant, per §2. `core/src/emit.rs:header` writes its `#import`
  **only when the document contained math**, which the walk knows before the
  header is written.

  **Math inside an image's alt text is decided here rather than left to an
  implementer**, because `mpdf-001` Phase 8 decided exactly this interaction for
  strikethrough and the precedent says it is the spec's call. Probed: with this
  repo's own `core/src/emit.rs:options`, `![a $x+y$ b](f.png)` emits
  `Event::InlineMath` inside the image tags. Alt is plain text by CommonMark, so
  the span's **LaTeX source arrives as text** in the alt string and the wrapper
  contributes nothing — the same disposition strikethrough got, and no `$…$` is
  written into an alt string that cannot typeset it.

  **`describe`'s math arm stays reachable after this phase**, through
  `DisplayMath`, and Phase 2 is where it is not. A draft of this phase claimed
  otherwise.
- **Exit gate:** (1) **Every command on the allowed list has a fixture that
  compiles** — one formula per command and per environment, in a golden-file
  test that matches its golden and produces a PDF with the `%PDF` magic bytes.
  This is the case that proves the prelude complete: a symbol the derivation
  missed fails here rather than reaching a user, which is why §2 declines to
  assert the prelude's membership in prose. A `\$` escape sits in the same
  fixture — **not decoration**, since `mpdf-001`'s OQ-8 made that escape the
  documented exit from math and this phase must not take it away.

  (2) **Four refusals, each exiting non-zero and naming the command and its
  line**, at both the `core` and the CLI level: `\includegraphics{fig.png}`,
  which is round 1's asset-contract escape; `\label{eq}`, which is its silent
  drop; `\begin{itemize}\item a\end{itemize}`, which is round 2's silent
  flattening and the case that falsified the previous design; and `\notacommand`,
  the ordinary unknown. **An implementer who tests only the last ships the other
  three**, and the first three are named individually for that reason.

  (3) A document with no math produces a Typst source with **no prelude
  import**, asserted directly — the same property gate (5) checks from the other
  side. (4) Math in an image's alt text produces the alt string §2 specifies,
  and compiles. (5) `cargo test --workspace` passes with **no shipped golden
  file changed**, and `cli/src` and `app/src` untouched.
  Three places assert today that inline math is refused, and each is amended so
  its display half survives to Phase 2: `tests/fixtures/unsupported_math.md`,
  `core/tests/golden_test.rs:each_refused_construct_names_itself_and_its_line`
  and `cli/tests/cli_test.rs:math_and_a_task_list_marker_exit_non_zero_and_name_themselves`.
  **`tests/fixtures/unsupported_display_math.md` already exists beside the
  first**, so the fixtures need no split — only the inline one changes, and the
  display one is what keeps `describe`'s math arm covered until Phase 2.
- **Close-out:** Update `rules/pipeline.md`'s dialect section against the code,
  raising `max_lines` in the same pass — its body is 279 against a cap of 280.
  **Its claim that math is an error stops being true for the inline form and is
  corrected rather than appended to.** The README's math error example and its
  `\$` sentence both change, and it gains one sentence on the bounded subset and
  one on Typst-math-in, per §1.2's measured `$frac(a,b)$` result.
  `mpdf-001` Phase 8's shipped prose gains a dated `CORRECTED` note pointing
  here, per §6.1: its math sentence is now actively misleading, and a sibling
  file cannot do that job because the reader never gets there. One push.

### Phase 2 — display math as its own block
*Produces the observable: yes — a PDF with a centred display equation, which is
what a formula on its own lines is for.*

- **Scope:** `Event::DisplayMath(source)` becomes a block equation rather than
  an inline one, per OQ-4's answer about where its placement is decided and
  OQ-5's about how it arrives. It runs the same input scan Phase 1 built, over
  the display span's own LaTeX.
  `describe`'s math arm becomes unreachable, restoring `mpdf-001` Phase 8's
  property in full.
- **Exit gate:** (1) A golden-file fixture with a display formula between two
  paragraphs matches its golden file and compiles, and the golden shows the
  block form rather than an inline one wrapped in a paragraph. (2) A display and
  an inline formula in one document each take their own form — the case a single
  shared arm would pass. (3) `describe` has no reachable math arm, by the means
  `mpdf-001` Phase 8 used for the arms it made reachable. (4) A display formula
  carrying a command outside the allowed list is refused with its own line
  number, so the scan is not silently inline-only. (5) `cargo test --workspace` passes, and
  `cli/src` and `app/src` are untouched again.
- **Close-out:** Update `rules/pipeline.md` against the code, and the README's
  math section gains the display form. Phase 1's `CORRECTED` note in `mpdf-001`
  is extended rather than duplicated — one note, both forms. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-004.md, append-only, one heading per round. See §7 of the
methodology.
-->
