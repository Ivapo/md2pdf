---
id: mpdf-002
title: images
note: >
  Markdown images become PDF images: the emitter maps ![alt](path), callers
  supply the named files as bytes, and the CLI reads them from disk. An http(s)
  URL is a name like any other: core still fetches nothing, and the caller
  supplies its bytes.
status: accepted
last_updated: 2026-09-21

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
  - name: "Phase 3 — a URL is a name the caller fills"
    reviewed: null
    shipped: null
    cut: null
    by: null
  - name: "Phase 4 — the CLI fetches, when asked"
    reviewed: null
    shipped: null
    cut: null
    by: null

extends: null
supersedes: null
superseded_by: null
related: [mpdf-001, mpdf-008]
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

### Why a URL is a name, and the caller fetches it (decision, recorded)

*Added 2026-09-21 with Phases 3 and 4.* §1.1 refused a URL because nothing
fetches. Half of that stands: `core` must not fetch. It reads no file and no
clock, which is what lets it compile to `wasm32` and keeps a compile a function
of its inputs alone. But the other half never followed from it. The asset
channel already separates naming a file from reading one: the emitter names
what it needs, `core/src/lib.rs:image_paths` hands the caller the list, and the
caller supplies the bytes. A URL is one more name on that list.

So `core` still fetches nothing and `core/src/lib.rs:TypstWorld` is untouched.
A caller fetches a URL the way the CLI reads a file. A caller that will not
fetch — an offline build, or a window that has not yet asked its user — supplies
nothing, and `core` refuses the image in its own words, naming the URL and the
line. **Whether and when to fetch is the caller's decision, because it differs
by caller.** The CLI runs on a file its user has just named, and even so it
fetches only under `--fetch` (OQ-3). The desktop app opens files it has never
seen, and a document that fetches on open tells a server who opened it and
when. One rule in `core` could not be right for both.

Only `http://` and `https://` are URLs here. `data:` stays refused: it is not
a fetch, but decoding it is a separate feature this decision does not take.

### Why a URL's format is read from its bytes (decision, recorded)

*Added 2026-09-21 with Phase 3.* A local file's extension decides its format,
and §2 declined content detection for files on purpose: a name that lies about
its format is the author's to fix. **A URL is not a file name.** Its path may
end in `.php`, in `?w=800` or in nothing at all, a CDN may serve WebP under a
`.png` path, and the author cannot rename what someone else serves.

So the generated source asks for `remote/` followed by a hash of the URL, a
name with **no extension**. That sends Typst to content detection:
`typst-library` 0.15.1's `Packed<ImageElem>::determine_format` takes the `format`
argument first, then the extension, and only then `ImageFormat::detect` on the
data, which recognises all eight formats in `IMAGE_EXTENSIONS`. Verified in
that source on 2026-09-21. `core/src/lib.rs:collect` runs the same detection
before the compile: the bytes must satisfy `core/src/lib.rs:bytes_match` for
some extension in the table. An HTML error page served where an image should be
is then refused naming the URL and the line, instead of by the compiler against
a `main.typ` the author has never seen.

**The missing extension is also what keeps the name from colliding with a
file.** `core/src/emit.rs:check_image` refuses every local destination with no
extension, so no image the author names can land on `remote/<hash>`. The name
is a hash and not a counter so that it depends on the URL alone, not on where
the URL is first met or in which walk: a footnote definition's images are
walked separately, by `core/src/emit.rs:collect_definitions`. Two URLs that
share a hash would share a name, so `collect` refuses as `Error::Internal` any
insert under a `FileId` that another name already holds, rather than silently
using the first set of bytes for both images.

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
- **OQ-3** — ~~does the CLI fetch by default, or only when asked? Design call;
  blocks Phase 4's scope. **Recommended: by default, with `--offline` to refuse.**
  The author wrote the URL and then ran the command, and pandoc behaves the
  same way. A build that must not touch the network is the case that names a
  flag. The alternative, an opt-in `--fetch`, keeps `mpdf-001` §2's "the
  running app stays fully offline" true without a note, but every user then
  pays a flag to get the image they wrote. Phase 4 is drafted on the
  recommendation.~~ **RESOLVED (2026-09-21), by the user: only when asked, with
  `--fetch`.** The recommendation lost to the guard. A run with no flag stays as
  offline as every run before it, so a CI build, and a document from someone
  else, reach the network only when a person decides they should. What the flag
  costs is one word typed by an author who wants the image, and the refusal names
  that word (Phase 4). The struck text overstated one thing: with the flag, the
  CLI does go online, so `mpdf-001` §2's "fully offline" still takes a short
  dated note naming the one exception. Landed in Phase 4's scope and gate.
