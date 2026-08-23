# Review record — mpdf-007 (`specs/citations_spec.md`)

Append-only. One heading per round, newest first.

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
