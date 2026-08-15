---
id: mpdf-004
title: math
note: >
  LaTeX math in markdown becomes typeset math in the PDF: the dialect allows a
  closed list of LaTeX commands, mitex converts them in process, and a command
  outside the list is an error naming the command and its line.
status: accepted
last_updated: 2026-08-15

phases:
  - name: "Phase 1 — inline math on the page"
    reviewed: 2026-08-11
    shipped: 2026-08-14
    cut: null
    by: null
  - name: "Phase 2 — display math as its own block"
    reviewed: 2026-08-14
    shipped: 2026-08-15
    cut: null
    by: null
  - name: "Phase 3 — numbered display equations"
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
  syntax for any of the three. `\label{…}` is refused rather than ignored,
  because §2's scan admits no command that is not on the allowed list.
- **No macro definitions that outlive a span.** A definition in one span visible
  from the next is a document-wide symbol table, which is a different subject.
- **No LaTeX outside math.** `mitex::convert_text` is not called.
- **No claim about mathematical accessibility.** Typst decides what tagging a
  math block carries; this spec neither adds to it nor tests it.
- **Not every formula MiTeX can convert.** The allowed list is what this dialect
  promises, and a command outside it is named at the point of refusal. **The
  promise is about which commands are accepted, not about every way a formula can
  be malformed**: §2 records what `mitex` repairs silently inside an allowed
  command, and accepts it.
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

Two answers were available once the 168 was known.

**Vendor the Typst half.** Copy `@preview/mitex`'s definitions into
`core/assets/`. Rejected on three counts: those definitions are not in any crate
this project can depend on, so they arrive by fetching a file from a package
repository and committing it — a supply step this project has never taken; the
result is ~168 definitions of another project's semantics that `core` would then
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
rather than identifiers, so a naive scan refuses them as unresolvable.
Named-argument keys (`arg0:`, `level:`), string-literal contents and content
blocks all need the same distinction. (`\text` is deferred for reasons of its
own, below; it is cited here because it is the clearest case of the ambiguity,
which any content-bearing command would have shared. **With it off the list no
allowed command emits a `#` escape or a content block at all**, so the
tokenization problem has no live instance — but the `itemize` case above needs
none of that and stands on its own.)

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
- An **unescaped `%`** is refused. It is not a comment marker to this dialect and
  it is not passed through; the reason is below, and it is the one rule here that
  exists because of a measurement rather than because of the grammar.
- `\text` is **not on the allowed list**, so it needs no rule of its own here and
  is refused as any unlisted command is. §2 records why below.
- **Every other character passes through untouched** — letters, digits,
  operators, braces, `^`, `_`, `&`. Only an unrecognised control sequence, an
  unrecognised control symbol, an unlisted environment name, and an unescaped
  `%` are refused.

### Why an unescaped `%` is refused (decision, recorded)

A draft of the scan treated `%` as a limit rather than a rule, and said the scan
"errs toward refusing, which is the direction that keeps the dialect honest".
**Round 3 measured the other direction, and it is a silent drop.** `%` opens a
LaTeX comment, so `mitex` discards the rest of the line:

| the span an author writes | what `convert_math` returns |
|---|---|
| `$x = 100 % of y$` | `x  =  1 0 0  ` — " of y" gone |

No control sequence, control symbol or environment appears anywhere in that
span, so a scan without this rule passes it; the output is non-empty, so nothing
downstream objects; and the PDF shows truncated prose. **That is `mpdf-001` §2's
cardinal case, reachable by typing `100%` in a formula** — and it is the third
instance of one shape, after `\includegraphics` in round 1 and `itemize` in
round 2: content the check cannot see because it is looking at the wrong tokens.

The rule is stated over the whole span rather than over the prose an author
might put in a `\text` group, and it stays necessary with `\text` excluded:
`100 % of y` needs no `\text` to reach the page truncated.

Refusing costs the author nothing, because the grammar already carries the exit:
`\%` is on the control-symbol list, so a percent sign that means a percent sign
is written `\%` and reaches the page — the same one-character escape `mpdf-001`
OQ-8 gave the dollar.

### Why `\text` is not in the initial list (decision, recorded)

