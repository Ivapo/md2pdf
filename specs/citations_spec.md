---
id: mpdf-007
title: citations-and-bibliography
note: >
  A document cites its sources and prints their reference list: the frontmatter names a
  bibliography file, the caller supplies it as bytes beside the images, `[@key]` becomes a
  citation, and Typst renders both the marks and the list.
status: accepted
last_updated: 2026-08-23

phases:
  - name: "Phase 1 — a cited source reaches the reference list"
    reviewed: 2026-08-22
    shipped: 2026-08-22
    cut: null
    by: null
  - name: "Phase 2 — a key the bibliography does not hold is named, not compiled"
    reviewed: 2026-08-23
    shipped: 2026-08-23
    cut: null
    by: null
  - name: "Phase 3 — the desktop app watches the bibliography"
    reviewed: 2026-08-23
    shipped: 2026-08-23
    cut: null
    by: null
  - name: "Phase 4 — the browser carries a bibliography of its own"
    reviewed: 2026-08-23
    shipped: 2026-08-23
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
- **No change to how a document carrying no citation compiles.** A file with no `[@…]`
  in it, and naming no bibliography, emits exactly the Typst it emits today, byte for
  byte. **This is deliberately narrower than the draft's "no document without a
  bibliography changes"**, which round 1 showed was incompatible with the citation being
  in the dialect at all: a `[@key]` in a document that names no bibliography is a
  *refusal* under §2, not literal text, so that document does change and should. A
  document that names a bibliography and cites nothing gains the list and its label —
  the promise is about documents that do neither.
- **Pandoc's prefix form is out of reach, and stated rather than discovered.** `[see @k]`
  and a bracketed email `[a@b.com]` stay literal text: §2's callback fires on a reference
  beginning `@` or `-@` and nothing else. This dialect has no prefix form, so they print
  for the reason any unclaimed markdown prints.

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

**The Typst side is `#cite(…)` and not `@key`**, for the reason `mpdf-005` OQ-8's
own CORRECTED note gives one construct over: the marker form takes its label from the
surrounding text and an adjacent character silently joins it, where the function form
takes an argument that ends where the parenthesis does.

**And the argument is `label("key")` rather than `<key>`, which is where this diverges
from `mpdf-005` on purpose.** Measured in round 1: `#cite(<DBLP:books/lib/Knuth86a>)`
fails to *parse* — "unclosed label; unexpected slash" — with no line and no construct,
because Typst's label syntax admits a narrower character set than a bibliography key does.
`label` is a constructor function over a string, so `#cite(label("DBLP:books/lib/Knuth86a"))`
carries any key a `.bib` file holds. `mpdf-005` could constrain its names through
`core/src/emit.rs:check_name` because a figure name is authored inside this dialect; **a
citation key is authored in a file the author often did not write and cannot change**, so
constraining it would refuse real bibliographies rather than protect anyone.

**One consequence is worth recording because nothing warns about it.** Typst's labels are
one namespace, so a figure named `{#smith2020}` in a document whose bibliography holds
`smith2020` fails with "label `<smith2020>` occurs both in the document and a
bibliography". Measured in round 1. `check_name` is where a named error for that would
live; the message is Phase 2's, and Phase 1 records the collision rather than discovering
it.

> **CORRECTED 2026-08-23, by Phase 2's round 1.** The paragraph above is kept as it was
> written and is wrong in both of its operative claims. **The collision needs a third
> ingredient**: measured against shipped Phase 1 over the whole matrix, a figure named
> `{#smith2020}` in a document whose bibliography holds `smith2020` **compiles clean**,
> and so does the same document citing `[@smith2020]`. The message fires only where a
> **cross-reference** points at the shared label — `[](#smith2020)` — and it fires there
> whether or not the key is also cited. That is consistent with where the message
> actually lives, which is `typst-library-0.15.1/src/model/reference.rs` and not
> `bibliography.rs`: it is raised while *resolving a reference*, not while realising a
> bibliography. **So `check_name` is the wrong home**, and not by a little: it runs at
> *declaration*, inside the walk, where neither the reference nor the bibliography's
> contents are known. `core/src/emit.rs:check_references` is the function that already
> owns this class — a reference Typst would refuse with a message naming a label and no
> line — but it too runs inside `emit`, which never sees asset bytes, so the check itself
> lands beside `collect` where the bytes are. Phase 2's scope carries the corrected
> version.

### `[@key]` is not a link until a scoped callback makes it one (decision, recorded)

**The draft said the emitter "maps a `[@key]` shortcut link", and there is no such
event.** Measured in round 1 and reproduced: under `core/src/emit.rs:options`,
`See [@smith2020] ok.` parses to five `Event::Text` runs — `"See "`, `"["`,
`"@smith2020"`, `"]"`, `" ok."` — and no `Tag::Link` at all. A CommonMark shortcut
reference link is only a link when a matching reference definition exists, and a
citation never has one. **The central mechanism of Phase 1 did not exist.**

Two routes were available and only one survives. Stitching the three text runs back
together in the walk is refused: `core/src/emit.rs` has no inline cross-event machinery —
`caption_name` and `equation_name` both work on a single run — and building some would put
a second, hand-written inline parser beside `pulldown-cmark`, which is the thing
`mpdf-001` §2 chose a parser to avoid.

**So: `Parser::new_with_broken_link_callback`, over a callback that fires on a reference
beginning `@` or `-@`.** `pulldown_cmark::BrokenLink` carries the `reference` text, and
returning `None` leaves the source exactly as it is. **The `-@` half is not decoration:**
round 2 measured that an `@`-only predicate leaves `[-@k]` — Pandoc's suppressed-author
form, which OQ-3 refuses by name — as five `Text` runs the emitter never sees, so it would
have reached the page as `\[\-\@k\]`: a refusal the spec promised and the parse could
not deliver. Measured over the whole hostile set:

