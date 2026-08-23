# Review record — mpdf-007 (`specs/citations_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 3 only — 2026-08-23 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, two non-blocking, both folded. Converged in two rounds.
All three round-1 blockers confirmed resolved **against the files**, and the reviewer
re-derived the new file count from the code rather than from the author's argument: every
site of `Render::images`, `Change::Figure`, `Changed::figures` and `Preview.images` lives
in the three named files and nowhere else, nothing crosses the Tauri boundary — `Status`
carries no image field — and neither renamed identifier collides, `watch.rs` importing no
`Asset` at all and `preview.rs` naming `md2pdf_core::Asset` only fully qualified inside its
test module. **The rename is compiler-enforced on both a struct field and an enum variant**,
so an unnamed site cannot pass silently.

**The route was resolved rather than deferred, and it is a third route neither round-1
option named.** The reviewer offered "ride `Render::images` unrenamed" and "give it a
`Change::Bibliography` of its own"; the author took *one list, renamed* —
`Render::assets`, `Change::Asset`, `Changed::assets`, and no new arm. The first was
refused because it leaves three shipped doc comments false, which is the one price this
project does not pay; the second because `on_change` would gain a branch identical to the
one `changed.figures` already runs and `Changed` a field no reader distinguishes, which is
§5's "don't pre-abstract before there are real consumers". `mpdf-003`'s own split decides
it: the open document against everything the disk supplies, and a bibliography is on the
second side for the reason a figure is. It also makes a *dropped* `bibliography:` key free,
`Preview::compile` replacing the whole list on every compile.

The two non-blocking, both folded:

1. **`classifier` and `on_change` are `Session` methods, not `Preview`'s.** The rewritten
   phase had qualified both to `Preview`, which is where `compile` lives. `rules/desktop.md`
   already spells it `Session::on_change`, so the spec now matches the rule file. The same
   bullet had `on_change` reading `Preview.images`; it reads `Changed::assets`.
2. **The rename lands in three `rules/desktop.md` sections, not two.** Beyond `## The file
   I/O` and `## The watch loop`, `## The session` carries "a figure is a bare recompile …
   read the new figures on the way", a sentence about the renamed arm. The stated cap
   arithmetic already covers the extra lines.

**No finding was rejected, in either round** — as in every round of this spec's episodes.

### Round 1 — Phase 3 only — 2026-08-23 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes, and the phase understated its
own case. It produces the observable, and today a document naming a bibliography does not
merely fail to *update* in the app — it fails to compile there at all, because
`app/src/document.rs:read_assets_with` builds its asset list from `image_paths` alone and
`core/src/lib.rs:collect` raises `MissingBibliography` on every pass. Phase 1 named that
gap; this is the phase that closes it, so the citation channel is reachable from that front
end at all rather than only kept current.

Verdict: `NOT READY`, three blocking, six non-blocking. One generalist reviewer, matching
every prior episode in this corpus.

**The largest finding is that the phase's stated mechanism was backwards.** The scope said
the bibliography "must be **read** [in `read_assets_with`] … or there is nothing for a
watch to re-render", and `read_assets_with`'s output is a `Vec<Asset>` that reaches
`md_to_pdf_with_anchors` and nothing else. What feeds the filter is `Render::images`, built
by a **second and separate** `image_paths` call in `render_with`, travelling
`Preview::compile` → `Preview.images` → `Session::classifier` → `watch::classify`. An
implementer doing exactly what the phase said would ship a document that compiles once and
then never updates. None of `render_with`, `Render`, `Preview::compile` or
`Session::classifier` was named, against §3's requirement that a phase name the functions
it touches.

The other two, and what each really was:

2. **"Two files" was not derivable from the code**, because the route that decides the
   count was left open — and the two available routes differed in scope, in which shipped
   contracts got reworded, and in whether `app/src/preview.rs` was in or out. The identical
   shape to Phase 1's OQ-5 and Phase 2's OQ-1, both forced closed on the ground that a
   phase leaving a design call open forces a guess. Resolved by a third route; see round 2.
3. **The exit gate passed with half the phase unbuilt.** "Changing the bibliography alone
   re-renders" is satisfied by the compile *counter*, and `Session::on_change` calls
   `on_render()` whenever the asset mark is set, a failed compile included — so an
   implementation that publishes the path to the filter and never supplies the bytes goes
   1 → 2 compiles while every one of them errors `MissingBibliography`. The shipped image
   tests it said it matched guard against exactly that by also asserting
   `pdf().unwrap().starts_with(b"%PDF")` and `!is_stale()`; the phase borrowed the harness
   and dropped the teeth. **The same defect the record has now rated blocking three times**
   — Phase 1 round 2, Phase 2 round 1, and here.

