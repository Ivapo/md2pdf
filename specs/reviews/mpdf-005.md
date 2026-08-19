# Review record — mpdf-005 (`specs/captions_and_references_spec.md`)

Append-only. One heading per round, newest first.

### Round 3 — Phase 6 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, nothing newly found. Converged **inside the cap**,
which is the first time in this spec's record that a phase has — Phases 4 and 5
each converged on the third round because the round that would have escalated was
the one that cleared, and this one had zero blocking findings from round 1 onward.

Both round-2 points confirmed in the files. The reviewer also declined the one
thing the changelog offered to escalate: a first-member clause in `README.md` and
`rules/pipeline.md` "would add information, not repair a falsehood", since neither
document asserts anything about a mixed group, so §6.1's rule that a phase corrects
the prose it makes misleading does not reach them. Recorded because the author
raised it as a possible blocker and the answer is a boundary worth keeping.

### Round 2 — Phase 6 only — 2026-08-18 — same reviewer, resumed — **READY**

Verdict: `READY`, zero blocking, one non-blocking — and **the non-blocking one
falsified a claim the author had just written into the spec**, which is the round's
whole value.

Fix 5 of round 1 had bounded the inherited-property argument with "the property was
written against the emitter". It was not. `mpdf-004` Phase 3 states it as "no
document's typeset output changes unless its author asks, **because the default name
is the one that numbers nothing**", and that phase's own scope changes
`core/assets/template.typ` and `core/assets/press-release.typ` as well as the
emitter. So a look **has** moved under this property before, and kept it by having a
key to condition on that left its new rule inert until the author set
`equations: numbered`. The bound is now the narrower true one: this phase has no such
key and cannot get one, because OQ-2 refused a frontmatter key for the three `figure`
kinds and OQ-9 priced the look-side per-kind alternative. Folded in as a correction
with the falsified draft quoted, rather than swapped silently.

Also folded: gate (3) said `tests/fixtures/groups.md` needs "no new document at all,
in either look", where reading the second look still wants gate (1)'s
`template: press-release` copy — now "no new **content**".

The round verified the other four round-1 fixes in the files and endorsed the one
judgement call among them: qualifying rather than dropping `rules/pipeline.md`'s
"`figure` centres its body where a bare block sits flush left" preserves two readings
a deletion would lose — the clause stays true for `image` and `table` in every look,
and for `raw` in any look without the new rule.

### Round 1 — Phase 6 only — 2026-08-18 — fresh clean-room reviewer with repo access — **READY**

Verdict: `READY`, zero blocking, five non-blocking — all five accepted and folded,
none rejected, none deferred.

**Round 0 — is this the right thing to build at all? Yes.** It produces the
observable directly: a PDF whose captioned code block stands flush left, where today
the same code sits in two places depending only on whether it carries a number. It is
the right one because the defect was found by reading a real page rather than
inferred, and the fix takes something off the page rather than adding a capability
nobody asked for.

**The round reproduced the phase end to end rather than trusting its measurements** —
copied the tree, added the rule to both looks, rebuilt, and rendered before/after in
both looks plus a two-column variant. Every load-bearing claim reproduced: the
captioned/uncaptioned split before, the return to the prose's left edge after, image
and table still centred, a listings group left and an images group centred, the suite
green with no golden and no `core/src` file moving.

The sharpest finding was a measurement the author had generalised from one order:
**a mixed group takes its first member's kind**, not "Figure". Image-then-listing
reads *Figure* and stays centred; listing-then-image reads *Listing* and goes left,
dragging its image with it. Re-measured by the author in both orders before folding.
Nothing in the build turns on it — no gate covers a mixed group — but "a mixed group
stays centred" is false half the time and was about to become the sentence a later
reader carried away.

The other four, all folded: "the emitter has never written an alignment" overreached,
since `core/src/emit.rs:table_call` writes `align:` from the GFM delimiter row — the
claim the seam needs is that it writes no *page* alignment for a block; the
reconciliation named one stale sentence in `rules/pipeline.md` where three go stale;
gates (1)–(3) named no documents, where Phase 2 gate (8) had named its, and
`tests/fixtures/captioned_blocks.md` has a captioned listing with no uncaptioned twin
so a scratch document is needed either way, while `tests/fixtures/groups.md` already
carries gate (3) whole; and §6.1's step-1 argument did not name the inherited
`mpdf-004` property it bends, which round 2 then corrected again.

### Round 3 — Phase 5 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Converged **at the cap** for the second time in
this spec's record, and for the same structural reason Phase 4 did: the round that
would have escalated is the one that cleared.

Round 2's blocker confirmed resolved — the two clauses that assigned the tight div
opposite outcomes are gone, and the reviewer found nothing else in the phase
reading on it either way. It also answered both questions the changelog asked, and
one answer removed a doubt the author had planted about their own gate: **`- :::` /
image / `:::` at column zero is a shape the parser really produces**, the closer
landing one frame shallower than the opener, so gate (6)'s third placement has
something to fire on rather than being a case over an impossible input.

Four non-blocking, all folded, and **one of them is the sharpest finding of the
episode even though it was not classified blocking.** `core/src/emit.rs:Figure::live`
tests `bufs.len() == self.depth`, and depth is not frame identity: `- :::` / image /
`- :::` puts both delimiters at **depth 2 in different frames**, so a depth-only
implementation accepts the pair and truncates a frame it never opened — and all
three of gate (6)'s stated placements pass under exactly that bug. `Figure` is safe
from it only because its content check catches the rest, which a group has no
equivalent of. The scope now says "the same frame, not merely the same depth" with
the measurement behind it, and gate (6) carries a fourth placement whose whole job
is to fail that implementation.

