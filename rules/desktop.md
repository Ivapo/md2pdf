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
  the desktop app: the crate and its files, the window and its menu, the two
  commands and the signal between them, the blob frame that draws the artifact,
  the file I/O the app owns, the watch loop and the filter and debounce it runs
  on, the state the loop writes, the errors it puts on the page, and the
  configuration facts a build enforces
max_lines: 155
generated: 2026-08-10
---

# Desktop

A macOS window that shows the PDF while you write it. `md2pdf-app` is the second
wrapper around `md2pdf-core`, beside `md2pdf-cli`, and it calls no other code of
this project's own; `core` gained nothing for it. Today it opens one file at a
time, compiles it, and recompiles on every save. There is no export, no editing
and no status chrome: those are Phases 3 to 5 of `specs/desktop_app_spec.md`.

## The crate

Eight files and three modules. `Cargo.toml` names `tauri` 2.11.5,
`tauri-plugin-dialog` 2.7.2 and `notify` 8.2.0, with `tauri-build` 2.6.3 under
`[build-dependencies]`; `build.rs` calls `tauri_build::build()`.
`tauri.conf.json` sets `app.withGlobalTauri: true`, which puts the API on
`window.__TAURI__` and is what removes the bundler and the node toolchain
entirely — `build.frontendDist` is `dist`, a directory of static files, so the
whole app is one Cargo build. `capabilities/default.json` grants `core:default`
and `dialog:allow-open` to the window labelled `main`.

Two configuration facts cost a build each. **`icons/icon.png` is required** —
without it `generate_context!` panics with "failed to open icon" — and **it must
be RGBA**, or `tauri-codegen` panics "is not RGBA" instead. The file in the tree
is a generated placeholder; Phase 5 owns the real icon and the bundle.
`app.security.csp` is unset and defaults to `None`, so no policy blocks the
`blob:` frame below. `app/gen/` is generated and is not committed.

## The window

`app/src/main.rs` holds only what needs a window. `main` registers the dialog
plugin, builds the menu, manages one `Mutex<Session>`, and registers two
commands.

The menu is built by hand, because macOS draws none of its own: an app submenu,
a `File` submenu whose `Open…` item carries `CmdOrCtrl+O`, an `Edit` submenu,
and a `Window` submenu. **The menu item does not open the dialog.** It emits an
`open` event to the window, and the page opens the dialog, so the menu item and
the button in the page run one code path and not two. `core:default` already
carries `core:event:allow-listen`, so neither that event nor the `rendered`
signal below needs a capability entry of its own.

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
of numbers, one per byte. `current_pdf`'s `Err` is not a fault but a state: a
stale pane keeps its bytes and gets the message instead. Phase 3's export reads
the same flag, so the file it writes and the page on screen cannot disagree.

## The page

`app/dist/index.html` is the whole front end — one file, inline CSS and JS.

It draws the artifact, not a picture of it: the bytes go into a `Blob` of type
`application/pdf`, and an iframe's `src` is the object URL, so WebKit builds its
own PDF document view inside the frame. No JavaScript PDF viewer is bundled and
none is wanted. The route is same-origin — a blob URL inherits the page's
origin, where a custom URI scheme does not — and the bytes never touch the disk.
The previous object URL is released once the frame has left it.

**A re-render returns the reader to the first page.** WebKit's PDF view leaks
nothing about where the reader was, so there is nothing to restore; `#page=N` on
a fresh blob URL is honoured, so a position can be set but never learned.

A failure puts its message in a bar above the pane and dims the pane, and the
page stays. An author mid-edit passes through broken states constantly, and
blanking the pane on each one would lose their place. With no page under it the
message takes the whole pane instead. No word spells "stale": that chrome is
Phase 3's.

## The file I/O

`app/src/document.rs:render` reads the document once and hands that one string
to both the image list and the compile. It returns a `Render`, which carries the
image paths **even when the compile failed** — emission reads the text and not
the disk, so a document whose figures are all missing still names them, and that
is what keeps the watch filter alive while nothing compiles. `images` is `None`
only when the document did not parse, and the caller then keeps the list it had.

`app/src/document.rs:read_assets_with` mirrors `cli/src/main.rs:read_assets` —
resolve each path against the open document's directory, read each once, and
keep the path the markdown wrote. The duplication is deliberate: the two
wrappers report their errors differently, which is most of what those lines do.
Its read is a parameter, so a caller counting its own reads can check that a
path named twice is read once, and `app/src/document.rs:render_with` is the same
seam one level up. Both classes of failure reach the page in the words the
terminal uses: a `md2pdf_core::Error` through its `Display`, and a file that
will not read through the plain sentence `read_assets_with` builds.

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
and `/var` are both symlinks into `/private`, so that is the default case, and
comparing the paths as they arrive would leave the watcher running, every event
dropped, and the page never redrawn. The document's *directory* is what gets
resolved rather than each file, because a figure named before it exists has no
real path to resolve.

`app/src/watch.rs:DEBOUNCE` is 100 ms, and it is measured. Twenty saves under
each of three write strategies, counting the events naming the document:
medians 4 for a truncate-and-write, 2 for a write-then-rename and 2 for a
`RENAME_SWAP`, with the span from a save's first event to its last never above
0.03 ms — FSEvents hands one save's events over in a single batch, so the
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
last good PDF bytes, the image list the filter reads, a stale flag and the
error. `Preview::compile` replaces the bytes on success and clears both marks;
**on failure it keeps the bytes**, records the message and sets the flag.

`app/src/preview.rs:Session` is that state plus the watch. `Session::open`
points the preview at a document, clears the previous one's page, compiles once,
and only then swaps the watch — old dropped before new started, so the two never
both hold a directory. Its callback checks the document before writing, because
dropping a `Watch` does not join its thread and a thread mid-compile could
otherwise write its page over a newer one.

Neither type needs a window, and neither does anything in `document.rs` or
`watch.rs`: a GUI whose logic is reachable only by clicking has no exit gate but
a screenshot. Exactly one claim is still read by a person, which is whether the
right pixels reached the glass.
