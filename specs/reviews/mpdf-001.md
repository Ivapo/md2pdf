# Review record — mpdf-001 (`specs/md_to_pdf_pipeline_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 14 only — 2026-09-04 — the same three lenses, resumed — **READY (converged)**

Verdict: `READY`, **zero blocking from all three**. Phase 14's `reviewed` is set to
2026-09-04. Converged in two rounds.

**Every lens verified against the files rather than the changelog, and each re-ran its own
measurements.** All three rebuilt the pinned derive *with* the doc comment against the locked
clap 4.6.6 and rendered `--help` at default width, 80 and 40 columns — one line each time — and
two of them established why beyond `cli/Cargo.toml`: `wrap_help` is off workspace-wide, since
`Cargo.lock` carries no `terminal_size`, and `clap_derive`'s markdown fallback merges the two
doc lines into one `help` string with no `long_help`. Each re-extracted the pinned notice and
counted 39 lines against the new 50-line bound; each re-ran the split rule and got eleven
terms trimmed, thirteen untrimmed, all eleven substrings of the notice; each confirmed mitex's
`repository` through `cargo metadata` and the five URLs at 200; each read `5cf8948` and agreed
`core` already owes a 0.1.3; each re-ran `spec-lint` at 0 errors and 62 warnings and confirmed
the diff against `HEAD` lands only in the frontmatter, the Phase 13 note and Phase 14.

**The dated note on Phase 13's gate (5) was put to all three, and all three called it the
right mechanism.** The claim it corrects — that the packaging run reaches a run-time read
anchored on `CARGO_MANIFEST_DIR` — was false on the day Phase 13 shipped, not made false by
this phase: `cargo package` builds and never runs, and both root licence files are inside the
archive it builds from. That is §6.1 step 1's dated note in place, with direct precedent in this
document at Phase 11's note of 2026-08-31; it is an insertion rather than a scope edit, so
Phase 13's `reviewed` stands; and it is coherent with this phase's own refusal of a note for
the flag's output — a decision true when shipped gets none, a claim false when written gets
one. The scope lens also conceded the author's call on `core`'s doc sentence: the version-bump
argument does not bite, because `cargo package -p md2pdf-cli` resolves the registry `core`
whatever a local doc line says.

**Four non-blocking findings, all folded.** Three lenses found the same residue: the falsified
claim has a **second copy**, in Phase 13's gate (4) — *"it closes case (5)'s stated limit in
the same stroke"* — which the note now names as well, so no uncorrected copy is left. Two
found that `clap_derive` **strips the doc comment's final period**, so *"the doc comment's two
sentences"* was not byte-verbatim; the table row and the paragraph now say *"less its final
period"*. The scope lens found step 1's *"earns no `CORRECTED` note"* sitting in the phase that
adds one to Phase 13 with neither passage stating the distinction; step 1 now does. And the
correctness lens found the **Phase 2 precedent weaker than it read** — Phase 1's own text had
already said *"Phase 2 parses it"*, so Phase 2 contradicted nothing, where Phase 13 announced
no successor; step 1 now marks the precedent as supporting and lets the quoted §6.1 rule carry
the argument.

**Rejections: none.**

### Round 1 — Phase 14 only — 2026-09-04 — three fresh lenses (correctness/grounding, exit-gate testability, scope/YAGNI + cross-file consistency) — **READY**

Verdict: `READY`. **All three lenses independently returned READY with zero blocking**, and
between them raised thirteen distinct non-blocking findings once deduplicated — six from the
correctness lens, seven from the gate lens, six from the scope lens, several shared. **The
author accepted all thirteen and folded them; rejected none outright; deferred none.** One
adjudication went between two lenses' opposing advice and is recorded below. Because a fold
happened, the same three lenses were resumed for a round 2 (§7.4), which the loop requires
after any change at all.

**Round 0, asked once for this episode and answered by the author.** Phase 14 produces no
observable and argues it on Phase 13's ground — a branch that returns before the pipeline is
entered, with gate (6) pinning the page. *Is it the right thing to build?* Yes: the bare flag's
982 lines bury what a binary-only reader needs told, two sibling tools on this machine print
the notice shape in 32 and 19 lines, and Phase 13's consumer keeps the texts at
`--licenses=full`. The episode proceeded.

**What the three lenses re-derived, each on its own, and all agree on.** Today's
`md2pdf --licenses` is 982 lines, from the installed 0.1.2 and from assembling the four files;
`dbiew --licenses` 32 and `oko --licenses` 19; 334 table rows, 23 distinct expressions, and
exactly eleven terms by the stated split rule, the set equal to the notice's eleven with
nothing extra and nothing missing; `typst` 0.15.1 and `mitex` 0.2.4 in the table and in
`Cargo.lock`; one `WITH LLVM-exception` row (`wasmparser`); the case-insensitive copyleft grep
over rows hits exactly `simplecss`, `thiserror-impl`, `unic-langid-impl` and
`unic-langid-macros-impl`, and zero terms; five `Libertinus*.otf` and one
`NewCMMath-Regular.otf`; `LICENSE` line 3 is the copyright line; `rules/pipeline.md` at
1279/1280 with both quoted sentences verbatim under `## The CLI`; `spec-lint` 0 errors and 62
`CIT_UNRESOLVED_PATH`; the showcase PDF hash-stable across two compiles, so gate (6) is a real
check; every cited symbol present; all URLs answering 200. **All three lenses built the
prescribed clap derive against `=4.6.6` in a debug build and reproduced every row of the
grammar table**, `require_equals` doing the job the scope says — the shape Phase 13's round 1
found unmeasured was measured before this phase was written, and the panel confirmed it three
times over.

