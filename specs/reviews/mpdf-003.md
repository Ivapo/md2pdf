# Review record — mpdf-003 (`specs/desktop_app_spec.md`)

Append-only. One heading per round, newest first.

### Round 5 — Phase 14 only — 2026-08-29 — the same reviewer, resumed — **READY (converged)**

Zero blocking. The round-4 blocker is resolved and **the reviewer preferred the author's
route to its own**, saying so: it had proposed keeping the mutation and stating the
ordering precondition, where the author removed the precondition instead — *"stating an
ordering precondition leaves a precondition to be honoured, and removing it leaves nothing
to honour."*

Two facts were established that the phase had assumed rather than checked, and both are
stronger than it needed. **`documentElement.removeAttribute` has exactly one call site in
all 3,968 lines of `app/dist/index.html`** — the `system` branch of `wearAppearance` — so
the patch can touch nothing else and its counter increments exactly once per `system` step
in every walk order; the ordering hole is gone rather than narrowed. And **the page carries
no bare `[data-theme]` presence selector at all**, every one being value-qualified, which
is the fact the palette-identity argument needs and had been leaning on implicitly.
Isolation to D1 holds on two independent arguments, and D3's teardown routes through
neither.

One non-blocking finding, accepted, and it is the sharpest kind — a hazard the *fix*
created. **`"system"` was the one value that collides with D1's own vocabulary**: D1
asserted "`data-theme` is the value or absent for `system`", and a permissive spelling of
that sentence reads `data-theme="system"` as satisfying "the value", under which the
mutation passes. The do-nothing form could not collide this way, leaving `"light"` or
`"dark"` behind. Resolved twice over: the mutation now writes the sentinel `"mutated"`,
which no appearance is named by, and **D1 is respelled strictly** — absent for `system`,
equal to the appearance otherwise.

**The episode's own tally, recorded because it is the useful part.** Five blocking findings
across five rounds; **three were introduced by the author's own fixes to earlier ones**,
each surfacing exactly one round later — the pattern §3 of the loop names. The most
valuable finding was non-blocking and deleted a third of the phase: a reviewer asking
whether the hard part was *necessary* rather than whether it was *correct*.

### Round 4 — Phase 14 only — 2026-08-29 — the same reviewer, resumed — **NOT READY**

Run past §7.6's three-round cap **on the human's explicit decision**, recorded as one: the
cap had been reached on a round that verified nothing, the fault being the author's tooling
rather than any disagreement, so the alternative was clearing a phase on a blocker no round
had confirmed resolved.

All five round-3 edits confirmed present. The mechanism question the previous round could
not reach is answered: the DOM-interception route **is** buildable without further
invention — `themeButton` is a live node reached by `getElementById`, `wearAppearance`
assigns `textContent` by property lookup at call time, so an own accessor shadows
`Node.prototype`'s and takes — and the invocation-counter rule **is** precise enough to
judge a run by.

**One new blocker, and it was the author's third fix-induced one.** The mutation had
changed form in round 3 — from "`wearAppearance` always sets `data-theme`" to
"`removeAttribute` patched to ignore it" — and the rationale beside it was carried over
verbatim. It argued from `data-theme="system"`, which the new form can never write: the
page's only `setAttribute` for that attribute is on the *non*-`system` branch, so ignoring
the removal leaves whatever was already there. **The consequence was worse than wrong
prose — the mutation could pass.** D1 walks all three appearances in no stated order; on a
launch from a stored `system`, which is the default and what clause 3's own restore
guarantees between runs, the attribute is already absent, the patched call still fires so
the counter rule does not catch it, D1's assertion holds, and clause 4 fails for a reason
the phase treats as impossible. Resolved by making the patch **write a value** rather than
do nothing.

One non-blocking, accepted: *"the same shape Phase 13 already introduced there"* has no
precedent — that phase's change is a block inside the `setup` closure, not a break in the
builder chain. Recorded as a correction rather than quietly deleted. The correction itself
then failed `spec-lint` for citing `main.rs:78`, a `file:line` citation this corpus forbids
(§2.5).

### Round 3 — Phase 14 only — 2026-08-29 — the same reviewer, resumed — **NOT READY (cap reached, escalated)**

**The round reviewed a file that did not carry the fix, and the fault was the author's
tooling rather than the reviewer's reading.** The round-2 changelog described five edits;
only one had landed. The author's replacement script is all-or-nothing — one pattern in a
batch failed to match, it exited before writing, and the author fixed that one pattern and
re-ran it alone without re-applying the other four. The round caught this decisively and
cheaply, by diffing against `18b4df7`: 139/85 at round 2 and 143/85 now, `+4/-0`, so
nothing that was to be replaced had been. **A changelog is the author's claim about the
fix; the fix is what shipped**, and this is the round that earned that rule.

So round 3 verified nothing about the content and the round-2 blocker stands formally
unresolved. **Per §7.6 the loop caps here and escalates**, and the escalation is recorded
as what it is: not a disagreement between author and reviewer, but a round spent on a
mistake.

The reviewer's advisory, given on the fix *as described* rather than as written, is folded
in: the DOM-interception mechanism is buildable — `document.documentElement` and
`getElementById('theme')` are global-scope-reachable instances that `wearAppearance`
reaches by property lookup at call time, so an own property shadows the prototype and
takes — with two details the summary had left out. **`third-mark` must delegate to the
original setter and actually write the glyph**, because D2 measures with
`getBoundingClientRect` and an accessor that lies about what it wrote changes no rendered
width and would falsify nothing. And **the counter wants a stated home and lifetime**: a
page global read back through `execute/sync`, reset when the mutation is installed, one
mutation live per run. Both are now in the phase, and all five round-3 edits were verified
present by grep before this entry was written.

### Round 2 — Phase 14 only — 2026-08-29 — the same reviewer, resumed — **NOT READY**

All three round-1 blockers resolved and verified in the files; **one new blocker, and it
was created by the author's own fix for the second** — the pattern §3 of the loop warns
about, arriving exactly one round later.

The round re-derived the isolation the author claimed rather than accepting it, and in
both directions: `attribute-always-set` fails D1 and leaves D2 standing **because
`data-theme="system"` is palette-identical to the attribute being absent** — it matches
neither attribute selector and does match the media query's `:not([data-theme='light'])` —
so the glyphs, their widths and `#brand`'s rect do not move; and `third-mark` fails D2
alone because `title`/`aria-label` come from `APPEARANCE_SAYS` on a separate line from the
glyph. That argument is now written into the phase, the author not having stated it as
precisely.

**The new blocker: "the page's own function replaced through `execute/sync`" is impossible
against this page.** `app/dist/index.html` has exactly one `<script type="module">`,
`wearAppearance` is a declaration inside it, and `window['__pane']` is the only binding the
page publishes in four thousand lines; `execute/sync` evaluates a classic script in the
global context and cannot rebind a module-local name. Resolved by naming the mechanism
instead of leaving it to an implementer — a mutation patches a DOM method or accessor on
one of the two objects that function writes through, those being reachable *because* they
are the DOM and not the module.

Four non-blocking, all accepted. **The sharpest was cut rather than reworded**: exit gate
clause 2's middle leg, "a `--release` build succeeds with the crate absent", is equally
true under `[target.'cfg(debug_assertions)'.dependencies]` — the very form the clause
exists to rule out — so a leg that passes on the thing being ruled out is not a check, and
counting it made the clause read stronger than it was. Also folded: both rules' `sources:`
must gain `app/driver/drive.mjs` or `/sync-rules` regenerates the claims away, per §8.1 and
Phase 12's own precedent; "two files, four lines" understates a `main.rs` edit that has to
break the builder chain; and D3's self-perpetuating loop must run last and tear itself down
or it sits under every later measurement.

### Round 1 — Phase 14 only — 2026-08-29 — fresh reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase):** the phase produces no observable and says
so in its first line, arguing it on Phase 9's and Phase 12's grounds, with clause 1 pinning
that no test moves. It is the right thing to build on measured rather than asserted
evidence: Phase 13's window gate reported five failures across three runs and **four were
the instrument**, each costing a manual window run. **One caveat recorded rather than waved
past:** it is infrastructure appended in the same session that motivated it, and the sample
is one gate. The episode proceeded.

The reviewer re-derived every number and nearly all held: 338/0/2 across nine binaries by
running the suite, ten `ok(` sites in `checks.mjs`, six keys in both `OWNS` and
`MUTATIONS`, 3,721 lines across the eight gates all carrying the same five-part ledger, and
each of the phase's claims about the toolchain — that `tauri-build` globs
`capabilities/**/*`, that a permission naming an uncompiled plugin fails the build, that
`[target.'cfg(debug_assertions)'.dependencies]` gates nothing and cargo says so, that
`setWindowRect` is offered where the page is refused `set_size`.

Verdict: `NOT READY` — three blocking, twelve non-blocking. The author accepted all
fifteen, rejected none, deferred none.

Blocker 1: **a mutation that owned no clause the phase converts.** Both converted clauses
drive through `invoke('set_appearance')` and never press `#theme`, so "the click rewired to
place instead of invoke" would have failed nothing. Resolved by re-scoping D1 off the mark
so D2 owns it, and by two new mutations that each have an owner.

Blocker 2: **how a broken page is produced was unspecified, and the obvious mechanism does
not exist here.** `generate_context!` compiles `frontendDist` into the binary, so there is
no served copy to mutate as `serve.mjs` builds one. Resolved by injecting through the
session — and that fix is what round 2 then found impossible as first written.

Blocker 3: **"keeps those two clauses and loses the other two" did not resolve against a
gate with four.** One reading deleted the error counter from a run that stays manual.
Resolved with a table stating the fate of each of the four, and a note that clause 1's
surviving manual half needs a boolean of its own, today's being entirely the converted half.

**The most valuable finding was non-blocking and removed a third of the phase.** The
capability machinery — a `build.rs` generating a `driven.json`, plus a `.gitignore` line —
rested on a build error that is real, but the round observed that the plugin declares
`COMMANDS: &[]` and `permissions = []` and therefore exposes no IPC at all. The author
verified it by building: a `--features driven` binary with **no capability entry anywhere**
serves `/status`, creates a session, runs `execute/sync` and moves the window through
`window/rect`. The generator, the second capability file and the `.gitignore` line were all
cut. **A reviewer asking whether the hard part was *necessary* rather than whether it was
*correct*.**

Also caught: the close-out's cap literal was 611 where `spec-lint` says **620/635**, which
reversed the paragraph's own reasoning about whether the cap must move; the `/status`
quote was a fragment of its envelope and `ready` is `false` until a webview exists;
`setWindowRect` was the phase's headline claim and no clause exercised it; and the claim
that exit-gate clause 4 held the live-value-read discipline was backwards — falsification
catches a mutation that *passes*, where that defect made correct code *fail*. The phase now
says that class is held by nothing.

Rejections: none.

### Round 3 — Phase 13 only — 2026-08-29 — the same reviewer, resumed — **READY (converged)**

Scoped to confirming round 2's four folds, and it confirmed them against the files. **The
arithmetic was re-derived rather than accepted**: 334 + 4 = 338, the baseline itself having
been re-run in round 1 as 90+0+24+24+180+1+3+12+0 = 334 passed, 0 failed, 2 ignored across
nine binaries. **And the fourth test was checked for whether it bites rather than merely
sits nearby** — the round compiled a mimic of the two-place composition and ran the clause
as worded against three mis-wirings: the override omitted, the override taken from a
constant, and `Preview::status()` guessing a value instead of filling `System`. All three
fail it; the correct composition passes. Both halves of the wording are load-bearing.

