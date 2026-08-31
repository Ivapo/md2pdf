#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Strikethrough

A sentence with #strike[struck text] in it, and #strike[one tilde] strikes the same way,
because the parser reads a delimiter run of one as well as one of two.

Nesting works either way round: #emph[emphasis around #strike[a struck phrase]], and a
struck link #strike[#link("https://typst.app")[Typst]].

A struck phrase inside alt text flattens to its own words:
#box(image("dot.png", alt: "a struck caption"))

Two things stay prose beside it. Inline code carries #raw("a ~~ pair") verbatim,
and a \$5 to \$10 range keeps both dollars.