**The thirteen findings, and the fold.** The three the author counts as substantive:

1. **The `--help` text was the one user-facing surface neither pinned nor reconciled** (all
   three lenses). The field's doc comment today — *"Print every licence this binary carries and
   exit…"* — goes false for the bare flag, and the grammar table's `--help` row had been
   measured with no doc comment at all. Folded: the derive snippet carries a two-sentence doc
   comment, the row describes what renders, and a paragraph records the measurement — one line
   at any width, `cli/Cargo.toml` enabling `derive` and not `wrap_help`.
2. **Gate (3)'s split rule omitted "trimmed"** (gate lens). `fnv`'s cell is `Apache-2.0 / MIT`
   with spaces about the slash; followed literally the rule yields thirteen terms where the
   prose says eleven, and the substring assertions pass by accident. Folded, with the example
   written into the case. This is §7.3's *"a fix can introduce a blocker"* caught one round
   early: the rule was new in this phase and a lens ran it rather than reading it.
3. **Gate (5) inherited an overstatement from Phase 13** (gate lens): *"closed by case (7)'s
   packaging run"*. `cargo package` builds and never runs, and both root licence files are
   inside the archive, so a run-time read anchored on `CARGO_MANIFEST_DIR` passes (5) and (7)
   alike. Folded in two places — Phase 14's (5) now states the limit as it stands, and **a
   dated `CORRECTED` note was added beside Phase 13's gate (5)**, whose *"Case (4)'s packaging
   run is what reaches that"* is a shipped claim about what a gate proves, now known false.
   §6.1 step 1's third bullet is the mechanism; the original sentence is kept. This is the
   one edit outside the phase under review, and round 2 was asked to judge it.

The other ten: gate (3) *"matches"* became *"contains"*, so a future `LGPL-2.1` is caught;
gate (4)'s *"from `cli/tests/`"* became *"from `CARGO_MANIFEST_DIR`, which is `cli/`"*; gate
(4) now derives the face-count word from the count rather than pinning `five faces` beside a
`5`, to gate (3)'s own standard, closing a swap the gate lens named; gate (1)'s
*"byte-identical stdout"* now says to what; gate (2)'s swap detector was given a home in case
(1)'s test; the OQ-15 analogy was corrected — case (3) is notice-against-file, OQ-15 is
file-against-`cargo metadata`, a different pair; step 1's *"no `CORRECTED` note"* argument now
stands on §6.1's rule for shipped phases, quoted, rather than on the narrower *"§2 never
states it"*; README `## Use`'s `--licenses` comment line was named as going false; gate (7)
now says *"no `include_str!`"* reaches outside the archive rather than *"nothing"*; and
**mitex was filed under Typst's project line** — its manifest's `repository` is
`github.com/mitex-rs/mitex`, so it has a block of its own, the notice runs 39 lines and gate
(1)'s bound moved from 40 to 50.

**One call between lenses, recorded.** `core/src/lib.rs:FONT_LICENSES`'s doc comment says
*"`md2pdf --licenses` is the first such caller"*. The correctness lens said widen the first
commit by that sentence or state the staleness; the scope lens said leave `core` alone — a
published crate, an edit costs a bump — and say so. **The author widened**: a doc comment that
names its caller wrongly is the drift `rules/` refuses, one file over; the edit needs no
release of its own because `5cf8948` already owes `core` a 0.1.3, and `cargo package -p
md2pdf-cli` resolves the published 0.1.2 regardless of a local doc line. The scope lens's
alternative is recorded in the scope with the reason it was not taken.

**Rejections: none outright.** The scope lens's advice on the doc comment was the one partial
one, above.

### Round 2 — Phase 13 only — 2026-09-02 — the same three lenses, resumed — **READY (converged)**

Verdict: `READY`, **zero blocking from all three**. Phase 13's `reviewed` is set to
2026-09-02. Converged in two rounds.

**Every lens verified against the files rather than the changelog, and each re-ran its own
round-1 measurement.** The correctness lens re-downloaded `md2pdf-cli-0.1.1.crate` to
confirm 1264 and 37041; the gate lens re-ran `spec-lint` and re-checked the GUST literal's
uniqueness; the scope lens took a class census of the linter's output — 62
`CIT_UNRESOLVED_PATH` and nothing else — and grepped the spec for
`(core/src/lib\.rs|cli/src/main\.rs):FONT_LICENSES`, no match, confirming the
`CIT_SYMBOL_ABSENT` the author had introduced and fixed was gone corpus-wide.

