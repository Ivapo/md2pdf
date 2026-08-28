# Review record — mpdf-010 (`specs/file_panel_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 2 only — 2026-08-28 — same reviewer, resumed with the author's changelog — **READY (converged)**

All five blockers confirmed resolved against the code rather than the changelog.
The reviewer re-derived the three numbers the fixes newly keyed the phase to and
found each right: `Status` carries ten fields today, so `edited` makes eleven;
`Changed` has three bools; and `notes-and-sources.md` is the last of the five
markers with headings on lines 1, 3 and 12. It also independently confirmed the
spelling claim from `core/src/sections.rs`, which sets `file: Some(marker.path)`
with `source_line: 1` — so `location.file` really is master-relative and
`location.line` is the line *within* the section, which is what makes gate
clause 2's two anchor sets disjoint (the master's first heading is line 11,
`mathematics.md`'s only heading is line 1).

**It sharpened blocker 3 on the way past.** The author's fix said `Edited` is
tested before `Asset`; the reviewer pointed out it must be tested before
`Document` too, or `main == edited` loses the divergence rule — and noted that
gate clause 5's third sentence already asserts exactly that case, so the gate
was right where the prose was loose.

Six non-blocking findings, all folded in on convergence. **The sharpest was the
author's own round-1 fix biting back**: "the switch re-arms both loops exactly
as `Session::open_at` does" borrowed too much of `open_at`, which assigns
`Preview { ..Preview::default() }` and zeroes `revision` and `reloaded` — while
`app/dist/index.html`'s `clear()`, which resets the counters the page compares
them against, runs on an Open and not on a row click. A switch built to that
sentence would have stranded `refresh` at its own guard and drawn nothing again.
The scope now says which half of `open_at` it borrows and lists what it leaves
alone. The others: `Session::set_main`'s refusal reporting through `divergence`
rather than its own `Err`, so one refusal does not arrive two ways; the one
field carrying two occasions at a time, which is a fact for `rules/desktop.md`;
the UTF-8 decode on main's read; the page's second row mark; and the window
gate ending with the discard, which both restores the tracked tree and exercises
the second way out.

### Round 1 — Phase 2 only — 2026-08-28 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode): yes.** Phase 2 delivers §1's headline sketch exactly —
the pane holds a section, the page shows the whole compiled document, the caret's
own page is right, `⌘S` writes the section — and that is a state this app has
never been in. It is also the right one: Phase 1 shipped a panel you can look at
and click once, which is the complaint §1 opens with, and
`specs/reviews/mpdf-008.md` independently records that a file explorer is what
the author said they wanted.

One generalist rather than a panel, matching Phase 1's round and `mpdf-008`
Phase 4's for a single app-facing phase. **The round's premise was that Phase 1
had shipped the same day**, so the reviewer was pointed hard at grounding: Phase
2 was written before Phase 1 was built, and its citations describe the
pre-Phase-1 code in places.

**Five blocking findings, all accepted; fourteen findings in total and no
rejections in either round.**

1. **The compile would have rendered the wrong text.** `Preview::compile` is
   `document::render(document::directory(&edited), &self.buffer)`, so with
   `edited != main` it renders a section's buffer as though it were the whole
   document — and the closure override the phase was built around is never
   reached, because the markdown never goes through the closure. The phase said
   only that the closure carries the override. Resolved by scope item 1: the
   markdown is main's text, the directory is main's, and **main's text is read
   through that same closure**, so it answers from the buffer exactly when the
   pane holds main. One rule instead of a branch.
2. **The switch would have disarmed both loops.** `Session::on_change` and
   `Session::recompile` guard on `preview.edited` against a path captured when
   the loops started, so setting `edited` and stopping there leaves the typing
   debounce compiling nothing and every filesystem event dropped — making gate
   clause 5 and the whole window gate unreachable. Resolved by scope item 4.
3. **`classify`'s arity and `on_change`'s meaning were both unstated.** The
   assets are main's and resolve against main's directory, so `classify` needs
   both paths; and `on_change` runs the divergence rule on `Change::Document`,
   where gate clause 5 requires main's external write to recompile *without* it.
   Resolved by scope item 3, which remaps `Document` to a bare recompile and
   gives `Edited` the rule.
4. **The refusal named a way out that does not exist.** `DIVERGED` opens *"this
   file changed on disk"*, false on this occasion, and §2's "Save, or discard"
   names a discard this app has never had — `main.rs` exposes ten commands and
   none of them drops the buffer. Resolved by giving the switch its own
   constant, **building the discard in this phase**, and widening
   `Preview::divergence` from a refused external change to a refused change.
5. **The window gate named an action the app does not perform.** `caretPage` is
   consulted only inside `refresh`, on a status carrying a new `revision`, so a
   caret move alone scrolls nothing; and `sections/mathematics.md` has exactly
   one heading, on line 1, so "moving the caret to its last heading" is where
   the caret already is. Resolved by moving to `notes-and-sources.md` — last of
   the five markers, three headings — and by saying the reader types a character.

Nine non-blocking, all accepted. Three are worth the record. **§2's own decision
text had gone stale in three places** by Phase 1 shipping: `classify` "answers
`Document` or `Asset`" (it answers `Tree` too), `classifier`/`on_change` "close
over the document alone today" (Phase 1 gave both the root), and
`Preview::save` "writes to `document`" (Phase 1 renamed the field). **The
close-out named text that no longer exists** — `rules/desktop-panes.md`'s "the
rows do not load" passage, which Phase 1's own close-out replaced; it now targets
Phase 1's live sentence. And **gate clause 2 was not checkable against what the
code returns**: `document::Anchor` is `{ line, page }`, and the `location.file`
the clause asserted on is dropped by the very filter under test, so the clause is
now keyed to anchor *lines*.


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
