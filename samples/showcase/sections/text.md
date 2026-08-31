# Text

## Headings

Levels 1 to 6 map to headings of the same level, so the four below are real
headings rather than bold paragraphs. This document sets `headings: 2`, so the
`#` and the `##` above carry numbers and the four below carry none — a level
past the depth costs the levels below it nothing, which is why *The inline
constructs* after them still counts on. `plain` is the default, and under it no
heading carries a number while `figures: sectioned` still numbers every figure:
the two keys are independent.

### A third level heading

Text under a third level heading.

#### A fourth level heading

Text under a fourth level heading.

##### A fifth level heading

Text under a fifth level heading.

###### A sixth level heading

Text under a sixth level heading, which is as deep as markdown goes.

## The inline constructs

Body text with *emphasis*, **strong emphasis**, ~~struck text~~ and
`inline code`. A single tilde strikes as well as a double one, so ~this~ is
struck too. Emphasis works inside a word, so mid*dle*ware sets the way you
wrote it — which is why these reach Typst as function calls rather than as its
own markup, whose delimiters are sensitive to word boundaries.

A single newline inside a paragraph is a soft break, and it does not start a
new paragraph. This sentence began on a new line in the source and still
belongs to the paragraph above it. A backslash at the end of a line is a hard
break instead,\
and this line follows it inside the same paragraph.

A formula in the running text, $\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$, typesets
as mathematics rather than as letters.

An image may sit inside a sentence, like this ![a mark](mark.svg) one, and an
image in a line is not a figure: it takes no caption, no number and no name.
Only an image standing alone in its own paragraph can become one.

## Links

An [inline link](https://typst.app), a [reference link][typst] whose
destination is defined at the foot of another file entirely, a bare autolink
<https://typst.app>, and an email address at <you@example.com>. All three link
forms reach the page the same way; the email one gets its `mailto:` added for
it.

A link that carries text is a link whatever its destination, so
[this one](#fig:pipeline) points at a figure below and is still an ordinary
link rather than a cross-reference. It is the *empty* brackets that make a
reference, and the section on naming below has them — as does the master, where
an empty-text link naming a `.md` file reads that file in instead.

## What reaches the page as itself

Characters Typst would otherwise read as its own syntax are escaped for you, so
a `#` alone is a hash, a `$5` is five dollars, an `a@b.com` is an address and
an `@thing` is a thing. None of them is a citation, a formula or a function
call. A ~ tilde on its own is a tilde, and 2026. at the start of a line is a
year and a full stop rather than a numbered list.
