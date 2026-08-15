# Review record — mpdf-004 (`specs/math_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 2 only — 2026-08-14 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two non-blocking, both folded after the verdict.
All ten of round 1's findings had been accepted, so this round was confirmation
plus the two things the author asked it to be sceptical of, and it was sceptical
of both by measurement rather than by reading.

**It re-derived the block-form rule rather than trusting the author's
measurement**, and both agree: `Equation::block` in Typst 0.15.1 is a space node
immediately inside each delimiter and nothing else, so `$ x $` is a block and
`$x$`, `$ x$` and `$x $` are not.

**The position-blind decision was tested against fourteen containers**, compiled
under this repo's own `template.typ` and `math.typ` with `mitexsqrt` and
`matrix` bodies so the prelude was exercised: standalone, mid-paragraph, in a
heading, a list item, a nested list item, a block quote, a table cell, a
footnote, link text, `#emph`, `#strike`, an `aligned` body, and a body ending in
`normalise`'s trailing `\ `. All fourteen compiled with zero warnings and all
parsed as `block: true`, and an inline-plus-display document gave `[false,
true]`. The reason it holds is stronger than the sample: `Equation::block`
consults only the equation's own first and last children, so position-blindness
is a property of Typst's grammar rather than an assumption about context.

**"No shipped golden file changes" was proved rather than estimated.** Every
golden-backed fixture converts today and `DisplayMath` is an error today, so no
such fixture can hold one; independently, the only `$$` under `tests/`, `core/`,
`cli/` or `app/` is `tests/fixtures/unsupported_display_math.md`, which has no
golden.

The two non-blocking, both folded. **The gate did not pin the phase's newest
decision**: every arm was satisfiable with standalone spans, so a
position-*aware* arm — block when alone, inline in a sentence — would have
passed all five while contradicting the scope. Gate (1) now carries a
mid-paragraph span, which costs one line because the retiring fixture is itself
one. And a closing gloss said two tests are left holding the task list marker
alone; the CLI one keeps two rows, since its `unsupported_math.md` row now names
`\includegraphics`. The instruction above it named the right three things to
retire, so the gloss was wrong rather than the instruction.

On this convergence: `status` was already `accepted`, and `reviewed: 2026-08-14`
on Phase 2.

### Round 1 — Phase 2 only — 2026-08-14 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0, for this episode:** yes. Phase 2 produces the observable — a `$$…$$`
span exits non-zero today, and this sets it as a centred display equation, which
is the other half of the construct §1's own example document carries. It is also
the right next thing: Phase 1 shipped the dependency, the scan and the prelude,
so this is only what a block needs beyond a span, and it is what makes
`describe`'s math arm unreachable and restores `mpdf-001` Phase 8's property in
full.

Verdict: `NOT READY`, three blocking findings, seven non-blocking. All ten were
accepted; none were rejected or deferred. The reviewer probed pulldown-cmark
0.13.4 with the emitter's own option set rather than reasoning from memory,
which is what produced the two measurements below.

**Two blockers were the same shape: an open question the phase's own scope was
defined by.** OQ-4 and OQ-5 both said "Blocks Phase 2" and both were unresolved,
so an implementer had to choose unaided — and for OQ-4 the choice ranged from a
form the emitter writes itself to an export both bundled looks gain, which would
rewrite the first line of all 16 goldens and break
`core/tests/golden_test.rs:every_bundled_template_meets_the_call_contract`. The
phase's blast radius was undetermined until it was answered. **This is the same
finding round 1 made against Phase 1**, and the methodology's §4 is the rule it
comes from: a code-answerable question is answered *during review*.

Both were answered in the round. OQ-4: **the look already has the decision**, so
nothing joins the contract — Typst's block form is whitespace inside both
delimiters, and a look reaches it with `show math.equation.where(block: true)`,
which is what both looks already do for `raw` and `table.cell`. OQ-5: the
reviewer's probe found `DisplayMath` is an *inline* event that arrives wrapped
in a paragraph when alone and unwrapped when not, and the author's answer is
that **the arm consults no position** — an image carries no signal about which
form its author wanted, which is why `write_image` infers one, while `$$` *is*
that signal.

