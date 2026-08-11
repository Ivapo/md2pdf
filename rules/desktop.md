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
  commands and the signal between them, the blob frame that draws the artifact,
  the file I/O the app owns, the watch loop and the filter and debounce it runs
  on, the state the loop writes and the four states it reports, the export and
  its two refusals, the errors it puts on the page, and the configuration facts
  a build enforces
max_lines: 210
generated: 2026-08-10
---

# Desktop

A macOS window that shows the PDF while you write it. `md2pdf-app` is the second
wrapper around `md2pdf-core`, beside `md2pdf-cli`, and it calls no other code of
this project's own; `core` gained nothing for it. Today it opens one file at a
time, compiles it, recompiles on every save, says what state the page is in, and
writes that page to a file the user names. There is no editing and no installable
bundle: those are Phases 4 and 5 of `specs/desktop_app_spec.md`.

## The crate

Eight files and three modules. `Cargo.toml` names `tauri` 2.11.5,
`tauri-plugin-dialog` 2.7.2, `notify` 8.2.0 and `serde` 1.0.229 with its `derive`
feature, with `tauri-build` 2.6.3 under `[build-dependencies]`; `build.rs` calls
`tauri_build::build()`. `tauri.conf.json` sets `app.withGlobalTauri: true`, which
puts the API on `window.__TAURI__` and is what removes the bundler and the node
toolchain entirely — `build.frontendDist` is `dist`, a directory of static files,
so the whole app is one Cargo build. `capabilities/default.json` grants
`core:default` to the window labelled `main`, plus **one entry per dialog** —
`dialog:allow-open` and `dialog:allow-save`.

Two configuration facts cost a build each. **`icons/icon.png` is required** —
without it `generate_context!` panics with "failed to open icon" — and **it must
be RGBA**, or `tauri-codegen` panics "is not RGBA" instead. The file in the tree
is a generated placeholder; Phase 5 owns the real icon and the bundle.
`app.security.csp` is unset and defaults to `None`, so no policy blocks the
`blob:` frame below. `app/gen/` is generated and is not committed.

## The window

`app/src/main.rs` holds only what needs a window. `main` registers the dialog
plugin, builds the menu, manages one `Mutex<Session>`, and registers five
commands.

The menu is built by hand, because macOS draws none of its own: an app submenu,
a `File` submenu carrying `Open…` at `CmdOrCtrl+O` and `Save a Copy…` at
`Shift+CmdOrCtrl+S` — not `Cmd+S`, which Phase 4's text pane will want for the
document — an `Edit` submenu, and a `Window` submenu. **Neither item opens its
dialog.** Each emits an event of its own id to the window, and the page opens
the dialog, so a menu item and the button beside it run one code path and not
two. `core:default` already carries `core:event:allow-listen`, so neither event
nor the `rendered` signal below needs a capability entry.

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
`app/src/main.rs:export_path` and `app/src/main.rs:export` are the two halves of
Save a Copy. Each of the four is a wrapper over a plain function.

## The page

`app/dist/index.html` is the whole front end — one file, inline CSS and JS.

It draws the artifact, not a picture of it: the bytes go into a `Blob` of type
`application/pdf`, and an iframe's `src` is the object URL, so WebKit builds its
own PDF document view inside the frame. No JavaScript PDF viewer is bundled and
none is wanted. The route is same-origin — a blob URL inherits the page's origin,
where a custom URI scheme does not — and the bytes never touch the disk. The
previous object URL is released once the frame has left it.

**A re-render returns the reader to the first page.** WebKit's PDF view leaks
nothing about where the reader was, so there is nothing to restore; `#page=N` on
a fresh blob URL is honoured, so a position can be set but never learned.

`refresh` asks for the status first and for the bytes only when the state is
`current`, so the page never draws a frame it has just been told is out of date.
`report` places that status: the line in the header, the message in a bar above
the pane, and the dimming a stale page wears — with no page under it the message
takes the whole pane instead. **Every word it places was chosen in Rust**, so
the page composes none of it and the four states are checked by tests rather
than by eye. The page survives a failure, because an author mid-edit passes
through broken states constantly and blanking the pane would lose their place.
`fail` is for the refusals that are not a compile status — an open that will not
read, an export the pane cannot serve — and the next status replaces what it
wrote.

## The file I/O

