# Review record — mpdf-011 (`specs/repository_split_spec.md`)

Append-only. One heading per round, newest first.

### Phase 3 shipped — 2026-09-02 — the crates are on the registry, and two greps were kept honest

All seven gates read, in the order the publish forces: everything cargo can check
locally, then the author's irreversible step, then everything that reads the registry.

**Gate 1, the one dry run.** `cargo publish --dry-run -p md2pdf-core -p md2pdf-cli` exits
**0**, and the transcript is the evidence for §2's dated correction rather than a
restatement of it: cargo packages the library, then *"Unpacking md2pdf-core v0.1.0
(registry `target/package/tmp-registry`)"* and compiles the tool against that. Two `-p`
runs "and then" each other could not have done it. `cargo package --list -p md2pdf-core`
lists **26 files** — `assets/fonts/` at seven faces and two licences, all three `.typ`,
and **`README.md`**, with the packaged manifest carrying `readme = "README.md"`, cargo
having rewritten `../README.md` on the way in exactly as the scope said it would.

**Two measurements the packaged manifests settled before the publish.** Neither carries a
`[profile]` section, so gate 2's refusal to compare the two binaries is a fact rather than
a precaution; and `md2pdf-cli`'s packaged `Cargo.lock` pins `icu_segmenter 2.2.0`, which
is the resolve `--locked` holds and the one an unlocked install would have moved.

**Gate 2, the claim the phase carries.** `cargo install --locked md2pdf-cli --root
<scratch>`, then that binary and the workspace's `target/release/md2pdf` over
`samples/showcase/showcase.md`, `-o` into two different scratch paths, both hashing
`e855b5119da5cc70e44b44ce48c0b3291c2781b9fc0525cff4a07113ad6b41b1`. The binaries differ by
15 MB — 49,590,448 bytes from the registry against 34,816,784 from the workspace, which is
`strip` and `lto` not reaching the registry — and the PDF does not differ by a byte.

**Gate 3.** Letur's `cargo test --workspace` reads 114 passed with one ignored and 12
passed with one ignored, the counts Phase 1 pinned. `cargo build --locked --manifest-path
web/Cargo.toml --target wasm32-unknown-unknown` exits **0**. Both lockfiles moved by
**two lines each** — `md2pdf-core` from `git+…?rev=` to `registry+…` with a checksum, no
other package's version touched — because they were updated through `cargo metadata` and
not `cargo generate-lockfile`, which would have thrown the seed away and re-resolved
Typst's transitive tree.

**Gate 4.** `bun harness/checks.mjs` in Chromium, with the registry-installed binary on
`PATH`: **15 clauses, 15 passed**.

**Gate 6.** `crates.io/crates/md2pdf-core` serves 45,937 bytes of rendered README carrying
the repository's own opening sentence and the `cargo install md2pdf-cli` line this phase
wrote into it.

**Gate 7.** `spec-lint .` by absolute path: **0 errors, 62 warnings in each repository**,
both unchanged from the reading taken before the phase began.

**Two author decisions the phase's scope had left open.** The keywords, which the scope
required and never named: `["markdown", "pdf", "typst", "typesetting", "converter"]` for
the library and the same with `"cli"` last for the tool. And the publish handoff — the
author ran both `cargo publish` commands, the session pausing between gate 1 and gate 2,
which is the scope's *"the publish itself is the author's step"* taken literally.

**One close-out artifact the spec's named list missed, and why.**
`.github/workflows/pages.yml`'s trigger comment carries the same *"the engine is a git
dependency of `web/Cargo.toml` now"* the close-out sends `rules/web-demo.md` to lose — and
`web-demo.md` declares that workflow among its own `sources`, so correcting the rule alone
leaves it contradicting the file it is generated from and `/sync-rules` re-seeds the stale
claim. That is Phase 2's close-out argument for `rules/pipeline.md` and its doc comment,
landing one repository over. **The close-out's own instrument could not have found it**,
for two reasons at once: `grep -rln '…' rules README.md` does not look outside those two
paths, and the comment wraps *"a git / dependency"* across two lines, so no single-line
alternative matches. Both files were corrected in the same commit; a sweep of `.github/`
for the same three terms now returns nothing. **The list being named rather than counted
is what made this a finding instead of a passing total** — the spec's own defence of that
choice, five times over, holds a sixth.

