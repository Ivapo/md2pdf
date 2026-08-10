# Review record — mpdf-003 (`specs/desktop_app_spec.md`)

Append-only. One heading per round, newest first.

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