- **OQ-4** — ~~the client and its limits. Design call, answerable from crate
  documentation during review; blocks Phase 4's scope. The questions are which
  blocking client to use (`ureq` is the candidate: no async runtime), the
  per-request timeout, the body-size cap, and where TLS roots come from.
  Bundled webpki roots would repeat the argument `mpdf-001` §2 made for
  bundled fonts: the same result on every machine. Drafted as 30 s, 20 MB,
  redirects at the client's default limit, and bundled roots.~~ **RESOLVED
  (2026-09-21), by the user: the drafted limits, which are 30 s per request,
  20 MB per body, redirects capped, and bundled roots.** The client is `ureq`
  3.4, and its defaults were read from `ureq` 3.4.0's own source on 2026-09-21:
  - The license is MIT OR Apache-2.0.
  - The default features are `rustls` and `gzip`. `rustls` pulls in `ring` and
    `rustls-webpki-roots`, and the default `RootCerts` is `WebPki`, so the roots
    are bundled with no OS store consulted.
  - `max_redirects` defaults to 10.
  - `http_status_as_error` defaults to true.
  - There is no cookie store unless the `cookies` feature is on.
  - **Its default body limit is 10 MB, not 20**, so the 20 MB cap must be set
    on the read explicitly.
  Landed in Phase 4's guards.

## 4. Implementation phases

Strictly sequential. Phase 1 builds the channel and the construct inside
`core`; Phase 2 gives the CLI hands. Phases 3 and 4, appended 2026-09-21,
repeat that split for a URL: Phase 3 makes it a name `core` accepts, and
Phase 4 gives the CLI the fetch.

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

### Phase 3 — a URL is a name the caller fills
*Produces the observable: **yes**, at the library level. A document naming
`https://…/plot.png`, with that image's bytes supplied under the URL, compiles
through `md_to_pdf` with the image on the page; today it is refused as
`image with a URL destination`. The CLI supplies nothing for a URL until
Phase 4, and after it only under `--fetch`, so a URL document still fails
there, but now with
`no image fetched for '…'`, which names what is missing instead of calling the
construct unsupported. Phase 1 shipped the same mid-state for local files.
It is also everything Letur needs, since the fetch belongs to the caller and
Letur's own spec decides when it fetches.*

**§6.1, worked:** step 0 — a decision: §1.1 says a URL destination is an
error, never a download. Step 1 — it removes no shipped work. Every document
that compiles today compiles byte-identically, and the only change is a refusal
becoming legal. What it contradicts is prose (§1.1's non-goal and two sentences
of §2), which gets dated `CORRECTED` notes at close-out rather than a new spec.
Step 2 — an image destination is this spec's subject. So: an appended phase.
Letur cannot do this alone: its own §1.1 refuses exactly what `core` refuses.