| source | with the callback |
|---|---|
| `See [@smith2020] ok.` | `Link { ShortcutUnknown, id: "@smith2020" }` |
| `Suppressed [-@smith2020] here.` | `Link { ShortcutUnknown, id: "-@smith2020" }` |
| `[@a; @b]`, `[@smith2020, p. 33]` | one `Link`, whole payload in `id` |
| `Collapsed [@k][] form.` | `Link { **CollapsedUnknown**, id: "@k" }` |
| `an [ open bracket, a ] close bracket` | **byte-identical to today** |
| `a[0]`, `[see this]`, `[](#fig:one)` | **byte-identical** |
| `[link](http://x.com)`, `![alt](dot.png)`, `[ref][d]` | **byte-identical** |
| `an email a@b.com and a bare @thing` | **byte-identical** |
| `a prefix form [see @k]`, `[a@b.com]` | **byte-identical** |

Three things that table settles, each of which round 2 raised:

- **`[@k][]` is `CollapsedUnknown`, not `ShortcutUnknown`.** The emitter matches both. One
  arm alone would send the collapsed form to the generic link arm as `#link("@k")[@k]` — a
  wrong document where today it is literal text, which is worse than either.
- **The callback returns the reference as the destination**, never an empty string: the
  existing `Tag::Link` arm errors on an empty destination, and the value also reaches
  `core/src/lib.rs:md_to_html`, where a citation renders as `<a href="@k">@k</a>`. That is
  honest for `mpdf-006`'s comparison column — a writer with no notion of citations makes a
  dangling link of one — and it is Phase 4's row to show, not Phase 1's problem.
- **The boundary is `@` or `-@` at the start, and nothing else.** Pandoc's prefix form
  `[see @k]` and a bracketed email `[a@b.com]` stay literal text. That is not a silent
  drop of something the dialect claims: this dialect has no prefix form, so `[see @k]`
  prints for the same reason any unclaimed markdown prints. §1.2 names it so it is a
  stated boundary rather than a discovered one.

**`link_type` rides `Tag::Link` at `Start` while the emitter decides at
`End(TagEnd::Link)`**, so the discriminator is carried on the existing `LinkFrame` rather
than read at the end. Ordinary work, named because the scope otherwise reads as one arm.

**Both backends must take the same constructor, or `md_to_html` stops telling the truth.**
`mpdf-006` Phase 4 rests on the demo's two columns coming out of "one parse with one set
of options". A callback is not part of `Options`, so `emit` exposes a parser constructor
rather than options alone, and `core/src/lib.rs:md_to_html` calls it. Measured: a plain
`fn` pointer satisfies `BrokenLinkCallback`, so the constructor returns a concrete
`Parser<'_, Cb>` with no boxing. **The demo's generated column needs no re-blessing** —
censused in round 1, none of `web/index.html`'s eleven examples contains `[@`, so every
stored block is byte-identical.

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

**A citation in a document that names no bibliography is refused, not printed.** Round 1
found the draft ambiguous here and the ambiguity was load-bearing: mapping only when the
frontmatter key is present would leave `[@smith2020]` reaching the page as
`\[\@smith2020\]` — visible, meaningless, and exactly the silent flattening
`mpdf-001` §2 refuses for every other construct. So the mapping is unconditional and the
missing bibliography is named with its line, in the dialect's own words rather than
Typst's "the document does not contain a bibliography", which carries neither construct
nor line. §1.2's non-goal is narrowed to match, because a promise that no document
without a bibliography changes cannot survive the citation being in the dialect at all.

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
- **A bibliography the caller did not supply is refused by name.** `collect` exists so a
  missing file is named with the author's own path before Typst is asked — without it the
  compile says "file not found (searched at refs.yml)" with no line. `Error::MissingImage`
  is the shape and the wrong words, so a sibling variant carries the bibliography's.
  **Its line comes from the shopping list**: the new export returns the path *with* the
  line it was declared on, in the shape `core/src/lib.rs:ImageRef` already has. Only
  `frontmatter::parse` ever sees that line today, so it is what carries it out — as a
  field beside the key or as a second return value, which are indistinguishable in
  behaviour and so are the implementer's to pick.
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
about a reference list that a one-column look does not.

**The emitter writes `title: none`, and this is not a style preference — it is what stops
Phase 1 silently breaking the desktop app.** Measured in round 1: `BibliographyElem`'s
default `title: auto` realises a real `HeadingElem`, so a document with one markdown
heading and a bibliography queries **two** headings where the walk counted one — and
`core/src/lib.rs:anchors_from` returns an **empty** vector on a count mismatch, by design.
Every document with a bibliography would therefore lose *all* its heading anchors, taking
`mpdf-003` Phase 6's scroll sync and `web/src/lib.rs:anchors` with it, with no error
anywhere. `title: none` keeps the counts equal.

**So the label above the list is the look's, and it must not be a heading.** Each look
gains a `show bibliography:` rule that prepends styled text — beside the `show figure:`
rules both already carry — which is the same seam and costs the anchors nothing. This
resolves OQ-5 rather than deferring it: Phase 1 cannot emit the call without choosing, so
a phase that left it open would be a phase that forced a guess.

**And both look files are in Phase 1's scope, which round 2 found the draft had left
out.** `core/assets/template.typ` and `core/assets/press-release.typ` are where the rule
goes. Without them `title: none` reaches looks that draw nothing, and the reference list
runs straight on from the last paragraph with no label at all — and with OQ-5 closed,
nothing would ever have forced one.

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

