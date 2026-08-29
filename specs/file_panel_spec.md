---
id: mpdf-010
title: file-panel
note: >
  The panel becomes the project's files: the opened document's folder is the
  root, the panel lists the markdown, images and bibliographies under it, one
  file is set as the main the app compiles, and clicking another edits it while
  the main still draws the page.
status: accepted
last_updated: 2026-08-28

phases:
  - name: "Phase 1 — the project's files, and the main among them"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null
  - name: "Phase 2 — a row opens in the pane, and the main still compiles"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null
  - name: "Phase 3 — a file is created from the panel"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null
  - name: "Phase 4 — a file is moved to the Trash"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null
  - name: "Phase 5 — an image row shows the figure"
    reviewed: 2026-08-28
    shipped: 2026-08-28
    cut: null
    by: null

extends: null
supersedes:
  - id: mpdf-008
    phases: ["Phase 4 — the document shows its parts"]
superseded_by: null
related: [mpdf-002, mpdf-003, mpdf-008, mpdf-009]
reference: >
  Overleaf is where the main-file setting is read from: a project has many
  files, any of them can be edited, and one of them is marked as the one the
  compile runs on, so editing chapter three still previews the book. Its
  server-side compile, its collaboration and its cloud storage are out of scope
  permanently — `mpdf-001` §2 keeps this app local and fetching nothing. Its
  continuous auto-save is out of scope too, and §2 records why: this app's save
  is deliberate, and the divergence rule `mpdf-003` OQ-5 settled depends on it.
---

# file panel

## 1. Goal

Let the author see and handle the files their document is made of, from inside
the app. **This spec does not widen the observable** — `mpdf-008` did that, and
the observable is still *"one markdown file, or a master and the sections it
names, plus the images they name, single PDF out."* What changes is which file
the app **compiles** and which file it **edits**, which are the same file today
and are two different files after this.

The consumer is the author `mpdf-008` §1 describes — writing something long
enough that one file stopped being comfortable — who has now spent a week in
that shape and found that the app can show them the parts of their document and
do nothing else with them. To edit `sections/method.md` they leave the window,
open it in another editor, and lose the preview. To add a section they leave the
window again. The panel `mpdf-008` Phase 4 shipped names the files and is the
one thing in the window that cannot be clicked.

After this spec the author opens `showcase.md` and the window holds:

```
README.md                   # the panel: everything under the root,
refs.bib                    # files before folders, each alphabetical
showcase.md  ◀ main
sections/
  blocks.md
  emit.svg
  figures.md
  mark.svg
  mathematics.md            # ← clicked; the pane holds this
  notes-and-sources.md
  parse.svg
  pipeline.svg
  text.md
```

That is `samples/showcase/` exactly as it sits on disk today, listed by §2's
rule, which is why the order is neither the author's nor the document's.

The pane holds `sections/mathematics.md`, the page still shows the whole
compiled document, the caret's own page is right because the anchors now belong
to the file the pane holds, and `⌘S` writes `sections/mathematics.md`. Setting a
different file as main is one action, and the app remembers it the next time
that folder is opened.

### 1.1 Why this is a new spec and not a phase of an existing one

§6.1 is an ordered test, first match wins. It stops at step 1, and the two steps
before it are worked anyway because the answer to step 0 is what makes the rest
worth reading.

- **Step 0 — does this change a decision, or only the code?** A decision, and it
  is written down verbatim. `app/dist/index.html`'s `#files` comment says **"The
  rows do not load, and that is decided rather than deferred,"** and names four
  things that turn on it. This spec inverts that decision. It is not a defect
  being fixed; it is a settled choice being reversed on new information, which is
  §6.1's own definition of spec work.

- **Step 1 — does it remove or contradict shipped work?** **Yes, another spec's,
  so the test stops here.** `mpdf-008` Phase 4 shipped a panel whose non-goals
  read *"Not a file browser. It lists the parts the master names… not the
  directory the master sits in, not the files it does not name."* This spec
  builds that file browser. A phase appended to `mpdf-008` would read as if
  Phase 4 had never been built, which §6.1 gives as the reason this shape is
  never a phase. So: **new spec**, which will carry
  `supersedes: [{id: mpdf-008, phases: ["Phase 4 — the document shows its parts"]}]`
  and put `cut` + `by: mpdf-010` on that phase. `mpdf-008` keeps
  `status: accepted` and its rollup stays `partial` by §1.1 of the methodology,
  which is the rule that exists for this shape.

  **`supersedes` is deliberately absent from the frontmatter above**, following
  `mpdf-009`, whose draft left the same edge out for the same reason: its inverse
  is a `cut` date on a phase whose mechanism is on `main` and running, and a
  draft has removed nothing yet. This paragraph carries the argument and the edge
  follows the code — it is declared when Phase 1 ships, which is the commit that
  makes `mpdf-008` Phase 4's panel stop existing.

**Two decision statements elsewhere are changed, and neither is a phase.**

- `mpdf-003` §1.1 parks **"multi-file projects and manifests, a document library
  or recent-files list"** in its out-of-scope list. This spec defines a project
  root and remembers one fact per root, so the first clause stops being true. It
  is prose rather than a phase, so §6.1's mechanism is a dated `CORRECTED` note
  in place, beside the text it corrects, written in Phase 1's own push alongside
  the `cut` above and never while this is `draft` — a draft has changed nothing.
  §1.2 states what stays parked, which is most of it.
- `mpdf-009`'s `reference` says armquill's **"file explorer is out of scope
  permanently"**, and this spec builds a file panel. The two do not collide: that
  sentence's own reason clause is *"`mpdf-001` §2 keeps this app local and
  fetching nothing"*, and it is armquill's *server-backed* explorer it puts out
  of scope. This panel reads the local disk and fetches nothing, so the reason
  holds and the sentence needs no correction. It is named here so a reader who
  finds it does not conclude a permanent non-goal was quietly dropped.

### 1.2 Non-goals

- **Not a general file manager.** Four kinds of file are listed and nothing else
  (§2). No rename, no move, no copy, no folder creation. OQ-4 carries rename.
- **No project file, no manifest.** Nothing is written into the author's folder.
  The one remembered fact lives in the app's own support directory (§2), which is
  what keeps `mpdf-003` §1.1's *manifests* clause true.
- **No document library and no recent-files list.** Still parked. The app opens
  one root at a time, through the dialog or a Finder association, exactly as it
  does today.
- **No tabs and no second pane.** One pane, one edited file. §2 records what that
  costs and why the alternative was refused.
- **No auto-save.** The save stays deliberate, per §2.
- **No merge.** `mpdf-003` OQ-5's divergence rule is unchanged and the app still
  refuses rather than merging.
- **A section more than one directory below its master is not opened as a
  project** — by Finder, by `⌘O`, or by any other route to an open. §2's climb caps at one level; the deeper layout works in
  every other respect and is reached by opening the master. OQ-7 carries it.
- **Nothing agentic**, per `mpdf-003` §1.1, which is untouched.

## 2. Design

### Three values where the app has one (decision, recorded)

`app/src/preview.rs:Preview` holds a single `document`, and it answers three
different questions today because those questions have never disagreed:

| | what it answers | today |
|---|---|---|
| **root** | what the panel lists, and what the watch is rooted at | `app/src/watch.rs:root`, which is `document.parent()` |
| **main** | what compiles, and what the page shows | `Preview::document` |
| **edited** | what the pane holds, and what `⌘S` writes | `Preview::document` |

This spec separates them. `main` and `edited` are both files under `root`; they
start equal and Phase 2 lets them differ.

**`root` does not move when a row is clicked.** `app/src/preview.rs:Session::open`
re-roots the watch on every open, so clicking `sections/mathematics.md` under
today's mechanism would set the root to `sections/` and strand the author below
their own project with no way back up. The root changes on an explicit Open and
at no other time.

### The root climbs one level, and one level is a cap rather than a derived bound (decision, recorded)

**The opened file's parent is where the search starts, not where it stops.**
`app/src/watch.rs:root` is `document.parent()`, so a double-click on
`samples/showcase/sections/text.md` would root the project at
`samples/showcase/sections` — below the master that names it, which the panel
would then never list and discovery would never find. Taking that parent as the
root makes this spec's headline case unreachable, and it is the reason this
decision exists rather than being left to `watch::root`.

**The rule: start at the opened file's parent; if any `.md` in *that
directory's* parent names the opened file as one of its sections, the root is
the parent instead.** `md2pdf_core::section_paths` is what answers "names it",
reading text and constructing `Ok` unconditionally, so the test is total over
every markdown file in that one directory.

**One level is a cap, and it is chosen rather than derived.** An earlier draft
argued it was a property, from `mpdf-008`'s refusal of an include inside an
included section; review found that a non-sequitur and it is recorded here so it
is not re-derived. That refusal is about a master naming a master. The climb
looks for the **master of the opened file**, which a deeper *relative path*
reaches without any nesting: `core/src/emit.rs:portable_path` refuses only a
scheme, a leading `/`, a `..` segment and a backslash, and
`core/src/sections.rs:Segment::directory` splits on the last `/`, so
`[](parts/ch1/text.md)` is a supported marker and not merely an unrefused one.

**So the cap is argued on cost instead.** Climbing further means reading `.md`
files in directories above the project to guess where the project is —
`~/Documents`, `~`, `/Users` — and this app has never opened a file the author
did not name or a document did not name. No ceiling above one has an argument
either: `mpdf-008` OQ-1 already rejected *"allow one further level and no more"*
in a neighbouring decision as **a number nobody can defend**, and that applies
here unchanged.

**The cost is real and is stated rather than hidden.** A section more than one
directory below its master — `parts/ch1/text.md` under a master at the root —
roots at `parts/ch1`, which is the failure the paragraph above says this
decision exists to prevent, surviving at a depth the cap does not reach. It is
**recoverable in one action and only that action**: opening the master itself
roots correctly, and because the store is keyed by root, a wrong root cannot be
corrected from inside the panel. §1.2 makes the deeper layout a non-goal,
OQ-7 carries it, and Phase 1's gate **asserts the capped behaviour** so it reads
as a decision in the suite rather than as a defect nobody noticed.

**Two edges answer the same way as the store's:** a file whose parent has no
parent, and a grandparent that will not `read_dir`. Both are *no candidate
found*, so the root is the opened file's own parent, and neither is an error in
the window.

A file nobody names roots at its own parent, which is every single-file
document and is `watch::root`'s answer unchanged.