- **Scope:**
  - **Which destinations.** `http://` and `https://`, with the scheme matched
    ASCII-case-insensitively, and nothing else; a new `emit::is_url` answers
    the question. `data:`, `file:`, every
    other scheme, and the drive path `C:\figure.png` stay refused by
    `core/src/emit.rs:written_shape`, exactly as today. The image rendering of
    `core/src/emit.rs:PathShape`'s `Scheme` changes to **`a URL scheme other
    than http or https`**, because "a URL destination" would read as a
    contradiction once one is accepted. `PathShape::key` keeps `a URL`: the
    `bibliography` key and the include marker still refuse every URL, and
    neither changes.
  - **The arm.** In `core/src/emit.rs:step`'s `Tag::Image` arm, a URL still
    passes the empty-destination and title checks. After that **it never
    reaches `core/src/sections.rs:Sources::resolve`, on either branch.**
    `resolve` normalises through `VirtualPath`, which drops a non-leading empty
    segment. The master's own `https://example.com/x.png` would arrive as
    `https:/example.com/x.png`, and a section's as
    `sections/https:/example.com/x.png`. A URL is an address, not a path
    relative to anything, and its identity is the string the author wrote. It
    reaches neither `core/src/emit.rs:landed_path` nor the extension table
    either, per §2's decision on a URL's format.
  - **The name the source asks for.** A new `emit::remote_name(url)` returns
    `remote/` followed by the sixteen lowercase hex digits of a 64-bit FNV-1a
    hash of the URL, written by hand with no new dependency.
    `core/src/emit.rs:image_call` writes that name, and the `ImageRef`'s
    `path` carries the URL. §2 records why the name has no extension and why
    it is a hash.
  - **The shopping list.** `core/src/lib.rs:ImageRef` gains
    `pub fn is_url(&self) -> bool`, which reads `emit::is_url`. It is a method
    rather than a field, so no caller that builds or destructures an
    `ImageRef` breaks. `image_paths`' doc comment states the contract: a URL
    comes back exactly as written, and a caller that fetches nothing supplies
    nothing for it.
  - **The check.** `core/src/lib.rs:collect` looks a URL up in `supplied` by
    the URL itself.
    - No bytes is a new `Error::UnfetchedImage { url, location }`, reading
      **`no image fetched for '{url}' {location}`**. It is a sibling of
      `MissingImage` on the argument `MissingBibliography` was added under:
      the words are the point, and "no image file supplied" names a file
      that does not exist.
    - Bytes that match no format in the table reuse `Error::ImageFormat` with
      `core/src/lib.rs:format_name`'s existing `"image"` fallback, reading
      `image file '{url}' {location} does not hold image data`. The usual case
      is an HTML error page served with a 200, and a new variant would buy one
      word.
    - Accepted bytes go into the map under `file_id(&remote_name(url))`, and
      the collision refusal §2 records sits at that insert.

    The earliest-line rule covers both new refusals unchanged.
  - **The CLI, for this phase only.** `cli/src/main.rs:read_assets` skips
    every entry whose `is_url()` holds, so `core`'s own refusal is what names
    it. Without the skip it would join the URL onto the directory and fail
    with an OS error about a file called `https:/…`.
  - **Tests that move.** `core/tests/golden_test.rs` has three rows asserting
    `image with a URL destination`. The `https://` row becomes a compile; the
    `data:` and `C:/figure.png` rows keep their input and take the new
    wording. In `the_prefix_launders_no_path_the_dialect_refuses`, a `data:`
    row replaces the `https://` row. That test proves the prefix launders
    nothing, and a URL no longer meets the prefix at all, so gate clause 3
    takes over what the old row checked.

- **Exit gate:** In the workspace suite, with no network:
  1. A document naming `https://example.com/figures/plot.png` once standalone
     and once inline, with `tests/fixtures/dot.png`'s bytes supplied under
     that URL: `md_to_pdf` returns `%PDF` bytes, and `md_to_typst` writes the
     same `remote/` name, followed by sixteen hex digits, in both calls. A
     second URL gets a different name.
  2. `image_paths` returns the URL twice, byte-identical to what was written,
     with `is_url()` true. A local image in the same document has `is_url()`
     false.
  3. **The prefix does not touch a URL.** A section `sections/one.md` naming
     `https://example.com/a/../b//c.png?w=800` yields an `ImageRef` whose path
     is exactly that string and whose location names the section.
     `the_prefix_launders_no_path_the_dialect_refuses` passes with its `data:`
     row.
  4. With nothing supplied for the URL, the error is
     `no image fetched for 'https://example.com/figures/plot.png' at line N`;
     written in a section, it ends `in sections/one.md at line N`.
  5. **The bytes decide, not the ending.** `tests/fixtures/mark.svg` supplied
     under a URL ending `.png` compiles. A PDF the test compiles itself,
     supplied under a URL with no extension, compiles. HTML bytes supplied
     under a URL ending `.png` are refused as
     `image file '…' at line N does not hold image data`.
  6. `data:image/png;base64,…`, `file:///x.png`, `ftp://example.com/x.png` and
     `C:/figure.png` are each refused as
     `image with a URL scheme other than http or https`, and
     `HTTPS://example.com/x.png` is accepted.
  7. The `bibliography` key naming `https://example.com/refs.bib` is refused
     in its existing words, byte-identical. `[](https://example.com/one.md)`
     is a plain link, not an include marker.
  8. The CLI on a document naming a URL exits 1 with clause 4's message and
     reads no file for it.
  9. **The corpus check:** every document in this tree compiles to
     byte-identical Typst source and a byte-identical PDF on either side of
     the change.
  10. `cargo test --workspace` passes.

