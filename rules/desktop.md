---
title: desktop
sources:
  - app/Cargo.toml
  - app/tauri.conf.json
  - app/capabilities/default.json
  - app/src/main.rs
  - app/src/document.rs
  - app/src/preview.rs
  - app/src/watch.rs
  - app/dist/index.html
covers: >
  the desktop app: the crate and its files, the window and its menu, the
  commands and the signal between them, the file I/O the app owns and the two read passes one
  closure serves, the three answers the asset list gives and the filter reads,
  the watch loop and the filter and the two debounces it runs on, the buffer
  that compiles and the rule an external change runs, the state the loop writes
  and the four states it reports, the export and its two refusals, the errors it
  puts on the page, the bundle and the document association that launches it,
  the vendored renderer the front end imports and what embedding it costs, and
  the configuration facts a build enforces
max_lines: 500
generated: 2026-08-25
---

# Desktop

A macOS window that shows the PDF while you write it. `md2pdf-app` is the second
wrapper around `md2pdf-core`, beside `md2pdf-cli`, and it calls no other code of
this project's own; **`core` gained one function for it, in Phase 6, and nothing
in any other phase** — `core/src/lib.rs:md_to_pdf_with_anchors`, which is
additive and left every existing signature alone. Today it opens one *document*
at a time — one file, or a master and the sections it names — holds that
document's own text in a pane beside the page, recompiles when the typing stops,
opens the page on the heading the caret is under, says what state the page is
in, saves the text back, and writes the page to a file the user names. It
bundles into an `.app` and a `.dmg`, and a `.md` double-clicked in Finder
launches it on that document. **The bundle is
unsigned**, which is a credential this machine does not hold rather than a step
skipped; `specs/desktop_app_spec.md`'s OQ-8 carries what that costs.

**The pane's text is what compiles**, and several claims below turn on it. The
file beside it need never have held that text. **The pane holds exactly one
file** — the master, where there is one — and the sections come off the disk;
that is what the anchor rule below turns on.

## The crate

Thirteen committed files, three of them the vendored renderer and its licence;
`src/` is `main.rs` and three modules. `Cargo.toml` names `tauri` 2.11.5,
`tauri-plugin-dialog` 2.7.2, `notify` 8.2.0 and `serde` 1.0.229 with its `derive`
feature, with `tauri-build` 2.6.3 under `[build-dependencies]`; `build.rs` calls
`tauri_build::build()`. `tauri.conf.json` sets `app.withGlobalTauri: true`, which
puts the API on `window.__TAURI__` and is what removes the bundler and the node
toolchain entirely — `build.frontendDist` is `dist`, a directory of static files,
so the whole app is one Cargo build. **The vendored `pdf.js` does not spend
that**: `pdfjs-dist` ships browser-ready ES modules, so `app/dist/pdfjs/` is two
more static files the page imports and not a dependency anything builds.
`generate_context!` walks `frontendDist` recursively with no allowlist, so a new
subdirectory is embedded with no configuration at all, and `.mjs` is served as
`text/javascript`. `capabilities/default.json` grants
`core:default` to the window labelled `main`, plus **one entry per dialog** —
`dialog:allow-open` and `dialog:allow-save`.

Two configuration facts cost a build each. **`icons/icon.png` is required** —
without it `generate_context!` panics with "failed to open icon" — and **it must
be RGBA**, or `tauri-codegen` panics "is not RGBA" instead. The file in the tree
is a generated placeholder, and **no phase has designed artwork to replace it**;
the bundler synthesises the `.icns` from it, so the Dock upscales one 512×512
image. **`app.security.csp` is unset and defaults to `None`, so no policy is
served at all** — which is what lets the page import ES modules and start a
worker from `app/dist/pdfjs/` over the `tauri://localhost` custom scheme, with no
capability added for either. `app/gen/` is generated and is not committed.

## The window

`app/src/main.rs` holds only what needs a window. `main` registers the dialog
plugin, builds the menu, manages one `Mutex<Session>` and one
`app/src/main.rs:Pending`, registers nine commands, and — since the bundle —
**builds the app rather than running it**, so that it can hand `App::run` a
callback and see the run events a `.run(generate_context!())` never surfaces.