No contradiction from the folds, including the one the author flagged: the `Session`
paragraph and the `main.rs` paragraph describe one call chain from two sides and each
defers to the other, with no double write, no double announce, and no third place claiming
the announce.

One limit was stated plainly rather than raised as a finding, and it is now in the phase:
**the fourth test reaches the composition and not the call site.** Whether
`app/src/main.rs:status` was actually moved onto `Session::status` is outside every test in
this repository, by the division `main.rs` records for itself — the same reason
`document::move_to_trash` is documented as the one function no test here calls. Clause 5's
window pass is where that line gets eyes.

### Round 2 — Phase 13 only — 2026-08-29 — the same reviewer, resumed — **READY**

Both blockers resolved, verified in the files rather than from the changelog, and **both
fixes were checked by construction rather than by reading**. For blocker 1 the round
compiled a standalone mimic — a non-`Default` `Status`, a `MutexGuard`-returning
`preview()`, and `Session::status()` as `Status { appearance: self.appearance,
..self.preview().status() }` — and confirmed functional record update needs only a base
expression of the same type, that the guard temporary lives to the end of the statement,
and that reading `self.appearance` beside `self.preview()` is two shared borrows. It also
established that `app/src/main.rs:456` is the **only** `.status()` call outside
`preview.rs`'s test module, so the author's "some thirty-five call sites, nearly all tests"
is exact, and that `Session::new` has exactly two call sites, so `settings: PathBuf` is a
two-line change.

For blocker 2 the round reproduced the author's measurements to the hundredth and then
tested isolation directly: the new group-beside-the-brand clause still reads 8 at 900px
under `flex-min` (which moves `#brand.left` to 334.69/335.28 at 240px, Phase 12 clause 3's
ground and not this one's), is untouched by `footer-last` and by `cell-main` measured both
with and without the 58-character name, and the new mutation leaves footer height at 24 and
`#brand` inside the bar at 900/500/240 in both engines — so it collides neither upward nor
downward.

Four non-blocking, all accepted. **The sharpest is one round 1's own fix created**, which is
the pattern §3 of the loop warns about arriving a round later: moving the appearance onto
`Session` made `Preview::status()` deliberately report `System` with only `Session::status()`
correcting it, and **nothing in the gate covered that seam** — the three named tests
exercise the store, the harness stubs Rust entirely, and clause 5 would still have read as
passing, `set_theme` flipping the native appearance directly so the title bar and the
palette follow while only the footer's mark stalls. A fourth test closes it and clause 1
moves 337 → 338. Also folded: clause 5 still carried the "before the window is shown"
wording the scope two paragraphs above now corrects; the `main.rs` paragraph still assigned
the write and the move to the command, whose loose reading is a command that writes the file
itself and never announces; and the new clause's exact equality depends on `#controls`
holding one flush, unpadded child, which is now named in the clause.

### Round 1 — Phase 13 only — 2026-08-29 — fresh reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase):** the phase produces no observable and says
so in its first line, arguing it — no line it adds reaches the compile path, and the gate
pins `--paper` unchanged in all three states so the byte-identity is checkable rather than
asserted. It is the right thing to build on the record's own evidence: the author named the
toggle as the next footer cell in the working note, Phase 11 named it again, and this
document's own round 0 for Phase 1 already accepted that its value is convenience rather
than capability. **One caveat recorded rather than waved past:** §1.1's parking of theming
was re-affirmed on 2026-08-28 and is narrowed on 2026-08-29, which is fast; the narrowing is
only honest because the gate pins `--paper`. The episode proceeded.

The reviewer re-derived every number the phase is keyed to and all but one class held: the
334/0/2-across-nine-binaries baseline by running the suite, the harness's seven clauses and
three mutations by running it, the rule caps 475/475, 596/605 and 145/150 against
`spec-lint`'s own body count, the eleven `@property` lines, and `Window::set_theme`'s
existence with `Theme = Light | Dark` and `set_ns_theme(None)` following the system. It also
confirmed empirically in both engines that the four-block CSS gives the right tokens **and**
the right `color-scheme` in all six system×state combinations with `--paper` unmoved.

Verdict: `NOT READY` — two blocking findings, eight non-blocking. The author accepted all
ten, rejected none, deferred none.

Blocker 1: **the value's stated home is wiped on every Open.** The phase held the appearance
on `Preview`, but `Session::open_at` does `*preview = Preview { root, main, edited, tree,
..Preview::default() }`, and both `Session::open` and `Session::set_main` route through it —
so a global preference there returns to `System` on the next `⌘O`, after which the footer
glyph flips, the cycle position is lost and `settings.json` disagrees with the running
window. **No clause of the gate opened a document after setting the appearance**, so nothing
would have caught it. Resolved by holding it on `Session` beside `store`, with a two-place
`Status` composition chosen over changing `Preview::status()`'s signature because that has
~35 call sites; the same move resolved two non-blocking integration gaps, the settings path
and the private `on_render`.

Blocker 2: **gate clause 3's third mutation fails no clause.** "`margin-left: auto` moved
back onto `#brand`" was keyed to `#brand`'s rect not moving — but an auto margin absorbs
exactly the free space in total, so a last child with no right margin cannot move:
`#brand.left` reads 862.98 at 900px and 202.98 at 240px under **both** layouts, in Chromium
and WebKit alike. Resolved by re-measuring rather than re-reasoning, and **the re-measurement
found a second trap**: the property that does separate them — the group's distance to the
brand against the footer's own computed `column-gap` — separates them at 900px (8 against
536.3 Chromium / 535.7 WebKit) and **not at 240px, where both read 8**, the 58-character name
having filled `#edited` and left no free space to absorb. The width is now part of the clause
rather than of the prose around it, which is the same correction round 2 of Phase 12 made.

Non-blocking, all accepted: Tauri builds the config's windows *before* the `setup` hook runs,
so "read before the window is shown" was wrong and the no-flash property comes from one
`Ready` callback, with `"visible": false` plus `show()` named as the fallback; `serve.mjs`
was missing from the scope's file list though it holds the mutations; the typedef test's
`declared.len() == 11` literal must move to 12 and gate clause 1 read as forbidding it;
`--paper` dates from `mpdf-009` Phase 5 (`9755682`) and not this spec's Phase 1, in two
places including inside the dated §1.1 note; the cycle order and glyph mapping were decided
inside a check rather than in the scope; the a11y deferral was routed to `mpdf-009` OQ-3,
which asks what a *canvas* costs a screen reader rather than what a `contentinfo` landmark
does, and the glyph question had no owner at all — **OQ-11 and OQ-12** now carry them; and
`Session::store`'s doc comment stops being true.

Rejections: none.

### Round 3 — Phase 12 only — 2026-08-29 — the same reviewer, resumed — **READY (converged)**

The blocker was confirmed resolved **by measurement rather than by reading**: serving the
real `app/dist/index.html` at 240px with the 58-character name, Chromium reproduced the
phase's literals to the digit — `#brand` at x=202.98 with both declarations, with
`min-width: 0` alone and with `overflow: hidden` alone, and at **317.45, right edge
342.47, outside the bar** with both dropped.

**And the round checked that the mutation isolates**, which is what clause 3 actually
demands. Under it the three boxes still sum to 600 = `innerHeight`, the footer is still
24px, `main.nextElementSibling === footer` and `footer.nextElementSibling === script`
both still hold, `documentElement.scrollWidth` is unchanged at 395, and nothing touches
`#status`, `#parts` or the cell's text — so it fails the brand check and no other. The
other two mutations isolate by construction: a `<script>` is `display: none`, so moving
the footer past it changes no geometry, and rewiring `report`'s cell line touches only
`editedCell.textContent`. The sweep's new 240px floor was also checked on the
*unmutated* page: the header has grown to 66px there and the sum is still exact.

Three non-blocking, all accepted. **The sharpest is a trap in clause 4**:
`cli/src/main.rs:default_output` writes beside its input and `.gitignore` covers only
`/samples/**/*.pdf`, so a harness compiling the fixture without `-o` leaves an untracked
PDF in `tests/fixtures/` and fails clause 4's own "`git status` is clean". The scope now
requires `-o` into the scratch directory. Also folded: `--doc` takes a directory or a
file, which is `mpdf-010`'s one level of climb rather than looseness; and a 58-character
`edited` is not among the fixture's eleven entries, so the panel marks no edited row
during the sweep — recorded as the reason the sweep and the row-click stay two checks.

### Round 2 — Phase 12 only — 2026-08-29 — the same reviewer, resumed — **NOT READY**

Blockers 2 and 3 resolved and verified in the files. **Blocker 1's fix had introduced a
new blocker, which is the pattern §3 of the loop warns about and the best catch of the
episode.** Clause 3's second mutation was rewritten to drop both declarations, but the
conditions that make it bite — a 240px viewport and a long name — were quoted only in
the justifying paragraph. The scope pins the stub to `tests/fixtures/panel`, whose
longest bare name is `missing.md` at ten characters, and the check being falsified named
no width floor: an implementer writing the sweep at Phase 11's 900/620/500 over the
fixture's own names lands back at round 1, with a mutation that fails nothing. Both
conditions are now stated inside the check itself.

The round re-measured what it queried, against the real served page, and confirmed the
three corrections made outside the phase — the `CORRECTED` note on Phase 11, the CSS
comment and the rule — including that `flex: none` on `#brand` really was not shown to
matter. `cargo test --workspace` still 334/0/2 with the corrections in the tree.

Five non-blocking, all accepted, and two were the author's own errors: the measured
string is **58** characters and not 57, and `317.45` does not re-derive without the exact
name, so both the phase and the `CORRECTED` note now quote it. Also: clause 5's headline
read as though a completed run always passed, where its own dispositions say otherwise;
`serve.mjs` had a revision axis but no document axis, which clause 5 needs since it runs
on `long.md`; "changes no line of the page" contradicted a close-out that edits a CSS
comment; and `md2pdf-cli` is the crate where `md2pdf` is the binary.

### Round 1 — Phase 12 only — 2026-08-29 — fresh reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase):** the phase produces no observable and
says so in its first line, arguing it — no Rust, no executable line of the page, PDF
byte-identical, clause 1 the check. Whether it is the right thing to build is the live
question and was put to the reviewer rather than assumed: OQ-10's justification for the
wide half was falsified in the same session, so the phase rests on a smaller claim and
must earn a node manifest in a Cargo workspace on that alone.

Three blocking, nine non-blocking. Nothing was rejected in any round.

**Blocker 1 is the round's best catch, and it falsified a claim `mpdf-003` Phase 11 had
already shipped.** Clause 3's mutation was to drop `min-width: 0` from `#edited` and
watch the brand leave the bar — but `overflow: hidden` in the same rule already zeroes
the flex item's automatic minimum size, so the mutation does nothing and the clause was
unsatisfiable. Re-measuring sharpened it: **the two declarations each supply that
minimum independently**, so they are redundant with each other and dropping *either*
alone changes nothing; only dropping both pushes the brand off the bar. Phase 11's
"`min-width: 0` on the cell and `flex: none` on the brand are load-bearing" was
therefore wrong in three places at once — the spec, the page's CSS comment and
`rules/desktop-panes.md` — and all three were corrected, the spec keeping its original
text under a dated `CORRECTED` note per §6.1.

