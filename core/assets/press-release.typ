// The press-release look, selected with `template: press-release` in the
// frontmatter. It owns all styling and layout, exactly as the article look
// does, and the parser and the emitter know nothing about either.
//
// The emitter names every argument on every call, so this file takes the same
// five the article look takes. It sets only what the frontmatter supplied and
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
#let template(title: none, author: none, columns: 1, date: none, equations: "plain", doc) = {
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
  show heading: set block(above: 1.8em, below: 0.85em)

  // A press release rarely numbers a formula, but the author asks rather than
  // the look, so this file answers the same question the article look answers
  // and reaches the same convention: `(1)`. The set rule sits at the top level
  // with the condition inside the argument, because a `set` written inside a
  // scoped `if` block dies with the block and would number nothing at all.
  set math.equation(numbering: if equations == "numbered" { "(1)" } else { none })

  // A caption, answered the way this look answers everything else: for itself.
  // The author writes the words and asks for the treatment; the supplement,
  // the number, the separator and the size are decided here, and the emitter
  // writes none of them. Where the article look separates the number from the
  // words with a full stop, this one uses a dash, which is the point of the
  // seam rather than a disagreement to reconcile.
  //
  // The caption still sets beneath the figure. A press release runs one wide
  // column and a caption above the thing it names would read as a heading.
  set figure.caption(position: bottom, separator: [ — ])
  show figure: set block(above: 1.2em, below: 1.2em)
  show figure.caption: set text(size: 9.5pt)

  // The space between two members of one figure, answered here as everything
  // else is. A figure with several members arrives as a `grid`, whose default
  // gutter is zero, so two images would touch. One wide column has room to
  // spare, so this look sets them further apart than the article's does — the
  // seam again, not a disagreement to reconcile.
  show figure: set grid(gutter: 1.4em)

  // Where a listing sits, decided here as everything else is. `figure` centres
  // its body; an image and a table lose nothing to that and code loses its
  // left edge, so a captioned block would stand where its uncaptioned twin
  // does not. This look reaches the same answer the article look reaches, and
  // that is not the seam collapsing: the two disagree on a caption's separator
  // because house style is a look's to pick, and they agree here because how
  // code is read is not. A look that centred its listings would be choosing
  // something rather than inheriting it.
  show figure.where(kind: raw): set align(left)

  // The label above the reference list, answered here as everything else is.
  // The emitter writes `title: none` and passes the words over, because Typst's
  // own title is a real `heading`: the compiled document would carry one more
  // than the markdown wrote, and every heading anchor would be withdrawn with
  // nothing on the page to show for it. This look sets the label to its own
  // heading rhythm — larger and further from the text than the article's, which
  // is what one wide column has room for.
  show bibliography: it => {
    block(above: 1.8em, below: 0.85em, text(size: 12pt, weight: "bold", "References"))
    it
  }

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
      // A headline that wraps wants its own leading. The body's is set for
      // 11pt lines and reads as a gap at 20pt.
      if title != none {
        block(below: 0.5em, {
          set par(leading: 0.35em)
          text(size: 20pt, weight: "bold", title)
        })
      }
      if author != none {
        block(text(size: 10.5pt, style: "italic", author))
      }
      divider()
    })
  }

  doc
}
