---
id: mpdf-007
title: citations-and-bibliography
note: >
  A document cites its sources and prints their reference list: the frontmatter names a
  bibliography file, the caller supplies it as bytes beside the images, `[@key]` becomes a
  citation, and Typst renders both the marks and the list.
status: draft
last_updated: 2026-08-22

phases:
  - name: "Phase 1 — a cited source reaches the reference list"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 2 — a key the bibliography does not hold is named, not compiled"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 3 — the desktop app watches the bibliography"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — the browser carries a bibliography of its own"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-003, mpdf-005, mpdf-006]
reference: >
  Pandoc's citation syntax is the borrowing, and only the syntax: `[@key]` is what a
  reader arriving from Pandoc already types, and `mpdf-001`'s `reference` calls Pandoc's
  pipeline the inspiration while its §1.1 refuses to *embed* Pandoc — heavy, no
  lightweight WASM build, GPL linking questions. None of that is a reason to spell a
  citation differently from every other tool. The rendering is Typst's `bibliography`
  and `cite`, which take Hayagriva `.yml`/`.yaml` and BibLaTeX `.bib`; no CSL engine,
  no citeproc and no `pandoc-citeproc` is adopted or reimplemented.
---

# citations-and-bibliography

## 1. Goal

**Let a document cite its sources and print the list.** Today `md2pdf` typesets an
article that can carry figures, tables, listings, equations and cross-references — and a
citation reaches the page as literal text. Measured 2026-08-22 against the built CLI,
every Pandoc citation form escapes and prints:

```
Bracketed cite [@smith2020]  →  Bracketed cite \[\@smith2020\]
A bare one @smith2020        →  A bare one \@smith2020
```

That is the escape rule working exactly as `mpdf-001` §2 designed it — nothing is
dropped and nothing is guessed — and it is also the whole gap. An article that cannot
cite is not an article.

**The observable is unchanged — the typeset PDF that Typst compiles from the user's
markdown — and this spec produces one instance of it that nothing else can: a document
whose last page is a reference list, with numbered marks in the body pointing into it.**
§4 holds each phase to that.

### 1.1 Why this is a new spec, and why it is not a phase of `mpdf-005`

The methodology's §6.1 is an ordered test and it is worked in full. **This is the second
case in the corpus where a shipped spec had already answered it.**

- **Step 0 — a decision, or only the code?** A decision: a new construct in the dialect,
  a second kind of file the caller must supply, and a section of the document the
  markdown never names.
- **Step 1 — does it remove or contradict shipped work?** **No.** `mpdf-001` §1.1 parks
  "citations and bibliography" for *later specs* by name — the same parking note that
  bought `mpdf-004` and `mpdf-006`. What it reinterprets is text that currently prints
  literally, which is the shape `mpdf-005` OQ-8 worked for `[](#name)` and shipped.
- **Step 2 — is the subject one an existing spec owns?** **No, and `mpdf-005` disowns it
  in its own §2**: "Folding a second, larger subject into a spec that has shipped none of
  its own phases is that failure exactly."
- **Step 3 — a named kind under a reserved framework?** **No, and this was decided on
  2026-08-15**, in `mpdf-005` OQ-5: *"two subjects. Nothing is reserved, `related` is the
  whole edge."* A framework under §2 reserves named kinds sharing a mechanism, and these
  share none — the direction is opposite, inward at content this compile already holds
  against outward at a record the document does not contain; the channel differs; and the
  observable differs, typography of existing content against a section the markdown never
  names. **So `extends` stays `null` and `related` carries `mpdf-005`**, which is what
  that resolution instructed.
- **Step 4 — a new spec.** With a shipped spec's own resolution as the argument.

**`mpdf-005` also left this spec a debt and paid it in advance.** Its §2: *"One thing is
owed to the citation spec that does not yet exist, and it is cheap: the addressing
convention."* OQ-8 chose `[](#name)` so that a citation spec could *extend rather than
contradict* it. §2 below is where that is spent.

### 1.2 Non-goals

- **Not the cross-reference realignment.** `[](#name)` is `mpdf-005`'s spelling and
  `mpdf-005`'s subject. Moving it toward `pandoc-crossref`'s `@fig:name` is a phase
  appended to *that* spec under §6.1 step 2, and doing it here would be the second,
  larger subject `mpdf-005` refused to fold into itself. §2 records what this spec does
  to keep that move cheap.
