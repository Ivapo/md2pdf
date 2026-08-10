---
title: desktop
sources:
  - app/Cargo.toml
  - app/tauri.conf.json
  - app/capabilities/default.json
  - app/src/main.rs
  - app/src/document.rs
  - app/dist/index.html
covers: >
  the desktop app: the crate and its files, the window and its menu, the command
  boundary and why the PDF bytes cross it raw, the blob frame that draws the
  artifact, the file I/O the app owns, the errors it puts on the page, and the
  configuration facts a build enforces
max_lines: 80
generated: 2026-08-10
---

# Desktop

A macOS window that shows the PDF. `md2pdf-app` is the second wrapper around
`md2pdf-core`, beside `md2pdf-cli`, and it calls no other code of this project's
own; `core` gained nothing for it. Today it opens one file at a time and
compiles it once. There is no watching, no export, no editing and no status
chrome: those are Phases 2 to 5 of `specs/desktop_app_spec.md`.

## The crate

Eight files. `Cargo.toml` names `tauri` 2.11.5 and `tauri-plugin-dialog` 2.7.2,
with `tauri-build` 2.6.3 under `[build-dependencies]`; `build.rs` calls
`tauri_build::build()`. `tauri.conf.json` sets `app.withGlobalTauri: true`,
which puts the API on `window.__TAURI__` and is what removes the bundler and the
node toolchain entirely — `build.frontendDist` is `dist`, a directory of static
files, so the whole app is one Cargo build. `capabilities/default.json` grants
`core:default` and `dialog:allow-open` to the window labelled `main`.

Two configuration facts cost a build each. **`icons/icon.png` is required** —
without it `generate_context!` panics with "failed to open icon" — and **it must
be RGBA**, or `tauri-codegen` panics "is not RGBA" instead. The file in the tree
is a generated placeholder; Phase 5 owns the real icon and the bundle.
`app.security.csp` is unset and defaults to `None`, so no policy blocks the
`blob:` frame below. `app/gen/` is generated and is not committed.

## The window

`app/src/main.rs` holds only what needs a window. `main` registers the dialog
plugin, builds the menu, and registers one command.

The menu is built by hand, because macOS draws none of its own: an app submenu,
a `File` submenu whose `Open…` item carries `CmdOrCtrl+O`, an `Edit` submenu,
and a `Window` submenu. **The menu item does not open the dialog.** It emits an
`open` event to the window, and the page opens the dialog, so the menu item and
the button in the page run one code path and not two. `core:default` already
carries `core:event:allow-listen`, so that event needs no capability entry of
its own.

`app/src/main.rs:open_document` is the one command. It titles the window with the
document's file name before the compile, so the window names what the user
opened whether or not it compiled, then returns the bytes as a
`tauri::ipc::Response`, which reaches the page as an `ArrayBuffer`. A returned
`Vec<u8>` would serialize as a JSON array of numbers instead, one per byte. The
command is `async`, so the compile runs off the thread that draws the window.

## The page

`app/dist/index.html` is the whole front end — one file, inline CSS and JS.

It draws the artifact, not a picture of it: the bytes go into a `Blob` of type
`application/pdf`, and an iframe's `src` is the object URL, so WebKit builds its
own PDF document view inside the frame. No JavaScript PDF viewer is bundled and
none is wanted. The route is same-origin — a blob URL inherits the page's
origin, where a custom URI scheme does not — which is what will let a later
phase reach into the frame for the scroll offset, and the bytes never touch the
disk. The previous object URL is released once the frame has left it.

A failure puts its message on the page and clears the frame. Nothing keeps a
previous page here, because this phase has none to keep.

## The file I/O

`app/src/document.rs` holds everything decidable without a window, in plain
functions with plain tests, because a GUI whose logic is reachable only by
clicking has no exit gate but a screenshot. One claim is still read by a person:
whether the right pixels reached the glass.

`app/src/document.rs:read_assets` mirrors `cli/src/main.rs:read_assets` —
resolve each path `md2pdf_core::image_paths` returns against the open document's
directory, read each once, and keep the path the markdown wrote. The duplication
is deliberate: the two wrappers report their errors differently, which is most
of what those lines do. `app/src/document.rs:read_assets_with` takes the read as
a parameter, and exists for one gate: that a path named twice is read once,
which a caller counting its own reads can check rather than argue.

`app/src/document.rs:render` reads, collects and compiles. Both classes reach
the page in the words the terminal uses: a `md2pdf_core::Error` through its
`Display`, and a file that will not read through the plain sentence
`read_assets` builds, naming the path and the line.
