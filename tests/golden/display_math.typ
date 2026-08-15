#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "Display math", author: none, columns: 2, date: none)

= Display math

A paragraph before the formula, so the block has prose on both sides of it.

$ mitexsqrt(x ^(2 ) + y ^(2 )) <= | x | + | y | $

A paragraph after it. An inline $frac(a ,b )$ and a display $ sum _(i = 1 )^(n ) i $ in one
sentence each take their own form.