> **CORRECTED 2026-08-23, by Phase 4's round 1.** The paragraph above is kept as it was
> written and its cost claim is backwards in both halves. **This project never reaches
> Typst's bytes form.** `core/src/emit.rs:emit` writes `#bibliography("<path>", title:
> none)` through `typst_string`, and `core/src/lib.rs:collect` inserts the bibliography's
> bytes into the virtual filesystem keyed by `file_id` exactly as it does an image's — so
> the bibliography *is* a virtual file here, and a page-owned one inherits that rather
> than escaping it. **And reaching the bytes form would cost Phase 2.** Measured in round
> 1 against `decode_library`, in `typst-library-0.15.1/src/model/bibliography.rs`: the
> path branch dispatches on `ext.to_lowercase()`, and the bytes branch **guesses** — Hayagriva
> YAML first, BibLaTeX second. Phase 2 shipped a refusal naming an extension outside the
> pair, and a guess has no extension to refuse. So Phase 4 sends its second file down the
> channel the image already uses, and its real cost is the one the draft never named:
> **the page's asset channel is singular in four places.** Phase 4's scope carries the
> corrected version.

## 3. Open questions

- **OQ-1 — does `core` parse the bibliography to name an unknown key, or map Typst's
  diagnostic?** *(design call)* ~~Parsing the file in `core` buys the author's own line
  number and costs a BibLaTeX/Hayagriva reader this project would then own.~~
  **RESOLVED 2026-08-23, in Phase 2's round 1: `core` parses the file, through
  `hayagriva`.** The dialect's rule is that nothing is dropped and every refusal names its
  line — `core/src/emit.rs:describe` and `Error::Math` both do. A key the file does not
  hold is that same failure, and Typst reports it in its own words with a Typst position,
  which `Error::Compile` passes through as ``typst compilation failed: citation key
  `nosuchkey` is not present in the bibliography`` — the key, and no line. **The cost that
  made this a design call was measured and is not there.** `hayagriva 0.10.1` is already a
  *direct* dependency of `typst-library 0.15.1`, and `biblatex 0.12.0` arrives under it,
  so both are already compiled into every build including the `wasm32` one: naming
  `hayagriva` in `core/Cargo.toml` adds **no crate to the tree**. It exposes exactly what
  is needed and covers both formats OQ-4 accepts — `io::from_yaml_str`,
  `io::from_biblatex_str`, `Library::get` and `Library::keys` — so this project owns no
  reader of its own. **And because it is the same reader at the same version Typst uses,
  `core`'s key set and Typst's cannot disagree**, which is the faithfulness risk any
  second parser would have carried. The rejected branch is not impossible — Typst's
  message names the key, and `core/src/emit.rs:Names::cited` already holds every key with
  its line, so a key→line lookup would work — but it means matching a diagnostic's
  *wording*, which a version bump moves silently, and it leaves the collision message of
  §2 with no route at all, since that one needs the key set rather than a message.
- **OQ-2 — which citation style, and is it frontmatter's or the look's?** *(design call)*
  Typst's default is IEEE-numeric. `equations: numbered` is precedent for a frontmatter
  key that selects a rendering, and `template:` is precedent for the look owning one.
- **OQ-3 — do the locator and suppressed-author forms land at all?** *(needs-input)*
  `[@key, p. 33]` and `[-@key]` are Pandoc's and Typst has `#cite(<k>, supplement: …)`
  and `form: "year"`. Phase 1 refuses them by name rather than guessing — **both are
  within the callback's reach, measured in round 2**, which is what makes "refuses by
  name" something the parse can actually deliver. Whether a later phase adds them is not
  this spec's to assume.
- **OQ-4 — `.bib`, `.yml`, or both?** *(deferred by evidence)* Typst 0.15.1 reads both
  and the emitter writes one string either way, so **the question does not discriminate**
  — there is nothing to decide until something costs different. Both are accepted.
- **OQ-5 — does the reference list carry a heading, and whose is it?** *(design call)*
  **RESOLVED 2026-08-22, in round 1: the emitter writes `title: none`, and the look draws
  the label as styled text rather than a heading.** It stopped being a design call the
  moment it was measured: Typst's default `title: auto` realises a real `HeadingElem`,
  which makes the walk's heading count disagree with the compiled document's, and
  `core/src/lib.rs:anchors_from` withdraws **every** anchor on a mismatch. Leaving this
  open would have shipped a phase that silently breaks `mpdf-003` Phase 6. §2 carries it.
- **OQ-6 — does Phase 4 happen at all?** *(needs-input)* ~~The same question `mpdf-006`
  OQ-3 asked of its own image phase, with the same shape: it opens a narrow slice of a
  story two specs parked, and Phases 1–3 stand without it.~~ **RESOLVED 2026-08-23, at
  Phase 4's round 0: yes, on `mpdf-006` OQ-3's own precedent — the phase was asked for.**
  That spec closed the identical question the same way and for the same reason: the ask
  *is* the answer, and this loop is the gate that ask passes through. Round 0 also found
  the phase understating its case. It produces the observable, and it produces the
  instance §1 says nothing else can — a document whose last page is a reference list — in
  the one front end that has never drawn one; and the page's own claim has been
  incomplete since Phase 1, `rules/web-demo.md` describing the examples as what the
  dialect adds while the dialect has held citations for a day. **The cuttability argument
  is therefore spent**, and Phase 4's own text no longer leans on it.

## 4. Implementation phases

Strictly sequential. Phase 2 refines the error Phase 1 produces; Phase 3 watches the file
Phase 1 introduces; Phase 4 carries one into a front end that cannot open it.

### Phase 1 — a cited source reaches the reference list

