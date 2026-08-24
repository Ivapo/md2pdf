# Review record — mpdf-008 (`specs/multi_file_documents_spec.md`)

Append-only. One heading per round, newest first.

### Round 2 — Phase 2 only — 2026-08-24 — same reviewer, resumed with the author's changelog — **READY**

Verdict: `READY`, zero blocking, five non-blocking, all five folded. **Converged in two
rounds, under the cap** — where Phase 1 took three and reached it. Every round-1 finding
was confirmed against the files rather than the changelog.

The reviewer worked both questions it was asked. **The prefix leaves the `..` refusal
intact**: `core/src/emit.rs:portable_path` scans `dest.split('/').any(|s| s == "..")`, a
segment scan rather than a prefix test, so `one/../x.png` trips it exactly as `../x.png`
does — confirmed against the binary. Two details the rewrite could have got wrong and did
not: `core/src/emit.rs:check_image` refuses on the *shape* and interpolates no path, so
the prefix never leaks into the sentence the author reads, and the refusal carries a
`Location` that relocates to the section's own file. **The phase is smaller than the draft
it replaces**: `step` has two call sites and `emit` four, so the work is one lookup, three
widened internal signatures, one prefix, two fixture images moved, one CLI test and one
re-blessed golden — an order of magnitude under Phase 1's counted ~180 edits.

The reviewer also newly verified what the whole rewrite now rests on and no shipped test
covers: **a nested asset path compiles end to end.** A document naming `one/figure.png`
and `two/figure.svg` produces a PDF, exit 0 — the `image("one/figure.png")` → `file_id` →
`VirtualRoot::Project` round trip works. The one existing subdirectory case,
`app/src/document.rs`'s `figures/mark.svg`, asserts a *failure*.

Two gate literals were re-derived rather than trusted: `tests/golden/multi_file.typ` holds
exactly two image destinations, so gate (5)'s "two … gain `sections/`" is exact; and gate
(2)'s string reproduces verbatim.

The five folded:

1. **`core/src/lib.rs:render` and `core/src/emit.rs:step` were unnamed** among the
   signatures the map reaches. Both are compiler-forced and decision-free, and both are now
   named.
2. **Gate (2) did not say at which level it is asserted.** It is the library's
   `MissingImage`; the CLI never reaches it, failing earlier at its own `std::fs::read`.
3. **A section with no directory of its own had no gate case.** `[](chapter.md)` must
   prefix with nothing, and a naive `format!("{dir}/{dest}")` yields `/dot.png` — refused
   as absolute, so loud rather than silent, but wrong. Gate (3) now covers both no-prefix
   shapes.
4. **"Five places" did not read off a four-row table.** Corrected to four.
5. **The close-out named a section that breaks nothing.** `rules/pipeline.md`'s §"Images
   and their files" carries *"once per path at its first reference"*, which the prefix
   leaves true — the prefix is what makes two paths two. Only §"Several files" goes
   actively false; §"The CLI" stays true of the caller and needs saying beside it.

### Round 1 — Phase 2 only — 2026-08-24 — fresh clean-room reviewer with repo access — **NOT READY**

**Round 0 — is this the right thing to build at all?** Yes. The phase produces the
observable — a PDF that compiles from a layout which refuses today, so a chapter folder
holding its own figures can be moved, copied or shared whole — and it is `mpdf-002` §2's
"a document and the files it names travel together" applied one level out. It discharges a
promise Phase 1 shipped in writing, in `README.md` and in `cli/src/main.rs:read_assets`'
own doc comment, rather than a want invented afterwards.

Verdict: `NOT READY`, one blocking, five non-blocking. One generalist reviewer, matching
every prior episode in this corpus. **No finding was rejected, in either round.**

**The blocking finding is that the phase's stated mechanism silently corrupts its own
headline case, and that no correct implementation existed inside its scope.** The draft
put the change in the caller — *"an `ImageRef` already carries the file that named it, so
`cli/src/main.rs:read_assets` joins against that file's parent rather than the input's"* —
but **the path the author wrote is an identity, not just a lookup**, in four places:
`core/src/emit.rs:image_call` writes it into the Typst source; `core/src/lib.rs:collect`
keys `supplied`, dedupes `seen` and builds the world's `FileId` from it; and both
`cli/src/main.rs:read_assets` and `app/src/document.rs:read_assets_with` dedupe on it.
Measured against the shipped binary and confirmed independently by the author: two sections
in different folders each naming `figure.png` emit

```
#image("figure.png", alt: "first")
#image("figure.png", alt: "second")
```

with nothing to tell them apart. A caller resolving the two differently would read the
first file, skip the second as already seen, and set one figure twice — no error, nothing
on the page to see, and it lands on exactly the case the phase exists for.

**The fix moved the mechanism out of the caller and into the emitter.** `core` prefixes a
section's image destination with that section's own directory at emission, through the
`core/src/sections.rs:Sources` map Phase 1 already builds, so the path is unique by
construction and there is no collision to detect or refuse. `§2`'s "A section's neighbours
are its own" was REWRITTEN in place, on the precedent Phase 1's own round 1 set. Three
consequences worth the record: **no caller changes at all**, so the app inherits the rule
with Phase 3's sections rather than needing a copy; `portable_path` and `check_image` are
untouched and a section's `..` is still refused; and a single-file document has a
one-segment map, so nothing is prefixed and every golden is byte-identical by the same
arithmetic Phase 1's inertness rests on.

The five non-blocking, all folded: **gate (2) already passed on the shipped tree** and so
pinned nothing, since Phase 1 already names the section's own file and line — what this
phase changes is the *resolved* path in the first slot; **the shipped fixture layout and
`cli/tests/cli_test.rs:a_master_and_its_sections_convert` break the moment this lands**,
and the phase now moves the images down beside the sections that name them rather than
leaving it to be discovered, with `cargo test --workspace` added to a gate that had no
suite item; **the rule would have been true in the CLI and false in the app**, which the
`core`-side mechanism dissolves rather than defers; **the close-out pointed at sections
that hold none of the sentences going stale**; and **"the folder can be moved without
editing a path" overclaimed**, since moving it still means editing the master's marker.

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
