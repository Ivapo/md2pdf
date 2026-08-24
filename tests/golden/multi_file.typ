#import "template.typ": template, divider
#show: template.with(title: "A Report Written Across Four Files", author: "Iva Po", columns: 2, date: none, equations: "plain", figures: "sectioned")

= Introduction

A master names its sections in the order they are read, and the emitter is
handed one stream. Nothing here knows it was written in four files.

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The pipeline, declared in the first file.]) <fig:pipeline>

Last line of part one.

First line of part two, and a paragraph of its own rather than the tail of the
one above it.

= Method

A claim the third file footnotes.#footnote[The note, defined in the third file and set at the foot of the
column that cites it in the second.]<fn-1>

#figure(image("mark.svg", alt: "A check mark"), caption: [The mark, declared in the second file.]) <fig:mark>

= Results

The pipeline of #ref(<fig:pipeline>) produced the mark of #ref(<fig:mark>), so a
number declared in the first file is read in the third.
