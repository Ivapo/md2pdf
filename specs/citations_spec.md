---
id: mpdf-007
title: citations-and-bibliography
note: >
  A document cites its sources and prints their reference list: the frontmatter names a
  bibliography file, the caller supplies it as bytes beside the images, `[@key]` becomes a
  citation, and Typst renders both the marks and the list.
status: accepted
last_updated: 2026-09-01

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
  - name: "Phase 5 — a citation reads in the sentence, and the marks may be author-date"
    reviewed: 2026-09-01
    shipped: 2026-09-01
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

  **WIDENED 2026-09-01, on drafting Phase 5: the promise holds for the PDF and not for
  the Typst.** A `citations` frontmatter key crosses to the look as a ninth argument that
  `core/src/emit.rs:header` writes on every call at its resolved default, exactly as
  `headings` did when `mpdf-005` Phase 8 re-blessed twenty-eight goldens and added one in a
  single commit —
  so every document's generated Typst gains `citations: "numeric"` on its `template.with`
  line. What stays byte for byte is the *page*: the look answers `numeric` with the style
  Typst already defaults to, and Phase 5's gate hashes the PDFs either side of the phase
  to hold it. The sentence above was written before the dialect had a key that reaches
  the look through this spec, and "the Typst it emits today" is the half it over-promised.
- **Pandoc's prefix form is out of reach, and stated rather than discovered.** `[see @k]`
  and a bracketed email `[a@b.com]` stay literal text: §2's callback fires on a reference
  beginning `@` or `-@` and nothing else. This dialect has no prefix form, so they print
  for the reason any unclaimed markdown prints. *(Since Phase 5 the callback fires on `+@`
  as well; the prefix form is as far out of reach as it was.)*

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

**WIDENED 2026-09-01, on drafting Phase 5: `[@key]` gains two bracketed siblings and the
bare form stays out.** `[+@key]` is the textual citation and `[-@key]` the year alone, both
claimed by the same callback on the argument the next section makes for `-@`. Nothing
above moves: the brackets are still what keeps `a@b.com` text, and the bare form is still
refused by the measurement recorded here. The decision section appended at the end of §2
carries the argument and the measurements.

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

**WIDENED 2026-09-01, on drafting Phase 5: the boundary is `@`, `-@` or `+@` at the start,
and nothing else.** The argument above is unchanged — a sigil glued to the at-sign inside the
brackets is a citation and an at-sign anywhere else is text — and `[+ @k]`, a plus not glued
to its at-sign, stays the text it is. §2's appended decision carries the spelling.

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

### The mark may read in the sentence, the scheme is the author's and the style is the look's, measured (decision, recorded)

**APPENDED 2026-09-01**, on OQ-2 and OQ-3 being taken together, and per §6.1 step 2.

**A citation may name its source inside the sentence, natbib's `\citet`, and the dialect
spells it `[+@key]`.** Today every citation is parenthetical — `[@key]` is `#cite(label("key"))`
and nothing else — so *"As (Postigo, 2026) showed"* is the only sentence an author can
write. Typst's `cite` takes a `form`, and `form: "prose"` is the textual mark; it also takes
`form: "year"`, which is what Pandoc's `[-@key]` has always meant. Both are one argument on
the call the emitter already writes, so the mechanism costs nothing and the whole question
is the spelling.

**The bare `@key` that Pandoc and Quarto use stays out, and `+` is the sigil, mirroring
the `-` the callback already claims.** The section above measured why an unbracketed `@`
cannot be a citation — `a@b.com` — and the parse gives the emitter no event for a bare
`@key` at all: reading it would mean a second, hand-written scanner over text runs beside
`pulldown-cmark`, which is the thing the callback design refused. Inside the brackets the
callback already fires on `-@`, and `core/src/emit.rs:citation_reference`'s doc comment
records that `-@` "is not decoration". A third prefix is the same class: `is_citation` gains `+@`, and
`[-@key]` stops being refused by name and lands as the year. Measured 2026-09-01 through
the shipped binary, `[+@k]` reaches the page today as `\[\+\@k\]` — literal text, so no
shipped document changes meaning — and a census of `tests/`, `samples/`, `web/`,
`README.md`, `core/tests/` and `cli/tests/` finds `[+@` nowhere, and `[-@` only in the
sentences that say it is refused and the two test tables that assert it.

