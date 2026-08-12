# Review record — mpdf-004 (`specs/math_spec.md`)

Append-only. One heading per round, newest first.

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