### The main file is discovered, overridden, and remembered outside the folder (decision, recorded)

**Discovery is total, so the common case needs no configuration.**
`core/src/lib.rs:section_paths` reads the master's own text and its body cannot
fail — it returns `Result` for signature symmetry with the two walks beside it
and constructs `Ok` unconditionally. So *"a `.md` under the root whose text names
section markers"* is a decidable test over every markdown file in the tree, and:

- **a stored override for this root → that file is main**, and discovery is not
  consulted. The store is read *first*, on every open, or it is a thing written
  and never used.
- **exactly one master → that file is main**, whatever the author opened;
- **no master → the file the author opened is main**, which is every single-file
  document and is today's behaviour exactly;
- **more than one master → the opened file if it is itself one of them,
  otherwise the **byte-wise** alphabetically first**, and the panel marks which
  file it landed
  on. **This state never leaves `main` unset**: an empty pane and no page is a
  worse answer than a guess the author can see and correct in one action, and
  the mark is what makes the guess visible. Alphabetical is not a claim about
  which is right — it is a claim that the same folder opens the same way twice,
  which a set iteration order would not be.

**The override is remembered in the app's support directory, keyed by the
canonical root path** — `~/Library/Application Support/dev.md2pdf.desktop/`, per
`app/tauri.conf.json`'s `identifier`. A dotfile in the author's own folder was
refused for two reasons and either is sufficient: it is the manifest `mpdf-003`
§1.1 parks, arriving by another name; and it writes a file into a directory the
author may have under version control, which is a thing this app has never done
and should not start doing as a side effect of a panel.

The cost is accepted and stated: **the choice does not travel with the files.**
Move the folder to another machine and discovery runs again. For one master —
which is the shape `mpdf-008` designed for — discovery gets it right, so the cost
falls only on a root holding two masters, where it is one click.

### The pane holds the edited file and the main file compiles, which reverses four things (decision, recorded)

`app/dist/index.html`'s `#files` comment names the four things that turn on the
rows not loading. Each is answered here, and none of them is `core`'s:

1. **The anchors.** `app/src/document.rs:render_with` keeps only the anchors
   whose `location.file` is `None`, because those are the master's own and the
   pane holds the master. It becomes **the anchors belonging to the file the pane
   holds**, and `None` is simply what that filter reads as when the pane holds
   main. `app/dist/index.html:caretPage` walks a flat list and is unchanged: it
   is still one file's lines against the whole document's pages, which is what it
   was always doing. **This is the phase's payoff, not its cost** — the caret's
   page is right while editing a section, which today it cannot be.

2. **The external-change rule.** `app/src/watch.rs:classify` answers `Document`
   or `Asset`, and it gains **two** further answers rather than one, because two
   different things want events it discards or mislabels today:

   - **`Tree`** — any path under the root that is neither the document nor a
     named asset, which `classify` currently answers `None` for. The panel needs
     exactly the events it drops, and the compile must not run for them. **Phase
     1 adds this one.**
   - **`Edited`** — the file the pane holds, when that is not `main`. It cannot
     ride `Asset`: a section the master names is *already* in the asset list
     `app/src/document.rs:render_with` builds, so it already classifies as
     `Asset` and would silently recompile instead of running the divergence
     rule. `Edited` is therefore tested **before** `Asset` and takes precedence
     over it for that one path. **Phase 2 adds this one**, with the rule that
     consumes it.

   The rule itself is not forked and not multiplied — it runs on one buffer, as
   it does today, and only the question of which event delivers it changes.
   `app/src/watch.rs:Changed` gains one `bool` per answer.
   `app/src/preview.rs:Session::classifier` and `Session::on_change` closed over
   the document alone when this was written; **Phase 1 gave both the root**, and
   Phase 2 gives them the second path, since the assets are `main`'s and resolve
   against `main`'s directory while `edited` is somewhere else entirely.
   **`Change::Document` also changes meaning in Phase 2** — main moving on disk
   is a bare recompile, and `Edited` is what carries the rule.

3. **The save.** `app/src/preview.rs:Preview::save` wrote to `document`; it
   writes to `edited`, and `app/src/preview.rs:Preview`'s `saved` field follows
   `edited` for the same reason. **Phase 1's rename did the code**, the two
   fields being equal there; Phase 2 changes what it *means* and is gated on
   that rather than on the diff.

4. **The join reading every section off the disk.** **This one is already
   solved, and measuring it is what shrank this spec.** `core` never reads a
   file: `core/src/sections.rs:assemble` takes `&[Asset]`, each carrying its own
   bytes, per `mpdf-008` §2's *"`core` stays OS-free"* decision. The disk read is
   the app's, in `app/src/document.rs:read_sections_with`, through the closure
   `app/src/document.rs:render_with` injects — whose own doc comment says **"One
   closure serves both passes… every file this app opens for one compile goes
   through the counter."** So the unsaved edit reaches the compile by passing a
   closure that answers the edited path from the pane's buffer and delegates
   every other path to `std::fs::read`. **No change to `core`, and no phase
   appended to `mpdf-008` for it.**

### Switching files refuses while the buffer is dirty (decision, recorded)

`app/src/preview.rs:Session::open` assigns `Preview { document, ..Preview::default() }`
with no check on the buffer it is replacing, and `app/dist/index.html:openDocument`
calls `clear()` and invokes it. **An unsaved edit is silently discarded on open,
today.** That is tolerable behind a menu item and a native dialog, which is where
it has lived; it is not tolerable one click away from every row in a panel.

So the switch **refuses while the buffer diverges from `saved`, names both ways
out, and takes neither** — which is `app/src/preview.rs:DIVERGED`'s *shape*,
already in this app and already reviewed, applied to a second occasion. Save, or
discard, and the author says which.

**The shape and not the sentence, and both halves of that cost something.**
`DIVERGED` opens *"this file changed on disk"*, which is false on this occasion,
so reusing the constant would put a lie in the window: the switch gets its own
words. And **discard does not exist in this app** — `save` is a command and
dropping the buffer is not — so a refusal naming two ways out is naming one that
cannot be taken until Phase 2 builds it. It is `Preview::load`'s own path, which
already re-reads the file and clears the divergence, behind a command.

**A buffer per file was considered and refused.** It would multiply `saved` by
the number of files touched, make `external_change` a question about a set rather
than a string, and give the window state no test can reach without enumerating
it. The app is not a tabbed editor, and §1.2 says so.

### The panel lists what the pipeline can read, from the pipeline's own list (decision, recorded)

Markdown, bibliographies, and **every extension the dialect accepts as an
image** — `core/src/emit.rs:IMAGE_EXTENSIONS`, which is
`png jpg jpeg gif webp svg svgz pdf`. Not a hand-written subset. A panel that
listed `.png` and `.svg` would be invisible to a perfectly legal `.jpg` figure,
and the author would conclude the app could not read it. `IMAGE_EXTENSIONS` is
private today and `core` exposes it for this, which is the smaller change of the
two available and keeps one list rather than two that drift.

`pdf` is in that list, so the document's own exported PDF appears in the panel.
That is correct — a PDF is a legal figure in this dialect — and it will look odd.
OQ-3 carries it rather than special-casing the export target on a guess.

### One flat list of entries, and folders are drawn rather than sent (decision, recorded)

`Status` carries **a flat `Vec` of entries, each `{ path, kind, missing }`** —
`path` root-relative with `/` separators, `kind` one of `markdown`,
`bibliography` or `image`, `missing` true for a path the master names that the
disk does not hold. **A directory is never an entry.** The page derives the
folder headings and the indentation from the path segments, which is a thing a
page can do and a thing a nested node type would make `Status` carry twice.

**The order is total and computed in Rust**, so the panel cannot reorder itself
between two compiles of the same tree: within each directory, **files
alphabetically first, then subdirectories alphabetically**, each expanded where
it sits. Byte-wise comparison on the path segment, not a locale collation —
a locale-dependent order is not reproducible by a second person, which is what
this phase's gate needs it to be.

**`main` is spelled the same way an entry is** — root-relative, `/`
separators — and not as the bare file *name* `Status::master` carried, or the
page could not match it to a row to mark.

This is the field `app/dist/index.html`'s `@typedef` block declares and
`app/src/preview.rs:the_page_typedefs_name_exactly_the_fields_status_serializes`
holds to the Rust side, so its shape is settled here and not in the window.

### The panel lists the disk *and* what the master names (decision, recorded)

A tree built from the disk alone loses the one thing `mpdf-008` Phase 4's panel
was good at: a section the master names and the disk does not hold is exactly the
row the author needs to see, and it is the state `MissingSection` refuses on. So
the panel lists the **union** — files found under the root, plus paths
`section_paths` names that are missing, marked as missing.

This is also what keeps the panel honest while a marker is half-typed.
`mpdf-008` §2 accepted a panel that flickers because `section_paths` reads the
text rather than the disk; here the disk half is stable and only the marked-missing
half flickers, which is strictly less motion than the shipped panel has.

### Every write is confined under the root (decision, recorded)

**The walk obeys it too, and that is not only Phases 3 and 4's concern.** A
symlink under the root pointing at a directory elsewhere would otherwise put
that directory's files in the panel as though they were the project's, and
Phase 2 would open one of them in the pane. So the listing **does not follow a
symlink that resolves outside the canonical root**, and Phase 1's gate tests it
against a link that would otherwise contribute rows.

Phases 3 and 4 are then the first time this app *writes* to a path the author did
not choose in a native dialog. The rule is the one the document's own paths obey:
resolve the panel's path against the root and **refuse anything that does not
land under the canonical root**, symlinks followed before the check rather than
after. **`app/src/watch.rs:resolve` is the spelling for the walk and not for a
write**: it is `canonicalize().unwrap_or_else(|_| path.to_path_buf())`, which
returns its *input* when canonicalization fails — and a file being created never
canonicalizes, so `root.join("../escape.md")` would survive the check textually.
A write therefore canonicalizes the **parent**, which does exist, and joins the
final component to it. The refusal is a sentence in the
window, not a panic.

### Creating a file does not write the marker into the master (decision, recorded)

Phase 3 creates the file and stops. The panel never writes markdown the author
did not type, and **where an include marker sits is a document-order decision no
panel can make** — a new section belongs after chapter two, or before the
appendix, and nothing in a file list knows which.

