#import "template.typ": template, divider
#import "math.typ": aligned, bmatrix, diff, matrix, mitexmathbf, mitexsqrt, negthinspace, pmatrix, sect, vmatrix
#show: template.with(title: "The math this dialect accepts", author: none, columns: 2, date: none, equations: "plain", figures: "flat")

= The math this dialect accepts

Every command below has a span of its own, so a symbol the bundled prelude misses
fails here rather than reaching a reader. A dollar that means a dollar is written
#raw("\\$"), as in a \$5 to \$10 range, and reaches the page as itself.

== Greek

- #raw("\\alpha") — $alpha$
- #raw("\\beta") — $beta$
- #raw("\\gamma") — $gamma$
- #raw("\\delta") — $delta$
- #raw("\\epsilon") — $epsilon.alt$
- #raw("\\varepsilon") — $epsilon$
- #raw("\\zeta") — $zeta$
- #raw("\\eta") — $eta$
- #raw("\\theta") — $theta$
- #raw("\\vartheta") — $theta.alt$
- #raw("\\iota") — $iota$
- #raw("\\kappa") — $kappa$
- #raw("\\lambda") — $lambda$
- #raw("\\mu") — $mu$
- #raw("\\nu") — $nu$
- #raw("\\xi") — $xi$
- #raw("\\pi") — $pi$
- #raw("\\varpi") — $pi.alt$
- #raw("\\rho") — $rho$
- #raw("\\varrho") — $rho.alt$
- #raw("\\sigma") — $sigma$
- #raw("\\tau") — $tau$
- #raw("\\upsilon") — $upsilon$
- #raw("\\phi") — $phi.alt$
- #raw("\\varphi") — $phi$
- #raw("\\chi") — $chi$
- #raw("\\psi") — $psi$
- #raw("\\omega") — $omega$
- #raw("\\Gamma") — $Gamma$
- #raw("\\Delta") — $Delta$
- #raw("\\Theta") — $Theta$
- #raw("\\Lambda") — $Lambda$
- #raw("\\Xi") — $Xi$
- #raw("\\Pi") — $Pi$
- #raw("\\Sigma") — $Sigma$
- #raw("\\Upsilon") — $Upsilon$
- #raw("\\Phi") — $Phi$
- #raw("\\Psi") — $Psi$
- #raw("\\Omega") — $Omega$

== Relations

- #raw("\\leq") — $a <= b$
- #raw("\\geq") — $a >= b$
- #raw("\\neq") — $a != b$
- #raw("\\approx") — $a approx b$
- #raw("\\equiv") — $a equiv b$
- #raw("\\sim") — $a tilde b$
- #raw("\\propto") — $a prop b$
- #raw("\\to") — $a -> b$
- #raw("\\gets") — $a <- b$
- #raw("\\mapsto") — $a |-> b$

== Operators

- #raw("\\pm") — $a plus.minus b$
- #raw("\\mp") — $a minus.plus b$
- #raw("\\times") — $a times b$
- #raw("\\div") — $a div b$
- #raw("\\cdot") — $a dot.c b$
- #raw("\\ast") — $a ast b$
- #raw("\\star") — $a star b$
- #raw("\\circ") — $a compose b$

== Sets and logic

- #raw("\\in") — $x in A$
- #raw("\\notin") — $x in.not A$
- #raw("\\subset") — $A subset B$
- #raw("\\subseteq") — $A subset.eq B$
- #raw("\\supset") — $A supset B$
- #raw("\\cup") — $A union B$
- #raw("\\cap") — $A sect B$
- #raw("\\setminus") — $A without B$
- #raw("\\emptyset") — $A = nothing$
- #raw("\\forall") — $forall x$
- #raw("\\exists") — $exists x$
- #raw("\\neg") — $not p$
- #raw("\\land") — $p and q$
- #raw("\\lor") — $p or q$

== The large operators

- #raw("\\sum") — $sum _(i = 1 )^(n ) i$
- #raw("\\prod") — $product _(i = 1 )^(n ) i$
- #raw("\\int") — $integral _(0 )^(1 ) f \(x \) d x$
- #raw("\\oint") — $integral.cont _(C ) f \(z \) d z$
- #raw("\\lim") — $lim _(x -> 0 ) f \(x \)$

== Constants and ellipses

- #raw("\\infty") — $oo$
- #raw("\\partial") — $diff x$
- #raw("\\nabla") — $nabla f$
- #raw("\\ldots") — $a _(1 )\, dots.h \, a _(n )$
- #raw("\\cdots") — $a _(1 ) + dots.h.c + a _(n )$
- #raw("\\dots") — $a _(1 )\, dots.h \, a _(n )$

== Structure

- #raw("\\frac") — $frac(a ,b )$
- #raw("\\sqrt") — $mitexsqrt(x )$
- #raw("\\binom") — $binom(n ,k )$
- #raw("\\left") and #raw("\\right") — $lr(\( frac(a ,b ) \) )$

== Accents

- #raw("\\hat") — $hat(x )$
- #raw("\\bar") — $macron(x )$
- #raw("\\vec") — $arrow(v )$
- #raw("\\tilde") — $tilde(x )$
- #raw("\\dot") — $dot(x )$
- #raw("\\overline") — $overline(A B )$
- #raw("\\underline") — $underline(A B )$

== Fonts

- #raw("\\mathbb") — $bb(R )$
- #raw("\\mathbf") — $mitexmathbf(v )$
- #raw("\\mathrm") — $upright(d )x$
- #raw("\\mathcal") — $cal(L )$
- #raw("\\mathit") — $italic(x )$

== The named operators

- #raw("\\sin") — $sin x$
- #raw("\\cos") — $cos x$
- #raw("\\tan") — $tan x$
- #raw("\\log") — $log x$
- #raw("\\ln") — $ln x$
- #raw("\\exp") — $exp x$
- #raw("\\min") — $min \(a \, b \)$
- #raw("\\max") — $max \(a \, b \)$
- #raw("\\det") — $det A$
- #raw("\\gcd") — $gcd \(a \, b \)$

== The control symbols

- #raw("\\\\") — $a \ b$
- #raw("\\,") — $a thin b$
- #raw("\\;") — $a thick b$
- #raw("\\:") — $a med b$
- #raw("\\!") — $a negthinspace b$
- #raw("\\{") and #raw("\\}") — $\{ a \, b \}$
- #raw("\\%") — $1 0 0 %$
- #raw("\\&") — $a amp b$
- #raw("\\_") — $a \_ b$
- #raw("\\#") — $hash S$
- #raw("\\$") — $dollar 5$

== The environments

- #raw("matrix") — $matrix( a zws , b zws ; c zws , d )$
- #raw("pmatrix") — $pmatrix( a zws , b zws ; c zws , d )$
- #raw("bmatrix") — $bmatrix( a zws , b zws ; c zws , d )$
- #raw("vmatrix") — $vmatrix( a zws , b zws ; c zws , d )$
- #raw("cases") — $cases( 1 & x > 0 , 0 & x <= 0 )$
- #raw("aligned") — $aligned( a &= b \ c &= d )$