The menu is built by hand, because macOS draws none of its own: an app submenu,
a `File` submenu carrying `Open…` at `CmdOrCtrl+O`, `Save` at `CmdOrCtrl+S` and
`Save a Copy…` at `Shift+CmdOrCtrl+S`, an `Edit` submenu, and a `Window`
submenu. **No item acts on its own.** Each emits an event of its own id to the
window, and the page invokes, so a menu item and the button beside it run one
code path and not two — which is why `Save` emits too, though it opens no dialog
and so costs no capability. `core:default` already carries
`core:event:allow-listen`, so neither those events nor the `rendered` signal
below needs an entry either.

`app/src/main.rs:open_document` titles the window with the document's file name
before the compile, so the window names what the user opened whether or not it
compiled, then hands the path to the session. **It returns no bytes.** The
compile and the fetch are two calls, because the watch loop compiles with nobody
asking. The command is `async`, so the compile runs off the thread that draws
the window.

`app/src/main.rs:current_pdf` is the fetch, and `rendered` — a signal carrying
**no payload**, emitted after every compile — is what asks for it. The bytes
cross as a `tauri::ipc::Response`, which reaches the page as an `ArrayBuffer`; a
returned `Vec<u8>`, or an event carrying them, would serialize as a JSON array
of numbers, one per byte. Its `Err` is a state and not a fault: a stale pane
keeps its bytes and gets the message instead.

`app/src/main.rs:status` answers the same signal, and is **a second command
rather than a wider return**, because the bytes cross raw and a status does not.
`app/src/main.rs:document_text` answers it too, and only when the status says the
buffer was replaced from disk. `app/src/main.rs:export_path` and
`app/src/main.rs:export` are the two halves of Save a Copy.

`app/src/main.rs:edit` takes what the author typed and
`app/src/main.rs:save` writes it to the document's own path. **Keystrokes
therefore cross the IPC boundary and the debounce is Rust's**, which is what puts
the interval on the testable side of the window: a debounce in the page would be
logic reachable only by typing at one.

Eight of the nine commands are wrappers over a plain function.
`app/src/main.rs:pending_open` is the ninth and is not: it reads no session and
**takes** a slot the run event filled, which the section on the bundle below
explains.

## The file I/O

`app/src/document.rs:render` **takes the markdown as a parameter and not a path**,
and hands that one string to both the asset list and the compile. The read left it
for `app/src/document.rs:read_document`, one call further out, so that the string
the pane holds is what compiles; `md2pdf_core::md_to_pdf` already took a `&str`,
so `core` gained nothing for *that* change. It returns a `Render`, which carries the
asset paths **even when the compile failed** — emission reads the text and not
the disk, so a document whose figures are all missing still names them, and that
is what keeps the watch filter alive while nothing compiles.

**`Render::assets` has three answers, and the section paths go in first and
unconditionally**: `Some(sections ++ bibliography ++ images)` when the two walks
answer, `Some(sections)` when they do not and the master names any, `None`
otherwise — and on `None` the caller keeps the list it had, which is what stops a
transient out-of-dialect edit from dropping the images the app knows about. The
first branch is what this returned before sections existed, in the same order,
with an empty list in front of it. **The middle branch is why the sections are
unconditional**: `section_paths` cannot fail where both shopping lists now fail
with `Error::MissingSection` for a section that does not exist yet, and
`Preview::compile` replaces the list only when it is `Some` — so without it the
list would stay empty, `classify` would drop the section's creation event, and
the window would never recover. It *replaces* the list with a shorter one, so
such a document stops watching its figures until the section returns: a
deliberate trade, since recovering the section beats watching figures through a
window in which nothing compiles anyway.

**A file the document names makes two journeys through this app, and they are
independent.** The bytes travel `read_sections_with` → `read_assets_with` →
`md_to_pdf_with_anchors` and reach nothing else; the paths are built separately
in `app/src/document.rs:render_with`, which calls all three exports for itself,
and travel `Render::assets` → the watch filter. **The bytes are what make the
document compile; the paths are what make a change to one of them redraw.** The
image and bibliography exports come off one walk, so they answer or fail
together, and the paths arrive even when a read failed — which is what an asset
named before it exists depends on.

