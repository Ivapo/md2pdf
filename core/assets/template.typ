// The article look, and the one a document gets when it names none. The
// template owns all styling and layout. The emitter produces only the template
// call and the translated body, so a new look is a new .typ file rather than a
// change to the parser or the emitter.
//
// The emitter names every argument on every call, so every bundled look takes
// title, author, columns, date and equations. That is the contract a third look
// has to meet. The defaults below are the fallback for a hand-written call;
// core/src/frontmatter.rs holds the ones a document actually gets.

#let template(title: none, author: none, columns: 2, date: none, equations: "plain", doc) = {
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

  set heading(numbering: none)
  show heading: set text(weight: "bold")
  show heading: set block(above: 1.4em, below: 0.8em)

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