*Produces the observable: **yes**.* A markdown file naming a bibliography and citing a
key compiles to a PDF with a mark in the body and a reference list at the end — the first
instance of the observable this project has never been able to produce.

- **Scope:** six files, and §2 settles each.
  - `core/src/emit.rs` exposes a **parser constructor** rather than `options` alone, built
    with `Parser::new_with_broken_link_callback` over a callback firing on a reference
    that begins `@` or `-@` — §2's block, with every hostile case measured.
    `core/src/lib.rs:md_to_html` is repointed at it, so `mpdf-006`'s two columns keep
    coming out of one parse. (`core/src/math.rs` holds a fourth `Parser::new_ext` call,
    checked in round 2: it is inside `#[cfg(test)] mod tests` and filters `InlineMath`
    only, so it is not a call site this change must follow.) **Both of `emit.rs`'s own
    parser sites take the constructor** — `collect_definitions` and `emit`, the two walks
    footnotes need — or a citation inside a footnote definition would print literally
    while the same text in the body cited.
  - `core/src/emit.rs` maps the resulting link to `#cite(label("key"))` — matching **both**
    `LinkType::ShortcutUnknown` and `LinkType::CollapsedUnknown`, since `[@k][]` is the
    latter and one arm alone would emit `#link("@k")[@k]`, a wrong document where today
    there is literal text. The discriminator rides `Tag::Link` at `Start` and the decision
    is made at `End`, so it is carried on the existing `LinkFrame`. It appends
    `#bibliography(…, title: none)` when the frontmatter names one — **the key reaching
    `label(…)` through `core/src/emit.rs:typst_string`**, which already escapes a `"` or a
    `\` for URLs and is the reuse rather than a second escaper — and **refuses by name**,
    each with its line: a citation with no bibliography declared, and the payloads
    `[@a; @b]`, `[@k, p. 33]` and `[-@k]` (OQ-3).
  - `core/src/frontmatter.rs` gains the seventh key.
  - `core/src/lib.rs` grows the shopping-list export — returning the path **with the line
    it was declared on**, in `ImageRef`'s shape — `collect` admits one unchecked asset,
    and a `MissingBibliography` variant mirrors `MissingImage`. `Asset` and `TypstWorld`
    are untouched, per §2 and confirmed in round 1.
  - `core/assets/template.typ` and `core/assets/press-release.typ` each gain a
    `show bibliography:` rule drawing the label as styled text, beside the `show figure:`
    rules they already carry. **Without this the list arrives unlabelled**, `title: none`
    having removed Typst's own — round 2's second blocker.
  - `cli/src/main.rs` reads the file.
  - A census of `tests/fixtures/`, `samples/`, `README.md` and `web/index.html` establishes
    that nothing in the corpus carries `[@` — run in round 1, **zero occurrences** — which
    is what makes the byte-identical claim in the gate provable rather than hopeful.
  - **The desktop app is knowingly left behind**: `app/src/document.rs:read_assets_with`
    builds its asset list from `image_paths` alone, so a document naming a bibliography
    fails in the app until Phase 3. It fails there today too, on the unknown key, so this
    is a gap and not a regression — but it is named here rather than discovered there.
- **Exit gate:** `cargo test --workspace`, and six things it must contain.
  1. A new fixture/golden pair under `tests/fixtures` and `tests/golden` proving the
     emitted Typst, plus a compile assertion. The compile is stronger than "it runs":
     `#cite` compiles only when the key resolves against a present bibliography, so a
     green compile proves both the mark and the list reached the document. The fixture's
     key **must carry a `:` and a `/`** — `DBLP:books/lib/Knuth86a` is the measured case —
     since that is what fails under `<key>` and passes under `label("key")`.
  2. **Every existing golden is byte-identical and no golden file is edited.** The goldens
     are hand-written `include_str!` constants in `core/tests/golden_test.rs` with no
     walking harness, so "unchanged" means the existing assertions still pass *and* the
     diff touches no file under `tests/golden/`. Stated because the first half alone is
     satisfied by editing a golden to match.
  3. **`tests/golden/hostile.typ` is the named teeth.** Its fixture carries
     `an [ open bracket, a ] close bracket`; an unscoped broken-link callback turns that
     into a link and moves the golden. This is the one assertion that can fail for the
     right reason, and round 1 showed that without naming it the byte-identical check
     passes on the status quo whichever way the phase is built.
  4. **Both looks draw the label.** **Two** fixtures, not one — a fixture carries one
     `template:` key, and `tests/fixtures/press_release.md` is the precedent for the pair —
     and the `show bibliography:` rule is asserted in each look the way
     `core/tests/golden_test.rs` already asserts look contracts, by naming the needle it
     must contain. Without this the phase's own resolution of OQ-5 is unenforced.
  5. **Each refusal is asserted by its sentence**, `[-@k]` included: it is the one round 2
     measured as unreachable under the narrower callback, so a gate that omits it would
     pass a phase that silently prints it.
  6. **The heading count is asserted.** A fixture with one markdown heading and a
     bibliography returns a non-empty `anchors` from
     `core/src/lib.rs:md_to_pdf_with_anchors` — which is what proves `title: none` was
     written, and which is the assertion whose absence would have let Phase 1 silently
     empty every anchor list in the project.
- **Close-out:** `rules/pipeline.md` carries the seventh key, the citation construct and
  its refusals, the parser constructor, the second asset channel and the new export; its
  **"Twenty-two things are supported" count is that file's own** and moves — round 1
  re-derived it, and `README.md` carries a prose list with no number, so the README gains
  the construct and no count. `rules/web-demo.md` notes that `md_to_html` now reads the
  shared constructor. One push.

### Phase 2 — a key the bibliography does not hold is named, not compiled

*Produces the observable: **no**, and it is argued.* It produces no new PDF; it replaces
three Typst diagnostics with the dialect's own sentences. **It earns its place because the
rejection rule is the one thing `mpdf-001` §2 makes non-negotiable** — a refusal that
names its construct and its line, in the words the CLI prints — and the citation channel
carries the three remaining failures that escape as `typst compilation failed: …`.
**Three rather than two**, round 2 having measured a third: `bibliography: refs.txt` over
a perfectly good Hayagriva file gives "unknown bibliography format (must be .yaml/.yml or
.bib)", reachable from ordinary markdown and naming no line, because
`core/src/frontmatter.rs`'s `bibliography` arm validates `portable_path` and nothing about
the extension. **Not the *first***:
round 1 measured a PNG corrupt past its magic bytes doing the same, and `rules/pipeline.md`
already documents that one as a recorded limit rather than a gap. These two are gaps,
because both are reachable from ordinary markdown and neither names a line.