The compile itself is `md2pdf_core::md_to_pdf_with_anchors`, and `Render` carries
its `anchors` beside the bytes. **They go the other way from `assets`**: a failed
compile has none, because they describe the *page* where the asset list describes
the text. `app/src/document.rs:Anchor` is `md2pdf_core::Anchor` again, and the
duplication is the one `read_assets_with` already makes — this copy crosses to
the page inside `Status`, so it must serialize, and `core` carries no serde. It
is also a line and a page where `core`'s carries a whole `md2pdf_core::Location`,
because **`render_with` keeps only the anchors whose location names no file** and
drops every one written in a section; §"The page" argues why.

**The reading is two passes and it mirrors the CLI's.**
`app/src/document.rs:read_sections_with` runs first, as
`cli/src/main.rs:read_sections` does and for the reason that function records:
the markers are in the master's own text, so the sections can be read with no
join, where neither shopping list can answer about a document that has not been
assembled. `app/src/document.rs:read_assets_with` then mirrors
`cli/src/main.rs:read_assets` — it takes the sections, seeds `seen` with their
paths, carries them out on the same array, resolves each remaining path against
`app/src/document.rs:directory`, reads each once, and keeps the path the markdown
wrote. **The bibliography is read first of the two remaining channels**, because
the line it names is the frontmatter's and is therefore earlier than every
image's. A section's own images are found beside it with nothing added here:
`core` wrote the folder into the destination before the list arrived, so an image
drawn in `sections/method.md` reaches this as `sections/figure.png` and still
joins the master's directory. The duplication with the CLI is deliberate: the two
wrappers report their errors differently — and **the app owes the terminal three
hand-built sentences**, one per channel, because a file that will not read is no
`md2pdf_core::Error` at all.

**One closure serves both passes**, borrowed by the first and handed to the
second in `app/src/document.rs:render_with`. The read is a parameter so a caller
counting its own reads can check that a path named twice is read once, and a
second closure would leave half of them outside that counter. Both classes of
failure reach the page in the terminal's words: an `Error` through its `Display`,
and a file that will not read through the sentence this builds.

`app/src/document.rs:default_output` is where an export lands unless the user
says otherwise: the document's path with a `.pdf` extension. It duplicates
`cli/src/main.rs:default_output`, because sharing it would make one crate's
binary reachable from the other.

## The watch loop

`app/src/watch.rs:root` is the whole watch set: **the document's own directory,
watched recursively**. `core/src/emit.rs:check_image` refuses a URI scheme, a
leading `/`, a `..` segment and a backslash, and `core/src/frontmatter.rs` puts
the bibliography under that same rule, so every path a document can legally name
resolves under there — one watch covers the document, every asset it names, every
asset it will name, and every directory not yet created. It is computable from
the document's path alone, so a document the dialect refuses is
watched too. The limit: an asset that is a symlink out of the directory is not
watched, because a watch follows the tree and not the targets.

`app/src/watch.rs:classify` is the filter, and the one list `section_paths`,
`image_paths` and `bibliography_path` fill is what it filters against — the list
is not the set, and it follows the buffer, because the buffer is the document
now. **A section changes nothing about the watch set**: `root` is already
recursive and `sections/` sits under it, so a section is one more string in that
one list, and it arrives as `Change::Asset` and never `Change::Document` —
`Preview::reload`'s rule is about the buffer the pane holds, which is never a
section. **It sorts rather
than admits**: an event is `Change::Document` or `Change::Asset` or neither,
because the two no longer mean the same thing. **A bibliography is one more string in that one list**
rather than an arm of its own: the split is the open document against everything
the disk supplies, and a bibliography sits on the second side for the reason a
figure does. It also makes a *dropped* `bibliography:` key free, since
`Preview::compile` replaces the whole list on every compile. The document **stays** in the filter though its events no
longer mean "compile" — a path dropped from it would never reach the rule that
decides what its events do mean. **Both sides are
canonicalized**, and the loop depends on it: `notify` canonicalizes a path as it
registers it, because FSEvents reports the resolved path, so an event names
`/private/var/…` where the Open dialog handed the app `/var/…`. On macOS `/tmp`
and `/var` are both symlinks into `/private`, so comparing the paths as they
arrive would leave the watcher running, every event dropped, and the page never
redrawn. The document's *directory* is resolved rather than each file, because a
figure named before it exists has no real path to resolve.