- **Close-out:**
  - **`rules/pipeline.md`:** "Images and their files" (the refused shapes,
    plus a paragraph on a URL: the bypass, the name and the check), the intro's
    API paragraph (`is_url` and `UnfetchedImage`), and the CLI section's skip.
    **The rule is at 1367 of its 1380 lines**, so the pass either tightens it
    or raises `max_lines` with a stated reason, not quietly.
  - **`README.md`:** the paragraph that refuses "a URL and a `data:` URI,
    because nothing is fetched over the network" is rewritten. A URL is now
    accepted, its bytes are the caller's to supply, and the CLI does not
    supply them yet.
  - **Dated `CORRECTED` notes** beside §1.1's "No fetching, ever" and beside
    §2's "A scheme is a fetch request, and nothing fetches". Both say that
    `core` still fetches nothing and that what changed is that a URL is a
    name.
  - **The release is its own commit afterwards, as 0.2.0's was.**
    `UnfetchedImage` is a new variant of an exhaustive enum, so the release is
    0.3.0. Its notes tell callers that joined every `ImageRef` onto a
    directory, as Letur's asset reader does, to check `is_url()` first.
    Otherwise a URL image turns into an OS error about a file called
    `https:/…`. Letur takes the caller side from there, under its own spec.
    **Packaging:** the CLI's skip already calls `ImageRef::is_url`, which is
    new `core` API. Until 0.3.0 is on the registry, `cargo package -p
    md2pdf-cli` resolves the published 0.2.0 `core` and fails, so the release
    verifies the CLI from the unpacked archives instead.
  - One push.

### Phase 4 — the CLI fetches, when asked
*Produces the observable: **yes**. `md2pdf --fetch paper.md` on a document
naming `https://…/plot.png` writes a PDF that shows it.*

**§6.1, worked:** the decision is Phase 3's, carried to the one caller in this
repository, and how the CLI gets an image's bytes was Phase 2's subject. What
this phase qualifies is one sentence of another spec: `mpdf-001` §2's "the
running app stays fully offline". The decision that sentence closes (the
`World` resolves no packages, and fonts are bundled) stands untouched. Every
run without `--fetch` stays exactly as offline as before. So the sentence gets
a dated `CORRECTED` note naming the one exception, not a supersession.

- **Scope:**
  - **The flag.** `--fetch`, a clap `bool` beside `--emit-typst` in
    `cli/src/main.rs`, is off by default, per OQ-3.
    - **Without it,** `read_assets` keeps Phase 3's skip, and `core`'s own
      `no image fetched for '…'` is the error. The CLI then adds one line of
      its own under it: **`hint: pass --fetch to download images named by a
      URL`**. `core` cannot say this, because the flag is the CLI's and Letur
      has none. The hint is printed only for `Error::UnfetchedImage`, so every
      other message stays byte-identical.
    - **With it,** an `is_url()` entry is fetched instead of skipped: one
      blocking GET per distinct URL, deduplicated by the same `seen` set, one
      at a time, in document order beside the file reads.
    - `--emit-typst` fetches nothing with or without the flag, just as it
      reads no image file today.
  - **The failure message.** A fetch that fails is exit 1 in the shape of the
    file-read message: `cannot fetch {url} for the image {location}:
    {reason}`. The reason is the HTTP status (`404 Not Found`), the transport
    error, the timeout, or the cap. `read_assets` already stops at the first
    failure in document order, and fetching keeps that.
  - **The guards.** OQ-4's limits, each held by a named constant or a
    `Cargo.toml` line, not by a client default that a version bump could
    move:
    1. **Opt-in.** Nothing is fetched without `--fetch` (above).
    2. **Two schemes.** Only `http://` and `https://` reach the client,
       because `core` has already refused every other scheme. A redirect
       leads only where the client can follow, which is those two.
    3. **Time.** `FETCH_TIMEOUT` is 30 s per request, set as the agent's
       global timeout so that a slow trickle of bytes counts against it, not
       only the connect.
    4. **Size.** `FETCH_LIMIT` is 20 MB (20 × 1024 × 1024 bytes), set on the
       body read with `with_config().limit(…)`. `ureq`'s own default is 10 MB,
       per OQ-4. **`ureq` is built with `default-features = false,
       features = ["rustls"]`, which leaves `gzip` out.** Its `LimitReader`
       sits *under* its content decoder: the reader chain is
       `ContentDecoder<LimitReader<…>>`, read in `ureq` 3.4.0's
       `body/mod.rs`. With `gzip` on, the cap would count the compressed wire
       bytes, and a small body could inflate past any size in memory. With it
       off, the cap counts exactly the bytes handed to `core`. An image format
       is already compressed, so the loss is SVG's transfer size and nothing
       else.
    5. **Redirects.** At most 10, set explicitly with `max_redirects` even
       though 10 is also the default.
    6. **Status.** Anything outside 2xx is a failure, which is the client's
       `http_status_as_error` default. The status is kept, and the message
       names it.
    7. **No state.** No cookie store (the `cookies` feature stays off), no
       cache, and nothing is written to disk. A second run fetches again,
       which is the honest reading of a flag its user repeated.
    8. **No trust in the header.** `Content-Type` is not read, because
       `core`'s byte check is the authority, per §2, and a header is one more
       thing a server can get wrong.
  - **The dependency.** `ureq` goes into `cli/Cargo.toml` only, pinned to 3.4.
    `core`'s dependency tree is unchanged, and the gate checks that.
  - **No sample names a URL.** `samples/` converts in CI, and CI must not need
    the network.

