# Review record — mpdf-003 (`specs/desktop_app_spec.md`)

Append-only. One heading per round, newest first.

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
