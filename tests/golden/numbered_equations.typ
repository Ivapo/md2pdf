#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "Numbered equations", author: none, columns: 2, date: none, equations: "numbered", figures: "flat", headings: "plain")

= Numbered equations

A derivation set over three lines is still one equation, so it takes one number
rather than one per line:

$ aligned( a &= b + c \ &= b + d + e \ &= f ) $

The span below takes the next number, and the inline $frac(a ,b )$ in this
sentence takes none, because Typst numbers the block form alone.

$ sum _(i = 1 )^(n ) i = frac(n \(n + 1 \),2 ) $
