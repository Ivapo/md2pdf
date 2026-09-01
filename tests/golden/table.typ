#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Tables

A pipe table, with a default column and one of each alignment:

#table(
  columns: 4,
  align: (auto, left, center, right),
  table.header([Construct], [Left], [Center], [Right]),
  [emphasis], [#emph[emphasis in a cell]], [plain], [1],
  [code], [#raw("inline code")], [a | pipe], [2],
  [link], [#link("https://typst.app")[Typst]], [plain], [3],
  [short row], [three cells], [only], [],
)
