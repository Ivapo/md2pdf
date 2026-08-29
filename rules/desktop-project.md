---
title: desktop-project
sources:
  - app/src/document.rs
  - app/src/preview.rs
covers: >
  the project the desktop app opens: the root the opened file climbs one level
  to and the cap that is chosen rather than derived, the two edges that answer
  as no candidate, the masters discovered in the root and the reason that search
  does not recurse, the four ways one of them becomes the file that compiles,
  the file the pane holds beside it and the switch that moves it, the two
  refusals that switch shares with the one that sets the main and the one exit
  they both name, the walk and the merge and why they
  are two functions rather than one, the filter that is each channel's own
  comparison, the total order the panel cannot reorder itself against, the
  confinement the walk and the commands now share, the one a write obeys
  differently and the third a delete asks, the two further questions a create
  asks and the empty file it stops at, the row a delete acts on by name and the
  panel it refreshes itself, and the one
  fact this app remembers about a folder and where it refuses to keep it
max_lines: 150
generated: 2026-08-28
---

# The desktop app's project

What the window opens when the author picks one file: the folder it belongs to,
the file under that folder which compiles, and the list of everything in it.
`rules/desktop.md` has the crate, the commands, the watch and the bundle;
`rules/desktop-panes.md` has the panel this feeds.

**The opened file's parent is where the search starts, not where it stops.**
`app/src/document.rs:project_root` reads every `.md` directly in that parent's
own parent and asks `md2pdf_core::section_paths` whether any names the opened
file; one that does makes the grandparent the root. **One level is a cap, chosen
rather than derived** — climbing further means reading markdown above the project
to guess where the project is, and this app has never opened a file the author
did not name or a document did not name. The cost it accepts, asserted in its own
test rather than left as a defect: a section two directories below its master
roots below the master, recoverable only by opening the master. A parent with no
parent and a grandparent that will not `read_dir` are both *no candidate*, so the
root is `watch::root`'s answer unchanged, which is every single-file document.

`app/src/document.rs:masters` is every `.md` **directly in** the root whose text
names a section, and `discover_main` resolves them: one is main whatever the
author opened, none makes the opened file main, several take the opened file when
it is one of them and the byte-wise first when it is not. **It does not recurse,
and that is a property rather than a preference**: `emit::landed_path` refuses a
marker climbing out of the document's folder, so a section always sits at or
below its master and a master is never below its own sections — the climb answers
"above", this answers "at", and below is another project's. Recursion got it
wrong in the window, which is what `samples/` is in the suite to catch: a
single-file document sits there beside the whole `showcase/` project, and
`showcase/showcase.md` was taken for the one master of `samples/`. **It never leaves the main
unset** — an empty pane is a worse answer than a guess the panel marks and the
author corrects in one action, and alphabetical claims that the same folder opens
the same way twice rather than claiming which is right.

**The file that compiles and the file that is edited are two.** `Preview::main`
is what `document::render_project` compiles — its text, its directory, every path
the document names resolving against it — and `Preview::edited` is what the pane
holds, what `⌘S` writes and what the buffer stands in for inside that compile.
They are equal at every open, because an open still puts the main in the pane,
and free to differ from the first click on a row. `Session::set_edited` is the
switch and **is deliberately not an open**: `open_at` assigns
`Preview { ..Preview::default() }` and zeroes `revision` and `reloaded`, where
`app/dist/index.html`'s `clear()` — which resets the counters the page compares
them against — runs on an Open and not on a click, so a switch built to the open's
shape would strand `refresh` at its own guard and draw nothing. It sets `edited`,
loads that file, re-arms both loops through `Session::arm`, and leaves the root,
the main, the listing, the bytes and both counters where it found them: they
advance, they do not restart.

**Both commands that move a file refuse over unsaved work, and they name one
exit between them.** `set_edited` and `set_main` each confine the path first, then
refuse while the buffer diverges from the last-saved text — silently discarding it
was tolerable behind a menu item and a native dialog and is not one click from
every row in a panel. The sentence is `SWITCHING`'s and not `DIVERGED`'s, which
opens *"this file changed on disk"* and would be false here, and it rides
`Preview::divergence` rather than an `Err` so one refusal does not arrive in the
window two ways. `Session::discard` is the exit both name, and is `Preview::load`
behind a command: it drops the buffer, takes the file again, and clears the
divergence on the way, which answers a refused switch and a refused external
change alike.

