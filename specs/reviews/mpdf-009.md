# mpdf-009 — review record

Append-only. Newest round first. One heading per round.

### Round 4 — Phase 5 only — 2026-08-26 — panel of two, fresh, past the cap by the human's call — **NOT READY (folded in; the phase is not cleared)**

**Past §7.6's cap of three, authorised by the human**, as Phases 1 and 2 of this
same spec each went to four. Between round 3 and this one the human directed that
round 3's three design findings be put to a **running prototype** rather than
answered in prose; that harness also found a fourth defect nobody had raised, and
the phase was rewritten from it.

**The fourth defect is the one worth recording, because no reviewer produced it
and the design it broke reads correctly on the page.** A freshness test keyed on
document generation alone — which is exactly what round 3 asked for — left
whole-mode pages holding a canvas made for the *old* width after a rest: **six
pages at a 1120 backing store where 1400 was wanted, every one soft.** That is
Phase 1's own *comes back sharp when you let go*, and its gate clause 3,
regressed. Freshness is (generation, width), and the round-trip 520 → 700 → 520
now returns to 116.64 MiB, sharp, where it opened.

**Sixteen blocking findings, and both lenses agree none needs a design change.**
The exit-gate lens: "none of the nine needs a design change. Seven are one added
or amended sentence." The correctness lens named the pattern that produced its
seven, and it is the finding this round exists for:

> **the prototype settled these questions in code and the write-up recorded the
> measurement instead of the rule.**

Checked against the harness, three sentences described something never built.
**"Awaits the observer's first delivery"** — it waits one animation frame; the
promise version hangs if the observer is disconnected first, stranding
`rendering` above zero and deferring every future width rest for the life of the
window. **"Disconnected by every open"** — it disconnects only on a mode
crossing, which is why a band→band recompile works: the reused wrappers are still
the right targets, and disconnecting without re-establishing freezes the wanted
set. **"Leaving the band frees the backing store"** — it sweeps after every
rendered page, without which a page dropped while its render is in flight has no
canvas to release at that moment and is never revisited. Each was folded in by
reading the harness rather than by reasoning again.

**Two implementations passed every clause of the previous gate and were wrong in
the way the prototype had been wrong.** One renders a band page only when it
holds no canvas — round 3's warning, design fixed and gate never written — and
**loses the author's edit on every long document**. The other leaves `openPdf`'s
positioning where it is, drains the band for `scrollTop` 0, jumps the reader and
shows a placeholder until the rest fires, reaching an identical end state — which
is the phase's **headline claim**, ungated. Both now have clauses.

**The gate goes from thirteen clauses to sixteen** and stops depending on
instruments that do not exist: `__pane` gains `renderMs`, and `made`/`released`
replace a "canvas allocation" reading that Safari's memory timeline does not
present — clause 2's blind instrument reproduced in its own replacement.
Preconditions are asserted with a single-file fixture open, `#pages` shipping
`hidden` and the showcase's own pane being ~305 px rather than 520, because
`#files` shows for a document naming sections.

**Every number re-derived clean, by both lenses independently**, and the
misattribution defect of rounds 1–3 did not recur: clause 15's 12 ms is genuinely
OQ-7's raster median and not the text-layer row. One correction came with it —
that median was taken at a **619 px** pane and the gate pins 520, so the
comparison is against ~8.5 ms, and it is a WKWebView number already rather than
a Chromium one to re-take.

**`reviewed` stays `null`.** What four rounds and two prototypes have established
is that the mechanism is sound and observed, and that this phase's remaining risk
is no longer design but the distance between a document and the program it
describes. **The next decision is the human's**, and the honest options are a
fifth round scoped to whether the scope now matches the harness, or building it
against the harness as a reference and letting the gate do the work it was
written for.

### Round 3 — Phase 5 only — 2026-08-26 — panel of three, fresh, against a phase rewritten from a prototype — **NOT READY (at the cap; escalated)**

**The design converged and the gate did not, which is a different failure from
round 2's and worth separating.** All three lenses confirmed the mechanism, and
the correctness lens confirmed the claims about the shipped file line by line:
`rendering > 0` genuinely holds across an await inside `renderPages` and
`rerender` checks it *before* `++renderSeq`, so a `settle` firing during an open
cannot bump the generation; a band render that neither bumps `renderSeq` nor
calls `cancelRenders` is safe because the per-await checks abort it before it
touches the DOM; `drawLayers` composes with a `DocumentFragment` verbatim;
placeholders satisfy every shipped consumer of a `#pages` child; and
`rootMargin: '100% 0px'` is well-founded, the initial observation reporting an
entry for every newly observed target.

**The arithmetic is mutually consistent rather than merely re-derivable**, which
is the strongest thing this round established. 71 × (735 + 16) + 16 = **53,337
exactly** — the pane width, the page height, Phase 4's gap and `#pages::after`
all agreeing on one number that no single figure could have been
reverse-engineered from. The consistency lens and the correctness lens reached
it independently.

**Eighteen blocking findings. Three are design, and the rest are the gate.**

1. **The budget's evaluation moment, found by all three lenses**, and round 2's
   finding 4 undischarged — the only one of that round's eight still open.
   `whole ≤ 128 MiB` is a function of pane width, and four shipped causes move
   it, none of them an open. Solved for the gate's own fixture, the boundary sits
   at a **289 px pane**, which the divider permits, so `long.md` crosses the
   budget in ordinary use in both directions. Whether an observer is created
   mid-life, whether canvases are released on whole → band, and whether the
   observer is disconnected on band → whole all have no answer in the text, and
   the retention consequences differ by five to seven times.
