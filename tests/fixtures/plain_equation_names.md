---
title: Names an equation does not take
equations: plain
---

# Names an equation does not take

A name is a whole run on or below the closing fence. This equation carries
one, and nothing points at it, which the shipped default allows:

$$
E = m c^2
$$ {#eq:energy}

The three shapes below are the prose they have always been, markers and all,
because in none of them is the group the whole of the run.

$$
w = 4
$$ {#eq:trailing} and more

$$
y = 5
$$ see {#eq:leading}

An inline $x + 1$ {#eq:inline} carries no number, so it carries no name either.

A caption's own name rides the end of its line, and a display span inside one
does not take it:

![The three steps, drawn as boxes](dot.png)

: The pipeline, whose middle step is $$y = m x + b$$ {#fig:pipeline}

so [](#fig:pipeline) resolves to the figure, as it did before equations could
be named at all.