- **No CSL, no citeproc, no style authoring.** Typst renders the marks and the list.
  A style beyond what Typst ships is not in the dialect.
- **No network, ever.** No DOI lookup, no fetching a record, no resolving a key against
  anything. `core/src/lib.rs:TypstWorld` "implements no package resolution, so no import
  can reach the network on any target", and that property is not weakened here.
- **No bibliography authoring.** The file is the author's, written in a format Typst
  already reads. This spec does not invent one, and does not read records out of the
  frontmatter — that alternative is argued and refused in §2.
- **Not full Pandoc-markdown.** `mpdf-001` §1.1's line holds. One citation form is
  borrowed because a reader already types it; the rest of Pandoc is not.
- **No change to how a document without a bibliography compiles.** A file naming no
  `bibliography` key emits exactly the Typst it emits today, byte for byte.

## 2. Design

### The citation is `[@key]`, bracketed, and the bare form is refused by measurement (decision, recorded)

**Pandoc's spelling is adopted because a reader arriving from Pandoc already types it**,
and nothing in `mpdf-001` argues otherwise: its §1.1 refuses to *embed* Pandoc — heavy,
no lightweight WASM build, GPL linking questions — and its `reference` calls Pandoc's
pipeline the inspiration. A syntax is not a dependency.

**The bare `@key` form is not adopted, and the reason is measured rather than
aesthetic.** Run through the built CLI on 2026-08-22:

```
an email a@b.com  →  an email a\@b.com
```

An unbracketed `@` is load-bearing in text that has nothing to do with citation.
Pandoc needs a special-case rule for exactly this, and a second one would be needed the
day the cross-reference realignment in §1.2 lands, because `@fig:one` and `@smith2020`
are then the same sigil disambiguated only by a prefix — and **`mpdf-005` ruled that the
prefix is not a kind**, so this dialect has no such disambiguator and would have to
invent one. `[@key]` is unambiguous today and stays unambiguous after that move.

**What `[@key]` costs is text that currently prints**, which is the shape `mpdf-005`
OQ-8 worked and shipped: a `[@key]` in a document today reaches the page as
`\[\@smith2020\]`, visible and meaningless. A census of `tests/fixtures/`, `samples/` and
the README is Phase 1's, on that precedent.

**The Typst side is `#cite(<key>)` and not `@key`**, for the reason `mpdf-005` OQ-8's
own CORRECTED note gives one construct over: the marker form takes its label from the
surrounding text and an adjacent character silently joins it, where the function form
takes an argument that ends where the parenthesis does.

### The bibliography is a file the frontmatter names, and the records are not in the frontmatter (decision, recorded)

`core/src/frontmatter.rs` reads six keys and refuses a seventh by name — an unknown key
is `frontmatter error at line N: unknown key '…'`, which means **a document carrying
`bibliography:` today already fails loudly rather than silently ignoring it.** The key
becomes the seventh.

The alternative was to carry the records *in* the frontmatter, and it is refused. It
would cost no new channel at all — no second file, no watch-loop entry, and the browser
would work unchanged, which is not nothing. But it would put a bibliography in every
document that cites it rather than in one file several documents share, which is the
thing a bibliography is *for*; and it would mean inventing a record format, or embedding
Hayagriva's inside YAML that `frontmatter.rs` parses with a hand-written line reader.
**A format Typst already reads, in a file the author already has, is the smaller change
and the larger capability.**

**Typst 0.15.1 takes Hayagriva `.yaml`/`.yml` and BibLaTeX `.bib`**, and takes them
either as paths or as raw bytes. Both extensions are accepted; nothing is gained by
choosing one, and OQ-4 records why that is not a design call worth making.

### The file rides the `Asset` type unchanged, and what is genuinely new is the shopping list (decision, recorded)

`mpdf-005` OQ-5 predicted the cost as "a new asset kind across `core/src/lib.rs:Asset`,
`core/src/lib.rs:image_paths` and `mpdf-003`'s watch loop". **Measured 2026-08-22, that
prediction is right about the third and wrong about the first two, and the correction is
what makes Phase 1 small.**

