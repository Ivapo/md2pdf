# Mathematics

An inline span sets in the running text, as $e^{i\pi} + 1 = 0$ does. A span
between double dollars is set as a block of its own, and `equations: numbered`
is why it carries a number:

$$
\int_{-\infty}^{\infty} e^{-x^2}\,dx = \sqrt{\pi}
$$

A display equation is named on its closing fence, since it has no caption line
to carry a name:

$$
a^2 + b^2 = c^2
$$ {#eq:pythagoras}

As [](#eq:pythagoras) shows, the two shorter sides settle the longest one. A
reference to an equation is the one that needs `equations: numbered` in the
frontmatter, because an unnumbered equation has no number for a sentence to
say.

The dialect accepts a closed subset of LaTeX — the Greek letters, the
relations, the operators, the set and logic commands, the large operators, the
accents, the font commands, the named operators, and six environments. A
command outside it is named with your line rather than passed through:

$$
\begin{aligned}
  \nabla \cdot \mathbf{E} &= \frac{\rho}{\varepsilon_0} \\
  \nabla \times \mathbf{B} &= \mu_0 \mathbf{J}
\end{aligned}
$$

A matrix is two more of the six, and a case split is the last:

$$
M = \begin{pmatrix} \alpha & \beta \\ \gamma & \delta \end{pmatrix}
$$

$$
f(x) = \begin{cases} 1, & x \geq 0 \\ 0, & x < 0 \end{cases}
$$

One `$$…$$` span is one equation whatever it holds, so the derivation above
takes a single number against its lines rather than one for each of them.
