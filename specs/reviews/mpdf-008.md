# Review record — mpdf-008 (`specs/multi_file_documents_spec.md`)

Append-only. One heading per round, newest first.

### Round 3 — Phase 1 only — 2026-08-24 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, six non-blocking, all six folded. Converged in three
rounds, at the cap rather than under it. All three round-2 blockers confirmed **against
the files** rather than the changelog.

The reviewer worked both questions it was asked. **The API break is covered by the gate
and was overstated in the scope**: `md_to_typst` has 97 call sites in
`core/tests/golden_test.rs` plus one in `cli/src/main.rs`, `image_paths` has five and
`bibliography_path` three — all inside the workspace, so gate (8)'s `cargo test
--workspace` catches every one by compilation, and `web/src/lib.rs` calls none of the
three, reaching `core` only through `md_to_pdf` and `md_to_pdf_with_anchors`. **Gate
(4)'s enumeration is complete** for every message that reaches a user through `Error`'s
`Display`; the one it does not cover is not an `Error` at all, and became NB14.

The six folded, every one an inconsistency the round-2 fixes introduced:

1. **§2 contradicted itself on the `emit.rs` construction count** — twenty-one in one
   place, twenty-three in another, with the scope repeating the stale one. 23 is right:
   21 `Err(Error::` plus two bare constructions.
2. **"Eight" went stale in three places** once the ninth variant landed — a subsection
   heading over a nine-row table, gate (4)'s "two of the eight variants", and the
   close-out's "§2's eight messages".
3. **The "four functions" and "three types" lists became five and four**, since
   `section_paths` returns `Vec<SectionRef>` and `SectionRef` carries a location. Recorded
   with the consequence that `section_paths` relocates to the identity and always will,
   because it reads the master alone and OQ-1 refuses nesting.
4. **Phase 1's scope contradicted itself about the wrappers** — "the only edit either
   wrapper takes" sat two sentences below "three signatures widen". It is four edits each.
5. **"Roughly eighty mechanical edits" omitted what the B6 fix created.** With the widened
   signatures the figure is near a hundred and eighty, and that is the number that answers
   "is this one plan-mode pass".
6. **The CLI's own sentence for a section it cannot read was unnamed**, and a section
   whose bytes are not UTF-8 is a new failure mode. Both named, with
   `core/src/bibliography.rs`'s precedent for the second.

**The restructure was ruled legal by the reviewer as well as by the author**: `status:
draft`, every phase `reviewed: null` and `shipped: null`, so §6.1's don't-renumber rule —
which governs shipped phases — does not bite, and §7's "a document-wide round on a spec
with no shipped phase is one episode covering the document" makes it interior to this
episode rather than a new one.

### Round 2 — Phase 1 only — 2026-08-24 — same reviewer, resumed with the author's changelog — **NOT READY**

Verdict: `NOT READY`, three blocking, five non-blocking. **Every one of the three blockers
was introduced by round 1's own fixes** — the failure mode §3 of the loop names in so many
words, and the round that proves the same-agent resume earns its place.

1. **§2 contradicted itself about who performs the join, and one reading destroyed the
   relocation map.** A paragraph left over from the draft said *"the caller reads sections
   first, joins, and only then asks for the image list"*, where the new decision says
   `core` joins "because it did the joining". A caller that joined would leave the map with
   no source — B4 by the back door, and the path of least resistance because
   `image_paths(md: &str)` accepts a joined string unchanged.
2. **Three of the four named relocation boundaries take a bare `&str`.**
   `core/src/lib.rs:md_to_typst`, `core/src/lib.rs:image_paths` and
   `core/src/lib.rs:bibliography_path` take no assets, so none can see a section's bytes,
   join, build a map or relocate. The spec said the four "relocate on the way out" and
   never said how their signatures change.
3. **The phase's own new refusals fell outside the gate its own fix had built.** Gate (4)
   enumerates §2's table, the table was closed at eight, and a marker naming an unsupplied
   section had no variant — while `web/src/lib.rs:render` calls `md_to_pdf` directly with a
   fixed asset array, so no wrapper can catch it. Resolved as a ninth variant on
   `mpdf-007` §2's own argument for `MissingBibliography`, *"the words are the only thing
   that differs, and they are the whole point"*.

**Both numbers in §2 were wrong, and both were wrong because the author copied them from
round 1 instead of deriving them.** Re-derived here and in the spec: **48** pattern sites,
**7** using `..`, of which `core/tests/golden_test.rs`'s
`Err(Error::UnsupportedConstruct { line, .. })` binds `line` *and* uses `..` — so **42
need editing and 6 do not**, not the 36 the spec claimed. And **36** construction sites —
23 in `core/src/emit.rs`, six each in `core/src/frontmatter.rs` and `core/src/lib.rs`, one
in `core/src/math.rs` — not 21, which had counted `Err(Error::` matches rather than
constructions. "Nothing inside the walk changes" was corrected to a claim about the
*value* rather than the source.

Also folded: the never-cleared `meta` accumulator left this spec entirely, as a code-only
fix needing no spec action under §6.1 step 0, and OQ-2's self-declared "Blocks Phase 1's
gate" was corrected to "Blocks nothing"; and the desktop app's mid-state after Phase 1 was
named in `mpdf-002` Phase 1's own words.

### Round 1 — Phase 1 only — 2026-08-24 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes, with a caveat recorded rather
than buried. The phase as drafted produced **no** observable and said so with an argument —
it was the precondition that stopped the join shipping a measured lie. The caveat: it was
only wanted if the join followed, since shipped alone it was a widened error type whose
file is always absent. **Round 1 then dissolved the question**, by establishing that the
phase could not deliver even that; the merge that followed made all three phases produce
the observable, and the caveat with it.

Verdict: `NOT READY`, four blocking, eight non-blocking. One generalist reviewer, matching
every prior episode in this corpus. **No finding was rejected, in any of the three rounds.**

**The largest finding is that the phase could not do what it claimed, and the work that
would have done it landed in no phase.** The draft widened the error types and left the
walk alone. The file would have had no source: every line is produced by
`core/src/emit.rs:line_of` from an offset into the joined string, and the carriers between
there and the caller are bare integers — `core/src/emit.rs:Emitted`'s
`headings: Vec<usize>` is the sharpest, since `core/src/lib.rs:anchors_from` takes
`lines: Vec<usize>` and could not know a file if it wanted one. Filling the field meant
widening some twenty internal carriers and no phase named them.

**The fix was not to thread the file but to stop needing to.** `core` already knows where
each section begins, because it did the joining, so a joined line resolves to a file and a
line by one comparison against that map, applied at the boundary. No internal carrier
changes. **That dissolved the phase split**: with no threading to do, the widening had
nothing to do alone, so it merged with the join and the document went from four phases to
three, all of which now produce the observable.

The other three blocking:

1. **The message a widened variant produces was never stated**, and three of the eight
   variants already carry a `{path}` — so a widened sentence would have carried two paths
   with nothing to tell them apart. Resolved by a table of all the messages verbatim and
   one rule: only the asset path is quoted.
2. **How `Display` omits an absent file was unspecified**, and thiserror's `#[error]`
   cannot conditionally drop a field, so the three available mechanisms had different blast
   radii. Closed to a `Location` type with one `Display`.
3. **Gate (1) was keyed to a property the suite does not have.** All 43 error assertions in
   `core/tests/golden_test.rs` destructure the variant's fields and produce no `Display`
   string at all; the repo's only byte-exact `Display` assertion is
   `core/tests/page_examples_test.rs:every_refusal_prints_the_sentence_beside_it`, three
   rows over two variants. "Every error shape" had no enumerable referent and "the message
   it produces today" no source of truth. Rebuilt as an enumerating test naming its own
   rows, with the two platform-dependent `"os error"` assertions excluded by name.

Notable among the non-blocking: `core/src/lib.rs:Anchor` silently loses `Copy`; the scope's
claim that no wrapper formats an error was true of `Error` but false of the two ref types
beside it, which `cli/src/main.rs:read_assets` and `app/src/document.rs:read_assets_with`
format by hand; `web/src` is compiled by nothing `cargo test --workspace` runs; the
`mpdf-004` quotation had dropped the word **typeset**, which was precisely what tied the
original property to a golden; and the `mpdf-002` Phase 1 precedent was misdescribed —
that phase produces its observable at the library level, so it was never a precedent for a
phase producing none. The last was accepted by deletion, the sentence having lived in the
phase the merge removed.