**Blocker 2: gate clause 5 had no failure condition.** Both its outcomes were declared
"a result worth having either way", so nothing about it could stop the phase, and the
branch that mattered stated no disposition. It was a measurement task appended to a
gate. It now passes only on a completed, recorded run, says outright that completing it
is necessary and not sufficient, and gives three exhaustive outcomes — one of which
halts the phase and forces a re-scope, which per §7 clears `reviewed`.

**Blocker 3: clause 5's method was not reproducible from the phase.** It named two
commits and a gate file; every parameter that makes the A/B mean anything — dpr 2, a
520px pane, the 71-page document, both engines, `long()` — lived only in OQ-10's note,
which records a run rather than a procedure. Nor did the scope supply the machinery:
`serve.mjs` served the working tree only. The method is now stated in full and
`serve.mjs` gained `--rev`.

Non-blocking, all accepted: seven gates and not six; the stub's return data and fixture
were unspecified though several checks key to them; the stub's injection point
interacts with the element-order check and had to be `<head>`; "`report` composes
nothing" is not directly assertable and was reworded; the close-out edited two rules
without extending their `sources`, which §8.1 makes unregenerable; the
Chromium-by-continuity argument was decorative, since no literal may be encoded and both
engines must pass; the Phase 11 anecdote was loose — the gate was committed with the
defect and corrected *before* the shipped marker, so it did not ship broken; and the
lockfile and the scratch directory's placement were unstated, the latter mattering
because `generate_context!` walks `frontendDist` into the shipped binary.

**The round's most useful non-blocking finding is not a defect but a judgement**: with
the A/B gone, the harness as scoped has **no automated consumer** — CI unwired, the
seven gates unmigrated, no manual clause replaced. It was accepted rather than argued
away, and the phase now says so outright, naming a person as the consumer and stating
that if that is not worth a node manifest the phase is the thing to cut rather than
shrink.

### Round 3 — Phase 11 only — 2026-08-29 — the same reviewer, resumed — **READY (converged)**

Blocker 3's gate half confirmed resolved by construction rather than by reading: the
reviewer built the footer in the position the scope mandates and evaluated all three
of clause 4's conjuncts as an implementer would write them —
`main.nextElementSibling === footer`, `footer.nextElementSibling === script`, and
`body.lastElementChild !== footer` — all true simultaneously, so the clause is
satisfiable by and only by that position. `<script` has exactly one hit in the file,
so "the module `<script>`" has a unique referent.

**Clause 5's added sentence was measured rather than accepted.** At 620px in the empty
state the header is 46.5px and does not grow; with the fixture open and `#status`
worded it is 66px. So a tester who armed before opening really would read the positive
control as satisfied by nothing — the false-negative-as-pass the sentence names — and
it is load-bearing rather than decorative.

The four non-blocking rewrites all spot-checked: 47/66/81 against rects of
47.25/66/80.5, including the two figures that round in opposite directions.

### Round 2 — Phase 11 only — 2026-08-29 — the same reviewer, resumed — **NOT READY**

Blockers 1 and 2 resolved; **blocker 3 was half-fixed, and the half that was missed
made the phase worse than it had been.** The scope was corrected to name the footer's
position exactly and gate clause 4 was left asserting `body`'s last element — so the
document then held a paragraph explaining that phrasing names the wrong place, and a
gate keyed to it. That is the author's error and is recorded as one: an implementer
meeting a scope and a gate that disagree is likelier to resolve it by weakening the
gate until it passes.

The round re-measured what it had queried. `offsetHeight` overshoots `innerHeight` by
one at 900px and not at 420px, exactly as the fix now states. The 627px derivation
reproduces to 626.86 from the seven children's real widths, and `flexWrap` is `nowrap`.

**One concern was raised and discharged inside the round**: clause 5 counts
`ResizeObserver` errors, the 21 were counted in a Chromium harness, and a WKWebView
that never reports the event would make the count vacuously zero. Discharged from the
repo — `tests/gates/mpdf-009-phase5.js` is pasted into a `cargo tauri dev` Web
Inspector and its own comment records a run reporting "21 uncaught", so the idiom is
proven in the target environment.

Four non-blocking, all accepted. The sharpest: **627px is the document-open,
status-worded threshold** — `#toggle` is `hidden` until a document opens and `#status`
is empty before a compile, so arming before opening reads a false negative on the
control as a pass.

### Round 1 — Phase 11 only — 2026-08-29 — fresh reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase):** the phase produces no observable and
says so in its first line; the PDF is byte-identical because no Rust reaches the
compile path, and clause 1 is the check. Still the right thing to build: Phase 10
shipped a product name the window stops saying at the first `⌘O`
(`app/src/document.rs:title`), and a status bar is where an application whose title
belongs to the document keeps its own name.

Three blocking, eight non-blocking. Nothing was rejected in any round.

**Blocker 1 is the round's best catch, and it falsified half the phase's own
motivation.** The draft claimed the file being edited is "named nowhere the panel can
be collapsed away from". But `app/src/main.rs:set_edited` ends in `window.set_title`
handed `document::title(session.preview().document())`; `app/src/preview.rs:document`
returns `edited` and `app/src/document.rs:title` returns the bare `file_name()` — **so
the title bar already carries the exact string the cell was specified to show.** The
claim was **withdrawn rather than reworded**, and the withdrawal recorded in place. The
cell is kept on a smaller argument that the phase now states outright: it is a second,
quieter copy, bought for the title bar being native chrome that macOS dims when the
window is not frontmost. **The root-relative path — the one thing the title bar cannot
carry — was weighed and declined for length, by the human author, after being shown
this finding.** The phase names the cell as the part to cut if a second placement is
not worth 24px.

**Blocker 2: the gate's sum named no measurement method, and the idiomatic one fails on
correct code.** The header is 46.5px at 13px/1.5, so `offsetHeight` rounds to 47 and a
three-term `offsetHeight` sum overshoots `innerHeight` by one at some widths and not
others — while every gate in `tests/gates/` so far reaches for `offsetHeight`. Clause 4
now names `getBoundingClientRect().height`. The round also found that the prototype
log's own derivation, "47 + 588 + 24 = 660", does not sum: it mixed two readings taken
at different moments.

**Blocker 3: scope and gate disagreed on where the footer goes.** `body`'s element
children are `HEADER, MAIN, SCRIPT`, so "after `</main>`" and "last element of `body`"
name two different places.

Non-blocking, all accepted: `edited` joined `Status` in `mpdf-010` **Phase 2**, not
Phase 1; the panel-marking quote belongs to the comment above
`app/dist/index.html:parts`, not to `rules/desktop.md` (the misattribution had
propagated from the prototype log, and was corrected there too); clause 1 credited
`core`'s suite with pinning the PDF, where `golden_test.rs` compares *Typst* byte for
byte and its PDF assertions are `starts_with(b"%PDF")` smoke checks; clause 4 named no
fixture, where every gate in `tests/gates/` does — now `tests/fixtures/panel`; a
dark/light toggle was announced as next against §1.1's parked theming, re-affirmed one
day before this phase was appended; and the empty-state duplicate brand was unstated.

**One non-blocking was already fixed before the review returned, by the author
re-deriving his own numbers.** "The header wraps to three lines at a 420px window" was
wrong twice: Chrome clamps a window below about 500px, so the prototype measured a
500px viewport and wrote 420 down anyway; and the header is `flex-wrap: nowrap` and
never wraps as a row — its items' text wraps inside them. Replaced by a derived
threshold, **627px** = 555px of children + six 8px gaps + 24px padding, which round 2
reproduced to 626.86.

**Deferred, with reason**: promoting the bare-name decision to a §2
`(decision, recorded)` subsection. Phase 10 used the same phase-local idiom, and
blocker 1's resolution moved that decision's substance into the phase body at length;
where decisions live is not a call to make inside a readiness round.


### Round 3 — Phase 10 only — 2026-08-29 — the same reviewer, resumed — **READY (converged)**

The consent blocker confirmed resolved against the sources rather than the
changelog: `app/src/watch.rs:start` is the recursive watch, `rules/desktop.md`
`## The bundle` carries the privacy-identity paragraph, and Phase 5's gate case (2)
third observation does state "the first launch of this identity" as its
precondition. **Three costs, three gates**, checked one by one: the store is
clause 6, consent is clause 7, and LaunchServices is a stated consequence whose
only checkable half — that the association survived — is clause 4's
`CFBundleDocumentTypes` assertion. The gate renumbering to 1–8 leaves no dangling
reference: nothing in this document, in `rules/`, or anywhere in the repo cites
Phase 10's gate by number.

**Clause 8 re-run verbatim with the new `--include="Cargo.lock"`**: it now reaches
`Cargo.lock:2861`, so the conjunct that was vacuous is no longer. Every hit outside
`specs/` falls inside a scoped file.

One non-blocking comment, folded in: clause 4 and the consent paragraph each
claimed to be "the one" thing that could fail silently, which two of them cannot
both be. Clause 4's phrasing predated the third cost and was softened.

### Round 2 — Phase 10 only — 2026-08-29 — the same reviewer, resumed — **NOT READY**

One new blocking finding, and it is the round's best catch: **the identifier move
resets macOS privacy consent**, which the phase's enumeration of costs stopped
short of. TCC grants are keyed to the bundle identifier, `watch.rs:start` watches
recursively, and `rules/desktop.md` already records the failure this produces — a
document under `~/Documents` compiles once through the open panel and then stops
redrawing, silently. Phase 5 wrote a by-eye gate case for exactly that and stated
its precondition as "the first launch of this identity"; this phase re-creates that
precondition for every installed copy and had no clause reaching it. Accepted: the
phase now states three costs and gate clause 7 re-runs Phase 5's observation under
the new identity, naming the negative so a draw-once-then-stop reads as a failure.

Three non-blocking, all accepted. **Clause 8's `Cargo.lock` conjunct was vacuous** —
none of its `--include` patterns match that filename, so a second person would have
reported it clean without the command having looked. **The store was cited to
`mpdf-010` Phase 5 and belongs to Phase 1**, its scope clause 4. **`md2pdf.icns` is
a build-output literal no declared source states**, so `/sync-rules` cannot derive
it from `productName`; the close-out now carves it out by hand, the same class of
miss as `desktop-project.md`.

### Round 1 — Phase 10 only — 2026-08-29 — fresh reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase):** the phase produces no observable,
and says so in its first line rather than leaving it assumed — the PDF is
byte-identical across it and clause 1 is the check. It is still the right thing to
build: the app's identity is a decision nothing else in the corpus would record, it
is the prerequisite for the footer phase, and the risk it carries is exactly what
its gate is aimed at.

Four blocking findings, all confirmed against the code before acting, all accepted.

1. **`app/src/document.rs:scratch_dir` holds a `md2pdf-app` string the scope
   forbade touching** — the phase's own grep clause would have failed against a
   file the scope excluded.
2. **"No behaviour changes" was false.** `app_data_dir()` is named from the
   identifier and `store_file` puts `projects.json` in it, so the rename orphans
   every project's remembered main. Replaced with the narrower true claim "no logic
   changes", plus a decision paragraph: **not migrated**, argued from `read_store`
   already treating a missing store as ordinary and from a one-shot migration being
   permanent code for a one-time event on an undistributed 0.1.0 app. The live
   store held three entries, two of them real project choices.