2. **What the drainer renders is never stated, and one reading loses the
   author's edit.** A compile goes through `openPdf`, wrappers are reconciled
   rather than replaced, so at a recompile the band's wrappers already hold the
   *previous* document's canvases. "Render only the wanted pages that hold no
   canvas" — the natural reading, and the one consistent with the phase's own
   five-render measurement — then renders nothing, and **an edit never reaches
   the screen on exactly the documents this phase exists for.** The missing rule
   is that a canvas belongs to the document generation that produced it.
3. **The 120 ms rest's scope is unstated, and a programmatic scroll fires a
   scroll event.** `applyAnchor` and `goToDestination` both write `scrollTop`, so
   a rest that gates every scroll costs the open path 120 ms it claims not to
   pay; and if the open path bypasses the drainer instead, then at a width rest
   the band pass inside `drawPages` and the observer's own drainer run
   **concurrently at the same `renderSeq`**, neither generation check aborting
   either. Round 2's finding 5 fixed drainer-versus-drainer; this is the
   uncovered half.

**The gate's two central quantities are measured by an instrument that cannot see
the defect and by no instrument at all.** Clause 2 sums over `#pages`, which
counts *attached* canvases — and the scope's own release paragraph says a canvas
detached first is invisible to exactly that measurement, so an implementation
that never zeroes passes at 17.5 MiB while holding 414. Clause 6 counts "renders
started", which is module-scoped and unreadable from the console; so are clauses
3 and 9. **Clause 3 is inverted**: it fails a correct implementation, since with
the await a slow delivery costs latency and not correctness, and passes the
non-awaiting defect, which measures the same interval. **Clause 4's stated
discriminator is false** — a per-page append still ends at 53,337 — so the
one-mutation commit, which the scope calls a decision rather than a style, is
checked by nothing. **Clause 7 names no landing coordinate**, which is verbatim
the defect Phase 2's clause 3 was written against.

**And the fixture defeats the gate on two ordinary machines.** At
`devicePixelRatio` 1 a page costs 1.458 MiB and `long.md` is **103.5 MiB, under
the budget** — it renders whole, no observer, and clauses 2, 3, 6 and 8 have
nothing to measure. With macOS's default overlay scrollbars the pane is 535 px
rather than 520, `scrollHeight` reads 54,899, and **clause 4 fails a correct
implementation outright**. Phase 1 and Phase 4 both pin the scrollbar setting;
Phase 5 inherited their geometry without inheriting the pin.

**One inherited error propagated into a new literal.** The showcase names five
sections, so `#files` is shown by default, and the showcase's pane at the default
window is roughly **300 px, not 520** — six pages at ~12.3 MiB rather than 34.99.
Clause 1's verdict is unaffected either way, but the gate instructs a tester to
record a threefold disagreement as a finding.

**The headline latency claim is wrong in a third way, in its third draft.** "Its
first page is on screen 27–147 ms after the layout instead of 1,182 ms" — but
`drawPages` appends each wrapper inside its own loop iteration, so page 1 is on
screen after page 1's own render today, which Phase 1 measured at 42 ms. What
waits 1,182 ms is the page the reader is *taken to*, because `applyAnchor` runs
after the loop. The claim is right for an edit compile and wrong for an open from
disk, which is the flagship scenario. The consistency lens separately found that
the sentence promising to re-take the per-page render in the app points at clause
9, which times `getPage` — the page fetch, not the raster.

**Escalated at the cap per §7.6.** `reviewed` stays `null`. What the round
established is that the mechanism is sound and the remaining work is of two
kinds: three questions the prototype can answer by observation as it answered the
last three, and a gate that needs rebuilding around instruments that exist,
preconditions that are asserted, and a fixture that is pinned.

### Round 2 — Phase 5 only — 2026-08-26 — two fresh reviewers standing in for round 1's — **NOT READY (escalated below the cap, with a diagnosis)**

**§7.4's same-agent resume was unavailable, not skipped, and this round is
weaker for it.** Round 1's three agents could not be resumed — no transcript —
so round 2 ran two fresh reviewers, one carrying the correctness lens and one
carrying scope and exit-gate together, each handed round 1's findings verbatim
and asked to verify them against the files. That recovers the checklist and not
the memory. Phase 2's round 4 recorded that two of its eight blockers "would not
have been caught by a fresh reviewer"; this round had no way to reproduce that
property and the record should not pretend otherwise.

**Eight distinct blocking findings, and every one of them was introduced by
round 1's fixes.** §3 of the loop names this pattern — *a fix can introduce a
blocker* — and this is the sharpest instance in the corpus.

1. **The retention expression was a ceiling written where a value was meant**,
   found independently by both reviewers, which is the round's strongest
   finding. The rule says *whole ≤ budget ⇒ render every page, otherwise the
   band*, so retention is `whole ≤ budget ? whole : band`. The phase wrote
   `min(whole, max(band, budget))` and the gate compared retained bytes to it.
   Worked at the gate's own geometry, a **correct** implementation retains
   17.5–29.2 MiB where the clause predicts 128 — off by four to seven times —
   and read as an inequality instead the clause tolerates an implementation
   holding four to seven times the design. That is round 1's own "100 MB admits
   a 3× leak" reproduced with a different literal.
2. **Fixing clause 1 emptied it.** The budget rule made "the showcase is
   unchanged" true rather than false, which was right — and by construction the
   showcase now never exercises a band render, a release or a re-entry, while no
   other clause looks at a layer at all. Releasing a canvas and leaving both
   layers attached passes all ten clauses, and so does appending a second text
   layer on every re-entry, which is the leak Phase 2's clause 7 exists for.