The other three: the backslash escape does not reach the reservation — `\:::` arrives
with first text `:::` because pulldown-cmark folds the escape into the same run, so
an author escaping the marker is refused on their own line, which is recorded as the
safe direction rather than fixed; §2's REOPENED bullet needs a dated clause at
close-out, since it records what the tight div emits *today* and this phase changes
it, which is the situation §2's own 2026-08-17 note was written for; and the gate
preamble was still crediting round 1 with two cases the later rounds rewrote again.

### Round 2 — Phase 5 only — 2026-08-18 — same reviewer, resumed — **NOT READY**

Verdict: `NOT READY`, one blocking — **newly introduced by round 1's own fix**, the
second time this corpus has recorded the loop doing that to itself, and this time it
was predicted in the changelog and happened anyway.

The three round-1 blockers all confirmed resolved, one of them under test: the
reviewer injected the OQ-13 rule into both looks and ran `cargo test --workspace` —
129 passing, `every_bundled_template_meets_the_call_contract` still at five, the
article's pagination unmoved — then compiled a two-image group through the
press-release look and read *"Figure 1 — Two images"* with that look's own separator.
So "the looks move and the contract does not" is measured rather than argued.

**The blocker: the reservation's carve-out and its new refusal assigned the tight div
opposite outcomes.** `:::` / image / `:::` with no blank lines is **one paragraph**, so
it both begins `:::` while being no valid opener (an error) and stands as a `:::`
"among other text in a paragraph" (untouched) — and the two guesses differ on exactly
the spelling §2 records an author reaching for first. Resolved by stating the position
exactly — the reservation reaches a `:::` that is **the first text of its paragraph**
and nothing else — and by taking the consequence rather than leaving it implicit: the
tight div becomes a **named error**, which is the strongest thing the reservation buys,
since that shape fails silently today.

Five non-blocking, all accepted: a sentence round 1's fix was supposed to have killed
was still standing four paragraphs from the paragraph that disowned it; the `columns`
paragraph still announced OQ-13 as blocking the phase five paragraphs above OQ-13's
resolution; **gate (8)'s needle was unnamed and the obvious one passes vacuously** —
both looks already carry `show figure: set block(…)`, so the gate now names
`set grid(gutter:` and says it is a test of its own rather than an extension of one
whose name would stop describing it; the reserved position left one guess, a group
opened inside a list item, which the scope now closes; and gate (7) said "fenced code
block" where `core/src/emit.rs:step` serves both block kinds from one arm.

### Round 1 — Phase 5 only — 2026-08-18 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0, asked once for this episode and answered by the author against §1: **yes.**
The phase produces the observable, and produces a page the dialect cannot express at
all — two figures side by side under one caption and one number, where `: ` attaches
only to the construct immediately above it. It is the right one because the group
takes one caption, one number and one name, which makes it a captioning want with
arrangement as the means, and because it has a stated consumer. **Recorded rather than
glossed: the justification shifted during drafting.** The motivation that raised it —
source legibility — was falsified by measurement, since a div written tight is one
paragraph and blank lines stay structural inside it, so the environment is *more*
vertical space than the marker. The phase now rests on the capability the requester
had listed as a "later we could even", and that is the kind of shift round 0 exists to
notice.

Verdict: `NOT READY`, three blocking and seven non-blocking. All ten adjudicated, all
ten accepted, none rejected or deferred.

**Blocker 1: the marker rule contradicted itself.** The scope opened a group on the
first `:::` met and refused an unclosed one, while the same scope and gate (7) demanded
a lone `:::` reach the page as prose — under the opener rule there is no such thing,
so gates (6) and (7) could not both pass. The reviewer was right that the `: ` analogy
does not carry: `: ` has an inert position and a group can begin anywhere. Resolved by
making the rule honest rather than patching the gate — `:::` at the start of a
paragraph is **reserved**, the first marker in this dialect that is, licensed by the
census and bounded by being positional.

**Blocker 2: OQ-13 was code-answerable, said it blocked the phase, and the scope and
gate asserted the opposite of its only viable answer.** The reviewer measured both
halves: Typst's default grid gutter is **0**, so two members touch, and a look-side
`show figure: set grid(gutter: …)` separates them while crossing nothing. So both look
files change and the contract stays at five — OQ-3's answer one construct further along
— and the phase stopped hedging its own scope and gate on an open question.

**Blocker 3: an empty group reached Typst.** `:::` / `: A caption.` / `:::` satisfies
every stated rule and emits `grid(columns: 0)`, measured failing with `number must be
positive`, naming no line and no construct the author would recognise — the labelless
failure §2's check-what-the-author-wrote decision exists to prevent, reached from
markdown every other rule accepts. A sixth refusal, taking the list from five to seven
with round 2's addition.

Seven non-blocking, all folded. The `Figure`/`body` reuse claim was **confirmed sound**
— `body` holds the bare call for all three constructs and is stale only after
`captioned`, which members never are — but the reviewer measured that the group shape
*already works today* and splices the caption onto the **second image**, so the closer
must suppress a shipped path rather than extend one; that, the caption being held past
`End(TagEnd::Paragraph)`, and the buffer removal are now in the scope. §2's append-only
claim gains its second exception and its "nowhere else in the dialect" gains one
position, both as dated notes. §4's preamble was stale at four phases. Near-miss
markers (`::::`, `::: two words`) were undecided and are now errors. And the
unclosed-group check had no stated home, with groups inside footnote definitions
unaddressed — it now fires where either walk ends.

