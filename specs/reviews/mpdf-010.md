# Review record — mpdf-010 (`specs/file_panel_spec.md`)

Append-only. One heading per round, newest first.

### Phase 5 — a code review after shipping — 2026-08-28 — five findings, four fixed and one recorded

`/code-review` at `high` over `70860ba..HEAD`, run after the phase had shipped
and after the race fix above. **Five findings, all confirmed against the code
before acting on any of them**, four fixed here and one deliberately left.

1. **The re-mirror enumeration was still wrong, and this is the third time this
   file's rule has earned itself.** Round 1 caught two missing gestures and the
   phase added them; the list was still a list. `#files` is `flex: 0 0 auto` and
   `#lines` has no width of its own, so **both are as wide as their contents**,
   and `parts` rebuilds the panel on every status while `relines` rewrites the
   gutter on every keystroke — neither calls `placeViewer`. A project gaining a
   longer filename, or a document crossing 99 lines, moved the text pane's left
   edge with **no gesture at all** and stranded the figure off its column.
   Measured: the panel grew 136 → 306 px and the gutter 28 → 43 px, both
   through the real paths. Fixed by deleting the enumeration and observing the
   three boxes that decide the column — `#files`, `#lines`, `#text` — which is
   structural where a list of gestures is a guess, and which also makes the
   surface follow the divider *during* a drag rather than at its end. 0
   `ResizeObserver` loop errors over a 40-move drag and twelve toggles.
2. **The error path marked the compiled page stale**, which is verbatim the rule
   scope item 5 was written to obey and which the `.pdf` branch was routed
   around to satisfy. The catch called `fail`, and `fail` adds `stale`. Fixed by
   putting every sentence in the sheet instead: Rust's refusal placed as the
   status bar places a compile's, the two page-written lines as labels about a
   kind of file.
3. **`.svgz` drew a permanently blank sheet, and any corrupt figure did too.**
   It is in `md2pdf_core::IMAGE_EXTENSIONS`, so the panel lists one and the
   pipeline typesets it, but it is gzip and a blob URL carries no
   `Content-Encoding`. Fixed twice over: the bytes are gunzipped through
   `DecompressionStream` before the blob is minted, so a legal figure is
   actually viewable, and `img.onerror` now says so for everything that still
   will not decode.
4. **A symlink to a file inside the project is listed and then refused.**
   `document::descend` pushes the *entry's* name after resolving its target, so
   `cover.jpg -> figures/cover.jpg` is a row named `cover.jpg`; `asset_bytes`
   then wants `relative` to answer that spelling back and gets
   `figures/cover.jpg`. **Not fixed, and deliberately.** The rule is shared
   verbatim with `Session::set_main` and `Session::set_edited`, so it predates
   this phase for markdown rows and Phase 5 only makes it more visible; and the
   repair — comparing the canonical target against the canonical root instead of
   requiring the spelling to round-trip — is a change to a confinement rule,
   which wants a decision rather than a drive-by. It **over**-refuses, so it is
   a usability wart and not a hazard. It wants an OQ.
5. **The session lock was held across `std::fs::read`**, stalling `status`,
   `save`, the watch and the compile behind one reader looking at a picture.
   Fixed by dropping the guard after the root is cloned out, which is what
   cloning it out was for.

**Phase 5's eight window clauses pass unchanged after all four fixes**, and so
do the four race gestures from the entry above. That they pass is the point: not
one of the five findings was reachable by that gate, and three of the five were
about states it never observes — a column that moves with no gesture, a read
that refuses, a figure that will not decode.

### Phase 5 — a defect found after shipping — 2026-08-28 — the surface had no sequence

**Four gestures, one cause, none of them reached by the exit gate.** The read
crosses IPC and `showAsset` had no supersession guard, so every way out of the
surface was undone by the bytes arriving after it. Reproduced against the
shipped file in the Chromium harness with the read delayed 400 ms, then fixed
and re-reproduced as passing:

