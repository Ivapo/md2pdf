// The press-release look, selected with `template: press-release` in the
// frontmatter. It owns all styling and layout, exactly as the article look
// does, and the parser and the emitter know nothing about either.
//
// The emitter names every argument on every call, so this file takes the same
// four the article look takes. It sets only what the frontmatter supplied and
// prints no fixed text of its own: the look is this file's job, and the words
// on the page are the author's.

// A thematic break, and the rule under the masthead below. The emitter calls
// this by name on every look, so every look exports it.
#let divider() = block(
  above: 1.2em,
  below: 1.2em,
  line(length: 100%, stroke: 0.5pt + luma(60%)),
)

// `divider` is defined above because a Typst closure captures the scope it is
// written in. The masthead calls it, so it has to exist by this line.
#let template(title: none, author: none, columns: 1, date: none, doc) = {
  // A press release runs in one column by convention, and the frontmatter
  // resolves the count to 1 where the document left the key out. An author who
  // writes `columns: 2` still gets two.
  set page(paper: "a4", margin: 3cm, columns: columns)
  set text(font: "Libertinus Serif", size: 11pt, lang: "en")

  // A press release is read once and quickly. The lines are set ragged right
  // and further apart than the article's, which is what a single wide column
  // needs to stay readable.
  set par(justify: false, leading: 0.8em, spacing: 1.1em)

  // Quotation marks reach the PDF verbatim, as they do in the article look.
  set smartquote(enabled: false)

  show raw: set text(font: "Libertinus Mono", size: 9.5pt)
  show table.cell.where(y: 0): strong

  set heading(numbering: none)
  show heading: set text(weight: "bold", size: 12pt)
  show heading: set block(above: 1.6em, below: 0.6em)

  // The masthead. The dateline sits above the title, where a press release
  // carries it, and the title is set flush left rather than centred, which is
  // what separates this look from the article's on the page. A document that
  // wrote none of the three keys gets no masthead at all.
  if title != none or author != none or date != none {
    block(width: 100%, {
      // The date is typeset exactly as the author wrote it. Nothing here
      // parses it, reformats it, or changes its case.
      if date != none {
        block(below: 0.9em, text(size: 9.5pt, weight: "bold", date))
      }
      if title != none {
        block(below: 0.45em, text(size: 20pt, weight: "bold", title))
      }
      if author != none {
        block(text(size: 10.5pt, style: "italic", author))
      }
      divider()
    })
  }

  doc
}