The reviewer also re-derived every Typst measurement §2 keys the phase to, against the
pinned 0.15.1: a `grid` of images reads **Figure 1**, of tables **Table 1**, of `raw`
**Listing 1**, all with no `kind:` argument; the group takes **one** count; the
reference resolves to the group; two full-size images in `grid(columns: 2)` scale to
fit rather than overflow. The `:::` census and the tight-div measurement both reproduce
exactly, and `Walk.para` and the standalone test are genuinely untouched inside a group.

### Round 3 — Phase 4 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Converged at the cap, which is worth noting: this
episode used all three rounds, and the round that would have escalated is the one
that cleared.

Round 2's blocker confirmed resolved, and the reviewer answered the two questions
the changelog asked it. **`caption.is_some()` is sufficient, for a stronger reason
than the author gave.** `core/src/emit.rs:caption_name` has exactly one caller,
`core/src/emit.rs:splice_caption`, so a `{#…}` group has meaning in exactly one
place in the dialect — and a display span plus a group was run through all five
frame pushes (item, block quote, table cell, caption, link) and through alt text.
All are prose today except the caption, so the new rule reassigns nothing
anywhere else; alt text never reaches the display arm at all, because the alt
capture sits above the main match and handles `DisplayMath` itself. **And the
guard being a `Walk` field rather than a depth test is what covers the one shape
a depth test would miss: a display span nested inside a link inside a caption.**

**Gate (5a) pins its property rather than passing for free**, which is the failure
gate (5) had in round 1. The no-guard implementation genuinely diverges — it emits
`caption: [See $ x = 1 $ <fig:one>]` with nothing after the closing paren, where
the shipped binary emits `#figure(…, caption: [See $ x = 1 $]) <fig:one>`, and
`[](#fig:one)` then resolves to the equation rather than the figure. The case
fails a wrong implementation on the bytes and on the reference.

Two non-blocking trivia, both folded rather than deferred, because §7's own bar is
that a gate keyed to a literal no stated method reproduces passes for the wrong
reason. The scope printed `: See $$x$$ {#fig:one}` beside the literal
`caption: [See $ x = 1 $]`, which came from a probe using `$$x = 1$$`; the source
line is corrected in both places and the literal re-derived against the built
binary. And the gate preamble's arithmetic was off by one — it said "six" of ten
plus three named — so it no longer counts.

### Round 2 — Phase 4 only — 2026-08-18 — same reviewer, resumed — **NOT READY**

Verdict: `NOT READY`, one blocking finding — **newly introduced by round 1's own
fold**, which is the pattern the loop warns about and the first time this corpus
has recorded it happening.

Round 1's blocker confirmed resolved: gate (4) names all three goldens, the
reviewer re-swept and found those three and no others, and gates (4) and (6) now
agree. The reviewer also confirmed the dropped trailing-separator allowance does
real work — `$$\nw = 4\n$$\n{#eq:nextline}` emits `$ w = 4 $\n{\#eq:nextline}`,
so the `SoftBreak`'s newline puts the recorded span off the end of the frame and
the group stays prose. Adjacency is enforced without a "next event" flag.

**The blocker: the scope claimed a display span inside a caption was refused by
the record's own liveness test, and it is not.** The caption's marker arm pushes
its `bufs` frame *before* anything later in that paragraph is written, so the
span records at that deeper frame and is spent at the same deeper frame — the
depth check passes — and nothing stands between the span and the adjacent text,
so the content check passes too. Both hold, the label is spent on the equation,
and the figure loses the name it has today while the run never reaches
`Caption.text`. Measured against the shipped Phase 3 binary:
`: See $$x = 1$$ {#fig:one}` beneath an image emits
`#figure(image(…), caption: [See $ x = 1 $]) <fig:one>` today, so **the name is
the figure's** and the phase as scoped would have moved it silently. The reviewer
noted this is unlike Phase 3's link-frame argument, which holds only because the
`Figure` record is made one frame *shallower* than where a marker can fire.

Resolved by naming an actual guard — the display arm records nothing while a
caption is open — by correcting the false claim in place rather than deleting it,
since it is the reasoning a later reader will repeat, and by adding gate (5a).

### Round 1 — Phase 4 only — 2026-08-18 — fresh clean-room reviewer with repo access — **NOT READY**

Round 0, asked once for this episode and answered by the author against §1:
**yes.** Phase 4 produces the observable — a PDF whose "as Equation 1 shows" stays
true after an equation is inserted above it — which is §1's promise for everything
a document shows, over the one construct Phase 3 could not keep it for. It is the
right one because `mpdf-004` Phase 3 already tells authors they may number their
formulas so a document can refer to them, while leaving them no way to point at
one; this closes the spec's own stated goal rather than opening a new want.

Verdict: `NOT READY`, one blocking and nine non-blocking. All ten adjudicated,
nine accepted, one remedy rejected, one recorded as a limit.

**The blocker: gate (4)'s blast radius was wrong, and the file it omitted was the
load-bearing one.** The draft said two shipped goldens carry an unnamed display
equation. Three do — `tests/golden/display_math.typ`,
`tests/golden/numbered_equations.typ` and `tests/golden/cross_references.typ` —
and the third is the one Phase 3 shipped four commits earlier, which is **the only
shipped golden where a display equation stands next to a `: ` marker paragraph**,
the exact interaction this phase touches. It is therefore what the phase's own
gate (6) rests on, so the gate excluded the file its neighbour depended on.
`samples/article.md` carries two further display equations and no golden; it is
protected by `core/tests/golden_test.rs:the_articles_last_heading_is_not_on_the_first_page`.