The cost is that adding a section is two actions rather than one. Accepted, and
OQ-1 carries the shape that would close it: an insert-at-caret gesture that would
serve images and new sections through one mechanism, which is a better answer
than a special case for one of them.

**CORRECTED 2026-08-28:** OQ-1 no longer carries that shape. It was built,
tried at the window and **refused** — a figure wants looking at rather than a
marker written on the author's behalf — and OQ-1 resolved to a preview instead.
So **this cost is now unmitigated**: adding a section is two actions, and
nothing open is going to make it one. The decision above is unchanged; only the
sentence naming a way out of its cost is.

### Delete moves to the Trash (decision, recorded)

Not `std::fs::remove_file`. **There is no undo anywhere in this app** — not for
an edit, not for a save, not for an export — and the Trash is the platform's own
undo for exactly this operation. `tauri-plugin-fs` offers no trash call and
neither does `std`, so this costs a dependency: the `trash` crate, or an
`NSFileManager trashItemAtURL` call through `objc2`. Phase 4 prices both and
picks one; the decision recorded here is only that a plain unlink is refused.

**CORRECTED 2026-08-28, in Phase 4's review round 1: the same argument retires
the confirmation.** The paragraph above says the Trash *is* the platform's undo
for this operation, and a confirmation is what stands in for an undo where there
is none. Finder itself does not confirm a move to the Trash, for that reason. So
Phase 4 ships **no confirmation**, and the sentence its scope carried — *"A
confirmation precedes every delete"* — is withdrawn rather than built: it was
drafted beside the Trash decision without noticing that the one makes the other
redundant. The cost of being wrong is one file in the Trash, recoverable by Put
Back, which is less than this app charges for a save. Round 1 priced what the
confirmation would have cost, which is what makes this a saving rather than a
preference: `app/capabilities/default.json` grants `core:default` and the two
file dialogs and nothing else, so a native `ask` wants a new permission and a
rewritten description, and the two alternatives either compose text in the
window or put state on a row that `app/dist/index.html:parts` rebuilds.

## 3. Open questions

- **OQ-1 — what does clicking an image row do?** ~~*(design call)* Three shapes:
  inert, which is what Phase 2 ships; a preview, which needs a second surface in
  a window that has two panes already; or **insert `![](figures/overview.svg)` at
  the caret**, which is the one that would also close the cost the create
  decision above accepts, since a new section and a new figure both want their
  marker placed by the author. It wants the resolved path written relative to the
  *edited* file rather than the root, per `mpdf-008` §2's section-relative rule —
  and what an image *above* the edited file may be written as depends on
  `mpdf-008` Phase 5, which is **drafted and neither reviewed nor shipped**. That
  is a second reason this stays open rather than becoming a phase here.
  **Blocks nothing** — Phase 2 leaves image rows inert and says so in the
  window.~~ **RESOLVED 2026-08-28, by prototype: the second shape. Clicking an
  image row shows the picture over the text pane.** Phase 5 below ships it.

  **Both blockers had cleared, and the insert shape was refused on its merits
  rather than on its cost.** `mpdf-008` Phase 5 shipped on 2026-08-28, so a
  section may name a figure above itself: a master naming `sections/text.md`
  whose text is `![](../cover.svg)` compiles, checked, and only an image above
  the *master's own folder* is still refused — `![](../../cover.svg)` from
  `parts/sections/text.md` under a master at `parts/book.md`, which fails with
  *"image with a path that leaves the document's folder"*. The insert gesture
  was then built and tried, and the answer was that **a figure wants looking at,
  not a marker written on the author's behalf**: where a section goes in a
  document is a decision the author is making, and so is where a figure goes.
  The path arithmetic it needed is recorded here because it was worked and
  verified and the next reader should not re-derive it — everything is
  *master-relative*, since `md2pdf_core::Sources::resolve` prefixes a
  destination with the section's own directory relative to the master, and the
  virtual root a `..` may not escape is the master's folder rather than the
  project root.

  **The preview's predicted cost — "a second surface in a window that has two
  panes already" — is real and is not a third pane.** It is a *view* over the
  text pane, the way `Lines` is a view: `edited` does not move, so `⌘S`, the
  compile, the page and the anchors are all untouched, and that is the whole
  reason this is cheap. Two findings from the prototype, both load-bearing and
  neither obvious: it is **absolutely positioned over the text pane's own
  column rather than replacing it**, because `app/dist/index.html`'s divider
  drag reads `#text`'s own box and a hidden textarea measures zero; and the
  bytes come through **a command returning `tauri::ipc::Response`, which the
  page turns into a blob**, which is `current_pdf`'s own route and needs neither
  Tauri's asset protocol, nor a scope in `app/tauri.conf.json`, nor a new
  capability.

  **What it does not settle is the PDF row**, and that is OQ-8 below.

- **OQ-2 — does a `.bib` open in the pane at all?** *(design call)* It is not
  markdown, so a pane holding it compiles nothing meaningful and the page would
  show the last good PDF beside text that cannot produce one. The shapes: inert,
  which is what Phase 2 ships; editable with the compile suppressed and the
  status line saying so; or handed to the system editor. **Blocks nothing.**

- **OQ-3 — does the panel show the document's own exported PDF?** *(deferred by
  evidence)* It does, per §2, because a PDF is a legal figure and the list is the
  pipeline's own. Whether that reads as correct or as noise needs the panel in
  use for a week; the alternative is to hide the path
  `app/src/document.rs:default_output` names, which is a special case that would
  be wrong the moment an author names their figure the same thing. **Blocks
  nothing.**

- **OQ-4 — rename and move.** *(design call)* §1.2 makes both non-goals for now.
  Rename is the one with a real argument behind it: renaming a section the master
  names breaks the document silently unless the marker is rewritten too, and
  rewriting the master's text is what the create decision above refuses to do. So
  rename is not "delete plus create" and should not be added as though it were.
  **Blocks nothing.**

- **OQ-5 — what is the gesture that sets main?** ~~*(design call)* **Only the
  gesture is open.** §2 settles what `main` *is* in every case — stored override,
  one master, none, or several — and Phase 1's gate clause 3 pins each of those
  four to a value, so an implementer needs nothing from this entry. What is left
  is whether the author changes it by a row action, a menu item or something in
  the status area, and whether the affordance appears at all when discovery found
  exactly one master. **Blocks nothing**, and it is left to the prototype per the
  standing practice that a UI shape is tried in the running app before it is
  specced.~~ **RESOLVED 2026-08-28, in Phase 1: a `main` button on the row,
  revealed on hover and on focus.** It sits where the `◀ main` mark sits —
  `margin-left: auto` — because only one of the two is ever on a row, and it
  carries the entry's own `path` rather than the row's text, the text being the
  last segment where Rust is handed the whole path. **Only a markdown row that
  is not already main gets one**, nothing else being able to compile.

  The two sub-questions are answered together, and the answer to the second is
  *yes, always*. **A row action rather than a menu item**, because a menu item
  needs a selection and Phase 1 has none — the rows hold no state of their own,
  which is what lets the panel be rebuilt whole on every status. And the
  affordance appears even where discovery found exactly one master: hiding it
  there would make the panel's behaviour depend on a count the reader cannot
  see, and a folder gains a second master the moment somebody writes one.

  **It is deliberately not the row's body**, and that is the part Phase 2 turns
  on: the body is inert in Phase 1 and Phase 2 gives it a meaning of its own —
  clicking it puts that file in the pane — so the two gestures were kept apart
  before there were two of them, rather than after.

- **OQ-6 — what does the panel cost a reader using a screen reader?**
  *(needs-input)* `mpdf-009` OQ-3 asks the same question of the canvas and is
  open for the same reason: nobody on this project has run one. A tree of rows is
  more tractable than a canvas, and the answer is probably a list with an
  accessible name per row, but *probably* is not an answer. **Blocks nothing.**

- **OQ-7 — how does a project rooted more than one directory above the opened
  file get found?** *(design call)* §2 caps the climb at one level and argues the
  cap on cost rather than deriving it, so this is the question that cap leaves
  open. The shapes, none of them free: climb until an ancestor holds a `.md`
  naming the file, which reads markdown outside the project to guess where the
  project is; stop at a repository marker such as `.git`, which assumes a tool
  this project has never assumed; or let the author name the root once and
  remember it, which is the store extended from one key to two and is the only
  shape that needs no guess at all. **Blocks nothing** — Phase 1 pins the capped
  behaviour in its own gate, and §1.2 makes the deeper layout a non-goal until
  this is answered.

- **OQ-8 — what does a PDF row show?** *(design call, opened by OQ-1)* `pdf` is
  in `core/src/emit.rs:IMAGE_EXTENSIONS`, so a PDF is a legal figure, the panel
  lists one, and Phase 5's viewer can be asked for one — which `<img>` cannot
  draw. Three shapes: **the sentence Phase 5 ships**, which says so and draws
  nothing; the first page through the `pdf.js` this app already vendors, which
  is reachable rather than a new dependency; or a PDF row that does not open at
  all, so the question never arises. It is **not hypothetical and not rare**:
  OQ-3 records that a document's own exported PDF sits in the panel beside its
  markdown, so a reader meets a PDF row by accident rather than by intent — and
  if OQ-3 is answered by hiding the export, most PDF rows go with it, which is
  why this waits on that one. **Blocks nothing.**

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. **Two of the five produce the
observable and three do not**, which is a higher ratio of scaffolding than any
spec here has carried, and it is a property of the subject rather than an
oversight: a panel is *about* the material the observable is made from. The
three that produce none are argued in place, and none is a prerequisite of
another — Phases 3, 4 and 5 could each be cut without touching what Phases 1 and
2 deliver, which is the honest test of a phase that shows nothing. **Phase 5 was
appended after Phase 2 shipped**, out of the prototype that resolved OQ-1; it
depends on Phase 2 and on neither of the two between them, so it may ship out of
order, which §3 of the methodology allows and this sentence is the record of.

### Phase 1 — the project's files, and the main among them
*Produces the observable: **yes** — open `samples/showcase/sections/text.md` from
Finder and the window compiles `showcase.md`, which is a different PDF than the
one the app draws today for that same double-click.*