**The multiple-key form lands too, and it is Typst's own merge and not a mechanism.**
`[@a; @b]` was refused by name because Phase 1 would not guess. Measured 2026-09-01 through
the crate's own `TypstWorld`: two `#cite` calls with nothing or only whitespace between
them are merged into one parenthesis — `(Claude and Knuth, 2025; Postigo, 2026)` — and a
word or a comma between them keeps them apart. So `[@a; @b]` emits the two calls adjacent,
and `[@a] [@b]` already renders the same page. **A form on a group is refused**, `[+@a; @b]`
and `[-@a; @b]` alike: natbib's `\citet{a,b}` exists, but Typst's merged prose group reads
`Claude and Knuth (2025), Postigo (2026)` with a comma, which is not a sentence, and the
rule "a form names one source" is the one an author can hold. The locator, `[@k, p. 33]`,
stays refused: Typst's `supplement` renders it — measured, `(Postigo, 2026, p. 33)` — but
nothing asked for it, and OQ-3 resolves into this phase with that half left open.

**The style is a frontmatter key that names the scheme, and the look names the CSL.**
`citations: author-date`, against a default `numeric` that is today's output. That is the
dialect's own precedent three keys deep — `equations: numbered`, `figures: sectioned`,
`headings: 2` — and `core/src/frontmatter.rs:Equations`' doc comment is the argument in its
own words: *the author decides whether; the look decides how*. The alternative, a
`citation-style:` key carrying Typst's CSL name, was weighed and loses: it puts Typst's
style catalogue into the dialect, makes every look promise every style, and §1.2 already
says no style beyond what Typst ships is in the dialect. The key crosses to the look as a
ninth argument on `core/src/emit.rs:header`'s call, and the look answers with one `set
bibliography(style: …)` in the ternary-in-argument shape `rules/pipeline.md` records for
`equations`. **Measured 2026-09-01** in a look function of that shape under the emitter's
own `#bibliography("refs.yml", title: none)` call: the marks and the list take the style,
and `#cite`'s own `style` defaults to the bibliography's, so no per-call argument is needed.
Under `numeric` the look sets `"ieee"` by name, which is Typst's default, so the page is
byte for byte the page it is today.

**Which author-date style is the look's, and it was chosen on the merged group.** Measured
2026-09-01 through the crate's Typst 0.15.1, one to four authors, parenthetical and prose:

| style | one, two, three authors | two sources merged |
|---|---|---|
| `harvard-cite-them-right` | (Postigo, 2026) · (Postigo and Claude, 2026) · (Postigo, Claude and Knuth, 2026); "et al." from four | (Claude and Knuth, 2025; Postigo, 2026) |
| `elsevier-harvard` | same, "et al." from three | (Claude and Knuth, 2025, Postigo, 2026) |
| `apa` | `&` for "and" | semicolons |
| `chicago-author-date` | no comma before the year; "et al." from four | semicolons |

Every style renders the prose form as *Postigo (2026)* and the year form as *2026*. No
bundled style matches natbib's `plainnat` on both counts, so the choice is which departure
costs less: Elsevier Harvard separates merged sources with the same comma it puts between
author and year, so a two-source group is one run of names and years the eye cannot split,
on every paper that cites two things at once; Harvard cite-them-right spells a third author
out and shortens from four. **Both bundled looks answer `author-date` with
`harvard-cite-them-right`**, on the precedent that both write `(1)` for `equations`; it is
one string in each look, and a second scheme name would be one line in `frontmatter.rs`.