3. **`.view` on a placeholder is ungated and unreachable.** Round 1 forced it
   into the design; it is read only by `goToDestination`, no clause on the long
   document follows a link, and the fixture is specified by `/Count 71` alone, so
   nothing requires it to contain a cross-page destination. An implementer who
   sets `.logical` and `.natural` and skips `.view` passes the whole gate and
   ships a viewer that throws on any cross-reference into an unrendered page.
4. **When the budget is evaluated is unstated.** Once at open, a twenty-page
   document dragged to twice the width retains some 470 MiB — the phase's subject
   failing in the scenario it cites as its own urgency — and no clause can tell
   the two answers apart.
5. **Nothing serialises band passes.** The observer's callback fires repeatedly
   through a scroll; the phase's only guard skips pages that have *left* the
   band and says nothing about a second pass finding a page whose render is in
   flight. The same page reaches `page.render()` several times on one throw, and
   clause 7's own counter is what breaks.
6. **"The canvas is what is swapped inside a stable wrapper" drops the layers.**
   `drawLayers` *appends*, which is safe today only because the wrapper is new on
   every pass. Under reconciled wrappers the literal reading yields two text
   layers and two annotation layers per page on every compile — which contradicts
   clause 1's own claim that Phase 2's clause 7 re-runs and passes verbatim.
7. **The reordered open asks the observer for a band it has not reported yet.**
   `IntersectionObserver` delivers its first records asynchronously, so at step
   three the band is empty; and the phase forecloses the workaround itself by
   deciding the band is observed and not computed. The remaining option adds an
   await inside `openPdf` — the one function round 1 blocked on for exactly this
   reason — needing a generation re-check and a statement about `rendering`.
8. **Clause 4 fails a correct implementation** unless the sizing pass commits its
   wrappers in one mutation, which the scope does not say and today's `drawPages`
   does the opposite of.

**Both reviewers re-derived every byte figure and found them right** — 5.832 MiB
at a 520 px pane, 8.264 at 619, 35.0 for the showcase, 414.1 and 586.8 for the
long document, 21 and 15 pages admitted, 4.59× — confirming that the probe's
"MB" can only have been MiB. **The one wrong quantity was the retention
expression itself**, which no amount of arithmetic checking would have caught,
because it was a modelling error rather than a slip.

**Escalated to the human at round 2 rather than spending round 3, and the reason
is a diagnosis rather than fatigue.** Twenty-three blocking findings across two
rounds, with round 2's eight all generated by round 1's fixes, is the signature
of a phase being *designed* in the review loop rather than reviewed in it. The
three questions that keep producing blockers — how band passes serialise, how
the layers swap under a stable wrapper, whether the observer's first delivery
lands before the band pass needs it — are questions a running prototype settles
in an afternoon and that no reviewer can settle from prose. The project's own
practice is that UI work is prototyped before it is specced; this phase was
drafted from first principles instead, and the loop has been paying for it.
**`reviewed` stays `null`.**

### Round 1 — Phase 5 only — 2026-08-26 — panel of three, fresh — **NOT READY**

**Round 0, answered before the panel and recorded here per §7.0.** *Does this
phase produce the observable, and is it the right one?* Yes, conditionally, and
the condition is the phase's weakest point: for a document the pane can already
show, nothing a reader sees changes; the stronger claim, that a long enough
document would otherwise not be shown at all, rests on OQ-8, which is
unmeasured. It is the right thing to build before Phase 3 on an argument that
does not depend on OQ-8 — a canvas is 8.26 MiB at fit and 33 at 200%, so the
zoom knob multiplies the quantity quadratically. **The qualification is recorded
rather than resolved**, and round 2 did not disturb it.

**Three fresh reviewers with repo access, one lens each — correctness and
grounding, scope/YAGNI/§6.1, exit-gate testability — fifteen blocking findings
between them.** All fifteen were accepted; the fold-in is `da46677` and the
arithmetic it shipped wrong is `f80c6a5`.

**Two were design changes rather than wording.** The correctness lens re-derived
the draft's premise that "six pages is inside any band this phase can choose"
and found it **false** at the app's own default window, where a correct
implementation would have left the last pages blank and failed the very Phase 2
clauses the draft told it to re-run — which forced the budget rule that decides
whether to virtualise at all. And nothing said how the band is seeded at open:
`openPdf` renders and then scrolls, so on a long document every compile at the
caret's page would have swapped in a blank wrapper, which forced the layout →
position → render ordering. Round 2 then found that ordering cannot execute.

**Two claims were withdrawn rather than repaired.** The "some 5 ms a page"
`getPage` figure and the 355 ms sizing budget derived from it were OQ-7's
*text-layer* median read as something else — the probe's table has no page-fetch
row at all — so they went, along with the header's "some 100 ms" latency claim,
and a gate clause now measures the sizing pass instead. And §6.1's step-1 prose
bullet **does** match: §2's "the first two are rebuilt by Phase 2" stops being
true for a page outside the band, which the draft's own "two costs" paragraph
said four paragraphs after claiming no shipped prose went misleading. **That is
the shape Phase 4's round 1 caught, recurring two phases later**, and it is now
discharged by a dated `CORRECTED` note in the close-out.

**One rejection, recorded.** The exit-gate lens read `rules/desktop-panes.md` at
375 body lines, at its cap; `spec-lint` reports 374/375 and the scope lens read
374 independently, so the close-out's figure stands.

**Three confirmations worth keeping**, since they cost a round to establish: the
`IntersectionObserver` root genuinely works given `#pages`'s `overflow-y:
scroll` and `position: relative`; `canvas.width = canvas.height = 0` is the
right release idiom and the vendored bundle uses it for the same purpose; and
the phase is one plan-mode pass rather than two, the `#[ignore]`d generator
notwithstanding.

### Round 4 — Phase 2 only — 2026-08-25 — same reviewer, resumed with the author's changelog — **READY (converged)**

