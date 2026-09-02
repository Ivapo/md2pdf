---
id: mpdf-011
title: repository-split
note: >
  The engine and the desktop app part ways: `md2pdf-core` and its CLI stay in this
  repository and are published as crates, Letur moves to a repository of its own with
  its history, its page and its gates, and depends on the engine by version.
status: accepted
last_updated: 2026-09-02

phases:
  - name: "Phase 1 — Letur's repository exists, with its history"
    reviewed: 2026-09-02
    shipped: 2026-09-02
    cut: null
    by: null
  - name: "Phase 2 — the engine stands alone"
    reviewed: 2026-09-02
    shipped: 2026-09-02
    cut: null
    by: null
  - name: "Phase 3 — the engine is published, and Letur depends on it by version"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-003, mpdf-006, mpdf-009, mpdf-010]
reference: >
  Typst's own layout is the model: one repository publishes `typst`, `typst-library`
  and `typst-pdf` as crates from a single workspace with the CLI beside them, and the
  web app is a separate product that depends on those crates. This project's engine
  already mirrors the first half — `core` and `cli` in one workspace, `typst` pinned as
  a dependency — and this spec gives Letur the second. What is out of scope from that
  model is everything the web app is: a server, accounts, collaboration. `mpdf-001` §2
  keeps this engine local and fetching nothing, and Letur inherits that.
---

# repository split

## 1. Goal

**Let the dialect be a crate, and let Letur be a product.** Today one repository holds
both, one workspace builds both, and one version number is claimed by three things:
`core/Cargo.toml`, `cli/Cargo.toml` and `app/Cargo.toml` all take `version.workspace =
true`, currently `0.1.0`, while `app/tauri.conf.json` hardcodes its own `"version":
"0.1.0"` for the bundle. A library version means *this API*; a product version means
*this release*. The moment either ships, the shared number is wrong, and it cannot be
made right inside one workspace.

The dialect is what this project set out to build, and it is built: every phase of the
six engine specs is shipped. What remains is to give it the shape a finished library has
— a crate on the registry, a repository that is about the dialect and nothing else — and
to give Letur the shape a product has: its own repository, its own version, its own
release machinery, and the engine as a dependency rather than a sibling.

**The observable is unchanged — the typeset PDF that Typst compiles from the user's
markdown — and this spec produces no new instance of it.** §1.1 argues why a spec that
produces no PDF still earns its place, and every phase in §4 is gated on the observable
*not moving*: the same document hashes to the same bytes before and after each step.

After this spec the consumer of the engine sees this:

```
$ cargo install md2pdf-cli
$ md2pdf paper.md                     # a binary called md2pdf, from a crate called md2pdf-cli

[dependencies]
md2pdf-core = "0.1"                   # what Letur, or anyone, writes
```

and the consumer of Letur sees a repository whose README opens with the window rather
than with a dialect, whose releases carry a `.dmg`, and whose only mention of `core/` is a
version in `Cargo.toml`.

### 1.1 Why this is a new spec, and why it produces no observable

The methodology's §6.1 is an ordered test and it is worked in full.

- **Step 0 — a decision, or only the code?** A decision, and a large one: which
  repository owns what, how the two version, and how one depends on the other. None of
  it is derivable from the code.
- **Step 1 — does it remove or contradict shipped work?** **It moves shipped work and
  contradicts one decision statement.** Nothing shipped is un-built: every phase of
  `mpdf-003`, `mpdf-006`, `mpdf-009` and `mpdf-010` keeps its id, its date and its code,
  in a repository that carries their history. What it contradicts is `mpdf-006` §1's
  opening — *"Give the dialect a front door that shows itself"* — because after this
  spec the page is Letur's, and the engine's front door is its README and its registry
  entry. That is shipped prose made misleading rather than a phase removed, so §6.1's
  rule is a dated `CORRECTED` note beside that sentence in `mpdf-006` §1, not a
  supersession; Phase 1 writes it, in Letur's copy, which is the one that lives on. The
  frontmatter `note:` — *"the published browser demo becomes the project's front
  door"* — is the index line and cannot hold a note; it is left as the record of what
  was decided.
- **Step 2 — is its subject one an existing spec owns?** No. No spec owns the
  repository, and the methodology's own words settle it: *"a cross-cutting feature still
  gets its own spec — if the work spans several subsystems and its unifying thread is a
  goal rather than a subject."* This spans every subsystem, and its thread is a goal.
- **Step 3 is not reached.**

**Why a spec that produces no PDF.** §3 of the methodology asks every phase whether it
produces the observable and requires the ones that do not to argue it. The argument here
is that the observable is the thing at risk: a split done by hand, path by path, is
exactly the kind of change that moves a byte nobody sees — a font that stops being found,
a fixture read from the wrong root, a look whose bytes the crate no longer embeds. So the
observable is the *gate* of every phase rather than its product. The precedent is the
corpus's own: `rules/pipeline.md` records `figures` and `headings` measured as
byte-identical PDFs across the trees either side of a phase, and `mpdf-007` Phase 5 held
its default the same way. **The identity is sound to key a gate to**, measured in round
1: `core/src/lib.rs:render` compiles with `PdfOptions::default()`, whose timestamp is
`None` and whose document id is `Auto`, nothing in `core/src/emit.rs` or the looks sets a
date, and `core/src/lib.rs:today` returns `None`; `cargo update --workspace --dry-run`
moves nothing, so a lock seeded from the engine's resolves the same tree.

### 1.2 Non-goals

- **Not the web version of Letur.** `web/` moves to Letur's repository as it is, and
  keeps working as it is. What it becomes — a landing site, a browser build of the app
  behind the same `window.__TAURI__` seam the harness already stubs — is a spec of its
  own, drafted after the two repositories exist. §2 records what this spec does to keep
  that move cheap and what it deliberately leaves alone.
- **Not a rename.** The registry name `md2pdf` is held by an unrelated project
  (`0.0.3`, last published 2022-10-02, a `tectonic` wrapper), measured 2026-09-01. The
  library publishes as `md2pdf-core` and the tool as `md2pdf-cli`, the names the crates
  already carry, and the binary stays `md2pdf`. A rename of the engine was weighed and
  refused: it would move every spec, rule, README and the `CLAUDE.md` observable
  sentence for a name that is only ever typed after `cargo install`.
- **Not signing, packaging or distribution.** `mpdf-003` OQ-8 owns that question and it
  stays Letur's. This spec gives it a repository where signing secrets can live without
  sitting beside a public library's publish token, and nothing more. **No secret is
  needed for anything in this spec**: the Pages deploy runs on the workflow's own
  identity token, and the engine is public, so a git dependency resolves without one.