`\text{…}` is the one allowed-list candidate whose argument reaches the page as
**Typst markup rather than math**, and that single difference cost two review
rounds and produced four blocking findings, none of which a character rule
closes. It is deferred rather than solved, and OQ-6 carries it.

The findings are recorded here because they are the input to that later phase,
all measured against `mitex` 0.2.4 and parsed with Typst 0.15.1's own
`typst::syntax::parse`:

| the span an author writes | what reaches the page |
|---|---|
| `$\text{= head}$` | a `Heading` |
| `$\text{- item}$` | a `ListItem` |
| `$\text{1. item}$` | an `EnumItem` |
| ``$\text{a `b` c}$`` | a `Raw` block |
| `$\text{<tag>}$` | a `Label`, which does not print |
| `$\text{50\$ each}$` | the literal words **"50dollar each"** |
| `$\text{a \alpha b}$` | the literal words **"a alpha b"** |

**The last two are why the answer is deferral rather than a wider character
rule.** `mitex` maps a control symbol or command to a Typst *math symbol name* —
`dollar`, `amp`, `alpha` — which is correct in math mode and becomes a literal
word inside a markup content block. No set of refused characters reaches that,
because the offending input is a backslash construct the dialect allows
everywhere else.

Three further facts bound the later phase. `.` is the case that shows why the
rule cannot key to `core/src/emit.rs:SPECIAL`: that constant does not contain it,
and `core/src/emit.rs:escape_into` carries it as a positional rule instead —
`ch == '.' && line_is_all_digits(out)` — so `1. item` is an enumeration while
`v1.0 ships` is clean. The group boundary is undefined for reachable inputs:
`\text{= head` unclosed still emits the heading, `\text x` takes the next token,
and `\text{a \text{= h} b}` nests. And a rule phrased as "a character `mitex`
does not itself escape" cannot be written ahead of conversion without
materialising the very hand-written list it exists to avoid.

**What the deferral costs is prose inside a formula, and the exit is markdown.**
An earlier draft of this paragraph claimed composition saved it —
`$100\% \text{ of y}$`, with the leading part converting — and that was refuted
by the very removal it defended: `\text` is an unlisted control sequence, so the
scan refuses that whole span before `convert_math` ever runs. There is no
leading part.

The real exit is better than the one that was wrong, and needs no LaTeX at all:
**`$100\%$ of y`** — a formula, closed, with ordinary markdown prose beside it,
which the emitter has handled since `mpdf-001`. That is the shape a markdown
author reaches for anyway, and the refusal names `\text` rather than mangling
it.

This follows `mpdf-001` Phase 8's precedent exactly, which refused a task list
marker with "support is a later phase or a later spec; the named error is the
honest floor meanwhile".

### Which commands are allowed, and why the list is not written here (decision, recorded)

The list is chosen editorially — the constructs an author of technical prose
actually reaches for — and Phase 1 states the initial one. **What this section
fixes is the criterion and the enforcement, not the membership**, and that is
deliberate: a list written into a spec is a claim nothing checks, while the gate
below cannot be wrong about it.

The prelude follows from the list rather than being chosen beside it. The
derivation is mechanical, and an implementer runs it rather than guessing:
convert each allowed command, collect the multi-character heads of the output,
subtract what the global, `math` and `sym` scopes of **Typst 0.15.1** define —
the version `core/Cargo.toml` pins — and define exactly the remainder.

Round 3 ran it over the allowed list as it then stood and it terminated at
**eleven**: `matrix`, `pmatrix`, `bmatrix`, `vmatrix`, `aligned`, `mitexsqrt`,
`mitexmathbf`, `textmath`, `sect`, `diff`, `negthinspace`. Round 6 re-derived it
over the list as it now stands and got **ten** — the same set without
`textmath`, which left with `\text`. The gate remains the authority, because the
set moves with the Typst version.

**That result is recorded as a measurement, not as the specification**, and the
last three are why the distinction is not pedantry. `sect` (`\cap`) and `diff`
(`\partial`) are **Typst version skew rather than MiTeX helpers** — `mitex`
0.2.4 emits the pre-0.13 spellings where Typst 0.15.1 has `inter` and `partial`
— so they look like ordinary Typst symbols and are exactly what a hand-written
list omits. A draft of this paragraph proved the point against itself: it named
`zws` as a prelude member, and Typst 0.15.1 defines `zws`, so the derivation
subtracts it.