**Run past the loop's cap, which a person decided.** §7.6 escalates at three and
round 3 escalated; the human was shown the outstanding blocker and authorized
this round. Phase 1 of this spec took the same shape, and that precedent is why
the option was offered rather than the date simply being set.

All four of round 3's changes confirmed against the files, by `grep` rather than
by reading the changelog: `writes it back to \`1\`` survives only inside §2's
retraction, where it names a rejected draft, and `whole of the scaffolding`
returns nothing.

**The check this round existed for was the end-to-end read**, since round 3's
finding was that the scope block was internally inconsistent while every
individual statement had a correct counterpart somewhere else in the document.
Read straight through as an implementer, in order, without consulting §2:
**nothing in it contradicts anything else in it.** The reviewer walked the chain
at every join — `.logical` on the wrapper, `size()` sizing the wrapper, the
canvas at `width/height: 100%`, `size()` writing `--total-scale-factor` as
CSS-width-over-unscaled-width, the gesture-and-rest paragraph deferring to that
one write rather than restating a value, the margin and hairline on the wrapper
so a canvas at `height: 100%` has none left to overflow with, `clear`'s
`replaceChildren()` disposing layers with wrappers because they are inside them,
and the build order putting the layers inside the same detached wrapper `size()`
already sized. **The value round 3 found stated twice is now stated once, in the
paragraph that owns it.**

Three non-blocking refinements folded in on convergence. `--scale-round-x` and
`--scale-round-y` are restated in the scope's `size()` paragraph rather than left
under a pointer to §2 — undefined they take the layer's size down with them,
which is clause 5's failure by a second route, and it was the one load-bearing
value the block did not carry. "Five shipped functions" against "all seven sites"
is resolved as **seven sites across six functions**, the prune living inside
`drawPages`. And "the text layer and the annotation layer as its two siblings"
becomes "beside it **inside that wrapper**", the rejected shape in the next
sentence having used the same noun with only a location clause to tell them
apart.

**Phase 2 `reviewed: 2026-08-25`.** `status` was already `accepted`. Nothing
about Phase 3 was judged in any of these rounds, and its `reviewed` stays `null`.

**What the four rounds cost and bought, since this spec now has two phases that
went to four.** Eight blocking findings in all: six in round 1, two in round 2 —
**both of which the round-1 fix introduced** — one in round 3, which was the
round-2 fix failing to delete what it superseded, and none in round 4. The two
that the fixes introduced are the loop's own §3 warning observed twice in one
episode, and neither would have been caught by a fresh reviewer: round 2's
needed someone who knew what the fix had claimed, and round 3's needed someone
who knew which sentence the fix was supposed to have removed. **That is the
argument for §7.4's same-agent resume, measured rather than asserted.**

### Round 3 — Phase 2 only — 2026-08-25 — same reviewer, resumed with the author's changelog — **NOT READY (escalated at the cap)**

**The design converged this round and the text did not.** Both round-2 blockers
confirmed resolved against the files, and the reviewer re-derived the `size()`
decision at all three of its call sites rather than taking the changelog's word:
`drawPages` establishes `.logical` and the unscaled width on the two lines above
its `size(wrapper, 1)`, so the property is written and not read; `fit`'s
`size(wrapper, width / logical.w)` reduces to `width / natural.w`, which also
survives the mid-gesture mixed-raster case the per-canvas factor exists for; and
`unscale`'s `size(wrapper, 1)` yields `logical.w / natural.w`, the render scale —
"precisely the value that is **not** `1`".

One blocking finding, and it is an author's failure to delete rather than a
design fault: **the round-2 paragraph survived verbatim inside Phase 2's scope**,
still saying `unscale()` "writes it back to `1`" and that the three properties
are "the whole of the scaffolding", four paragraphs below the corrected `size()`
text and three below the stylesheet requirement. §2 retracted both, but phrased
the retraction as "an earlier draft of this section said…", which points at §2's
history rather than at a live sentence in the phase — and §3 makes the phase
block the unit of a plan-mode pass, so an implementer planning from it had to
choose between two adjacent instructions, one producing a 595 px layer over a
535 px page and the other building no stylesheet. Fixed by replacing the
paragraph.

Two non-blocking folded in with it: §2's own bullet still asserted the claim §2
later calls false — that the bundle's inline span styles key off
`--total-scale-factor` — and the clause is deleted rather than annotated, Phase 2
not having shipped, so this is §5's consistency sweep and not §6.1's `CORRECTED`
discipline; "the five sites" is now seven and said so in one place while still
reading "five" in the next paragraph; and the build order is decided rather than
left open — **the layers are built while the wrapper is detached and swapped in
with it**, which extends Phase 1's "a canvas is swapped in only once it holds
pixels" to cover them, and is available because nothing in the layer path needs
layout.

**Escalated at the loop's cap of three rounds**, per §7.6. Nothing is set: no
`reviewed` date, `status` unchanged. What is outstanding is the confirmation
round, not a design question — every literal the gate is keyed to reproduced
again this round.

### Round 2 — Phase 2 only — 2026-08-25 — same reviewer, resumed with the author's changelog — **NOT READY**

All six round-1 blockers confirmed resolved. **Two new ones, and both were
introduced by the round-1 fix** — the failure mode §3 of the loop names, caught
in the round that exists to catch it.