- **Not a change to what the dialect does.** No construct, key, refusal or look moves.
  Every golden is byte-identical, and that is a gate rather than a hope.
- **Not a monorepo tool.** No submodule, no subtree, no workspace-of-workspaces. Letur
  depends on the engine the way any crate depends on any crate.
- **Not Windows or Linux.** `mpdf-003` OQ-9 asked what the pane becomes on a webview
  that draws no PDFs, and `mpdf-009` has since made the app draw its own pages; that
  question is probably answered and wants a dated note, but it is Letur's note to write
  in Letur's repository.

## 2. Design

### The engine keeps the name, the dialect and its evidence; Letur takes the window, the page and the gates (decision, recorded)

**Measured 2026-09-01, the code is already split and the dependency runs one way.**
`app/Cargo.toml` and `cli/Cargo.toml` each take `md2pdf-core = { path = "../core" }`;
`core` takes nothing back. `grep -rn 'app/' core cli` finds four mentions, three in
`core/` — `core/src/sections.rs`, `core/src/lib.rs`, `core/tests/long_document_test.rs`
— and one in `cli/tests/cli_test.rs`, every one a doc comment. The rules corpus splits
with one crossing: `rules/pipeline.md` declares ten sources under `core/` and `cli/`,
the four `rules/desktop-*.md` files declare twenty-three sources, all under `app/`, and
`rules/web-demo.md` declares four, three under `web/` and `.github/` and one that is
`core/tests/page_examples_test.rs` — the test that reads the page, which moves with it.
So the boundary is not designed here; it is found, and the one file that straddles it is
named.

**`Ivapo/md2pdf` stays the engine's repository, and nothing in its history is
rewritten.** It keeps `core/`, `cli/`, `tests/fixtures/`, `tests/golden/`, `samples/`,
the six dialect specs with their review records, `rules/pipeline.md`, and the README
minus its desktop section. Its workspace members become `core` and `cli`.

**`Ivapo/letur` is new, public, and carries the history of what it takes.** Public
because the engine already is, and because GitHub Pages — which gate 7 needs — is not
offered to a private repository under a personal account without a paid plan. `git
filter-repo` over a fresh clone, one `--path` per entry below, yields a repository whose
`git log --follow` on `app/src/preview.rs` reads thirty-one commits back to the one that
created the file, *"feat(mpdf-003): the watch loop, and the state it writes"*, dated
2026-08-10 — the record of what happened, kept, which is the methodology's own rule for
phases and holds for files. **No commit keeps its id**, and nothing may be keyed to one:
rewriting parents and trees rewrites every hash, so the engine's own is absent from the
filtered repository by construction. A subject and a date are what survive, and they are
what gate 5 reads. What it takes:

| moves to Letur | why |
|---|---|
| `app/` whole — `src/`, `dist/`, `harness/`, `driver/`, `icons/`, `capabilities/`, `types/`, `build.rs`, `typecheck.mjs`, `tsconfig.json`, `tauri.conf.json`, `package.json`, `bun.lock` | the app |
| `web/` whole — `index.html`, `src/`, `Cargo.toml`, `Cargo.lock` | the page, by the decision this spec was drafted on: it is Letur's demo, and later its browser build |
| `tests/gates/` whole | ten scripts, four `mpdf-003`'s, one `mpdf-009`'s, five `mpdf-010`'s |
| `tests/fixtures/panel/`, `panel-decoy/`, `panel-pair/`, `panel-manifest.txt` | read by Letur's tests, harness and gates alone; `panel/outside` is a symlink into `panel-decoy` and travels as one |
| `specs/desktop_app_spec.md`, `pdf_renderer_spec.md`, `file_panel_spec.md`, `web_demo_spec.md`, and `specs/reviews/mpdf-{003,006,009,010}.md` | the four specs whose sources are Letur's, with their review records |
| `rules/desktop.md`, `desktop-panes.md`, `desktop-project.md`, `desktop-geometry.md`, `rules/web-demo.md` | every rule whose sources are Letur's |
| `.github/workflows/pages.yml`, `typecheck.yml` | both build Letur's files alone, by their own comments |
| `core/tests/page_examples_test.rs` → `app/tests/page_examples_test.rs` | it `include_str!`s `web/index.html`; renamed in the same filter with `--path-rename`, so `--follow` reads through |
| README's `## The desktop app`, the app half of `## Install`, and the `pdf.js` licence note | the words about the window |

**What is copied rather than moved is what both suites read, and the list is derived
rather than remembered.** The instrument is `grep -oh 'fixture("[^"]*")\|sample("[^"]*")\|
render_fixture("[^"]*")' app/src/*.rs | sort -u` together with the literal paths the
harness and the gate scripts carry; `app/src/watch.rs` reads no fixture, its two
`samples/` mentions being a timing table in a doc comment. Read 2026-09-02, Letur's tests
reach eleven things under the engine's `tests/fixtures/` — `basic.md`, `citations.md`,
`figure.md`, `multi_file.md`, `unsupported_html.md`, `long.md`, `near.md`, `refs.yml`,
`dot.png`, `mark.svg` and `sections/` — and four under `samples/`: `article.md`,
`check.svg`, `pipeline.svg` and `showcase/`. Each of the eleven becomes a file under
Letur's own `tests/fixtures/`, frozen at the split, and the engine keeps its originals.
**The four from `samples/` keep their shape**, as `tests/fixtures/samples/` with the
three files and the `showcase/` directory beneath it and nothing else, because
`app/src/document.rs:a_master_in_a_subdirectory_is_not_this_roots_master` reads the
`samples/` *directory* and asserts that `app/src/document.rs:masters` finds no master at
its top level — `article.md` names no section, and a flat copy beside `multi_file.md`
would. A copy is the honest shape: Letur's tests assert what the *app* does with a
document, not what the dialect does with it, so a fixture that stops tracking the
dialect costs those tests nothing, and a fixture that kept tracking it would move Letur's
measured numbers on every dialect phase.

**The showcase stays the engine's, and Letur keeps a frozen copy as a fixture.** It is
the dialect's every-construct document, and `README.md` says so. `app/src/preview.rs:showcase_in`
copies it for the app's own tests, three of Letur's gate scripts open it by name and a
fourth opens `samples/article.md`, and none of them needs the current dialect: they need
six pages and known heading lines. All of them read `tests/fixtures/samples/showcase/` in
Letur's repository, taken from the split commit rather than the working tree. **This is
what stops `rules/desktop-panes.md`'s measured numbers rotting**: that file records 1184
text items and 16 internal links off the showcase, `mpdf-007` Phase 5 moved both, and its
own sentence says a phase that edits the showcase moves them. A frozen copy is edited by
no dialect phase, so the numbers hold until Letur itself moves them.