**Both rejections were conceded by the lens that lost them.** The scope lens withdrew its
1260-line reading — its round-1 python had split on `\n` and counted the trailing-newline
artifact; `spec-lint` prints `1259/1265` and `wc -l` minus a frontmatter ending at 38 gives
1259. It also conceded `mpdf-011` §1.2 over its own §1.1. And on the clap rationale it
conceded the author's split of the two lenses' readings: *"with `Option` alone, bare
`md2pdf` would exit 0 into `run` and this phase would have to author a message"*, so the
"acquire no message" goal survives and attaches to the attribute — its round-1 "the whole
argument dissolves" was true only as an argument against the `Option`.

**Six non-blocking findings, all folded.** Two were the fold's own renumbering debris: the
phase's opening line still said *"gate (4) is the PDF not moving"* after five cases became
seven, which is the §3 discharge for producing no observable pointing at the wrong clause;
and the close-out called `rules/pipeline.md:66` its paragraph's *opening* sentence when it
is the fourth. Two were pointer precision — the *"a fix can introduce a blocker"* literal
lives in `loops/review-spec.md`'s §7.3 section rather than in `spec-authoring.md` §7 rule
3, and the description of a published `md2pdf-cli` archive's contents was incomplete in the
half that is not load-bearing.

**The two that mattered were a residue the round-1 fold left.** The gate lens found that
case (4)'s heading claimed *"and the CLI reads it"* while its body asserted only the
const's shape: an implementation writing the const **and** separately reaching into
`core/assets/` from `cli` passes in a checkout and breaks for every consumer off the
registry — the exact failure the scope invokes as its reason for the export existing. And
case (5) was found to catch a *cwd-relative* run-time read but not a
`concat!(env!("CARGO_MANIFEST_DIR"), …)` one, the repository still being on the machine.
**One instrument closes both**: `cargo package -p md2pdf-cli`, which builds the CLI from its
own archive. Case (4) now carries it and case (5) names its own limit.

**The archive provenance took two rounds and got a route rather than a version number.**
Round 1 called the draft's `md2pdf-cli-0.1.0.crate` wrong; round 2 found the correction
naming a `0.1.1` archive not present on the machine. Both were true about the mechanism and
unreachable as evidence, and the reason is a trap now written into the scope:
`target/package/md2pdf-cli-0.1.0.crate` is a **local repackage** that does hold both files,
carrying the version number of a published archive that holds neither. The scope now names
where a second person can actually look.

**One observation explicitly not folded**, on the lens's own advice: `Option<PathBuf>` means
`run` unwraps `args.input` on the non-licences path, which is a routine unwrap of a
clap-guaranteed value under an invariant the scope already states.

### Round 1 — Phase 13 only — 2026-09-02 — three fresh lenses (correctness/grounding, exit-gate testability, scope/YAGNI + cross-file consistency) — **NOT READY**

Verdict: `NOT READY`. **All three lenses independently returned NOT READY**, and between
them raised three blockers and thirteen non-blocking findings.

**Round 0, asked once for this episode and answered by the author.** Phase 13 produces no
observable, and the phase argues it rather than assuming it. *Is it the right thing to
build?* Answered **yes, with a recorded reservation**, and the reservation was deliberately
kept out of the reviewer prompts so a scope lens would have to find it independently. **It
did.** `.github/workflows/` holds `test.yml` and nothing else — no release workflow, no
bottle, no bundle — and both documented install paths build on the reader's own machine,
leaving the licence files on disk beside the source. So the consumer this phase serves is
the third party handed a *hand-copied* binary, and this project ships none. The phase's
opening argument was rewritten down to that, with the `core` half separated out as the part
that is not hypothetical: `include_str!` cannot reach `core/assets/fonts/` from a published
`md2pdf-cli` archive, so the CLI's own requirement forces the export whatever anyone
downstream ever does.

**Blocker 1 — the phase pinned a mechanism as a decision and pinned the one that cannot
run.** The scope had `input` taking `required_unless_present = "licenses"` and *not*
becoming `Option<PathBuf>`. **All three lenses built that exact shape against this
workspace's clap 4.6.6 and ran it**: the derive infers `required(true)` from a non-`Option`
field, which collides with the attribute, so a **debug** build — the one `cargo test` runs —
panics on every invocation at exit 101, and a **release** build lets `required` win and
refuses `--licenses` at exit 2. `required = false` does not rescue it, `clap_derive` emitting
`ok_or_else(… MissingRequiredArgument …)` for any non-`Option` field. Gate (1)'s first clause
was unreachable from the prescribed code and gate (5) would have failed with it.

**The recorded rationale was wrong in its shape, not only its answer**, and that is why the
fold rewrote it rather than deleting it: it framed the two as alternatives trading clap's own
bare-invocation message against one this phase would author. They are complements. The scope
now requires both, and says so.