1. `Escape` before the bytes land put the figure **up**, not away.
2. Clicking a markdown row moved the pane and then drew the picture over the
   file it had just opened.
3. Two figures in flight left the bar naming one and the sheet holding the
   other — the label was written *before* the read, so it named the last row
   clicked while the sheet held the last one to arrive.
4. A figure in flight overwrote a `.pdf` row's sentence.

The fix is `viewSeq`, which is `renderSeq`'s own idea applied to a second
asynchronous pass — the page had the pattern at seven sites in the render path
and this pass did not use it. The label moved to after the guard.

**And the `Escape` guard was itself the defect, which the first fix did not
catch.** `if (e.key === 'Escape' && !viewer.hidden)` reads as tidy and is wrong
in the one case that matters: while the read is in flight the surface is *still
hidden*, so the key did nothing and `viewSeq` never moved. Case 1 went on
failing after `viewSeq` landed. `Escape` is now unconditional — it means *no
figure*, whether one is up or one is coming.

**Why the gate did not reach any of this**, and the note is for the next phase
rather than for this one: every clause waits for the figure to land before it
measures, so the whole class of *a reader who changes their mind mid-read* is
outside it. The eight clauses still pass unchanged after the fix. A window gate
that only ever observes settled states cannot see a race, and the harness — a
stub whose latency is a variable — is where this kind of claim is cheap.

Found by asking whether the phase wanted a code review, before any review had
been run.

### Phase 5 shipped — 2026-08-28 — the window gate, and three things the rounds did not measure

**The window gate passes, at eight clauses of eight**, on `cargo tauri dev` on
macOS 26.5.2: the figure is drawn over the text pane's own column (`viewer
153+360`, `text 153+360`), `main`, `edited` and `revision` are unchanged across
the click, the `.pdf` row says so without touching the stale mark or the error
bar, the surface follows the divider and the `Files` fold and the `Lines`
toggle, all three ways back work and a markdown row still moves the pane, and
`emit.svg` shrinks from 120 px to 96 × 58 in a 96 × 601 sheet with overflow 0.

**Its first run failed clause 6, and the app was right — the gate was measuring
the engine.** The clause guarded against passing vacuously with
`drawn.width < picture.naturalWidth`, and **WebKit reports an SVG's *rendered*
size for `naturalWidth`**, not its intrinsic one: 95 for a figure declared
120 × 72, against a drawn 96, while the containment claim underneath held
exactly. It now measures the same element at two pane widths and asserts it got
smaller, which a stylesheet with no fit rule fails and every engine agrees on.
The same shape as Phase 2's own first run: a clause written from the spec that
was true of one environment rather than of this phase.

**Round 2's `box-sizing` measurement reproduced, with the declaration dropped.**
The phase records that flexbox distributes free space over items' *outer* sizes
and that `box-sizing` is not what frees the sheet's top padding. Built with no
`box-sizing` on the sheet at all and measured in Chromium: the sheet is 797 px
inside an 853 px surface with a 56 px bar, `scrollHeight - clientHeight` is 0,
and a deliberately tall figure — 100 × 4000 — is capped at 728.75 px against a
729 px content box. So the reviewer's third-round arithmetic holds at
`content-box`, and the phase prescribes nothing it does not need.

**The height half of the fit rule has no window clause, and that is stated
rather than hidden.** Gate clause 6 is `emit.svg` in a narrow column, which
binds on width; the sheet is 601 px tall and nothing in either sample project
is tall enough to bind on height. The 4000 px case above is where that half was
checked, in the Chromium harness, and it is recorded here because the exit gate
does not reach it.

Two smaller notes, neither a departure. The five re-mirror occasions are an
enumeration in a file whose own rule is that it watches the pane rather than the
causes, and the exception is argued in place: an observer over `#text` never
fires for a fold, which moves its left edge without changing its size, and the
existing one over `#pages` does not fire while that pane is hidden. And the
prototype the phase was written from was **not** on `main` — it sat in a stash —
so this was a build rather than the reconciliation an in-place prototype makes
it, and the four ways the stash differs from the phase are each in the feat
commit's message.

