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
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — a file is created from the panel"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — a file is moved to the Trash"
    reviewed: null
    shipped: null
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
   `app/src/watch.rs:Changed` gains one `bool` per answer, and
   `app/src/preview.rs:Session::classifier` and `Session::on_change` close over
   the document alone today, so both take the root beside it.

3. **The save.** `app/src/preview.rs:Preview::save` writes to `document`; it
   writes to `edited`. `app/src/preview.rs:Preview`'s `saved` field follows
   `edited` for the same reason.

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
out, and takes neither** — which is `app/src/preview.rs:DIVERGED`'s shape,
already in this app and already reviewed, applied to a second occasion. Save, or
discard, and the author says which.

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

### Delete moves to the Trash (decision, recorded)

Not `std::fs::remove_file`. **There is no undo anywhere in this app** — not for
an edit, not for a save, not for an export — and the Trash is the platform's own
undo for exactly this operation. `tauri-plugin-fs` offers no trash call and
neither does `std`, so this costs a dependency: the `trash` crate, or an
`NSFileManager trashItemAtURL` call through `objc2`. Phase 4 prices both and
picks one; the decision recorded here is only that a plain unlink is refused.

## 3. Open questions

- **OQ-1 — what does clicking an image row do?** *(design call)* Three shapes:
  inert, which is what Phase 2 ships; a preview, which needs a second surface in
  a window that has two panes already; or **insert `![](figures/overview.svg)` at
  the caret**, which is the one that would also close the cost the create
  decision above accepts, since a new section and a new figure both want their
  marker placed by the author. It wants the resolved path written relative to the
  *edited* file rather than the root, per `mpdf-008` §2's section-relative rule —
  and what an image *above* the edited file may be written as depends on
  `mpdf-008` Phase 5, which is **drafted and neither reviewed nor shipped**. That
  is a second reason this stays open rather than becoming a phase here.
  **Blocks nothing** — Phase 2 leaves image rows inert and says so in the window.

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

- **OQ-5 — what is the gesture that sets main?** *(design call)* **Only the
  gesture is open.** §2 settles what `main` *is* in every case — stored override,
  one master, none, or several — and Phase 1's gate clause 3 pins each of those
  four to a value, so an implementer needs nothing from this entry. What is left
  is whether the author changes it by a row action, a menu item or something in
  the status area, and whether the affordance appears at all when discovery found
  exactly one master. **Blocks nothing**, and it is left to the prototype per the
  standing practice that a UI shape is tried in the running app before it is
  specced.

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

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. **Two of the four produce the
observable and two do not**, which is a higher ratio of scaffolding than any spec
here has carried, and it is a property of the subject rather than an oversight: a
panel is *about* the material the observable is made from. The two that produce
none are argued in place, and neither is a prerequisite of the other — Phase 3
and Phase 4 could each be cut without touching what Phases 1 and 2 deliver, which
is the honest test of a phase that shows nothing.

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
  stops tracking `main`.

  `app/src/document.rs:render_with`'s closure carries the override: the caller
  passes one that answers `root.join(edited)` from the pane's buffer and
  delegates every other path to `std::fs::read`. **`core` is not touched**, per
  §2. `app/src/preview.rs:Preview::compile` builds it.

  `app/src/document.rs:render_with`'s anchor filter takes the edited path and
  keeps the anchors whose `location.file` matches it, `None` meaning main.
  `app/src/preview.rs:Preview::save` writes `edited`, and `saved` follows it.
  `app/src/watch.rs:classify` gains §2's **`Edited`** answer here — Phase 1 added
  only `Tree` — tested **before** `Asset`, since a section the master names is
  already in the asset list and would otherwise recompile silently instead of
  reaching `app/src/preview.rs:external_change`. `Changed` gains its bool beside
  the other three.

  The switch: a new command sets `edited`, refusing with `DIVERGED`'s two-ways-out
  sentence while `buffer != saved`, per §2. `app/dist/index.html` places that
  refusal exactly as it places every other status sentence, composing none of it.

- **Exit gate:** In the Rust suite:
  1. With `main = showcase.md` and `edited = sections/mathematics.md` and a
     buffer differing from that file on disk, the compiled PDF equals the one
     produced by writing the buffer to disk and compiling `showcase.md` — the
     override reaches the compile.
  2. The same compile's anchors all carry `sections/mathematics.md`, and none
     carry `None`; with `edited = showcase.md` the reverse.
  3. `save` with `edited` set writes `sections/mathematics.md` and leaves
     `showcase.md` untouched, byte-for-byte.
  4. Setting `edited` while `buffer != saved` refuses, names both ways out, and
     changes neither `edited` nor the buffer.
  5. An external write to `showcase.md` while `edited` is a section recompiles
     and does not run the divergence rule; an external write to the edited
     section runs it.
  6. `samples/article.md` — no sections, `main == edited` — produces a PDF equal
     byte-for-byte to the one it produces before this phase.

  At the window, on `tests/gates/mpdf-010-phase2.js`: with the pane holding
  `sections/mathematics.md`, moving the caret to its last heading scrolls the
  page to the page that heading landed on in the whole document.