**Under `numeric` the new forms still render, so they need no refusal.** Measured with
`ieee`: `form: "prose"` reads *I. Postigo [1]* and `form: "year"` reads *2026* — which is
what natbib does in its numeric mode too. The form is the citation's and the style is the
document's, and the two do not have to agree for the page to make sense.

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
  ~~Typst's default is IEEE-numeric. `equations: numbered` is precedent for a frontmatter
  key that selects a rendering, and `template:` is precedent for the look owning one.~~
  **RESOLVED 2026-09-01, on drafting Phase 5: both, split the way every numbering key is
  split.** The frontmatter names the *scheme* — `citations: numeric | author-date`, the
  first the default — and the look names the CSL style that answers it, which both bundled
  looks take as `harvard-cite-them-right`. §2's appended decision carries the measurement
  the style was chosen on and the alternative it refused.
- **OQ-3 — do the locator and suppressed-author forms land at all?** *(needs-input)*
  ~~`[@key, p. 33]` and `[-@key]` are Pandoc's and Typst has `#cite(<k>, supplement: …)`
  and `form: "year"`. Phase 1 refuses them by name rather than guessing — **both are
  within the callback's reach, measured in round 2**, which is what makes "refuses by
  name" something the parse can actually deliver. Whether a later phase adds them is not
  this spec's to assume.~~ **RESOLVED 2026-09-01 in one half, on drafting Phase 5: the
  suppressed-author form lands as the year, with Pandoc's meaning, and the multiple-key
  form lands beside it as Typst's own merge; the locator stays refused by name**, because
  nothing has asked for it — `supplement` renders it, measured, so it is one argument away
  when something does. The phase was asked for by a reader arriving from natbib, which is
  the input this question waited on.
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

### Phase 5 — a citation reads in the sentence, and the marks may be author-date

*Produces the observable: **yes** — a PDF whose sentence reads "As Postigo (2026) showed,
the method holds (Claude and Knuth, 2025)" and whose reference list is alphabetical, where
today the same document reads "[1]" and "[2]" and the textual spelling prints as literal
text.*

**Drafted 2026-09-01**, on §2's appended decision and per §6.1 step 2. The ordered test
lands on step 2, and the steps above it are worked rather than skipped.

- **Step 0 — a decision, not only code?** Yes, three: a frontmatter key, two spellings, and
  a style. Each is recorded in §2.
- **Step 1 — does it remove or contradict shipped work?** **It inverts two shipped
  refusals, asserted at three sites, and all were placeholders by their own words.**
  `core/tests/golden_test.rs:each_refused_citation_names_the_authors_line` and
  `cli/tests/cli_test.rs:each_refused_citation_exits_non_zero_and_names_its_payload` both
  assert that `[@a; @b]` and `[-@k]` are refused by name — the second through the built
  binary, on stderr — and `rules/pipeline.md`'s citation section says `cite_key` "refuses
  three payloads". **The CLI table is named here because round 1's three lenses each found
  it missing**: "`cli/src` untouched" is true and `cli/tests` is not `cli/src`, which is the
  shape Phase 4's round 1 blocked on one directory over. Phase 1's own scope wrote those
  refusals "rather than guessing", and OQ-3 said in so many words that "whether a later
  phase adds them is not this spec's to assume" — a refusal-by-name is a reservation, and
  this is the phase it was reserved for. Nothing built is un-built: `[@key]` names exactly
  what it named, the bare form stays out, and the locator stays refused. **It also widens
  one §1.2 promise**, and step 1's rule is a dated note in place rather than silence: "emits
  exactly the Typst it emits today" was written before this spec had a key that reaches the
  look, and every such key moves the `template.with` line of every golden — twenty-eight
  re-blessed and one added in `mpdf-005` Phase 8's one commit. The note narrows the promise
  to the page, and gate (2) is what holds it there.
- **Step 2 — the subject.** Citations, which this spec owns, and a frontmatter key that
  serves them, which rides the spec whose subject it serves — `figures` and `headings` both
  rode `mpdf-005`. `mpdf-001` owns no part of it.
- **Step 3 is not reached.**