**Two rewordings that kept a gate literal rather than bending it.** `app/Cargo.toml`'s new
note on the git seam was written quoting `{ git = "…", rev = "…" }`, which put the string
`git = ` back into a manifest gate 3 greps and requires nothing from; the note now spells
the shape without the `=` and says why. And `rules/desktop-panes.md`'s *"it was `cargo run
-p md2pdf-cli` until that phase"* began reading as Phase 3 once the sentence before it
changed, so it names Phase 1, which is the phase that actually moved it.

**What is left open, deliberately.** OQ-3 — whether the engine keeps a page of its own —
and OQ-6, the unresolvable `by`, which is the methodology's rather than this repository's.
Neither is Phase 3's, and with this phase `mpdf-011` is done.


### Round 3, resumed — Phase 3 only — 2026-09-02 — the two rate-limited lenses, retried — **READY**

Verdict: `READY`, **zero blocking from all three lenses**. Phase 3's `reviewed` is set to
2026-09-02. The entry below this one stands as the record of the interrupted attempt; the
session limit had in fact already reset when it was written, and both agents reported on
the retry.

**Both lenses verified their own concerns rather than taking the changelog's word.** The
exit-gate lens ran gate 3's replacement itself — `cargo build --locked --manifest-path
web/Cargo.toml --target wasm32-unknown-unknown`, **exit 0 in 53 s**, compiling `typst`,
`md2pdf-core` and `md2pdf-web-spike` for `wasm32-unknown-unknown` — and recorded that it
discriminates in both directions the curl could not: it compiles the crate the phase
rewrites, and `--locked` fails outright if the manifest is swapped and the lockfile is not
regenerated, which is the omission the old clause could not see. The correctness lens
resolved the same tree with `cargo tree --locked` and answered the ordering question the
flag raises: the scope regenerates both lockfiles, so `--locked` **asserts the committed
pair agree** rather than silently repairing them, and run before that regeneration it fails
loudly, which is a correct failure rather than a trap.

**One round-1 number was corrected in the reviewer's favour of the author.** The
correctness lens's round-1 "~30 transitives" was a truncated tail; `cargo update --dry-run`
reports **48 `Updating` lines plus 4 `Adding`**, so the 48 written into §1.1's note and
gate 2 is exact rather than approximate.

**Four non-blocking findings, all folded before the date was set.** Two were stale pointers
left by this round's own renumbering — scope bullet 1 still sent the reader to "gate 5" for
the crates.io check, which had become gate 6; and gate 3's *"the only thing that compiles
the wasm crate"* was contradicted by its sibling, gate 5's `pages` run compiling it with
`wasm-pack`, both added in the same fold. The clause now says *"the only thing run
locally"* and names the other. The third was a **CommonMark lazy continuation**: the round-1
`CORRECTED` note on §2's *"needs"* had no blank line after it, so the `core/assets/`
paragraph was absorbed into the blockquote — the same defect repaired two paragraphs down
in round 2, made twice. A sweep of every blockquote in the document found one more of the
same shape in §1.2 and both are fixed; the sweep is the fourth.

**The verification instrument the close-out offers was itself measured, twice.** Round 3's
first pass found `grep -rln 'git = \|md2pdf-cli' rules README.md` returning three of the
four Letur artifacts, missing the one two rounds had been spent finding, because
`web-demo.md` states the dependency in prose. Both other lenses then re-ran the widened
three-alternative form and confirmed it returns exactly four, that `git dependency` is the
only alternative reaching `rules/web-demo.md:264`, and that `rules/INDEX.md` matches none —
all three claims the close-out makes about its own grep.

**What this episode cost, and the shape of it.** Six blockers in round 1, two in round 2
**both introduced by round 1's fold**, and four non-blocking in round 3 of which two were
introduced by round 2's. Every self-inflicted one was a consistency defect rather than a
design error — a stale pointer, a missing blank line, a list widened but not re-derived, a
gate borrowed from a phase where it had been sound. The one that mattered, gate 3's
constant, is the same species as the vacuous pass gate 2's `-o` closes, and it was written
*by the author into a gate whose neighbouring clause explains that exact trap*. That is
the argument for the same-agent resume stated as a measurement: the lens that had spent
round 1 on gate testability is the one that caught it.

### Round 3 — Phase 3 only — 2026-09-02 — three lenses resumed, **one completed** — **INCOMPLETE**

Verdict: **not recorded as converged.** The scope/consistency lens returned `READY` with
zero blocking. The correctness and exit-gate lenses were resumed at the same moment and
**both terminated on a session rate limit before reporting** (HTTP 429, resets 16:50
America/Lima) — a tooling failure mid-round, not a verdict. Phase 3's `reviewed` is
therefore **left null**, per §7.7's rule that the date records convergence.

**The round-2 blocker is resolved, and was verified by a lens other than the one that
raised it.** `rules/web-demo.md` is named in the Letur close-out, scoped to the one clause
— its *"the engine is a git dependency of `web/Cargo.toml` now"* — with the trigger
reasoning after it left standing, which a version bump does not falsify. The scope lens
also re-checked the two folds it had not raised: gate 3's replacement *"is sound and
non-vacuous … unlike the curl it fails when the build fails"*, and gate 5's workflow list
is complete, `letur/.github/workflows/typecheck.yml` triggering only on paths this phase
never touches while `pages` does fire, `web/Cargo.toml` and `web/Cargo.lock` sitting under
its `web/**` trigger.

**One non-blocking finding, folded: the close-out's own verification grep did not find the
file two rounds were spent finding.** `grep -rln 'git = \|md2pdf-cli' rules README.md`
returns three of the four artifacts — `web-demo.md` states the dependency in prose rather
than in a manifest line. Measured both ways; the alternation now carries `git dependency`
as a third term, and the close-out records that the third alternative is load-bearing.

**What is outstanding is confirmation, not repair.** The exit-gate lens's round-2 blocker
and the correctness lens's two round-2 non-blocking findings are all folded; neither agent
has re-read the fold. That is the convergence property §7.4 buys with the same-agent
resume, and it is the only thing this round did not deliver.

### Round 2 — Phase 3 only — 2026-09-02 — three lenses, all resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY` on **two blockers, both introduced by round 1's own fold** — which is
§7.3's *"a fix can introduce a blocker"* landing twice in one pass. Correctness returned
`READY`; scope and exit-gate each returned one blocker.

- **`letur/rules/web-demo.md:264` says the engine is a git dependency of `web/Cargo.toml`,
  and this phase falsifies it.** Round 1's fold had just widened the Letur close-out from
  one rule to three and still missed the fourth. Scoped to the one clause on the reviewer's
  own precision.
- **Gate 3's Pages check was a constant.** Round 1 substituted Phase 1's gate 7 —
  `curl … | grep -c 'data-example=' == 12` — into a place where it distinguishes nothing.
  Four measurements: `pages.yml` copies `web/index.html` verbatim, the page already carries
  twelve rows, the module is a separate runtime import at `web/index.html:760`, and
  `deploy` is `needs: build`, so a `wasm-pack build` that fails against the rewritten
  manifest **skips the deploy and leaves the previous deployment answering 12**. Phase 1's
  gate 7 was sound because nothing was served at that URL yet. Replaced with
  `cargo build --locked --manifest-path web/Cargo.toml --target wasm32-unknown-unknown`,
  and the gate now carries the reason the curl was withdrawn so it is not helpfully
  restored — the same service the counts note does one clause up.

Non-blocking, all folded: no gate clause read `.github/workflows/test.yml`, the very
workflow round 1 had just made a decision (**new gate 5** reads both repositories' run
conclusions); two surviving §2 sentences still prescribed the unlocked install; a round-1
`CORRECTED` blockquote had split the sentence it sat beside, leaving the body resuming
mid-sentence; `rules/INDEX.md` was missing from the engine half; and `cli/Cargo.toml`'s new
field had no literal, now `version = "0.1.0"`.

**The two `--locked` corrections record a distinction rather than adding a flag.** The
plain `cargo install md2pdf-cli` is right as a *consumer's* install story and stays in §1's
usage example and the engine README; `--locked` is what anything *comparing two builds*
needs — §2's two sentences, gate 2, and the harness's four sites.

**One count was written and removed before the round closed.** The author's first draft of
the Letur close-out read *"Five Letur artifacts, not three"* — a total over a tree this
phase edits, which is the instrument this document's own record broke five times across
Phase 2. It was replaced with the named list and a grep, before the reviewers saw it;
round 3 then found the grep itself incomplete.

### Round 1 — Phase 3 only — 2026-09-02 — three fresh lenses (correctness/grounding, exit-gate testability, scope/cross-file consistency) — **NOT READY**

Verdict: `NOT READY` from all three, on **six blockers after deduplication**.

**Round 0, asked once for this episode and answered by the author.** Phase 3 produces no
observable, and that is argued rather than assumed: §1.1 makes the observable the *gate* of
every phase in this spec rather than its product, and Phase 3's own line says the gate is
that the registry's binary writes the same bytes the workspace's does. It is the right
thing to build — §1's goal is the shape a finished library has, *"a crate on the
registry"*, and without this phase the split stops half-done with Letur pinned to a git
revision.

The blockers, and how each resolved:

1. **`cargo publish --dry-run -p md2pdf-cli` cannot pass before `md2pdf-core` is
   published** — measured independently by two lenses and by the author:
   *`no matching package named md2pdf-core found — location searched: crates.io index`*,
   because packaging rewrites the path dependency into a registry one. So §2's *"the dry
   run is the gate"* held for the library and not for the tool, and the phase's only
   pre-publish clause covered one of the two crates it ships. **One invocation naming
   both** — `cargo publish --dry-run -p md2pdf-core -p md2pdf-cli` — packages both and
   verifies the tool against the locally packaged library; measured clean on cargo 1.97.1.
2. **`cargo install md2pdf-cli` carried no `--locked`**, so a gate keyed to byte-identity
   was reading a fresh resolve. The published crate *does* ship a `Cargo.lock`, and cargo
   ignores it without the flag; `cargo update --dry-run` reports **48 semver-compatible
   updates** over this lock today, `icu_segmenter 2.2.0 → 2.3.0` among them — Typst's own
   text segmentation, where line breaking is decided. The bytes happen to match today
   (measured both ways, `e855b511…`), which is why this was a latent gate and not a broken
   one.
3. **The CI decision was restated, not made.** *"whether that wants a workflow of its own
   is this phase's call"* let two implementers, one adding a workflow and one not, both
   pass every clause. **Decided by the human**: `.github/workflows/test.yml` running
   `cargo test --workspace` on push and pull request.
4. **OQ-5 was cited for a literal it had never chosen.** **RESOLVED by the human** to
   `md2pdf-core = "0.1"`, which is what §1's usage example and Letur's own manifest comment
   already promised.
5. **OQ-7 was open immediately before an irreversible publish**, and one of its three
   answers changes the published surface. **RESOLVED by the human** to its first answer, on
   a measurement taken this round: the export is not unread, it is read next door —
   `letur/app/tests/page_examples_test.rs:42` imports `md2pdf_core::{Asset, md_to_html,
   md_to_pdf}` and calls `md_to_html` at lines 309 and 656. The third answer was refused on
   the same fact: removing the `pub fn` would break that suite.
6. **The close-out omitted artifacts the scope's own edits falsify**, two of them in Letur.
   Split into an engine half and a Letur half; the install line enumerated as **four sites
   in three files**, `serve.mjs`'s header comment and its `INSTALL` constant counted
   separately.

**Numbers re-derived this round, and each held**: `md2pdf-core` and `md2pdf-cli` are both
free on crates.io while `md2pdf` is the 2022 `0.0.3` tectonic wrapper §1.2 describes;
`cargo package --list -p md2pdf-core` lists the eight files under `assets/fonts/` and
exactly the three `.typ`, plus `README.md` once `readme = "../README.md"` is set — cargo
copies it in and rewrites the field; the packaged manifests carry **no `[profile]`
section**, so the workspace's `strip`/`lto`/`codegen-units` do not travel and the two
binaries are necessarily different objects; `rules/pipeline.md` sits at 1238 of 1240.

**Three shipped sentences took dated `CORRECTED` notes** rather than rewrites, per §6.1:
§1.1's *"`cargo update --workspace --dry-run` moves nothing"*, which reports `Locking 0
packages` vacuously because it restricts the update to the two members; §1.2's *"No secret
is needed for anything in this spec"*, false for a phase whose central act needs a
crates.io token; and §2's *"three things the registry needs"*, where only `description` and
`license` are mandatory and the `version` beside the path dependency is the one genuinely
required.

**One rejection, recorded.** The exit-gate lens asked for a `keywords` clause in gate 1.
Refused: cargo validates no part of it, and a missing keyword is the one thing this phase
touches that a later version fixes cheaply — a gate literal with no failure mode worth
catching is maintenance for nothing. Gate 1 now carries that reasoning in place. The same
lens's complaint that *"clean"* was undefined against warnings **was** accepted; gate 1 now
says "exits zero".

### Rounds 4 and 5 — Phase 2 only — 2026-09-02 — three lenses, resumed — **READY**

Verdict: `READY`, zero blocking from all three lenses. **Converged at round five, two
rounds past §7.6's cap, on the human's explicit decision** — which is how that rule says
the cap is passed, and this is the record of it. Phase 2's `reviewed` is set to
2026-09-02.

**The escalation and what the human decided.** Round 3 escalated with one blocker and two
candidate fixes from the reviewers — (a) word the corrected doc comment so it names no
path, keeping the count at nine, or (b) let it name the path and set the count to ten.
The author was given both plus a third the reviewers had not proposed: **replace the count
with a property**, which was the only option that also closed the correctness lens's
second instance, `core/tests/examples_test.rs` being a file the phase *creates* and whose
right module doc would breach any closed list. **The human chose the third**, and
authorised the rounds that verified it.

**Gate 4 is now four checks and no number**: no path-form mention of the removed page test
survives under `core/` or `cli/`; every `app/`/`web/` match is a comment and never code;
the four doc comments that deliberately keep an `app/` path are still present, **named by
symbol rather than counted**; and `core/tests/long_document_test.rs` names neither the
page test nor `web/index.html`. The third is a floor against over-deletion with no ceiling
a new comment could breach. The fourth was added in round 5 from a non-blocking finding
and **negative-tested**: reverting the first correction alone fails check 4 and passes
checks 1, 2 and 3, so it is the only clause that catches a skipped correction.

**Round 4 found the defect had moved one clause over, into gate 5 — moved there by gate
4's own repair.** Two lenses measured it independently: gate 5 asserted "0 errors and 60
warnings, being 43 / 6 / 9 / 2", and the folds fixing gate 4 pushed the total to 63 by
adding three unresolvable citations, **two of them inside gate 4's own new text**. Gate 5
is now a property on the same principle: `0 errors`, and every warning one of four kinds,
the fourth widened to a bare `<file>.rs:<symbol>` whose file left with the split. The
totals and the per-spec census are gone, and so is the scope's five-spec census, which had
stood two bullets above a sentence claiming no count was asserted anywhere in the phase.
**The cause was removed as well as the symptom**: gate 4's check 1 names the page test's
refusal case in prose rather than citing it, so the clause no longer adds the very kind of
citation it exists to explain.

**Why this is recorded as one seam and not eight slips.** Five separate numbers in this
phase were wrong across five rounds, and every one was a count over something the phase
itself edits. Gate 4's moved four times, twice by its own repairs. Gate 5's moved three
times, once *because* gate 4 was fixed — and once more in round 5, 63 → 62, when the
self-citation was removed, **with nothing breaking, because no clause asserts a total any
more.** That last movement is the property demonstrated rather than argued.

**The distinction that survives, in the exit-gate lens's words**, because it is what a
later reader needs in order not to helpfully restore a number: gate 4's broken counts were
totals over *lines the phase rewrites*, so the phase's own prose moved them; gate 1's
242 is a total over `#[test]` attributes, which no doc-comment edit can see. "The
denominator is what decides whether a phase can falsify its own gate."

**Three numbers were deliberately kept and each was tested this round.** Gate 2's nine
`ok` and three `error` rows, counted off a `web/index.html` this phase reads and then
deletes without editing. Gate 1's 255 today and 242 without the page test's thirteen —
the one that *is* a count over a file the phase edits, verified by measuring
`long_document_test.rs` at two `#[test]` before and after and the post-phase tree at
exactly 242. And check 3's four-symbol floor, which is a floor rather than a total.

**One blocker was the author's own and is recorded as such.** Round 4's check 3 named
`core/src/sections.rs:segment`; the `app/src/watch.rs:classify` note is in
`Sources::resolve`'s doc comment, `segment`'s own beginning twelve lines later with no
`app/` path. All three lenses converged on the correction, and `Sources::resolve` is the
corpus's own spelling in `rules/pipeline.md`. `spec-lint` could not have caught it — both
symbols exist, so the citation resolved either way.

Non-blocking, folded in round 5: gate 4's preamble read "Three checks" over four bullets,
which is the same species as everything else this review found.

Two non-blocking notes left open rather than folded: gate 6's "re-checked after a wait"
names no interval, and "the draft's" in gate 2 has no antecedent in the document now that
the round-1 `CORRECTED` block is gone, this record being the only place that version
exists.

### Round 3 — Phase 2 only — 2026-09-02 — three lenses, all resumed — **NOT READY — ESCALATED AT THE CAP**

Verdict: `NOT READY` from all three, on **one** blocker, which all three found
independently and which is the third consecutive round broken by the same seam.
**§7.6's cap is reached, so this escalates to the human and nothing is set** — Phase 2's
`reviewed` stays `null`. Going past the cap is a decision a person makes, and this entry
is where it is recorded.

**The blocker.** Gate 4 asserts `grep -rn "app/\|web/" core cli` returns **nine**, and
its justification is that the two doc comments the phase corrects "each match on
`web/index.html` and on nothing else, so a correction that leaves the token has not
corrected anything". The first half is true and all three lenses verified it. The
conclusion silently assumes the *replacement* text introduces no token of the other kind,
and the grep is `app/` **or** `web/`. Round 2's fold changed the prescribed `md_to_html`
correction from "name `tests/fixtures/examples/`" — which carried no token, and is the
wording the 11 → 10 → 9 arithmetic was measured against — to "the reader left with the
page, and it is now Letur's `app/tests/page_examples_test.rs`", which carries an `app/`
one. Measured on a rebuilt tree with both corrections applied exactly as the scope words
them: **ten**, with `core/src/lib.rs` ×4. The corrected line does not leave the
population; it moves from the `web/` column to the `app/` column.

The correctness lens found a second instance of the same closed-world assumption:
**`core/tests/examples_test.rs` is a file this phase creates, and gate 4's list of six
files does not include it.** §2's whole argument for the extraction is provenance, so a
module doc naming `web/index.html` is the right comment to write there — and it breaks a
closed count of nine or ten either way.

**Why this is recorded as a seam rather than three arithmetic slips.** Round 1 found the
count wrong by seven (`app/`-only measured into an `app/`-or-`web/` clause). Round 2 found
the corrected count wrong by two (the phase's own edits removing counted tokens). Round 3
finds it wrong by one (a replacement introducing a token). Each round fixed the number and
the next edit broke it. **The instrument is a count over a tree the same phase edits**,
and gate 1 already catches the thing gate 4 exists for — code in `core`/`cli` depending on
a removed tree would not compile. That is the observation the escalation carries.

**Everything else in Phase 2 survived re-derivation, by all three lenses independently.**
Gate 5 re-measured **after both corrections**: 0 errors, 60 warnings, split 43 `app/` /
6 `web/` / 9 page-test / 2 bare, matching the four kinds the clause names. Gate 1's 255
today and 242 without the page test's thirteen, exact. Gate 2's three sentences verbatim
from the page. Gate 3's `-o` and its `press-release.typ` / `math.typ` justification.
Gate 6's premise — `https://ivapo.github.io/md2pdf/` answers 200 today, so it checks a
real state change — with no conflict against §1.2, whose non-goal is signing and
distribution and whose "no secret is needed" still holds, nor against OQ-3. The extraction,
the asset provenance with `samples/pipeline.svg` at 510 against the page's 509, the
`ok/`–`error/` split, and the close-out all hold. `rules/pipeline.md` is 1237 of 1240;
`cli/src/main.rs:default_output` exists.

**Round 2's non-blocking fixes verified resolved:** the `CORRECTED` block is gone and no
reference to it dangles, the only remaining `CORRECTED` mentions being §1.1's and Phase
1's legitimate §6.1 use; §3 reads 1–7 in order; gate 6 has its scope bullet; gate 5 names
four kinds; gate 1 pins rather than records. The review record's own form was checked
against the corpus and matches, append-only with Phase 1's rounds untouched.

Two non-blocking notes left open: gate 6's "re-checked after a wait" names no interval, so
a verifier has no threshold at which a persistent 200 becomes a failure; and "the draft's"
in gates 2 and 5 now has no antecedent in the document, this record being the only place
that version exists.

### Round 2 — Phase 2 only — 2026-09-02 — three lenses, all resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`. **All three lenses independently found the same thing: the round-1
fold broke the gate it was fixing.** This is §7.3's "a fix can introduce a blocker" case,
and it is recorded in full because the loop generated it on itself.

**The one blocker, from three directions.** Round 1's fold ordered two doc comments
corrected — `core/src/lib.rs:md_to_html`'s and `core/tests/long_document_test.rs`'s
module block — and gate 4 counts the lines both sit on. Each matches
`grep "app/\|web/"` on `web/index.html` and on nothing else, so correcting either
removes it from the count. The exit-gate lens measured the arithmetic step by step: **11**
with Phase 2's removals alone, **10** once the `md_to_html` comment is corrected, **9**
once the module block is. Gate 4's "eleven, and only these" therefore describes a tree
the phase will not produce: an implementer who performs the corrections fails the gate,
and one who passes it has not performed them. The gate's own hedge — line numbers are an
aid, the file tally is the assertion — does not rescue it, because the per-file tally is
exactly what moves. **Gate 4 now reads nine**, with the corrected tally and the reason it
is nine rather than eleven.

**Two more the correctness lens found inside the same fold.** First, the fold told the
implementer to correct `md_to_html`'s doc comment to name `tests/fixtures/examples/` as
its reader — while adding OQ-7 in the same pass, which records that
`core/tests/examples_test.rs` never calls `md_to_html`. That swaps one false decision
statement for another, and the close-out would have propagated it into `rules/`, the one
artifact that must track the code. The honest correction is that **the reader left with
the page**: it is Letur's `app/tests/page_examples_test.rs` now, and this repository has
none, which is what OQ-7 holds. Second, repointing the module block's precedent clause at
`examples_test.rs` asserts a shape that file does not have — the clause appeals to an
`#[ignore]`d generator and a compiled-in copy, and `bless_the_generated_blocks` went to
Letur in Phase 1. **The precedent clause is deleted rather than repointed**, which also
settles the 10-vs-9 ambiguity the exit-gate lens named: the bullet had not said whether
`web/index.html` survived.

**A non-blocking finding accepted whole, because it was right about the method.** The
consistency lens argued that the fold's dated `CORRECTED` block did not match §6.1 or
either corpus precedent: §6.1 reserves that instrument for **shipped prose kept in place
and now misleading**, where Phase 2 is unshipped and its scope and gate were rewritten
outright — nothing was kept, so a reader could not see what was corrected. It was a
changelog wearing a correction's clothes, and §7.7 puts the round narrative here instead.
**The block is deleted and this entry is where it went.**

Other non-blocking findings folded: gate 5's leading sentence named three warning kinds
against a four-kind population (the two bare `preview.rs:<symbol>` warnings are a fourth);
gate 1 *recorded* a `#[test]` count without comparing it to anything, and now pins **255
today → 242 plus `examples_test.rs`'s own**, the difference being the page test's
thirteen; gate 3's "three bundled looks" was loose, `core/assets/math.typ` being a
preamble; gate 6 lived only in the gate where Phase 1 had put its settings step in the
scope as well, and now has a scope bullet and a re-check after a wait, Pages' CDN serving
a cached build for a while after a disable; and OQ-7 was listed between OQ-4 and OQ-5,
making §3 read 1, 2, 3, 4, 7, 5, 6 — it is now in sequence.

**Numbers re-measured this round**, so a later round can trust them: the post-removal grep
is 11, and 9 after the two corrections; `core/tests/` and `cli/tests/` carry 255 `#[test]`
today and 242 without the page test; `spec-lint` on a simulated post-phase tree reports
0 errors and 60 warnings, being 43 `app/`, 6 `web/`, 9 page-test and 2 bare
`preview.rs:`; `samples/pipeline.svg` is 510 bytes against the page's 509;
`rules/pipeline.md` is 1237 body lines against `max_lines: 1240`; and
`curl -sI https://ivapo.github.io/md2pdf/` answers **200**, so gate 6 checks a real state
change. `https://ivapo.github.io/letur/` answers 200 with twelve `data-example` rows,
which is Phase 1's gate 7 still holding.

One finding was left to Phase 3's own round rather than folded: Phase 3's scope now
carries the CI hand-off — the engine keeps no workflow once both leave with Letur — with
no exit-gate clause, so it can pass silently. Named here so that round meets it.

### Round 1 — Phase 2 only — 2026-09-02 — three fresh lenses (correctness/grounding, exit-gate testability, cross-file consistency/scope) — **NOT READY**

Verdict: `NOT READY` from all three. **Round 0, asked once for this episode and answered
by the author:** Phase 2 produces no observable and says so, and §1.1 argues the omission
rather than assuming it — in a path-by-path split the observable is the thing *at risk*,
so it is the gate of every phase rather than its product, with precedent in
`rules/pipeline.md`'s byte-identical measurements and `mpdf-007` Phase 5. It is the right
thing to build now: Phase 1 shipped the same day, both repositories hold `app/` and
`web/`, and that interim silently loses an edit made in the wrong one.

Deduped to four blockers, all folded.

**1. Gate 4's grep returns eleven, not four.** All three lenses, independently. §2 measured
`grep -rn 'app/' core cli` — `app/` alone — and got four doc comments; gate 4 greps `app/`
*or* `web/`, which adds seven `web/` mentions the scope never enumerated. A correct
implementation failed the gate as written, and an implementer chasing "only four" is
pushed to delete true doc comments.

**2. Gate 5's warning population is wrong by eleven of sixty.** All three lenses, each
having built a simulated post-phase tree. Nine warnings cite
`core/tests/page_examples_test.rs:<symbol>` from specs that stay, **four of them in this
spec itself**, and two cite bare `preview.rs:<symbol>` that resolve today through the
suffix branch and stop once `app/` leaves. Since four sit in an accepted, append-only
document, **the gate had to be reworded rather than the corpus repaired** — which is the
call the exit-gate lens argued and the author took.

**3. The page-test extraction is not mechanical.** Correctness and exit-gate lenses. The
scope created "the twelve rows as files" and said the new test compiles them "against the
two assets" — but the two assets are the page's inline `data-asset` elements, and the
phase deletes the page without extracting them. Neither has a substitute in the engine:
`tests/fixtures/refs.yml` is keyed `"DBLP:books/lib/Knuth86a"` and its own header says a
fixture keyed `knuth1986` would "prove nothing", so the `citation` row fails outright;
and `samples/pipeline.svg` differs from the page's by one trailing newline, which is the
*dangerous* case because it compiles and the gate would accept a fixture set that is not
the page's. How a file "carries its `data-expect`" was also unstated, and the natural
in-file marker is actively wrong — two rows open with `---`, and a leading HTML comment is
itself a `raw HTML block` refusal. Resolved by copying both assets from the parent commit
byte for byte, and by carrying `data-expect` in the directory name.

**4. The scope contradicted itself over `core/tests/long_document_test.rs`'s module doc.**
Correctness lens. One bullet said it "moves with it"; another said the `app/`-naming doc
comments "stay as they are" — and that `//!` block holds **both** the `app/dist/index.html`
mention the second bullet keeps and the page-test precedent the first would move. Nothing
can move with the page test in any case, the file staying with the engine.

**OQ-4 was a fifth blocker to one lens and non-blocking to another**, the second arguing an
implementer could proceed by elimination since the phase deletes both workflows and adds
none. Put to the author and **resolved: the old URL is let go.** The consistency lens found
the state neither of OQ-4's two options covered — Pages staying enabled on `Ivapo/md2pdf`
and serving the last build of a deleted `web/` — which became gate clause 6.

Non-blocking findings folded: gate 2's verb ("twelve rows" over-counting nine `ok` plus
three `error`) and its unreproducible "copying the needles", the three sentences now
quoted verbatim since both of their sources die in the same commit; gate 1's
baseline-free `git diff --stat`, which is empty whatever the commit did; gate 3's missing
`-o`, which would dirty the tree; gate 5's `spec-lint` invocation, the tool being on no
`PATH`; the stale `by: mpdf-010` scope sentence, which contradicted the gate and §2's rule
2; the `app/`-path census, five specs rather than two; `.gitignore`'s `/.playwright-mcp/`
and `node_modules/`, whose comments name things that leave; `Cargo.lock`; the phase
header's "two hashed fixtures" against a gate naming three documents; and README's
`## Try it` claim.

**Deferred rather than fixed:** `md_to_html` loses its only test one phase before it is
published — **new OQ-7**, pointed at Phase 3 with three candidate answers, because the
dropped comparison is *about the page* and the page is Letur's. And the engine keeps no CI
once both workflows leave, written into Phase 3's scope.

**Rejected: none.** Every finding from all three lenses was accepted, folded or deferred.

### Round 3 — Phase 1 only — 2026-09-02 — the correctness lens, resumed — **READY**

Verdict: `READY`, zero blocking. **Converged at round three, at the cap.** Phase 1's
`reviewed` is set to 2026-09-02 and `status` moves `draft` → `accepted`.

**Both round-2 blockers verified resolved against the file, and against the extraction
itself.** The reviewer re-ran gate 5 verbatim on the real filtered repository it had
built in round 2: thirty-one commits under `--follow`, the last line the creating commit
with the subject and date the gate now names, and its hash `109b9fd` rather than the
engine's — which is what the gate now says it must be. `grep` for the old hash over the
spec returns nothing. Rule 2's three sites match `_foreign`'s only call sites, and Phase
2's gate 5 is consistent with it.

**Rule 1 checked as asked and found sufficient**: `id_prefix` has exactly two readers,
and `id_pattern` builds its regex from `id_min_digits` alone, so no third reader is
missed. The dependency the rule states is real — left comparing a string against a list,
`_foreign` returns true for every target and rule 2's scoped demotion becomes a blanket
one. The reviewer's round-2 patch implementing all three rules linted the simulated Letur
tree at **0 errors, 63 warnings**, every one inside gate 6's three named kinds, with all
61 unresolved paths under `core/` or `cli/`.

Two non-blocking notes, both folded: gate 5's `--format=%ad` gains `--date=short`, which
is what prints the literal the gate names; and rule 1 now says equality over a string and
membership over a list, *never* a substring test, which would accept `mpd-001` under
`mpdf`.

**What this episode's record is for, and one thing it is not.** Rounds 2 and 3 ran on the
correctness lens alone: the other three were terminated mid-round by an account rate
limit and never returned a round-2 verdict. The convergence is therefore narrower than
round 1's four-lens sweep, and it is recorded as such rather than implied to be broader.
The lens that did run is the one that had caught the deepest round-1 defect, and in round
2 it caught two more by *implementing* the phase's prerequisite and running it, which is
what turned a plausible design into a measured one.

### Round 2 — Phase 1 only — 2026-09-02 — the correctness lens, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, two new blockers, both introduced by round 1's own fixes.
**All eight round-1 blockers verified resolved against the files**, the first of them
empirically: the reviewer patched a copy of `bin/spec-lint` with §2's three prerequisite
rules exactly as written, built the Letur tree the phase describes, and linted it at 0
errors and 63 warnings.

**Blocker 9: gate 5 was keyed to `c926c4e`, a hash `git filter-repo` necessarily
destroys.** Rewriting parents and trees rewrites every commit id, so a second person
running the gate as written marks a correct extraction failed. Everything else the gate
asserted held on the real extraction — 31 commits, the subject, the date, the rename
followed, the `panel/outside` symlink travelling. Folded: §2 states the general rule that
no commit keeps its id, and gate 5 reads a subject and a date with an explicit clause
that the hash is not the engine's and must not be.

**Blocker 10: rule 2's implementation sentence was false against the code.** The headline
was right; the sentence after it said `_foreign` is reached at four sites including a
phase's `by`, and it is reached at three — `extends`, `supersedes`, `superseded_by` — a
`by` never being resolved against the id map at all. Proved: a tree holding
`multi_file_documents_spec.md` alone, whose Phase 4 carries `by: mpdf-010` with
`mpdf-010` absent, lints clean. So there was nothing to demote, and covering `by` would
mean inventing a check rather than widening one. Folded: rule 2 names three sites and
states the gap; §2's error-source bullet stops claiming the engine meets a mirror image;
**Phase 2's gate 5 stops expecting a `by` warning**; and the asymmetry becomes OQ-6, a
design call for the methodology rather than for this repository.

One non-blocking finding folded: §2 said four gate scripts open the showcase and a fifth
the article, where three open it by name and a fourth opens `samples/article.md`.

**A defect the author found between rounds, inside the fix for round-1 blocker 1**, and
recorded because it is the pattern this loop generates on itself: rule 1 said only that
`id_prefix` "accepts a list", and both of its readers compare with `!=` against a string,
so a list would have made every id malformed and every edge foreign — disabling rule 2 at
the site rule 2 depends on. Rule 1 now names both readers and the semantics.

### Round 1 — Phase 1 only — 2026-09-02 — four fresh lenses (correctness/grounding, exit-gate testability, cross-file consistency, scope/YAGNI) — **NOT READY**

Verdict: `NOT READY` from all four. **Round 0, asked once for this episode: yes.** The
phase produces no PDF and says so in its own header; §1.1 argues that the observable is
the gate rather than the product, hashed either side of the move on the corpus's own
cross-tree precedent, and round 1 confirmed the identity is sound to key a gate to
(`PdfOptions::default()`, no date set anywhere, `today()` returning `None`, a lock that
`cargo update --dry-run` does not move). It is the right thing to build because the
dialect is the project by the author's own word, and the one version number shared by
three crates plus a separate Tauri version is a defect no change inside one workspace
can fix.

**Three of the four lenses built the Letur tree the phase describes and linted it**,
which is what turned the draft's tool claims into measurements: 15 errors and 48
warnings, identical with and without a sibling checkout.

**Eight blockers, deduplicated across the lenses; every one accepted and none
rejected.**

1. **Gate 6 could not pass, and §2's cross-repository design was wrong for the tool**
   (all four). `source_roots` is a filter over the repository's own file list, so the
   sibling root healed nothing; a cited path that does not exist resolves by basename,
   so `cli/src/main.rs:read_assets` landed on `app/src/main.rs` as a symbol-absent error
   thirteen times; `mpdf-010`'s `supersedes: [{id: mpdf-008}]` was an error because the
   tool excuses a missing target only under a different prefix; and `rules/web-demo.md`
   names `core/tests/page_examples_test.rs` as a source, an error by existence. Folded:
   §2's decision rewritten around three `spec-lint` rules named as a prerequisite of the
   phase — a prefix list, an absent edge target as a warning, no basename fallback for a
   path with a directory in it — the sibling-root idea withdrawn, the rule's source
   repointed in scope, and gate 6 restated to zero errors with its warnings named by
   kind. OQ-1 resolved by the same rule.
2. **The warning population was not what gate 6 said** (exit-gate, consistency, scope):
   `rules/desktop-geometry.md` emits `RULE_SOURCES_WITHOUT_GENERATED` today and moves to
   Letur. Folded into gate 6's three named kinds.
3. **The harness compiles through `cargo run -p md2pdf-cli`, which Letur's workspace
   will not contain** (correctness). Folded: a new §2 decision — `serve.mjs` runs a
   `md2pdf` on `PATH`, installed from the engine by git until Phase 3 publishes, the
   prerequisite stated with the harness's others; a compile binary inside Letur weighed
   and refused, since the app has no library target.
4. **Gate 3 named three broken copies where `checks.mjs:OWNS` holds twelve** (all four).
   Folded: twelve, every one `ISOLATED`.
5. **The `samples/` copies had no stated layout and were missing `pipeline.svg`** (all
   four). `app/src/document.rs:a_master_in_a_subdirectory_is_not_this_roots_master` reads
   the samples *directory* and asserts no master at its top level, so a flat copy beside
   `multi_file.md` fails it. Folded: `tests/fixtures/samples/` mirroring the four
   entries; the copy list re-derived by the grep instrument and written with it —
   eleven fixtures, `images.md` and `refs.bib` dropped as read by nothing.
6. **Gate 5 did not discriminate a filtered history from a copied tree** (exit-gate).
   Folded: keyed to 31 commits under `--follow` and the creating commit `c926c4e`,
   2026-08-10, which is `mpdf-003` Phase 2, not Phase 1 as §2 had said.
7. **Gates 1 and 4 collided on where the hashing test lives** (exit-gate, correctness,
   consistency, scope). Folded: `#[ignore]`d in `app/src/document.rs`'s test module,
   reading assets through `read_assets_with` and writing under `temp_dir()`; gate 1 pins
   the per-file counts with that one added, 0/46/61/8.
8. **Gate 7 hinged on an unstated author step and contradicted the scope's "private or
   public"** (exit-gate, consistency, scope). Folded: the repository is public, the
   author enables Pages with the workflow as source, `web/Cargo.toml` and
   `web/Cargo.lock` join the scope, and the gate reads a `curl … | grep -c` of 12.

**Non-blocking, all folded.** The page test needs no path repointed and no new
dev-dependency, said so; the `--path` list expanded per entry with a `--path-rename`
for the page test; Letur's root manifest keeps `[workspace.package]` for `edition` and
`license` and the release profile; `app/build.rs` enforces nothing and the version
assertion is dropped for Tauri's own fallback, verified in `tauri-utils`' source ("if
removed the version number from `Cargo.toml` is used"); "four places" corrected to
three in `core/` and one in `cli/`; `rules/web-demo.md` declares four sources not three;
the `CORRECTED` note placed beside `mpdf-006` §1's *"Give the dialect a front door that
shows itself"* rather than the frontmatter `note:`; `mpdf-003` cites `md_to_pdf`, not
`collect`; `build.rs` and `tsconfig.json` added to the table; the frozen copy taken from
the split commit and the engine binary built there on a clean tree; the phase touches
the engine for this spec's `shipped` date; four gate scripts open the showcase and one
the article; OQ-2 reclassified as defaulted so the phase is not blocked on input; gate
1 written as numbers; the push carries the code and the corpus as separate commits; no
secret is needed anywhere, noted in §1.2. Phase 2's text corrected where §2's rewrite
reached it (the `by: mpdf-010` edge, the basename trap, the doc-comment count).

**Rejected: none.** One design of the author's own withdrawn on the panel's
measurement, the sibling source root, recorded in §2 as withdrawn rather than deferred.

Numbers the panel re-derived: 114 tests in `app/src/*.rs` (0/45/61/8); `EXPECTED` 12 and
two assets; twelve `data-example` rows, nine `ok` and three `error`; ten gate scripts
(4/1/5); twenty-three sources across the four desktop rules, ten in `pipeline.md`, four
in `web-demo.md`; twelve `OWNS` mutations; 31 commits under `--follow` on
`app/src/preview.rs`; 2.5 MB under `core/assets/`; the engine lints today at 0 errors
and 1 warning; the simulated Letur tree at 15 and 48.
