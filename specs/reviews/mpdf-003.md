# Review record — mpdf-003 (`specs/desktop_app_spec.md`)

Append-only. One heading per round, newest first.

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
