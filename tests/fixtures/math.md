---
title: The math this dialect accepts
---

# The math this dialect accepts

Every command below has a span of its own, so a symbol the bundled prelude misses
fails here rather than reaching a reader. A dollar that means a dollar is written
`\$`, as in a \$5 to \$10 range, and reaches the page as itself.

## Greek

- `\alpha` — $\alpha$
- `\beta` — $\beta$
- `\gamma` — $\gamma$
- `\delta` — $\delta$
- `\epsilon` — $\epsilon$
- `\varepsilon` — $\varepsilon$
- `\zeta` — $\zeta$
- `\eta` — $\eta$
- `\theta` — $\theta$
- `\vartheta` — $\vartheta$
- `\iota` — $\iota$
- `\kappa` — $\kappa$
- `\lambda` — $\lambda$
- `\mu` — $\mu$
- `\nu` — $\nu$
- `\xi` — $\xi$
- `\pi` — $\pi$
- `\varpi` — $\varpi$
- `\rho` — $\rho$
- `\varrho` — $\varrho$
- `\sigma` — $\sigma$
- `\tau` — $\tau$
- `\upsilon` — $\upsilon$
- `\phi` — $\phi$
- `\varphi` — $\varphi$
- `\chi` — $\chi$
- `\psi` — $\psi$
- `\omega` — $\omega$
- `\Gamma` — $\Gamma$
- `\Delta` — $\Delta$
- `\Theta` — $\Theta$
- `\Lambda` — $\Lambda$
- `\Xi` — $\Xi$
- `\Pi` — $\Pi$
- `\Sigma` — $\Sigma$
- `\Upsilon` — $\Upsilon$
- `\Phi` — $\Phi$
- `\Psi` — $\Psi$
- `\Omega` — $\Omega$

## Relations

- `\leq` — $a \leq b$
- `\geq` — $a \geq b$
- `\neq` — $a \neq b$
- `\approx` — $a \approx b$
- `\equiv` — $a \equiv b$
- `\sim` — $a \sim b$
- `\propto` — $a \propto b$
- `\to` — $a \to b$
- `\gets` — $a \gets b$
- `\mapsto` — $a \mapsto b$

## Operators

- `\pm` — $a \pm b$
- `\mp` — $a \mp b$
- `\times` — $a \times b$
- `\div` — $a \div b$
- `\cdot` — $a \cdot b$
- `\ast` — $a \ast b$
- `\star` — $a \star b$
- `\circ` — $a \circ b$

## Sets and logic

- `\in` — $x \in A$
- `\notin` — $x \notin A$
- `\subset` — $A \subset B$
- `\subseteq` — $A \subseteq B$
- `\supset` — $A \supset B$
- `\cup` — $A \cup B$
- `\cap` — $A \cap B$
- `\setminus` — $A \setminus B$
- `\emptyset` — $A = \emptyset$
- `\forall` — $\forall x$
- `\exists` — $\exists x$
- `\neg` — $\neg p$
- `\land` — $p \land q$
- `\lor` — $p \lor q$

## The large operators

- `\sum` — $\sum_{i=1}^{n} i$
- `\prod` — $\prod_{i=1}^{n} i$
- `\int` — $\int_0^1 f(x) dx$
- `\oint` — $\oint_C f(z) dz$
- `\lim` — $\lim_{x \to 0} f(x)$

## Constants and ellipses

- `\infty` — $\infty$
- `\partial` — $\partial x$
- `\nabla` — $\nabla f$
- `\ldots` — $a_1, \ldots, a_n$
- `\cdots` — $a_1 + \cdots + a_n$
- `\dots` — $a_1, \dots, a_n$

## Structure

- `\frac` — $\frac{a}{b}$
- `\sqrt` — $\sqrt{x}$
- `\binom` — $\binom{n}{k}$
- `\left` and `\right` — $\left( \frac{a}{b} \right)$

## Accents

- `\hat` — $\hat{x}$
- `\bar` — $\bar{x}$
- `\vec` — $\vec{v}$
- `\tilde` — $\tilde{x}$
- `\dot` — $\dot{x}$
- `\overline` — $\overline{AB}$
- `\underline` — $\underline{AB}$

## Fonts

- `\mathbb` — $\mathbb{R}$
- `\mathbf` — $\mathbf{v}$
- `\mathrm` — $\mathrm{d}x$
- `\mathcal` — $\mathcal{L}$
- `\mathit` — $\mathit{x}$

## The named operators

- `\sin` — $\sin x$
- `\cos` — $\cos x$
- `\tan` — $\tan x$
- `\log` — $\log x$
- `\ln` — $\ln x$
- `\exp` — $\exp x$
- `\min` — $\min(a, b)$
- `\max` — $\max(a, b)$
- `\det` — $\det A$
- `\gcd` — $\gcd(a, b)$

## The control symbols

- `\\` — $a \\ b$
- `\,` — $a\,b$
- `\;` — $a\;b$
- `\:` — $a\:b$
- `\!` — $a\!b$
- `\{` and `\}` — $\{a, b\}$
- `\%` — $100\%$
- `\&` — $a \& b$
- `\_` — $a\_b$
- `\#` — $\#S$
- `\$` — $\$5$

## The environments

- `matrix` — $\begin{matrix} a & b \\ c & d \end{matrix}$
- `pmatrix` — $\begin{pmatrix} a & b \\ c & d \end{pmatrix}$
- `bmatrix` — $\begin{bmatrix} a & b \\ c & d \end{bmatrix}$
- `vmatrix` — $\begin{vmatrix} a & b \\ c & d \end{vmatrix}$
- `cases` — $\begin{cases} 1 & x > 0 \\ 0 & x \leq 0 \end{cases}$
- `aligned` — $\begin{aligned} a &= b \\ c &= d \end{aligned}$
