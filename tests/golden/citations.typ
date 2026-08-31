#import "template.typ": template, divider
#show: template.with(title: "Citing a source", author: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Citations

A bracketed citation #cite(label("DBLP:books/lib/Knuth86a")) reaches the reference list, and
the collapsed form #cite(label("DBLP:books/lib/Knuth86a")) cites the same source. The key
carries a #raw(":") and a #raw("/"), which is the shape Typst's own label syntax cannot
read, so it crosses as a string.

Nothing else here is a citation. An email a\@b.com, a bare \@thing, a prefix form
\[see \@k\], a bracketed email \[a\@b.com\], an \[ open bracket, a \] close bracket and
an a\[0\] index all reach the page as the text they have always been.

A citation inside a footnote definition#footnote[The definition cites #cite(label("DBLP:books/lib/Knuth86a")) as well.]<fn-1> is set where the note is set,
which is why it travels out of that definition's own walk.

#bibliography("refs.yml", title: none)
