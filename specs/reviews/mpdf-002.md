# Review record — mpdf-002 (`specs/images_spec.md`)

Append-only. One heading per round, newest first.

### Phase 1 implementation — 2026-08-09 — **SHIPPED**

Two departures from the reviewed text, both decided with the author before
the work and recorded here rather than folded into the append-only spec.

**A seventh error shape at the image arm.** §2 and Phase 1's scope fix six
shapes. A destination holding a literal backslash — `figures\pipeline.png`,
the way a Windows user writes a relative path — passes all six: it has no
scheme, is not absolute, holds no `..`, and its extension is in the table.
Typst's `VirtualPath` rejects a backslash segment, so the path never
resolves and the compile fails with the compiler's own message naming a
span in `main.typ`. That is exactly the failure §2's pre-compile decision
exists to prevent, so the shape joins the six: `core/src/emit.rs:check_image`
builds the virtual path itself and reports any path error as a construct
error naming the line. The gate's error list grew from ten shapes to eleven.

**`image_paths` runs the emitter's own walk.** §2 says the list comes from
"the same parse the emitter runs". The literal reading — a second walk that
collects paths — would need a second copy of the seven-shape check, and the
two would drift. `core/src/emit.rs:emit` therefore returns the source and
the list together, and `md_to_typst`, `image_paths` and `md_to_pdf` all read
it. One consequence, recorded: `image_paths` errors on any out-of-dialect
construct, not on image shapes alone.

Both gates pass, and the observable was checked by eye as well as by magic
bytes: the fixture's PDF was rendered and read, and the standalone PNG, the
inline SVG and the boxed image with text beside it all appear as the spec
draws them. `rules/pipeline.md` gained an images section and its `max_lines`
rose from 155 to 205, the subject being a whole channel rather than one arm.

The README gap Phase 1's close-out named is closed: its rejection example
now shows a raw HTML block. Its images section still waits for Phase 2, and
that is honest — the binary supplies no assets, so an image document at the
CLI errors naming the file it needs.

### Round 2 — 2026-08-09 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking findings, one new non-blocking. The
reviewer verified every fix against the file, not the changelog.

Blocker 1 is resolved by decision: an extension outside Typst's table —
a missing extension included — is a construct error at the emitter arm,
so everything that reaches pre-compile validation has an extension the
table names, and the "extension decides" claim is now true as scoped.
The reviewer confirmed the enumerated table matches typst-library
0.15.1's own extension table exactly, and that gate case (2) carries the
two new shapes. Blocker 2 is resolved with the reviewer's own OQ-2
answer adopted whole: the single-space rule for breaks, the depth count
over image starts and ends, and the nested destination and title staying
out of validation and out of `image_paths`' list — confirmed against
pulldown-cmark 0.13.4's `raw_text`, including the interaction with the
six-shape check. The six non-blocking fixes were spot-checked in the
file, and the reviewer verified the pending-slot deferral is
behaviorally equivalent to the standalone rule, the two-adjacent-images
edge included.

The new finding, accepted and folded after the verdict: Typst's `is_svg`
is a namespace search over the first 2048 bytes, not a root-element
check, so §2's one word moved from "root-element sniff" to "namespace
sniff" — the degenerate divergence sits inside the decision's recorded
corrupt-past-the-sniff limit either way.

On this convergence: `status: accepted`, and `reviewed: 2026-08-09` on
both phases — the round was document-wide, and no phase had shipped.

### Round 1 — 2026-08-09 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0 (once for this episode, the document-wide round on a spec with
no shipped phase): the document produces the observable — a typeset PDF,
now from documents that name image files — and it is the right one: real
articles carry figures, one image is a fatal error today, and mpdf-001's
Phase 5 explicitly reserved the subject for a spec of its own. Phase 1
produces the observable only at the library level, and the spec argues
the mid-state explicitly. The episode proceeded.

The reviewer's grounding pass confirmed every citation against the
pinned sources, with an empirical probe against pulldown-cmark 0.13.4
for the event-stream claims: Typst's detection order (format, extension,
content) in typst-library 0.15.1's `determine_format`; the
neither-dimension-forced bound in typst-layout 0.15.1's `layout_image`;
block-by-default with `box` as the documented inline form; alt as a
plain string in the accessibility layer; alt text arriving as inner
inline events; a `<div>` block parsing with `Options::empty()` where
strikethrough, footnotes and math each need an option; and the
five-artifact census of the image-rejection tests, with the two-way
resolution covering all five at the right lines. Numbers re-derived:
five artifacts, three cases per gate, two new error variants.

Both open questions were answered from the pinned sources in this round.
OQ-1: a boxed inline image is bounded by the column — the inline
collector passes the paragraph's full region to the box layout, whose
pod keeps the base region for auto sizes, and `layout_image` bounds the
natural size by it. OQ-2: a nested image arrives as a full start–end
pair, pulldown-cmark's own `raw_text` flattens under a depth counter,
and a break contributes a single space.

Verdict: `NOT READY` — two blocking findings, six non-blocking. The
author accepted all eight, rejected none, deferred none.

Blocker 1: **the pre-compile validation was undefined for an extension
Typst does not recognize** — the "extension decides" conclusion had no
defined left-hand side for `photo.bmp`, three implementations were
defensible, and they differ observably. Resolved: the extension table is
enumerated, membership is a construct error at the emitter arm, and the
decision records why Typst's content-detection fallback is deliberately
not mirrored. Blocker 2: **the alt-capture wording did not determine
nested-image behavior** — whether a nested destination is validated,
whether it joins `image_paths`, and what a break contributes were all
open. Resolved with the OQ-2 answer, landed in §2 and Phase 1's scope.

Non-blocking, all accepted: `md_to_pdf`'s callers understated (the
golden suite has ten call sites); "four shapes" enumerated five; the
bare-against-boxed decision needs a one-event deferral, and the gate
under-covered the image-at-paragraph-start case a wrong implementation
gets right by accident — the fixture now pins it; the format set was
never enumerated and `pdf` neither embraced nor excluded; a repeated
path's error line was unstated — first reference now; the Phase 1
close-out named a rule section that does not exist.

Rejections: none.