3. **`rules/desktop-project.md` states the old identifier and `/sync-rules` cannot
   fix it** — the string is in none of its declared sources. Now corrected by hand
   as part of the phase.
4. **`README.md`'s `## Install` states `md2pdf.app`**, which the scope's
   "desktop-app section" did not reach and whose dot the hyphenated grep did not
   match.

Five non-blocking, all accepted. The sharpest changed a decision: **`dev.letur.app`
was refused for `dev.letur.desktop`**, because `tauri-cli` 2.10.1 — the pinned
version — warns that an identifier ending `.app` conflicts with the bundle
extension. The warning string was confirmed verbatim in the installed binary. It is
a `log::warn!` that would not fail a build, which is precisely why it would become
a warning nobody reads. Also folded in: gate clause 5 now reads the built bundle,
because **macOS takes the application-menu title from the bundle rather than from
`SubmenuBuilder`**, so the clause was not verifying the edit it appeared to;
`app/dist/index.html`'s `<title>`; a runnable CLI invocation in clause 2; and a
forward reference that named a phase the corpus does not record.

Nothing was rejected in any round.

### Round 2 — Phase 9 only — 2026-08-27 — the same reviewer, resumed — **READY (converged)**

Both blockers confirmed resolved in the file as written, and **every re-keyed
number re-derived independently** rather than read off the changelog: 243 strict
and 41 loose under the phase's stated method, 112 nullable lookups, 35 implicit
parameters against 43 implicit variables, 2,566 mirror lines with line 1,737
identical, 149 vendored declaration files at 824 KB, and — the two the fold-in
introduced — 243 on TypeScript 7.0.2 and 274 on 5.9.3 given no `target`/`module`/
`lib`. All 41 loose errors map onto the enumerated annotation list with nothing
left over. The eight-defect enumeration was checked against the history commit by
commit.

**The best catch of the round was the author's own fold-in going further than the
finding that prompted it.** Round 1's non-blocking #4 said the `doc?.destroy()`
catch was conditional on a typed shim. Measured during the fold-in, it is worse:
with the real vendored declarations in place it still raises **nothing**, because
`let doc = null` under `strictNullChecks: false` infers `any`. It fires only once
`doc` carries an explicit annotation. That went into the scope, and gate clause 2
now states that the middle class is what catches an implementer who typed `doc`
as `any` in order to reach clause 1 — a gate written to catch the gate passing
for the wrong reason.

**One non-blocking observation, folded:** `app/src/preview.rs` already carries a
`#[cfg(test)] mod tests` running 45 tests, so the Rust half is a function added to
it rather than a new module.

**Converged at zero blocking. Phase 9 `reviewed: 2026-08-27`.** Two rounds.

### Round 1 — Phase 9 only — 2026-08-27 — one generalist, fresh, with repo access — **NOT READY**

**Round 0, asked once for the episode.** Does this produce the observable and is
it the right thing to build? It produces **none**, and the phase argues that
rather than assuming it: it is the first mechanism in this project that can tell
the observable has broken in a file whose only check is a person at a console.
The residue recorded rather than resolved: it catches one defect in eight, which
the phase states in its own scope, and the answer turns on the toolchain being
bounded to CI and never to `cargo test`. OQ-10 having asked twice is what settles
it.

**Two blockers, both of which would have stopped an implementer dead.**

1. **The Rust half had nowhere to live.** The phase put it in `app/tests/`, but
   `app/Cargo.toml` declares only `[[bin]]` — no lib target for an integration
   test to link — and `main.rs`'s `mod preview` is private. The reviewer
   reproduced `error[E0433]` on a minimal bin-only crate and established that the
   cited `core/tests/page_examples_test.rs` precedent does not transfer, `core`
   having a `[lib]`. Worse, the phase's own "no `.rs` file under `src/` is
   edited" forbade both escapes. Resolved by moving it to the `#[cfg(test)]`
   module `preview.rs` already has, and restating the constraint as no shipped
   *behaviour* changing.
2. **`declare module` cannot shim a relative specifier.** TypeScript ignores
   ambient module declarations for `./pdfjs/pdf.min.mjs`, measured three ways, so
   gate clause 1's "zero errors" was unreachable by the stated mechanism.
   Resolved by vendoring the `types/` tree from the same `pdfjs-dist` 6.2.108
   tarball as the two `.mjs` files — which also closes the round's own
   non-blocking hazard that a hand-written shim's fidelity is unchecked — kept at
   `app/types/` and deliberately not under `app/dist/`, because `generate_context!`
   walks `frontendDist` with no allowlist and would embed 824 KB in the binary.
   The mirror gains a second stated rule, the specifier rewrite, and it is
   line-preserving. The round also caught that only one of the two vendored files
   is imported as a module at all.

**Eleven non-blocking findings, all accepted, none rejected.** The load-bearing
ones: no TypeScript version was pinned and a bare `bunx tsc` resolves to whatever
is current, which moved the headline from 242 to **243** once the method was
stated exactly; the "the file does not split" argument rested on two premises the
repo falsifies, since `app/dist/pdfjs/` is already committed static modules, and
was rewritten to the one that survives; the annotation list enumerated seven
items under a heading of six and covered neither `__TAURI__` nor the
`new Promise()` hint; the mirror's location was unspecified and one of the two
plausible places both embeds it in the bundle and breaks the count; the close-out
named one rules file where the phase moves facts stated in another; and the
defect denominator was **eight**, not six — a correction that weakens the phase's
own benefit claim and was taken anyway.

**A twelfth was folded into OQ-10 rather than the phase, and taken further than
the finding asked**: `doc?.destroy()` never reached `main`, being fixed in the
same commit that introduced it, so OQ-10 now records the type check's measured
record against *shipped* defects as zero and one in eight against written ones.

### Round 2 — Phase 8 only — 2026-08-25 — same reviewer, resumed with the author's changelog — **READY (converged)**

All four blockers confirmed resolved against the file, with the reviewer
re-deriving the gate's document itself: `samples/showcase/showcase.md` is 96
lines with 22 blank ones and a longest line of 82 characters — tall enough to
scroll, blank enough for the zero-width-space clause, and as a master it also
puts the Sections panel in the row beside the gutter, which is the
two-controls-take-width case. It also confirmed the suite clause: the `.rs`
half of `6fb21a7` is entirely `mpdf-008`'s panel work, so "no `.rs` file
touched" is true of this phase.

**A live defect found, not a spec one.** With `Lines` on, `clear()` empties the
buffer and calls `relines()`, which returns early before reaching `markLine()`
— and the two other things that move the mark both need a focused, enabled
textarea, which a cleared pane is not. So closing a document left the previous
document's band painted across an empty, disabled pane until something opened.
**Fixed in `app/dist/index.html` in the same pass**, the early return now
clearing the mark on its way out; recorded in the scope and gated by a new
clause 7. This is a review of already-shipped code earning its keep.

Other non-blocking folded in on convergence: the close-out gained a *verb* —
`rules/desktop-panes.md` is **verified and regenerated where it disagrees**,
not written afresh, since the rule was written from the same prototype and
"gains the gutter" would read as an instruction to duplicate a section that
exists; gate clause 1 now narrows the pane before typing, the showcase's 82
character longest line meaning it would otherwise pass at a wide window without
testing anything; and the zero-width-space failure is restated as
**accumulating**, the rows being a running sum, where the scope had understated
it as one line's height.

**Rejected, with reason:** raising `web/index.html`'s identical missing-line-
numbers gap as an `OQ-N` here. It is `mpdf-006`'s subject, that spec's rollup
is `done`, and an open question filed against `mpdf-003` for another spec's
gap is a question no phase of this spec will ever force. The prose note stays.

### Round 1 — Phase 8 only — 2026-08-25 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode): yes.** Phase 8 produces no observable and says so, and
its argument holds — the app's errors name lines, and a line number the author
cannot see names a place they must count to. It is also the rare phase with
direct evidence: the author used it through a working session and confirmed the
numbers hold against wrapped prose and read correctly. Recorded caveat: **the
gutter's value compounds with the one-sentence-per-line reflow that does not
exist yet**, so today a wrapped paragraph is one number spanning several rows,
and the strongest form of the argument is not available until that ships.

One generalist rather than a panel — a gutter is low blast radius and §7.1's
panel is for the other case. The reviewer was told the code was **already on
`main`**, written as a prototype before the phase was drafted, so its question
had two halves: could an implementer build this from the spec alone, and does
the phase describe what shipped.

Four blocking findings, all accepted, none rejected:

1. The close-out named `rules/desktop.md` "which Phase 7's split leaves in
   place" — but that split had happened, the pane content is in
   `rules/desktop-panes.md`, and `rules/desktop.md`'s `covers` no longer admits
   a pane. An implementer would have written the gutter into the wrong file.
2. "One push, with Phase 7" made shipping contingent on a phase that is
   unreviewed and scheduled to be cut by `mpdf-009` Phase 1 — and §7's gate
   forbids implementing a phase whose `cut` is set, so the joint push could not
   happen.
3. **The gutter's scroll-follow was shipped, load-bearing and absent from the
   phase.** The code holds `#lines` in a non-scrolling box and drives its
   `scrollTop` from the textarea's; the phase carried only the *band's* half of
   that contrast — "the band needs none of this" — without the half being
   contrasted against. Built from the spec alone, the numbers part company with
   their lines on any document taller than the pane, which is every document.
   **The clearest instance of the failure mode of writing a spec after the
   code**: what is written is what the author remembers deciding, not what was
   built.
4. The exit gate could not catch the two rules the scope's longest paragraph
   states. Its three by-eye checks were typing, caret movement and the
   off-toggle — none scrolled the pane and none touched the divider, which is
   the only path that exercises either the width-change rebuild or the
   not-per-pointermove timing. Now six clauses, three of them owed to this
   finding.

Non-blocking also folded in: the phase now names `app/dist/index.html`, which
it had never cited; "the only front end that could not show it" corrected, as
`web/index.html` is a third with the same gap; the caret's line stated as
marked **twice**, the gutter row and the band, where the scope described only
the band; the zero-width space for empty logical lines and
`background-repeat: no-repeat` both recorded as facts that cost a build; "taken
at a drag's end" corrected to `settle`'s 200 ms timer, with the note that this
hook is why `mpdf-009` Phase 1 keeps `settle` whole; the by-eye departure
argued in its own paragraph rather than borrowed from a phase being cut; and
the close-out given its user-facing half, the README gaining the `Lines`
toggle.

### Round 3 — Phase 6 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, and the round exists only because §7.4 requires one
whenever changes are folded in at all. Round 2 was already READY; four non-blocking
fixes went in after it, and the loop's own warning — a fix can introduce a blocker —
is what this round tested. It did not: the diff is seven hunks, all inside Phase 6, and
no shipped phase or gate numbering outside it moved.

All four landed. The follow rule's slogan became its mechanism — "a redraw **that did
not replace the text**", which matches `state.reloaded !== takenReload` exactly — and
the figure-change case the reviewer found is named and left as it falls rather than
special-cased, with governance settled in one line: *the mechanism sentence is the
specification; the slogan is prose*. The two-by-eye departure is argued against Phase
5's precedent. `NativeElement` joined the import list. The Finder case took Phase 5's
`LSHandlerRank: Default` precondition inline.

The reviewer re-checked the fold-in's one new cross-spec citation rather than take it:
`mpdf-004`'s position-blind display arm does say what the analogy claims.

