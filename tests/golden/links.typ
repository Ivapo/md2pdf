#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Links

An #link("https://typst.app")[inline link] opens this sentence, a #link("https://spec.commonmark.org/0.31.2/")[reference
link] follows it, and an autolink closes it at
#link("https://github.com/")[https:\/\/github.com\/].

Write to #link("mailto:ivapo@example.com")[ivapo\@example.com], and the address reaches the PDF as a mailto
destination rather than as plain text.

A destination carries whatever CommonMark lets it carry, including the two
characters the markup escape would otherwise touch:
#link("https://example.com/a\"b#frag")[a hostile URL].