- **Close-out:** `rules/desktop.md` and `rules/desktop-panes.md` — the latter's
  "the rows do not load" passage and the four things it names are the text this
  phase makes false, and it is replaced rather than annotated, rules being the
  artifact that tracks the code. README: what `⌘S` writes.

### Phase 3 — a file is created from the panel
*Produces the observable: **no**. A created file is empty and named by nothing —
the master learns about it only when the author types the marker, per §2 — so
no compile changes and no PDF differs. It is argued on the shape of the task
instead: `mpdf-008` made a document several files, and every one of them today
is created outside the app, which is the point at which the author leaves the
window and loses the preview. This phase is the smaller half of closing that;
Phase 2 is the larger half and produces the observable for both.*

- **Scope:** A command taking a path relative to the root and a kind — `.md` or
  `.bib` — which resolves and canonicalizes per §2's confinement rule, refuses a
  path that escapes the root or names an existing file, and creates it empty.
  Ordinary function in `app/src/document.rs` with the write injected, matching
  that file's existing seam. **Not every refusal is reachable without a
  filesystem**, and the phase should not claim otherwise: §2's write rule
  canonicalizes the parent, which is a real call on a real directory, and the
  symlink case in the gate below needs a real link. The seam keeps the
  *write* out of the test; the resolution is checked against a temporary
  directory the test creates.

  The panel gains the gesture and the name field. A created file appears in the
  panel by the watch event Phase 1 already classifies — the panel is not told
  about it directly, so creation and an external `touch` reach the window by one
  path.

- **Exit gate:** In the Rust suite: a create under the root produces the file and
  no other; `../escape.md`, `/tmp/escape.md`, and a path through a symlink
  leaving the root are each refused with a sentence naming what was asked for;
  an existing path is refused without truncating it; a `.typ` or extensionless
  name is refused. At the window: creating `sections/discussion.md` puts a row in
  the panel and leaves the page unchanged.

- **Close-out:** `rules/desktop.md` (the commands, the file I/O the app owns),
  `rules/desktop-panes.md` (the panel's gestures). README: the panel's actions.

### Phase 4 — a file is moved to the Trash
*Produces the observable: **no** — or rather it produces one only by breaking
the document, when the file deleted is one the master names and the next compile
refuses with `MissingSection`. A refusal is not the observable this project
produces, so this phase claims none. Its argument is symmetry with Phase 3 and
nothing more: a panel that creates and cannot remove leaves the author in the
Finder for half the task, which is the state Phase 3 was written to end.*

- **Scope:** Depends on Phase 3's confinement rule and reuses it unchanged. The
  trash call is the one open cost: `trash` (the crate) and an
  `NSFileManager trashItemAtURL` call through `objc2` are both priced in the
  plan-mode pass — dependency weight, what it adds to the bundle, and whether it
  builds under the workspace's pinned toolchain — and one is picked with the
  measurement recorded in the review record.

  **A plain unlink is refused**, per §2.

  Three cases get stated rules rather than inherited behaviour: deleting the
  **edited** file leaves the pane holding a buffer whose path is gone, and the
  answer is that `edited` falls back to `main` and the buffer is discarded only
  if it is clean — a dirty buffer refuses the delete, per §2's switch rule, since
  the two are the same hazard; deleting **main** is refused outright while it is
  main; deleting a file the master **names** is allowed, and the next compile
  refuses with `MissingSection`, which is the existing, recoverable behaviour and
  is what the marked-missing row in §2's union exists to show.

  A confirmation precedes every delete.

- **Exit gate:** In the Rust suite: the confinement refusals of Phase 3, re-run
  against delete; deleting main is refused; deleting the edited file with a dirty
  buffer is refused and with a clean buffer moves `edited` to `main`; deleting a
  named section leaves the panel holding a marked-missing row and the next
  compile refusing by name. And the one claim no unit test can make, run at the
  window and recorded in the review record: **the deleted file is in the Trash
  and can be put back**, checked by eye on macOS 26.5.2, because a call that
  silently unlinked would pass every assertion above.

- **Close-out:** `rules/desktop.md` (the file I/O the app owns, and the
  dependency the bundle gains), `rules/desktop-panes.md` (the panel's gestures).
  README: that deletion is to the Trash, which is the kind of promise a user
  reads before they trust it.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-010.md, append-only, one heading per round. See §7 of
spec-authoring.md. `/review-spec specs/file_panel_spec.md --phase 1` opens it.
-->