**OQ-1 is resolved and this phase builds its answer**, on the precedent OQ-5 set for Phase
1: a phase that left the question open would be a phase that forced a guess.

- **Scope:** six files.
  - `core/Cargo.toml` gains `hayagriva = "0.10.1"` — the version `typst-library 0.15.1`
    already pins, so **the tree gains no crate** and the `wasm32` build gains nothing.
    OQ-1 records the measurement.
  - **`core/src/bibliography.rs`, new**, and the only new module: bytes in, a key set out.
    `io::from_yaml_str` for `.yml`/`.yaml` and `io::from_biblatex_str` for `.bib`,
    dispatched on the extension through `core/src/emit.rs:extension_of` — already
    `pub(crate)`, and already the function Typst's own format detection reads, so the two
    cannot disagree about where a name ends. **Three arms, not two**: an extension outside
    the pair is the third failure named above, refused here rather than left to Typst.
    **And the dispatch folds case**, which is a seam round 2 measured: Typst's
    `decode_library` matches on `ext.to_lowercase()` where `extension_of` returns the
    extension unfolded, so `bibliography: refs.YML` compiles clean today and would be
    neither format to a naive `match`. `core/src/lib.rs:bytes_match` carries the same
    shape, so this is a clause rather than a new rule.
    **A file whose own parse fails is its own refusal**, named with the frontmatter line,
    rather than a panic or a Typst diagnostic: nothing checks the extension against the
    content, so a `.yml` holding BibLaTeX reaches this module too.
  - `core/src/emit.rs` exposes what the walk already computes. `Names::cited` holds every
    key with its line and `Names::referenced` every referenced name with its line, and
    **neither leaves `emit` today** — both become fields on `core/src/emit.rs:Emitted`,
    the way `bibliography` did in Phase 1. Nothing about the walk changes.
  - `core/src/lib.rs` runs both checks **beside `core/src/lib.rs:collect`, before the
    compile**, which is the only place the bibliography's bytes exist — `emit` never sees
    an asset, as `md_to_typst`'s own doc says. `Error::Citation { line, problem }` already
    exists and already carries a line, so **no new variant is added**; Phase 1's own doc
    comment on it already claims this ground.
    - *An absent key* names the key and the **citation's own markdown line**, from
      `Names::cited`.
    - *The collision*, per §2's `CORRECTED` note: a name that is declared in the document,
      present in the bibliography's key set, **and referenced** — the reference is the
      trigger, measured over the whole matrix — named at the **reference's** line.
    - **Where several are refused, the earliest line is the error**, by `min_by_key` over
      a `Vec`, which is what `check_references` and `check_citations` already do and for
      the reason they record: "the first" out of a set varies between runs. **The rule
      reaches `collect`'s own refusals too**, which round 2 found the draft had left
      scoped to the two new checks: a document with a missing image on line 3 and an absent
      key on line 9 has two candidate errors, and `collect`'s doc already argues the
      bibliography is checked before the images "since its line comes from the frontmatter
      and is therefore earlier than every image's". One rule, over every refusal this
      function can raise.
  - `core/tests/golden_test.rs` and `cli/tests/cli_test.rs` carry the gate.
- **One asymmetry is stated rather than discovered.** Every citation refusal Phase 1
  shipped is raised inside `emit`, so `--emit-typst` refuses them too. Both of this
  phase's need the bytes, so they can only run on the PDF path: `--emit-typst` will emit
  `#cite(label("nope"))` for a key nothing holds, and that is correct — emission reads no
  file on either channel, which `cli/tests/cli_test.rs:emit_typst_reads_no_bibliography`
  already pins.