- **Scope:** `app/src/preview.rs:Preview` gains `root` and `main` beside
  `document`, which becomes `edited` and still equals `main` at the end of this
  phase. `app/src/preview.rs:Session::open` takes the opened path, resolves the
  root by §2's one-level climb, **reads the store, and falls back to discovery
  only when the store holds nothing for that root**, then opens the file it
  landed on rather than the file it was handed.

  Four functions in `app/src/document.rs`, beside the other window-free logic,
  each an ordinary function with an ordinary test per that file's own header:

  1. **the root**, implementing §2's climb — the opened file's parent, or *its*
     parent when some `.md` there names the opened file as a section.
  2. **discovery**, returning the `.md` files under the root that
     `md2pdf_core::section_paths` reports a non-empty list for, and §2's
     resolution when there is more than one.
  3. **the listing**, returning §2's flat `Vec` of `{ path, kind, missing }` in
     §2's order, filtered to `.md`, `.bib` and `md2pdf_core::IMAGE_EXTENSIONS`,
     unioned with the paths `section_paths` names that the disk lacks, and **not
     following a symlink out of the canonical root** — `app/src/watch.rs:resolve`
     is the existing canonicalization and the existing spelling.
  4. **the store**, reading and writing one JSON object keyed by canonical root
     path under `dev.md2pdf.desktop`'s app-support directory, holding the main
     override and nothing else. A missing, unreadable or malformed store is
     *nothing remembered*, never an error in the window.

  **`core` exposes the extension list**, and `pub` alone is not enough:
  `core/src/lib.rs` declares `mod emit;` privately, so the phase adds
  `pub use emit::IMAGE_EXTENSIONS;` — **the crate's first `pub use`**, since
  everything it exports today is declared in `lib.rs` itself. The alternative
  is moving the constant to `lib.rs`, which splits it from
  `core/src/emit.rs:check_image`, its only reader — so the re-export is the
  smaller change of those two.

  **The store costs a dependency move.** `serde_json` is a *dev*-dependency in
  `app/Cargo.toml` today, under a comment arguing it is deliberately dev-only,
  and it becomes a real one. The comment is rewritten rather than deleted, and it
  keeps its own mitigating fact: `serde_json` is already in `Cargo.lock` at this
  version by way of `tauri`, so promoting it **adds no crate to the tree**.

  `app/src/preview.rs:Status` gains the listing and the main path, and **loses
  `sections` and `master`** — the tree replaces both, and leaving them would
  leave the page two lists to disagree about.
  `app/dist/index.html`'s `#files` replaces the section list with the tree, and
  its `@typedef` block follows: the entry type is declared there and
  `app/src/preview.rs:the_page_typedefs_name_exactly_the_fields_status_serializes`
  is extended to it, which means editing that test's field-by-field `Status`
  construction. **Its `declared.len() == 10` literal does not move** — `Status`
  has ten fields today, this phase removes two and adds two, and the coincidence
  is worth naming so nobody "fixes" the count to make a failure go away.

  `app/dist/index.html:parts` hides the panel on `state.sections.length === 0`.
  **That rule goes**: per §2 the panel is drawn for every open document,
  including a lone `.md` that names nothing, which is a visible change to
  today's window and is what lets an author build a first section from inside
  the app. The fold toggle stays and is the reader's answer to a cluttered
  folder.

  **The tree is held in `Preview` and refreshed on two occasions only** — an
  open, and a `Tree` event — never recomputed inside `Preview::status()`, which
  the page calls on every render and which would then walk the disk on every
  keystroke. `app/src/watch.rs:classify` gains §2's `Tree` answer, and
  `app/src/watch.rs:Changed` a `tree: bool` beside its two;
  `app/src/preview.rs:Session::classifier` and `Session::on_change` take the
  root beside the document, since `classify` re-derives its root from the
  document it is handed and the two can now differ. **A `Tree` event refreshes
  the listing and does not compile.**

  Rows are not clickable in this phase except the one gesture that sets main.

- **Exit gate:** In the Rust suite, over
  **`tests/fixtures/panel/`, a fixture this phase creates** — the sample tree is
  not used, because `samples/showcase/` gains a `showcase.pdf` for any developer
  who has run its own README and an exact-enumeration gate over it is therefore
  not reproducible by a second person:
  1. **The root climbs, once.** Opening `<fixture>/sections/text.md`, whose
     master `<fixture>/book.md` names it, yields `root = <fixture>`. Opening
     `<fixture>/loose/orphan.md`, which no `.md` above names, yields
     `root = <fixture>/loose`. Opening `<fixture>/book.md` yields
     `root = <fixture>`. **And the cap is asserted, not merely accepted:**
     `<fixture>` also holds `parts/ch1/deep.md`, which `book.md` names as
     `[](parts/ch1/deep.md)`, and opening it yields `root = <fixture>/parts/ch1`
     — §2's stated cost, pinned here so a later change to the climb has to
     change this line deliberately.
  2. **Discovery.** Over `<fixture>` it returns exactly `book.md`; over a
     directory holding one markdown file that names no section it returns empty.
  3. **Main follows §2's order.** With no store, opening `sections/text.md`
     leaves `main = book.md` and `edited = book.md`, and the compiled PDF equals
     byte-for-byte the one produced by opening `book.md` directly — the
     observable claim, checked. With a store naming `other.md` for that root,
     `main = other.md` and discovery's answer is not used. With two masters and
     no store, opening a third file lands on the alphabetically first and
     opening one of the two masters lands on that one.
  4. **The listing.** Over `<fixture>` it returns exactly the entries the
     fixture's manifest names, in §2's order, with `kind` right for each. **The
     manifest is a `.txt` beside the fixture and not inside it**, so it cannot
     become a row in the listing it defines;
     a master naming `sections/missing.md` puts one `missing: true` entry in it
     and no file on disk; a `.txt` and a `.typ` in the fixture contribute
     nothing; a `.jpg` **does**, per §2's extension list.
  5. **Confinement is tested where it can fail.** A symlink at
     `<fixture>/outside` pointing at `tests/fixtures/panel-decoy/` — **a
     committed sibling, so the link resolves the same on any clone** — which
     holds `decoy.md` and `decoy.png`, contributes **no** entries. (A link to a directory holding
     nothing the filter matches would pass under an implementation with no
     confinement at all, which is why the target holds two files that do match.)
  6. **The store round-trips**, and a truncated store file reads as no override
     with no error surfaced.
  7. **`classify`.** A path under the root that the document does not name
     answers `Tree`; the document still answers `Document`; a named asset still
     answers `Asset`; a path outside the root still answers `None`. And a `Tree`
     event refreshes the listing without incrementing
     `app/src/preview.rs:Preview`'s `revision`, which is what "does not compile"
     means as an assertion.

  At the window, on `tests/gates/mpdf-010-phase1.js` pasted into the console per
  `mpdf-009` Phase 5's instrument: the panel holds the rows the Rust listing
  returns, in that order; the row for `main` is marked; and **`samples/article.md`
  — a document naming no section — draws a panel where today it draws none**,
  which is `mpdf-008` Phase 4's gate case (2) inverted deliberately and is worth
  seeing fail against the old build.