- **Exit gate:** CLI tests in `cli/tests/cli_test.rs`, against a server the
  test runs itself on `127.0.0.1` with `std::net::TcpListener` in a thread. No
  new dev-dependency, and no internet.
  1. With `--fetch`, a document naming `http://127.0.0.1:<port>/plot.png`
     twice, the server serving `dot.png`: exit 0, a `%PDF` file, and exactly
     one request seen by the server.
  2. **Without `--fetch`**, the same document: exit 1, stderr reading
     `error: no image fetched for 'http://127.0.0.1:<port>/plot.png' at line
     N` followed by the hint line, and zero requests seen by the server.
  3. With `--fetch`, the server answering 404: exit 1, stderr reading
     `cannot fetch http://127.0.0.1:<port>/plot.png for the image at line N: `
     followed by the status.
  4. With `--fetch`, a port nothing listens on: exit 1 naming the URL and the
     line.
  5. With `--fetch`, a body of `FETCH_LIMIT + 1` bytes: exit 1 naming the cap.
  6. With `--fetch`, a redirect to a second path serving `dot.png`: exit 0.
     Eleven chained redirects: exit 1.
  7. With `--fetch`, a response carrying `Content-Encoding: gzip` over gzip
     bytes: the bytes reach `core` undecoded, which pins guard 4's feature
     line. `core` then reads the gzip magic as SVGZ, which is §2's recorded
     limit for bytes corrupt past their magic.
  8. `--emit-typst --fetch`: exit 0, and zero requests seen by the server.
  9. `cargo tree -p md2pdf-core -e normal` lists no HTTP client, so `core` has
     stayed network-free. `cargo tree -p md2pdf-cli -e features -i ureq` shows
     neither `gzip` nor `cookies`.
  10. `cargo test --workspace` passes.

  **Two things the gate does not cover, and why.** No clause fetches over TLS,
  because a test certificate is more machinery than the check is worth; the
  README's own `https://` example is converted once, by hand, at close-out. No
  clause waits out the timeout, because 30 s per run is more than the suite
  should pay; `FETCH_TIMEOUT` is one constant, passed to the client unchanged.
- **Close-out:**
  - **`rules/pipeline.md`'s CLI section:** the flag, the hint and the eight
    guards.
  - **The README's images section:** a URL is fetched only under `--fetch`,
    within the limits above.
  - **`THIRD-PARTY-LICENSES.md`**, regenerated with
    `tools/third-party-licenses.py`. The TLS stack brings `webpki-roots`,
    which is CDLA-Permissive-2.0. The script's licence-text identifier does
    not recognise that licence today, so it gains the case. `ring` and
    `rustls-webpki` bring ISC. The script already identifies ISC, but the file
    carries no ISC text yet, so the regenerated file gains a section for it.
  - **A dated `CORRECTED` note** beside `mpdf-001` §2's "the running app stays
    fully offline", naming `--fetch` as the one exception.
  - **The `CLAUDE.md` stanza needs nothing:** "plus the images they name"
    already covers a URL.
  - **Packaging:** Phase 4 adds no `core` API, so its release has no
    packaging order to respect beyond Phase 3's.
  - One push.

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/mpdf-002.md, append-only, one heading per round. See §7 of the
methodology.
-->
