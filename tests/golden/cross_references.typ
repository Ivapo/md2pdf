#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Cross\-references

A name on a caption line makes the figure that caption makes referable, and a
link with no text points at it. The number is Typst's, so it stays true when
anything moves.

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The conversion pipeline.]) <fig:pipeline>

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a table], [its own],
), caption: [The constructs and their #emph[counters].]) <tab:counters>

#figure(raw(block: true, lang: "rust", "fn main() {\n    println!(\"hello\");\n}"), caption: [The entry point.]) <lst:main>

As #ref(<fig:pipeline>) shows, the emitter sits in the middle; the counters are
set out in #ref(<tab:counters>), and the program that drives it is #ref(<lst:main>).

A reference ends this sentence, as it does in #ref(<fig:pipeline>). The prose
carries on afterwards, which is the shape ordinary writing puts one in.

A reference may stand above the figure it names, so #ref(<fig:later>) resolves
against a caption further down the page.

#figure(image("mark.svg", alt: "A check mark"), caption: [The mark this paragraph named before it stood.]) <fig:later>

The prefix is the author's own convention and the dialect neither requires nor
reads it. This image is named without one:

#figure(image("dot.png", alt: "The three steps again"), caption: [A figure named with no prefix at all.]) <pipeline>

and this one is named with a table's:

#figure(image("mark.svg", alt: "A check mark again"), caption: [A figure named with a table's prefix.]) <tab:pipeline>

Both are figures, and #ref(<pipeline>) and #ref(<tab:pipeline>) say so.

A link that carries text is the link it has always been, whatever its
destination: #link("#fig:pipeline")[some words] is one, and so is #link("#fig:pipeline")[ ],
whose text is a space rather than nothing.

A name declared inside a footnote definition is a declared name#footnote[The definition holds a figure of its own.

#figure(image("dot.png", alt: "The three steps, a third time"), caption: [The figure inside the note.]) <fig:note>]<fn-1>, and
#ref(<fig:note>) reaches it from out here.

$ a ^(2 ) + b ^(2 ) = c ^(2 ) $

: This paragraph follows a display equation, which takes no name and no number
of its own, so the marker reaches the page as the prose it is.