### Round 2 — Phase 6 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`. All eight of round 1's findings had been accepted — no rejections in
this episode.

**The blocker was resolved and the reviewer traced the fix through the code rather than
the changelog**, on all three questions it was asked. The signal exists where it is
needed: `refresh` computes `state.reloaded !== takenReload` before it assigns, and
`draw` has exactly one call site, strictly after the reload branch — so no flag has to
be threaded anywhere. It fires on all three of gate (7)'s paths, via `Session::open` →
`Preview::load` → `take` → `compile` with `clear()` having set `takenReload = -1`. And
it composes with the existing gating: a reload pass whose compile failed returns before
`draw` with `takenReload` already advanced, and the no-compile signals return at
`revision === drawnRevision` and never reach `draw` at all, because `revision`
increments only in `compile`'s `Ok` arm.

Four new non-blocking, all accepted and folded in — see round 3.

### Round 1 — Phase 6 only — 2026-08-15 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase, per §7.0).** *Does this produce the
observable, and is it the right one?* Yes, on this spec's own established reading: all
five shipped phases deliver "the same PDF" under a new condition — redrawn on save,
written to a file, launched from Applications — and Phase 6 delivers it at the author's
working position rather than page 1. The friction is recorded rather than assumed, and
OQ-6 deliberately declined to close this as a non-goal once Phase 4 made the caret
available. **The risk recorded rather than dismissed:** that same entry judged the
friction *tolerable*, and heading-density precision may make this close to a no-op on
the short documents that motivated it.

Verdict: `NOT READY`. One blocking finding, seven non-blocking, **all eight accepted**.

**The blocker is the best catch in this spec's record, because no gate in the phase
reached it and the phase would have shipped it.** `app/dist/index.html:refresh` assigns
`text.value = await invoke('document_text')` on a reload and falls through to `draw` in
the same pass — and assigning `value` moves a textarea's caret to the end of the
control, which is normative WHATWG behaviour the reviewer confirmed in a browser. So
the caret formula would read the document's *last* line and open `samples/article.md`
on page 3, contradicting Phase 1's shipped gate case (1), Phase 2's external-reload
loop and Phase 5's Finder open. The fix makes the follow *condition* part of the phase
— the pass that replaced the text is the pass that must not follow — and adds gate (7),
the three shipped paths that must still open on page 1.

**Two findings corrected claims this author had asserted rather than measured.** The
phase said "no shipped golden file changes" was a clause every phase had held; it is
not — `mpdf-001`'s look phase moved thirteen goldens on their second line, gaining
`date: none`, and said so in its own commit message. The sharper distinction survives
and is now what the paragraph rests on: those goldens moved because *what the document
compiles to* changed, which is what a golden pins. And "no dependency is added" holds
only while `PagedDocument` stays unnamed, since `typst` re-exports `typst-library`,
`typst-syntax` and `typst-utils` but not `typst-layout` — so the extraction staying
inline in `md_to_pdf_with_anchors` became a constraint rather than a preference.

**One instruction to measure became a measured gate case.** Gate (4) had told the
implementer to find out whether a count mismatch is reachable from markdown; the
reviewer answered it, and the author reproduced it — `[^1]: # A heading in a note`
emits `#footnote[= A heading in a note]`, a heading `collect_definitions` walked into a
discarded `Walk`. That document is now the second half of the case.

The rest: `Selector::Elem` takes `(Element, Option<SmallVec<…>>)` so the idiom is
`Element::select`; the count guard was overstated and does not catch one-extra-and-one-
missing, and what it guards is a mis-scroll rather than a wrong document; §2's "A
re-render therefore returns the reader to the first page" goes stale and the close-out
had missed it; and "corrected in place" was replaced by §6.1's actual remedy, a dated
`CORRECTED` note beside the text with the original kept.

**On the §2 crossing**, which the reviewer was asked to judge: the argument is adequate
and a phase is the right mechanism. §6.1's ordered test lands on step 2 — the preview
pane is this spec's own subject and OQ-7 reserved this item by name — and step 1 does
not divert it, since nothing shipped is removed, `md_to_pdf`'s signature is unchanged
and `cli/src/main.rs` keeps calling it. What it needed was the correction-mechanism
wording, not a different spec.

Numbers re-derived this round, so a later one can trust them: `tests/golden/` holds
exactly 17 `.typ` files; `rules/desktop.md` is 390/394 and `rules/pipeline.md` 338/340;
`samples/article.md` is three pages and its last heading lands on page 3, which is what
makes gate (2) non-vacuous.

### Round 4 — Phase 5 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, zero non-blocking residuals. **This round is past
§7.6's cap, and a person decided to run it**, which is what that rule reserves to a
person. The escalation in round 3 named two ways to close the blocker and recommended
the narrow one; the decision was the narrow fix plus one more round, and the reason
given for the round rather than a straight acceptance was the episode's own pattern —
the author's fixes had introduced a blocker twice, in rounds 2 and 3, which is exactly
what a further reviewer pass catches.

**The reviewer checked the two claims it was asked to check rather than argue, and
both hold.** The icon paragraph's numbers are now attributed to the two different
producers, which is the confusion that made the previous draft contradict itself: the
bundler synthesises one `ic09` at 19,582 bytes from a lone PNG, and `cargo tauri icon`
writes 52 files and a 12-entry icns at 74,735 bytes that `bundle.icon` never reads. No
sentence anywhere in the phase still claims the command is run.

**Gate (1)'s "and nothing else" was verified by building it**, with the configuration
the phase now specifies, and the bundle is three files: `Contents/Info.plist`,
`Contents/MacOS/md2pdf-app`, `Contents/Resources/md2pdf.icns`. Two qualifications came
with it, neither a finding, and both worth keeping. The probe used `cargo tauri bundle`
over a pre-built binary rather than `cargo tauri build`, which is the same bundler code
path. And **the assertion survives the signing branch**: a signed bundle gains
`Contents/_CodeSignature/`, not a `Resources/` entry, so the case does not silently
become false if the credentials OQ-8 waits on ever appear.

The run-event sentences were judged sufficient to build from. The one thing left
unstated is `Url::to_file_path`'s `Err` arm, which the reviewer recorded as not a
guess: it is unreachable for a `file://` URL from LaunchServices, and handling it is a
no-op. The close-out was verified line by line, including both command-count strings —
line 59, wrapping to 60, and line 96 — and the 303-against-307 line count re-derived.

On this convergence: `reviewed: 2026-08-11` on Phase 5. `status` was already
`accepted`. Every phase in this spec now carries a `reviewed` date, and Phase 5 is the
only one with `shipped: null`.

### Round 3 — Phase 5 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY (escalated at the cap)**

Verdict: `NOT READY`, one blocking finding, four non-blocking. This is §7.6's cap,
so the episode **escalates to the human rather than running a fourth round**, and
Phase 5 keeps `reviewed: null` — the date records convergence, and this did not
converge. What is outstanding is one paragraph and one configuration key, and the
reviewer named both ways to close it, which is why the escalation is a decision to
make rather than a problem to solve.

Both round-2 blockers were confirmed resolved **against the files**. The reviewer
traced the new open path through `app/dist/index.html` as it stands and found the
hook real rather than invented: the script registers its four `listen` calls and
*then* calls `refresh()` as its last statement, so the startup take sits where the
scope says it does. It also recorded why that ordering is load-bearing — the listens
are issued before the take on the same channel, so an `Opened` landing before the
page's script ran is collected by the take, one landing after by the listener, and
the take clearing the slot is what stops the second collector opening the document
twice.

**The blocker is the icon paragraph, and it was introduced by the fix for round 1's
icon finding** — the pattern §7.3 warns about, a fix that introduces a blocker,
landing on this episode for the second time. The paragraph claims `cargo tauri icon`
yields "a valid `.icns` with one entry". Measured: the command writes **52 files**
across eight subdirectories including iOS, Android and Windows assets, and its
`icon.icns` carries **12 entries** at 74,735 bytes. The one-entry icns is what the
*bundler* synthesises from a lone PNG when the command is **not** run — 19,582 bytes,
a single `ic09`, which is how the icns in the round-1 probe bundle was produced. So
the sentence's two halves cannot both be true, and `bundle.icon` stays
`["icons/icon.png"]` either way, which means running the command changes nothing
about the `.app` at all. Gate (1) asserts only that `Contents/Resources/` "holds the
icon", which passes under every branch and catches none of it.

The four non-blocking findings, recorded for whoever closes this: `Url::to_file_path`
is not named, and a path with a space arrives percent-encoded as
`file:///…/my%20doc.md`, which `url.path()` does not decode — `tauri` re-exports
`Url`, so this needs no dependency, which is worth a clause in a crate that pins
every one it has. `RunEvent::Opened` carries a `Vec` and the slot is singular, so
several `.md` files selected at once is a choice the phase does not make. The
store-emit-take leaves one residual race — `listen()` completes over IPC, so an event
landing between the startup take and the listener's registration would sit in the
slot until the next signal — practically unreachable by the source ordering, and this
spec's habit is to write such limits down beside the mechanism. And the close-out
names two stale claims in `rules/desktop.md` where there are three: it also says
"registers eight commands" and "Each of the eight commands is a wrapper over a plain
function", which the ninth command and the `opened` signal both falsify.

### Round 2 — Phase 5 only — 2026-08-11 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, two blocking findings, both found by measurement rather than by
reading, and four non-blocking. All three of round 1's blockers were confirmed
resolved against a real bundle the reviewer built and inspected.

**The first blocker was in the author's own fix, and it was exact rather than racy.**
The scope had claimed the page needed no change at all, citing two facts that are
each true — `app/src/preview.rs:Session::open` does call `on_render`, and
`Preview::take` does bump `reloaded`. They do not compose. `Session::open` rebuilds
from `Preview::default()`, so `revision` and `reloaded` restart at 0 for every
document, while the page's `drawnRevision` and `takenReload` reset only inside
`app/dist/index.html:clear()`, which the dialog path calls before it invokes and a
path straight into Rust never reaches. A second document opened from Finder returns
Rust to `revision 1`, `reloaded 1`, which the page already holds from the first: both
panes keep the old document under a new title. The cold launch happens to work only
because the page's counters start at `-1`.

The fix routes the open back through the page, and the author took it one step
further than the reviewer proposed, because the emit alone carries a launch race of
its own: on a cold open the run event can fire before the page has registered its
listener. The scope now specifies the payload-less signal **plus a take** — `RENDERED`
→ `current_pdf`'s existing shape one command over — with the path in a managed slot,
a ninth command that returns it and clears the slot, a take at startup and a take on
every signal. `app/src/main.rs:open_document` keeps its signature and its `async`,
which also dissolved a non-blocking finding about the compile moving to the main
thread and preserved Phase 1's recorded reason for that `async`.

**The second blocker was a sentence written to save a build that cost one.** The
scope said the UTI key is "spelled `content-types` in the file by its own serde
alias". `tauri-utils` does declare `content_types` with that alias, but the CLI never
reaches serde: it validates against the generated JSON Schema first, where an alias
does not appear and `deny_unknown_fields` becomes `additionalProperties: false`, so
`cargo tauri` 2.10.1 stops with "Additional properties are not allowed
('content-types' was unexpected)". The working spelling is `contentTypes`, and the
schema-before-serde reason is now recorded with it.

