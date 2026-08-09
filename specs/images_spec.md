---
id: mpdf-002
title: images
note: >
  Markdown images become PDF images: the emitter maps ![alt](path), callers
  supply the named files as bytes, and the CLI reads them from disk.
status: accepted
last_updated: 2026-08-09

phases:
  - name: "Phase 1 — the asset channel and the image construct"
    reviewed: 2026-08-09
    shipped: 2026-08-09
    cut: null
    by: null
  - name: "Phase 2 — the CLI reads the files"
    reviewed: 2026-08-09
    shipped: 2026-08-09
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001]
reference: >
  Pandoc's implicit_figures extension is the inspiration for drawing a line
  between a standalone image and an inline one. Its caption behaviour — the alt
  text becomes a numbered figure caption — is out of scope: markdown carries no
  caption syntax of its own, and a later spec may take figures up properly.
---

# images

## 1. Goal

Convert a markdown document that names image files into a PDF that shows them.
The observable is unchanged from `mpdf-001` — the typeset PDF that Typst
compiles from the user's markdown — but the input widens: a document plus the
image files it names, rather than one file alone.

```markdown
The pipeline in one picture:

![The three steps, drawn as boxes](figures/pipeline.png)

A small icon ![a check mark](check.svg) sits inside this sentence.
```

```console
$ md2pdf paper.md -o paper.pdf    # reads figures/pipeline.png beside paper.md
```

Today one image is a fatal error, so a real article — and real articles have
figures — still does not convert unmodified. `mpdf-001`'s Phase 5 recorded
images as "a later spec's subject, not a construct", because supporting them
means widening what the `World` holds, not just what the emitter maps. This is
that spec. It is a new document rather than a Phase 7 by the methodology's
§6.1: the subject is an asset channel crossing `core`'s API, the `World` and
the CLI, where every dialect phase of `mpdf-001` widened the emitter alone.

### 1.1 Non-goals

- **No fetching, ever.** A URL destination is an error, never a download,
  per `mpdf-001` §2's network decision. `data:` URIs are errors too.
- **No captions and no figure numbering.** Markdown has no caption syntax;
  alt text is accessibility metadata, not a caption. A later spec may adopt
  Typst's `figure` properly.
- **No sizing or transform syntax.** No width attributes, no rotation, no
  crops. The Typst defaults stand, and any future look is a `template.typ`
  rule per `mpdf-001` §2's styling decision.
- **No new frontmatter keys.**

## 2. Design

`core` stays OS-free. The caller supplies every image as named bytes, and the
API grows accordingly, all in `core/src/lib.rs`:

- `pub struct Asset { pub path: String, pub bytes: Vec<u8> }` — one named
  file, the name being the path exactly as the markdown wrote it.
- `md_to_pdf(md: &str, assets: &[Asset]) -> Result<Vec<u8>>` — the existing
  function gains the parameter. Its callers today are the CLI and the
  golden-file suite, so the change touches both crates' tests.
- `pub fn image_paths(md: &str) -> Result<Vec<ImageRef>>`, with
  `pub struct ImageRef { pub path: String, pub line: usize }` — the caller's
  shopping list, produced by the same parse the emitter runs. The CLI calls
  this first, reads the named files, and calls `md_to_pdf` with the results.
  The list preserves document order and may repeat a path; the caller
  deduplicates, and a validation error for a path referenced more than once
  names the first reference's line. The line is what lets every downstream
  error name where the image was asked for.

`core/src/lib.rs:TypstWorld` gains the assets beside its two sources.
`World::file` serves an asset's bytes by its virtual path, exactly as it
serves `template.typ` today; `source` and the package story are untouched, so
nothing new can reach the network on any target.

`core/src/emit.rs` maps the construct. A paragraph whose entire content is one
image emits `#image("path", alt: "…")`; an image with anything beside it in
its paragraph emits `#box(image("path", alt: "…"))`. Which form applies is
known one event late, so the emitter holds the finished call in a pending slot
at the image's end event: an image that opened its paragraph and is followed
directly by the paragraph's end is bare, and anything else boxes it. The path
and the alt travel through `core/src/emit.rs:typst_string`, never the markup
escape. An empty alt omits the argument. The alt text is captured by
flattening, which is CommonMark's own reading of alt and what pulldown-cmark's
HTML renderer itself implements: between the image's start and end events,
text and code contribute their text, a soft or hard break contributes a single
space, styling and link wrappers contribute nothing, and a construct outside
the dialect still errors. A nested image is flattened by the same rule under a
depth count — the capture counts image starts and ends, so the inner end does
not close it — and contributes only its own inner text. Its destination and
title are not content under the alt reading, so they are not validated and do
not join `image_paths`' list. Typst's `alt` — a plain string, carried into the
PDF's accessibility layer — is what the flattening feeds.

