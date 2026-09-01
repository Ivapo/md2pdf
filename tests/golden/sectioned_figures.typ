#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "sectioned", headings: "plain")

= The scheme

A figure number may carry the section it stands in. #raw("figures: sectioned") in the
frontmatter asks for it, and the look decides what one looks like, exactly as it
decides a caption's separator and an equation's format.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a table], [its own],
), caption: [The first table of the first section.]) <tab:one>

== A subheading takes no number and restarts nothing

The counters restart at a #raw("#") and nowhere else, so the table below carries the
second number of this section rather than the first of a new one.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a heading], [none],
), caption: [The second table of the first section.])

Each kind keeps a counter of its own, so the image below is the first figure of
this section however many tables stand above it.

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The first figure of the first section.])

= The second section

A #raw("#") restarts every kind at once, so the table below is the first table of this
section rather than the third of the document.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a section], [restarts],
), caption: [The first table of the second section.])

The number a reference reads is the number its caption reads, so #ref(<tab:one>)
names the first table of the first section and carries both halves of it.