**The second by-eye item was judged and not blocked on.** The reviewer accepted the
argument — §2 caps the list for the claim "the right pixels reached the glass", and
this phase's claim is that a `.app` runs away from `cargo` and that LaunchServices
routes a document into it, which no `cargo test` can double-click. It then found two
of the three observations unreproducible as written, both folded: the emitted entry
ranks `LSHandlerRank` as `Default`, so any machine with an editor already registered
for `.md` keeps that editor and the observation must name Get Info → Open With →
Change All (and say that `rank: "Owner"` is the wrong fix); and the `~/Documents`
case was a single positive against a consent that is sticky, so it now names its
precondition and the negative it watches for. Two smaller ones folded too: the
`otool` path is `Contents/MacOS/md2pdf-app`, because `productName` renames the `.app`
and not what is inside it, and the cold-launch mechanism was misattributed —
`AppState::open_urls` calls `handle_nonuser_event`, which **drops** an event when no
callback is set rather than queueing it, and what saves the cold case is that tao
installs the callback before `NSApp.run()`.

### Round 1 — Phase 5 only — 2026-08-11 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0, for this episode:** yes. Phase 5 produces the observable — the same
typeset PDF — for a consumer who holds no Rust toolchain, which is the one gap the
four shipped phases leave: §1's author still reaches the window only through `cargo`.
It is also the right one, because its gate case (1) was drafted as the first check of
`mpdf-001`'s standing claim that the fonts ship inside the binary. The round then
falsified that half of the reason, which is recorded below and does not change the
answer.

Verdict: `NOT READY`, three blocking findings, nine non-blocking. The reviewer built
a real bundle in a scratch copy of the tracked tree and inspected it rather than
reading about it, which is what produced every measurement in this episode.

**Blocker 1 — gate case (2) required code the scope never mentioned.**
`bundle.fileAssociations` makes LaunchServices *launch* the app and hands the process
nothing; the path arrives as `tauri::RunEvent::Opened { urls }`, which
`app/src/main.rs:main` cannot see, because it ends `.run(tauri::generate_context!())`
and surfaces no run events at all. An implementer doing exactly what the scope said
would get a blank window. Folded: the scope now names the `.build(…)?.run(|handle,
event| …)` change, the event, the `file://` payload, that a bundled app is handed its
document by that event and not in `argv`, and the semantics of a second open.

**Blocker 2 — the phase named no file, no symbol and no configuration key**, against
§3's requirement that a phase name what it touches; Phase 4 names eight-plus and
Phase 5 named none. The key it turns on is `bundle.active`, `false` in
`app/tauri.conf.json` today, and the two obvious commands disagree under that value:
`cargo tauri build` skips bundling while the standalone `cargo tauri bundle` bundles
anyway. Folded, with the measurement, in OQ-2's idiom.

**Blocker 3 — gate case (1) was not reproducible, and its premise was false.** It
asked for a launch on "a machine that has no Rust toolchain and no fonts installed";
macOS cannot be in the second state, and the first is checked better by `otool -L`
than by finding a second machine. Worse, whether the case passed turned on how the
bundle travelled rather than on what the build produced: unsigned, it carries only
the linker's ad-hoc signature — `codesign -dv` reports
`flags=0x20002(adhoc,linker-signed)` with `Sealed Resources=none` — and `spctl -a
-vvv -t exec` rejects it, so a copy over USB runs while a downloaded `.dmg` is
blocked by Gatekeeper until a person overrides it by hand. **The case was replaced
rather than weakened**, on the precedent OQ-3 set for Phase 2: gate (1) is now four
shell checks over the built `.app`, with `codesign` and `spctl` recorded rather than
asserted, because the unsigned branch fails `spctl` by design. The distribution
question left the gate entirely and became **OQ-8**.

**The fonts clause was falsified in passing and is worth its own line.** Gate (1)
called the launch "the last check that the bundled fonts really are bundled".
`core/src/lib.rs` embeds all five faces with `include_bytes!` and the Typst world
exposes those alone, so that is a compile-time fact no launch tests and no packaging
can break. What packaging *can* do is grow a font under `Contents/Resources/` that
somebody added on the theory it was needed, so the gate now asserts the absence, and
the scope says the phase adds nothing to `bundle.resources`.

The other non-blocking findings, all folded: gate (3) was self-certifying and now
names the command, the README section and the two output paths; the icon was
ambiguous and the phase now declines new artwork explicitly, correcting
`rules/desktop.md` rather than meeting it; `LSItemContentTypes` is not emitted by
default and the fallback key is named; `tauri-cli` was the one unpinned thing in a
repo that pins everything, and pins at 2.10.1 with the `tauri-utils` 2.8.3-against-
2.9.3 `deny_unknown_fields` hazard recorded; a bundle gets its own privacy identity,
so the recursive watch under `~/Documents` needs a consent the terminal-launched
build inherited, now in scope and in the gate; and the close-out's `max_lines` was
four lines from its cap.

**One finding was rejected as a factual error**, and the reviewer confirmed the
rejection in round 2: it reported that the README has no install section. `README.md`
line 10 is `## Install`.

### Round 3 — Phase 4 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, four non-blocking residuals, all four folded
after the verdict. This was the loop's cap, and it converged on it rather than
past it, as Phase 3's episode did.

The reviewer verified the round-2 blocker's fix by reading OQ-5, §2 and Phase 4
against **each other** rather than against the changelog, and confirmed all
three now say one thing. It took the three questions the changelog asked it to
attack and answered each from the rule rather than from the prose. **The three
outcomes are exhaustive**: the partition is `file == buffer` first and
`buffer == last-saved` second, so `F==B` takes the first outcome whatever `S` is
and `F!=B` splits cleanly on `B==S`, with no combination falling through and no
dirty flag needed. **The clean-buffer branch preserves Phase 2 as shipped**: an
author who is not typing has `B==S`, so an external save redraws with no action
in the window, which is exactly what Phase 2's by-eye case read. **Gate (2)
still holds**: a save sets `S = B`, the event arrives with `F == B`, the first
outcome fires, nothing compiles — and it cannot flake, because the test owns the
buffer.

**The reviewer also withdrew its own non-blocking finding, having found why it
was wrong**: it had read the vendored `tauri-plugin-dialog` 2.6.0, which does
ship `ask.toml` and `confirm.toml`. The pin is 2.7.2, which does not. That is
the second time this episode that reading a version other than the pinned one
produced a finding, and §2 now records the mechanism so the next reader does not
repeat it.

The four residuals, all folded: an author typing between a save and that save's
event lands in the third outcome, so the app can name a divergence that was
really its own write — it loses nothing and the next save clears it; an external
writer that writes exactly the author's unsaved text takes the first outcome and
leaves the last-saved text unrefreshed; the Rust-to-page direction for the
replacement text and the divergence report was unnamed, and now follows Phase
3's precedent explicitly; and a divergence is **not** `stale`, because nothing
failed to compile, with its placement left as an implementation choice the gate
does not turn on.

On this convergence: `reviewed: 2026-08-10` on Phase 4. `status` was already
`accepted`. Phase 5 keeps `reviewed: null`.

### Round 2 — Phase 4 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY` — one new blocking finding, four non-blocking. Five of the
six were accepted; **one was rejected**, the first rejection of this document's
seven rounds. All five round-1 blockers were confirmed resolved against the
files.

**The blocker was an error in the author's own round-1 fix, and it was the fold
half-landing.** Round 1's fix had already been corrected once before it was sent
— the first draft said the document's path "leaves the watch filter", which
contradicts the divergence check that a dropped path could never reach — but the
correction reached §2 and **not** OQ-5's resolution. The reviewer's argument for
why that blocks is the finding's real value: "the watch filter" has a fixed
referent in this document, §3 is *the record* of a resolved question under §4 of
the methodology, and an implementer reading it first builds a filter that drops
the document, passes gate (2)'s first half, and leaves gate (3)'s divergence
report unreachable. OQ-5 now says the path **stays** in the filter, and says so
alongside a sentence recording that an earlier draft said the opposite.

**The best non-blocking finding changed the answer rather than the wording.**
The reviewer observed that Phase 4, as fixed in round 1, removed a shipped
behaviour and said so only obliquely: an unconditional refusal falsifies the
README's "Save the file and the page redraws", Phase 2's by-eye gate case, and
`rules/desktop.md`'s "recompiles on every save". Folding that as documentation
was available and was the wrong fix. §6.1 holds that contradicting shipped work
is never what a phase does, so **OQ-5's answer gained a condition instead**:
refuse when the buffer holds unsaved edits, take the disk copy when it does not.
That costs one comparison, needs no dirty flag, and leaves Phase 2's loop
untouched. Gate (3) went from two cases to three, one per outcome, with the
failure mode named.

**The rejection.** The reviewer held that `dialog:allow-message` was the wrong
permission for the two-choice prompt §2 rejects, and cited `ask.toml` and
`confirm.toml` "in the same directory". Those files do not exist at the pinned
version: `tauri-plugin-dialog` 2.7.2 ships only `message.toml`, `open.toml` and
`save.toml`, its `generate_handler!` registers exactly `open`, `save` and
`message`, and both `ask` and `confirm` in `guest-js/index.ts` call
`messageCommand`, which invokes `plugin:dialog|message`. The original text was
right. The finding was folded in the direction it was useful anyway — §2 now
states that mechanism, so the next reader does not re-derive it. Round 3 found
the cause and withdrew the finding.

The other two non-blocking, both accepted: where the buffer lives was derivable
but unstated, and is now named along with its consequence — keystrokes cross the
IPC boundary and the debounce is Rust's, which is what makes gate (1) a test at
all; and OQ-7 sat between OQ-5 and OQ-6, so §3 read 1, 2, 3, 4, 5, 7, 6.

### Round 1 — Phase 4 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 was **not re-asked**, on the same grounds Phase 3's round 1 recorded:
§7.0 asks it once per episode and forbids re-litigating it, and this document's
round 1 for Phase 1 answered it for this episode, the drafted document.

Verdict: `NOT READY` — five blocking, seven non-blocking. All twelve accepted,
none rejected, none deferred.

**The reviewer's opening move was the round's most useful one: Phase 4 cited no
`file:symbol` at all**, so there was nothing to verify, and it verified the
phase's implicit assumptions instead. Exactly one held — `app/src/main.rs:menu`
had deliberately reserved `Cmd+S` for this phase and said so — and the rest
failed or were unstated. It also noted that the phase keyed to no number, in a
document whose Phases 2 and 3 both key to measured constants.

**The five blockers were four omissions and one knot.** (1) **OQ-5 was still
open**, and both the scope and gate case (3) deferred to a resolution that did
not exist — verbatim the shape that blocked Phase 1 on OQ-2, Phase 2 on OQ-4,
and Phase 2's gate case (2) on OQ-3. (2) **Gate case (2) contradicted the
shipped watch loop**: `is_relevant` admits the document's own path and
`Preview::compile` re-reads unconditionally, so a save from the text pane would
compile a second time, and nothing in the scope built a mechanism to stop it —
with the obvious workaround racy on the project's own measurement, since a
save's first event reaches the process 12 ms after the write. (3) **The phase
named no file and no function**, against §3's requirement, and the path it needs
does not exist: every compile in the tree reaches `std::fs::read_to_string`
through `document::render_with`. (4) **The gate never said which cases were
tests**, and there is no JavaScript test harness in the repository, because
OQ-2's `withGlobalTauri` decision removed the npm toolchain — so gate (1) was
either a silent second by-eye item against §2's one-item cap or a test with
nowhere to live. (5) **OQ-6's resolution, written hours earlier, handed Phase 4
a named design question**, and Phase 4 was silent on it.