The format gate splits in two. At the emitter arm, the path's extension must
sit in Typst's own table — `png`, `jpg`, `jpeg`, `gif`, `webp`, `svg`,
`svgz`, `pdf` — and an extension outside it, a missing extension included, is
a construct error naming the line, so `--emit-typst` rejects it too. Before
compiling, `md_to_pdf` then validates the assets: every referenced path must
have bytes, and the bytes must be the format the extension names, by a
content check that mirrors Typst's own detection — magic bytes for the raster
formats and PDF, the gzip magic for `svgz`, a namespace sniff for `svg`. A
failure is an error naming the path and the line — two new variants beside
`Error::UnsupportedConstruct`, one for a missing asset, one for bytes that
are not the image their name claims.

### Why the caller supplies bytes (decision, recorded)

The `mpdf-001` §2 split — `core` takes strings and returns bytes, the caller
does the file I/O — is what lets one crate compile natively and to `wasm32`.
Images keep the split rather than break it: a browser build has no filesystem
to read, and a Tauri wrapper wants to feed dropped files from memory. The
`World` is already the mechanism — it serves `template.typ` as bytes from a
virtual filesystem today — so assets extend `core/src/lib.rs:TypstWorld`
rather than adding a second channel.

### Why validation runs before the compiler (decision, recorded)

`mpdf-001`'s Phase 5 established the guarantee: generated source always
compiles, and input that would break the compile is caught first, as an error
naming the construct and its line. A missing or mislabeled image file would
break it second-hand — Typst's error would name a span in `main.typ`, which
the user has never seen. So `md_to_pdf` checks every referenced asset up
front. The check leans on Typst's detection order, which is the `format`
argument first, then the extension, then the content. The emitter passes no
`format`, and an extension outside Typst's table is already a construct error
at the emitter arm, so for everything that reaches validation the extension
decides, and `core` requires the content to agree with it. Typst's own
fallback — content detection for an extension it does not know — is
deliberately not mirrored: a file whose name says nothing about its format is
a file the dialect refuses to guess about, and under the fallback `photo.bmp`
holding PNG bytes would compile while its name lies.
The recorded limit: a file corrupt past its magic still fails
at compile time with the compiler's own message. Catching that would mean
decoding every image twice, and the failure is in the user's file, not in
generated markup, so the guarantee this decision protects is not broken.

### Why paths are relative and stay inside the document's directory (decision, recorded)

Three shapes are errors naming the construct and its line: a destination with
a URI scheme, an absolute path, and a path with a `..` segment. A scheme is a
fetch request, and nothing fetches. An absolute path converts on one machine
only, which breaks the reproducibility that bundled fonts bought. A `..`
segment escapes the document's directory, so a document and its images stop
being a folder that travels as one thing — and inside the `World` it would
escape the virtual root, where relative paths resolve cleanly beside
`main.typ` by construction. One recorded consequence: a Windows drive path
like `C:\figure.png` reads as a scheme and errors; the relative form is the
portable one, and the error says so.

### Why a standalone image is bare and an inline one is boxed (decision, recorded)

Typst lays an image out as a block; its documented inline form is
`box(image(..))`. Emitting the bare call mid-sentence would split the
paragraph around the image, which rewrites the user's prose — the §2
faithfulness failure. Boxing everything instead would work, but it would deny
a later figure treatment its hook: a bare `#image` in its own paragraph is
what a future `show` rule or `figure` wrapper can address, and a box is not.
So the emitter draws the line the source draws: a paragraph that holds
exactly one image and nothing else emits the bare call, and every other
occurrence is boxed. Both forms carry the same path and alt mechanics.

### Why the template gains no sizing rule (decision, recorded)

Verified in `typst-layout` 0.15.1's `layout_image` at drafting: when neither
dimension is forced, an image takes its natural size bounded by the available
space, aspect ratio preserved. An oversized figure therefore scales down to
its column on its own, and a small icon stays small. There is nothing for the
template to own yet; when a figure look arrives, it is a `template.typ` rule
per `mpdf-001` §2's styling decision, not emitter output.

## 3. Open questions

