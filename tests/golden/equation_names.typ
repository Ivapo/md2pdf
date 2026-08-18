#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "Named equations", author: none, columns: 2, date: none, equations: "numbered")

= Named equations

A name rides the closing fence, so a formula can be pointed at the way a figure
is. The number is Typst's, which is what keeps the sentence true when anything
moves above it.

$ a ^(2 ) + b ^(2 ) = c ^(2 ) $ <eq:pythagoras>

As #ref(<eq:pythagoras>) shows, the two shorter sides settle the longest one. The
sum below takes the next number and no name, because a name is worth writing
only where something points at it.

$ sum _(i = 1 )^(n ) i = frac(n \(n + 1 \),2 ) $

A reference ends this sentence, as it does in #ref(<eq:pythagoras>). The prose
carries on afterwards, which is the shape ordinary writing puts one in.