- **Scope: one key, one argument, two spellings, one merge, one style — and the showcase.**

  **`core/src/frontmatter.rs` gains its eleventh key, on `Equations`' exact shape.** A
  `Citations` enum with `Numeric` first, as the default, and `AuthorDate`; `name()`,
  `from_name` and `names()` as its three siblings have them; the `"citations"` arm of
  `core/src/frontmatter.rs:parse` refusing a value outside the set with
  `key 'citations' takes numeric or author-date, not '…'`, the sentence
  `core/src/frontmatter.rs:Equations` already shapes. The module doc's "ten keys" and "two
  of the ten" move to eleven.

  **`core/src/emit.rs:header` writes a ninth argument, always, at the resolved default.**
  `citations: "numeric"` after `headings`, on the rule `rules/pipeline.md` records — "header
  writes the *resolved* default on every call, so 'no key' and `figures: flat` emit
  identical Typst". Every golden's `template.with` line moves and nothing else in any of them
  does; the re-bless is gate (2). `header`'s doc, *"`equations`, `figures` and `headings`
  cross as Typst strings"*, gains the fourth.

  **`core/src/emit.rs:is_citation` claims `+@`**, beside `@` and `-@`, and stays the one
  predicate the callback and `core/src/emit.rs:wrote_citation` both read;
  `core/src/emit.rs:citation_reference`'s doc — *"begins `@` or `-@`, and no other"*, and
  `-@` as the form "the dialect refuses by name" — moves with it.
  **`core/src/emit.rs:cite_key` becomes a parse of the payload into a form and its keys, in
  this order so that no shape is guessed at**: a `,` anywhere in the payload is the locator
  refusal it is today, first, so `[+@a; @b, p. 33]` is the locator; then a leading `+` is the
  prose form and a leading `-` the year form, neither the normal form; then the rest splits
  on `;`, each piece trimmed — `[@a ; @b]` reaches the callback with its spaces, measured —
  and each piece must be non-empty and begin `@`, so `[@a; b]`, `[@a;]` and `[@a;;@b]` are
  refused at the line as pieces that are not keys, in words the implementer chooses; and a
  form over more than one key is refused at its line — *"puts a form on several sources, and
  a form names one"*, or words the implementer prefers, asserted by needle. Every key joins
  `core/src/emit.rs:Names::cited` with the group's line, so a key the bibliography does not
  hold is still named at the line its bracket sits on, and `check_citations` needs no
  change: its message names `@key` and that is still the key's name.

  **The write is the one call it is today, with a `form:` where the form is not normal, and
  the calls of a group adjacent with nothing between.** `[+@k]` is
  `#cite(label("k"), form: "prose")`, `[-@k]` is `#cite(label("k"), form: "year")`, and
  `[@a; @b]` is `#cite(label("a"))#cite(label("b"))` — nothing between, because that is the
  form the measurement in §2 merged and a space would be a byte the author did not write.
  `crate::md_to_html` follows `is_citation` and renders `<a href="+@k">+@k</a>`, which is
  as honest as the `@k` it renders today; the page's blessed column does not move because
  the page's row does not.

  **Both looks take `citations: "numeric"` and answer it with one `set`, and the contract
  test holds both to it.** In `core/assets/template.typ:template` and
  `core/assets/press-release.typ:template`, beside the `show bibliography:` rule each
  already carries: `set bibliography(style: if citations == "author-date"
  { "harvard-cite-them-right" } else { "ieee" })` — the ternary-in-argument shape, because a
  `set` inside a scoped `if` dies with the block, which `rules/pipeline.md` records for
  `equations`. `"ieee"` by name rather than `auto`, so the numeric page is provably the
  default's page rather than assumed to be. The "References" label and its rule are
  untouched. Each look's header comment enumerating the eight it takes — *"title, author,
  affiliation, columns, date, equations, figures and headings"* and *"the same eight the
  article look takes"* — gains the ninth. **The press-release look is held by needle, not by
  a fixture**: `core/tests/golden_test.rs:every_bundled_template_meets_the_call_contract`
  reads each look's source, and its own doc records that a call-contract parameter joins it
  as a *pair* of needles because "the parameter alone would be satisfied by a look that took
  the argument and ignored it". The pair here is the `citations` parameter and the string
  `harvard-cite-them-right`, which only a look that maps the scheme can carry; round 1 found
  that without it a press-release look writing `"ieee"` unconditionally passed every case.

  **The showcase takes the key, and the phase names what that costs, by the sentence.**
  `samples/showcase/` is "one document that uses every construct in the dialect … under all
  ten frontmatter keys" (`README.md`), so an eleventh key it does not carry makes that
  sentence false. `samples/showcase/showcase.md` gains `citations: author-date` in its
  block, **and its `## The frontmatter, all ten keys` section gains a paragraph for the
  key** — the section describes every key, and one it never mentions breaks its own
  promise — with *"all ten are optional"* and *"You say *whether* in all four cases"*
  moving to eleven and five. `samples/showcase/sections/notes-and-sources.md` gains one
  textual citation, **merges two of its adjacent citations into one `[@a; @b]` group** —
  round 1 found gate (5) asking for a merged parenthesis the scope never put there — and
  reworks "the mark and the numbering are the typesetter's", which reads wrong under
  author-date. `samples/showcase/README.md`'s *"all ten frontmatter keys"* and
  `samples/article.md`'s *"four of the ten keys"* with its *"all ten are optional"*, *"The six
  keys this file leaves out"*, *"names all ten at once"*, *"A key outside those ten"* and *"an
  equations or figures name outside its own two"* move with it, on the precedent `mpdf-001`'s tenth-key close-out
  set for exactly this operation. Three measured numbers move and each is named so the
  implementer owes them knowingly: the master's three headings sit at lines 31, 46 and 85,
  which `app/src/preview.rs:the_anchors_are_the_headings_of_whichever_file_the_pane_holds`
  pins by literal beside a doc comment reading *"below its twelve-line frontmatter"* — the
  block grows by one line and the frontmatter section by a paragraph, **so the literal is
  re-read off the edited file with `grep -n '^#' samples/showcase/showcase.md` rather than
  derived**; `rules/desktop-panes.md` records **1140 text items across six pages**,
  re-measured through the vendored `app/dist/pdfjs/` by `mpdf-005` Phase 11, and its own
  sentence says "it is a phase that edits the showcase that moves it"; and
  `tests/gates/mpdf-009-phase5.js` asserts the six pages by hand. The alternative — leaving
  the showcase numeric and rewording the README to "ten of its eleven keys" — was weighed
  and loses, because the showcase's one promise is the whole surface.

  **What is deliberately not touched.** `core/src/bibliography.rs` and the key set it reads;
  `core/src/emit.rs:check_citations`; the callback's refusals of `[see @k]`, `[a@b.com]` and
  the bare `@thing`; `cli/src`; `app/src` beyond the one literal and its comment above;
  `web/`. No new crate, no CLI flag.