The set therefore moves with the Typst version, which is what makes a list in
prose the wrong instrument. **Phase 1's gate requires one compiling fixture per
allowed command**, so a missed entry — including one a Typst upgrade creates —
fails the gate rather than shipping. That is the reasoning `mpdf-001` used for
`describe`: a claim is worth having only where something reaches every arm of it.

### What `mitex` repairs silently, accepted rather than fixed (decision, recorded)

This decision was made in round 1, lost in round 2's rewrite, and is restored
here because the behaviour survives every version of the design. **`mitex` does
not always refuse malformed input; sometimes it repairs it**, and the repairs are
scan-clean — they use only allowed commands, so §2's scan passes them:

| the span | what `convert_math` returns |
|---|---|
| `$\frac{a}{$` | `frac(a ,zws )` |
| `$\sqrt{$` | `mitexsqrt(zws )` |
| `$\frac{a}{b}{c}$` | `frac(a ,b )c ` |

An unclosed group becomes an empty slot, and the document converts with a
fraction that has an empty denominator.

**It is accepted, and the reasoning is the same one that survived three rounds.**
The failure is *visible on the page* rather than silent — a wrong-looking
fraction is something an author reading their own PDF sees, which is the property
the `%` case above lacked and why that one is refused and this one is not. The
error class is narrow, since an unknown command is still an `Err`. And closing it
means this project validating LaTeX group structure itself, which is a second
parser to keep correct and the whole of what taking `mitex` was meant to avoid.

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

- **OQ-4** — ~~is a display formula's placement a look decision? An inline span
  needs nothing. A block one has spacing, alignment, and a decision about
  breaking across columns — and `mpdf-001` §2 gives look decisions to
  `template.typ`, the rule that kept the emitter out of the table header's
  boldness. If it is one, both bundled looks gain an export and the emitter names
  it, which widens the look contract `mpdf-001` Phase 9 fixed. Design call.
  Blocks Phase 2.~~ **RESOLVED (2026-08-14), in Phase 2's round 1: it is a look
  decision, and the look already has it — so nothing is added to the contract.**

  Typst's block equation is written by putting whitespace inside both delimiters,
  `$ … $` against the inline arm's `$…$`, and that *is* the rule:
  `typst::syntax::ast:Equation::block` tests for a space after the opening
  delimiter and before the closing one. Measured against Typst 0.15.1: `$ x $` is
  a block equation, while `$x$`, `$ x$` and `$x $` are not — which also
  corroborates the `normalise` step Phase 1 shipped, since converted markup that
  kept its trailing space could not have become one.

  A look that wants different spacing, alignment or numbering reaches it with
  `show math.equation.where(block: true): …` in its own file. That is a show rule
  over a Typst element rather than an export the emitter has to name, and **this
  repo already does exactly that twice**: both looks carry `show raw: …` and
  `show table.cell.where(y: 0): strong` for elements the emitter emits and never
  exports. So the emitter writes the block form and decides nothing about how it
  sits, `mpdf-001` Phase 9's look contract — `template` and `divider`, four named
  arguments — is untouched, and Phase 2 keeps Phase 1's "no shipped golden file
  changed".

- **OQ-5** — ~~does a display span alone in its paragraph arrive wrapped in a
  paragraph the emitter must not print? `mpdf-002` hit this shape for images and
  `core/src/emit.rs` already tells a standalone image from an inline one, so the
  question is whether that generalises or whether math needs its own.
  Answerable from code during review. Blocks Phase 2.~~ **RESOLVED (2026-08-14),
  by probe in Phase 2's round 1: it does arrive wrapped, and the emitter needs no
  rule for it.**

  `DisplayMath` is an *inline* event. A `$$…$$` alone on its lines arrives as
  `Start(Paragraph)`, `DisplayMath`, `End(Paragraph)`, and the same event also
  arrives mid-paragraph, inside a heading, a table cell, a list item and a
  footnote definition. The paragraph is printed as it always was; the block
  equation inside it is what breaks the line, and Typst reads the spaced form as
  a block wherever it sits — measured mid-paragraph, not only alone.

  **It does not generalise from the image case, and does not need to.** An image
  carries no signal about which form its author wanted, which is why
  `core/src/emit.rs:write_image` infers one from `Walk.para` and the event that
  follows. `$$` **is** that signal. So the display arm is position-blind: it
  writes the block form wherever the span sits, and a paragraph holding one is
  split by it, which is what the author's own `$$` asked for. Nothing is dropped
  and nothing is guessed, so §2's rule is satisfied with no second error shape
  and no new container state.

