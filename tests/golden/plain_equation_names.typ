#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "Names an equation does not take", author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Names an equation does not take

A name is a whole run on or below the closing fence. This equation carries
one, and nothing points at it, which the shipped default allows:

$ E = m c ^(2 ) $ <eq:energy>

The three shapes below are the prose they have always been, markers and all,
because in none of them is the group the whole of the run.

$ w = 4 $ {\#eq:trailing} and more

$ y = 5 $ see {\#eq:leading}

An inline $x + 1$ {\#eq:inline} carries no number, so it carries no name either.

A caption's own name rides the end of its line, and a display span inside one
does not take it:

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The pipeline, whose middle step is $ y = m x + b $]) <fig:pipeline>

so #ref(<fig:pipeline>) resolves to the figure, as it did before equations could
be named at all.