- **Exit gate:** `cargo test --workspace`, and six things it must contain.
  1. A fixture citing an absent key returns `Err(Error::Citation)` whose `to_string()`
     names **the key and the citation's own line**, asserted in `core/tests/golden_test.rs`
     and at the CLI on a **compile** run — not `--emit-typst`, per the asymmetry above.
  2. **The collision is asserted too**, and by its own fixture: a figure named `{#k}`, a
     bibliography holding `k`, and a `[](#k)` pointing at it. Without this item the phase
     passes its gate with half its scope unbuilt — the defect round 2 rated blocking on
     Phase 1, when OQ-5's label was assigned to the looks with no look fixture. The
     fixture must carry the reference: the same document **without** it compiles clean,
     so a fixture missing it would assert nothing.
  3. **Where two keys are absent, the error names the one on the earlier line**, asserted
     over a document built so the two orderings differ. A gate that took either would pass
     a non-deterministic implementation. **The construction is the footnote splice**, and
     round 2 re-derived it after the resolution of OQ-1 made the draft's hint stale — core
     now refuses before the compile, so Typst's diagnostic ordering no longer participates
     in anything the gate can check. What can differ is `Names::cited`'s *vector* order
     against *line* order, because `core/src/emit.rs` extends `names.cited` from a
     definition's body at the **reference**: measured with the reference on line 8, a body
     citation on line 10 and the definition's on line 12, the vector is `[12, 10]` where
     the lines are `[10, 12]`, so `.first()` and `min_by_key` disagree. **A document with
     two plain body citations does not discriminate** — it comes out in document order —
     so a fixture built from the obvious shape would have no teeth.
  4. **A bibliography whose own parse fails is refused by name**, with the frontmatter
     line, asserted over a `.yml` holding something that is not Hayagriva.
  5. **Both accepted formats are exercised** — one `.yml` fixture and one `.bib` — since
     OQ-4 accepts both and the extension is what dispatches. `tests/fixtures/refs.yml`
     exists; the `.bib` is new. **A third case joins them**: an extension outside the pair
     is refused by name with the frontmatter line, which is the third failure the
     observable argument counts and which nothing else in this gate would force.
  6. **Every existing golden is byte-identical and no golden file is edited.** This phase
     emits no markup at all, so unlike Phase 1 **no golden may move for any reason** —
     which makes this the cheapest possible check that the walk was not disturbed.
- **Close-out:** `rules/pipeline.md`'s **`## Citations and the bibliography`** section —
  *not* a "rejection section", which that file does not have; the rejection rule sits under
  `## The dialect` and the citation failures under their own heading. It gains the two
  refusals, the reader and its module. **Its `max_lines: 760` has seven lines of headroom
  against 753 used, so the cap moves with the section.** `README.md`'s `## Citing sources`
  says "Four shapes are errors" and "a key the file does not hold fails the compile", and
  this phase falsifies both, so the count and that sentence move with it.
  `rules/web-demo.md` needs nothing: no export crosses to wasm that did not before, and
  the dependency was already in that build. One push.

### Phase 3 — the desktop app watches the bibliography

*Produces the observable: **yes**, and more of it than "re-renders" says.* Today a
document naming a bibliography does not merely fail to *update* in the app — it fails to
compile there at all: `app/src/document.rs:read_assets_with` builds its asset list from
`image_paths` alone, so `core/src/lib.rs:collect` raises `MissingBibliography` on every
pass. Phase 1 named that gap rather than leaving it to be discovered. This phase closes it
and then keeps the page current when the `.yml` changes, which is the promise `mpdf-003`
makes about every file a document names.

**The bibliography makes two journeys through this app, and its own round 1 found the
draft had named one of them and got its direction backwards.** The draft said the file
"must be **read** [in `read_assets_with`] … or there is nothing for a watch to
re-render". Measured against the shipped app: `read_assets_with` returns a `Vec<Asset>`
that reaches `md_to_pdf_with_anchors` and nothing else. What feeds the filter is
`app/src/document.rs:Render::images`, built by a **second and separate** `image_paths`
call in `app/src/document.rs:render_with`, which `app/src/preview.rs:Preview::compile`
copies into `Preview.images` and `app/src/preview.rs:Session::classifier` hands to
`app/src/watch.rs:classify`. So the journeys are independent: the **bytes** are what make
the document compile, and the **path** is what makes a change to it redraw. A phase built
to the draft's sentence would ship a document that compiles once and then never updates.

**The watch set needs nothing, and that is `mpdf-003`'s decision rather than a new one.**
Its "Why the watch set is the document's own directory" records one recursive watch on the
document's own directory, covering every path the dialect can legally name — and
`core/src/frontmatter.rs` puts the bibliography under the same `portable_path` rule every
image takes, so it already resolves inside. **"Watches it beside the images" was the wrong
verb**: nothing is watched per file. What changes is the *filter*, which that same section
fixes as the list's second job.

**The filter carries one list, renamed to hold what it now holds (decision, resolved
rather than deferred).** Three routes, on the precedent Phase 1's OQ-5 and Phase 2's OQ-1
both set — a phase that leaves this open is a phase that forces a guess.

- *Ride `Render::images` unrenamed.* Cheapest, and it leaves three shipped doc comments
  false at once: `Render::images` says "the image paths the document names",
  `classify` says "one of the paths `md2pdf_core::image_paths` returned", and
  `Change::Figure` says "one of the figures the document names". This project's whole
  discipline is that the documentation tracks the code, so that price is not payable.
- *Give it a `Change::Bibliography` of its own.* Honest, and it buys nothing: `on_change`
  would gain a branch identical to the one `changed.figures` already runs, and `Changed`
  a field no reader distinguishes. §5's "don't pre-abstract before there are real
  consumers" is exactly this case.
- **Chosen: one list, renamed.** `Render::images` becomes `Render::assets`,
  `Change::Figure` becomes `Change::Asset`, `Changed::figures` becomes `Changed::assets`.
  `Asset` is this project's own word for the set — `rules/pipeline.md` already calls these
  "the two asset channels", and `read_assets_with` already names the same set in bytes
  where these name it in paths. **No new arm**, because `mpdf-003`'s split is the open
  document against everything the disk supplies, and a bibliography sits on the second
  side for the reason a figure does: nothing but the disk supplies it, so its change is a
  bare recompile. And it is what makes a *dropped* `bibliography:` key free —
  `Preview::compile` replaces the whole list on every compile, so a path that stops being
  named disappears on its own, where a separate `Option<String>` field would have to be
  cleared by hand.