All five were resolved together, because OQ-5's answer supplies blocker 2's
mechanism. OQ-5 resolved to "refuse the reload" and landed in §2 as its own
decision; §2 states that the document's path stops triggering a recompile
directly, which removes the race rather than racing it; the scope names the
compile chain and the split that makes a string compile, and records that `core`
needs nothing because `md_to_pdf` already takes a `&str`; the gate became five
cases, all tests, each keyed to a seam that exists, with the unexercised user
path recorded as a cost; and OQ-7 took cursor-following out of Phase 4, with the
reason read off `core/src/lib.rs`'s actual exports.

The seven non-blocking, all accepted: the typing debounce had no constant and no
method, and now has both, measured against the compile it gates rather than
against FSEvents; the `cargo test --workspace` and untouched check was missing
again, as it had been in Phases 2 and 3, and is restored as gate (5); gate (4)
did not say what round-trips against what, and now names the buffer at save as
the baseline and the CRLF hazard it aims at; §1 said Phase 4 "is last", which
Phase 5 falsifies; **the close-out named none of the claims this phase makes
false**, which was the best of the seven and grew in round 2; and the save's
menu item and accelerator were unnamed.

Rejections: none.

### Round 3 — Phase 3 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two cosmetic notes, both folded after the
verdict. This was the loop's cap, and it converged on it rather than past it.

The reviewer verified the round-2 fix against the code rather than the
changelog, and made the correction sharper than the finding had been: `empty` is
not merely one more state but exactly the right boundary, because
`Preview::compile` early-returns when `document` is `None` and `Session::open`
sets the document and compiles inside one lock scope, so no observable state
sits between `Preview::default()` and the first outcome. It re-resolved every
citation in the new text and re-confirmed the three facts its earlier rounds had
established from outside the tree — `dialog:allow-save` in the plugin,
`CARGO_BIN_EXE_*` reaching an integration test of a `[[bin]]`-only package
alongside that package's `[dependencies]`, and `samples/article.md` naming its
two figures once each, so the in-test asset read has no dedup subtlety.

The two notes: gate (2) said export is refused "in both states that have no file
to write", which is loose for *stale*, where the bytes exist and the reason is
that they are known out of date — it now says **refused unless the pane is
current**, which is two refusals to test and one rule. And "three of those four
facts" sat two paragraphs from "four states", two different fours; the sentence
is rewritten so only one four is in play.

On this convergence: `reviewed: 2026-08-10` on Phase 3. `status` was already
`accepted`. Phases 4 and 5 keep `reviewed: null`.

### Round 2 — Phase 3 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY` — one new blocking finding, four non-blocking. All five
accepted, none rejected. The three round-1 blockers were confirmed resolved, and
the reviewer checked B1's fix by **building a throwaway workspace of the same
shape** rather than reasoning about Cargo: a `[[bin]]`-only package with no
`[lib]`, a path dependency, and an integration test that compiles with both
`CARGO_BIN_EXE_<bin>` and a `use` of the package's dependencies. That is what
established that gate (1b) can live in `cli/tests/cli_test.rs` at all.

**The blocker was an error in the author's own round-1 fix**, and the best
finding of the episode. The fix claimed `app/src/main.rs:current_pdf` "already
distinguishes the last two by its two `Err` branches". It does not.
`Preview::compile` sets `stale` on **every** failure, so *stale* and *failed*
both take the first branch; the second is reachable only when the flag is clear
and there are no bytes, which is `Preview::default()` — the state the app
launches into and holds until the first Open, and the one the three-state
enumeration had omitted. Confirmed in the code before the fix was folded.

The consequence was not cosmetic. Gate (3) asked for one test per state over an
enumeration missing a state, and gate (2) refused export "while the pane is
stale", which is `false` at launch, when there is also no file to write. The
enumeration is now four states, the real distinguisher is named
(`Preview::pdf().is_some()`), the misattribution is **written out rather than
silently repaired**, gate (3) is four cases, and gate (2) covers both refusals
with a sentence on why the second is not the first.

Of the four non-blocking, one closed a hole in the round-1 fix itself: the split
gate tied the app's bytes to `Preview::pdf()` and the CLI's file to `md_to_pdf`,
but **the middle leg — that those are the same bytes — was argued and not
gated**, resting only on §2's line-for-line reading of the two asset readers, so
a later divergence in either would have passed both halves while the wrappers
disagreed. Case (1a) now asserts against an in-test `md_to_pdf` call, and the
spec says that assertion is not optional. The other three: neither half named
its document, and both now take `samples/article.md`, with
`tests/fixtures/figure.md` named as the trap because its `figures/mark.svg` is
absent and both sides would fail rather than agree; the export's user path is
exercised by nothing, which is the cost of an all-tests gate and is on the
record beside it; and one line overran the wrap. The reviewer also noted that
§2's timings include a process spawn the app does not pay, so the recorded
stale window is conservative rather than wrong — folded.

Rejections: none.

### Round 1 — Phase 3 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 was **not re-asked**. §7.0 asks it once per episode and forbids
re-litigating it, and this document's round 1 for Phase 1 recorded it as
answered for this episode — the drafted document. Phase 3 was drafted with it
and states that it produces the observable.

Verdict: `NOT READY` — three blocking, ten non-blocking. All thirteen accepted,
none rejected, none deferred.

**The three blockers were one knot: gate case (1) could not be run.** Blocker 1:
it asserts the export is byte-identical to what `md2pdf <the same document>`
writes, and **no crate boundary in the workspace permits that comparison** —
`CARGO_BIN_EXE_md2pdf` is set only for integration tests of the package defining
that binary, which is `cli`, and `app/Cargo.toml` declares a `[[bin]]` with no
`[lib]`, so nothing in `app/src/` is importable either. Four materially
different builds were available to an implementer and the phase named none.
Blocker 2: **either reading of the gate's form broke something** — as a test it
had nowhere to live, and as a by-eye check it silently added a second item to a
list §2 says stays at one, where Phases 1 and 2 both label their by-eye case
explicitly. Blocker 3: **half the phase's scope carried no gate at all** — an
implementer could build the Rust state, ship `app/dist/index.html` unchanged
from Phase 2, and pass everything, which fails the methodology's §3 rule that a
gate is sized to its blast radius.

All three were resolved together. Gate (1) is **split and composed**: (1a) in
`app`, the export writes the bytes `Preview::pdf()` holds; (1b) in
`cli/tests/cli_test.rs`, where the binary already is, the CLI's file equals
`md_to_pdf` called in process. §2 gains a decision recording the Cargo fact so
the next reader does not rediscover it. Both halves being tests dissolves
blocker 2, and the gate now says so in as many words. Blocker 3 is closed by
making the status **a value a plain function computes**, which the gate tests
per state and the page merely renders — §2's own rule about where logic lives,
applied to keep the by-eye list at one item.

**The reviewer measured the faithfulness claim rather than arguing it**, which
is what makes the split defensible: `samples/article.md` compiled in five
separate processes gave five identical files, each matching a
`samples/article.pdf` a different build had produced seven hours earlier;
`samples/press-release.md` gave 3/3 across processes, also matching an older
build; a PNG-bearing document gave 3/3, and the same content under a different
file name gave the same bytes, so output does not depend on the path. It also
read `app/src/document.rs:read_assets_with` against `cli/src/main.rs:read_assets`
line for line and found no divergence — the risk the round was asked to look for
does not exist in the tree. All of it is now in §2 with its method, together
with what the claim rests on and the note that a Typst release could falsify it
silently, which is why the gate checks it.

The other nine non-blocking, all accepted: the state machine's "three states"
were two in the code and unnamed; the compile duration and an accessor for the
open document do not exist and are now stated as things the phase adds; the
status had no channel, which is the omission Phase 2's round had already treated
as a finding; the Save dialog and `dialog:allow-save` were unnamed where Phase 1
named both because the fact cost a build; both sides of gate (1) wrote to the
same path; the stale flag answers "did the last compile fail" and not "does the
page match the disk", now recorded as a limit the phase accepts; the gate had
dropped the `cargo test --workspace` and untouched check that Phases 1 and 2
both carry, restored and narrowed to `core/src` and `cli/src` because (1b)
deliberately adds one case to `cli/tests/`; the close-out did not raise
`rules/desktop.md`'s cap, at 151 lines against 155; and "the window shows the
open document" was already shipped as the window title.

Rejections: none.

### Round 2 — Phase 2 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, seven new non-blocking, all accepted and
folded after the verdict. The reviewer verified against the file rather than
the changelog, confirming the working tree held only the spec's own diff and
that `app/` still sat at `fa75a13`.

It strengthened one of the round's own claims rather than merely checking it.
§2 argues that every legal image path resolves under the document's directory
because `core/src/emit.rs:check_image` refuses a scheme, a leading `/`, a `..`
segment and a backslash; the reviewer found that `check_image` is called
**inside the walk `emit()` runs**, so `image_paths` enforces those refusals
itself rather than only `md_to_pdf` — an illegal path never reaches the list,
it fails the call. It also confirmed the two gate cases the author flagged for
scepticism: `notify`'s stream carries `kFSEventStreamCreateFlagNoDefer`, so
gate (2)'s bounded wait absorbs only the debounce and FSEvents' coalescing, and
FSEvents watches a tree **by path** rather than by descriptor, which is exactly
why the directory answer reaches a subdirectory that did not exist when the
watch began — the case `append_path`'s `path.exists()` check denied the file
answer. It noted gate (3) is self-defending: an implementer who recomputed the
list after a successful *compile* rather than a successful *parse* would have
no list at open and fail that case alone.

**The best finding was NB-1, and it would have shipped a silently dead loop.**
`notify` canonicalizes a path as it registers it, because FSEvents reports the
resolved path, while the filter as drafted compared paths as the Open dialog
produced them. Verified here independently: `std::env::temp_dir()` is
`/var/folders/…` whose real path is `/private/var/folders/…`, and `/tmp` and
`/var` are both symlinks into `/private`, so this is the default case. Every
event would fail the filter, and the app would run, watch, and never redraw.
§2 now records it in the idiom OQ-2 used for the icon facts, the filter
canonicalizes both sides, and two gate cases were rewritten to catch it — one
unit case over `/var` against `/private/var`, and a rule that gate (2)'s
scratch directory sit under `std::env::temp_dir()` so the case cannot pass
under a directory that happens not to be symlinked.

The other six, all folded: the phase did not say **where the compile lives**,
which read two ways, one of them two compiles per save with a race between
them — it now says the compile happens once, in the loop, and that the page's
invoke returns bytes already compiled; gate (2) needed an observable, and now
names `app/src/document.rs:read_assets_with` as the seam Phase 1 built for the
same job; "successful parse" was load-bearing and undefined, and now says
emission; a figure that is a symlink out of the directory is not covered, and
is recorded as a limit rather than fixed; no gate opened a **second** document,
so an implementer who set the watcher up once would have passed everything —
that is now gate case (4); and two §4 hygiene nits, the struck questions having
dropped their classification sentences instead of striking them, and a bare
`OQ-6` in §2 that meant `mpdf-001`'s, now written without the token.