1. **`--total-scale-factor` is an absolute scale and the fix had written it as a
   relative one.** `PageViewport`'s `rawDims.pageWidth` is `viewBox[2] -
   viewBox[0]` — unscaled, 595.28 for A4, independent of the render scale — and
   `setLayerDimensions` multiplies the property by it. So "`unscale()` writes it
   back to `1`" puts a 595 px layer over a 535 px page on the app's own default
   geometry: an 11.3% offset on every span and every annotation rect, with most
   of the overhang clipped by `overflow-x: hidden` rather than visible. §2 now
   carries the value as one expression, current CSS width over unscaled width.
2. **The layers have no stylesheet, and defining three properties is not one.**
   A text span receives `left` and `top` as percentages, `--font-height` in
   unscaled px, `font-family`, and `--scale-x`/`--rotate` where they apply —
   **no `position` and no `font-size`** — and `_createContainer` is the same
   shape with no box for its `<a>`. Without app-written rules the text layer
   renders as a wall of readable text over the raster.

**The reviewer corrected the author on the record**, and was right: the
changelog claimed the bundle's own span styles key off `--total-scale-factor`,
and that property appears **zero times** inside the `TextLayer` class body — the
`calc()` hits are annotation-editor code. Verified independently before the fix
was written.

The round's sweep also found two sites the author's list had missed, and the
second is the catch of the round: `drawPages`' prune, and **`size()`, the only
writer of a CSS box in the file**, which `fit`, `unscale` and `drawPages` all
delegate to. With `.logical` on the wrapper, `size()` sizes the wrapper and
nothing sizes the canvas inside it — a canvas with no CSS size lays out at its
backing store, which at `devicePixelRatio` 2 is a page twice the pane's width,
and is the exact failure Phase 1 argued `unscale()` into existence to prevent,
arriving by a different door. `size()` is now both the property's home and the
sixth named site.

Non-blocking folded in: gate clause 4 looked for all five external annotations
on page 2 where **four** are there and the fifth is inside a footnote on page 5 —
the one a filter applied to the wrong page would miss, and §2's enumeration had
the same off-by-one; `_isValidProtocol` accepts five protocols, `tel:` included,
where §2 listed four; the three stylesheet comments in `app/dist/index.html`
(`#pages canvas`, `#pages::after`, `#pages`) go false with the wrapper and are
written into the close-out, this being Phase 4's round-1 miss recurring one phase
later; and `TextLayer.update()` is recorded as *not* called per gesture step,
`--scale-x` being a measured-to-target ratio a pure scale change does not
disturb.

**On the observable, the reviewer withdrew its own round-1 point**: "this closes
a regression against six shipped phases rather than adding a capability" is a
stronger argument than Phase 4's, not a weaker one, so the phase stays **yes** in
Phase 4's argued class. The round-1 finding had been about the flat assertion,
which is gone.

### Round 1 — Phase 2 only — 2026-08-25 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode): yes.** Phase 2 restores two of the three properties
`mpdf-003` §2 chose to draw a real PDF for — live link annotations and selectable
text — which Phase 1 of this spec removed with WebKit's view; the page has
neither today. It closes a regression against six shipped phases rather than
adding a capability, which is the stronger form of the argument, and the one fact
that would trip an implementer is already banked in §2 as a measurement.

One generalist rather than a panel: Phase 2's blast radius is one file, which is
where `mpdf-003` Phase 8's round drew the same line.

**Six blocking findings, all accepted, none rejected.**

1. The phase named **no file, no function and no DOM shape** — against §3's "each
   phase names the files and functions to touch" — and the shape it left unnamed
   collides with the `#pages` children invariant Phase 4 shipped. Five functions
   index `pages.children` or dereference `canvas.logical`; layers as siblings
   break the indexing, layers under a non-child wrapper get dropped by
   `replaceChild`.
