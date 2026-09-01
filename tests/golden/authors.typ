#import "template.typ": template, divider
#show: template.with(title: "A Paper With Several Authors", author: ((name: "Po, Iva", markers: (1,)), (name: "Someone Else", markers: (2,)), (name: "A Third Person", markers: (1, 2))), affiliation: ("Anthropic, San Francisco", "MIT, Cambridge"), columns: 2, date: "31 August 2026", equations: "plain", figures: "flat", headings: "plain", citations: "numeric")

= Introduction

Body text under a title block that carries three authors and the two
affiliations their markers point at. One name holds a comma of its own, which
is why the separator between two authors is a semicolon.
