// The article look, and the one a document gets when it names none. The
// template owns all styling and layout. The emitter produces only the template
// call and the translated body, so a new look is a new .typ file rather than a
// change to the parser or the emitter.
//
// The emitter names every argument on every call, so every bundled look takes
// title, author, affiliation, columns, date, equations, figures and headings.
// That is the contract a third look has to meet. `author` arrives as an array of
// `(name, markers)` dictionaries and `affiliation` as an array of strings, both
// `none` where the document wrote the key out: what crosses is the relation
// between the two lists, and every question of how one looks is answered below.
// The defaults are the fallback for a hand-written call; core/src/frontmatter.rs
// holds the ones a document actually gets.

#let template(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", doc) = {
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
  if title != none or author != none or affiliation != none or date != none {
    place(top + center, scope: "parent", float: true, clearance: 1.6em, {
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