2. A per-page wrapper **silently breaks Phase 4's gap arithmetic**: `#pages
   canvas` still matches a canvas nested one deep, so the 16 px would move inside
   the page and Phase 4's `offsetTop` expression would read `0` where it shipped
   `16`.
3. **"The layers are positioned from the same viewport the canvas rendered with"
   is false for a gesture** — the case §2 spends its longest decision on. A rest
   makes a new viewport; `fit()` makes none, so the layers would drift through
   every drag, which is the outcome the sentence claimed to prevent.
4. **The phase assumed layer machinery `app/dist/pdfjs/` does not contain.**
   `SimpleLinkService` is zero hits in the bundle, and `--total-scale-factor`,
   `--scale-round-x` and `--scale-round-y` are read by `setLayerDimensions` and
   defined only in an unvendored `pdf_viewer.css`.
5. **The external-link rule was undecided by the phase's own words** — "belongs
   to a round" — unspecified as a mechanism, and cited `mpdf-001` §2, which is
   about Typst package resolution and font discovery inside the compiler's
   `World` rather than about following a hyperlink.
6. **The exit gate was "it runs"**: no document, no page, no expected string, no
   instrument, and no clause at all for the external-link rule.

**What the round measured, so a later one need not.** `/Count 6`; page 1 at
**190** text items beginning `"Everything the Dialect Carries"`, the full profile
190/173/165/139/234/67 for 968 in all; `getOutline` at six entries; and
`getTextContent`'s trap grounded in the vendored bytes as `for await (… of …)`
over the stream.

**The author sharpened one finding past the reviewer's own statement**, and it
became the clause an implementer fails first: `sections/text.md:56`'s
`[this one](#fig:pipeline)` reaches the PDF as a **`/URI (#fig:pipeline)`
annotation refused on protocol**, not as a destination-less one — so a filter
written as "has no URL" renders it. The author's parse of the compiled PDF also
found that **twenty** internal destinations exist where the gate had assumed
cross-references: seven are, and the rest are footnote marks, their return
arrows and citation marks, so the phase delivers a document navigable three ways.
Re-derived with it: `[](#fig:halves)` → Figure 4.2 at **458 pt down an 841.89 pt
page 4**, and `[](#tab:kinds)` → Table 3.1 on **page 3**, the document's only
cross-page reference.

Two new open questions came out of the round: **OQ-6**, what a reader gets back
for a link the app refuses to open, and **OQ-7**, what the layers cost per render,
pointed at OQ-1's page budget.

**One partial correction to the reviewer, changing no fix:** Phase 4 was said to
be "explicitly forbidden to touch" `fit`/`unscale`/`size`. Phase 4's scope says
"the stylesheet alone" about *itself*, which is a statement of its own scope and
not a prohibition binding Phase 2. The substance stood and Phase 2 now says so.

### Round 3 — Phase 4 only — 2026-08-25 — panel of two, resumed — **READY (converged)**

Both lenses READY, zero blocking, within the three-round cap. Phase 4's
`reviewed` set to 2026-08-25. Phases 2 and 3 remain `null` and are not cleared
to build.

Narrow by design: round 2 had already converged, and this round existed only
because folding round 2's non-blocking findings re-keyed a gate clause to a new
measurement — §7.3's warning that a fix is how a blocker gets introduced.

**Both lenses independently sharpened clause 1 in the same direction**, and the
author's own sentence was the thing wrong: the hedge "16 or 32 depending on
whether the engine carries a scroll container's last margin into `scrollHeight`"
understates the test. With `#pages::after` present the last canvas's bottom
margin is not a last-child margin at all — it collapses with the pseudo-element's
zero top margin and contributes a real 16 px of advance — so a stray
`margin: 16px 0` reads **32 on any engine**. The check is more deterministic than
the draft claimed, which is the harmless direction for a gate to be wrong in.

**A tester trap was caught and is now in the clause**: clause 1's three
measurements must be taken at a *settled* width. `fit()` writes fractional
heights while a width is moving, so the rounded differences can read 15 or 17
with the margin still exactly 16.

Measured by the panel this round, on six canvases rather than argued: correct
`margin-top: 16px` gives 16 / 16,16,16,16,16 / 16; the `margin: 16px 0` failure
gives 16 / 16,16,16,16,16 / **32** while a declaration read still reports
`16px`; `margin-top: 3%` at a narrow pane gives **9 / 8,9,8,9,8** / 16 with the
declaration resolving to `8.54688px`. So expression 3 alone carries the
shorthand discrimination, and clause 5 catches the percentage on expressions 1
and 2. 16/535 = 2.99%, re-derived.

**Won't do**: the scope says "some 535 px" where §2 carries "~540 px" and the
763.7 px A4 figure keyed to it. §2 is shipped prose no gate reads, and §6.1's
rule on stale figures says to fix one only in a section the new phase touches.
Recorded so a later round does not read the two as a contradiction.

### Round 2 — Phase 4 only — 2026-08-25 — panel of two, resumed — **READY**

Both lenses READY, zero blocking. Both had raised round 1's blocker
independently, and both verified the fix against the files rather than the
changelog.

Non-blocking findings folded on convergence: `#pages::after` needs `content: ''`
and `display: block` to generate a box at all; 16 px was the one value in the
phase carrying no argument; and — the finding that forced round 3 — gate clause 1
read *declarations* where it should read *rendered geometry*.

The correctness lens **withdrew its own round-1 suggestion**: it had offered
`inset` as one escape from the blocker, and confirmed on re-review that an inset
shadow paints beneath a replaced element's content, so an ink ring under an
opaque `pdf.js`-filled bitmap would be invisible. That is now recorded in the
phase so a later round does not propose it.

### Round 1 — Phase 4 only — 2026-08-25 — panel of two, fresh — **NOT READY**

**Round 0 (this episode — one appended phase): yes, with the weakness named.**
The phase changes no byte of the compiled PDF, but Phase 1 put the artifact on
screen as one uninterrupted white strip, so its *pagination* — a property of the
artifact, not of the chrome — was invisible. The phase argues the observable
explicitly rather than claiming a clean yes, which is what §3 asks of a thin
claim; a clean yes would have been the finding.

Two blockers, thirteen non-blocking. **Every finding accepted, none rejected.**

**Blocker 1, found independently by both lenses with independent browser
probes.** The scope prescribed `box-shadow: 0 0 0 1px` and gate clause 1 demanded
a hairline "on all four sides" — unpassable. A fit-to-width canvas is exactly
`pages.clientWidth` wide and `#pages` ships `overflow-x: hidden`, so a 1 px
spread paints at x ∈ [−1, 0) and [W, W+1), both outside the clip. Measured:
canvas border-box right edge and container clip edge coincide at x=285, and
`scrollWidth` stayed at `clientWidth`. **The requirement was wrong, not the
mechanism**: a page flush to the pane's own side edges has no background beside
it to separate it from, so a side hairline carries no information. Resolved to a
top-and-bottom hairline painted into the gap.

**Blocker 2.** Clause 6 read "visible in light and not obtrusive in dark" —
§3's "looks done" verbatim — and no colour was named anywhere, so `box-shadow`
would resolve to `currentColor` and inherit `--ink`: an ink ring in light, a
near-white one on white paper in dark. Resolved to `var(--edge)` with the
computed value pinned per palette.

**The sharpest non-blocking finding was about the argument, not the code**: the
§6.1 preamble asserted step 1 did not match *while the close-out performed one
of step 1's own bullets*. The conclusion held — nothing shipped is removed, no
phase is cut, and §2 pre-authorised a phase for exactly this — but the argument
was self-serving as written. It now says step 1's phase-removing bullets do not
match, step 1's prose bullet does, and the close-out discharges it.

Also caught: §6.1's correction rule was cited as the third when it is the
second; clause 2 added a width-equality check that Phase 1 itself records as
passing by construction; clause 3 dropped Phase 1's named instrument; clause 4
re-ran one of OQ-2's three causes while claiming all of Phase 1's clause 4;
clause 5 named no measurement method; the gap constant and the carrying property
were never stated; the anchor's "reproduces it exactly" overstated the in-gap
case across a scale change; the side-padding argument was weaker than the fact
(`clientWidth` *includes* padding, so padding would not reduce `paneWidth` at
all — the canvases would be sized past the content box and clipped silently);
and `rules/desktop-panes.md`'s `covers:` was unnamed by the close-out.

**A third copy of the corrected sentence lives in the stylesheet**, in the very
block this phase edits. The author's grep for it missed on a line break — the
same unguarded-match failure this record noted at Phase 1's round 3, recurring
in the pass that was checking for it.

### Round 4 — Phase 1 only — 2026-08-25 — panel of three, resumed — **READY (converged)**

**Past the loop's three-round cap, by the human's explicit decision** at the
escalation. All three lenses READY, zero blocking. `status` set to `accepted`
and Phase 1's `reviewed` set to 2026-08-25; Phases 2 and 3 remain `null` and
are not cleared to build.

Round 3's two blockers resolved, both in the shape their raisers prescribed.
**The gesture's drift**: the *(page, fraction)* anchor is now taken when the
gesture *starts* and reapplied on every step, not taken at the render, where it
would have preserved a displaced position faithfully and restored nothing.
§2, §3 and gate clause 4 were brought into agreement, and clause 4 now reads
the anchor "before the drag was started", which is what makes it able to fail.
**Gate clause 8** is re-keyed to the drawn canvas showing the edit rather than
to the header's word, `current`/`stale` being set in Rust from the compile
alone on a thread occlusion does not stop — the old clause passed on the
defect it existed to find.

**The drift figure was re-derived independently** from the app's default
geometry — a 900 px window, the text pane at 40%, A4 at 763.7 px a page — giving
611 px of displacement on page 5 for a 20% widen, against the draft's "some
600 px". The spec now carries the derivation.

Non-blocking folded in on convergence: what marks a gesture's *start* for a
cause with no pointer event is now stated — `box().width === fitted` opens one,
a difference continues one — which also closes the window-resize hole in clause
4; clause 4 runs again for a window drag-resize; and a compile landing
mid-gesture is decided to keep the reader's place and skip case 2's caret jump
for that compile, on the grounds that it never moves the page out from under a
hand that is on it.

**The pattern across four rounds, recorded because it is the finding about this
phase rather than in it: every round's blockers were introduced by the previous
round's fix.** Eleven blocking findings, all accepted, none rejected — seven
latent in the draft, four created by fixing them. It is a mechanism whose parts
are tightly coupled enough that each correction shifts load onto the next, and
a fifth round would not have been surprising.

One author error worth its own line: the `NOTICE` correction was reported as
folded in when an unguarded string replace had matched nothing across a line
break. A reviewer caught it, not the author. Every edit since has been
assert-guarded.

### Round 3 — Phase 1 only — 2026-08-25 — panel of three, resumed — **NOT READY (escalated at the cap)**

Two of three READY; the grounding and exit-gate lenses each returned one
blocking finding, **both newly introduced by round 2's own fixes**. §7 rule 6
caps the loop at three rounds, so it stops here and the human decides. Nothing
was set: `status` stays `draft` and Phase 1's `reviewed` stays `null`.

Round 2's blocker confirmed resolved by its raiser, against the code: `fitted`,
`box()`, `fit()`, `unscale()` and `settle()` are all kept by name, `settle`
whole at 200 ms with the divider's `pointerup` still calling it, and only
`draw`, `remint` and the object URL's minting, revocation and fragments go —
a coherent cut, `settle`'s first half being the only caller of `remint`. Gate
clause 3 was confirmed genuinely discriminating: at rest `cssWidth ==
paneWidth` and the backing store is `floor(paneWidth × dpr)` so they agree,
while during a gesture they diverge.

**Outstanding blocker 1 — the gesture moves the reader, and nothing governs
it.** Switching `fit()` from a transform to CSS `width`/`height` is right, but
it relocates round 1's pixel-offset problem out of the re-render and into the
gesture. `fit()` now reflows, so a gesture of factor *s* maps content offset
`T` to `T·s` while the browser holds `scrollTop` at `T` — the reader is
displaced by `T(s−1)` continuously, about 600 px on page 5 of the showcase for
a 20% widen, and WebKit implements no scroll anchoring that would compensate.
OQ-2 case 3 names the drag as a cause that "restores where the reader was",
but its capture point is "taken before the re-render", which under a 200 ms
settle is *after* the drag displaced them; the anchor then faithfully preserves
the displaced position. Gate clause 4 inherits the ambiguity — read "before the
drag" it fails a correct implementation of §4, read "after the drag" it passes
trivially. Prescribed fix: fix the capture point at **gesture start**, held
through the gesture and reapplied at the rest render, with clause 4 saying
"before the drag".

**Outstanding blocker 2 — gate clause 8 passes on the defect it exists for.**
Clause 8 was added in round 2 to sample OQ-5, and it is keyed to `current` /
`stale`. Those are compile-status words set in Rust from the compile alone;
the watch loop compiles on its own thread with no webview involvement, so in
OQ-5's scenario the header reads `current` whether or not a canvas ever
rasterised. A pane stalled behind another window shows the previous page under
a header saying `current`, and the clause passes. Its own text claims it "is
the only clause that can see it", which makes the closure void rather than
partial. Prescribed fix: key it to the drawn page — *the page on screen shows
the edited section* — not to the status word.

Non-blocking left standing, agreed across lenses: §2's `(decision, recorded)`
heading still reads "A gesture is carried by a **transform**" while §4 argues
against exactly that, and a heading is the surface a later pass cites; the
frontmatter's `reference:` still promises a `NOTICE` that `pdfjs-dist` 6.2.108
does not ship — **reported as folded in and it was not, an unguarded string
replace that matched nothing**; `unscale()` "resets the canvases to" inverts
for a canvas, which clears to its backing-store size, so the CSS size must be
*written* to the logical one; and `fitted`'s only writer today is the deleted
`draw()`, so the new render must be named as its writer.

### Round 2 — Phase 1 only — 2026-08-25 — panel of three, resumed — **NOT READY**

Two READY, one NOT READY. All seven of round 1's blocking findings confirmed
resolved against the files, including independent re-derivation of `settle`'s
call graph, `relines`' five call sites, and 454,669 + 1,262,398 = 1,717,067 B.

**Blocker, newly introduced by round 1's own fix:** §2's new "a gesture is
carried by a transform; a rest is answered by a render" decision re-adopted
`mpdf-003` Phase 7's five-part design in full, while §4 still deleted all five
parts by name and offered a `ResizeObserver` in their place — which has no
notion of moving versus rest, that being precisely what `settle` supplies.
§2 and §4 described different phases. Resolved by inverting §4's paragraph:
Phase 7's mechanism is mostly *kept*, each part named, with the gesture carried
by CSS `width`/`height` rather than a transform — argued from the consequence
that a transform leaves layout alone, so the container's extent and the
*(page, fraction)* anchor would be read stale.

Also folded in: gate clause 3 widened to run after a window resize, closing a
hole where a rest ending only on `pointerup` leaves a resized pane soft and
every other clause passes; clause 8 added for OQ-5; `PDFDocumentProxy`
destroyed on re-open; OQ-1 re-derived from the `tauri://` run; §2's path
corrected to `app/dist/pdfjs/`. Round 1's rule-4 → rule-6 "correction" was
itself corrected back: the clause describes the post-cut moment, where rule 4
matches first.

### Round 1 — Phase 1 only — 2026-08-25 — panel of three, fresh — **NOT READY**

**Round 0 (this episode): yes, with a named risk.** Phase 1 produces the
observable — the PDF Typst compiled, on screen, fitted to the pane at every
width — and the observable itself is untouched; only who rasterises it changes.
The challenge is that the pane works today and this spends 1.72 MB of vendored
JavaScript and three phases to replace it; the answer is that the fit-and-
sharpness problem was reported twice in use, its current answer is a workaround
the spec calls temporary, and the same change unblocks two platforms that are
otherwise dead. **The risk that could make it wrong is OQ-3**: the
accessibility regression is real, admitted and unmeasured.

Panel of three — correctness/grounding, scope/YAGNI, exit-gate testability.
All three NOT READY. Seven deduped blocking findings, **all accepted, none
rejected**:

1. §1.1 landed §6.1 step 1 on `mpdf-003` Phase 7 as "shipped work". Phase 7 is
   `shipped: null, reviewed: null` with no round in `mpdf-003`'s record; its
   code is on `main` as prototype. Re-landed on Phase 1's shipped **no bundled
   JavaScript PDF viewer** constraint, with Phase 7 reframed as what the edge
   *names* rather than what makes this a new spec.
2. OQ-2 was `needs-input` in §3 and "resolved in this phase" in §4, with a
   recommendation in place of a decision and no gate clause either way.
   Resolved into three causes keyed to what `refresh` already distinguishes.
3. The specified restore was a raw `scrollTop`, which is wrong precisely when
   the scale changed — the case it was assigned to. Now *(page, fraction)*.
4. "Every line of Phase 7's mechanism goes" would have taken `relines()` with
   `settle()` — the only width-change path that rebuilds Phase 8's line gutter
   — silently, with nothing in the repository able to see it.
5. What the reader sees *during* a width change was unspecified, and the naive
   reading blanks the pane: a `ResizeObserver` firing per frame against 94 ms
   of work, each render cancelled by the next, and a cancelled render on a
   just-resized canvas leaves it cleared.
6. The enabling measurement was taken over `tauri dev`'s
   `http://127.0.0.1:1430` with the **unminified** build — not the file
   vendored, not the scheme shipped. **Re-measured, not reworded**, against
   `cargo run -p md2pdf-app` at `tauri://localhost`, minified, worker real:
   import 29 ms, open 59 ms, page 1 42 ms at dpr 2, six pages 94 ms, 190 text
   items. OQ-5 records that one earlier run stalled fourteen minutes before its
   first render in a non-frontmost window.
7. "The pane's content width" named no element, and the width clause passed by
   construction under `scale = paneWidth / naturalWidth`. Pinned to the scroll
   container's content box, with the clause rewritten to test the ~15 px
   always-show-scrollbars case that can actually fail.

**Rejections worth recording.** Deferring the `CORRECTED` notes and the `cut`
to Phase 2, on the grounds that Phase 1 alone leaves the pane worse than
today: rejected, because the notes describe what Phase 1 itself makes true — a
JavaScript viewer *is* bundled the moment it ships — so deferring them leaves
the corpus stating something false for the interval. And the two Rust doc
comments citing `app/dist/index.html:caretPage`: no action, since OQ-2 case 2
keeps `caretPage` and its anchors.

**Numbers re-derived independently by the panel**: `samples/showcase/showcase.md`
compiles to `/Count 6`; its outline carries six entries; `pdf.min.mjs` +
`pdf.worker.min.mjs` = 1,717,067 B (1.72 MB / 1.64 MiB); `pdfjs-dist` 6.2.108
ships a `LICENSE` and no `NOTICE`; `cargo test --workspace` green on a clean
tree.