- **Exit gate:** six cases.

  (1) **A new fixture matches a new golden, and the golden pins the three forms and the
  merge.** `tests/fixtures/author_date.md` carries `citations: author-date` and names
  `tests/fixtures/author_date.yml` — named for the fixture it serves, because
  `tests/fixtures/authors.md` already exists and is about frontmatter authors — a new file
  holding four records of one, two, three and four authors, new rather than added to
  `refs.yml` so `citations.md`'s golden and the both-formats test keep their key set. The
  fixture cites every shape: `[@one]`, `[+@two]`, `[-@one]`, `[@three; @one]`, `[+@four]` and
  the collapsed `[@two][]`, each in a sentence written to read under author-date.
  `tests/golden/author_date.typ` shows `citations: "author-date"` on its `template.with`
  line, `#cite(label("two"), form: "prose")`, `#cite(label("one"), form: "year")`, and
  `#cite(label("three"))#cite(label("one"))` with no byte between; its
  `#bibliography("author_date.yml", title: none)` call is the shape Phase 1's is. `md_to_pdf`
  returns bytes starting `%PDF`. **The page is measured by hand and recorded in the test's
  doc comment**, on the precedent `rules/pipeline.md` records for `figures` and `headings`
  — the suite reads no PDF text — with `pdftotext` showing *Claude and Knuth (2025)*, *2026*,
  and one merged parenthesis with a semicolon. **The wrong build this discriminates
  against**: one that passes the form as `#cite`'s `style` argument compiles and renders
  the wrong mark, and fails the golden's needle.

  (2) **Every shipped golden moves in its `template.with` line and nowhere else, both looks
  are held to the scheme, and the PDFs under the default do not move at all.** Thirty-four
  goldens gain `citations: "numeric"`, and the instrument for "nowhere else" is
  `git diff --numstat -- tests/golden` reading `1 1` on thirty-four rows, the shape
  `mpdf-005` Phase 8's re-bless shows. `grep -c '_matches_its_golden_file'` reads
  thirty-two today and thirty-three after, and `ls tests/golden | wc -l` thirty-four then
  thirty-five, the instruments named rather than the arithmetic; the `template.with`
  literal in `core/tests/golden_test.rs:absent_frontmatter_gets_every_default` gains the
  argument. `every_bundled_template_meets_the_call_contract` gains the pair of needles per
  look named in scope, which is what fails a press-release look that took the argument and
  set `"ieee"` regardless. **The PDF half is hand-measured and recorded**: `md_to_pdf` of
  `citations.md` and `citations_press_release.md`, one per look, hashed either side of the
  phase, byte-identical — the method `rules/pipeline.md` records for `figures` and
  `headings`, which hashes *unchanged sources* across two trees. **The showcase is not in
  that set, and round 2 is why**: this phase moves its content — the textual mark, the
  merged group, the reworded sentence — so its page differs on a right build and a wrong
  one alike, and a hash of it discriminates nothing. The two fixtures are unchanged sources
  and carry the promise for both looks. That is the §1.2 promise in the form the note narrows it to,
  and a look that wrote `auto` instead of `"ieee"` would still pass it, which is why
  `"ieee"` is by name in scope rather than left to a gate.

  (3) **The refusals move as the scope says, at both sites.** In
  `core/tests/golden_test.rs:each_refused_citation_names_the_authors_line` the `[@a; @b]` and
  `[-@k]` rows leave, the locator row stays, and four rows join: `[+@a; @b]`, `[-@a; @b]`,
  `[+@k, p. 33]` and `[@a; b]`, each at its line, **and the `[+@k]`-without-bibliography row
  joins the same table** — refused as `'@k' is cited and the frontmatter names no
  bibliography` — so the table's one `Err(Error::` site stays one and the **forty-eight**
  that `core/tests/messages_test.rs`' module doc records *over* `golden_test.rs` does not
  move, checked with `grep -c "Err(Error::" core/tests/golden_test.rs`. In
  `cli/tests/cli_test.rs:each_refused_citation_exits_non_zero_and_names_its_payload` the
  `[@a; @b]` row becomes `[+@a; @b]` with the new needle, the `[-@k]` row leaves, and its doc
  comment's *"the three payloads Pandoc spells"* moves. `citations: apa` is refused in
  `core/src/frontmatter.rs`' own tests, a sibling of
  `an_equations_name_outside_the_set_lists_the_names_it_accepts` naming the key and both
  accepted names.

  (4) **The forms are emitted, not read off the golden**, in one new test whose documents
  name `bibliography: refs.yml` — `check_citations` refuses a cited key without it, whatever
  the `citations` key says — and carry **no** `citations` key: `[+@k]`, `[-@k]`, `[@a; @b]`,
  `[@a;@b]` and `[@a ; @b]`, the last two emitting what the first of the three does, and
  `[@k][]` still emitting `#cite(label("k"))`, which is what says the form is the citation's
  and the style the document's. `the_callback_claims_a_citation_and_nothing_else` keeps
  every row and gains one: `[+ @k]`, a plus not glued to its at-sign, stays text — measured
  today as `\[\+ \@k\]`.

  (5) **The showcase compiles to six pages, its marks read author-date, and the numbers it
  moves are re-taken.** `pdftotext` shows the merged parenthesis the new group makes, with a
  semicolon, and *Table 3.1* still, which is the regression half; the app test's literal is
  re-read off the file and moved, with its "twelve-line" comment; the text-item count is
  re-measured through the vendored `pdf.js` and written into `rules/desktop-panes.md` with
  the date; the six-page gate is run by hand.

  (6) **`cargo test --workspace` passes and `spec-lint` exits zero with no error.** The one
  warning is inherited and named so an implementer does not chase it:
  `rules/desktop-geometry.md`'s `RULE_SOURCES_WITHOUT_GENERATED`.

