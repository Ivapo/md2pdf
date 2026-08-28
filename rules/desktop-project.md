---
title: desktop-project
sources:
  - app/src/document.rs
  - app/src/preview.rs
covers: >
  the project the desktop app opens: the root the opened file climbs one level
  to and the cap that is chosen rather than derived, the two edges that answer
  as no candidate, the masters discovered in the root and the reason that search
  does not recurse, the four ways one of them becomes the file that compiles, the walk and the merge and why they
  are two functions rather than one, the filter that is each channel's own
  comparison, the total order the panel cannot reorder itself against, the
  confinement the walk obeys and the one a write obeys differently, and the one
  fact this app remembers about a folder and where it refuses to keep it
max_lines: 90
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

`app/src/document.rs:store_file` names the one file this app writes outside the
author's own folders — `projects.json` under the directory Tauri's resolver gives
`dev.md2pdf.desktop` — and `read_override`/`write_override` keep one main per
canonical root in it, a `BTreeMap` so two writes of one map are two identical
files. **A missing, unreadable or malformed store is nothing remembered and never
an error in the window**; a failed *write* is reported, the author having just
asked for it in as many words. A dotfile in the author's own folder was refused
twice over: it is the manifest `specs/desktop_app_spec.md` §1.1 parks arriving by
another name, and it writes into a directory that may be under version control.
**The choice does not travel with the files**, and a folder moved to another
machine is discovered again.