- **Close-out:** `rules/desktop.md` (the commands, the state the loop writes, the
  three answers the asset list gives and the filter reads, the configuration
  facts, and the dependency move), `rules/desktop-panes.md` (the panel section,
  which is currently `mpdf-008` Phase 4's and is replaced wholesale, along with
  the "rows do not load" passage's first half), `rules/pipeline.md`
  (`IMAGE_EXTENSIONS` becoming part of `core`'s surface). README: the app's
  description of what it opens, and the panel it now draws for every document.
  **`mpdf-008` Phase 4 takes `cut` and `by: mpdf-010` in this phase's own
  push**, and this spec takes the matching `supersedes` entry in the same
  commit — per §1.1, the edge is declared when the panel it removes stops
  existing, which is here. `mpdf-003` §1.1 takes its dated `CORRECTED` note in
  that commit too. **Its own push**; nothing here depends on another phase's.

### Phase 2 — a row opens in the pane, and the main still compiles
*Produces the observable: **yes** — the page shows the whole compiled document
while the pane holds one section of it, and the caret's own page is right, which
no state of this app has ever shown.*

- **Scope:** Depends on Phase 1 and on nothing outside this repository. `edited`
  stops tracking `main`, and **five things follow from that one sentence.**
  Round 1 found every one of them missing, so they are enumerated here rather
  than left to be rediscovered at a keyboard.

  1. **The compile renders `main`, not the pane.**
     `app/src/preview.rs:Preview::compile` is
     `document::render(document::directory(&edited), &self.buffer)`, which with
     `edited != main` renders a section's buffer as though it were the whole
     document — the override in the closure is never reached, because the
     markdown never goes through the closure. So the markdown becomes **`main`'s
     own text** and the directory **`main`'s own directory**, every path the
     document names resolving against the master. **Main's text is read through
     the same closure the override rides on**, which is one rule instead of a
     branch: the closure answers `root.join(edited)` from the pane's buffer and
     every other path from `std::fs::read`, so asking it for `root.join(main)`
     returns the buffer exactly when the pane holds main and the disk otherwise.
     **The closure yields bytes where the compile wants a string**, so this
     read decodes as UTF-8 and a `main` that is not text fails here; the message
     and the *failed* state are `app/src/preview.rs:Preview::load`'s, built the
     way `app/src/document.rs:read_document` builds them, so a main that will not
     read reads the same in the window whichever path reached it.

  2. **The anchor filter takes the path as the master names it.**
     `app/src/document.rs:render_with` keeps the anchors whose `location.file`
     matches the edited file, `None` meaning main. **`location.file` is
     master-relative** — it is the marker's own spelling, per
     `md2pdf_core::Location` — where `edited` is root-relative, and the two
     coincide only while main sits at the root, which a stored override or
     `Session::set_main` may perfectly well make false.
     `app/src/preview.rs:Preview` computes the master-relative spelling — strip
     `main`'s own directory prefix from `edited` — and passes that.
     **`app/src/document.rs:beside` is its inverse and not its implementation**:
     that one takes a master-relative path to a root-relative one, and this
     wants the reverse, so it is a second small function rather than a call to
     the first. An `edited` that does not sit under `main`'s directory has no
     master-relative spelling at all, and keeps every anchor filtered out, which
     is the same answer as a file the master does not name.

  3. **`classify` needs both paths, and `on_change` remaps two answers.**
     `app/src/watch.rs:classify` resolves the asset list against the document's
     own directory and the assets are **main's**, so it takes `main` *and*
     `edited` beside the root. It gains §2's **`Edited`** answer here — Phase 1
     added only `Tree` — tested **before** `Asset`, since a section the master
     names is already in that list. `app/src/watch.rs:Changed` gains its bool
     beside the other three. **`app/src/preview.rs:Session::on_change` then
     means something new by `Document`**: main moved on disk, which is a bare
     recompile, where **`Edited`** is the answer that runs
     `app/src/preview.rs:Preview::reload` and the divergence rule. While
     `main == edited` an event answers `Edited` first, so the rule runs exactly
     where it runs today and Phase 1's behaviour is unchanged by construction.

  4. **The switch re-arms both loops, and is not an open.**
     `Session::on_change` and `Session::recompile` guard on `preview.edited`
     against a path captured when the loops were started, so a command that set
     `edited` and stopped there would leave the typing debounce compiling nothing
     and every filesystem event dropped — and gate clause 5 and the whole window
     gate unreachable. The command drops and restarts both loops the way
     `app/src/preview.rs:Session::open_at` does, and the guard itself stays: it
     is what stops a thread mid-compile from writing its page over a newer one.
     **It borrows that half of `open_at` and not the other half.** `open_at`
     assigns `Preview { .. ..Preview::default() }`, which zeroes `revision` and
     `reloaded` — and `app/dist/index.html`'s `clear()`, which resets the
     counters the page compares them against, runs on an Open and **not** on a
     row click. A switch that replaced `Preview` wholesale would therefore
     strand `refresh` at its own guard and draw nothing again. It sets `edited`,
     reads that file into the buffer and `saved`, and leaves `root`, `main`, the
     listing, the bytes, the anchors and both counters exactly as they were.

  5. **`Preview::save` and `saved` already follow `edited`** — Phase 1's rename
     did the code, the two paths being equal there. This phase changes what that
     rename *means* and is gated on the meaning rather than on a diff.

  **The switch, and the two ways out it names.** A new command sets `edited` and
  **refuses while `buffer != saved`**, per §2. It gets **its own sentence and its
  own constant** beside `app/src/preview.rs:DIVERGED` rather than reusing it,
  for the reason §2 now records: `DIVERGED` opens *"this file changed on disk"*,
  which is false here. **The second way out is built in this phase**, because a
  refusal naming one that does not exist is worse than a refusal naming one —
  a command that drops the buffer and re-reads the edited file, which is
  `Preview::load`'s own path. The refusal rides `app/src/preview.rs:Preview`'s
  `divergence`, whose meaning widens from *a refused external change* to *a
  refused change*: both are cleared by the same two actions —
  `app/src/preview.rs:Preview::save` and `Preview::take`, which the discard
  reaches through `load` — which is the argument for one field carrying both
  rather than a second field the page would have to tell apart. **One field
  means one occasion at a time**: a switch refused while a real divergence
  stands overwrites its sentence and is overwritten by the next, which costs
  nothing — the two name the same two exits — and is a fact for
  `rules/desktop.md` to carry rather than for a later reader to rediscover.
  **`Session::set_main` reports through that same field** and not through its
  own `Err`, which is where its other refusals go, so one refusal does not
  arrive in the window two ways. `app/dist/index.html` places that sentence exactly as it
  places every other status sentence, composing none of it. **`Session::set_main`
  takes the same refusal** — it routes through `open_at`, which assigns
  `Preview { .. }` with no check on the buffer it replaces, and that is verbatim
  the hazard §2's rule exists for.

  `app/src/preview.rs:Status` gains the **edited** path beside `main`,
  root-relative and spelled the same way, so the panel can mark the row the pane
  is holding — §1's end-state draws both marks and the page cannot derive one
  from the other. **That moves `declared.len()` from 10 to 11** in
  `app/src/preview.rs:the_page_typedefs_name_exactly_the_fields_status_serializes`,
  against an in-code comment Phase 1 wrote telling the next reader the count was
  a coincidence and not to move it. Moving it here is deliberate, this sentence
  is the authority for it, and that comment is rewritten in the same commit.
  `app/dist/index.html:fileRow` gains a second mark beside its existing `here`
  — a class of its own and a rule of its own in the panel's CSS, because the two
  are two facts and a row can carry both. `app/src/main.rs` registers the two new
  commands in `generate_handler!` and retitles the window from
  `preview().document()`, as `open_document` already does — that being `edited`
  now.

  **`core` is not touched**, per §2.

  A markdown row the master does not name — a `README.md` beside a master — opens
  like any other: the compile is still main's, the override is never consulted
  for it, and the pane's file contributes no anchors, so the page opens at page
  1. Correct rather than special-cased, and stated so nobody reads it as a defect.

- **Exit gate:** In the Rust suite, over **a scratch copy of `samples/showcase/`**
  — clauses 1 and 3 write, `samples/` is tracked, and a suite that left the
  repository dirty would also destroy clause 1's own premise on its second run.
  `app/src/preview.rs`'s `scratch_dir` is the existing spelling.
  1. With `main = showcase.md`, `edited = sections/mathematics.md` and a buffer
     differing from that file on disk, the compiled PDF equals the one produced
     by writing the buffer to disk and compiling `showcase.md` — the override
     reaches the compile.
  2. **The anchors are lines, not files.** `app/src/document.rs:Anchor` is
     `{ line, page }`, and the filter under test is precisely what drops
     `location.file`, so the clause is keyed to what survives it: with
     `edited = sections/mathematics.md` the anchor lines are that file's own
     heading lines, and with `edited = showcase.md` they are the master's. Two
     disjoint sets, checked both ways.
  3. `save` with `edited` set writes `sections/mathematics.md` and leaves
     `showcase.md` untouched, byte-for-byte.
  4. Setting `edited` while `buffer != saved` refuses, **names both ways out in a
     sentence that does not claim the file changed on disk**, and changes neither
     `edited` nor the buffer. The discard command then drops the buffer and the
     same switch succeeds. `Session::set_main` refuses on the same terms.
  5. An external write to `showcase.md` while `edited` is a section recompiles
     and does not run the divergence rule; an external write to the edited
     section runs it. With `main == edited` it is the second that happens, which
     is Phase 1's behaviour and is asserted so this phase cannot quietly move it.
  6. **`samples/article.md` — no sections, `main == edited` — compiles to the
     bytes `md2pdf_core::md_to_pdf` produces for it in the test itself.** "Equal
     to what it produced before this phase" has nothing committed to compare
     against: `tests/golden/` holds `.typ` and no PDF. Compiling it in the test
     is the reproducible form of the same claim, and is how
     `a_single_file_document_keeps_its_anchors_and_its_bytes` already makes it.

  At the window, on `tests/gates/mpdf-010-phase2.js`: open
  `samples/showcase/showcase.md` and click `sections/notes-and-sources.md` — **the
  last section the master names, and one with three headings**, so the pages its
  headings land on are the document's last and cannot be confused with page 1.
  Put the caret under its `## Citations` heading and **type a character**:
  `app/dist/index.html:caretPage` is consulted only on a status carrying a new
  `revision`, so a caret move alone scrolls nothing and the gate has to say what
  makes the redraw happen. The page then opens on the page that heading landed on
  **in the whole document**.

  **The gate then discards, and that is a clause rather than tidiness.** Typing
  the character leaves a dirty buffer in a tracked file, so the run ends by
  taking the discard the switch's refusal names — which both leaves the tree as
  it was found and exercises the second way out. Phase 1's gate made
  re-runnability an explicit property and this one keeps it.

- **Close-out:** `rules/desktop.md` (the state, the session, the watch's fourth
  answer, the two new commands, and the one `divergence` field now carrying two
  occasions), `rules/desktop-panes.md` — whose **"A row's body
  is inert and one button on it is not"** is Phase 1's own text and is the
  sentence this phase makes false, its `covers:` clause "the inert row and the
  one button on it" following it — and `rules/desktop-project.md` (the file the
  pane holds, beside the one that compiles). README: what `⌘S` writes, and what
  clicking a row does.

### Phase 3 — a file is created from the panel
*Produces the observable: **no**. A created file is empty and named by nothing —
the master learns about it only when the author types the marker, per §2 — so
no compile changes and no PDF differs. It is argued on the shape of the task
instead: `mpdf-008` made a document several files, and every one of them today
is created outside the app, which is the point at which the author leaves the
window and loses the preview. This phase is the smaller half of closing that;
Phase 2 is the larger half and produces the observable for both.*

- **Scope:** A command taking **one argument — a root-relative path, extension
  and all** — which confines it per §2's write rule, refuses a path that escapes
  the root or names an existing file, and creates it empty. `app/src/main.rs`
  registers it in `generate_handler!` and reads the root off the session, which
  is `app/src/main.rs:asset_bytes`'s own shape exactly; the rule is a plain
  function in `app/src/document.rs`, per that file's header.

  **The extension decides the kind, and there is no kind parameter.** An earlier
  draft took a path *and* a kind — `.md` or `.bib` — and review found the two
  cannot both be built: a kind that supplies the extension makes an extensionless
  name the normal input rather than the refusal the gate wants, and turns
  `notes.typ` into `notes.typ.md`. So the path carries its own extension and
  **`app/src/document.rs:kind_of` is what reads it**, the create being accepted
  exactly when that answers `Markdown` or `Bibliography`. This is §2's *"from the
  pipeline's own list"* argument applied a second time: `kind_of` already accepts
  a `.yml` and a `.yaml` bibliography, because `core/src/bibliography.rs` does,
  and a create that offered `.bib` alone would be the hand-written subset §2
  refuses — two lists that drift, and a panel that lists a file kind it cannot
  create. `Image` is refused with the rest: the panel does not make pictures.

  **`app/src/document.rs:confined` is the wrong helper and the create needs a
  sibling.** It opens `if !landed.is_file() { return None }`, so it refuses every
  path that does not yet exist — which is the create's entire input domain — and
  its own doc comment calls it *"the one confinement rule three commands share"*,
  so an implementer reading the code is invited to reuse it and would refuse
  every create. §2's write rule is the one that applies: **canonicalize the
  parent, which does exist, and join the final component to it**, refusing
  anything that does not land under the canonical root. A parent that does not
  canonicalize is a refusal too, which is how `newdir/x.md` is refused without a
  clause of its own — §1.2 makes folder creation a non-goal and this is the rule
  that keeps it one.

  **The write is not injected**, and the seam sentence an earlier draft carried
  is withdrawn: the injected seam in `app/src/document.rs` is a *read*, existing
  to count reads for a Phase 1 gate, and this file's only **write** —
  `app/src/document.rs:write_override` — is a plain `std::fs::write` tested
  against a real `scratch_dir`. §2's rule canonicalizes a real parent anyway, so
  an injected write would buy a seam the test still has to go around. This
  follows `write_override`: plain `std::fs`, checked against a scratch tree.

  **The name field lives in `#files` and outside `<ol id="parts">`, which is a
  correctness constraint rather than a layout preference.**
  `app/dist/index.html:parts` runs on *every* status — every compile, every
  typing settle, every `Tree` event, `.DS_Store` included — and does
  `list.replaceChildren(...)`, whose in-code comment states the invariant the
  shipped panel rests on: *"A row that remembered something of its own would make
  this wrong."* A field inside `#parts` is therefore wiped under the author's
  fingers by a filesystem event they did not cause. It goes beside the panel's
  own `<h2>Files</h2>`, is panel-level rather than per-row, and takes the whole
  root-relative path — which is what the window gate's `sections/discussion.md`
  needs, and what keeps `rules/desktop-panes.md`'s *"the rows hold no
  selection"* true.

  **Sitting outside `#parts` is also why the field must be cleared explicitly.**
  `app/dist/index.html:clear()` empties `#parts` and hides `#files`, and touches
  nothing else in the panel — so a path typed for one project and a refusal
  raised in it would both survive the Open into the next one, which is the same
  reasoning that put them there read backwards. `clear()` empties the field and
  its sentence with the rows.

  **A refusal is drawn beside that field, and both other routes are already
  refused in this spec.** `app/dist/index.html:fail` adds `stale` to the
  compiled page, which a gesture that compiled nothing must not do — that was
  Phase 5 round 1's blocking finding and is now written into
  `rules/desktop-panes.md` — and `Preview::divergence` draws a `Discard` button
  beside its sentence, which is meaningless for a create. So the sentence Rust
  returns is placed in the panel, where the gesture is and where the author is
  looking. This is the same exception Phase 5 took and for the same stated
  reason: a label about the panel's own action, not a status about the document.
  **Unlike `Session::set_edited`'s refusal, this one is reachable by typing**,
  so it is the first refusal in this panel a reader actually meets.

  **The created file does not go into the pane** — §2's *"Phase 3 creates the
  file and stops"*, stated here because it is otherwise an inference. It appears
  in the panel by the watch event Phase 1 already classifies: `Change::Tree`,
  which `app/src/watch.rs:classify` answers for any path under the root that is
  neither `main`, `edited` nor a named asset, and which
  `app/src/preview.rs:Session::on_change` turns into a listing refresh with no
  compile. The panel is not told about the create directly, so creation and an
  external `touch` reach the window by one path. **`Status` gains no field**, so
  `app/src/preview.rs:the_page_typedefs_name_exactly_the_fields_status_serializes`'s
  `declared.len() == 11` does not move — named because Phase 5 named its own
  knock-on and this phase's answer is that there isn't one.

- **Exit gate:** In the Rust suite, and **each clause says where it runs**,
  because this is the first phase of this spec whose gate writes:

  1. **Over a fresh `app/src/document.rs:scratch_dir`** — that helper's own doc
     comment is *"so runs do not collide and the repository stays clean"* — a
     create of `sections/note.md` under a root holding `sections/` produces that
     file, empty, **and no other path in the tree**, checked by walking the root
     before and after.
  2. **The confinement refusals, and the two sentences are distinguishable.**
     `/tmp/escape.md` and `../escape.md` are each refused **over a scratch root**,
     and the clause asserts the *outside-the-project* sentence rather than merely
     *a* sentence: a create's other refusal is *already exists*, and a clause
     gated only on "some sentence" passes on the wrong rule. **Neither path may
     exist on disk for this clause** — the mirror of Phase 5's clause 3 rather
     than a copy of it, since for a *create* an existing file would be refused
     by the exists-rule first and the confinement rule would never execute.
     Both, and not only `../escape.md`: a machine that happens to hold
     `/tmp/escape.md` would otherwise fail this clause for an environmental
     reason under an implementation that checks existence first, and requiring
     the absence of both is what makes it independent of that ordering. The
     clause also asserts no file appeared at the resolved path.
  3. **The symlink case runs over `tests/fixtures/panel/`**, which Phase 1 built:
     `outside/escape.md` through the committed `<fixture>/outside` link to
     `tests/fixtures/panel-decoy/` is refused. It runs there rather than over
     scratch because the link is already committed and resolves the same on any
     clone, **and because this clause writes nothing** — a refusal cannot dirty
     a tracked tree. **The fixture does not grow and
     `tests/fixtures/panel-manifest.txt` is not edited**, which is this phase's
     deliberate difference from Phase 5.
  4. **An existing path is refused without truncating it**, over scratch: the
     file is written with bytes first and its contents are compared after.
  5. **The extension rule**, which needs no filesystem at all past a root: a
     `.typ`, an extensionless name and a `.png` are each refused; a `.md`, a
     `.bib`, a `.yml` and a `.yaml` are each accepted. The last two are the
     clause that would catch a hand-written `.md`-or-`.bib` subset.

  At the window, on `tests/gates/mpdf-010-phase3.js`, **over a copy of the
  fixture made outside the repository** — `cp -R tests/fixtures/panel
  "$TMPDIR/mpdf-010-phase3"`, which the script's own header states as its one
  setup step. The fixture itself is fully tracked and is enumerated to eleven
  rows by `app/src/document.rs:the_listing_is_the_disk_and_what_the_master_names`,
  so creating a twelfth row in it would fail that test on the next `cargo test`;
  `samples/showcase/` is tracked too, and **there is no delete in this app until
  Phase 4**, so nothing in the gate could undo either. Remaking the copy is what
  makes the run re-runnable, which Phase 2's gate made an explicit property and
  this one keeps.

  Open `<copy>/sections/text.md`, which roots at `<copy>` per Phase 1's climb.
  Create `sections/discussion.md`: after `settled()` — the panel is refreshed by
  the watch and not by the command's return, so the row lands one
  `app/src/watch.rs:DEBOUNCE` later and "puts a row in the panel" read literally
  is a race — the panel holds a new row under the `sections` heading, in §2's
  order, `invoke('status')`'s `revision` and `edited` are **unchanged**, and the
  drawn page is the same one. Then create `../escape.md` and the refusal appears
  **beside the name field with the page not marked `stale`** and no `Discard`
  button, which is the clause the two refused routes exist to make checkable.

- **Close-out:** `rules/desktop.md` — the file I/O the app owns, and **two
  counted literals that move**: *"registers thirteen commands"* becomes fourteen
  and *"Twelve of the thirteen commands are wrappers over a plain function"*
  becomes thirteen of the fourteen. `rules/desktop-panes.md` — **this phase makes
  no sentence in it false**, which is the difference from Phases 2 and 5 and is
  worth saying: the row is unchanged, so *"A row carries two gestures and can
  carry two marks"* stands, and *"the rows hold no selection"* stands and is now
  load-bearing rather than incidental — it is the reason the field sits outside
  `#parts`, which that file should record. What moves is the `covers:` clause
  "the two gestures on a row and the two marks it may carry", which gains the
  panel's own field beside them, and a new paragraph for the field, its refusal
  and its clearing. README: the panel's actions.

### Phase 4 — a file is moved to the Trash
*Produces the observable: **no** — or rather it produces one only by breaking
the document, when the file deleted is one the master names and the next compile
refuses by that file's name. A refusal is not the observable this project
produces, so this phase claims none. Its argument is symmetry with Phase 3 and
nothing more: a panel that creates and cannot remove leaves the author in the
Finder for half the task, which is the state Phase 3 was written to end.*

- **Scope:** Depends on Phase 3 for a **precedent and not a prerequisite** —
  §4's preamble says none of these three phases is the other's, and that holds:
  Phase 3 could be cut and this one still built, because what it borrows is a
  shape, `app/src/document.rs:landing`, rather than a feature. Nothing outside
  this repository.

  **1. The rule is a third confinement question, and "Phase 3's, unchanged" was
  wrong.** Round 1 found that neither shipped rule serves a delete unchanged, and
  the correction is recorded here so it is not re-derived:
  `app/src/document.rs:confined` opens on `is_file()`, so it refuses a
  `missing: true` row and a dangling symlink the walk lists;
  `app/src/document.rs:landing` canonicalizes only the **parent**, so it accepts
  `secret.png` where that name is a symlink resolving out of the project — which
  `confined` refuses and
  `app/src/document.rs:a_link_out_of_the_project_is_refused_however_it_is_spelled`
  pins as a refusal for every other command.

  A delete wants both halves of a question neither asks: **the name is under the
  root, and something is at it.** So a `trash_file` in `app/src/document.rs` reuses
  `landing` for the first half — the same parent-canonicalizing rule, genuinely
  unchanged — and adds `symlink_metadata().is_ok()` for the second, which is
  `is_file()`'s answer widened to *anything at that name*.

  **That settles the symlink row, which was undecided.** `landing` answers with
  the join, so the delete acts on the **name and not the resolution**: a
  `cover.jpg` pointing at `figures/cover.jpg` trashes the link and leaves the
  figure. That is the opposite of `confined`'s reading for a read, and
  deliberately — a read wants the bytes the author meant, where a delete wants
  the row the author clicked, and the target of a link out of the project is not
  the project's to move. It is also what makes the widened existence test safe:
  a dangling link is a row the panel draws, and trashing it removes exactly the
  row.

  **2. The trash call, priced.** The two options are **the same call**, which the
  phase's own text did not know: `trash` 5.2.6's macOS implementation is
  `objc2_foundation::NSFileManager::trashItemAtURL_resultingItemURL_error`. So
  the pick is about packaging, and it goes to **`objc2-foundation` directly**.
  The measurement, recorded in the review record: it is already in `Cargo.lock`
  at 0.3.2 by way of `tauri`, so declaring it **adds no crate to the tree** —
  verbatim the argument `app/Cargo.toml`'s own comment records for the
  `serde_json` promotion in Phase 1 — where `trash` adds one compiled crate on
  macOS and 206 lines of `TrashItem`, listing and restore this app never calls.
  The features are `NSFileManager`, `NSURL`, `NSError` and `NSString`. The cost
  is stated rather than hidden: it is one more version this app pins itself
  rather than inherits, under `app/Cargo.toml`'s *"Every version is pinned"*.
  `trash`'s own MSRV of 1.85.0 and the workspace's edition 2024 on 1.97.1 were
  checked and neither route is blocked by the toolchain.

  **3. The OS call is injected, and that is deliberately not Phase 3's answer.**
  Phase 3 withdrew an injected write because §2's rule canonicalizes a real
  parent anyway and the write itself was a plain `std::fs::write` a scratch tree
  could check. Neither holds here: the call's whole effect is **outside the
  repository**, in the developer's own `~/.Trash`, which nothing cleans and
  `app/src/document.rs:scratch_dir`'s doc comment — *"so runs do not collide and
  the repository stays clean"* — has no reach over. So `trash_file` takes the
  call as a parameter, the way `app/src/document.rs:render_with` takes its read,
  and `app/src/main.rs` passes the real one. Every refusal below is then
  reachable with nothing entering the Trash, and the one claim that needs a real
  Trash is the window's. **The double removes the file as well as recording the
  call**: gate clauses 1, 3 and 7 each turn on the file actually being gone, and
  one that only counted would pass all three over an unchanged tree.

  **4. The command and the gesture.** **The template is
  `app/src/preview.rs:Session::set_edited`'s and not
  `app/src/main.rs:asset_bytes`'s**, which round 2 corrected: a delete must also
  refuse on `main`, set `Preview::divergence`, call `Session::arm` and write
  `Preview::tree` — and `arm`, `tree`, `divergence` and `on_render` are all
  private to `app/src/preview.rs` and unreachable from `app/src/main.rs`. So the
  part that touches the session is a `pub fn` on `Session`, the confinement and
  the OS call are the plain function in `app/src/document.rs`, and the
  `#[tauri::command]` is thin over the first — registered in
  `generate_handler!`. The compiler forces this eventually; naming it here saves
  a planner one wrong structure.

  The gesture is a **third control on the row**, drawn like
  `app/dist/index.html`'s `.set`: revealed on hover and on focus, carrying the
  entry's own `path`. **OQ-5's `margin-left: auto` argument was written for one
  control and now has to seat two** — a non-main markdown row draws `main` and
  this one together — so the two share that edge inside a group rather than each
  claiming it, and the `◀ main` mark keeps it alone on the row that has no
  `main` button. Named because it is the one place this phase touches CSS whose
  reasoning is already recorded against a smaller case. It holds no state — there is no confirmation, per §2's
  correction — so `app/dist/index.html:parts` may go on rebuilding the panel
  whole and *"the rows hold no selection"* stays true. **A `main` row and a
  `missing: true` row get none**: the first is refused below, and the second
  names a file the disk does not hold, so there is nothing to move.

  **5. The three cases, and the fourth the panel needs.**

  - **Deleting `main` is refused outright** while it is main, in a sentence of
    its own.
  - **Deleting `edited`** refuses first while the buffer diverges from `saved`,
    which is §2's switch rule and the same hazard. It needs **a sentence of its
    own**, not `app/src/preview.rs:SWITCHING`, which opens *"the pane holds
    unsaved edits, so it is still holding this file. Save to keep them, or
    discard them to open the other file"* — nothing is being opened by a delete,
    so reusing it would put a lie in the window, verbatim §2's argument for why
    `SWITCHING` is not `DIVERGED`. It rides `app/src/preview.rs:Preview`'s
    `divergence` and returns `Ok(())`, as `Session::refused_while_dirty` does, so
    one refusal does not arrive in the window two ways.

    Over a clean buffer the fallback is
    **`app/src/preview.rs:Session::set_edited`'s own body and not a field
    write**: set `edited`, `Preview::load` it, announce, and re-arm. **Both
    halves are load-bearing and round 2 found the second missing.**
    `Session::arm` is what keeps the loops alive — Phase 2's scope item 4
    records that both guard on a path captured when they were started, so a
    command that moves `edited` and stops leaves the typing debounce compiling
    nothing and every filesystem event dropped. But **arming without loading is
    worse than not arming**: the buffer still holds the *trashed* file's text
    while `edited` now names `main`, so `app/src/preview.rs:Preview::save`
    writes a deleted section over the master, and
    `app/src/document.rs:render_project`'s closure answers `main` from that same
    buffer once the two paths are equal — the next compile draws the deleted
    section as the whole document. Nothing announces either, so the window never
    shows it happen.
  - **Deleting a file the master names is allowed**, and the next compile
    refuses. **Not with `md2pdf_core::Error::MissingSection`**, which this
    phase's text claimed and which this app never reaches:
    `app/src/document.rs:read_sections_with` fails first with *"cannot read
    {path} for the section {location}"* and `?` propagates before
    `core/src/sections.rs:section_text` — the only place `MissingSection` is
    raised — is called at all. The recoverable behaviour is the same; the
    sentence is the app's, and the gate below is keyed to the sentence that
    exists.
  - **The panel is refreshed by the command and not by the watch**, which is
    this phase's one departure from Phase 3 and is forced.
    `app/src/watch.rs:classify` answers the **first** match, and a section the
    master names is already in the asset list `app/src/document.rs:render_with`
    builds — so deleting it answers `Change::Asset`, never `Change::Tree`, and
    `app/src/preview.rs:Session::on_change` refreshes `preview.tree` only under
    `changed.tree`. The panel would keep an ordinary unmarked row for a file that
    is gone. **The asymmetry with Phase 3 is real rather than an inconsistency**:
    a created file is not in the asset list, so the watch classifies it
    correctly; a deleted section is, so it cannot. The delete therefore re-walks
    with `app/src/document.rs:files_under` itself, which it may because the app
    made the change and knows it — the watch is for changes the app did not make.
    `app/src/document.rs:merge` then adds the path back as `missing: true`,
    because the master still names it, which is the row §2's union exists to
    show.

  **`Status` gains no field**, so
  `app/src/preview.rs:the_page_typedefs_name_exactly_the_fields_status_serializes`'s
  `declared.len() == 11` does not move — named because Phases 3 and 5 named their
  own knock-on and this phase's answer is again that there isn't one.

- **Exit gate:** In the Rust suite, and **each clause says where it runs**, per
  Phase 3's standard. The injected call means every clause here writes nothing to
  any Trash: the double records what it was asked to move, and removes it.

  1. **The happy path, over `app/src/document.rs:scratch_dir`.** Trashing
     `sections/note.md` calls the double exactly once with the path `landing`
     answers — the join, `<root>/sections/note.md` — and the walk
     before and after differs by exactly that entry — the shape of Phase 3's
     clause 1, run backwards.
  2. **Confinement, and the clause is the *mirror* of Phase 3's rather than a
     copy.** Phase 3 required `../escape.md` to be **absent**, because for a
     create an existing file is refused by the exists-rule first; for a delete
     that is inverted, so this clause **writes `escape.md` in the scratch root's
     parent first** — otherwise the not-there rule refuses and the confinement
     rule never executes. It asserts the outside-the-project sentence
     specifically, that the double was **not** called, and that the file is still
     there afterwards.
  3. **A link out of the project trashes the link and not its target**, over
     scratch: a `secret.png` in the root pointing at a file outside it is
     accepted — `landing` confines the *name* — the double is called with the
     root's own path, and the target still exists. This is the clause that
     distinguishes this rule from `confined`, which refuses the same row.
  4. **Nothing at that name is refused**, in a sentence of its own, with the
     double uncalled. **One sentence and not two**, which round 2 corrected:
     `merge` marks a row missing *because* the walk did not find it, so a
     `missing: true` path and a plain absent one are the same
     `symlink_metadata()` failure inside a function that never sees
     `Preview::sections` — a clause asking for two would be asking for a
     distinction the rule cannot make. Both inputs are still run, since what is
     pinned is that the panel's marked-missing row is not a special case.
  5. **`main` is refused** while it is main, and the double is uncalled.
  6. **`edited` with a dirty buffer is refused in a sentence that does not claim
     a file is being opened**, and neither `edited` nor the buffer moves. With a
     clean buffer, **three things and not one**: `edited` becomes `main`; the
     pane's `buffer` and `saved` are `main`'s **own text**, byte-for-byte, which
     is what says the file was re-read rather than the field assigned; and both
     loops are live afterwards, asserted the way Phase 2's clause 5 asserts them
     — a subsequent external write still runs the rule and a subsequent edit
     still compiles. **The middle one is the clause, and round 2 found it
     missing.** Liveness alone passes a build that arms without loading, and
     that build writes the deleted file's text over the master on the next
     `⌘S`.
  7. **A named section, over a scratch copy of `tests/fixtures/panel/`** — not
     the committed fixture, which Phase 3's gate could use because a refusal
     writes nothing and a delete does not have that property: trashing
     `sections/text.md` leaves `merge`'s listing holding it as `missing: true`,
     and the next compile fails with the *"cannot read … for the section …"*
     sentence, matched on that text and not on `MissingSection`.

  At the window, on `tests/gates/mpdf-010-phase4.js`, **over a copy of the
  fixture made outside the repository**, exactly as Phase 3's gate is set up and
  for a reason this phase makes sharper: this is the app's first destructive
  operation, and `tests/fixtures/panel/` is tracked. The claim no unit test can
  make, because the call is injected everywhere else and a double proves nothing
  about the OS: **the trashed file is in the Trash and Put Back returns it to the
  project**, checked by eye on macOS 26.5.2 and recorded in the review record. A
  call that silently unlinked would pass every assertion above.

- **Close-out:** `rules/desktop.md` — the file I/O the app owns, the dependency
  the bundle gains, and **two counted literals that move**: *"registers fourteen
  commands"* becomes fifteen and *"Thirteen of the fourteen commands are wrappers
  over a plain function"* becomes fourteen of the fifteen.
  `rules/desktop-project.md` — the third confinement question and the row a
  delete acts on by name. `rules/desktop-panes.md` — the row's **third** gesture,
  its `covers:` clause "the two gestures on a row and the two marks it may carry"
  following it. **Both caps are at their limit and the close-out says so rather
  than discovering it**: `rules/desktop-panes.md` is 379 body lines against
  `max_lines: 380` and `rules/desktop.md` 556 against 560, so each either trims or
  moves its cap in this phase's own commit, with the reason stated. README: that
  deletion is to the Trash, which is the kind of promise a user reads before they
  trust it, and that nothing asks twice.

### Phase 5 — an image row shows the figure
*Produces the observable: **no**. Nothing here compiles, no PDF differs, and the
pane goes on holding the file it held. The argument is the panel's own, made
about a different half of it: `mpdf-008` made a document several files **and its
figures several files**, and the panel has listed those figures since Phase 1
while being the one thing in the window that could not show one. Checking that
`emit.svg` is the diagram you meant means leaving for Preview and losing the
pane — the same complaint §1 opens with, about pictures rather than prose. It is
the smallest of the three phases that show nothing, and it is the one an author
meets most often.*

- **Scope:** Depends on Phase 2 — the panel's rows must already be clickable and
  `edited` must already be a thing a row can move — and on nothing outside this
  repository. **OQ-1 settles the shape and the prototype settled the mechanism**;
  both are recorded there, and this scope is the buildable form of them.

  1. **The read is an ordinary function, and the command is a wrapper over it.**
     `app/src/document.rs` gains one taking the root and a root-relative path and
     answering `Result<Vec<u8>, String>`; `app/src/main.rs` gains the
     `#[tauri::command]` that calls it, registers it in `generate_handler!` and
     returns `tauri::ipc::Response`. **The split is not tidiness and the gate
     below depends on it**: `app/src/main.rs` has no test module, the crate is
     bin-only, and `tauri::State` has a private field and no public
     constructor — so a rule written into the command is a rule no test in this
     repository can reach, which is that file's own header stated backwards.
     Eleven of this app's twelve commands are already wrappers over a plain
     function — `app/src/main.rs:pending_open` is the one that is not — and this
     is a twelfth.

     The function **confines rather than merely checking existence**, exactly as
     `app/src/preview.rs:Session::set_main` does: `document::relative` must
     answer the path back, which `root.join("../../secrets.png")` cannot.

     The `tauri::ipc::Response` is `app/src/main.rs:current_pdf`'s own route: a
     `Vec<u8>` would serialize as a JSON array of numbers, one per byte, and a
     figure is bigger than a page. **It needs no capability and no
     `app/tauri.conf.json` change** — Tauri's asset protocol would want both, and
     an app-defined command wants neither.

  2. **A surface over the text pane, and not a third pane.** The page turns the
     bytes into a `Blob` and an object URL, draws an `<img>`, and **revokes the
     previous URL**, or the window holds every figure the reader has looked at
     for its own lifetime. `app/dist/index.html:fileRow` is what changes to reach
     it: its `opens` test is
     `entry.kind === 'markdown' && !entry.missing && !holding` today, so an image
     row's label is a `<span>` carrying the path's last segment with
     *"— not edited here"* in the row's `title`, and it becomes a second kind of
     button beside the one that opens a markdown row.

     It is **absolutely positioned over `#text`'s own column rather than
     replacing it**: `app/dist/index.html`'s divider drag reads
     `text.getBoundingClientRect().left` on every `pointerdown`, and a hidden
     textarea measures zero, so replacing it would break the drag for as long as
     a figure was up. `<main>` therefore takes `position: relative` — it carries
     no `position` today, and `#pages` already has its own, so no existing
     `offsetTop` reader moves — and the surface mirrors `#text`'s `offsetLeft`
     and `offsetWidth` **whenever that column can move**: a show, a window
     resize, the end of a divider drag, the panel fold (`#toggle`) and the line
     gutter (`#numbers`). The last two are the ones an enumeration drops, and
     both shift `#text`'s left edge. **A `<figure>` carries `margin: 1em 40px`
     from the user agent**, which puts it 40 px off that column; the prototype
     hit it and it is written here so the next reader does not.

  3. **The figure sits in the upper third**, not centred and not against the
     top: centred, a small figure lands at the pane's waist and reads as
     nothing; flush to the top it reads as a header. The sheet holding it is a
     flex child of the surface with `flex: 1; min-height: 0`, which is what makes
     the padding above the figure free: **flexbox distributes the free space over
     items' *outer* sizes**, so the padding is already accounted for, the sheet
     cannot overflow the surface, and the image's `max-height: 100%` — resolving
     against a content box that already excludes it — measures the right number.

     **`box-sizing: border-box` is not what makes that work, and two drafts said
     it was.** The first justified the padding by it; review called that
     conditional on a specified height; the second answered that `flex: 1` makes
     the height definite, which is true and is not the point. Measured in
     Chromium over a 400 px column with a 100 px top padding, a `flex: 1` sheet
     is 400 px and overflows by 0 under **both** values of `box-sizing`, and the
     figure lands identically. The overflow-by-exactly-the-padding behaviour is
     real and belongs to `height: 100%`, which is not what this builds. Recorded
     because it is a justification a later pass would otherwise re-derive and
     find false — the declaration itself is harmless either way, and gate clause
     6 checks the fit by measuring rather than through this reasoning.

  4. **The pane goes on holding its file, and that is the decision the rest
     rests on.** Nothing here touches `edited`, the buffer, the compile, the
     bytes or the anchors: `⌘S` still writes the markdown, the page still shows
     the whole document, and `app/src/preview.rs:Status` gains **no field**. It
     is a view, the way `Lines` is a view. Three ways back to the text, because
     the reader arrives by three routes: the surface's own control, `Escape`,
     and clicking any markdown row **that opens one** — which already means *put
     that file in the pane* and must not leave a picture over it.
     `app/dist/index.html:clear()` closes it too, an open being a new project.

     **The row the pane already holds is inert and stays inert**, per `opens`'s
     `!holding` term above, so clicking *it* while a figure is up does nothing —
     which is the one gesture a reader might reasonably expect to work and does
     not. Accepted rather than fixed: the alternative is a row whose drawing
     depends on page state, and `rules/desktop-panes.md`'s **"the rows hold no
     selection"** is what lets the panel be rebuilt whole on every status. The
     surface's own control and `Escape` both cover that reader, so nobody is
     stuck.

  5. **A PDF row says so and draws nothing**, per OQ-8, **and that sentence is
     the page's own** — which is a deliberate exception to the rule that the
     window composes no text, and the reason is that neither route for a Rust
     one is available. `app/dist/index.html:fail` is where a refusal from a
     command lands, and it runs `pages.classList.add('stale')`: a click that
     compiled nothing would mark the compiled page out of date, contradicting
     item 4 above. The `divergence` field Phase 2 widened draws the `Discard`
     button beside its sentence, which is wrong here. And a field on `Status` is
     what item 4 refuses. **So the page reads the extension and writes the line**,
     the way it writes `Back to the text` — this is a label for a file kind and
     not a status about the document, which is the distinction that rule was
     always about. The command is never called for a `.pdf` at all.

- **Exit gate:** In the Rust suite, over `tests/fixtures/panel/`, which Phase 1
  created:
  1. The function returns `cover.jpg`'s bytes and `sections/mark.svg`'s bytes,
     each byte-for-byte against `std::fs::read` of the same file.
  2. It refuses `/tmp/escape.png` and `outside/decoy.png` — the second through
     `<fixture>/outside`, the committed symlink to `tests/fixtures/panel-decoy/`
     that Phase 1's clause 5 already uses, which is the case that exercises
     confinement against a path the disk really holds.
  3. It refuses `../escape.png` **over a scratch root**, not over the fixture:
     the test makes a directory under `app/src/document.rs:scratch_dir`, writes
     `escape.png` in its *parent*, and asks the function for `../escape.png` from
     the root below. The write is what makes the case worth running — `is_file()`
     would otherwise refuse first and the clause would pass without the
     confinement rule ever executing — and the scratch root is where it goes,
     because `tests/fixtures/` is tracked and that helper's own doc comment says
     it exists so the repository stays clean. `Session::set_main`'s test writes
     to a scratch parent for the same reason.

  **The fixture gains one file, and it costs two edits to Phase 1's work.**
  `tests/fixtures/panel/plan.pdf`, so the PDF case is reachable at all: `pdf` is
  in `md2pdf_core::IMAGE_EXTENSIONS`, and without it the only PDF in reach is
  `samples/showcase/showcase.pdf`, which `.gitignore` excludes — so a second
  person on a fresh clone would check nothing, which is verbatim the
  irreproducibility Phase 1 refused to gate on. It slots between `other.md` and
  `refs.bib` by §2's own order, and **`tests/fixtures/panel-manifest.txt` and
  `app/src/document.rs:the_listing_is_the_disk_and_what_the_master_names` each
  gain that one row in this phase's own commit.** An exact-enumeration gate
  failing when its fixture grows is that gate working; it is named here so the
  failure is expected rather than investigated.

  At the window, on `tests/gates/mpdf-010-phase5.js`, opening
  `tests/fixtures/panel/sections/text.md` — which roots at the fixture, per
  Phase 1 — and then `samples/showcase/showcase.md`:
  1. Clicking `sections/mark.svg` shows it over the text pane, in that pane's
     own column: the surface's left and width equal `#text`'s, read off the live
     DOM.
  2. `invoke('status')`'s `edited`, `main` and `revision` are **unchanged across
     the click**, which is what "it is a view" means as an assertion, and the
     pane's text is the same string it was.
  3. Clicking `plan.pdf` draws no `<img>` and shows the sentence instead.
  4. The divider still drags while a figure is up, and the surface follows it;
     so do the `Files` fold and the `Lines` toggle.
  5. `Escape` puts the text back. **Then the figure is shown again**, and
     clicking a markdown row puts the text back *and* moves the pane — two
     claims, and the re-show is what makes the second reachable after the first.
  6. In `samples/showcase/`, `sections/emit.svg` — **`viewBox="0 0 120 72"`,
     where `mark.svg` is `viewBox="0 0 16 16"` and would prove nothing about the
     fit** — is drawn no wider than the sheet and no taller than it.

- **Close-out:** `rules/desktop.md` (the commands, now thirteen, and the file
  I/O the app owns — this is one more reader of a path the author did not name
  in a dialog, beside the walk, the compile's own closure and
  `Session::set_edited`). `rules/desktop-panes.md`: the surface and the three
  ways back, and **the sentence this phase makes false is *"A bibliography, an
  image and a marked-missing row open nothing and say so in their `title`, where
  OQ-1 and OQ-2 leave them"***, with its `covers:` clause "the two gestures on a
  row and the two marks it may carry" following it — the shape Phase 2's own
  close-out set. README: that clicking a figure shows it and the pane keeps its
  file. **`specs/file_panel_spec.md` OQ-1 is already resolved** — this phase
  ships what that resolution names and adds nothing to it.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-010.md, append-only, one heading per round. See §7 of
spec-authoring.md. `/review-spec specs/file_panel_spec.md --phase 1` opens it.
-->
