#import "press-release.typ": template, divider
#show: template.with(title: "Acme Ships md2pdf 1.0", author: ((name: "Iva Po", markers: ()),), affiliation: none, columns: 1, date: "10 August 2026", equations: "plain", figures: "flat", headings: "plain", citations: "numeric")

= Background

Acme today released md2pdf 1.0, a tool that converts one markdown file into
one typeset PDF entirely on the local machine. The frontmatter names no column
count, so this document takes the one its look gives it.

The dialect is unchanged here. #emph[Emphasis], #raw("inline code") and a
#link("https://typst.app")[link] reach the page through the arms that already serve
them, and the look decides how they are set.

== Availability

The tool is available today.