Non-blocking, all accepted and folded: gate (5) tested only trailing text, which
anything shaped like `core/src/emit.rs:caption_name` refuses anyway, so the case
passed for the wrong reason — the leading-text shape `$$…$$ see {#eq:lead}` was
measured emitting prose today and would take a label under that reuse, and gate
(5) now carries both sides. The scope's blanket "nothing else in Phase 3's
machinery changes" contradicted its own later sentence and is replaced by explicit
reused/extended lists. "Entirely a `{#name}` group" was falsified by the phase's
own measured literal, which carries a leading space. `core/src/emit.rs:caption_name`
was never cited despite being the symbol an implementer must reach for, and its
contract — group at the *end* — is not this phase's. The ordering between the two
after-the-walk refusals was unstated. Reconciliation was unnamed, and one of its
three artifacts is not free: `samples/article.md` says in its own prose that there
is no way to label a formula, and that sample's pagination is pinned by a test.

**One remedy rejected.** The reviewer proposed the `pending` slot for the
adjacency problem. Not taken: rounds 2 to 4 of Phase 1 each measured a different
construct broken by holding a call across an event, and nothing here needs a hold,
because the equation is already written when the name arrives. The accepted shape
is a record the display arm leaves and the Text arm spends where it is live —
`core/src/emit.rs:Figure::live`'s argument with the trailing-separator allowance
dropped.

**One limit recorded rather than resolved.** The reviewer could not independently
re-derive three Typst-side literals: no `typst` binary is installed and the
dialect admits no raw-Typst injection. The author measured them by temporarily
injecting probe Typst into `core/assets/template.typ` and compiling through the
CLI, then reverting — the technique earlier phases used, since a look file is
arbitrary Typst. Under `equations: numbered` a labelled `$ x = 1 $` needs no other
change: the page read `(1)`, `(2)`, `(3)` over three equations and the references
read *Equation 1* and *Equation 3*. Under the `plain` default the same source
failed with `cannot reference equation without numbering` verbatim. `it.label` in
a `show math.equation` rule failed with `equation does not have field "label"`,
while `it.at("label", default: none)` returned `<eq:a>`, `none`, `<eq:b>` — which
is what let OQ-4 measure, and reject, the seam-preserving alternative.

The reviewer independently confirmed both looks answer the key with a **`set`**
rule (`core/assets/template.typ` and `core/assets/press-release.typ`), which is
the code-side premise that makes OQ-4's rejection of the `show`-rule shape sound,
and re-derived OQ-10's parser measurement and the empty collision census for `{`
after a `$$`.

### Round 3 — Phase 3 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking. Run because §7.4 makes folding *any* change a
trigger, and two of round 2's three folds touched the gate — the loop's own
warning is that a fix can introduce a blocker.

Both gate literals were re-derived against the shipped binary rather than read.
A fixture line `: The conversion pipeline.` already emits
`caption: [The conversion pipeline.]` verbatim today — the trailing `.` is
unescaped because `line_is_all_digits` is false, and `splice_caption` trims — so
once the phase drops the `{#name}` group the gate's exact string is what the
stated method produces. **Gate (1a) was confirmed to be a real assertion rather
than a restatement of (1)'s compile**: the failure it catches is the fixture
never carrying a sentence-final reference at all.

OQ-11's classification was re-derived and holds, including that both its shapes
are genuinely reachable from markdown — `[](#fig:pipeline)(top)` emits `…[](top)`
today, which becomes the chained call.

One non-blocking prose note, accepted and folded: OQ-11 had claimed its two
survivors are "loud where the marker form's plural was silent", which does not
hold — the plural failed at the compile too. What actually distinguishes them is
that they need an adjacency ordinary prose does not produce, and that is what the
entry now says. The two reasons carrying the argument, both measured, are
unchanged.

**Converged.** Phase 3's `reviewed` is set to 2026-08-18; `status` was already
`accepted`.

### Round 2 — Phase 3 only — 2026-08-18 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round two. Round 1's blocker was verified
resolved in the files rather than in the changelog, and the reviewer **strengthened
the fix rather than merely confirming it**: reading `typst-syntax-0.15.1`'s lexer,
entry to a label is gated on `is_id_continue` in *markup* and in *code* alike, so
within the declared set "first character not `:` and not `.`" leaves
`{letters, digits, -, _}` — every one `is_id_continue`. The rule is not merely
sufficient, it is **exactly equivalent to what both lexers require**.

**The round's best catch is what it checked about the `#ref` switch rather than
what it found wrong with it.** `typst-eval-0.15.1` evaluates `ast::Ref` to
`RefElem::new(target)` — the same element `ref()` constructs — so §2's measurement
table, OQ-7's footnote measurement and OQ-4's and OQ-9's compile failures, every
one of them taken with `@`, still underwrite the phase after the spelling changed.
A switch that had quietly invalidated three earlier measurements would have been
the expensive kind of correct.

It also closed the reserved-name question completely: `core/src/emit.rs` writes
labels in exactly two places, both `fn-{number}`, and neither look emits one at
all — so `fn-` and digits is the whole reservation rather than a guess at it.

Three non-blocking, all accepted and folded. The scope claimed the function form
is not boundary-sensitive, and two shapes survive it — `#ref(<x>)(a)` parses as a
chained call and `#ref(<x>).Then` as a field access — so the claim is now "very
nearly not" and **OQ-11** carries the residue with its price. Gate (1)'s "no `{`
in any caption" needle was unwritable in this repo's negative-needle idiom,
because the same fixture carries a named listing whose `raw` string holds braces;
it is now a positive assertion of the exact caption string. And two literals in
the newly folded text did not reproduce: a Typst message quoted from a stand-in
probe, and a citation of `mpdf-001` §2 where the sentence actually lives in that
spec's §4 Phase 7 — checked at `specs/md_to_pdf_pipeline_spec.md` line 676, inside
the block opening at 627.

