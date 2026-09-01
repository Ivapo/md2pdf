#import "template.typ": template, divider
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Images

#image("dot.png", alt: "The three steps, drawn as boxes")

A small icon #box(image("mark.svg", alt: "a check mark")) sits inside this sentence.

#box(image("fig#2.png", alt: "an \"outline\" of the whole idea")) opens this paragraph, and text
follows it on the same line.

#image("dot.png")
