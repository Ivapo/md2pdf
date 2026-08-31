#import "press-release.typ": template, divider
#show: template.with(title: "Acme Cites Its Sources", author: "Iva Po", columns: 1, date: "22 August 2026", equations: "plain", figures: "flat", headings: "plain")

= Background

Acme today confirmed that md2pdf documents may cite their sources. The
frontmatter names the file, the caller supplies it beside the images, and the
reference list is set at the end of the document #cite(label("DBLP:books/lib/Knuth86a")).

The look decides what the label above that list says and how it is set. This
one is not the article look, and the words below it are typeset differently.

#bibliography("refs.yml", title: none)