- **`core/src/lib.rs:Asset` needs no change.** It is `{ path: String, bytes: Vec<u8> }` —
  a named blob, with nothing image-specific in it.
- **`core/src/lib.rs:TypstWorld` needs no change.** Its `file` already answers from the
  asset map by `FileId` before falling through to sources, and its own doc says "an image
  asks for its bytes here, and so does a template". A `.yml` asks the same way.
- **`core/src/lib.rs:collect` does need one.** It iterates the *image* list alone and
  checks each against `bytes_match`, so a bibliography would never enter the map and, if
  it did, would be refused for not holding PNG or SVG data. It grows a second, unchecked
  entry — unchecked because Typst parses the file itself and names its own error, where
  an image's magic bytes are the only thing that could.
- **The shopping list is the real new thing.** `core/src/lib.rs:image_paths` is derived
  from the walk — a document's images are found by reading it. A bibliography path is not
  walked; it is one frontmatter value. So a caller needs a second way to be told, and
  that is a new export rather than a widening of `image_paths`, whose name and contract
  `mpdf-002` fixed and three callers cite.

**Every caller reads files and `core` reads none**, which is `mpdf-001`'s split and is
not touched: `cli/src/main.rs` opens the bibliography beside the images it already opens,
and hands back bytes.

### The list is placed by the emitter and dressed by the look (decision, recorded)

Typst's `bibliography` renders where it is placed, so something must place it. **The
emitter appends it after the document's own content**, which is where a reference list
goes and is the only placement the markdown gives it — the source names the file in the
frontmatter and never names a position.

What it *looks* like is the look's, on `mpdf-001` §2's own seam: `core/assets/template.typ`
and `core/assets/press-release.typ` decide type, and a two-column look has an opinion
about a reference list that a one-column look does not. OQ-5 carries whether the heading
above it is the emitter's or the look's, and it is a real question rather than a
formality — a heading the emitter writes appears in the anchors `mpdf-003` Phase 6
reports, and one the look writes does not.

### The browser is the one front end this cannot reach, and it already has the answer (decision, recorded)

`mpdf-006` §1.2 parks a *reader's own files* permanently: a browser has no filesystem, so
a `bibliography: refs.yml` typed into the demo has nowhere to read from. That is the same
limit the image story met, and `mpdf-006` Phase 3 answered it the narrowest possible way
— one file, owned by the page, carried inline in a `<script data-asset>` element.

**Phase 4 is that answer applied a second time, and Typst's own API makes it cheaper than
the first.** `bibliography` accepts raw bytes as well as a path, so a page-owned
bibliography needs no new virtual file at all. The phase is cuttable: nothing in Phases
1–3 depends on it, and a demo that cannot show a citation is a demo with a gap rather
than one that lies.

## 3. Open questions

- **OQ-1 — does `core` parse the bibliography to name an unknown key, or map Typst's
  diagnostic?** *(design call)* The dialect's rule is that nothing is dropped and every
  refusal names its line — `core/src/emit.rs:describe` and `Error::Math` both do. A key
  the file does not hold is that same failure, and Typst reports it in its own words with
  a Typst position, which `Error::Compile` would pass through as "typst compilation
  failed: …". Parsing the file in `core` buys the author's own line number and costs a
  BibLaTeX/Hayagriva reader this project would then own. Phase 2 is where it lands.
- **OQ-2 — which citation style, and is it frontmatter's or the look's?** *(design call)*
  Typst's default is IEEE-numeric. `equations: numbered` is precedent for a frontmatter
  key that selects a rendering, and `template:` is precedent for the look owning one.
- **OQ-3 — do the locator and suppressed-author forms land at all?** *(needs-input)*
  `[@key, p. 33]` and `[-@key]` are Pandoc's and Typst has `#cite(<k>, supplement: …)`
  and `form: "year"`. Phase 1 refuses them by name rather than guessing; whether a later
  phase adds them is not this spec's to assume.
- **OQ-4 — `.bib`, `.yml`, or both?** *(deferred by evidence)* Typst 0.15.1 reads both
  and the emitter writes one string either way, so **the question does not discriminate**
  — there is nothing to decide until something costs different. Both are accepted.
