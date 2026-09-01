// The article look, and the one a document gets when it names none. The
// template owns all styling and layout. The emitter produces only the template
// call and the translated body, so a new look is a new .typ file rather than a
// change to the parser or the emitter.
//
// The emitter names every argument on every call, so every bundled look takes
// title, author, affiliation, columns, date, equations, figures, headings and
// citations.
// Every look also exports `divider`, `abstract` and `keywords` beside
// `template`, the last two of which the emitter imports only for a document
// that opened one — separately, so a document may have either, both or neither.
// That is the contract a third look has to meet. `author` arrives as an array of
// `(name, markers)` dictionaries and `affiliation` as an array of strings, both
// `none` where the document wrote the key out: what crosses is the relation
// between the two lists, and every question of how one looks is answered below.
// The defaults are the fallback for a hand-written call; core/src/frontmatter.rs
// holds the ones a document actually gets.

#let template(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", citations: "numeric", doc) = {
  set page(paper: "a4", margin: 2.5cm, columns: columns)
  set text(font: "Libertinus Serif", size: 10pt, lang: "en")
  set par(justify: true, leading: 0.65em)

  // Quotation marks reach the PDF verbatim. The emitter therefore does not
  // escape them, and this line is why.
  set smartquote(enabled: false)

  // Inline code and, later, code blocks. Typst's own default here is a family
  // this binary does not carry, so the family is named rather than left to a
  // fallback that would render code as body text.
  show raw: set text(font: "Libertinus Mono", size: 9pt)

  // The header row of a table. A GFM table has exactly one, and it is always
  // the first, so row zero is the header by construction. Typst's own default
  // sets it in body type, which would flatten a distinction the markdown
  // source draws with its delimiter row.
  show table.cell.where(y: 0): strong

  // What a heading's number says, and whether it says anything. Two keys meet
  // on this one rule, and it is one rule rather than two because the later of
  // two set rules over one field wins: a second `set heading(numbering: …)`
  // above this one would silently no-op back to unnumbered headings and
  // `Table 0.1`.
  //
  // Under `headings`, the author has named the deepest level that carries a
  // number and this look decides what one looks like: `1.1`, the levels joined
  // by a full stop. `numbering("1.1", ..)` repeats its last separator for
  // arguments beyond the pattern, so this one pattern serves all six levels.
  // The cap is the closure returning nothing above the depth — which is what
  // lets a `###` past the cap carry no number while the `##` after it still
  // reads `1.2`, where suppressing the heading *element* would have cost the
  // levels below it their numbers too.
  //
  // Under `plain`, this falls back to the rule `figures` needs. A figure's
  // section prefix is built off `counter(heading)`, and that counter does not
  // advance while its own numbering is `none` — a figure numbered off it would
  // read `Table 0.1` and `Table 0.2` across two sections. A numbering function
  // returning `none` advances it and still puts nothing on the page.
  //
  // The two keys compose without either knowing about the other: the `none`
  // branch exists only to advance `counter(heading)`, and a real numbering
  // function advances it too, so a sectioned document that also numbers its
  // headings gets both.
  //
  // `show heading: it => it.body` reaches the unnumbered page and is refused:
  // it leaves the block rule below nothing to apply to, so a sectioned
  // document's headings lose their spacing and read as ordinary paragraphs.
  set heading(numbering: if headings != "plain" {
    (..n) => if n.pos().len() <= int(headings) { numbering("1.1", ..n.pos()) }
  } else if figures == "sectioned" { (..n) => none } else { none })
  show heading: set text(weight: "bold")
  show heading: set block(above: 1.4em, below: 0.8em)

  // A section restarts every kind's counter. All three are named, because this
  // look numbers images, tables and listings on counters of their own — a reset
  // naming one would hand the other two the advancing prefix with no restart.
  //
  // Scoped to level 1, because an unscoped rule restarts at a `##` as well and
  // two tables either side of a subheading would both read `Table 1.1`.
  //
  // The condition sits *inside* the closure and the rule is installed
  // unconditionally. That is not the shape `equations` takes below, and the
  // difference is the rule kind rather than a style: a `set` takes its
  // condition in its argument because a `set` inside a scoped `if` dies with
  // the block, and a `show` rule has no such form at all.
  show heading.where(level: 1): it => {
    if figures == "sectioned" {
      counter(figure.where(kind: image)).update(0)
      counter(figure.where(kind: table)).update(0)
      counter(figure.where(kind: raw)).update(0)
    }
    it
  }

  // The author asks for numbers in the frontmatter and this look decides what
  // one looks like: `(1)`, which is what an article carries. The set rule sits
  // at the top level of this body, with the condition inside the argument,
  // because a `set` written inside a scoped `if` block dies with the block —
  // it would compile, emit a valid PDF, and put no number on any page.
  //
  // Typst numbers the block form alone, so an inline formula takes none, and
  // one `$$…$$` span is one equation whatever it holds: a multi-line `aligned`
  // derivation takes a single number against its lines rather than one each.
  set math.equation(numbering: if equations == "numbered" { "(1)" } else { none })

  // A caption. The author supplies the words and asks for the treatment by
  // writing a caption at all; this look decides everything about how one
  // looks — the supplement, the number, the separator, the size and the side
  // it sits on. The emitter writes no "Figure", no ":" and no "1" anywhere,
  // which is the seam the table header's boldness already sits on.
  //
  // Beneath the figure is Typst's own default, and it is written out rather
  // than inherited: a look that owns the format owns the position with it, and
  // the other bundled look answers this same question for itself.
  // What a figure's number says. The author asks for the scheme in the
  // frontmatter and this look builds it: `1.1`, the section then the figure,
  // off the heading counter the rule above keeps advancing. `flat` is Typst's
  // own `"1"`, written out rather than inherited, for the same reason the
  // caption's position below is.
  //
  // An equation is deliberately not reached here. Typst numbers one through
  // `math.equation`, so a `$$…$$` under `equations: numbered` keeps `(1)` and
  // takes no section.
  set figure(numbering: if figures == "sectioned" {
    (..n) => numbering("1.1", counter(heading).get().first(), n.pos().first())
  } else { "1" })

  set figure.caption(position: bottom, separator: [. ])
  show figure: set block(above: 1.4em, below: 1.4em)
  show figure.caption: set text(size: 9pt)

  // The space between two members of one figure. A figure with several members
  // reaches this look as a `grid`, and Typst's own default gutter is zero, so
  // two images would touch. The emitter writes how many members there are and
  // never how far apart they sit — a `show` rule over an element it emits, the
  // way this look already reaches `raw` and `table.cell`, so nothing crosses
  // the call contract. The other bundled look picks its own value.
  show figure: set grid(gutter: 1em)

  // Where a listing sits. `figure` centres its body, which is what an image
  // and a table want and what code cannot have: the same `fn` would stand
  // centred with a caption and at its own left edge without one, in one
  // document, and a multi-line block centres as a unit so its left edge lands
  // wherever its longest line puts it. This rule returns a captioned listing
  // to the position its uncaptioned twin holds, and the caption follows the
  // body left with it — one consequence rather than a second rule. Scoped to
  // `raw` because the argument is about how code is read and about nothing
  // else; a group of listings takes it too, since Typst infers the kind
  // through the `grid`.
  show figure.where(kind: raw): set align(left)

  // How far off the margin a block of code sits. Nothing above this line was
  // wrong: this is legibility rather than a defect. A block that begins where
  // the sentence above it begins is doing all its work with a fixed-width face
  // and no other mark that it is a block.
  //
  // The inset is on `raw` and not on the `figure`, and that is the whole of why
  // it does not undo the rule above. A rule on the figure reaches only a
  // *captioned* listing, so the same code would stand at two edges in one
  // document depending on whether it carries a number — the defect the rule
  // above removes, reintroduced by the fix for something else. On
  // `raw.where(block: true)` both twins move by the same 2em, and an inline
  // `raw` span is untouched.
  //
  // The inset compounds with an enclosing indent, and that is wanted: a block
  // inside a list item or a block quote sits 2em off *its own container's*
  // edge, which is the same relation it has at the top level.
  show raw.where(block: true): set block(inset: (left: 2em))

  // A listing's caption goes with its body. The rule above cannot carry it —
  // a caption is not a `raw` block, so it would sit at the margin under a block
  // that had moved — which makes this a second rule with a second argument
  // rather than a consequence of the first. The argument is that a caption
  // names the thing above it and reads as detached from a block whose edge it
  // does not share.
  //
  // The two `2em` are the same number in different units: the first resolves
  // against the size in effect on the code, the second against the size in
  // effect on the caption. They land on one edge here only because this look
  // sizes both at 9pt. The contract a third look inherits is "the caption sits
  // on the block's edge", not "both numbers are 2em" — a look that set
  // its captions smaller than its code would misalign the two silently, by
  // exactly the ratio.
  show figure.caption.where(kind: raw): it => pad(left: 2em, it)

  // The scheme the marks follow. The author asks for `numeric` or `author-date`
  // and this look names the style that answers it, which is the seam
  // `equations` set: `harvard-cite-them-right` for the second, chosen on the
  // merged group — it separates two sources with a semicolon where Elsevier
  // Harvard uses the comma it also puts between author and year, so a pair
  // cited at once stays two — and `ieee` by name for the first, which is Typst's
  // own default, so the numeric page is provably the page it always was rather
  // than assumed to be. The condition sits inside the argument for the reason
  // the equation rule's does: a `set` inside a scoped `if` dies with the block.
  // `cite`'s own `style` defaults to the bibliography's, so the marks and the
  // list take this one rule together.
  set bibliography(style: if citations == "author-date" { "harvard-cite-them-right" } else { "ieee" })

  // The label above the reference list. The emitter writes
  // `#bibliography(…, title: none)` and hands the words to the look, because
  // Typst's own title is a real `heading` — one more heading in the compiled
  // document than the markdown wrote, which withdraws every anchor
  // `md_to_pdf_with_anchors` reports and breaks the desktop app's scroll sync
  // with nothing on the page to show for it. So this is styled text, set to the
  // rhythm this look gives a heading without being one. The other bundled look
  // picks its own size and spacing.
  show bibliography: it => {
    block(above: 1.4em, below: 0.8em, text(size: 1.2em, weight: "bold", "References"))
    it
  }

  // The title block spans every column. `scope: "parent"` is what lifts it out
  // of the column grid, and Typst supports that only together with `float`.
  // A document with none of the four keys gets no title block at all. The date
  // and the affiliations join that test for the same reason: a key the author
  // wrote that reached no page would vanish, which is what the dialect refuses.
  //
  // **`clearance` was 1.6em and is 1.834em, and the reason is down-page from
  // here**: it is the third of the three numbers that set the front matter's
  // rhythm, and the only one that can lift the gap between this block and an
  // abstract. Measured 2026-09-01, it puts 11.00 pt between the two, and 12.39 pt
  // between this block and the body of a paper carrying no front-matter block at
  // all, where 1.6em read 10.05. `#let abstract` below carries the working, since
  // the three numbers are only legible together.
  if title != none or author != none or affiliation != none or date != none {
    place(top + center, scope: "parent", float: true, clearance: 1.834em, {
      set par(justify: false)
      align(center, {
        // Whether the markers reach the page at all, read off the data rather
        // than off a key. The schema makes them optional at exactly one
        // affiliation, so a document that wrote none belongs entirely to it and
        // a lone superscript on every name would say nothing. This one answer
        // governs both runs: markers appear on the names and on the
        // affiliations together, or on neither.
        let marked = author != none and author.any(one => one.markers.len() > 0)

        // Each run, with the gap that stands above it. The gap rides the run
        // rather than its index, because which of the four are present varies
        // and an index cannot tell the affiliations from the date.
        //
        // The `em`s resolve against this block's 10pt body and never against
        // the 17pt title. That is how the value shipped since Phase 9 came to
        // ask for 4pt where the title-to-author join needs 6.78pt and the
        // author-to-date join 5.07pt just to clear zero: under 4pt the boxes
        // overlapped and the author rode up into the title's descenders.
        //
        // The `linebreak()`s stay and so does `weak: true`. Neither was the
        // defect. Weak spacing applies here in full and linearly, with no
        // threshold and no collapse, so the shape was never discarding
        // anything; and dropping the `linebreak()`s would make each run its own
        // paragraph and land Typst's paragraph spacing on top of whatever these
        // lines ask for.
        let written = ()
        if title != none {
          written.push((0em, text(size: 17pt, weight: "bold", title)))
        }
        // The authors run on one line, separated by commas, each name followed
        // by its own markers as a superscript. A name carrying two of them
        // reads as one superscript rather than two, which is what joining
        // before the call to `super` buys.
        if author != none {
          written.push((1.8em, text(size: 11pt, author.map(one => {
            if marked and one.markers.len() > 0 {
              [#one.name#super(one.markers.map(str).join(","))]
            } else {
              [#one.name]
            }
          }).join(", "))))
        }
        // The affiliations sit directly beneath the authors, one to a line,
        // smaller and italic, each behind the number that points at it. They
        // set closer to the names above them than the block's other joins,
        // because they belong to that line rather than standing beside it.
        if affiliation != none {
          written.push((1.0em, text(size: 9pt, style: "italic",
            affiliation.enumerate(start: 1).map(((index, name)) => {
              if marked { [#super(str(index))#name] } else { [#name] }
            }).join(linebreak()))))
        }
        // The date sits under the block, beneath the affiliations where a
        // document carries them, set smaller. It is typeset exactly as the
        // author wrote it.
        if date != none {
          written.push((0.9em, text(size: 10pt, date)))
        }
        for (index, run) in written.enumerate() {
          let (gap, line) = run
          if index > 0 {
            linebreak()
            v(gap, weak: true)
          }
          line
        }
      })
    })
  }

  doc
}

// A thematic break. The emitter calls this and decides nothing about the rule's
// weight or its spacing; both live here, with the rest of the styling. The
// length is 100% of the containing column, so the rule matches the text width
// under either column count.
#let divider() = block(
  above: 1.2em,
  below: 1.2em,
  line(length: 100%, stroke: 0.5pt + luma(60%)),
)

// The abstract a paper opens with, set across every column above the body. The
// emitter writes `#abstract[…]` for a document that opened `::: abstract`, and
// names this in the import only for such a document, so nothing about a
// document that has none changes.
//
// `scope: "parent"` is what lifts the block out of the column grid, and Typst
// supports that only together with `float` — the title block's own shape,
// above. The three floats stack in source order, the title block first, because
// the emitter's calls stand after the show line and the title block is placed
// inside `template` before `doc`. Which of the other two follows is the
// author's: keywords written above an abstract typeset above it.
//
// `clearance` is the gap *below*, and only that: the space above this block is
// the title block float's own clearance, so the two are set independently.
//
// **It is below-only, which is the whole difficulty here**: this one number is
// the gap to the keywords block in a paper that has one and the gap to the body
// in a paper that does not, and a float cannot see what follows it. Re-measured
// 2026-09-01 over `tests/fixtures/keywords.md` and `tests/fixtures/abstract.md`,
// the relation is linear — the text-to-text gap is `clearance` less 4.34 pt, at
// 1em = 10 pt — so the value is a trade between the two cases rather than a
// reading of either.
//
// **Both front-matter floats carry the same clearance, and that is the whole
// design decision here.** Each one's clearance is the gap below *itself*, so
// whichever block the author wrote last is the one that sets the boundary out of
// the front matter — and the author's order is theirs, not this look's. Giving
// the two different values makes the page depend on that order: measured
// 2026-09-01 with the abstract at 1.5em and keywords at 2em, an abstract-first
// paper read 10.66 pt inside and 14.30 pt out, and a keywords-first one read
// 15.21 pt inside and 9.30 pt out — the same defect, mirrored. Equal values make
// all four shapes read alike, and `keywords` below carries this same 1.6em.
//
// **The front matter is a header, so the gap inside it is the smallest of the
// three.** Measured 2026-09-01: 11.00 pt from the title block to the first
// front-matter block, 10.30 pt between the two of them, and 11.00 pt out to the
// body. It read 10.72 / 11.66 / 10.30 when the two blocks merely shared a
// clearance — the widest gap on the page sitting *inside* the thing that gap is
// meant to bind together — and 10.72 / 15.66 / 14.30 before that.
//
// **Three gaps, three numbers, and none of the three is free of the others**,
// which is why this is written out rather than left as tuned values. A float's
// clearance is the gap below itself, so, in points:
//
//   title -> first  =  title clearance  -  the pull
//   internal        =  this clearance   -  4.34  -  the pull
//   -> body         =  this clearance   -  5.70
//
// The 4.34 and 5.70 are what a float's box and a heading's box leave above their
// own text; their 1.36 pt difference is why **equal clearances can never put the
// boundary above the internal gap**, and it is what the pull below exists to
// beat. Read the other way, with the title block's clearance held still, asking
// the boundary to exceed the internal gap by 0.70 pt *forces* the title gap to
// 8.66 pt — not a preference but the only value left. Lifting it back to 11.00 is
// what moved the title block's own clearance, and that number is spent on every
// paper carrying no front matter at all: this look chose that knowingly, and the
// title block's own comment records what it cost there.
//
// Going lower here was measured and refused: 1.5em puts the boundary at 9.30 pt
// and 1.2em at 7.66, against the ~8 pt that read glued to the text when this was
// first tuned.
//
// **The label is styled text and never a `heading`**, on the reference list's
// own reasoning one section up and for one more reason besides: a heading here
// would take a section number under `headings` and restart every figure counter
// under `figures: sectioned`, so the abstract would renumber the document it
// opens. The block is 80% of the text width, which is narrower than the page
// and wider than a column under either column count.
#let abstract(body) = place(
  top + center,
  scope: "parent",
  float: true,
  clearance: 1.67em,
  block(width: 80%, {
    // `place(top + center)` sets the alignment its content inherits, so the
    // paragraphs would centre their last lines without this. The label is the
    // one thing here that is centred, and it says so.
    set align(left)
    set text(size: 9pt)
    // **The pull, and it acts on the gap *above* this block only.** Shortening
    // the content lifts its text and its own box bottom together, so whatever
    // follows keeps the gap the clearance gave it. That is the one lever that
    // separates two gaps a single clearance would otherwise set as one.
    v(-2.06pt)
    block(width: 100%, above: 0em, below: 0.7em,
      align(center, text(size: 1.15em, weight: "bold", "Abstract")))
    body
  }),
)

// The terms a paper is indexed by, set under the abstract and above the body.
// The emitter writes `#keywords((…))` for a document that opened
// `::: keywords`, handing over an array of *content* — one element per term,
// already escaped, in the order the author wrote them.
//
// **The separator between two terms is this look's and never the emitter's**,
// which is why the terms arrive as an array and not as a joined string: `core`
// has no way to know that this look wants a comma and another wants a middle
// dot. `join` is where that call is made, and a term may itself hold a comma —
// *figure numbering, sectioned* — which is what made `;` the author's separator
// rather than a preference.
//
// The block is the abstract's own shape at the abstract's own measure, so the
// two read as one piece of front matter rather than as two blocks that happen
// to be adjacent. It stacks *after* the abstract when the author wrote it
// after, floats being issued in source order — and it carries the **same
// clearance the abstract does**, deliberately, so that which of the two ends the
// front matter changes nothing about the page. The abstract's own comment
// carries the measurements and why they are shared. **The label is styled text and
// never a `heading`**, on the abstract's own reasoning: a heading here would be
// one `core` never counted, and `anchors_from` withdraws every anchor in the
// document on that mismatch, silently.
#let keywords(terms) = place(
  top + center,
  scope: "parent",
  float: true,
  clearance: 1.67em,
  block(width: 80%, {
    // `place(top + center)` sets the alignment its content inherits, so a
    // one-line list would centre itself without this.
    set align(left)
    set text(size: 9pt)
    // The abstract's own pull, for the reason recorded there. Both blocks carry
    // it, so which of the two the author wrote first changes nothing.
    v(-2.06pt)
    text(weight: "bold", style: "italic", "Keywords: ")
    terms.join(", ")
  }),
)
