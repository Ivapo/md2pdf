#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", citations: "numeric")

= Captions on blocks

One mechanism over three constructs, each keeping a counter of its own. This
document carries one of each, so the three numbers can be read apart.

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The conversion pipeline.])

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a table], [its own],
), caption: [The constructs and the #emph[counters] they keep.])

#figure(raw(block: true, lang: "rust", "fn main() {\n    println!(\"hello\");\n}"), caption: [The entry point, with a #raw("raw") span in its caption.])

A caption reaches the construct above it and no other. The table below carries
none and stays the bare block it has always been; the listing after it takes
the caption beneath them both.

#table(
  columns: 2,
  table.header([Uncaptioned], [Table]),
  [stays], [bare],
)

#figure(raw(block: true, lang: "rust", "fn second() {}"), caption: [The listing above this line, and not the table above that.])
