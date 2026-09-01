#import "template.typ": template, divider, abstract
#show: template.with(title: "A Paper With An Abstract", author: ((name: "Iva Po", markers: ()),), affiliation: none, columns: 2, date: none, equations: "plain", figures: "sectioned", headings: "2", citations: "numeric")

#abstract[
An abstract is the one block a paper opens with that is neither a section nor a
figure nor body, and until this construct existed no markdown in this dialect
could ask for one. A pair of #raw(":::") lines says so, with #raw("abstract") after the
opener — the single word the dialect reads, every other one being the author's
own convention over a figure group.

The block collects every paragraph between the delimiters rather than only the
first, which is why this fixture writes two of them, and both run long enough
to fill the measure the look gives them: a paragraph of short sentences
produces no line wider than a column of body text, and a page that spans is
exactly what is being read here.
]

= Introduction

The abstract above takes no number and restarts no counter, so the table below
is the first table of the first section rather than the second.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a table], [its own],
), caption: [The only table this paper shows, and the first of its section.]) <tab:one>
