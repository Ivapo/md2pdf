# Review record — mpdf-010 (`specs/file_panel_spec.md`)

Append-only. One heading per round, newest first.

### Round 3 — Phase 1 only — 2026-08-28 — same reviewer, resumed with the author's changelog — **READY (converged, at the cap)**

The climb blocker confirmed resolved against the file and the code. The reviewer
re-derived the cost statement independently — opening `<root>/parts/ch1/text.md`
gives parent `parts/ch1`, grandparent `parts`, no `.md` there, no candidate, root
`parts/ch1` — and confirmed gate clause 1's new case is reproducible and free of
environment dependence: `<fixture>/parts` holds only `ch1/` and no markdown, so
the asserted `root = <fixture>/parts/ch1` follows from the rule. It also
re-checked the clause's other three cases against the real tree, finding that
`tests/fixtures/` holds only two files with paragraph-level markers —
`multi_file.md`, and `cross_references.md`'s `[](#fig:note)`, which is a fragment
and not an include — so nothing there names `panel/book.md`.

**This round sat on the three-round cap.** It converged rather than escalating,
and the record notes it because a fourth round was not available: had the climb
fix introduced a blocker, this would have gone to the human instead.

Three non-blocking findings, all folded in on convergence. **The sharpest was
that the climb decision's own heading still asserted the argument its body had
retracted** — it read *"the bound is `mpdf-008`'s own refusal"* over two
paragraphs explaining why that refusal entails nothing here. A
`(decision, recorded)` heading exists so a later pass does not relitigate, and
that one recorded the rejected version; it now reads *"one level is a cap rather
than a derived bound"*. §1.2's non-goal said "from Finder" where the cap applies
to `⌘O` equally. And the confinement rewrite of round 2 had made Phase 3's
"tested without a filesystem" claim false — canonicalizing a parent is a real
call — so Phase 3's scope was corrected in the same pass rather than left for its
own round to rediscover; the reviewer had flagged it forward rather than
re-opening a phase outside its scope.

### Round 2 — Phase 1 only — 2026-08-28 — same reviewer, resumed with the author's changelog — **NOT READY**

Five of six blockers confirmed resolved. **One new blocking finding, and it is
the best catch of the review: the one-level climb's stated bound was a
non-sequitur.** The author had argued the climb could not need a second level
because `mpdf-008` refuses an include inside an included section, so no master
can be a section of another master. That refusal is about a master naming a
*master*; the climb searches for the master of a *file*, which a deeper relative
path reaches with no nesting at all. Verified in the code both ways:
`core/src/emit.rs:portable_path` refuses only a scheme, a leading `/`, a `..`
segment and a backslash, and `core/src/sections.rs:Segment::directory` splits on
the last `/` — so `[](parts/ch1/text.md)` is a supported marker rather than an
unrefused one, and `parts/ch1/text.md` would have rooted at `parts/ch1`, below
its own master. That is verbatim the failure §2's opening paragraph says the
decision exists to prevent, surviving one level down.

**Resolved as an honest cap rather than a deeper bound**, which is one of the two
resolutions the reviewer offered. §2 now says one level is chosen rather than
derived, keeps the discarded non-sequitur in place so it is not re-derived,
argues the cap on cost — climbing further means reading markdown in `~/Documents`
and above to guess where the project is, and this app has never opened a file the
author did not name or a document did not name — and cites `mpdf-008` OQ-1's own
rejection of *"allow one further level and no more"* as a number nobody can
defend. The cost is stated with the reviewer's own example, and named as
recoverable in exactly one action and only that action: opening the master, since
the store is keyed by root and a wrong root cannot be corrected from inside the
panel. §1.2 gains the non-goal, **OQ-7** carries the question with three shapes
and their costs, and **gate clause 1 asserts the capped behaviour** so it reads as
a decision in the suite rather than as a defect nobody noticed.