**The third blocker is the one a gate would not have caught.** Probed:
`![a $$x+y$$ b](f.png)` emits `DisplayMath` inside the image's capture, which
has no arm for it. So changing only the main match leaves gate (3) false, while
moving `describe`'s arm without touching the capture makes alt text fail with
`unsupported markdown construct 'supported construct'` — a nonsense message. It
is the interaction `mpdf-001` Phase 8 decided for strikethrough and Phase 1
decided for inline math, and the precedent says it is the spec's call, so it is
now in the scope rather than left to an implementer.

Of the seven non-blocking, the two worth recording: **gate (3) cited the wrong
half of the Phase 8 precedent** — no test can show an arm is *un*reachable, and
the applicable half is the one that *dropped* two arms, which turns the gate
into a grep; and **no arm exercised the conditional prelude import on the
display path**, since a fixture whose heads Typst already defines would pass
even if the display arm forgot `Walk.math`. Gate (1)'s fixture is now keyed to a
prelude-only head. The rest were precision: the phase named symbols but no
files, the close-out did not say whether the rule's line cap moves, "extended
rather than duplicated" left the `CORRECTED` note's stamp ambiguous, gate (2)'s
trailing clause read backwards, and the README's "on its own lines" is
imprecise about a refusal that also covers a mid-paragraph span.

### Round 6 — Phase 1 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, three non-blocking, all three folded after the
verdict. **Rounds 4, 5 and 6 are past §7.6's cap and a person authorised each**,
on the grounds round 3's escalation named: the author's fix had introduced the
next blocker in three consecutive rounds, which is what a further pass catches.
It kept catching them — rounds 4 and 5 found a fourth and a fifth instance — so
the authorisations paid for themselves rather than merely being spent.

**The convergence came from a deletion, not a rule.** Round 5's findings were all
about `\text{…}`, and the response was to take `\text` off the allowed list and
defer it to OQ-6. All four dissolved rather than being answered. The property
that closes the whole six-round family is now stated and true: **no command on
the allowed list reaches Typst's markup mode**, so the shape that produced every
one of the six findings has no seventh instance available to it.

**The best finding of the round was against the paragraph defending that
deletion.** §2 claimed the deferral was cheap because composition still worked —
`$100\% \text{ of y}$`, the leading part converting. The removal is exactly what
falsifies it: `\text` is an unlisted control sequence, so the scan refuses the
whole span before `convert_math` runs, and there is no leading part. The
reviewer noted the irony to stop it recurring — its own round-5 phrasing had
contrasted "prose inside the group" with "prose beside a formula" to argue the
cost was small, and after the removal the cost *is* prose beside a formula. The
paragraph now gives the exit that is actually available and is better than the
one that was wrong: `$100\%$ of y`, a closed formula with ordinary markdown
prose beside it, which needs no LaTeX and which `mpdf-001` has handled since its
own Phase 1.

Two smaller ones: the scan's closing bullet still said "the two cases above"
where only `%` remains, and the tokenization argument gained a clause recording
that with `\text` gone no allowed command emits a `#` escape or a content block
at all — so that problem has no live instance, while the `itemize` case it rests
on stands alone.

**The round-0 question was re-asked and answered no.** The reviewer was given
standing to call the deferral a round-0 objection — that a math phase without
`\text` does not produce the observable — and declined: §1's usage example uses
no `\text` and both its spans convert, the exit is the natural markdown shape
rather than a workaround, and `mpdf-001` Phase 8's task-list precedent is apt.
It re-ran the allowed list at **138 cases, 0 refusals**.

It also re-derived the prelude over the list as it now stands and got **ten** —
`matrix`, `pmatrix`, `bmatrix`, `vmatrix`, `aligned`, `mitexsqrt`,
`mitexmathbf`, `sect`, `diff`, `negthinspace` — confirming the author's
prediction that `textmath` was the only member to leave with `\text`.

On this convergence: `status: accepted`, and `reviewed: 2026-08-11` on Phase 1.
Phase 2 keeps `reviewed: null` and takes its own round.

### Round 5 — Phase 1 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, four blocking. The author asked three specific questions
about holes the fix's own designer would not think to ask, since round 4's fix
was the reviewer's own suggestion. **All three were holes, and the round found a
fourth.**

