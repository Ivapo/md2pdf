#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "Names a line below", author: none, affiliation: none, columns: 2, date: none, equations: "numbered", figures: "flat", headings: "plain")

= Names a line below

A name may stand on the line below the closing fence:

$ x = 1 $ <eq:soft>


Or in the paragraph below it:

$ y = 2 $ <eq:para>



Or on the fence itself, as it always could:

$ z = 3 $ <eq:adjacent>

#ref(<eq:soft>), #ref(<eq:para>) and #ref(<eq:adjacent>) each point at the formula
above their name, and all three labels land in the same place.

Naming is a rule about a run, so a name a paragraph below may be followed by
more prose on the line after it:

$ w = 4 $ <eq:four>


This sentence follows the name on the line below it, and #ref(<eq:four>) still
resolves.