- **OQ-6** — what does it take to support `\text{…}`? §2 defers it with the four
  findings that decided the deferral, and they are the question's input rather
  than the question. The shapes available: refuse every backslash construct and
  every character `escape_into` would escape inside the group, which is simple
  and refuses `\text{well-defined}` along with the hazards; or **escape the
  group's content on the way out** rather than refusing it, which is what
  `escape_into` already does correctly for body text and which would put the
  emitter in the business of editing `mitex`'s output; or take the group's LaTeX
  source and emit it as a Typst string, bypassing the converter for this one
  command. The last is the least explored and may be the smallest. It also needs
  the group boundary settled, which §2 records as undefined for three reachable
  inputs. Design call, with the mechanism answerable from code. Blocks nothing;
  it is a phase to append. **`\text` is the commonest LaTeX math command this
  dialect will not accept**, so this is the first candidate for the list's growth
  rather than one deferral among many.

- **OQ-7** — how does an author *reference* a numbered equation? Phase 3 puts a
  number on the page and stops there, so "see equation (1)" is prose the author
  types and keeps true by hand — which goes stale the moment an equation is
  inserted above it. §1.2 refuses labels and cross-references on the grounds that
  markdown carries no syntax for them, and that reason is untouched by Phase 3:
  numbering needs no syntax, and a label does. The shapes available, none
  explored: a `\label{…}`/`\ref{…}` pair admitted to the allowed list, which puts
  a document-wide symbol table in `core` and is the thing §1.2's third non-goal
  and its macro non-goal both refuse for the same reason; markdown's own link
  syntax pointed at an equation, which invents dialect; or nothing, and the
  manual reference is the answer permanently. **The last is a real candidate**,
  because a document that numbers its equations to reference three of them is a
  different document from one that numbers them for the reader's convenience.
  Design call. Blocks nothing; it is a phase to append if the answer is yes.

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. All three produce the
observable.

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
  \sqrt \binom \left \right`, **`\text` excluded per §2**; the accents `\hat \bar \vec \tilde \dot
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

  **This is one table, not 139 files.** The allowed list is a table in the
  source, so the fixture set is the same table walked, and its size is what makes
  it mechanical rather than what makes the phase large.

  (2) **Four refusals, each exiting non-zero and naming the command and its
  line**, at both the `core` and the CLI level: `\includegraphics{fig.png}`,
  which is round 1's asset-contract escape; `\label{eq}`, which is its silent
  drop; `\begin{itemize}\item a\end{itemize}`, which is round 2's silent
  flattening and the case that falsified the previous design; and `\notacommand`,
  the ordinary unknown. **An implementer who tests only the last ships the other
  three**, and the first three are named individually for that reason.

  (3) **The character refusals, which are not command refusals and which gate
  (2) would not reach.** `$x = 100 % of y$` exits non-zero naming the line, and
  `$100\% of y$` — the escaped form — converts and puts the percent sign on the
  page. **That second half is what stops the fix being a ban**, since §2 rests
  the refusal on the escape existing. Then `$\text{a}$` exits non-zero naming
  `\text`, which is the ordinary unlisted-command path and is what §2's deferral
  rests on — **an implementer who adds `\text` to the list passes every other
  case in this gate and fails this one**, which is the point of testing a
  deferral rather than trusting it.

  (4) A document with no math produces a Typst source with **no prelude
  import**, asserted directly — the same property gate (6) checks from the other
  side. (5) Math in an image's alt text produces the alt string §2 specifies,
  and compiles. (6) `cargo test --workspace` passes with **no shipped golden
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

