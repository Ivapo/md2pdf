#import "template.typ": template, divider
#show: template.with(title: "Footnotes", author: ((name: "Iva Po", markers: ()),), affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", citations: "numeric")

= Footnotes

A reference whose definition follows it#footnote[A note with #emph[emphasis], #raw("inline code"), and more below.

A second paragraph inside the same definition.

- a list item
- a second list item]<fn-1>, in a sentence long enough to
show where the note lands on the page.

A second paragraph cites the note defined above it#footnote[A definition written above the reference that cites it.]<fn-2>, and cites the
first note again#footnote(<fn-1>), spelled in another case.