**Blocker 2 — no gate clause read the `core` half of the scope, and the close-out gave the
new export no home.** Raised from two sides. The gate lens: cases (1)–(5) read only `cli`, so
an implementation putting all four `include_str!`s in `cli/src/main.rs` passed the entire
gate unchanged. The scope lens: the scope argued the const's name was a decision *because*
`rules/pipeline.md` would cite it, then named two close-out sites, both inside `## The CLI`,
neither of which documents a `core` export — **and missed the sentence the export falsifies**,
`rules/pipeline.md:66`'s *"`core/src/emit.rs:IMAGE_EXTENSIONS` is the crate's one non-function
export"*. Following that close-out would have shipped the artifact that must track the code
asserting something false. Fixed both ways: a new gate case over `core`, and the falsified
sentence named as the citation's home. `core/src/lib.rs`'s own *"first re-export"* doc comment
was checked and survives — `FONT_LICENSES` is a definition, not a re-export.

**Blocker 3 — the scope and gate (2) specified two different byte streams.** The scope printed
each font licence *"under its filename"*; gate (2) asserted stdout equalled *"the four files
concatenated"*. Raised blocking by one lens and non-blocking by two; taken at the higher
classification because it forces a guess about a user-facing output format **and** because the
same implementer writes the arm and the test, so the gate would have pinned the guess. Now
pinned: four parts joined by `"\n\n"`, font entries rendered `"{filename}\n{text}"`.

**Ten non-blocking findings folded**, of which the substantive ones: the `mpdf-003` OQ-8
citation was **withdrawn rather than repaired**, a lens having read that OQ and found it about
toolchain, fonts and signing with nothing about licence text; two attributions to `mpdf-011`
Phase 3's close-out were corrected to `db69839`, a code-only fix commit, which the phase's own
step 0 had already contradicted; gate (3) gained a fifth literal, `of the GUST Font License`,
the maths font's licence having been discriminated by no clause so that a list with both
entries pointing at `OFL.txt` passed everything; a new case was added for running the binary
outside the tree, the phase's own title claim being read by nothing; gate (1)'s `extra.md`
clause now states its dependence on that path not existing; the `spec-lint` absolute path is
given; the over-listing figure became *"eight crate names, or nine name-version pairs"*; and
`CLAUDE.md` and the status artifact are now stated as "none needed" with reasons, which §3's
reconciliation step asks for by name.

**Two rejections.** A lens read `rules/pipeline.md` at 1260 body lines; it is **1259**, by
`spec-lint`'s own print and by two hand methods. And a lens cited `mpdf-011` §1.1 for the
distribution non-goal; it is §1.2.

**The fold introduced a blocker of its own and the phase's own rule caught it.** The new
close-out paragraph spelled the const as a `core/src/lib.rs:FONT_LICENSES` citation —
precisely what the phase's own gate forbids for a symbol that does not exist yet — and
`spec-lint` reported `CIT_SYMBOL_ABSENT` on the spot, taking the corpus from 0 errors to 1.
Rewritten as a file plus a name. Recorded because it is §7.3's *"a fix can introduce a
blocker"* landing on the author inside the single pass that wrote the fix, and because the
rule that caught it is one this same phase had argued for two screens earlier.


### Round 3 — Phase 12 only — 2026-08-31 — the same three lenses, resumed — **READY**

Verdict: `READY` from all three, zero blocking. **Converged.** Phase 12's `reviewed` is set to
2026-08-31; `status` was already `accepted`.

The single round-2 blocker verified resolved by all three independently, each measuring rather
than taking the changelog: `grep -c 'core/src/frontmatter.rs:check_affiliations'` on the spec
returns 0, and `spec-lint` reports 29 citations where it reported 30. Two lenses re-extracted
**every** `file:symbol` citation in the document and resolved each against its file —
`check_affiliations` is the only symbol this phase renames, so nothing else goes stale when it
lands.

The exit-gate lens re-ran the four changed or added rows of gate (3) against a rebuilt binary
and got a real refusal from each, confirming all five refusals the code carries are covered.
The cross-file lens verified §2's corrected counter-example empirically — a probe through
`--emit-typst` returns the literal `\[\@smith2020\]`, so the fallback really does put wrong
glyphs in a sentence, which is the mechanism a round-1 draft had backwards.