**There are two intervals, and each is measured against a different thing.**
`app/src/watch.rs:DEBOUNCE` is 100 ms, margin over FSEvents' own batching —
twenty saves under each of three write strategies, tabulated in its own doc
comment. `app/src/watch.rs:TYPING_DEBOUNCE` is 300 ms, and it is not protecting
the compile: twenty compiles of each sample through the pane's path put the warm
median at 1.5 ms and 0.6 ms, against 24.6 ms and 12.5 ms for the first compile of
a process, which the app pays at the open. It is set to the pause between phrases
rather than the gap between keystrokes, because a redraw moves the reader —
narrowed but not removed by the anchor, which returns them to the top of the
section they are in. `app/src/watch.rs:Debounce` takes the time as a parameter, so both tests
need no clock and cannot flake.

`app/src/watch.rs:settle` is the thread both intervals run on: it folds a stream
into one call per quiet interval, accumulating what arrived. `app/src/watch.rs:start`
registers the watch and settles `notify` events into a `Changed` — which of the
document and the assets moved, since one window can hold both.
`app/src/watch.rs:debounced` settles bare nudges, and is what the keyboard uses.

**Dropping the `Watch` stops the loop**: it unregisters the directory and drops
the sender its handler holds, which disconnects the channel, which ends the
thread. Dropping the typing channel ends its thread the same way. That is the
whole mechanism by which opening a second document moves both.

## The state

`app/src/preview.rs:Preview` is what the loop writes and the pane shows: **the
text the pane holds and the text as it stood at the last open or save**, the last
good PDF bytes, how long they took, the asset list the filter reads, two counters,
a stale flag, the error and the divergence. The two strings are what
`app/src/preview.rs:external_change` compares, and holding them here rather than
in the page is what keeps that rule testable at all.

`Preview::compile` compiles the **buffer**, not the file. `Preview::load` is the
only thing that reads the document into it, and `Preview::edit` takes what the
author typed without compiling, because one keystroke is not a document.
`Preview::save` writes the buffer to the document's path and moves the last-saved
text with it — which is what makes that save's own event mean nothing a moment
later.

`Preview::compile` replaces the bytes on success and
clears both marks; **on failure it keeps the bytes**, records the message and
sets the flag. **The duration travels with the bytes**, replaced and kept
exactly as they are, so the time the window shows describes the page on screen
rather than the last attempt at one.

`app/src/preview.rs:State` is the four the window reports: *empty*, the state the
app launches into; *current*, a compile that succeeded; *stale*, one that failed
over a page still drawn; and *failed*, one that failed with no page to keep.
**What separates *stale* from *failed* is whether there are bytes**, not any
branch in `current_pdf` — `compile` sets the flag on every failure, so both take
that command's first branch. **Empty is exactly "no document has been opened"**:
`compile` returns early with no document and `Session::open` sets the document
and compiles inside one lock scope, so nothing observable sits between
`Preview::default()` and the first outcome. The serialized name is lowercase, and
the page uses it as a word and as a class.

`app/src/preview.rs:Status` is that state, the compile time worded as `"28 ms"`,
the error, whether a page is drawn, the divergence, two counters and the anchors —
one value the page places rather than composes. **`revision` counts compiles that
produced bytes** and `reloaded` counts the times the buffer was replaced from
disk; both exist so the page can tell a signal apart from work it has already
taken. **The anchors ride the status because the status is already fetched on the
path that draws**, so following the caret needs no command of its own. Like the
compile time, they are replaced on a success and kept on a failure, so they always
describe the page on screen rather than the last attempt at one.

## The rule an external change runs