- **Scope:** three files, which is what resolving the route above settles.
  - `app/src/document.rs` — `read_assets_with` reads the bibliography through
    `core/src/lib.rs:bibliography_path`, **mirroring `cli/src/main.rs:read_assets`
    exactly**: first, seeded into the same `seen` set, and refused in the same sentence,
    `cannot read {file} for the bibliography at line {line}: {e}`. That wording is not
    decoration — `render_with`'s own doc records that a file which will not read is no
    `Error` at all and that this app owes the CLI the sentence. `render_with` then
    publishes the path on `Render`, from the same `Option` the images ride, so a document
    that did not parse still keeps the list the caller already had.
  - `app/src/watch.rs` — `classify` takes the one renamed list, and `Change::Asset` and
    `Changed::assets` are renamed with it. Its doc comment's "one of the paths
    `md2pdf_core::image_paths` returned" moves with them.
  - `app/src/preview.rs` — `Preview.images` follows the rename, and so do its readers:
    `Preview::compile` writes it, `Session::classifier` reads it, and
    `Session::on_change` reads the renamed `Changed::assets` beside it. **Both
    `classifier` and `on_change` are `Session` methods and not `Preview`'s**, which is
    the form `rules/desktop.md` already uses. **`on_change` gains no branch**, which is
    the whole point of the route chosen above.
- **Exit gate:** `cargo test --workspace`, and three things it must contain.
  1. **Changing the bibliography alone re-renders, and the page that results is good.**
     In `app/src/preview.rs`'s test module, on the harness the image tests already use —
     `counted()`, `wait_for()`, `settle()` and a real `Session::open` on a `scratch_dir` —
     a document, a `.yml` beside it, then a rewrite of the `.yml` alone. **The compile
     count moving is not the assertion.** `Session::on_change` calls `on_render()`
     whenever the asset mark is set, a *failed* compile included, so a counter alone
     passes an implementation that publishes the path and never supplies the bytes — half
     this phase, unbuilt, through its own gate. It must also assert
     `pdf().unwrap().starts_with(b"%PDF")` and `!is_stale()`, which is exactly what
     `preview.rs:a_saved_document_and_a_replaced_figure_each_compile_again` already does.
     This is the defect the record has rated blocking twice, at Phase 1 round 2 and Phase
     2 round 1.
  2. **A bibliography named before it exists is watched and then compiles**, the sibling
     of `preview.rs:a_figure_that_does_not_exist_yet_is_watched_and_then_compiles`: open
     on a document whose `.yml` is not beside it, assert the error names the path and
     `pdf()` is `None`, create the file, assert the compile lands and the page is good.
     **This is the case the directory watch exists for**, and it is the one that fails for
     an implementation publishing the path only out of a successful read — which
     `Render::images`' own doc says the image list is built to survive.
  3. **The rename moves no behaviour.** `watch::tests`' existing `classify` cases and
     `preview::tests`' image cases pass with nothing changed but the renamed identifiers.
     That is what `cargo test --workspace` is anchored for, and it is what makes item 1's
     "alone" provable rather than asserted.
- **Close-out:** `rules/desktop.md`, **three sections rather than one.** `## The file
  I/O`, where `read_assets_with` is documented as mirroring `cli/src/main.rs:read_assets`
  and `Render`'s list contract is stated; `## The watch loop`, where the filter and
  `Change`'s two arms are; and `## The session`, whose "a figure is a bare recompile …
  read the new figures on the way" is a sentence about the renamed arm and goes stale
  with it. **Its `max_lines: 435` has four lines of headroom against 431 used, so the cap
  moves with the sections.** One push.

### Phase 4 — the browser carries a bibliography of its own

*Produces the observable: **yes**.* The demo shows a citation for the first time: a row
whose PDF ends in a reference list, in the one front end that has never drawn one.
**OQ-6 is resolved and the cuttability argument is spent** — §3 carries the resolution
and round 0's reasoning.

**What makes this more than "one more row" is that the page's asset channel is singular
in four places, and round 1 found the draft had named none of them.**
`core/tests/page_examples_test.rs:the_page_carries_two_assets` — then named for the one
asset it allowed — asserts `PAGE.matches("data-asset=\"").count() == 1` and argues the
singularity in its own doc comment; `core/tests/page_examples_test.rs:image` — then
`asset` — builds one `Asset` from the *first* such element; `web/index.html`'s module does `querySelector('script[data-asset]')`, which
takes the first; and `web/src/lib.rs:render` takes `asset_path` and `asset_bytes` as two
scalar `wasm_bindgen` arguments. A second file is refused by all four, and §2's
`CORRECTED` note records why it cannot dodge them through Typst's bytes form.

**The one channel generalizes rather than gaining a second (decision, resolved).** Three
routes, on the precedent Phase 1's OQ-5, Phase 2's OQ-1 and Phase 3's filter route all
set — a phase that leaves this open is a phase that forces a guess.

- *A second attribute, `data-bibliography`.* Refused: a second mechanism, a second scan
  and a second selector for something the page already has one word for. `data-asset`'s
  value **is** the path `md2pdf_core::Asset` is given, which is as true of a `.yml` as of
  an `.svg`.
- *Records inside the row's own markdown.* Refused by §2 already — the frontmatter names
  a file and does not carry records — and it would make the demo's source something no
  reader could paste into the CLI, which is the one thing every row promises.
- **Chosen: one attribute, two elements, told apart by their `type`.** `data-asset` stays
  the name and `type` is the discriminator — which the page already reads and already
  tests: `core/tests/page_examples_test.rs:asset_media` returns that attribute, and it is
  what the `data:` URI substitution's own prefix is built from. Neither half of the
  mechanism is new. The new element is `<script type="application/yaml"
  data-asset="refs.yml">` — a non-JavaScript type like the other two, so it is not
  executed and its content needs no escaping.

