#import "template.typ": template, divider
#show: template.with(title: "A Minimal Example", author: "Iva Po", columns: 2, date: none, equations: "plain", figures: "flat")

= Introduction

Body text in an article look. The frontmatter names no column count, so this
document gets the default of two.

== Background

A second paragraph, long enough that the column break has something to work
with. The template owns the layout, and the emitter only passes the three
frontmatter keys through to it.
