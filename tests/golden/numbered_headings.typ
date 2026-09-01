#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "sectioned", headings: "2")

= First

A heading may carry its number. #raw("headings: 2") in the frontmatter asks for one
and says how deep the numbers go; the look decides what one looks like, exactly
as it decides a caption's separator and an equation's format.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Number]),
  [a heading], [1],
), caption: [The first table of the first section.]) <tab:one>

== Background

A #raw("##") sits inside the cap, so it carries a number of its own — and the table
below carries the second number of this section, because the figure counters
restart at a #raw("#") and nowhere else.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Number]),
  [a heading], [1\.1],
), caption: [The second table of the first section.])

=== Detail

A #raw("###") sits below the cap and carries no number at all. The #raw("##") after it still
reads 1.2, so a level above the cap costs the levels below it nothing.

== Second Background

The second subheading of the first section, numbered where the #raw("###") above it is
not.

= Second

A #raw("#") takes the next section number and restarts every kind's counter, so the
table below is the first table of this section rather than the third of the
document.

#figure(table(
  columns: 2,
  align: (auto, center),
  table.header([Construct], [Number]),
  [a section], [2],
), caption: [The first table of the second section.])

The number a reference reads is the number its caption reads, so #ref(<tab:one>)
names the first table of the first section.