### The page moves as it is, and the dialect keeps its public claims under its own gate (decision, recorded)

`mpdf-006`'s design is that every claim `web/index.html` makes is a snippet the suite
compiles: `core/tests/page_examples_test.rs:PAGE` reads the page by `include_str!`, and
its tests compile all twelve examples, compare the generated column against
`pulldown_cmark::html` over the crate's own parser, and pin the two assets the page
carries. That property is worth keeping, and it belongs to whoever owns the page.

**So the test moves with the page, unchanged.** `app/tests/` and `core/tests/` sit at the
same depth, so `include_str!("../../web/index.html")` and
`core/tests/page_examples_test.rs:SOURCE`'s `concat!(env!("CARGO_MANIFEST_DIR"),
"/../web/index.html")` resolve to the same file from either; its only crate use is
`md2pdf_core::{Asset, md_to_html, md_to_pdf}`, which `app` already depends on, so no
dev-dependency is added. The `#[ignore]`d
`core/tests/page_examples_test.rs:bless_the_generated_blocks` moves with it, since it
writes the page.

**And the engine keeps the twelve snippets as fixtures of its own.** The page's rows are,
today, the only place the dialect's front-door claims — a caption makes a figure, a
`[@key]` cites — are compiled *as a reader meets them*, and after this spec the page will
be rewritten as Letur's landing site by the later spec. So the engine gains
`tests/fixtures/examples/`, one file per row as the page holds it today, and a
`core/tests/examples_test.rs` that compiles each `ok` row and asserts each `error` row's
sentence — the two halves `page_examples_test.rs` already has, minus the page. The
extraction is mechanical, and it is what lets the landing page change without the
dialect's claims going unchecked.

**What this spec deliberately leaves alone in `web/`.** The page's lede still says
"Twenty-three constructs are supported", three behind; its heading still says nine keys
where ten reach the look; `rules/web-demo.md` logged both and named them a phase of
`mpdf-006` waiting to be drafted. That phase is now the web-Letur spec's, and this one
moves the page with its gap intact rather than half-fixing it on the way.

### The harness compiles through the installed CLI, because Letur's workspace has none (decision, recorded)

**Round 1 found the one wire nothing in the code declares.** `app/harness/serve.mjs`
compiles every document it serves with `cargo run --quiet -p md2pdf-cli`, and `cargo run
-p` reaches workspace members only; in Letur's workspace `app` is the sole member, so
`bun harness/checks.mjs` would die at its own "the CLI would not compile" before a single
clause ran. The dependency was invisible because both crates sat in one workspace.

**So `serve.mjs` runs a `md2pdf` binary on `PATH`**, and the prerequisite is stated where
the harness's other prerequisites are — its header comment and Letur's README, beside
`bun` and Playwright. Until Phase 3 publishes, the install line is `cargo install --git
https://github.com/Ivapo/md2pdf --locked md2pdf-cli`; after it, `cargo install
md2pdf-cli`. That is also the product's own install story for the tool, so the harness
asks for nothing a Letur contributor would not have.

The alternative — a compile binary inside Letur's crate — was weighed and refused. The
app has no library target, so a second binary would either restructure the crate to
share `app/src/document.rs:read_assets_with` or duplicate `cli/src/main.rs:read_assets`
and `cli/src/main.rs:read_sections`, and either is machinery for a harness that today
needs one process on the path.

### Letur depends on the engine by version, with a git revision as the escape hatch (decision, recorded)

