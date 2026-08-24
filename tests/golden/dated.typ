#import "template.typ": template, divider
#show: template.with(title: "A Dated Article", author: "Iva Po", columns: 2, date: "10 August 2026", equations: "plain", figures: "flat")

= Introduction

Body text in the article look. The frontmatter names no template, so this
document gets the default one, and its import line is the one every document
carried before the key existed.

== Background

The date is a free string that the author wrote. Nothing parses it, nothing
reformats it, and no clock is read. The article look sets it beneath the
author line.