`app/src/document.rs:files_under` walks the root and `merge` adds what the master
names and the disk lacks. **Two functions because the app runs them at two
rates**: the walk costs a `read_dir` per directory and happens at an open and a
`watch::Change::Tree` event, where the merge is pure and runs on every status.
The filter is each channel's own comparison rather than one invented here —
`eq_ignore_ascii_case("md")` as `emit::lone_markdown_link` reads a marker,
`bib`/`yml`/`yaml` folded down as `core/src/bibliography.rs` reads one, and
`md2pdf_core::IMAGE_EXTENSIONS` case-sensitively as `emit::check_image` does. The
order is total and computed here — files byte-wise, then subdirectories byte-wise,
each expanded where it sits — so the panel cannot reorder itself between two
compiles of one tree. **The walk obeys the confinement rule**: a link resolving
outside the canonical root contributes nothing and is not descended into, and a
directory already visited is not visited twice.

**The walk and the three commands ask one question, and that is a correction.**
`app/src/document.rs:confined` is the whole rule — resolve the path, require its
target under the resolved root — and `Session::set_main`, `Session::set_edited`
and `document::asset_bytes` all go through it. They used to ask something
stricter, that `document::relative` answer the same root-relative spelling back,
and stricter was wrong in one direction: `descend` lists a link under its **own**
name, so a `cover.jpg` pointing at `figures/cover.jpg` *inside* the project was a
row the panel offered and every command refused, while the compile rendered it.
The path `confined` answers with is the join and not the resolution, so a read, a
write and a title go through the link the author made rather than behind it, and
`Preview::status` spells it with `document::spell` exactly as the row is spelled.

**A file being created canonicalizes to nothing, so a write asks a fourth
question.** `app/src/document.rs:landing` is `confined`'s sibling: it
canonicalizes the **parent**, which does exist, joins the final component onto
it, and answers with the join for the reason above. `watch::resolve` is no help:
it answers with its *input* when canonicalization fails, so
`root.join("../escape.md")` would survive a `starts_with` textually. A parent
that will not canonicalize is a refusal too, which is how `newdir/x.md` is
refused with no clause of its own and folder creation stays a non-goal.
`app/src/document.rs:create_file` is the one caller, and asks two more:
`document::kind_of` is the predicate, so the extension decides the kind and
`.md`, `.bib`, `.yml` and `.yaml` are the panel's own filter minus the images it
does not make; and `File::create_new` makes *already exists* the filesystem's
answer rather than a check the write races with, which refuses a **dangling
symlink** an existence test would have written straight through.

It makes the file empty and stops — where an include marker sits is a
document-order decision no file list can make — and nothing announces the row:
it lands under the watched root, arrives as `watch::Change::Tree`, and the panel
is rebuilt off the status that follows, one path for a create and a `touch`.

**A delete asks a third question, neither of the other two serving it.**
`confined` opens on `is_file`, refusing a `missing: true` row and a dangling
link the walk lists; `landing` alone accepts a `secret.png` that is a link out
of the project. A delete wants both halves — **the name is under the root, and
something is at it** — so `app/src/document.rs:trash_file` reuses `landing`
unchanged for the first and adds `symlink_metadata` for the second, `is_file`'s
question widened to *anything at that name*. **It therefore acts on the name and
not the resolution**: a `cover.jpg` pointing at `figures/cover.jpg` loses the
link and keeps the figure, which is the opposite of `confined`'s reading for a
read and deliberately so — a read wants the bytes the author meant, a delete
wants the row the author clicked.

The OS call is a parameter where the create's write is not, the difference being
where the effect lands: a `std::fs::write` goes into a scratch tree the suite
owns, and this one goes outside the repository. `document::move_to_trash` is
what `main.rs` hands in. **`preview::Session::trash` refuses the main outright
and asks the dirty-buffer question only of the file the pane is holding** —
deleting any other throws no unsaved work away — and it re-walks with
`document::files_under` itself rather than waiting on the watch, which it must:
`watch::classify` answers the **first** match and a section the master names is
already in the asset list, so deleting one answers `Change::Asset` and never the
`Tree` a create rides on. `document::merge` then puts the path back as
`missing: true`. **The asymmetry with the create is real rather than an
inconsistency**: a created file is in no asset list, so the watch gets that one
right.

`app/src/document.rs:store_file` names the one file this app writes outside the
author's own folders — `projects.json` under the directory Tauri's resolver gives
`dev.letur.desktop` — and `read_override`/`write_override` keep one main per
canonical root in it, a `BTreeMap` so two writes of one map are two identical
files. **A missing, unreadable or malformed store is nothing remembered and never
an error in the window**; a failed *write* is reported, the author having just
asked for it in as many words. A dotfile in the author's own folder was refused
twice over: it is the manifest `specs/desktop_app_spec.md` §1.1 parks arriving by
another name, and it writes into a directory that may be under version control.
**The choice does not travel with the files**, and a folder moved to another
machine is discovered again.