### Round 1 — Phase 3 only — 2026-08-17 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — one phase, per §7.0).** *Does this phase produce the
observable, and is it the right one?* Yes, and this is the phase whose observable
is the *motivating* one: §1's whole argument is that "the diagram above" goes
quietly wrong when something is inserted, and a reference Typst renumbers is the
fix — Phases 1 and 2 built the carrier. **The risk recorded rather than
dismissed:** the narrowing takes equations out of what §1 promised, which is a
real reduction; §1's own sketch is a figure reference, and Phase 4 carries the
remainder with named blockers rather than dropping it.

Verdict: `NOT READY`. One blocking, eight non-blocking; all nine accepted, none
rejected.

**The blocker was that the declared name character set is not closed under the
label→reference round-trip, and the author's re-measurement changed the shape of
the fix.** The reviewer named two halves. The first reproduced exactly: a name
opening with `:` or `.` is **not a label at all**, and `#figure(…) <:foo>`
typesets the literal text `<:foo>` on the page raising nothing — the silent drop
§2 exists to refuse, reached through a name the dialect would have accepted. The
second — that a trailing `.` or `:` is silently dropped from a reference — was
measured to be a property of the **`@name` marker only**, not of labels. So
instead of the trailing rule the reviewer proposed, the phase now writes
**`#ref(<name>)`, the function form**, on `mpdf-001`'s own argument for `#emph[…]`
over `_…_`: every shape that is a label at all round-trips through `#ref(<…>)`
onto the right figure, `fig:`, `fig.` and `fig..one` included. That single change
also closed the reviewer's adjacency finding outright — `@figAs` fails with
``label `<figAs>` does not exist in the document``, where a call cannot swallow
what follows it.

One correction to the round's own report, recorded so a later reader does not
re-derive it: `_foo` **is** a valid label. The author's first probe failed on it
because the probe's caption `[_foo]` opened emphasis — a defect in the instrument,
not in Typst.

The eight non-blocking, all folded: emptiness is not knowable at
`Event::Start(Tag::Link)`, so a reference opens a `bufs` frame and settles at the
`End` arm, with the Start arm's two existing refusals untouched; the `{#name}`
group must leave the caption and is not a substring removal, since `escape_into`
has made it `{\#fig\-two}`; a named splice must carry the label into
`Figure.written` or Phase 1's second-caption refusal silently stops firing, which
Phase 2's unnamed cases cannot catch — now gate (3a); `fn-N` is a namespace the
emitter already owns, now a reserved name; the post-walk check falsifies a
document-order property two other artifacts state without qualification, now
stated as the one exception with both artifacts named for reconciliation and §2's
"pre-pass" sentence corrected in place; the undeclared reference reported is the
one on the earliest line, because the obvious container is a set; and
`[ ](#name)` is decided — empty means empty, `is_empty()` and not `trim()`.

Grounding confirmed rather than corrected, so a later round need not re-verify:
every `file:symbol` the phase cites is real and says what the phase says, both
looks leave `ref` and `figure(numbering:)` alone, and **OQ-8's §6.1 step-1 working
survives** — the census holds, no consumer of `#`-fragment links exists in `app/`,
`cli/` or `web/`, and `[](#fig:one)` really does emit `#link("#fig:one")[]` and
put nothing on the page.

### Round 2 — Phase 2 only — 2026-08-17 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round two. **Converged**, and Phase 2's
`reviewed` is set to 2026-08-17. `status` was already `accepted`.

Round 1's blocker was verified resolved in the files rather than in the
changelog. The reviewer re-derived the claim the fix rests on — that both looks'
caption rules are kind-agnostic — and confirmed it: `set figure.caption(…)`,
`show figure: set block(…)` and `show figure.caption: set text(…)` in each file,
not one `.where(kind: …)` among them, and Typst's `figure` default
`numbering: "1"` overridden nowhere. So a captioned table and a captioned
listing are styled and numbered the day the emitter emits one, with zero look
edits, and gate (7)'s diff check makes that enforceable rather than trusted.

**The reviewer also named the property that makes the fix safe, which the author
had not:** the numbering measurement is load-bearing as a *reason* and not as a
*dependency*. If it were wrong, Phase 2's work would be unchanged, because
nothing in the phase needs a look edit either way. The decision is keyed to the
right artifact.

Five non-blocking, all accepted; none rejected this round.

**One is a Phase 3 correctness finding that this round's own measurement
surfaced, and it is the round's best catch.** Typst's "cannot reference X
without numbering" is generic over the element, not special to `figure`. Both
looks set `math.equation(numbering: … else { none })` and `mpdf-004` Phase 3
made `plain` the frontmatter default — so **the default path is the unnumbered
one**, and Phase 3's "a reference becomes `@name` … equations included" would
fail the compile for every document that did not opt in. Measured by the author
before folding it in, twice against the pinned 0.15.1: `equations: plain` fails
with ``cannot reference equation without numbering``; `equations: numbered`
compiles. It also explains why §2's table missed it — that probe ran with
`mpdf-004` Phase 3's numbering rule active, which is the non-default. Recorded
against OQ-4 with three shapes for Phase 3 to weigh. Not Phase 2's problem:
Phase 2 ships no reference and touches no look.

The other four: gate (8)'s third bullet claimed to catch the separator asymmetry
and does not — that bug glues `#figure(` to the line above and Typst breaks the
block out anyway, so the symptom is spacing, and gate (1)'s golden and gate (5)
net it byte-exactly; the read stays, its rationale is corrected. Gate (1) still
said "the one thing an implementer trips on" after the scope moved to two. OQ-9
carried no §4 classification. And `last_updated` was three revisions stale.