Notable among the non-blocking: the phase claimed "round 1 found the draft named only one"
where every recorded round to that point was scoped "Phase 1 only" or "Phase 2 only", and
Phase 1's round-1 entry closes by recording that Phases 2–4 were unjudged; the close-out
named one `rules/desktop.md` section where the change lands in three, and stated no cap
arithmetic against that file's four lines of headroom; the gate dropped
`cargo test --workspace`, which both shipped phases anchor on; and the "watches it beside
the images" verb was wrong, `mpdf-003` having fixed the watch set as one recursive
directory watch with the path list as the *filter* rather than the set.

### Round 2 — Phase 2 only — 2026-08-23 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, four non-blocking, all four folded. Converged in two
rounds. All five round-1 blockers confirmed resolved **against the files**, with every
behaviour the fixes are keyed to re-measured rather than read off the changelog.

**The reviewer's own round-1 finding was corrected by the author, and the reviewer
confirmed the correction.** It had written that the label collision "fires even when the
colliding key is never cited, so the check is against the whole key set" — true, and
incomplete. The full matrix, measured on both sides: a figure `{#smith2020}` in a document
whose bibliography holds `smith2020` **compiles clean**, and so does the same document
citing `[@smith2020]`; the message fires only where a **cross-reference** `[](#smith2020)`
points at the shared label, and then whether or not the key is cited. Consistent with the
message living at `typst-library-0.15.1/src/model/reference.rs:267` — reference
resolution — rather than in `bibliography.rs`. **This moved the check's home twice**: off
`check_name`, which runs at declaration and knows neither, and past `check_references`,
which runs inside `emit` and never sees asset bytes, to beside `collect`. §2's paragraph
carries a dated `CORRECTED` note in §6.1's form.

**OQ-1 resolved rather than deferred, on OQ-5's precedent, and by measurement rather than
argument.** Its cost sentence — "a BibLaTeX/Hayagriva reader this project would then own"
— was measured false: `hayagriva 0.10.1` is already a **direct** dependency of
`typst-library 0.15.1`, declared with no `default-features = false`, so its
`["biblatex", "archive"]` defaults are already on and `biblatex 0.12.0` is already
compiled. The reviewer checked `cargo tree -i hayagriva` inside `web/` and confirmed the
no-new-crate claim holds for the wasm build too, and strengthened the faithfulness claim
past what the author wrote: `typst-library`'s own `decode_library` dispatches a path
bibliography on the extension into `io::from_yaml_str` and `io::from_biblatex_str`, so
`core` mirrors Typst's **code path**, not merely its behaviour. The rejected branch is
recorded with why — it is not impossible, `Names::cited` already carrying every key with
its line, but it means matching a diagnostic's wording and leaves the collision message no
route at all.

The four non-blocking, all folded and all worth their round:

1. **A third failure escapes, so the phase's own count was wrong.** `bibliography: refs.txt`
   over a good Hayagriva file gives "unknown bibliography format (must be .yaml/.yml or
   .bib)" — reachable, no line, because `frontmatter.rs` validates `portable_path` and
   nothing about the extension. "Two remaining failures" became three, the module got a
   third dispatch arm, and gate item 5 got a case that forces it.
2. **A case-folding seam between `core` and Typst.** `decode_library` matches
   `ext.to_lowercase()`; `core/src/emit.rs:extension_of` returns the extension unfolded.
   Measured: `bibliography: refs.YML` compiles clean today and would be neither format to
   a naive `match`. `bytes_match` carries the same shape, so it folded as a clause.
3. **Gate item 3's construction hint was made stale by OQ-1's own resolution** — the
   pattern the loop warns about, a fix introducing a defect. With `core` refusing before
   the compile, Typst's diagnostic ordering no longer participates in anything the gate can
   check. Re-derived: what differs is `Names::cited`'s vector order against line order, via
   the footnote splice, which extends `cited` from a definition's body at the *reference*.
   Measured — reference line 8, body citation line 10, definition's line 12 — the vector is
   `[12, 10]` where the lines are `[10, 12]`. **A document with two plain body citations
   does not discriminate**, so the obvious fixture would have had no teeth.