- **Scope:** In `core/src/emit.rs`, `Event::DisplayMath(source)` gains an arm
  beside Phase 1's inline one, writing `$ ` + `core/src/math.rs:convert`'s output
  + ` $` — Typst's block form, per OQ-4 — and setting the same `Walk.math` flag,
  so the prelude import stays conditional on either form. `convert` already scans
  before converting, so "the same input scan Phase 1 built" is one call rather
  than a second mechanism. **The arm consults no position**, per OQ-5.

  **A display span inside an image's alt text is decided here rather than left to
  an implementer**, exactly as Phase 1 decided the inline case and `mpdf-001`
  Phase 8 decided strikethrough. The capture in `core/src/emit.rs:step` gains
  `Event::DisplayMath(latex)` beside its `InlineMath` arm, so the LaTeX source
  arrives as text and the wrapper contributes nothing. Without it the capture's
  `other` arm calls `describe` for an event `describe` no longer names as
  refused, and `![a $$x$$ b](f.png)` fails with the nonsense
  `unsupported markdown construct 'supported construct'`.

  With both arms in place, `describe`'s math arm is **removed**:
  `Event::DisplayMath(_)` joins the group `describe` documents as "the walk
  handles these". That is the half of `mpdf-001` Phase 8's precedent that
  applies — it dropped two arms as strikethrough joined the dialect — and it
  restores that phase's property in full, since every arm `describe` still names
  is reachable.
- **Exit gate:** (1) A golden-file fixture with a display formula between two
  paragraphs matches its golden file and compiles to a PDF with the `%PDF` magic
  bytes, and the golden shows `$ … $`, the spaced form OQ-4 fixed, rather than
  the inline `$…$`. **Its formula uses a command the bundled prelude defines** —
  `\sqrt`, or one of the matrix environments — so an arm that forgot the
  `Walk.math` flag fails here as an unknown identifier, rather than passing on a
  formula whose heads Typst defines anyway. **The same fixture carries a second
  display span mid-paragraph, in a sentence of prose**, and the golden shows it
  in that same spaced form: every other arm of this gate is satisfied by a
  standalone span, so an arm that consulted position — block when alone, inline
  in a sentence — would pass all five while contradicting the scope OQ-5 fixed.
  The fixture this phase retires is itself a mid-paragraph span, so the case
  costs one line. (2) A display and an inline formula in one document each take
  their own form, which gate (1) alone would not catch, since one arm serving
  both would pass it. (3) `describe` no longer names
  math: `Event::DisplayMath(_)` sits in its "the walk handles these" group, which
  is a grep rather than an inspection, and `![a $$x+y$$ b](f.png)` produces the
  alt string `"a x+y b"` and compiles — the case that would otherwise turn an
  in-dialect construct into that nonsense error. (4) A display formula carrying a
  command outside the allowed list is refused with its own line number, so the
  scan is not silently inline-only. (5) `cargo test --workspace` passes, **no
  shipped golden file changes**, and `cli/src` and `app/src` are untouched again.

  **Three shipped assertions retire, named here because `cargo test` finding them
  is not the same as a phase budgeting for them**, which is the precedent Phase 1
  set: `tests/fixtures/unsupported_display_math.md`, the `DISPLAY_MATH_MD` row in
  `core/tests/golden_test.rs:each_refused_construct_names_itself_and_its_line`,
  and the `unsupported_display_math.md` row in
  `cli/tests/cli_test.rs:math_and_a_task_list_marker_exit_non_zero_and_name_themselves`.
  That leaves the first of those two tests holding the task list marker alone and
  the second holding two rows, since its `unsupported_math.md` row now names
  `\includegraphics` — and CLI coverage of an inline refusal does not rest on the
  retiring row either way, because
  `cli/tests/cli_test.rs:each_refused_formula_exits_non_zero_and_names_its_latex`
  writes its own document for all six shapes.