Seven non-blocking, all accepted. Two are worth the record. **`declared.len() == 10`
does not move**: `Status` has ten fields, and this phase removes `sections` and
`master` while adding two — a coincidence now named in the scope so nobody
"fixes" the literal to silence a failure. And **`app/src/watch.rs:resolve` cannot
carry the write-confinement rule**: it is
`canonicalize().unwrap_or_else(|_| path.to_path_buf())`, so it returns its input
when canonicalization fails, and a file being created never canonicalizes —
`root.join("../escape.md")` would have survived a `starts_with` check textually.
A write now canonicalizes the parent, which exists, and joins the final
component.

### Round 1 — Phase 1 only — 2026-08-28 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode): yes.** Phase 1 produces the observable and the claim is
real rather than asserted: today a double-click on
`samples/showcase/sections/text.md` compiles that section standalone, and after
this phase discovery finds `showcase.md` and compiles the whole document — a
different PDF from the same gesture. It is also unusually well-evidenced for a
round 0, the want being recorded independently in `specs/reviews/mpdf-008.md`,
whose Phase 4 round noted the author had already said a file explorer was what
they actually wanted; this phase is the load-bearing half of that rather than the
decoration.

One generalist rather than a panel, matching `mpdf-008` Phase 4's round for a
single app-facing phase. The reviewer was pointed at five claims the author most
expected to be wrong and asked to verify each against the code.

**Six blocking findings, all accepted; no rejections in any round of this
episode.**

1. **The root could not be what the phase said it was.** `app/src/watch.rs:root`
   is `document.parent()`, so opening `sections/text.md` roots at `sections/`,
   discovery never sees the master above it, and the phase's headline observable
   and gate clause 2 were both unreachable. Resolved by §2's climb — which round
   2 then found under-argued, above.
2. **The gate's enumeration of `samples/showcase/` was wrong three ways**: five
   `.md` under `sections/`, not four; `README.md` omitted; and `.gitignore`
   carries `/samples/**/*.pdf` while the sample's own README instructs
   `md2pdf showcase.md`, so any developer who has run it holds a `showcase.pdf`
   the listing must include. An exact-enumeration gate over that directory is not
   reproducible by a second person. Resolved by moving the gate to
   `tests/fixtures/panel/`, a fixture the phase creates; the sample tree now
   appears only in §1's sketch, corrected to the real contents.
3. **The tree's shape was unspecified and the document disagreed with itself**
   about whether directories are rows — the sort rule presupposed directory
   entries, §1's sketch drew them, the gate enumerated none. Resolved by §2's
   flat `{ path, kind, missing }` entry, no directory entries, folders derived by
   the page, and a total byte-wise order.
4. **`classify`'s third answer had two incompatible definitions** across §2,
   Phase 1 and Phase 2, and the reviewer's sharpest point was that a section the
   master names is *already* in the asset list `app/src/document.rs:render_with`
   builds, so it classifies as `Asset` today and would recompile silently instead
   of running the divergence rule. Resolved by splitting it: `Tree` in Phase 1
   for the panel, `Edited` in Phase 2 tested *before* `Asset`.
5. **The store was written and never read.** The scope never said `Session::open`
   consults it, and no gate clause checked that an override changed which file
   opened; separately the several-masters case left `main` undefined, while OQ-5
   claimed the gate pinned behaviour it did not pin. Resolved: the store is read
   first, the multi-master case is decided and never leaves `main` unset, gate
   clause 3 pins all four cases, and OQ-5 is narrowed to the gesture.
6. **The symlink gate passed vacuously** — the link's target held nothing
   matching the filter, so the clause could not distinguish full confinement from
   none, and §2's confinement rule was scoped to writes while the listing stated
   no requirement at all. Resolved by putting the walk under the rule and
   pointing the link at a committed sibling holding `decoy.md` and `decoy.png`.

Eleven non-blocking, all accepted. Two were latent build failures rather than
polish: **`pub` alone would not have exposed `IMAGE_EXTENSIONS`**, because
`core/src/lib.rs` declares `mod emit;` privately and the crate has no `pub use`
at all; and **the store promotes `serde_json` from a dev-dependency to a real
one**, against an existing comment arguing it is deliberately dev-only — a move
whose one mitigating fact, that it is already in `Cargo.lock` by way of `tauri`
and so adds no crate to the tree, is now recorded beside it.
