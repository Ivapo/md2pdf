#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Hostile inline code

Inline code reaches the PDF verbatim, so a #raw("`backtick`"), a #raw("#hash"), a
#raw("$dollar"), and a #raw("\\backslash") all survive.

All four at once: #raw("#hash $dollar \\backslash `backtick`").
