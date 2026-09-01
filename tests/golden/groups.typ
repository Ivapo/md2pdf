#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Figures with more than one member

A caption attaches to the construct directly above it, so no arrangement of the
#raw(": ") marker says "these two images are one figure". A pair of #raw(":::") lines does.
This document runs one counter past a group and back, so the number a group
takes can be read against the numbers around it.

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [A single captioned image, which takes the first number.])

#figure(grid(columns: 2, image("dot.png", alt: "The first of a pair"), image("dot.png", alt: "The second of a pair")), caption: [Two images under one caption, side by side.]) <fig:pair>

As #ref(<fig:pair>) shows, a group is one figure: one caption, one number, and one
name to point at it with. The image below takes the number after the group's
rather than the number after its last member, which is what a group being one
figure means on the page.

#figure(image("mark.svg", alt: "A check mark"), caption: [A second single image, which takes the third number.])

#figure(grid(columns: 2, image("dot.png", alt: "The first of another pair"), image("mark.svg", alt: "The second of another pair")), caption: [A group opened with a word, which is still a Figure.]) <fig:word>

The word after an opener is the author's own convention and the dialect does
not read it: Typst takes the kind from what the members are, so the group above
is a Figure however it was opened. The two tables below are a Table, on the
same rule and with a counter of their own.

#figure(grid(columns: 2, table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a table], [its own],
), table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Counter]),
  [a group], [one],
)), caption: [Two tables under one caption, and the #emph[first] Table.]) <tab:pair>

A group holds the three constructs a caption reaches, so two code blocks make a
Listing the same way. See #ref(<tab:pair>) for the counters they keep apart.

#figure(grid(columns: 2, raw(block: true, lang: "rust", "fn first() {}"), raw(block: true, lang: "rust", "fn second() {}")), caption: [Two listings under one caption.])

The marker is reserved at the first text of a paragraph and nowhere else. A
line reading ::: inside a sentence is prose, and so is one standing later in a
paragraph:
::: not a group.

A fenced block is where a document that documents this syntax puts one, so the
marker reaches the page verbatim there:

#raw(block: true, lang: "markdown", ":::\n\n![The first of a pair](pipeline.svg)\n\n![The second of a pair](check.svg)\n\n: Two images under one caption. {#fig:pair}\n\n:::")

An indented block is the same arm, so it is the same hatch:

#raw(block: true, ":::\n![tight](dot.png)\n:::")

Neither of those is a group, and neither raises anything.