### Round 3 — Phase 5 only — 2026-08-28 — same reviewer, resumed with the author's changelog — **READY (converged)**

A confirmation pass over three non-blocking fixes rather than a re-open, run
because one of them had been wrong twice. All three confirmed against the file,
nothing newly broken. The reviewer re-measured the geometry a third time —
`flex: 1` sheet 400 px in a 400 px column, overflow 0, figure 300 px, against
`height: 100%` at 500 px and overflow 100 under `content-box` — and confirmed the
scratch-root refusal genuinely fires, `strip_prefix` of the canonical root
against a canonical path one level above it failing.

### Round 2 — Phase 5 only — 2026-08-28 — same reviewer, resumed with the author's changelog — **READY**

All three blockers confirmed resolved against the code. The reviewer re-derived
the numbers the fixes newly keyed the phase to and found each right:
`generate_handler!` registers twelve commands and `pending_open` is the only one
that is not a wrapper over a plain function; `plan.pdf` slots between `other.md`
and `refs.bib` byte-wise (`o` 0x6F < `p` 0x70 < `r` 0x72); and `Status` gains no
field, so `declared.len() == 11` is untouched.

**Three non-blocking findings, all accepted, and the first is this loop biting
the author's own round-1 fix.** The `box-sizing: border-box` justification was
**measured false** rather than argued against: in a 400 px column with a 100 px
top padding, a `flex: 1` sheet is 400 px and overflows by 0 under *both* values
of `box-sizing`, because flexbox distributes free space over items' **outer**
sizes. The overflow-by-exactly-the-padding behaviour is real and belongs to
`height: 100%`, which is the shape the round-1 rewrite had moved off. Two drafts
had it wrong — the first justified the padding by `box-sizing`, the second by
`flex: 1` making the height definite, which is true and beside the point — so the
phase now rests on the outer-size rule, prescribes no declaration it does not
need, and carries the measurement so a later pass cannot re-derive the false
version.

The second: the author's own repair to round 1's `escape.png` finding would have
left an untracked file in **tracked** `tests/fixtures/` on every `cargo test`,
against `document::scratch_dir`'s own doc comment. The refusal case now builds
over a scratch root, which is one of the two repairs the reviewer offered. The
third: a quote of `fileRow`'s `opens` test had dropped its `!holding` term, and
that term is load-bearing — the markdown row the pane already holds is inert, so
clicking the row you are on while a figure is up does nothing. **Accepted rather
than fixed**, the alternative being a row whose drawing depends on page state
where `rules/desktop-panes.md`'s *"the rows hold no selection"* is what lets the
panel be rebuilt whole on every status; the cost is now stated in the phase
instead of discovered at a keyboard.

### Round 1 — Phase 5 only — 2026-08-28 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode): yes.** Phase 5 produces no observable and argues it
explicitly: `mpdf-008` made a document several files *and its figures several
files*, the panel has listed those figures since Phase 1, and checking that
`emit.svg` is the diagram you meant means leaving for Preview and losing the
pane — §1's opening complaint, about pictures rather than prose. It is also
*wanted* on the strongest evidence round 0 can have: both of OQ-1's candidate
shapes were built and tried in the running app, and the author chose this one.

One generalist rather than a panel, matching Phase 1's and Phase 2's rounds for
a single app-facing phase. **Three blocking findings, all accepted; ten findings
in total and no rejections in any round.**

1. **The Rust exit gate was keyed to a unit that cannot be tested.** The scope
   said `app/src/main.rs` gains a command and the gate said "the command
   refuses" — but that file has no test module, the crate is bin-only,
   `tauri::State` has a private field and no public constructor, and
   `ipc::Response`'s body is private. Every sibling phase named the testable
   seam and this one did not. Resolved by putting the read in
   `app/src/document.rs` as an ordinary function with the command a wrapper over
   it, which is what eleven of the twelve existing commands already are.