`md2pdf-core` publishes to the registry and Letur's `app/Cargo.toml` writes `md2pdf-core
= "0.1"`. Between the split and the first publish, and between any two publishes Letur
cannot wait for, the dependency is `{ git = "https://github.com/Ivapo/md2pdf", rev =
"…" }` — an ordinary Cargo shape, recorded here so it is a documented seam rather than
a workaround. **`web/Cargo.toml` takes the same dependency the same way, and its own
committed `web/Cargo.lock` is regenerated with it**; a git dependency builds for
`wasm32` exactly as a path one does, Cargo fetching the whole repository and resolving
the workspace inheritance inside it. `web/` is already detached from the workspace by its
own empty `[workspace]` table and stays so.

**Three things the registry needs that the crate lacks, measured against
`core/Cargo.toml`.** It has `name`, `description`, `version`, `edition` and `license`,
and needs `repository`, `readme` and `keywords`. `cli/Cargo.toml`'s path dependency
needs a `version` beside its `path`, since a published crate cannot depend on a path.
`core/assets/` is 2.5 MB, almost all of it the bundled fonts under `core/assets/fonts/`,
and Cargo packages everything under the crate that `.gitignore` does not exclude, so the
looks and the fonts travel without an `include` list; the registry's limit is well
above it.

**Versions part ways, and Letur carries one number rather than two agreeing ones.** The
engine keeps `version.workspace` over `core` and `cli`, because they release together
and a `cli` at `0.2` over a `core` at `0.1` says nothing. Letur's root `Cargo.toml` keeps
a `[workspace.package]` for `edition` and `license`, which `app/Cargo.toml` inherits
today and goes on inheriting, and the root `[profile.release]` block travels with it,
since `strip`, `lto` and `codegen-units` are what halve the bundle; only `version`
leaves the workspace table, inlined in `app/Cargo.toml` as Letur's own. **`"version"` is
removed from `app/tauri.conf.json` rather than asserted against**: Tauri's own
documentation of the field says *"if removed the version number from `Cargo.toml` is
used"*, so the duplication the draft would have guarded against is one the configuration
need not have. `app/build.rs` stays the one line it is.

**The dry run is the gate and the publish is the author's.** `cargo publish --dry-run`
packages and builds the crate exactly as the registry would and uploads nothing, so it
is what a phase can assert. The publish itself is irreversible — a version can be yanked
and never deleted — so Phase 3's scope names it as a step the author runs, and its gate
is read *after* it: a `cargo install md2pdf-cli` from the registry writes the same bytes
the workspace binary writes.

### Two corpora, one id space kept, and what the linter has to learn first (decision, recorded)

**Every spec keeps its id.** The methodology forbids renumbering and never reuses an
id, and `spec-lint` has no contiguity rule — so `mpdf-003`, `006`, `009` and `010` keep
their numbers in Letur's repository, and the engine's `mpdf-001` through `011` read as
the numbered record they are, with four numbers that now live next door. Each
`specs/INDEX.md` is regenerated from what its own tree holds.

**The draft of this section claimed the crossings would degrade to warnings and that a
sibling checkout would heal them. Round 1 measured both claims against the tool and both
are false.** A Letur tree built exactly as Phase 1 describes lints at fifteen errors and
forty-eight warnings, and a `../md2pdf` checkout beside it changes nothing:
`source_roots` is a filter over the repository's own file list, so a path outside it can
never match. The errors have three sources, and none of them is a warning as the tool
stands:

- **The basename fallback.** A cited path that does not exist is resolved by its
  filename alone, so `cli/src/main.rs:read_assets` lands on `app/src/main.rs` and
  `core/src/lib.rs:section_paths` on `web/src/lib.rs`, where the symbol is absent — an
  error, thirteen times across `desktop_app_spec.md`, `file_panel_spec.md`,
  `rules/desktop.md` and `rules/web-demo.md`. The same trap waits for the engine after
  Phase 2, where `app/src/main.rs` citations would land on `cli/src/main.rs`.
- **An edge whose target lives next door.** `mpdf-010` carries `supersedes: [{id:
  mpdf-008, …}]` and `mpdf-008` stays with the engine; the tool excuses a missing target
  only when its *prefix* differs from the configured one, and it will not. **The engine
  meets no mirror image after Phase 2**, which round 2 measured rather than assumed:
  `mpdf-008` Phase 4 carries `by: mpdf-010`, and a `by` is never resolved against the id
  map at all — a tree holding that spec alone lints clean — so the engine's side of this
  edge reports nothing, and needs nothing.
- **A rule source that moved.** `rules/web-demo.md` names `core/tests/page_examples_test.rs`,
  checked by existence against the repository root. That one is a scope fix, not a tool
  fix: Letur's copy names `app/tests/page_examples_test.rs`.

**So three rules in `spec-lint` are a prerequisite of Phase 1's gate, and they are the
tool learning that a corpus may span two repositories rather than the corpus being
wrong.** They are one change, in the methodology's own repository, made once:

1. **`id_prefix` accepts a list**, the first entry the one new ids are allocated under
   and the rest legacy prefixes an existing id may carry. **Both of its readers compare
   with `!=` against a string and must instead test the parsed prefix against the
   configured set** — equality where the configuration is a string, membership over the
   entries where it is a list, and never a substring test, which would accept `mpd-001`
   under `mpdf` — or a list makes every
   id malformed and every edge foreign: the id check in `spec-lint:validate_spec`, and
   `spec-lint:_foreign`, whose result decides whether a missing edge target is a warning
   or an error — so rule 2 depends on this one being done at that site rather than only
   at the first. Letur's configuration is `[ltr, mpdf]` per OQ-1; the engine's stays the
   string `mpdf`, and a string must go on meaning what it means today.
2. **An edge target absent from the tree is a warning, never an error.** The tree
   stopped being the universe; a `supersedes`, `superseded_by` or `extends` that names a
   spec the tree does not hold is reported and does not fail. `spec-lint:_foreign`'s
   demotion is the shape to widen — it already anticipates a cross-repository edge — and
   it is reached at **three** sites, which round 2 counted against the code: `extends`,
   `supersedes` and `superseded_by`. **A phase's `by` is not among them and needs no
   rule**: it is never resolved against the id map, so a `by` naming an absent spec
   reports nothing today and will report nothing after. That is a gap in the tool rather
   than a consequence of this split — an unresolvable `by` was always silent — and OQ-6
   holds it rather than this spec widening a check nothing here needs.
3. **A cited path carrying a directory component resolves only as a path.**
   `spec-lint:SourceIndex.resolve` tries the exact relative path, then a suffix match,
   then falls back to the **basename** — and that last step is what lands
   `cli/src/main.rs:read_assets` on `app/src/main.rs` and calls the symbol absent. The
   fallback is right for a bare filename and wrong for a path, so it applies only when
   the cited string holds no `/`; a path that reaches it unresolved returns no candidate,
   which is the `UNRESOLVED` tier and the warning the tool already has for that case.

Until they land, Phase 1 is blocked and says so; nothing in its scope is worth doing
against a gate that cannot pass. **The sibling-root idea is withdrawn**, not deferred:
the tool has no such mechanism, and the warnings that remain after the three rules are
the designed answer — a pointer that says where the code went is what a later reader
needs, and nothing is rewritten in either corpus to make the crossings go away. What
does remain a warning after the rules is named in the gate, and one of them is
inherited rather than made: `rules/desktop-geometry.md` has never been regenerated and
emits `RULE_SOURCES_WITHOUT_GENERATED` today, in the engine, and will in Letur.

**Each repository's `CLAUDE.md` names its own observable.** The engine's is the sentence
it has always carried. Letur's is the thing an author of Letur sees, and OQ-2 holds the
sentence Phase 1 writes; the stanza is the one `spec-init` writes, minus its conditional
hooks paragraph, which the engine's own `CLAUDE.md` also omits.

### The history is extracted for Letur and untouched for the engine (decision, recorded)

`git filter-repo` over a fresh clone of `Ivapo/md2pdf`, with one `--path` per entry in
§2's table — the tool matches a path exactly or as a directory prefix, so
`tests/fixtures/panel` does not take `panel-decoy`, `panel-pair` or `panel-manifest.txt`,
and each of the four specs, four review records, five rules and two workflows is its own
argument — and one `--path-rename` for the page test, produces Letur's history: every
commit that touched those paths, with its message, its author and its date, and
nothing else. The engine's repository removes the same paths in an ordinary commit and
keeps every commit that ever touched them, because rewriting a published repository's
history invalidates every checkout and every link into it, and this project's commit
messages are part of its record. The cost is that the four moved specs' commits exist
twice, once in each history, which is what a copy of a record is.

## 3. Open questions

- **OQ-1 — what prefix do new Letur specs carry, and what does `spec-lint` need to
  learn?** *(design call)* ~~The four migrated specs keep `mpdf-NNN`. A new Letur spec
  under `mpdf` would collide with the engine's next allocation one day; under a prefix
  of its own it fails `spec-lint`'s `SPEC_ID_MALFORMED`, which compares every id against
  the one configured prefix.~~ **RESOLVED 2026-09-02, in Phase 1's round 1: the tool
  learns a list, and it learns two more things beside it.** §2's last decision carries
  the three rules, because round 1 measured that the prefix was the smallest of three
  ways the linter fails a corpus that spans two repositories. The prefix itself is
  `ltr`, three letters inside the methodology's `[a-z]{2,4}` — **proposed as a default
  and confirmed by the author 2026-09-02**, so it is what Phase 1 writes into Letur's
  `.spec-lint.yaml` and `CLAUDE.md` rather than a value still open to a rename.
- **OQ-2 — what is Letur's observable, in one sentence?** *(design call)* ~~The engine's
  is "the typeset PDF that Typst compiles from the user's markdown", and Letur's wants
  one of its own.~~ **RESOLVED 2026-09-02, confirmed by the author:** Letur's observable
  is *"the page the author sees beside their text, redrawn as they write it"*, which is
  `mpdf-003`'s note and `mpdf-009`'s title in one line. Phase 1 writes that sentence
  into Letur's `CLAUDE.md` as the stanza's observable, and every phase of every Letur
  spec answers to it the way this corpus's phases answer to the PDF.
- **OQ-3 — does the engine keep a page of its own?** *(deferred by evidence)* After this
  spec the engine's front door is its README and its registry entry, and the only
  browser build of the dialect is Letur's. A small engine playground — a textarea and a
  PDF, no argument, no rows — is what the spike was before `mpdf-006`, and it may be
  wanted again once the landing page is Letur's. Nothing asks for it today.
- **OQ-4 — where does `https://ivapo.github.io/md2pdf/` go?** *(design call)* ~~The Pages
  workflow moves with `web/`, so the site publishes from Letur's repository at
  `https://ivapo.github.io/letur/`, once the author enables Pages there with its source
  set to the workflow — a settings step the deploy action does not perform for itself.
  The old URL is linked from the engine's README and from the page's own prose. A
  redirect page at the old path is one file in the engine's repository with a Pages
  workflow of its own, or the links move and the old URL is let go; the second is
  cheaper and the first is kinder to a bookmark.~~ **RESOLVED 2026-09-02, in Phase 2's
  round 1, confirmed by the author: the old URL is let go.** The links move to
  `https://ivapo.github.io/letur/` and no redirect is written — the redirect option
  would have added a file *and a Pages workflow of its own* to a phase that otherwise
  removes both workflows, and left the engine keeping a Pages deploy forever for one
  `<meta refresh>`. The half neither option covered, and the reason this needed
  resolving rather than eliminating: **Pages stays enabled on `Ivapo/md2pdf` until
  someone disables it**, serving the last deployed build of a `web/` Phase 2 deletes.
  That is a live site outliving its source, so disabling it is Phase 2's gate 6 — an
  author settings step, the mirror of Phase 1's enabling Pages on `Ivapo/letur`. The
  first half of the paragraph above is unchanged and shipped in Phase 1: Letur publishes
  at `https://ivapo.github.io/letur/`, measured 2026-09-02 at twelve `data-example`
  rows.