`app/src/preview.rs:external_change` is three strings and two comparisons, and it
needs no dirty flag. The file equal to the buffer is `Unchanged` — the app's own
save arriving back, and **nothing happens**. The file differing under a clean
buffer is `Taken` and recompiled, which is the loop the app has shipped since
Phase 2 and the case an unconditional refusal would have broken. The file
differing under a dirty one is `Diverged`: the work is kept and the divergence is
named. **The app does not merge**, and it makes neither choice for the author —
saving overwrites the disk, reopening takes it.

`Preview::reload` reads the file and carries that answer out; a document that
will not read at that instant counts as `Unchanged`. **A divergence is not
staleness**: nothing failed to compile, and the page belongs to the text in the
pane.

Two limits it accepts. An author typing between a save and that save's event
lands in `Diverged`, so the app can name a divergence that was really its own
write — it loses nothing, and the next save clears it. And an external writer
that writes exactly the author's unsaved text takes `Unchanged`, leaving the
last-saved text unrefreshed. Both err toward keeping work. **Nothing suppresses a
self-write**: the first outcome *is* the self-write case, decided by comparing
content rather than by winning a race against an event that arrives 12 ms after
the write.

## The session

`app/src/preview.rs:Session` is that state plus the two loops. `Session::open`
points the preview at a document, clears the previous one's page and text, reads
and compiles once, and only then swaps the loops — old dropped before new
started, so no two of them ever hold a document. Both callbacks check the
document before writing, because dropping a `Watch` or a typing channel does not
join its thread and a thread mid-compile could otherwise write its page over a
newer one.

`Session::on_change` is where the document's events and the assets' events reach
different code: an asset is a bare recompile, and the document runs the rule. A
window that took the disk copy compiled inside the rule and read the new assets
on the way, so one window never compiles twice, and **nothing is announced when
nothing happened**.

## The export

`app/src/preview.rs:Preview::export` writes the page's own bytes where the user
asked. **Nothing in it compiles**: the file is what the pane is already showing,
which is what keeps the two from disagreeing.

`Preview::exportable` is the one refusal rule, and **it words two sentences
rather than one**, because an *empty* pane holding no bytes and a *stale* or
*failed* one holding bytes known to be old are two problems. `export_path` runs
it before returning the default path, so a pane that cannot be exported never
opens a dialog whose answer it would throw away.

**The file it writes is the file `md2pdf` writes**, byte for byte, for the same
document — **while the pane and the file say the same thing**. The PDF is a pure
function of the text and the asset bytes, and the text the app compiles is the
pane's; a buffer with unsaved edits is a different document from the one on disk,
and the two front ends then agree about nothing except their own inputs. No
single test holds even the qualified claim: `CARGO_BIN_EXE_md2pdf` reaches only
integration tests of the package defining that binary, and `md2pdf-app` declares
a `[[bin]]` and no `[lib]`. It is gated in two halves, in `app/src/preview.rs`
and `cli/tests/cli_test.rs`, meeting at an in-test `md_to_pdf` call.

The limit the flag accepts: **"stale" answers "did the last compile fail", not
"does the page match the file on disk"**. It never answered the second question,
and the window in which the two can differ is now unbounded rather than the
debounce plus the compile: an author who types and does not save leaves the file
behind the page for as long as they like, and the pane reads *current* throughout,
because it is current — for the text in the pane.

Nothing in `preview.rs`, `document.rs` or `watch.rs` needs a window: a GUI whose
logic is reachable only by clicking has no exit gate but a screenshot. Two claims
are read by a person: whether the right pixels reached the glass, and whether the
bundle below runs at all away from `cargo`.

## The bundle

`cargo tauri build` is the one command, and **`bundle.active` is what it turns
on** — while that key is false the two obvious commands disagree, `cargo tauri
build` skipping the bundle and the standalone `cargo tauri bundle` making one
anyway. `bundle.targets` is `app` and `dmg`, leaving
`target/release/bundle/macos/md2pdf.app` and a `.dmg` beside it named for the
version and the architecture. **`productName` renames the `.app` and not what is
inside it**: the bundle is `md2pdf.app` and `CFBundleExecutable` is
`md2pdf-app`. `tauri-cli` pins at 2.10.1, as everything here pins.