- **`.` is not in `core/src/emit.rs:SPECIAL`.** `escape_into` carries it as a
  positional rule instead — `ch == '.' && line_is_all_digits(out)` — so
  `$\text{1. item}$` sets an `EnumItem` while `v1.0 ships` is clean. A rule keyed
  to the constant is one rule short of the function.
- **The rule needed the hand-written list it claimed to avoid.** The scan runs
  before conversion, so evaluating "a character `mitex` does not itself escape"
  means materialising `{#, *, _, @, [, ], ~, /}` — the very list the bullet said
  it was not writing. And refusing `\` would have removed the exit the `%`
  decision rests on, since `\text{100\% sure}` would then be unwritable.
- **The fourth instance, which no character rule reaches.** `mitex` maps a
  control symbol or command to a Typst *math symbol name* and drops it into a
  markup content block, where it is a word: `$\text{50\$ each}$` sets "50dollar
  each", `$\text{a \alpha b}$` sets "a alpha b". Both use constructs the dialect
  allows everywhere else, and outside a `\text` group the same mapping is
  correct.
- **The group boundary was undefined** for three reachable inputs, and one
  branch shipped the case gate (3) called sharpest: `\text{= head` unclosed is
  repaired by `mitex` and still emits the heading, so a scanner requiring balance
  would check nothing and under-refuse exactly that case.

The author's response was to remove `\text` rather than to widen the rule, on
the reading that the last finding is evidence about the construct rather than
about the rule.

### Round 4 — Phase 1 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, one blocking, two non-blocking. Round 3's `%` blocker was
confirmed resolved, and the reviewer verified the half that makes the fix a
redirection rather than a ban rather than assuming it: `100\% of y` converts and
parses under Typst 0.15.1 with zero error nodes, so the escaped form does reach
the page.

**The finding was the fourth instance of the shape, and the author had asked for
it.** `\text` content was worse than the single case round 3 recorded: `mitex`
escapes **eight** of the characters Typst markup interprets — `#`, `*`, `_`,
`@`, `[`, `]`, `~`, `/` — and leaves **six**: `` ` ``, `<`, `>`, `-`, `+`, `=`.
The spec had said five and two. Parsed with Typst's own `typst::syntax::parse`,
`$\text{= head}$` yields a `Heading`, `$\text{- item}$` a `ListItem`,
`$\text{+ item}$` an `EnumItem`, a backtick pair a `Raw` block — all arriving as
a single `Event::InlineMath`, the backtick case included, because the math span
wins over the code span.

The reviewer's proposed fix was to key the rule to `core/src/emit.rs:SPECIAL`
rather than to a character list, on the grounds that the constant is already
maintained against Typst's markup mode for `escape_into`'s sake. The author took
it. Round 5 then found that fix insufficient — recorded above — and the
subsection it produced survives as the record of why `\text` is deferred.

The round also bounded the problem usefully, and that bound is what made the
later deletion clean: re-sweeping all 139 allowed-list cases, `\text` was the
only member emitting a `#` escape at all, and in plain math mode `-`, `=`, `<`,
`>` and a backtick parse as operators with no markup nodes.

### Round 3 — Phase 1 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY (escalated at the cap)**

Verdict: `NOT READY`, one blocking finding, five non-blocking. This is §7.6's cap,
so the episode **escalates to the human rather than running a fourth round**, and
Phase 1 keeps `reviewed: null` — the date records convergence, and this did not
converge. `status` stays `draft`.

All four round-2 blockers were confirmed resolved against the files. The reviewer
also answered both questions the author put to it, by measurement rather than by
opinion, and both answers are worth more than the finding they came with.

**The derivation the spec declines to pre-compute was run end to end**, over all
139 cases of the allowed list: convert each, take the multi-character heads,
subtract `typst::Library::default()`'s global, `math` and `sym` scopes, keep the
remainder. It terminates in minutes and yields a stable 11-member set —
`matrix`, `pmatrix`, `bmatrix`, `vmatrix`, `aligned`, `mitexsqrt`,
`mitexmathbf`, `textmath`, `sect`, `diff`, `negthinspace`. **Two of those eleven
are Typst version skew rather than MiTeX helpers**: mitex 0.2.4 emits the
pre-0.13 spellings `sect` and `diff` where Typst 0.15.1 has `inter` and
`partial`. Those are precisely the entries a hand-written list omits, because
they look like ordinary Typst symbols — which is the strongest available
argument for the spec's decision to state the derivation and let gate (1) prove
it, and the round said so.

**The clean negatives are recorded because they bound the risk.** All 139 cases
convert without error, uppercase Greek included. `\text` is the only allowed
member emitting a `#` escape, and mitex escapes its content — `]`, `#`, `*`,
`_`, `@` — so the injection the round went looking for does not exist. Nothing
on the list emits a markup-mode list marker, a heading or a figure.

**The blocker is the `%` comment, and it is the third instance of one shape.**
§2 records the scan not modelling `%` as a limit that "errs toward refusing".
That is only half true, and the other half is a silent drop: `$x = 100 % of y$`
converts to `x  =  1 0 0  ` with " of y" gone, `$\text{100% sure}$` loses
"sure", and no control sequence, control symbol or environment appears anywhere
in those spans, so every arm of the scan passes them and the output is non-empty.
A document converts and its PDF shows truncated prose — `mpdf-001` §2's cardinal
case, reachable by typing `100%` in a formula. A milder second: `\text{<tag>}`
becomes a Typst label rather than text, because mitex escapes five characters in
`\text` content but not `<` and `>`. The fix is small and the grammar already
carries the hook, since `\%` is on the control-symbol list.

**This is the same shape as round 2's `itemize` and round 1's `\includegraphics`**
— content the check cannot see because it is looking at the wrong tokens — and
it is the third round running in which the author's own fix introduced the next
blocker. That pattern is what §7.3 warns about and what the escalation hands to
a person.

The five non-blocking, recorded for whoever closes this: the scan's fourth
bullet ("anything else is refused") has no reading that works — vacuous under
the narrow one, and under the literal one it refuses `x` and `1`; §2's informal
prelude example names `zws`, which **Typst 0.15.1 already defines**, and misses
`sect`, `diff` and `negthinspace`; two stale `180`s survive the correction to
168; the derivation's subtraction step should pin the Typst version, since
`sect` and `diff` prove the set moves between releases; and round 1's recorded
decision about what mitex repairs silently was lost in the round-2 rewrite while
the behaviour survives and is scan-clean (`\frac{a}{` → `frac(a ,zws )`).

### Round 2 — Phase 1 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, four blocking findings, two non-blocking. Seven of round
1's eight blockers were confirmed resolved against the files, and the reviewer
**withdrew one of its own round-1 findings** after checking the corpus: the
author had rejected the suggestion to carry `extends: mpdf-001`, on the grounds
that `mpdf-001` §1.1 is a non-goals list rather than a §2 framework reserving
named kinds, and that `mpdf-002` and `mpdf-003` both sit in exactly this relation
with `extends: null`. The reviewer verified both siblings and agreed the reading
was right rather than the precedent wrong.

**The blocker that mattered killed the design round 1 produced.** Round 1's
eighth finding had established that `mitex` can emit constructs that escape the
image asset contract, and the author's fix was to check the *converted output*:
every multi-character identifier had to resolve in Typst's math scope or a
bundled prelude, no unsanctioned `#` escape, no empty output from non-empty
input. Round 2 falsified it in one line —
`$\begin{itemize}\item a\end{itemize}$` converts to `"\n-  a "`, which has no
multi-character identifier, no `#` escape and is not empty. Every arm passes,
Typst reads `-` as an ordinary operator, and the list flattens to `− a`. **The
silent flattening `mpdf-001` §2 exists to prevent, surviving the check
introduced to stop it.**

The round also found that rule was never well-defined: deciding which tokens in
Typst markup are identifiers needs a parser, and the naive scan refuses
`\text{hello world}`, whose `hello` and `world` are content-block text — a
construct the same phase intended to support. And the author's claim that the
`#` escapes number four was refuted: the four hardcoded writes in `converter.rs`
were right, but the spec data contributes **18** more, with `#footnote`,
`#textbf`, `#mitexcite`, `#heading` and `#miteximage` all verified reaching
`convert_math` output. The spec's own instruction to "treat a fifth as a
blocker" would have halted an implementer on the first `\textbf` fixture.

The fourth blocker was that three open questions still blocked Phase 1 while its
deliverable and two gate cases were defined by reference to their unanswered
state, against the methodology's §4.

**The reviewer corrected its own round-1 number at the author's request**, and
downward: 611 distinct emitted heads is right, but **168 are absent from Typst's
scopes, not 180** — 12 of the 611 are not identifiers at all (`.`, `...`, and
ten Unicode literals), and `sym` had not been consulted. The design conclusion
was untouched; the figure in the spec was wrong and is now 168.

The author's fix changed the instrument rather than the wording: the check moved
to the LaTeX input, ahead of conversion.

### Round 1 — Phase 1 only — 2026-08-11 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0, for this episode:** yes. Phase 1 produces the observable — the typeset
PDF — for documents that today produce none at all, since any inline `$…$` makes
`md2pdf` exit non-zero. It is also the right half to build first: it carries the
dependency, the prelude and the error path, leaving Phase 2 only what a block
needs beyond a span.

Verdict: `NOT READY`, eight blocking findings, six non-blocking. The reviewer
built and ran probes against `mitex` 0.2.4 rather than reading about it, which is
what produced every measurement in this episode.

**The central blocker refuted the number the design was sized by.** §2 claimed
"exactly five identifiers" that Typst does not define, inferred from 26 sampled
formulas. Enumerated instead: `mitex-spec-gen` 0.2.4's `DEFAULT_SPEC` emits 611
distinct heads, of which 180 (later corrected to 168) are absent from Typst
0.15.1 — the whole matrix family, `aligned`, `substack`, `operatorname`, the
`xrightarrow` and `text*` families, 24 `mitex*` names. **The spec's own corpus
refuted its own count**: §2 attributed `zws` to a matrix separator, so a matrix
was among the 26, and that matrix's head `matrix` is undefined in Typst and was
not among the five. A sample cannot support a completeness claim, and the
correction moved the design from "bundle a small prelude" to "bound the subset".

**Three findings were semantic rather than lexical, and no symbol fix would have
touched them.** `$\includegraphics{fig.png}$` converts to `#image("fig.png")` for
a path that never passed `core/src/emit.rs:check_image`, never joined the walk's
image list, and is absent from `core/src/lib.rs:collect`'s asset map — the exact
failure that pre-compile check exists to prevent. `$\label{eq}$` converts to the
empty string: the construct vanishes with nothing on the page and no error.
`$\begin{itemize}…$` emits markup-mode list syntax inside `$…$`.

Two more were grounding errors in the draft. "The mechanism exists and this adds
an asset to it" was false — `core/src/lib.rs:TypstWorld` builds its sources from
`frontmatter::Template::ALL`, a closed two-variant enum, so a third bundled
`.typ` returns `FileError::NotFound`; and a `Template` variant is the wrong fix
because `Template::from_name` would make the prelude selectable from the
`template:` frontmatter key. And the preferred import shape broke the phase's own
gate: all 15 shipped golden files begin with the two lines
`core/src/emit.rs:header` writes, so an unconditional third import contradicts
"no shipped golden file changed" — resolved by making the import conditional,
which `core/src/emit.rs:emit` permits because it completes its walk before
calling `header`.

The remaining blockers: two open questions marked "Blocks Phase 1" were
unanswered while the deliverable was defined by their answers; OQ-2's stated
method could not have produced the list it promised, since the spec data lives in
`mitex-spec-gen` rather than `mitex-spec`, `zws` is written by the converter at
nine sites and appears in no spec data, and nine environments carry no alias;
math inside image alt text was undecided, with the phase's claim that
`describe`'s math arm stops being reachable shown false; and the prelude's
provenance was self-contradictory, described as `core`'s own and attributed to
Apache-2.0 in the same phase.

Of the six non-blocking, all folded: the unreproducible "26 of 26" corpus, a
citation naming `step` where `emit` and `collect_definitions` do the walking, the
25 transitive crates one direct dependency brings and `mitex-spec-gen`'s build
script, three existing tests asserting inline math is refused, and OQ-6 —
answered in the round and closed, since `$frac(a,b)$` converts to
`f r a c \(a \,b \)` and sets as letter-spaced garbage with no error, confirming
"confusing PDF" rather than "clean error".