- **OQ-5 — how tightly does Letur pin the engine?** *(design call)* `"0.1"` takes every
  patch release; `"=0.1.0"` takes none. A product that measures its page to the text
  item, as `rules/desktop-panes.md` does, has a reason to prefer the exact pin and bump
  by hand. Cheap to hold, and decided by the first time an engine patch moves a Letur
  number.
- **OQ-6 — should an unresolvable `by` be reported at all?** *(design call, for the
  methodology rather than for this repository)* Measured in Phase 1's round 2: a phase
  carrying `cut` and `by` is checked for the pair and for a matching `supersedes` on the
  named spec *when that spec is present*, and never for the named spec existing at all —
  so a `by` pointing at nothing lints clean, and did before this split. The three edges
  get the demotion §2's rule 2 describes because they are checked today; `by` would need
  a check invented for it, which nothing here asks for. It is named so the asymmetry is
  a recorded gap rather than a discovery.

- **OQ-7 — `md2pdf_core::md_to_html` is published in Phase 3 with no test and no
  reader.** *(design call, raised by Phase 2's round 1 and deferred by it)* Measured:
  `core/tests/page_examples_test.rs` is its only caller in this tree, and the
  extraction Phase 2 performs deliberately takes two halves — compile the `ok` rows,
  assert the `error` sentences — and leaves the third, the generated column compared
  against `pulldown_cmark::html`, with the page in Letur. So after Phase 2 the export
  has no exercise here, and Phase 3 ships it to the registry that way. This is a
  consequence of a decision §2 already recorded rather than a defect in it, and it is
  not Phase 2's to fix: the comparison is *about the page*, and the page is Letur's.
  Three answers are open — Letur's `app/tests/page_examples_test.rs` goes on covering it
  from next door and that is enough; the engine grows a small test of its own over
  `tests/fixtures/examples/`; or the export is reconsidered before Phase 3 publishes an
  API this repository does not use. **Named so Phase 3 meets it as a recorded question
  rather than a discovery.**

## 4. Implementation phases

Strictly sequential, and the order is the safety argument: Letur's repository exists
with everything it needs *before* the engine drops anything, so at no commit does the
app's code exist only in history; and the engine is published *after* it stands alone,
so what the registry holds is what the repository holds.

### Phase 1 — Letur's repository exists, with its history

*Produces the observable: **no**, and the gate is that it does not move.* Letur's window
draws the same page from the same document in its own repository as it did in this one,
hashed either side.

**Prerequisite, outside this repository.** The three `spec-lint` rules §2 names land in
`spec-driven-dev/bin/spec-lint` first — a prefix list, an absent edge target as a
warning, and no basename fallback for a path with a directory in it. Gate 6 cannot pass
without them, measured, so this phase is blocked until they do and is planned only after.

> **SATISFIED 2026-09-02.** All three shipped as `sdd-001` Phase 13, *"a corpus may span
> two repositories"*, reviewed there over three rounds. The third landed in a better
> shape than this spec asked for: the basename fallback was **removed** rather than
> guarded, that session having found it unreachable for a bare filename — the suffix
> branch already selects exactly the rels whose last segment is the cited name and
> returns first, so all the fallback ever answered were citations that do carry a
> directory. Measured against the shipped tool the same day: the Letur corpus simulation
> lints at **0 errors and 63 warnings**, every one of gate 6's three named kinds, with all
> 61 unresolved paths under `core/` or `cli/`; and `md2pdf`, `zukai` and `assimilator` are
> each unchanged at 0 errors. **This phase is no longer blocked.**

- **Scope:**
  - `Ivapo/letur` is created, public, and populated by `git filter-repo` over a fresh
    clone on the paths §2's table names, one `--path` each and one `--path-rename` for
    the page test, so the history comes with the files. The author enables Pages on it
    with the workflow as its source, which is the one settings step the deploy action
    cannot do for itself. **The engine's repository is touched for one thing**: this
    spec's own `shipped` date, which lives here.
  - Letur's root `Cargo.toml`: workspace member `app` alone, with `web` detached as
    today; `[workspace.package]` keeping `edition` and `license` and the
    `[profile.release]` block kept whole; `md2pdf-core` by git revision, pinned to the
    engine's `main` at the split, in both `app/Cargo.toml` and `web/Cargo.toml`; Letur's
    `Cargo.lock` seeded from the engine's and `web/Cargo.lock` regenerated.
    `app/Cargo.toml` carries `version = "0.1.0"` inline, and `app/tauri.conf.json` loses
    its `"version"` key, so the bundle takes the crate's.
  - Letur's `tests/fixtures/`: the four panel entries moved; the eleven fixtures §2
    derives copied flat; `tests/fixtures/samples/` holding `article.md`, `check.svg`,
    `pipeline.svg` and `showcase/`, the last taken from the split commit. Two things
    repoint and nothing else does: `app/src/preview.rs:sample`'s `../samples` and the
    `../samples` literal in `app/src/document.rs:a_master_in_a_subdirectory_is_not_this_roots_master`,
    both to `../tests/fixtures/samples`; `fixture()` in both files, the harness's
    `tests/fixtures/panel` default and the driver's resolve unchanged. The `open
    samples/…` prose lines in `tests/gates/mpdf-009-phase5.js` and
    `mpdf-010-phase{1,2,5}.js` say the fixture path instead.
  - `app/harness/serve.mjs` compiles through a `md2pdf` binary on `PATH` rather than
    `cargo run -p md2pdf-cli`, and its header and Letur's README carry the install line
    §2 gives.
  - `core/tests/page_examples_test.rs` lands as `app/tests/page_examples_test.rs` with
    not one line changed.
  - `specs/`: the four specs and their review records, `specs/_template.md`; `rules/`:
    the five rule files and `rules/_template.md`, with `rules/web-demo.md`'s third
    source repointed to `app/tests/page_examples_test.rs`; both `INDEX.md` files
    regenerated. `mpdf-006` §1 gains the dated `CORRECTED` note §1.1 names, beside
    *"Give the dialect a front door that shows itself"*, in the form
    `mpdf-003` §1.1's note takes.
  - `.spec-lint.yaml` with `id_prefix: [ltr, mpdf]` and `source_roots: ["."]`;
    `CLAUDE.md` in `spec-init`'s stanza shape with the id prefix and OQ-2's sentence;
    `.gitignore` reduced to Letur's entries; `LICENSE` copied; a README opening with the
    window, built from the engine README's `## The desktop app`, the app half of
    `## Install`, the `pdf.js` licence note, and the harness prerequisite.
  - `.github/workflows/pages.yml` and `typecheck.yml`, with `pages.yml`'s `paths:`
    trigger losing `core/**`, since there is no `core/` to watch.
  - One new test, `#[ignore]`d and run deliberately as
    `core/tests/page_examples_test.rs:bless_the_generated_blocks` is: in
    `app/src/document.rs`'s test module, beside the one that reads the samples tree, it
    compiles `tests/fixtures/samples/showcase/showcase.md` through
    `app/src/document.rs:read_assets_with` and `md2pdf_core::md_to_pdf` and writes the
    bytes to a file under `std::env::temp_dir()`, which is what gate 4 hashes.
- **Exit gate:**
  1. `cargo test --workspace` in Letur's repository is green. `grep -c '#\[test\]'
     app/src/*.rs` reads `main.rs` 0, `document.rs` 46, `preview.rs` 61, `watch.rs` 8 —
     the 114 the engine's tree reports today plus the one ignored test this phase adds
     — and the thirteen in `app/tests/page_examples_test.rs`, one of them ignored, are
     gate 2's.
  2. `app/tests/page_examples_test.rs` passes with `EXPECTED` at twelve and its asset
     count at two, unchanged.
  3. With `md2pdf` installed from the engine's `main` at the split, `bun
     harness/checks.mjs` passes in Chromium, and `bun harness/checks.mjs --falsify`
     exits zero with every one of the twelve mutations `app/harness/checks.mjs:OWNS`
     names reporting `ISOLATED`.
  4. **The page does not move.** The engine's `target/release/md2pdf`, built at the
     split commit on a clean tree, over `samples/showcase/showcase.md`; and the ignored
     test's bytes over `tests/fixtures/samples/showcase/showcase.md` in Letur's
     repository; hashed with `shasum -a 256`, identical.
  5. `git log --follow --oneline -- app/src/preview.rs | wc -l` in Letur's repository
     reads 31, and its last line is the commit that created the file, subject
     *"feat(mpdf-003): the watch loop, and the state it writes"*, dated 2026-08-10 under
     `--format=%ad --date=short`, the format qualifier being what prints that literal
     rather than a full timestamp. **Its hash is not the engine's and must not be**, per
     §2; a gate keyed to one would fail on a correct extraction.
  6. `/Users/ivapo/dev/main/spec-driven-dev/bin/spec-lint . --format json` in Letur's
     repository reports zero errors, and every warning it reports is one of three kinds:
     an unresolved path under `core/` or `cli/`, the absent edge target `mpdf-008` on
     `mpdf-010`, or `RULE_SOURCES_WITHOUT_GENERATED` on `rules/desktop-geometry.md`. The
     counts are written into the round that runs this gate.
  7. The Pages workflow builds `web/` from Letur's repository, and
     `curl -s https://ivapo.github.io/letur/ | grep -c 'data-example='` reads 12.
- **Close-out:** Letur's `rules/desktop.md` records the fixture paths, the version
  ownership and the harness's compile path with its prerequisite;
  `rules/desktop-panes.md`'s measured-numbers sentence names the frozen fixture as what
  it measures; `rules/web-demo.md` carries the moved source. The push to the new remote
  carries at least two commits, the code and the corpus, so each half is bisectable on
  its own; this spec's `shipped` date is one commit in the engine.

### Phase 2 — the engine stands alone

*Produces the observable: **no**, and the gate is that it does not move.* Every golden
is byte-identical and the three hashed documents hash as they did.

- **Scope:**
  - `Cargo.toml`'s members become `["core", "cli"]`. `app/`, `web/`, `tests/gates/`, the
    panel fixtures, the four specs and their review records, the five rule files and the
    two workflows are removed in one ordinary commit; nothing is rewritten. `Cargo.lock`
    is pruned by the members change and is committed with it.
  - **`core/tests/page_examples_test.rs` is removed, and `tests/fixtures/examples/`
    takes the page's twelve rows *and its two assets*.** The assets are the half a
    "mechanical" extraction drops, and the engine holds no substitute for either:
    - The page carries two `data-asset` elements, `pipeline.svg` (509 bytes) and
      `refs.yml` (151 bytes, one key, `knuth1986`). **Neither may be reused from the
      engine's own tree.** `tests/fixtures/refs.yml` is keyed
      `"DBLP:books/lib/Knuth86a"` and its own header comment says why — a fixture keyed
      `knuth1986` "would pass under either spelling and prove nothing about
      `label(…)`" — so the `citation` row cites a key that file does not hold and fails
      outright. `samples/pipeline.svg` is 510 bytes against the page's 509, one trailing
      newline, which is worse than failing: it compiles, and the gate would accept a
      fixture set that is not the page's. **Both are copied out of `web/index.html` at
      the phase's parent commit**, byte for byte, into `tests/fixtures/examples/`.
    - **A row's `data-expect` is carried by the directory, not by the file**:
      `tests/fixtures/examples/ok/` and `tests/fixtures/examples/error/`, each file
      named for its `data-example`. An in-file marker is *actively wrong*, and the trap
      is specific — two rows (`frontmatter`, `citation`) open with `---`, and a leading
      HTML comment is itself a `raw HTML block` refusal.
    - `core/tests/examples_test.rs` compiles the **nine** `ok` rows with both assets in
      hand, refuses the **three** `error` rows, and asserts the counts, which is
      `core/tests/page_examples_test.rs:EXPECTED`'s job arriving without the page.
  - **Two doc comments are corrected, and both are decision statements this phase makes
    false** — as against the many that merely name a path which now lives next door, and
    which stay. The test is whether the sentence still says something true after the
    split, not whether it mentions a moved file:
    - `core/tests/long_document_test.rs`'s `//!` block, at line 17, appeals to
      `page_examples_test.rs`'s shape as a precedent — *"the generator is `#[ignore]`d
      … and the check reads a compiled-in copy"*. **The precedent clause is deleted, not
      repointed.** `core/tests/examples_test.rs` has neither half: `bless_the_generated_blocks`
      went to Letur with the page in Phase 1, and there is no compiled-in copy to bless.
      Repointing it at `examples_test.rs` would assert a shape that file does not have.
      The claim the sentence carries — the generator writes the fixtures and this file
      pins them — stands on its own and stays; only the appeal goes, and
      `web/index.html` goes with it.
    - `core/src/lib.rs:md_to_html`'s doc comment says "the only reader it has —
      `web/index.html`'s comparison column, generated by `core/tests/page_examples_test.rs`".
      **The honest correction is that the reader left with the page**, and it is now
      Letur's `app/tests/page_examples_test.rs` over Letur's own copy of that column.
      It is *not* `tests/fixtures/examples/`: `examples_test.rs` never calls
      `md_to_html`, so naming it would swap one false statement for another — and the
      close-out would then propagate that into `rules/`, the artifact that must track
      the code. OQ-7 holds what this export's missing exercise here costs.
  - The README loses `## The desktop app`, the app half of `## Install` and the
    `pdf.js` note, and gains one paragraph pointing at Letur's repository. **Its demo
    link moves to `https://ivapo.github.io/letur/` per OQ-4, now resolved**, and
    `## Try it`'s claim that "every example on it is one this repository's tests
    compile" is rewritten: after this phase the page is Letur's and what this repository
    compiles is `tests/fixtures/examples/`.
  - **Pages is disabled on `Ivapo/md2pdf`.** An author settings step, the mirror of
    Phase 1's enabling it on `Ivapo/letur`, and named in the scope rather than only in
    the gate so a plan-mode pass reading the scope alone sees the step it must arrange.
    Without it GitHub goes on serving the last deployed build of a `web/` this phase
    deletes — measured 2026-09-02, that URL answers **HTTP 200**, so this is a live site
    outliving its source rather than a hypothetical.
  - `CLAUDE.md` drops nothing from its stanza — the observable sentence is unchanged.
    `.spec-lint.yaml` is unchanged: **the three rules Phase 1 needed are what let the
    cross-repository paths report as warnings rather than errors here.** They are *not*
    what makes `mpdf-008` Phase 4's `by: mpdf-010` report as one: a `by` is never
    resolved against the id map, so it reports **nothing**, per §2's rule 2 and
    confirmed by measurement. Those paths are spread across five of this corpus's
    specs; gate 5 names the kinds rather than counting them, and says why.
    `.gitignore` loses the
    `app/`, `web/` and harness entries — `/app/gen/`, `/web/pkg/`, `/web/target/`,
    `/app/.mirror/`, `/app/.harness/` — **and two more**, whose comments name things
    that leave: `/.playwright-mcp/` (the gate driver) and `node_modules/` (which names
    `app/package.json`). Both `INDEX.md` files regenerated.
  - Every other doc comment in `core/` and `cli/` naming an `app/` or `web/` path stays
    as it is, and so does every historical mention of the page test by bare filename:
    they are the record of why a shape was chosen, and a path in prose that now lives
    next door is what a warning is for. **No count of them is asserted anywhere in this
    phase**, deliberately — gate 4 says why.
- **Exit gate:**
  1. `cargo test --workspace` is green with `core` and `cli` alone, and
     `git diff --stat <parent>..HEAD -- tests/golden` is empty: no golden moves. **The
     revision range is not optional** — with no baseline the command compares the
     working tree to the index and is empty whatever the commit did. And the suite is
     **counted, not just green**: `grep -rh '#\[test\]' core/tests/*.rs cli/tests/*.rs |
     wc -l` reads **255** today and must read **242 plus whatever `examples_test.rs`
     adds**, the difference being the page test's thirteen. Phase 1 pinned its counts
     this way; without it an over-deletion that leaves a green suite passes.
  2. `core/tests/examples_test.rs` compiles the **nine** `ok` rows and refuses the
     **three** `error` rows — twelve rows, of which nine compile, measured off
     `web/index.html`'s `data-expect` attributes. The three sentences are quoted here so
     the clause is self-contained, both of the draft's sources being deleted by this
     same commit: `unsupported markdown construct 'raw HTML block' at line 3`,
     `unsupported markdown construct 'task list marker' at line 1`, and
     `math error at line 1: unsupported command '\includegraphics'`.
  3. **The page does not move.** `target/release/md2pdf` over `tests/fixtures/citations.md`,
     `tests/fixtures/citations_press_release.md` and `samples/showcase/showcase.md`,
     hashed before this phase's commit and after, pairwise identical. **`-o` into a
     scratch path**, since `cli/src/main.rs:default_output` writes beside its input and
     would dirty `tests/fixtures/` and `samples/`. Three is the right number because
     between them they reach all three files in `core/assets/`:
     `citations_press_release.md` is the only one taking `press-release.typ`, and
     `showcase.md` the only one reaching `math.typ`.
  4. **Nothing under `core/` or `cli/` reaches into `app/` or `web/`, and no surviving
     line names a file this phase deleted.** Four checks, none of them a count:
     - `grep -rn "core/tests/page_examples_test" core cli` returns **nothing**. The
       **path** form is a claim about *this* tree, and this phase deletes the file it
       names; the one line carrying it is `core/src/lib.rs:md_to_html`'s, which the scope
       corrects. **The bare filename is deliberately not caught**, and that distinction
       is the clause's whole design: `core/tests/messages_test.rs`'s module block records
       that before that file existed, the repository's only byte-exact `Display`
       assertion was the page test's refusal case — a statement about what *was* true,
       which the split does not falsify and which this phase therefore leaves alone.
       (Named in prose rather than cited, so this clause does not itself add the very
       kind of citation it is about.) A check that demanded zero mentions would force true
       history to be erased to satisfy a gate, which is the error this clause replaced.
     - Every line `grep -rn "app/\|web/" core cli` matches is **a comment, never code**:
       each begins, after leading whitespace, with `///`, `//!` or `//`. That is the
       property the phase actually turns on — that the crossings are a record of why a
       shape was chosen and not a dependency — and `cargo test --workspace` in gate 1
       is what would fail if it were ever false, a compile reaching a deleted tree not
       being a thing a grep has to discover.
     - The four doc comments that deliberately keep an `app/` path are **still there**,
       named rather than counted: `core/src/sections.rs:Sources::resolve`'s note on
       `app/src/watch.rs:classify`, `core/src/lib.rs:Anchor`'s on `app/src/document.rs`,
       `core/tests/long_document_test.rs`'s module block, and
       `cli/tests/cli_test.rs:the_binarys_file_is_byte_identical_to_the_librarys_bytes`'s.
       A floor on what must survive, so an over-deletion is caught, with no ceiling that
       a new comment could breach.
     - **`core/tests/long_document_test.rs` names neither `web/index.html` nor the page
       test.** Without this an implementer could skip the first correction entirely: the
       surviving clause is a comment, so the check above passes it, and its page-test
       mention is the bare form the first check spares.

     **This clause was a count in three drafts and broke in all three review rounds**, by
     seven, then two, then one: a total over a tree *this same phase edits* moves whenever
     the phase writes or rewrites a line, and both corrections below do, as does the new
     `core/tests/examples_test.rs` — whose module doc naming `web/index.html` as its
     fixtures' provenance is the right comment to write and would breach any closed list.
     The instrument was the defect, not the arithmetic. Recorded so a later reader does
     not helpfully restore a number.
  5. `spec-lint .` — **by absolute path, the tool being on no `PATH` here** — exits
     zero with **no error**, and every warning is an unresolved citation of one of four
     kinds: a path under `app/`, a path under `web/`, a path under the removed page
     test, or a **bare `<file>.rs:<symbol>` whose file left with the split** — the last
     covering both `mpdf-007`'s two `preview.rs:` citations, which resolve today through
     the suffix branch and go unresolved once `app/src/preview.rs` leaves (the basename
     fallback having been removed rather than guarded by `sdd-001` Phase 13), and any
     bare page-test citation the corpus carries. **None of these is repairable in the
     corpus**, several sitting in accepted, append-only documents — including this one —
     which is why the gate names kinds.

     **No total is asserted here, for the reason gate 4 gives one clause up, and this
     clause is where that reason was proved twice.** A count of warnings is a count over
     a corpus that *contains this spec*, and every review round edits this spec: the
     drafts read 60, and the folds that fixed gate 4 moved it to 63 by adding three
     citations of their own — one in the `examples_test.rs` scope bullet and **two
     inside gate 4's own repair**. A number that a gate's own correction falsifies is
     not a gate. `mpdf-008` Phase 4's `by: mpdf-010` reports nothing, and the inherited
     `RULE_SOURCES_WITHOUT_GENERATED` leaves with `rules/desktop-geometry.md`, so the
     gate expects neither — both properties, both measured, neither a count.
  6. **Pages is off on `Ivapo/md2pdf`**, per OQ-4 and the scope bullet above.
     `curl -sI https://ivapo.github.io/md2pdf/` returns 404 rather than today's 200.
     **Re-checked after a wait**, Pages' CDN serving a cached build for a while after a
     disable, so a single 200 immediately afterwards is not a failure.
- **Close-out:** `rules/pipeline.md`'s `covers:` loses nothing — all ten of its
  `sources` are under `core/` and `cli/` and none is removed — and the correction it
  does need is the sentence asserting `md_to_html`'s "one reader is `web/index.html`'s
  comparison column, generated by `core/tests/page_examples_test.rs`", two files this
  phase deletes. It takes the same correction as the doc comment it is generated from,
  in the same pass — the reader left with the page — or `/sync-rules` will faithfully
  re-seed the stale claim. **The file sits at 1237 of its 1240 `max_lines`**, so the cap
  moves with the edit rather than the prose being shaved to fit. `rules/web-demo.md` is
  gone from this tree, and `rules/INDEX.md` says so by regeneration. One push.

### Phase 3 — the engine is published, and Letur depends on it by version

*Produces the observable: **no**, and the gate is that the registry's binary writes the
same bytes the workspace's does.*

- **Scope:**
  - `core/Cargo.toml` gains `repository`, `readme` and `keywords`; `cli/Cargo.toml`
    gains the same and a `version` beside its `path` dependency. A `README.md` for the
    crate is the repository's own, which the field names.
  - **Two things Phase 2 leaves this phase, both raised in its round 1 and neither
    fixed there.** After Phase 2 the engine carries **no CI at all** — both workflows
    were Letur's and left with it — so nothing but a local `cargo test --workspace`
    stands between a commit and a publish; whether that wants a workflow of its own is
    this phase's call, at the moment it first matters. And `md_to_html` publishes
    untested, which OQ-7 holds.
  - `cargo publish --dry-run -p md2pdf-core` and then `-p md2pdf-cli` both package and
    build clean. **The publish itself is the author's step**, run by hand in the order
    the dependency requires, `md2pdf-core` first; this spec does not run it.
  - Once published, Letur's `app/Cargo.toml` and `web/Cargo.toml` replace the git
    revision with the version per OQ-5, both lockfiles follow, and the harness's install
    line becomes `cargo install md2pdf-cli`.
- **Exit gate:**
  1. Both dry runs clean, and the packaged `md2pdf-core` lists `assets/fonts/` and the
     three `.typ` files, read from `cargo package --list`.
  2. **The registry's binary is the workspace's.** `cargo install md2pdf-cli` into a
     scratch prefix, then `md2pdf samples/showcase/showcase.md` from it and from
     `target/release/md2pdf`, hashed, identical.
  3. Letur's `cargo test --workspace` is green against the registry version with no
     `[patch]` and no `git` dependency left in either `Cargo.toml`.
  4. `crates.io/crates/md2pdf-core` shows the README the repository holds.
- **Close-out:** `rules/pipeline.md`'s CLI section records the install path and the
  registry names; the engine README's `## Install` opens with `cargo install
  md2pdf-cli`. Letur's `rules/desktop.md` records the version dependency. One push per
  repository.