`Contents/Resources/` holds `md2pdf.icns` and nothing else, and **no font ships
there**: `core/src/lib.rs` embeds all five faces with `include_bytes!` and the
Typst world exposes those alone, so one added there would be dead weight.

**The bundle is unsigned**, measured rather than assumed: `codesign -dv` reports
`flags=0x20002(adhoc,linker-signed)` with `Sealed Resources=none`, and `spctl -a
-t exec` rejects it. How it travels therefore decides whether it launches
elsewhere — a copy over USB or `scp` sets no `com.apple.quarantine` attribute and
runs, a download or an AirDrop sets one and Gatekeeper refuses until a person
overrides by hand. `bundle.macOS.signingIdentity` and `hardenedRuntime` are where
the fix goes, and both need an Apple Developer credential.

**A bundle gets its own privacy identity, and the watch loop depends on one.**
Under `cargo tauri dev` the process inherits the terminal's grants; as
`dev.md2pdf.desktop` it does not, and `watch.rs:start` watches a whole directory
recursively. A document under `~/Documents`, `~/Desktop` or `~/Downloads` can
compile once through the open panel and then stop redrawing — the silent-failure
class the canonicalization note above records, by another route.

## The document association

`bundle.fileAssociations` emits one `CFBundleDocumentTypes` entry: `ext` becomes
`CFBundleTypeExtensions`, `name` `CFBundleTypeName`, `role` `CFBundleTypeRole`,
`rank` `LSHandlerRank`, and `contentTypes` `LSItemContentTypes` — the last only
when asked for, carrying `net.daringfireball.markdown` because modern
LaunchServices prefers a UTI. **Spell it `contentTypes`, not `content-types`**:
`tauri-utils` declares that alias, but the CLI validates against the generated
JSON Schema before serde runs, an alias does not appear in a schema, and
`deny_unknown_fields` becomes `additionalProperties: false` there. The rank is
`Default`, so **a machine with an editor already registered for `.md` keeps that
editor** — the ranking working, not a broken association.

**The association only launches the app; it hands the process nothing**, and
nothing here reads `std::env::args`. The path arrives as
`tauri::RunEvent::Opened`, which is why `main` builds and then runs. Its URLs are
`file://`, and the path comes from `Url::to_file_path` and **not** `Url::path`,
which leaves a space percent-encoded — `my doc.md` would arrive as `my%20doc.md`
and open as nothing; `tauri` re-exports `Url`, so this costs no dependency.
`urls` is a `Vec`, because Finder delivers a multiple selection as one event, and
the app takes the first. A cold launch survives by ordering rather than by
queueing: tao's `AppState::open_urls` **drops** an event when no callback is set,
and tao installs the callback before `NSApp.run()`.

**The open goes through the page rather than around it**, and `clear()` is why.
`Session::open` rebuilds from `Preview::default()`, so `revision` and `reloaded`
restart at 0 per document, while the page resets `drawnRevision` and
`takenReload` only inside `clear()` — which the dialog path calls and a path
straight into Rust never would. A second document opened that way would return
Rust to `revision 1, reloaded 1`, which the page already holds, and both panes
would keep the old document under a new title.

So the run event **stores and emits**, in the menu items' own idiom: the path
goes into `Pending`, a managed slot, and a payload-less `opened` signal goes to
the window. The page takes that slot through `pending_open` at startup and again
on every signal, and **the take clears it**, so a cold open is collected by the
first, a warm one by the second, and whichever runs second finds nothing. The
page then calls `clear()` and invokes `open_document` as the dialog does.
`core:event:allow-listen` already covers the signal, and `open_document` keeps
its signature and its `async`.

The limit that accepts: `listen` completes over IPC, so a document landing
between the startup take and the listener's registration would sit in the slot
until the next signal. `app/dist/index.html` registers its five `listen` calls
before it calls `refresh()` and `takePendingOpen()`, which makes that window
practically unreachable, and the cost if it is reached is a document that opens
late rather than one that opens wrong.