`app/src/document.rs:render` reads the document once and hands that one string
to both the image list and the compile. It returns a `Render`, which carries the
image paths **even when the compile failed** — emission reads the text and not
the disk, so a document whose figures are all missing still names them, and that
is what keeps the watch filter alive while nothing compiles. `images` is `None`
only when the document did not parse, and the caller then keeps the list it had.

`app/src/document.rs:read_assets_with` mirrors `cli/src/main.rs:read_assets` —
resolve each path against the open document's directory, read each once, and
keep the path the markdown wrote. The duplication is deliberate: the two wrappers
report their errors differently. Its read is a parameter, so a caller counting
its own reads can check that a path named twice is read once, and
`app/src/document.rs:render_with` is the same seam one level up. Both classes of
failure reach the page in the terminal's words: a `md2pdf_core::Error` through
its `Display`, and a file that will not read through the sentence this builds.

`app/src/document.rs:default_output` is where an export lands unless the user
says otherwise: the document's path with a `.pdf` extension. It duplicates
`cli/src/main.rs:default_output`, because sharing it would make one crate's
binary reachable from the other.

## The watch loop

`app/src/watch.rs:root` is the whole watch set: **the document's own directory,
watched recursively**. `core/src/emit.rs:check_image` refuses a URI scheme, a
leading `/`, a `..` segment and a backslash, so every path a document can
legally name resolves under there — one watch covers the document, every figure
it names, every figure it will name, and every directory not yet created. It is
computable from the document's path alone, so a document the dialect refuses is
watched too. The limit: a figure that is a symlink out of the directory is not
watched, because a watch follows the tree and not the targets.

`app/src/watch.rs:is_relevant` is the filter, and the list `image_paths` returns
is what it filters against — the list is not the set. **Both sides are
canonicalized**, and the loop depends on it: `notify` canonicalizes a path as it
registers it, because FSEvents reports the resolved path, so an event names
`/private/var/…` where the Open dialog handed the app `/var/…`. On macOS `/tmp`
and `/var` are both symlinks into `/private`, so comparing the paths as they
arrive would leave the watcher running, every event dropped, and the page never
redrawn. The document's *directory* is resolved rather than each file, because a
figure named before it exists has no real path to resolve.

`app/src/watch.rs:DEBOUNCE` is 100 ms, and it is measured rather than guessed —
twenty saves under each of three write strategies, tabulated in its own doc
comment. FSEvents hands one save's events over in a single batch, so the
interval is margin against a slower writer rather than a spread to cover.
`app/src/watch.rs:Debounce` takes the time as a parameter, so its test needs no
clock and cannot flake.

`app/src/watch.rs:start` registers the watch and spawns the thread that filters,
debounces and calls back. **Dropping the `Watch` stops the loop**: it
unregisters the directory and drops the sender its handler holds, which
disconnects the channel, which ends the thread. That is the whole mechanism by
which opening a second document moves the watch.

## The state

`app/src/preview.rs:Preview` is what the loop writes and the pane shows: the
last good PDF bytes, how long they took, the image list the filter reads, a
stale flag and the error. `Preview::compile` replaces the bytes on success and
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
the error, and whether a page is drawn — one value the page places rather than
composes.

`app/src/preview.rs:Session` is that state plus the watch. `Session::open`
points the preview at a document, clears the previous one's page, compiles once,
and only then swaps the watch — old dropped before new started, so the two never
both hold a directory. Its callback checks the document before writing, because
dropping a `Watch` does not join its thread and a thread mid-compile could
otherwise write its page over a newer one.

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
document. No single test holds that: `CARGO_BIN_EXE_md2pdf` reaches only
integration tests of the package defining that binary, and `md2pdf-app` declares
a `[[bin]]` and no `[lib]`. It is gated in two halves, in `app/src/preview.rs`
and `cli/tests/cli_test.rs`, meeting at an in-test `md_to_pdf` call.

The limit the flag accepts: **"stale" answers "did the last compile fail", not
"does the page match the file on disk"**. For the debounce plus the compile, a
saved document reads *current* while its bytes belong to older text, and an
export in that window writes those older bytes.

Nothing in `preview.rs`, `document.rs` or `watch.rs` needs a window: a GUI whose
logic is reachable only by clicking has no exit gate but a screenshot. Exactly
one claim is still read by a person, which is whether the right pixels reached
the glass.