4. **The earliest-line rule read as scoped to the two new checks**, leaving the ordering
   against `collect`'s own refusals unstated. Widened to one rule over every refusal that
   function can raise, on `collect`'s own recorded argument.

**No finding was rejected, in either round** — as in every round of Phase 1's episode.

### Round 1 — Phase 2 only — 2026-08-23 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes. Phase 2 produces no observable
and says so and argues it, rather than assuming it: `mpdf-001` §2 makes the rejection rule
non-negotiable, and the citation channel is where failures still escape as a raw Typst
diagnostic. Measured against shipped Phase 1 before the round opened, the gap is real and
unchanged — an absent key gives ``typst compilation failed: citation key `nosuchkey` is not
present in the bibliography``: the key, no line, and none of the dialect's own words.

Verdict: `NOT READY`, five blocking, six non-blocking. One generalist reviewer, matching
every prior episode in this corpus.

**The largest finding is that the phase had no scope.** `spec-authoring.md` §3 requires each
phase to name the files and functions it touches, because one phase is one plan-mode pass
from a fresh context; Phase 1's scope named six files with a bullet each and Phase 2 named
**zero**. Under any resolution of OQ-1 the real work was discoverable only by reading code.

The other four, and what each really was:

1. **OQ-1 was open and the phase was scoped as "whatever OQ-1 decides".** The two branches
   differ in dependency footprint, in where the error is raised and in what it can say —
   the identical shape to OQ-5, which Phase 1's round 1 forced closed on the ground that a
   phase leaving it open forces a guess.
2. **The exit gate demanded a line one permitted branch could not produce**, so the gate and
   the scope contradicted each other and an implementer had to guess which to believe.
   `core/src/lib.rs:join` maps each `SourceDiagnostic` to `d.message` and drops every span,
   and there is no markdown↔`main.typ` source map.
3. **The one pointer the phase gave for its second message contradicted the code** —
   `check_name` runs inside the walk, and `emit` never receives asset bytes.
4. **The gate covered one of the phase's two messages**, so the phase could pass it with
   half its scope unbuilt — the same defect round 2 rated blocking on Phase 1, when OQ-5's
   label was assigned to the looks with no look fixture.

Notable among the non-blocking: "a citation is the **first** construct whose failure escapes
as `typst compilation failed: …`" is false — a PNG corrupt past its magic bytes already
does, and `rules/pipeline.md` documents that as a recorded limit; and several absent keys
produce several diagnostics joined by `; ` in Typst's order rather than the document's,
which would have made the error non-deterministic.

### Round 3 — Phase 1 only — 2026-08-22 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, six non-blocking, all folded. Converged **on the cap**,
which is the first time in this corpus a phase has needed all three rounds — and it needed
them because rounds 1 and 2 each found a real blocker rather than because either looped.

Both round-2 blockers confirmed **by construction against the real files**, not against the
changelog. The widened `@`-or-`-@` predicate was re-run over the whole hostile set and every
non-citation row came back byte-identical to `Parser::new_ext`, *offsets included* — and the
reviewer probed two shapes the spec's table does not list, `[x -@k]` and `[-k]`, both
unchanged, establishing that the predicate is tight at **both** ends rather than only the
one the author measured. The look rule was compiled end to end against
`core/assets/template.typ` and `core/assets/press-release.typ`: one markdown heading still
queries one `HeadingElem` in every case, and the article PDF grew 15,236 → 15,902 bytes with
the rule in, which is the label actually being drawn rather than a rule that parsed.

The six non-blocking findings were all folded rather than deferred: the gate said "four
things" and numbered six; gate item 4 needed **two** fixtures, a fixture carrying one
`template:` key, on `tests/fixtures/press_release.md`'s precedent; **both** of `emit.rs`'s
parser sites must take the constructor, or a citation inside a footnote definition would
print literally while the same text in the body cited; the line `MissingBibliography`
reports has to be carried out of `frontmatter::parse`, which is the only thing that sees it;
and the key reaching `label(…)` goes through `core/src/emit.rs:typst_string`, the escaper
that already exists for URLs.

### Round 2 — Phase 1 only — 2026-08-22 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, two blocking, six non-blocking. All four round-1 blockers confirmed
resolved against the files. **Both new blockers were introduced by the round-1 fixes**,
which is the pattern the loop warns about, and both were caused by the same fix.