- **Close-out:** `rules/pipeline.md`'s Math section is **corrected in place
  rather than grown** — the display form replaces the sentence refusing it, and
  the rejection paragraph loses its third item — so the cap Phase 1 set (a body
  of 330 against `max_lines: 340`) is expected to hold; raise it in the same pass
  if it does not. The README's math section gains the display form, and its "a
  display formula **on its own lines**" sentence is corrected rather than
  extended: the shipped refusal covered a mid-paragraph `$$x$$` too, which is
  what its own fixture holds, and after this phase both positions typeset.
  `mpdf-001` Phase 8's `CORRECTED` note gains **a second dated line beneath the
  existing one** rather than a restamp — the first stamp records when the inline
  half stopped being true, and a note that moved its own date would lose that.
  One push.

### Phase 3 — numbered display equations
*Produces the observable: yes — a PDF whose display equations carry `(1)`,
`(2)`, which is what a document that refers to its own formulas needs.*

Appended 2026-08-15, after Phase 2 shipped, per the methodology's §6.1: math is
this spec's subject, and step 2 lands here rather than on a new document.

- **Scope:** A document may ask for its display equations to be numbered, and
  the two bundled looks decide what a number looks like. **Nothing about the
  dialect changes** — no new command, no new environment, no change to §2's scan
  — which is what keeps this phase small enough to be one plan-mode pass.

  **This phase contradicts one third of a §1.2 non-goal, deliberately, and the
  argument is that the non-goal's own reason does not cover it.** That bullet
  reads "No equation numbering, no labels, no cross-references. Markdown carries
  no syntax for any of the three." The reason is exactly right for labels and
  cross-references, and it is **not a reason about numbering at all**: blanket
  numbering needs no markdown syntax, because the author is not naming anything.
  OQ-4's resolution had already located where it would live, in a sentence
  written before anyone asked for it — "a look that wants different spacing,
  alignment **or numbering** reaches it with `show math.equation.where(block:
  true): …` in its own file." Labels and cross-references stay refused, and OQ-7
  carries them.

  **The frontmatter gains a fifth key, and the look gains a fifth argument.**
  `core/src/frontmatter.rs:Frontmatter` gains `equations`, resolved the way
  `template` is — a name checked against a closed set, with an error listing the
  set — rather than a boolean, so that a later per-section or per-chapter scheme
  is a new name rather than a new key. The initial set is two names, and the
  default is the one that changes nothing.

  ```markdown
  ---
  title: A paper
  equations: numbered      # the other name is `plain`, which is the default
  ---
  ```

  `core/src/emit.rs:header` passes it on, so its `#show: template.with(…)` call
  carries five named arguments where it carries four today, and both
  `core/assets/template.typ` and `core/assets/press-release.typ` take the
  parameter and act on it.

  **The author decides *whether* and the look decides *how*, and that seam is
  the design.** Numbering is not a house style: a paper numbers its equations
  because it refers to them, a press release with one formula in it does not, and
  two documents in the *same* look will disagree. That is the shape `columns`
  already has — an author override of a look's own default, which `mpdf-001`
  Phase 9 settled. What the look keeps is the format: `(1)` against `1.`, where
  it sits, what type it is set in. So this is `mpdf-001` §2's rule about look
  decisions applied at the seam rather than against it.

  **Two alternatives were available and both lose.**

  *Number in the look and give the author no say.* The cheap one — one line per
  look, no schema change, no emitter change, and **not one golden file moves**.
  It loses because it answers a different question: every existing article-look
  document with a display span would gain numbers nobody asked for, and an author
  who wanted them off would have no way to say so. A default that cannot be
  overridden is a decision taken away from the consumer this spec exists to
  serve.

  *Write the `set` rule from the emitter.* `core/src/emit.rs:header` could emit
  `set math.equation(numbering: "(1)")` directly when the key is present, which
  widens no look contract and moves no argument. It loses on `mpdf-001` §2's rule
  — the one that kept the emitter out of the table header's boldness. The format
  of a number is a look decision, and an emitter that writes `"(1)"` has taken
  it.

  **What the widening costs, stated because it is this phase's real price.** The
  look contract moves from four named arguments to five, and `mpdf-001` Phase 9
  fixed it at four. OQ-4 declined to widen it for Phase 2 and said so in as many
  words — "nothing is added to the contract" — so this phase does the thing the
  previous one avoided, and **the round should say whether the seam above is
  worth it or whether the emitter route is less bad after all.**

  **And every shipped golden file changes on line 3.** All 17 carry
  `#show: template.with(title: …, author: …, columns: …, date: …)`, and all 17
  gain a fifth argument. That is the largest golden movement in this project's
  record — `mpdf-001`'s look phase moved 13 on their second line, gaining
  `date: none`, and said so in its own commit message. **It is the same *kind* of
  movement, which is what makes it acceptable**: a golden pins what a document
  compiles to, and what a document compiles to has genuinely changed. It is not
  the kind `mpdf-003` Phase 6 refused, where an anchor marker would have moved all
  17 for a reason with nothing to do with what the document says.

  **No document's typeset output changes unless its author asks**, because the
  default name is the one that numbers nothing. The generated source moves for
  every document; the PDF moves only for a document carrying the key. Gate (2)
  holds those apart, and it is the case an implementer is likeliest to skip.

  **What a number attaches to was measured rather than assumed.** One `$$…$$`
  span is one equation and takes one number, whatever it contains. Measured on
  2026-08-15 against the Typst 0.15.1 `core/Cargo.toml` pins, with the rule set in
  `core/assets/template.typ` and the CLI run over a three-line `aligned` block
  followed by a one-line span: the block took `(1)`, centred against its three
  lines, and the span after it took `(2)`. Inline math was untouched in the same
  document, because Typst's `math.equation` numbering applies to the block form
  alone.

  **That is LaTeX's `equation`-wrapping-`aligned`, not LaTeX's `align`**, and the
  difference is the one an author arriving from LaTeX will trip on: `align`
  numbers every line of a derivation, this numbers the derivation. A per-line
  scheme needs the `align` environment on the allowed list and a second numbering
  mechanism under it. Both are out of scope here, and `align` stays refused by
  name, as §2's scan refuses it today.
