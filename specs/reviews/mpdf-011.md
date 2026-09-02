# Review record — mpdf-011 (`specs/repository_split_spec.md`)

Append-only. One heading per round, newest first.

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