### Round 1 — Phase 2 only — 2026-08-17 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — one appended phase, per §7.0).** *Does this phase
produce the observable, and is it the right one?* Yes: the typeset PDF, with
tables and code blocks carrying captions and their own counters. **The risk
recorded rather than dismissed:** the conversation that prompted this spec asked
about *labels*, and Phase 2 widens the carrier to two more constructs while the
motivating function still lands in Phase 3 — the same flag Phase 1's round 0
raised. The ordering is right rather than accidental: Phase 3 makes `@name` read
"Table 1", which needs a table to be a figure at all, so Phase 2 is a
prerequisite rather than a detour; and Phase 3 is blocked by OQ-4, OQ-7 and OQ-8
while Phase 2 was unblocked by OQ-2's resolution the same day.

Verdict: `NOT READY`. One blocking, eight non-blocking; seven accepted, one
rejected.

**The blocker was the phase not being self-contained on its look half, and it
was the author's own defect twice over.** Phase 2's scope named `emit.rs` and
three things it leaves untouched but never said whether either `.typ` file
changes, while gate (8) handed the phase an unmade design call outright —
"decided in this phase's round". No `OQ-N` carried it, which is §4's named
failure mode. **The two halves of gate (8) could not both hold**: it demanded
"three supplements and three counters at 1" while leaving open a suppression
that would falsify exactly that on the press-release PDF. OQ-2's own text pulled
both ways — "it belongs in `press-release.typ`, where this resolution puts it"
against "whether that look should suppress is left to Phase 2".

**Resolved by measurement rather than by taste, and the measurement inverted the
question.** A figure whose numbering a look has suppressed **cannot be
referenced at all**: Typst fails with ``cannot reference figure without
numbering``, isolated by removing only the suppression and changing nothing
else. So suppression in a look breaks every Phase 3 cross-reference to that
kind, in documents whose authors chose neither. Neither look changes and both
keep all three kinds numbered; gate (7) asserts it as a diff; **OQ-9** now
carries the parked want with its price, and records that the cheapest of its
three shapes is a look hiding the supplement typographically while numbering
stays on, which leaves `ref` working.

**Grounding confirmed rather than corrected, so a later round need not
re-verify.** The reviewer built and ran the shipped binary to settle the one
question reasoning alone would not: after a table, a fenced block and an
indented block alike, a following image-only paragraph still emits the
*standalone* form — the same `*para == Some(top(bufs).len())` predicate the
caption branch tests — and the separator is exactly `\n\n` in every case. So
`Figure::live`'s three conditions and `splice_caption`'s truncate carry over
unchanged, and Phase 1's mechanism does extend as the phase assumes. Every
number re-derived: one golden each for `#table(` and `#raw(block: true`;
`samples/article.md` carries a table, a fenced block **and an indented one**,
captioning none; `samples/press-release.md` carries an uncaptioned table;
`tests/fixtures/captions.md` carries neither construct, so Phase 1's golden
cannot move.

Six non-blocking accepted and folded: the separator-ownership asymmetry, which
Phase 1 never met and which would otherwise glue `#figure(` to the line above;
`table_call` named as the site where a pure formatter can record neither offset
nor depth, so the scope now names both `End` arms; "the fenced-block arm"
contradicting the paragraph below it; gate (7) understating the article's three
constructs and over-claiming `samples/press-release.md`, which no Rust test
compiles; **the centring read**, since a wrapped `raw` block is centred where
every code block in the corpus sits flush left and no gate could see it; and the
gate (1)/(3) fixture ambiguity, now one fixture carrying all three kinds. Two
stale forward-looking clauses in OQ-3 and §2 took dated closing notes.

**One rejected**, recorded so it is not re-raised: the refusal message reads
"second caption for one figure" over a table and a code block. A captioned table
*is* a Typst `figure`, so the message names the element the emitter writes; gate
(4) asks for the construct and the line, both unchanged; and the string is
asserted verbatim by `core/tests/golden_test.rs:each_caption_refusal_names_its_construct_and_its_line`,
so changing it moves shipped work for wording. If it is worth changing, it is
worth changing for all kinds at once, which is not this phase.

### Round 5 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, at round five — **two rounds past §7.6's cap, each
authorised by the human as its own decision.**

Round 4's blocker was not patched. **Three rounds had found three different
constructs broken by one cause** — holding `Walk.pending` a paragraph longer than
the machinery around it was written for: `Walk::finish`'s hardcoded inline form,
the second `finish()` site in `core/src/emit.rs:collect_definitions`, and
`Walk.para`'s offset. That is evidence about the design, so the mechanism was
replaced rather than the symptom fixed, and the rejected deferral is recorded in
§2 with all three failures so a later reader does not propose it again.

**The replacement looks back instead of holding forward**: `write_image` records
where a standalone call began, and a `: ` paragraph splices the recorded region
into `#figure(…)`. The reviewer tested the load-bearing claim — that the flush
timing does not change *at all*, making the three symptoms unreachable rather
than fixed — and confirmed it at each of the three sites.

**It also verified the property the look-behind rests on, which the author had
asserted:** every write into a `bufs` frame in `core/src/emit.rs` is an append —
no `truncate`, `insert`, `replace_range`, `drain` or `remove` touches one — so a
recorded offset cannot shift while its frame is live. That is what makes "verify
at use" sound rather than optimistic, and it is why gates (2) and (7) are now
structurally true instead of hopeful: the record is inert unless a `: ` paragraph
appears, and the census found none in the corpus.

