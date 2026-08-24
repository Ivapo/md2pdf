// The article look, and the one a document gets when it names none. The
// template owns all styling and layout. The emitter produces only the template
// call and the translated body, so a new look is a new .typ file rather than a
// change to the parser or the emitter.
//
// The emitter names every argument on every call, so every bundled look takes
// title, author, columns, date, equations and figures. That is the contract a
// third look has to meet. The defaults below are the fallback for a
// hand-written call; core/src/frontmatter.rs holds the ones a document actually
// gets.

#let template(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat", doc) = {
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

  // Whether a heading advances a counter of its own. Under `sectioned` a figure
  // number is built off `counter(heading)`, and that counter does not advance
  // while its own numbering is `none` — a figure numbered off it would read
  // `Table 0.1` and `Table 0.2` across two sections. A numbering function
  // returning `none` advances it and still puts nothing on the page.
  //
  // This *replaces* the plain `set heading(numbering: none)` rather than
  // joining it. Two set rules over one field resolve to the later one, so the
  // old line standing below this one would win and the whole scheme would
  // silently no-op back to `Table 0.1`.
  //
  // `show heading: it => it.body` reaches the same page and is refused: it
  // leaves the block rule below nothing to apply to, so a sectioned document's
  // headings lose their spacing and read as ordinary paragraphs.
  set heading(numbering: if figures == "sectioned" { (..n) => none } else { none })
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
  // centred with a caption and flush left without one, in one document, and a
  // multi-line block centres as a unit so its left edge lands wherever its
  // longest line puts it. This rule returns a captioned listing to the
  // position its uncaptioned twin holds, and the caption follows the body
  // left with it — one consequence rather than a second rule. Scoped to `raw`
  // because the argument is about how code is read and about nothing else; a
  // group of listings takes it too, since Typst infers the kind through the
  // `grid`.
  show figure.where(kind: raw): set align(left)

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
  // A document with none of the three keys gets no title block at all. The
  // date joins that test, because a key the author wrote that reached no page
  // would vanish, which is what the dialect refuses.
  if title != none or author != none or date != none {
    place(top + center, scope: "parent", float: true, clearance: 1.6em, {
      set par(justify: false)
      align(center, {
        let written = ()
        if title != none {
          written.push(text(size: 17pt, weight: "bold", title))
        }
        if author != none {
          written.push(text(size: 11pt, author))
        }
        // The date sits under the author, set smaller. It is typeset exactly
        // as the author wrote it.
        if date != none {
          written.push(text(size: 10pt, date))
        }
        for (index, line) in written.enumerate() {
          if index > 0 {
            linebreak()
            v(0.4em, weak: true)
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
