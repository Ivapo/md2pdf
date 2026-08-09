// The template owns all styling and layout. The emitter produces only the
// template call and the translated body, so a new look is a new .typ file
// rather than a change to the parser or the emitter.
//
// The emitter names every argument on every call. The defaults below are the
// fallback for a hand-written call; core/src/frontmatter.rs holds the ones a
// document actually gets.

#let template(title: none, author: none, columns: 2, doc) = {
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

  set heading(numbering: none)
  show heading: set text(weight: "bold")
  show heading: set block(above: 1.4em, below: 0.8em)

  // The title block spans every column. `scope: "parent"` is what lifts it out
  // of the column grid, and Typst supports that only together with `float`.
  // A document with neither key gets no title block at all.
  if title != none or author != none {
    place(top + center, scope: "parent", float: true, clearance: 1.6em, {
      set par(justify: false)
      align(center, {
        if title != none {
          text(size: 17pt, weight: "bold", title)
        }
        if author != none {
          if title != none {
            linebreak()
            v(0.4em, weak: true)
          }
          text(size: 11pt, author)
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