Three non-blocking, all accepted. The `Walk.para` analogy overstated — `para` is
cleared at `Event::End(TagEnd::Paragraph)` as well as compared at use, so §2 now
borrows the comparison and says so. The second-`: `-line refusal does **not**
fall out of the three spend conditions, because a spliced region carries
`#figure(…)` and would fail the content check into silence rather than the error
gate (4) demands; §2 now names the state that closes it. And OQ-5 was the one
item left between the verdict and a plannable phase.

**OQ-5 resolved at convergence rather than in a round**, recorded as an author's
call taken outside the loop because it gated `status: accepted` and no round had
been asked to decide it: **two subjects, not a framework.** §2's own argument for
excluding citations is the evidence — opposite direction, a new asset channel
across three crates against `core` alone, and a different observable. A shared
spelling convention is not a framework; OQ-8 honours it at no cost.

**Converged.** `status` set to `accepted` and Phase 1's `reviewed` set to
2026-08-15. Phases 2 and 3 stay `null` and are not cleared to build: OQ-2 blocks
Phase 2, and OQ-4, OQ-7 and OQ-8 block Phase 3.

### Round 4 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY (past the cap, by the human's decision)**

Verdict: `NOT READY`, one blocking. **Run past §7.6's three-round cap on the
human's explicit authorisation**, recorded here because going past the cap is a
decision a person makes.

**Round 3's blocker was accepted in its conclusion and rejected in its premise,
and the reviewer confirmed the rejection against the code.** Rounds 2 and 3 both
claimed a held image call would be lost — at the document's end, then at a
footnote region's end. `core/src/emit.rs:Walk::finish` drains `pending` itself,
under a doc comment that says why it exists, and both walk-ending sites go
through it. **No image is dropped at either site**, and the drain the spec had
promised to add would have fixed a bug that does not exist.

What survives is smaller and real: `finish` destructures `(call, _)` and writes
`write_image(…, false)`, discarding the form. Held one paragraph longer, a
standalone image reaching `finish` is demoted to `#box(image(…))` — visible
rather than silent, and a one-line fix in the one place both sites share. Round
3's conclusion also survives: two sites, and the footnote one covered by nothing,
since `tests/fixtures/footnotes.md` ends its definition with a list.

**The new blocker is the second consecutive one caused by perturbing the flush
timing**, which is a fact about the design rather than about the prose.
`Walk.para` is a buffer *offset*, not a flag —
`core/src/emit.rs:step` computes `opened: *para == Some(top(bufs).len())` against
the length recorded when the paragraph opened. Holding the flush past
`Start(Paragraph)` means the held call is appended *after* `para` was recorded,
so the offsets no longer match and **the second of two consecutive standalone
images is demoted to `#box(image(…))`**. Re-derived by the author against the
shipped binary: both are bare `#image(…)` today. An `awk` sweep of
`tests/fixtures/*.md` and `samples/*.md` finds **no two consecutive standalone
image paragraphs anywhere**, so no gate case reaches the shape.

One non-blocking: gate (3a)'s "a new fixture case" does not say which file, and
adding the footnote-final image to `tests/fixtures/footnotes.md` would move a
shipped golden and fail gate (7). Outstanding at escalation.

**Escalated again rather than folded.** Two rounds running, the blocker came from
the deferral colliding with machinery it was not designed against, which is
evidence about the mechanism; the author's recommendation to the human is to
replace the deferral rather than patch it. `status` stays `draft` and Phase 1's
`reviewed` stays `null`.

### Round 3 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY (escalated at the cap)**

Verdict: `NOT READY`, one blocking, at the loop's three-round cap. Round 2's
blocker was verified resolved in the files — both sketches carry the blank line,
§2 records the rejected spelling with its measurement, the attachment subsection
no longer says "the next event", and gate (3) pins the rejected form so an
implementer who "fixes" it by widening the standalone test fails there.

**The new blocker is the second instance of the fix's own hazard, and it is the
third time this episode that a fix introduced one** — the pattern §7.3 names.
§2 and Phase 1's scope both say `core/src/emit.rs:emit` gains a post-loop drain
for the held call. **`emit` is not the only site.**
`core/src/emit.rs:collect_definitions` handles `End(TagEnd::FootnoteDefinition)`
and `continue`s *before* dispatching to `step`, taking the body with
`std::mem::replace(&mut walk, Walk::new()).finish()` — so a call still held at a
footnote region's end is discarded with the walk and `step` never sees the
closing event.

Reachable today, and re-confirmed by the author against the shipped binary
rather than taken from the changelog: a definition whose last block is a
standalone image emits
`#footnote[Some text.\n\n#image("dot.png", alt: "alt")]`. Under the longer
deferral that image vanishes while `core/src/lib.rs:image_paths` still returns
its `ImageRef`, so the CLI reads a file whose image never reaches the page —
the silent-drop class `mpdf-001` §2 exists to refuse, which §2 line 343 cites by
name for the *first* drain site.

**No gate case can see it**, which is what makes it blocking rather than a note:
(3a) is scoped to a document-final image in `tests/fixtures/images.md`, (2) to
`tests/golden/images.typ`, and `cargo test --workspace` is silent because
`tests/fixtures/footnotes.md`'s definition ends with a list, not an image —
verified at lines 20–21.

Three non-blocking, all outstanding at escalation: OQ-1's resolution line still
records the pre-round-2 wording and the wrong round, which is the same
stale-summary class that produced round 2's blocker; §2's "this needs no new
state" predates the deferral design and is now inaccurate, since the held call
carries its verdict across events; and the consumed caption paragraph leaves two
`'\n'` separators already pushed into the buffer by
`core/src/emit.rs:step`'s paragraph arms, which the implementer must unwind.

**Escalated rather than looped, per §7.6.** `status` stays `draft` and Phase 1's
`reviewed` stays `null`: the date records convergence, and this phase has not
converged. Going past the cap is a decision a person makes.