1. **The scoped callback made `[-@key]` invisible while OQ-3 promised to refuse it.**
   Measured: with the predicate at `@` alone, the reference of `[-@k]` is `-@k`, so the
   callback never fires and the form reaches the page as `\[\-\@k\]` — a refusal the spec
   promised and the parse could not deliver, and by §2's own rule the silent flattening
   `mpdf-001` §2 refuses. Resolved by widening to `@` or `-@` and re-measuring the whole
   set; `[see @k]` and `[a@b.com]` fall outside it and became a **stated** boundary in §1.2
   rather than a discovered one.
2. **OQ-5's resolution assigned the reference-list label to the looks, and no phase touched
   a look file.** `title: none` removes Typst's own heading, so as scoped the list would
   have arrived unlabelled — and with OQ-5 closed, nothing would ever have forced one. The
   scope compounded it by saying "six files" over four bullets. Resolved by scoping
   `core/assets/template.typ` and `core/assets/press-release.typ`, with a gate item.

Of the six non-blocking, three were load-bearing: `[@k][]` parses as `CollapsedUnknown`
rather than `ShortcutUnknown`, and one match arm alone would emit `#link("@k")[@k]` — a
wrong document where today there is literal text; the callback's returned destination was
unspecified and is observable twice, through the existing empty-destination error and
through `md_to_html`; and `MissingBibliography` implied a line `Frontmatter` does not carry.

### Round 1 — Phase 1 only — 2026-08-22 — fresh clean-context reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes. Phase 1 produces the project's
observable and produces the one instance of it `md2pdf` has never been able to make — a
body mark resolving into a reference list — and it is the smallest slice that produces one
at all, a bibliography channel without citations rendering an empty list and citations
without a bibliography having nothing to resolve. It was asked for directly, so the "was it
wanted" half is answered by the request rather than inferred.

Verdict: `NOT READY`, four blocking, six non-blocking. One generalist reviewer, matching
every prior episode in this corpus. **No finding was rejected, in any round.**

**The first blocker was fatal to the phase's central mechanism.** The draft said the emitter
"maps a `[@key]` shortcut link", and measured under `core/src/emit.rs:options`,
`See [@smith2020] ok.` produces five `Event::Text` runs and **no `Tag::Link` at all** — a
CommonMark shortcut reference link is only a link when a matching definition exists, and a
citation never has one. Resolved with `Parser::new_with_broken_link_callback` over a scoped
callback, which is also what preserves `mpdf-006` Phase 4's "one parse, one set of options"
claim once `md_to_html` takes the same constructor.

The other three, and what each really was:

2. **`#bibliography`'s default `title: auto` realises a real `HeadingElem`**, so the walk's
   heading count disagrees with the compiled document's and `core/src/lib.rs:anchors_from`
   withdraws **every** anchor on a mismatch — silently taking `mpdf-003` Phase 6's scroll
   sync and `web/src/lib.rs:anchors` with it. OQ-5 was open and attached to no phase while
   Phase 1 could not emit the call without deciding it. Resolved by `title: none`, and OQ-5
   was **resolved rather than deferred** because a phase that left it open forces a guess.
3. **§1.2's byte-identical promise contradicted the unconditional mapping**, and the case
   between them failed with an unmapped Typst diagnostic carrying neither construct nor
   line. Resolved toward the dialect's own ethos: the mapping is unconditional and a
   citation with no bibliography is a **named refusal**, with §1.2 narrowed to match.
4. **The gate could not fail for the reason it was given.** Because the census is true — zero
   `[@` anywhere in the corpus — the byte-identical assertion was satisfied by the status quo
   whichever way the phase was built. It has teeth against exactly one thing, an unscoped
   callback moving `tests/golden/hostile.typ`, whose fixture carries
   `an [ open bracket, a ] close bracket`; the gate now names that fixture, and adds "no
   golden file is edited", the goldens being hand-written `include_str!` constants with no
   walking harness.

Notable among the non-blocking: `#cite(<DBLP:books/lib/Knuth86a>)` fails to *parse*, so the
emitted form became `#cite(label("key"))` — a deliberate divergence from `mpdf-005`'s
`#ref(<name>)`, on the argument that a figure name is authored inside this dialect and can
be constrained while a citation key is authored in a file the author often did not write.

**One process deviation is recorded rather than resolved.** `spec-authoring.md` §7 says a
spec with no shipped phase takes one **document-wide** round; this episode was scoped to
Phase 1 by explicit instruction. Phases 2–4 are therefore unjudged, and the author escalated
the deviation to the human rather than silently widening the round.
