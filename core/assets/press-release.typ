// The press-release look, selected with `template: press-release` in the
// frontmatter. It owns all styling and layout, exactly as the article look
// does, and the parser and the emitter know nothing about either.
//
// The emitter names every argument on every call, so this file takes the same
// seven the article look takes. It sets only what the frontmatter supplied and
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
#let template(title: none, author: none, columns: 1, date: none, equations: "plain", figures: "flat", headings: "plain", doc) = {
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

  // What a heading's number says, and whether it says anything, answered here
  // as everything else is. Two keys meet on this one rule, and it is one rule
  // rather than two because the later of two set rules over one field wins: a
  // second `set heading(numbering: …)` above this one would silently no-op the
  // whole scheme.
  //
  // Under `headings` the author has named the deepest level that carries a
  // number, and this look reaches the same convention the article look reaches:
  // `1.1`, the levels joined by a full stop. `numbering("1.1", ..)` repeats its
  // last separator past the pattern, so one pattern serves all six levels, and
  // the cap is the closure returning nothing above the depth — a level past the
  // cap costs the levels below it nothing.
  //
  // Under `plain` this falls back to the rule `figures` needs: a figure's
  // section prefix is built off `counter(heading)`, which does not advance
  // while its own numbering is `none` — every figure would read `Table 0.1`. A
  // numbering function returning `none` advances it and puts nothing on the
  // page, where `show heading: it => it.body` would leave the block rule below
  // nothing to apply to and cost this look its heading spacing. A real
  // numbering function advances that counter too, so the two keys compose.
  set heading(numbering: if headings != "plain" {
    (..n) => if n.pos().len() <= int(headings) { numbering("1.1", ..n.pos()) }
  } else if figures == "sectioned" { (..n) => none } else { none })
  show heading: set text(weight: "bold", size: 12pt)
  show heading: set block(above: 1.8em, below: 0.85em)

  // A section restarts every kind's counter. All three are named — this look
  // numbers images, tables and listings separately, so a reset naming one hands
  // the other two an advancing prefix with no restart — and it is scoped to
  // level 1, or a `##` would restart them inside their own section.
  //
  // The condition sits inside the closure and the rule is installed
  // unconditionally. That is not the shape `equations` takes below, and the
  // difference is the rule kind: a `set` carries its condition in its argument
  // because a `set` inside a scoped `if` dies with the block, and a `show` rule
  // has no such form at all.
  show heading.where(level: 1): it => {
    if figures == "sectioned" {
      counter(figure.where(kind: image)).update(0)
      counter(figure.where(kind: table)).update(0)
      counter(figure.where(kind: raw)).update(0)
    }
    it
  }

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
  // What a figure's number says. A press release rarely runs long enough to
  // want a section in one, but the author asks and the look answers, so this
  // file answers the same question the article look answers and reaches the
  // same convention: `1.1`, the section then the figure. `flat` is Typst's own
  // `"1"`, written out rather than inherited.
  //
  // An equation is deliberately not reached here. Typst numbers one through
  // `math.equation`, so a `$$…$$` under `equations: numbered` keeps `(1)` and
  // takes no section.
  set figure(numbering: if figures == "sectioned" {
    (..n) => numbering("1.1", counter(heading).get().first(), n.pos().first())
  } else { "1" })

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

  // How far off the margin a block of code sits, answered here as everything
  // else is. Nothing above this line was wrong: this is legibility rather than
  // a defect, and a press release that carries code at all carries little of
  // it — a block wants a mark that it is one, and a fixed-width face is the
  // only one it has otherwise.
  //
  // The inset is on `raw` and not on the `figure`, which is what keeps it from
  // undoing the rule above: a figure rule reaches only a captioned listing, so
  // the same code would stand at two edges in one document depending on
  // whether it carries a number. On `raw.where(block: true)` both twins move
  // by the same 2em, an inline `raw` span is untouched, and a block inside a
  // list item or a block quote sits 2em off its own container's edge rather
  // than off the page.
  show raw.where(block: true): set block(inset: (left: 2em))

  // A listing's caption goes with its body. The rule above cannot carry it —
  // a caption is not a `raw` block — so this is a second rule with a second
  // argument rather than a consequence of the first, and the argument is that
  // a caption names the thing above it and reads as detached from a block
  // whose edge it does not share. This look reaches the same answer the
  // article look reaches, for the reason it reaches the same alignment.
  //
  // The two `2em` are the same number in different units: the first resolves
  // against the size in effect on the code, the second against the size in
  // effect on the caption. They land on one edge here only because this look
  // sizes both at 9.5pt. The contract a third look inherits is "the caption
  // sits on the block's edge", not "both numbers are 2em".
  show figure.caption.where(kind: raw): it => pad(left: 2em, it)

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
      // The author, and the masthead's one piece of spacing. It sits on this
      // block's `above:` and deliberately not on the title block's `below:`,
      // because that value also governs the gap between a title-only
      // document's headline and its `divider` below — the fix put there would
      // move a document that carries no author at all. Two blocks are separated
      // by the larger of the first's `below:` and the second's `above:`, so
      // 1.0em here wins over the title's 0.5em and the two boxes stop
      // overlapping, which under the shipped values they did by 2.03pt.
      if author != none {
        block(above: 1.0em, text(size: 10.5pt, style: "italic", author))
      }
      divider()
    })
  }

  doc
}