### Round 2 — Phase 1 only — 2026-08-15 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`. All four of round 1's blockers resolved, and **one new
blocker the author's own fix introduced** — §7.3's named hazard, and the reason
that step exists.

Round 1's B-2 was resolved by choosing a caption syntax: a `: ` line after the
construct. **The reviewer falsified it by running the parser rather than by
arguing.** With no blank line that is *one* paragraph, not two — the events are
`Start(Paragraph)`, the image, `SoftBreak`, `Text(": …")`, `End(Paragraph)` — so
`core/src/emit.rs:step`'s standalone test, which looks for `End(TagEnd::Paragraph)`
as the next event, answers false. The spec's own flagship example therefore
converted to the **inline** `#box(image(…))` plus a literal `: The conversion
pipeline…`, and the document contradicted itself: the syntax prose specified the
no-blank-line form while the attachment mechanism specified the blank-line form.

Resolved toward the blank line — the caption is a paragraph of its own — because
the prettier spelling is not a syntax choice but a change to the standalone test,
the one discrimination every image in the dialect flows through, and that is the
blast radius Phase 1 is scoped to avoid. The author reproduced the measurement
against the shipped binary before choosing.

Two non-blocking, both accepted: a longer deferral needs a drain, because
`core/src/emit.rs:emit` has no post-loop flush and `tests/fixtures/images.md`
ends with a standalone image (this became round 3's blocker at its second site);
and gate (6)'s caption needles were moved off
`every_bundled_template_meets_the_call_contract`, which is named for the
five-argument contract OQ-3 had just resolved does not widen.

### Round 1 — Phase 1 only — 2026-08-15 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 (this episode — a new spec with no shipped phase, per §7.0).** *Does
this phase produce the observable, and is it the right one?* Yes: the typeset
PDF, with a captioned, numbered figure under an image. **The risk recorded rather
than dismissed:** the conversation that prompted this spec asked about *labels*,
and captions arrived because Typst's `figure` carries both and numbers by
default — so Phase 1 delivers the carrier while the function that motivated the
request lands in Phase 3, which is the shape `mpdf-004` Phase 3's round 0 flagged
about numbering without referencing.

**Scope note.** §7 says a spec with no shipped phase takes one document-wide
round; this episode was scoped to Phase 1 at the human's explicit instruction,
which `loops/review-spec.md` honours. Phases 2 and 3 are deliberately sketches
pending OQ-1 and OQ-2, so a document-wide round would have rediscovered what the
draft already says.

Verdict: `NOT READY`. Four blocking, eight non-blocking, **all twelve accepted;
no rejections in this episode.**

**Two blockers were the phase not being buildable at all.** Phase 1 carried no
exit gate — "to be written when OQ-1 and OQ-3 resolve" — which §3 requires of
every phase and §7 rule 2 names as blocking by definition; and OQ-1, the caption
syntax, was Phase 1's entire input surface and the spec itself said it blocked
the phase. The gate became eight cases; OQ-1 was split, its caption half resolved
and its reference half moved to OQ-8 against Phase 3, which is what unblocked
Phase 1 without pre-deciding the harder question.

**The third was measured, and it inverted the phase's blast radius.** Wrapping
every standalone image was undefined against wrapping only captioned ones, and
the reviewer measured both costs of the former: an uncaptioned `#figure` prints
no number but **still consumes the counter**, so the next captioned figure reads
"Figure 2" with no Figure 1 on the page, and `figure` centres its body where a
bare block sits flush left — a bounding box moving from `xMin=70.87` to
`277.48`. That violates `mpdf-004` Phase 3's stated property, "no document's
typeset output changes unless its author asks". Resolved: the caption is what
makes it a figure. The fourth was the caption content model — whether caption
text is walked as markdown, what an empty caption does, what a `: ` line
following nothing does — none of which any OQ carried.

**The round re-derived every number and corrected one.** OQ-6 had guessed "the
second-largest golden movement after `mpdf-004` Phase 3's seventeen"; the count
is **one** golden with a standalone `#image(` and **one** with a `#table(`, so
the unconditional wrap would have been the *smallest* movement in the record, and
with the third blocker resolved it is **zero** — no fixture carries a caption
line, so Phase 1 can assert "no shipped golden file changed". OQ-3 was answered
from the code in the same round: no sixth look argument, since both looks already
reach `raw` and `table.cell` with `show` rules and no export.

Grounding confirmed rather than corrected, so a later round need not re-verify:
18 golden files, and 17 at `d6d9edc^`, corroborating `mpdf-004` Phase 3's
"seventeen"; §2's measured Typst table reproduced exactly against a probe crate
built on the pinned 0.15.1, including the labelless
``label `<nosuchthing>` does not exist in the document``; `mpdf-002` §1.1's
non-goal and its "bare `#image` is the hook" sentence verbatim; and §1.1's §6.1
working sound on all four steps, so `extends: null` and `supersedes: null` stand.

Five smaller corrections were accepted and folded: `table_call` writes `#table(`
**with** the `#` where `image_call` omits it, which is the asymmetry Phase 2
trips on; the caption attaches at the `pending` flush in `core/src/emit.rs:step`
and not in `write_image`, which cannot see a later event; "`figure` appears
nowhere" narrowed to the Typst *element*; the "changes nothing in `cli` or `app`"
claim narrowed to `cli/src` and `app/src`, with the two tests in the blast radius
named; and OQ-7's grounding corrected —
`core/src/emit.rs:collect_definitions` walks a footnote body **once** and keeps
it, the document walk skips the region, and what is discarded is the headings
vector.