2. **The PDF sentence had no route, and one of the two available routes
   contradicted the phase's own invariant.** `app/dist/index.html:fail` runs
   `pages.classList.add('stale')`, so routing it there would mark the compiled
   page out of date for a click that compiled nothing — verbatim what scope item
   4 says this phase never does — and the `divergence` field draws the `Discard`
   button beside its sentence. Resolved by giving the sentence to the page as a
   label for a file kind rather than a status about the document, and by never
   calling the command for a `.pdf` at all.
3. **The PDF case had no reproducible gate clause.** `tests/fixtures/panel/`
   holds no `.pdf`, and the only PDF in reach — `samples/showcase/showcase.pdf` —
   is excluded by `.gitignore`, so a second person on a fresh clone would check
   nothing: verbatim the irreproducibility Phase 1's gate refused. The reviewer
   also caught that the obvious repair was silently costly. Resolved by adding
   `tests/fixtures/panel/plan.pdf` **and naming the cost in the phase**:
   `tests/fixtures/panel-manifest.txt` and
   `document::tests::the_listing_is_the_disk_and_what_the_master_names` each gain
   that row in this phase's own commit, an exact-enumeration gate failing when
   its fixture grows being that gate working.

Seven non-blocking, all accepted. The sharpest were that the re-mirror
enumeration dropped the panel fold and the `Lines` toggle, both of which move
`#text`'s left edge; and that the `../escape.png` refusal would have passed on
`is_file()` rather than on confinement, never running the rule under test.

### Phase 2 shipped — 2026-08-28 — the window gate, and one consequence the rounds did not enumerate

**The window gate passes, at ten clauses of ten**, on `cargo tauri dev` on macOS
26.5.2: the pane holds `sections/notes-and-sources.md` while `showcase.md`
compiles, the panel marks the two rows separately, typing under `## Citations`
opens the page that heading landed on **in the whole document**, the switch
refuses over the dirty buffer in a sentence that does not claim the file moved,
and the discard puts the tracked file back byte for byte.

**Its first run failed clause 6, and the app was right.** `showcase.md` is six
pages and that heading is on the sixth, so `applyAnchor`'s
`scrollTop = kids[5].offsetTop` clamps to `scrollHeight - clientHeight` and the
fifth page's tail stays at the top of a pane taller than one page. The clause
read *which page is at the top*, which would have passed on a short window and
failed on a tall one — a claim about the reader's monitor, which that file's own
header promises none of its clauses are. It now compares the scroll the app took
against the anchor's own page position, clamped the way the browser clamps it.
Worth the record because the gate was written from the spec and the spec's own
sentence — *"the page then opens on the page that heading landed on"* — is
ambiguous between the two readings at the end of a document.


Round 1's five blockers were framed as *"five things follow from that one
sentence… enumerated here rather than left to be rediscovered at a keyboard"*,
and building it found a sixth. **`Preview::export_path` named the file in the
pane**, so `Save a Copy…` would have offered `mathematics.pdf` for a PDF holding
the whole book. It is one line and a test, taken in the phase rather than left as
an open question, and it is recorded here so a later reader does not read the
phase's diff as wider than its review.

Two smaller notes, neither a departure. The master-relative spelling §Scope item
2 gives to `Preview` is computed in `document.rs` beside its inverse, because
that is where an ordinary test reaches it without building a `Preview`; the rule
and the direction are unchanged. And the anchor filter took a three-armed
`document::Pane` rather than the `Option<&str>` the prose implies: an `edited`
with no master-relative spelling and an `edited` that *is* the master are two
different answers, and an `Option` collapses them into one absence that would
hand the master's own line numbers to a buffer whose lines mean something else.

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