- **Close-out.**

  **`rules/pipeline.md` owes a cap raise, and the number is measured.** It stands at
  **1209 of `max_lines: 1215`**, six lines of headroom against an edit of roughly
  twenty-five: the citation section's `[@key]` paragraph gains its two siblings and the
  merge; "`cite_key` refuses three payloads" becomes one payload and two new refusals; the
  frontmatter section's "Ten keys" becomes eleven, its list gains `citations`, "Nine reach
  the look … all eight arguments" becomes ten and nine, and *"`equations`, `figures` and
  `headings` are names against closed sets"* gains the fourth; the templates section's
  *"`figures` and `headings` before the trailing `doc`"*, *"The export list is four and the
  call is eight"* and *"the call contract has not moved since `mpdf-001` Phase 11"* all move,
  *"Only the third takes `equations`' ternary-in-argument shape"* gains the bibliography
  `set` as a fourth taker, and the needle paragraph — *"two needles for `equations`"* — gains
  `citations`' pair. **The constructs count does not move**: *"Twenty-six things are
  supported"* counts citations once, a form of a construct is not a construct, and a
  frontmatter key was never counted. The `covers:` line gains "the scheme the marks follow"
  beside the citation clause. `max_lines` moves to **1240**, with that reason.

  **`README.md`, six sites by their words.** *"Name a bibliography file in the frontmatter
  and cite a key with `[@key]`"* gains the two siblings and a rendered sentence; *"The mark
  and the numbering are Typst's"* gains the key; *"Eight things are errors"* stays eight
  and *"Four are about the citation"* stays four, two of the three Pandoc forms it lists replaced by
  the form-on-a-group and a piece that is not a key — the missing bibliography, the
  locator, `[+@a; @b]` and `[@a; b]` — which is the count the `rules/pipeline.md` paragraph
  keeps too, and round 2 caught the two disagreeing; the frontmatter block
  *"It takes ten keys, all optional"* gains `citations: author-date` and a paragraph beside
  `equations:`' on the *whether/how* split; *"A key outside the ten"* becomes eleven and its
  *"`template`, `equations`, `figures` or `headings` value outside its set"* gains
  `citations`; and *"under all ten frontmatter keys"* in the showcase paragraph becomes
  eleven, true once the showcase takes the key.

  **The samples and the doc comments that state a count**, each named because a count
  nothing mechanical reads is the one that rots: `samples/showcase/showcase.md`,
  `samples/showcase/README.md` and `samples/article.md` as scope says;
  `core/tests/golden_test.rs`'s *"`header` names all eight arguments"* and *"the call
  contract stays at eight"* in one doc block, *"the suite reads all thirty-four of them"* and
  *"The other thirty-two goldens"*; `cli/tests/cli_test.rs`'s
  *"three payloads"*; `app/src/preview.rs`'s *"twelve-line frontmatter"*; both looks' header
  comments; and `core/src/frontmatter.rs`'s module doc.

  **`rules/desktop-panes.md`**: the re-measured count, with the date, as gate (5) says.

  **`web/index.html`: a logged gap, on Phase 4's own precedent.** The citation row's prose,
  *"`[@key]` cites it"*, stays true and does not show the new forms; the page's
  *"Nine frontmatter keys decide the look"* heading is one behind after this phase; and the
  lede's *"Twenty-three constructs are supported"* stays three behind, since the count does
  not move. `rules/web-demo.md`'s *"the nine frontmatter keys that decide the look"* and
  *"Twenty-six constructs are supported"* describe the page and the count, and both stay
  true. A logged gap that keeps growing is a phase of `mpdf-006` waiting to be drafted, as
  Phase 11 of `mpdf-005` said.

  **`CLAUDE.md` and the status artifact: none needed.** The observable sentence is
  untouched and this repository keeps no status artifact.

  `specs/INDEX.md` and `rules/INDEX.md` regenerated, never hand-edited — this spec's rollup
  goes `done` → `partial` as the phase is appended and back to `done` when it lands. One
  push.