- **OQ-1** — ~~the bare-image bound is verified, but the boxed form is not:
  does an image inside `box(..)` mid-paragraph get bounded by the line width
  the way a block image is bounded by the column, or can it overflow the
  line? Answerable from code (`typst-layout`'s inline layout path) during
  review. Blocks Phase 1's gate case (1) only in its look claim — an
  overflowing inline image would still compile.~~ **RESOLVED (2026-08-09),
  in review round 1:** bounded, and it cannot overflow. The inline collector
  passes the paragraph's full region — the column width — to the box layout,
  whose pod keeps the base region for auto sizes, and `layout_image`'s
  neither-forced branch bounds the natural size by that region; a box too
  wide for the remaining line wraps to a line of its own, never past the
  column. No scope change; the gate's look claim stands.
- **OQ-2** — ~~what does pulldown-cmark emit for an image nested inside another
  image's alt text, and does the alt capture need a depth counter to flatten
  it the way CommonMark specifies? Answerable from code (pulldown-cmark
  0.13.4's event stream) during review. Blocks the alt-capture wording in
  Phase 1's scope.~~ **RESOLVED (2026-08-09), in review round 1:** a nested
  image arrives as a full start–end pair inside the outer image's content,
  so the capture keeps a depth count of image starts and ends. The
  flattening follows pulldown-cmark's own alt rendering: text and code
  contribute their text, a break contributes a single space, and wrappers —
  a nested image's destination and title with them — contribute nothing.
  They are not content under the alt reading, so they are not validated and
  do not join `image_paths`' list. Landed in §2 and Phase 1's scope.

## 4. Implementation phases

Strictly sequential. Phase 1 builds the channel and the construct inside
`core`; Phase 2 gives the CLI hands.

### Phase 1 — the asset channel and the image construct
*Produces the observable: yes, at the library level — tests compile a PDF
with an image through `md_to_pdf` directly. The CLI cannot reach it until
Phase 2: it passes no assets yet, so an image document at the CLI errors
naming the missing bytes and its line — an honest mid-state, and a named
one.*

- **Scope:** In `core/src/lib.rs`: `Asset`, `ImageRef`, `image_paths`, the
  `md_to_pdf` signature, the two new error variants, the pre-compile
  validation, and the `World` serving asset bytes, all per §2. In
  `core/src/emit.rs`: the image arms. At the image's start event, six
  shapes are errors naming the construct and the line, mirroring the link
  arm: a destination with a URI scheme, an absolute path, a path with a
  `..` segment, an extension outside Typst's table, and — as with links —
  an empty destination and a non-empty title. Otherwise the alt capture
  runs, with its depth count and its space-for-a-break rule per §2, and the
  end event holds the call in the pending slot until the next event settles
  bare against boxed, per §2's standalone rule. In `cli/src/main.rs`: pass no
  assets; the signature change is the only edit. The rejection migrates
  again, as `mpdf-001`'s Phase 6 did it: `describe` drops its image arms,
  and the five artifacts keyed to an image rejection resolve in two ways.
  `tests/fixtures/unsupported_image.md` becomes `unsupported_html.md` — a
  raw HTML block, which pulldown-cmark parses with no option at all, where
  strikethrough, footnotes and math would each need one — and the core and
  CLI tests on it become raw-HTML tests naming the same lines. The inline
  images in `line_numbers_survive_a_frontmatter_block` and
  `a_frontmatter_error_wins_over_a_later_construct_error` become `<div>`
  blocks at the same lines.
- **Exit gate:** Golden-file tests, three cases, plus the full existing
  suite; no shipped golden file changes, because `image` and `box` are
  standard-library names and the import line is untouched. (1) A fixture
  with a standalone image, an inline image mid-sentence, an image that
  opens its paragraph but is followed by text — boxed, and pinned because
  it is the case a decide-on-what-preceded implementation gets wrong — an
  alt text carrying a `"` and an emphasis marker, and a path carrying a `#`
  matches its golden file — the standalone call bare, both inline calls
  boxed, every path and alt a string literal — and compiles to a PDF with
  the `%PDF` magic bytes, the tests supplying a small checked-in PNG and
  SVG as assets. (2) Each error shape names its construct or path and its
  line: a URL destination, a `data:` destination, an absolute path, a `..`
  path, an extension outside the table, a path with no extension at all, a
  titled image, an empty destination, a missing asset, and bytes that are
  not the format their extension names. (3) A raw HTML block exits non-zero
  naming the construct and its line at both levels — rejection survives
  the widening, through the migrated tests.
- **Close-out:** Update `rules/pipeline.md` — the dialect and world
  sections, and the intro paragraph that states the API. The README keeps
  its images section for Phase 2, and
  the gap is named in the review record; its rejection example, which
  names an image today, must move to a construct that still errors, or the
  README lies the day this phase ships. One push.

### Phase 2 — the CLI reads the files
*Produces the observable: yes — `md2pdf paper.md` with a figure beside it
writes a PDF that shows the figure.*

- **Scope:** In `cli/src/main.rs`: call `image_paths`, resolve each path
  against the input file's parent directory, read the bytes, deduplicate,
  and pass the assets to `md_to_pdf`. A file that cannot be read is exit 1
  naming the path, the line, and the OS reason. `--emit-typst` reads no
  images: emitting needs paths only, and the flag must keep working on a
  document whose images are absent. `samples/` gains a small hand-written
  SVG figure, and `samples/article.md` gains an image section that uses
  it — which is what keeps the corpus check from passing vacuously,
  because no corpus file names an image today. The README gains its
  images section: the formats, the relative-path rule, the alt text, and
  the error shapes.
- **Exit gate:** CLI tests, three cases. (1) A document and its image
  copied into a scratch directory convert; the PDF starts with the `%PDF`
  magic bytes. (2) A document naming a file that does not exist is exit 1,
  and stderr carries the path, the line, and the reason. (3)
  `--emit-typst` on that same document is exit 0 and prints the source —
  pinning that emission never reads an image. The corpus check closes the
  phase: the README and the sample both convert without error, or the gap
  is named in the review record.
- **Close-out:** Update `rules/pipeline.md`'s CLI section, the README and
  the sample against the code. Amend the `CLAUDE.md` stanza's "Single
  file in, single PDF out" line to name the widened input — the document
  and the images it names. One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-002.md, append-only, one heading per round. See §7 of the
methodology.
-->