Two non-blocking folded at convergence: gate (2)'s aside explaining the press release's extra
space was an unverified causal guess and is now "measured, not explained, since the gate does
not rest on why"; and gate (5) now names the one inherited `spec-lint` warning
(`rules/desktop-geometry.md`'s `RULE_SOURCES_WITHOUT_GENERATED`) so an implementer does not
chase it.

### Round 2 — Phase 12 only — 2026-08-31 — the same three lenses, resumed with the author's changelog — **NOT READY**

All three returned `NOT READY` with **the same single blocker, and it was the round-1 fix
defeating itself** — the pattern the loop warns about, observed for the second episode running.

Round 1's fix for the rename was to cite `core/src/frontmatter.rs` by **file** rather than by
symbol, so nothing in an append-only document would go stale when `resolve_affiliations` lands.
The close-out written in the same pass then used the full `file:symbol` form four paragraphs
later. `spec-lint` raises `CIT_SYMBOL_ABSENT` at **error** severity and gate (5) requires it to
run clean, so the phase could not pass its own gate, and the document being append-only meant
the line could not be corrected afterwards. The scope's own paragraph telling an implementer
the hazard was handled is what made it worse. Fixed by dropping the path; the citation count
fell 30 → 29.

All three confirmed every round-1 blocker resolved, with work rather than assent: one ran every
row of gate (3) against the built binary, one computed the SHA-256 pair to show the unnamed
digest was a real hazard, one re-swept the repo and confirmed no prose site remained missed.

Seven non-blocking folded, three of them the same class the blocker was: **a fact this phase
had checked and then miscounted.** The `.typ` census said "both use the word" where
`core/assets/press-release.typ` uses it zero times; "round 1 rewrote three of them" was five;
"seven sites" was eight. Also folded: §2's `check_citations` counter-example had its mechanism
backwards — the escaped brackets are what the refusal *prevents*, not what it causes; gate (4)
publishes SHA-1 hashes and now names the digest, since `shasum -a 256` yields a different pair a
second person would read as a failure; gate (3) gained a row for the **fifth** refusal §2 had
just promoted, and split an "either list, under none" row that was vacuous for the affiliation
half, an empty element there presupposing the key; and gate (2)'s illustrative dump is now
qualified as the article byline.

### Round 1 — Phase 12 only — 2026-08-31 — a three-lens panel of fresh reviewers with repo access (correctness/grounding, exit-gate testability, cross-file consistency) — **NOT READY**

**Round 0 (this episode — Phase 12 is its own, per §7.0):** *does this phase produce the
observable, and is it the right one?* **Yes to both.** It produces a typeset PDF from a document
that today does not compile at all, its byline setting the names their author wrote; and it is
the right one because it narrows a refusal a real user hit on the shipped build rather than
adding a capability nobody asked for.

All three lenses returned `NOT READY`. Four distinct blockers, each raised by two or three of
them independently.

**Gate (3) named an input a correct build cannot refuse.** `^1` under one affiliation is valid
today and stays valid — `core/src/frontmatter.rs:one_affiliation_makes_the_marker_optional`
already asserts it — so the case as drafted was unwritable. **The boundary this phase moves is
at zero, not one**; the row that actually guards it is `^2` under one, which is what fails an
implementation over-narrowing to `count <= 1`.

**Gate (5)'s diff enumeration was failed by a correct implementation of gate (1).** Gate (1)
needs a checked-in assertion and every such assertion in this repo lives in
`core/tests/golden_test.rs`, which gate (5)'s "and nothing else" excluded — along with the spec
itself and the two indices the close-out regenerates. Gate (1) now names its file and the
`include_str!`-plus-inline-assertion shape.

**The rename broke a live `file:symbol` citation while gate (5) requires `spec-lint`.** The
sharpest catch of the round: the scope said "is renamed to say so" without giving a name, and
`rules/pipeline.md` cites the old one. The name is now given — `resolve_affiliations` — and the
phase's own scope cites the file rather than the symbol.

**"One code comment states the refusal absolutely" undercounted by three**, inside the one file
the phase edits, with `fn author`'s doc the one that becomes **false** rather than merely stale.

**§2's central argument was falsified.** A draft bounded the bend in the escape-and-reject
decision by saying a marker "is not content: it is one end of a relation". The relation argument
proves too much — a citation key with no `bibliography` is the same shape and
`core/src/emit.rs:check_citations` **refuses** it — so the argument licensed dropping citations.
The bound is now what the drop leaves on the page: dropping a marker removes no glyph a reader
could misread, where the citation's fallback puts wrong glyphs in a sentence.

Also folded: `samples/showcase/showcase.md` carries the same one-affiliation sentence and was
unnamed, **with the `[14, 29, 68]` line-count trap Phase 11's own close-out had named at line 60
before the showcase grew**; `rules/pipeline.md`'s *looks* paragraph is a third site in that file,
and the one a reader consults for what becomes of a marker; the correction blamed the shipped
error message for a reading that comes from §2's absolute bullet, the message having already
been reworded by commit `ceb5145`; OQ-11's continuity carries its optional half and breaks its
honouring half; and **the shipped code carries a fifth refusal this document had never
counted** — `author: ^1`, an entry with no name before its `^` — recorded as a discrepancy
rather than renumbered, since the tally has been wrong since Phase 11 shipped.

**One rejection, recorded.** The cross-file lens asked for the `CORRECTED` block to sit beside
refusal 1's bullet rather than ~50 lines below it. Not moved: §6.1's "beside the text it
corrects" is satisfied within the `###` subsection, which is the precedent Phase 11 set in this
same section, and inserting a pointer into Phase 11's shipped bullet would edit shipped prose in
an append-only document to fix a navigation problem.

### Round 3 — Phase 11 only — 2026-08-31 — the two panel lenses that were NOT READY, resumed — **READY**

Verdict: `READY` from both, zero blocking. **Converged.** Phase 11's `reviewed` is set to
2026-08-31; `status` was already `accepted`.

Both round-2 blockers verified resolved against the files. The affiliations sit **directly
beneath the authors in both looks**, and gate (2) carries **each look's own three joins** —
article title→authors, authors→affiliations, affiliations→date; press release date→title,
title→authors, authors→affiliations. Refusal 2's line is assigned by the phase rather than
credited to §2, and `Frontmatter` carries a `Location` for **both** keys.

The consistency lens re-verified every quoted fragment in the enlarged close-out verbatim —
twenty-three of them across nine files — and found no newly-introduced false claim. It also
confirmed the sharpened showcase bound independently: `grep -n "^#" samples/showcase/showcase.md`
returns 13, 28, 60, matching `app/src/preview.rs`'s pin, so "no line added or removed above
line 60" is the real constraint and the close-out's own line-34 correction is a re-wrap risk
under it. The cap history checks out in `git log -p`: 960 → 1010 at `mpdf-005` Phase 8, 1010
→ 1020 at Phase 10, 1015 body lines today, so 1070 is calibrated rather than picked.

Five non-blocking folded at convergence: §2's standing "Three refusals, each naming the
author's **own** line" is falsified in that last respect too and the correction block now
says so; gate (4)'s label for refusal 4 was narrower than the widened refusal; `affiliation`
reaches `none` but not through `typst_string_or_none`, which renders a string literal from an
`Option<&str>`; `rules/web-demo.md` carries "twelve" on eight lines where the close-out named
one; and `core/tests/golden_test.rs`'s doc comment on
`every_bundled_template_meets_the_call_contract` is a **fifth** prose statement of the
contract count, three lines above code gate (7) already edits.

### Round 2 — Phase 11 only — 2026-08-31 — the same three lenses, resumed with the author's changelog — **NOT READY**

Grounding returned `READY`; exit-gate and cross-file returned `NOT READY` with **two new
blockers, both introduced by the round-1 fix** — the pattern the loop warns about, observed.

**The affiliations' position, found independently by two lenses.** Round 1 caught the
placement being stated only in the close-out; folding it into the scope wrote "between the
authors and the date", which is article order. `core/assets/press-release.typ`'s masthead is
date → title → author → `divider`, so the phase's one positional instruction was
unsatisfiable in one of the two looks it named, and gate (2) inherited it as a single
article-order join list — the same enumeration failure Phase 10's round 2 caught, one level
up. **Refusal 2's line, stated two ways**: "four refusals, each naming the author's line" and
then "refusal 2 names the `affiliation` line", with the assignment credited to §2, which
never makes one, and line-carrying state provisioned for one key where the refinement needs
two.

**One lens was right against the other two, and the author's own check decided it.** Round
1's grounding lens reported that `core/src/frontmatter.rs`'s tests assert the problem alone;
the author wrote that into gate (4) as a correction and the cross-file lens repeated it in
round 2. The exit-gate lens disputed it. `errors_name_the_key_and_the_line` asserts
`location.line` **and** `problem.contains(needle)`, and there is a second `location.line`
assertion in the file. A false claim about the code would have landed permanently in an
append-only document on a 2-to-1 consensus. The gate now cites that test as the shape it uses
and keeps the falsified claim visible.

Also folded: refusal 4 widened to an empty element in **either** list — `affiliation: MIT;`
leaves a blank a `^2` would point at without tripping refusal 1; OQ-11's recorded reasoning
corrected, since under its own rules a one-affiliation unmarked document that gains a second
is **refused** by refusal 2 rather than silently re-rendered, so the page cannot move behind
the author; §2's "four prose sites" corrected to five locations across four files, the same
class of undercount the correction was written to fix; the fixture's full key list, since the
press release's first join is its dateline; the `[13, 28, 60]` pin being heading *line
numbers*, so the bound is "no line above line 60"; and the cap given a number, 1070.

### Round 1 — Phase 11 only — 2026-08-31 — a three-lens panel of fresh reviewers with repo access (correctness/grounding, exit-gate testability, cross-file consistency) — **NOT READY**

**Round 0 (this episode — Phase 11 is its own, per the note in Phase 10's round 1):** *does
this produce the observable, and is it the right one?* **Yes to both.** A PDF whose title
block carries several authors and the affiliations their markers point at, and the warrant is
a gap §2 records rather than a speculation: `author` has been one free string since Phase 2,
so a document with two authors has had nowhere to put the second. The one thing worth
pressing — whether affiliations are scope creep beside plain multi-author support — the
cross-file lens worked and declined to assert, on the ground that the `author` type change,
the golden sweep and the contract break are each paid once either way, and that shipping
`;`-lists first would put `^` into documents before it means anything.

All three lenses returned `NOT READY`. **Five blocking findings, deduped; three were raised
independently by all three.**

1. **"Seven of twenty-nine goldens re-blessed" is unsatisfiable.** Gate (5) makes
   `affiliation` an eighth call-contract argument, and `core/src/emit.rs:header` is one
   unconditional `format!` naming every argument on every call — verified by the author
   directly — so **all twenty-nine** call lines move, as OQ-9's `date` moved all thirteen and
   `mpdf-005` Phase 8's seventh argument moved twenty-eight (`2bcb441`). The only build
   satisfying "the other twenty-two must not move" emits `affiliation` conditionally, which
   OQ-10 resolved against. Fixed by naming both counts: 29 move, 7 in two ways, 22 in one.
2. **Gate (3) contradicts gate (4).** `tests/golden/frontmatter.typ` carries
   `author: "Iva Po"` — a one-name document, and one of the seven. Both cases cannot hold.
   §2's survivable wording is that a one-name document's *page* does not move, and gate (5)
   now reads exactly that: the compiled PDF byte-identical, `--emit-typst` changing on the
   call line alone, over a named document.
3. **OQ-11 unresolved, and the gate case it declares it blocks does not exist.** Resolved
   with a **third** answer neither the question nor any reviewer enumerated: the schema makes
   the marker **optional** at exactly one affiliation. The cross-file lens is what forced it,
   by observing that the schema-as-refusal reading would add a *fourth* refusal against §2's
   three; and the deciding fact is that §2's refusal 2 as written made the commonest real
   paper — one lab, several authors — unwritable, leaving only `^1` noise or an error. The
   look answer was rejected: a look suppressing a marker decides a *fact* rather than a
   typographic question, and two looks disagreeing would make one document read as two.
4. **The parsing grammar forces guesses**, caught by the grounding lens alone. Now specified:
   every element trimmed on both lists and each marker, so `^1, 2` and `^1,2` agree; the name
   splits at the **first** `^`, so `A^B^1` is refused naming `B^1` rather than guessed; a
   marker is a digit run indexing `affiliation` from 1. Its empty-element case became a
   **fourth** refusal.
5. **Nothing checked in pins the phase's own output**, caught by the exit-gate lens. Every
   construct- or key-adding phase in this spec ships a fixture and a golden; this one named
   none, and case (1) was a by-eye read over a document it did not name — the shape Phase
   10's round 2 blocked. Now `tests/fixtures/authors.md` and `tests/golden/authors.typ`,
   carrying a comma'd name to pin §2's sharpest call and `^1, 2` to pin the trim.

**One rejection, and both lenses that raised the area accepted it:** no gate case for a
document writing `authors:`. `core/src/frontmatter.rs:parse`'s unknown-key arm already
refuses it and `an_unknown_frontmatter_key_is_an_error_that_names_it` already pins it, so the
no-synonyms rule is shipped code rather than this phase's to prove. **One deferral:**
intra-frontmatter refusal ordering left a free choice, since refusals 1 and 2 can only
resolve after the line loop; the exit-gate lens agreed it is defensible and no shipped
assertion contradicts either answer.

The grounding lens reproduced §2's `super()` measurement in a throwaway clone —
`A Third Person¹˒²`, superscript digits and a superscript comma, no package and no change to
`TypstWorld` — so that claim now stands on two independent runs. The cross-file lens found
the phase's sharpest trap: extending `samples/showcase/showcase.md` with the tenth key breaks
`app/src/preview.rs`'s `[13, 28, 60]` pin, colliding with the phase's own "`app/src`
untouched". Six close-out sites were missed and are now named, and §2's "the contract is
stated in exactly two places" — the mitigation's load-bearing sentence — was falsified.

### Round 2 — Phase 10 only — 2026-08-31 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, five non-blocking, all five folded at convergence. **Both
round-1 blockers resolved against the files**, and the round re-derived every number in a
scratch clone rather than reading the changelog.

**The round added a fourth point to the equivalence sweep and it holds.** Author box top
under the shipped shape at `v(0pt)` / `v(2pt)` / `v(0.4em)` / `v(6pt)`: **79.2352**,
**81.2352**, **83.2352**, **85.2352** — slope exactly 1. The structural shape measures
identically at `0pt`, `2pt` and `6pt`, date line included. The reviewer withdrew its own
round-1 model ("collapses against the line advance, excess applies") as a two-point fit.

Everything else reproduced: `1.8em`/`0.9em` → **+11.22 / +3.93** on both a short and a
wrapping title; press-release `above: 1.0em` → **+3.47** either way, date→title unchanged at
**+2.58**; the 5.07pt threshold exactly; gate (3) byte-identical in both looks and for a
no-key document; `cargo test` green with no golden re-blessed, both named geometric
assertions passing, showcase 6 pages and samples 3 and 1. On `samples/article.md` itself the
join goes **−2.78 → +11.22**.

The five folded: gate (3) published two hashes over a document it never named, now named as
the case-(2) probe is; the owed `rules/pipeline.md` line lands that file at exactly its
`max_lines: 1010` from 1008, so the close-out now **raises the cap to 1020** rather than
trimming a third time; gate (2)'s general clause said "each join" and its enumeration listed
three where the press release has **four** (date→title dropped); §2 attributed the wrapping
effect to the press release's hand-set `par(leading: 0.35em)` when the article look shows it
too at **−1.92**, so the join-scoping is load-bearing in both; and §2's account of the
rejected shape explains the sign but not the size — **+20.4pt** measured where `par.spacing`
defaults to 12pt — now recorded as unexplained rather than implied. One truncation
corrected: the title→author threshold is **6.78pt**, not 6.77.

Gate (5)'s disposal — the fix is a value, a value is not a needle here, so the gap is
disclosed in `rules/pipeline.md` rather than tested — was checked against the four
`BUNDLED_TEMPLATES` tests whose own doc comments say the same, and accepted.

**Converged.** Phase 10's `reviewed` is set to 2026-08-31; `status` was already `accepted`.

### Round 1 — Phase 10 only — 2026-08-31 — fresh general-purpose reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase; Phase 11 is a second episode and gets its
own):** *does this produce the observable, and is it the right one?* **Yes to both.** A PDF
whose author and date stand clear of the title, and the warrant is a measured defect rather
than a want — which is a stronger footing than `mpdf-005` Phase 9 ran on. Asked for
directly, the precedent `mpdf-006` OQ-3 and `mpdf-007` OQ-6 set.

Verdict: `NOT READY`, **two blocking**, six non-blocking. The round reproduced all seven
published numbers, built a working prototype, and confirmed the close-out census, the "no
golden moves" claim and that the numbers are look properties rather than probe artifacts —
so this phase does not repeat `mpdf-005` Phase 9's re-gridding mistake.

1. **[BLOCKING] Gate (2)'s discriminating claim was false.** It said a value raised without
   the structural change "still collapses to nothing and still measures negative"; the
   reviewer changed only the value — `0.4em` → `0.8em`, `linebreak()` and `weak: true`
   intact — and measured **+1.22** and **+2.93**, an implementation that passed every case.
2. **[BLOCKING] Gate (2) was keyed to an unnamed scratch document, and a realistic
   press-release headline made it unsatisfiable.** A wrapping title sets its own two lines
   **−2.90** apart under the `par(leading: 0.35em)` Phase 9 chose deliberately, so "every
   consecutive pair of boxes is positive" fails a correct fix and invites undoing that
   leading.

**Chasing the first blocker falsified the phase's whole mechanism, which is the sharpest
thing this episode produced and it came from the author's re-measurement rather than the
round.** §2 claimed the weak spacing was *discarded* at a paragraph boundary and prescribed a
structural rewrite. Varying only the value under the shipped shape gives author box top
**79.24 / 81.24 / 83.24** at `v(0pt)` / `v(2pt)` / `v(0.4em)` — linear, slope 1, no
threshold — and the prescribed structure measures **identically** at the same values. **The
rewrite was a no-op and the value was the whole bug.** The phase was rebuilt around that:
scope is now a value per join, `1.8em`/`0.9em` in the article and `above: 1.0em` on the
press release's author block, with the thresholds (6.78pt, 5.07pt) recorded.

Four of the six non-blocking were folded in the same pass. Two changed the design rather
than the prose: gate (3)'s byte-identity bound means the press-release spacing **cannot**
sit on the title block's `below:` — that value also governs a title-only document's gap to
its `divider` and moves the hash at `1.4em` — which resolved **OQ-12** by measurement; and
`core/tests/long_document_test.rs:the_fixtures_are_the_lengths_phase_5_measures_against`
(71 pages, a cross-reference on page 64 at fraction 0.620 ± 0.01, over a document carrying
all three keys) joined gate (4) as the tightest shipped assertion in the blast radius.
**One was accepted as a disclosure and rejected as a test**: nothing in the suite will pin
this fix, because a value is not a needle in this corpus, so gate (5) states the gap and the
close-out records it in `rules/pipeline.md`.

### Implementation note — Phase 9 — 2026-08-10 — the sweep held, and one stale claim was left standing

Not a review round. Phase 9 shipped as reviewed, with three things worth
recording.

The round-16 census held exactly. All thirteen shipped goldens changed on
their second line and on no other: `git diff` reports thirteen files,
thirteen insertions, thirteen deletions, and every hunk header reads
`@@ -2 +2 @@`. No import line moved and no `columns` value moved. The
whole suite passes, 95 tests over five targets.

One stale claim was left standing deliberately. `tests/fixtures/frontmatter.md`
says in its body that "the emitter only passes the three frontmatter keys
through to it", which is now five. Correcting it would change that
fixture's golden on a body line, and the gate's claim — every shipped
golden changes on exactly its second line — is a claim about this phase's
diff. The fixture's prose is filler that no reader is directed to, and
the close-out's documentation targets are the rule, the README and the
samples, all three corrected. A later phase that touches the fixture may
fix it in the same pass.

Two facts the plan had wrong, corrected during the build. The sample PDFs
are **not** checked in — `.gitignore` carries `/samples/*.pdf` with a
comment saying converting a sample writes its PDF beside it — so the
close-out committed no PDF. And the phase needed three new fixtures
rather than four: gate case (1)'s no-`template`-key half is already
served by the shipped `frontmatter.md`.

`rules/pipeline.md`'s cap rose from 255 to 280 in the same commit as the
prose that needed it — the third time this rule has taken that treatment,
after Phase 7 raised it from 205 and Phase 8 from 245. The growth is the
two new keys and their convention, the templates section becoming plural,
and the world binding every look rather than one. The file lands at
279/280.

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