Timings were not re-measured this round. Round 1's 9.0 ms and 29.0 ms against
§2's recorded 8.5 and 28.7 stand, and the whole-recompile argument reproduces.

On this convergence: `reviewed: 2026-08-10` on Phase 2. `status` was already
`accepted`. Phases 3 to 5 keep `reviewed: null`.

### Round 1 — Phase 2 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 was **not re-asked**. §7.0 asks it once per episode and forbids
re-litigating it, and this document's round 1 for Phase 1 recorded it as
answered for this episode — the drafted document. Its answer covers Phase 2,
which §1 names as the phase that turns the app from a viewer into a loop, and
which states that it produces the observable.

Verdict: `NOT READY` — three blocking, ten non-blocking. The author accepted
all thirteen, rejected none, deferred one to a new open question. Two blockers
were the same shape round 1 on Phase 1 hit: **an unresolved open question
deciding the phase's mechanism.**

Blocker 1: **OQ-4 was unresolved and the scope deferred its central mechanism
to it** — "with OQ-4's answer deciding whether the watcher takes the files or
their directories" leaves two materially different builds. Blocker 2: **the
phase named no file-watching crate and none was in the tree**, so OQ-4 had
nothing to be read against; the two were one knot. Blocker 3: **gate case (2)
was keyed to OQ-3's resolution, which did not exist.**

Blockers 1 and 2 were resolved by reading the crate. `notify` 8.2.0's macOS
backend refuses a path that does not exist — its own `append_path` opens
`if !path.exists() { return Err(Error::path_not_found()…) }` — so a file-valued
set cannot hold the `figures/new.svg` case the question poses. **The answer
then came out smaller than the question assumed.** `check_image` refuses a URI
scheme, a leading `/`, a `..` segment and a backslash, so every legal image
path already resolves under the document's own directory: the set is one
recursive watch on that directory, needing no recomputation when an edit adds
or drops a figure, and computable from the document's path alone — which also
dissolved a separate non-blocking finding about what to watch for a document
the dialect refuses. `image_paths` keeps a second job one layer in, as the
filter rather than the set.

Blocker 3 was resolved by probing the running Phase 1 app three times, the app
reverted afterwards. **Both halves of OQ-3's own guess were wrong.** It
expected "the honest floor may be an offset rather than a semantic position";
the floor is lower — after a human scrolled several pages by hand, the parent
saw `scrollTop` 4 and `scrollY` 4, which is the four pixels of slack between
the frame's document (`scrollHeight` 729) and its viewport (`clientHeight`
725), the `<embed>`'s `scrollTop` 0, no enumerable properties on it, and no
`hashchange`, so the view does not write its page into the fragment either.
And the ceiling is higher — `#page=N` on a **fresh** blob URL, which is what
every recompile produces, is honoured at load, confirmed on that operation
rather than on a cheaper same-document one. Read impossible and write working
is the combination the question did not consider.

Gate case (2) was therefore **deleted rather than weakened**: a gate keyed to
an impossibility is not a gate. Phase 2 ships without the property and §2
records the cost as its own decision. The residual became **OQ-6** — take §2's
`typst-svg` escape hatch, or accept the cost permanently — which blocks
nothing. The human owner chose that over taking the hatch inside this round, on
the grounds that felling a recorded §2 decision as a side effect of unblocking
a phase is the wrong way to make it. The round also corrected a sentence no
finding had named but the measurement falsified: §2's blob decision had claimed
the same-origin route was what made the scroll offset reachable.

The ten non-blocking, all accepted: the phase named no channel for pushing a
redraw, and the obvious one reintroduces the JSON-array-of-numbers cost §2's
IPC decision already refused; three of four gate cases were read by a person
against §2's rule that the list stays at one item; gate (4) named no document
and no image; the debounce constant asked for "its measurement beside it" with
no method, where §2's own timings state one; Phase 1 shipped no "current page"
state, which gate (3.3) located in Rust only by implication; the watch set was
uncomputable for a document the dialect refuses; the stale mark overlapped
Phase 3 with no boundary drawn; the close-out omitted the README, which this
phase makes false, and `rules/desktop.md` sits at exactly its 80-line cap; and
the gate dropped Phase 1's falsifiable "`core` gains nothing" check at the
phase most likely to leak into `core`.

The reviewer re-measured §2's compile timings over 20 subprocess runs each:
`samples/press-release.md` 9.0 ms and `samples/article.md` 29.0 ms, medians,
against the recorded 8.5 and 28.7. The argument reproduces.

Rejections: none.

### Round 2 — Phase 1 only — 2026-08-10 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, four new non-blocking. The reviewer
verified against the file rather than the changelog, diffing the working tree
against the committed `604c3f8` and confirming that the diff held what the
changelog claimed and nothing else.

Both blockers are resolved. What makes this round worth reading is that the
reviewer independently checked the probe's checkable half, in the vendored
crate sources rather than by trusting the report: `tauri-codegen` 2.6.3 really
does carry the `panic!("failed to open icon …")` the round hit;
`with_global_tauri` really is a field of the same config struct as `windows`
and `security`, and really does default to `false`, so setting it is
necessary; and `tauri-plugin-dialog` really does ship an `allow-open`
permission that namespaces to `dialog:allow-open`. It added one fact the
decision leans on and the round had not checked: `SecurityConfig.csp` is
`Option<Csp>` defaulting to `None`, so no default policy silently blocks a
`blob:` frame — the route does not depend on a config key the spec omits.

It also named honestly what it could not verify: the probe itself, being a
throwaway outside the tree. It took `navigator.pdfViewerEnabled`, the
`contentType` and the single `<embed>` on the author's report, and recorded
that its own round-1 web sweep had leaned the other way — which is the reason a
probe on the target machine outranks it. Its own re-measurement of the timings
came within half a millisecond of the round's: 8.9 ms and 28.3 ms against
8.5 ms and 28.7 ms.

The four new findings, accepted and folded after the verdict. §2's "this
project's gates are tests" sat against a gate the amendment had made
explicitly manual, so that rule now says plainly that it governs where logic
lives and that exactly one claim — whether the right pixels reached the glass —
is read by a person. Gate (3)'s third case did not name its directory, where
its two siblings did. **The SVG fallback had vanished from the live text while
the constraint hardened from "if it can be avoided" to unconditional** — the
best of the four, because the escape hatch survived only inside a struck-through
open question, where no implementer would look; §2 now states it, with
`typst-svg` 0.15.1's presence in `Cargo.lock` and what taking it would cost.
And the IPC boundary is now named: `tauri::ipc::Response` rather than a
returned `Vec<u8>`, which would serialize as a JSON array of numbers.

No unresolved open question blocks Phase 1. OQ-3, OQ-4 and OQ-5 block Phases 2
and 4 only.

On this convergence: `status: accepted`, and `reviewed: 2026-08-10` on Phase 1.
Phases 2 to 5 keep `reviewed: null` — each takes its own round before it is
built.

### Round 1 — Phase 1 only — 2026-08-10 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the drafted document): Phase 1 produces the
observable — the typeset PDF that Typst compiles from the user's markdown,
drawn on screen from a document the user picked — and §1 states that the
observable is unchanged from `mpdf-001` and only stops being a file opened by
hand. It is the right one, with a caveat recorded rather than waved past: this
is the first spec in the corpus whose value is convenience rather than
capability. The argument that it is still wanted is that `mpdf-001` §1
predicted this exact wrapper, the project's own notes name the desktop
workflow as the goal, and the loop it removes is one the author pays on every
edit. The episode proceeded.

The reviewer's grounding pass confirmed every repo citation against the code:
`cli/src/main.rs:read_assets` and `default_output` do what §2 says; the
`unsupported_html.md` rejection really does name line 5, run against the built
binary; both samples exist and name the images they name; and `core`'s public
surface already carries everything Phase 1 needs, which is §2's
falsifiable claim holding for this phase. It also confirmed the toolchain is
present — `cargo-tauri` 2.10.1, `tauri` 2.11.5 cached — and that `typst-svg`
0.15.1, OQ-1's stated fallback, is already in `Cargo.lock` transitively.

Verdict: `NOT READY` — two blocking findings, eight non-blocking. The author
accepted all ten, rejected none, deferred none.

Blocker 1: **OQ-2 was unresolved and the phase deferred its entire
construction to it** — "a Tauri window per OQ-2" is a scope that names zero
files in the crate it creates, against the methodology's §3. Blocker 2:
**OQ-1 was unresolved and decided both the phase's central mechanism and its
gate** — custom protocol, `blob:`, temp file and the SVG fallback are
materially different builds, and OQ-1 itself said a failed probe would amend
§2's recorded decision. The reviewer could not verify the WKWebView question
from the repo or from an authoritative source, and noted that the community
guidance it found leaned toward the one route §2 forbids.

Both were resolved in the round, by building rather than by reading. A
throwaway Tauri 2.11.5 app in a scratch directory answered them on macOS
26.5.2:

- **OQ-1 → a `blob:` URL in an iframe, and no bundled viewer.** From inside
  the webview: `navigator.pdfViewerEnabled` is `true`,
  `navigator.mimeTypes['application/pdf']` is present, and the frame fed a
  blob of the PDF bytes exposes a `contentDocument` whose `contentType` is
  `application/pdf`, whose location is `blob:tauri://localhost/…`, holding one
  `<embed>` — WebKit's own PDF document view. The same frame served over a
  custom `pdf://` scheme returns 200 with the right content type but exposes a
  `null` `contentDocument`, being a separate origin, which is what ruled that
  route out and what makes OQ-3 answerable at all.
- **OQ-2 → Tauri 2, seven files, no npm.** Four facts cost a build each and
  are recorded in the OQ so nobody pays twice: `icons/icon.png` is required or
  `generate_context!` panics at compile time; `withGlobalTauri: true` removes
  the bundler and the node toolchain entirely; the command boundary is
  `#[tauri::command]` with `generate_handler!`; and `tauri-plugin-dialog`
  needs `dialog:allow-open` in `capabilities/default.json`, confirmed by
  calling it and watching the native dialog open rather than reject.

The round's residual, recorded rather than hidden: the probe proves WebKit
instantiated its PDF view, not that the pixels are right. Nothing readable
from JavaScript proves that, and this machine denies the terminal Screen
Recording permission, so no screenshot could be taken. Phase 1's gate case (1)
now says explicitly that a person reads it at the window.

Non-blocking, all accepted: gate case (3) could not hold of one invocation on
`figure.md` and could not pin the dedup it named — `images.md` cannot either,
because `fig#2.png` on line 7 fails before the repeated `dot.png` on line 10 —
so the case became three, each with the directory it needs, and the dedup case
moved to an inline document; the "not a rewrite" quote was attributed to
`mpdf-001` §2 and lives in §1, in two places; the by-eye precedent cited that
spec's Phase 6, which faced the same problem and chose the *opposite* answer, a
textual assertion; one of the two timings did not reproduce; the error story
did not cover the plain `String` failures `read_assets` produces, which is the
class its own gate tests; the Open command did not say how the user picks a
file; and §1's mock-up depicted Phase 2 and Phase 3 chrome.

The timings were re-measured in the round, twenty runs each of the release
binary including the process spawn, medians: `samples/press-release.md`
8.5 ms, `samples/article.md` 28.7 ms. The drafted 42 ms came from a single
cold run and is not reproducible; §2 now states the method beside the numbers,
and §1's status readout — which showed a number Phase 3 owns — is gone.

Rejections: none.
