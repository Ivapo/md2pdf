// The template owns all styling and layout. The emitter produces only the
// template call and the translated body, so a new look is a new .typ file
// rather than a change to the parser or the emitter.
//
// Phase 1 owns the page setup and the heading style. Phase 2 adds the title
// block and the column toggle.

#let template(doc) = {
  set page(paper: "a4", margin: 2.5cm)
  set text(font: "Libertinus Serif", size: 10pt, lang: "en")
  set par(justify: true, leading: 0.65em)

  // Quotation marks reach the PDF verbatim. The emitter therefore does not
  // escape them, and this line is why.
  set smartquote(enabled: false)

  set heading(numbering: none)
  show heading: set text(weight: "bold")
  show heading: set block(above: 1.4em, below: 0.8em)

  doc
}