- **OQ-5 — does the reference list carry a heading, and whose is it?** *(design call)*
  §2 records the consequence: a heading the emitter writes enters `mpdf-003` Phase 6's
  anchor list, and one the look writes does not.
- **OQ-6 — does Phase 4 happen at all?** *(needs-input)* The same question `mpdf-006`
  OQ-3 asked of its own image phase, with the same shape: it opens a narrow slice of a
  story two specs parked, and Phases 1–3 stand without it.

## 4. Implementation phases

Strictly sequential. Phase 2 refines the error Phase 1 produces; Phase 3 watches the file
Phase 1 introduces; Phase 4 carries one into a front end that cannot open it.

### Phase 1 — a cited source reaches the reference list

*Produces the observable: **yes**.* A markdown file naming a bibliography and citing a
key compiles to a PDF with a mark in the body and a reference list at the end — the first
instance of the observable this project has never been able to produce.

- **Scope:** `core/src/frontmatter.rs` gains the seventh key; `core/src/emit.rs` maps a
  `[@key]` shortcut link to `#cite(<key>)` and appends `#bibliography(…)` when the key is
  present; `core/src/lib.rs` grows the new shopping-list export and `collect` admits one
  unchecked asset; `cli/src/main.rs` reads the file. `TypstWorld` and `Asset` are
  untouched, per §2. A census of `tests/fixtures/`, `samples/` and the README establishes
  that no existing document carries a `[@…]` shortcut link, on `mpdf-005` OQ-8's
  precedent. The locator and suppressed forms are refused by name (OQ-3).
- **Exit gate:** `cargo test --workspace`, with a new fixture/golden pair under
  `tests/fixtures` and `tests/golden` proving the emitted Typst, and a compile assertion
  proving the PDF. A document naming no `bibliography` key emits **byte-identical** Typst
  to what it emits today — asserted over every existing golden, which is what makes the
  no-change claim in §1.2 checkable rather than asserted.
- **Close-out:** `rules/pipeline.md` carries the seventh key, the citation construct, the
  second asset channel and the new export. README gains the construct and moves the
  supported count off twenty-two. One push.

### Phase 2 — a key the bibliography does not hold is named, not compiled

*Produces the observable: **no**, and it is argued.* It produces no new PDF; it replaces a
Typst diagnostic with the dialect's own sentence. **It earns its place because the
rejection rule is the one thing `mpdf-001` §2 makes non-negotiable** — a refusal that
names its construct and its line, in the words the CLI prints — and a citation is the
first construct whose failure currently escapes as `typst compilation failed: …`.

- **Scope:** OQ-1 decides it. Either `core` reads the file and names the key with the
  author's own line, or the Typst diagnostic is mapped to a new `Error` variant.
- **Exit gate:** a fixture citing an absent key returns `Err` whose `to_string()` names
  the key and the line, asserted in `core/tests/golden_test.rs` and at the CLI.
- **Close-out:** `rules/pipeline.md`'s rejection section. One push.

### Phase 3 — the desktop app watches the bibliography

*Produces the observable: **yes**.* Editing the `.yml` re-renders the pane, which is the
whole promise `mpdf-003` makes about a document's files.

- **Scope:** `app/src/watch.rs` — the loop watches the document and the images it names;
  the bibliography joins that list, through the same export Phase 1 added.
- **Exit gate:** an integration check that changing the bibliography alone re-renders,
  matching how the image watch is already tested.
- **Close-out:** `rules/desktop.md`'s watch section. One push.

### Phase 4 — the browser carries a bibliography of its own

*Produces the observable: **yes**, and **this phase is cuttable** (OQ-6).* The demo shows
a citation for the first time.

- **Scope:** `web/index.html` carries the records inline as `mpdf-006` Phase 3 carries the
  SVG, one row uses them, and `web/src/lib.rs:render` passes them. `core/src` untouched.
  §2 records that Typst's raw-bytes form may make this cheaper than the image was.
- **Exit gate:** `core/tests/page_examples_test.rs`'s counts move together and the new
  row compiles; in a browser, the row draws a PDF carrying the reference list.
- **Close-out:** `rules/web-demo.md`. One push.