**`render` takes the second file as two more scalars, and that is deliberately not an
asset array (decision, resolved).** `web/Cargo.toml` carries `wasm-bindgen` alone — no
`js-sys`, no `serde-wasm-bindgen` — so a `Vec<Vec<u8>>` across that boundary means a new
dependency on the page whose entire cost is its 7.8 MB. And the set is **closed**:
`mpdf-006` §1.2 parks a reader's own files *permanently*, so this page has exactly two
files and will never have three. §5's "don't pre-abstract before there are real
consumers" is this case exactly.

- **Scope:** four files, and the decisions above settle each.
  - `web/index.html` — a twelfth row, in **group 2, "Things markdown has no way to say"**,
    which is what a bibliography is. Its source names `bibliography: refs.yml` and cites a
    key the records hold; the records are the second `data-asset` element. **Three
    sentences of the page's own prose go stale and are in scope**: the lede's "eleven are
    shown here"; the byte-rule comment's "one of the eleven ends without a trailing
    newline"; and the filenote's "The box below reads one image file and no other:
    `pipeline.svg`", which after this phase reads two. The lede's "Twenty-two constructs
    are supported" is **already** stale against `rules/pipeline.md`'s twenty-three, moved
    by Phase 1 — it is corrected here rather than left, because this phase edits that
    sentence anyway. The generated column is **blessed, not written**, through the
    `#[ignore]`d `bless_the_generated_blocks`; §2 records what it will hold, a citation
    reaching `md_to_html` as `<a href="@k">@k</a>`, which is the honest thing for a
    writer with no notion of citations to make of one. **Group 2's own intro carries a
    link row** — `Frontmatter · Footnotes` — which gains a third entry pointing at the
    README's citations section.
  - `web/src/lib.rs` — `render` gains `bibliography_path` and `bibliography_bytes`, and
    **`web/src/lib.rs:anchors` takes the same two files**. It passes `&[]` today, so it
    would answer `MissingBibliography` for a document this page itself ships. Nothing
    calls it, but an export that cannot answer for the page's own source is a gap in
    `mpdf-003` Phase 6's contract rather than a limit worth recording, and the shape is
    already being decided one function above.
  - `core/tests/page_examples_test.rs` — **named in scope rather than left to the gate**,
    which is where round 1 found the draft's "`core/src` untouched" hiding it: `core/src`
    *is* untouched, and this file is not `core/src`. `asset` becomes the pair, the
    singular scan becomes a count keyed to a constant beside `EXPECTED`, and both
    `let assets = [asset()]` slices take both files. **The `data:` URI substitution stays
    keyed to the image element alone** — a bibliography is named in the frontmatter and
    never as an image destination, so generalizing it would add a branch nothing takes.
    **This file's own doc comments carry counts the twelfth row moves** — "the ten that
    name no image", "three of the eleven" twice, "the eleven outputs", "the eleven
    generated blocks", and a "twelfth `data-example=\"`" that becomes a thirteenth. They
    are prose in a file the phase is already editing, and the documentation tracks the
    code.
  - `web/Cargo.toml` — only if the module grew, and only to record it; see the gate.
- **Exit gate:** `cargo test --workspace`, and five things.
  1. **Every literal a twelfth row moves is named here rather than discovered.**
     `core/tests/page_examples_test.rs:EXPECTED` 11 → 12, which **four** assertions read:
     the example count, the `data-example="` count, and both the `<!--html:` and
     `<!--/html:` marker counts. **And one literal moves the other way** — the asset
     scan's `1` → `2`, which is not a count of examples at all. That is why "the counts
     move together" was a gate that passed for the wrong reason, and round 1 said so.
  2. **The new row compiles and its PDF carries the reference list.**
     `every_ok_example_compiles` reaches it once both files are in the slice, and that it
     compiles at all is the proof: `#cite` compiles only when the key resolves against a
     present bibliography, which is the argument Phase 1's own gate rests on.
  3. **The generated column is the generator's own output**, unchanged —
     `every_generated_block_is_the_parsers_own_html` covering the twelfth block — and the
     three guards still hold: the marker counts, the image named once and surviving
     nowhere after substitution, and no block carrying `<script`, `data-example="`,
     `data-asset="` or `<!--`.
  4. **The browser half names its recipe and its engine**, which round 1 found the draft
     had left as "in a browser". `web/pkg/` is gitignored and never committed, so the
     check is `wasm-pack build --target web --release` inside `web/`, served over
     `http://127.0.0.1` — a second person following the draft literally opens `file://`
     and sees nothing. `mpdf-006` Phase 2 put that recipe in its own gate for this reason.
     **Chromium alone is enough**, on that spec's own recorded argument for the asset
     channel: a byte array crossing an existing `wasm-bindgen` boundary is not something
     two engines can disagree about.
  5. **The module size is recorded against the baseline.** `mpdf-006` Phase 4 fixed
     **25,342,182 bytes** — `web/pkg/md2pdf_web_spike_bg.wasm` as its Phase 3 left it —
     and requires any growth at all to be recorded. This phase changes `web/src/lib.rs`,
     so it owes that measurement whether or not the number moves. **The delta is not all
     this phase's, and the record says so rather than implying it**: that baseline
     predates Phase 2, which made `core/src/bibliography.rs` and `hayagriva`'s reader
     reachable from the wasm build. The requirement is to *record*, never to stay under,
     so this cannot fail for the wrong reason — but an unattributed number would read as
     one row costing what two phases did.
- **Close-out:** `rules/web-demo.md` — the second asset and the one channel that now
  carries two, the twelfth example, and the two exports' shapes. Its `max_lines: 205` has
  ten lines of headroom against 195 used, so unlike Phases 2 and 3 the cap may not need
  to move; that is checked rather than assumed. `README.md` needs nothing — it documents
  the dialect and the CLI, and the demo's row count is not a fact it carries. One push.