- **Exit gate:** (1) A fixture carrying `equations: numbered` and two display
  spans matches its golden file and compiles to a PDF with the `%PDF` magic
  bytes, and its golden shows the fifth argument in the `template.with` call.
  **The same fixture carries an inline span**, so an implementation that numbered
  both forms fails here rather than shipping.

  (2) **A document without the key compiles to a PDF that is byte-identical to
  the one it compiled to before this phase** — the property that keeps the
  widening honest, and the case the whole design rests on. It is checkable
  because the PDF is a pure function of the markdown and the assets, which
  `mpdf-003` Phase 3's gate established and measured over five processes. An
  implementer who makes `numbered` the default passes every other case here and
  fails this one.

  (3) A three-line `aligned` span takes **one** number and the next span takes
  the next number, which is the limit §2 records rather than a behaviour to
  discover in use.

  (4) An `equations` value outside the set is a `Frontmatter` error naming the
  key, its line and the accepted names, exactly as
  `core/src/frontmatter.rs`'s template arm does — one mechanism, not two.

  (5) Both looks honour the key. **Two fixtures, because one cannot carry both**:
  `template: press-release` with `equations: numbered` numbers too, and its
  golden shows the argument reaching the second look. An implementer who wires
  only `template.typ` passes (1) through (4) and fails this.

  (6) `cargo test --workspace` passes, and `cli/src` and `app/src` are untouched
  — the claim `mpdf-003` §2 makes pointed the other way, which both prior phases
  checked as a diff. **All 17 golden files change on line 3, and that is expected
  rather than a failure**; a diff touching any other line of any of them is not.
- **Close-out:** `rules/pipeline.md`'s frontmatter section gains the fifth key
  and its two names, and its look-contract section is **corrected rather than
  appended to** — it records four named arguments, which stops being true. Raise
  `max_lines` in the same pass if it does not fit; the cap moved to 372 in
  `mpdf-003` Phase 6 and the body sits at 368.

  The README's frontmatter table gains the key, and its math section gains the
  numbered form.

  **§1.2's non-goal takes a dated `CORRECTED` note**, per §6.1, splitting the
  bullet rather than deleting it: equation numbering is now supported and labels
  and cross-references are still refused, with the reason — no markdown syntax —
  now attached to the two it actually covers. **OQ-4's resolution needs no note**:
  its sentence about a look reaching numbering is what this phase spends, not
  something this phase falsifies. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-004.md, append-only, one heading per round. See §7 of the
methodology.
-->
