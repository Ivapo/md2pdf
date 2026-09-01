---
title: A Paper That Opens With an Abstract
author: Iva Po
affiliation: The Typesetting Works, Madrid
date: 1 September 2026
columns: 2
headings: 2
---

::: abstract

An abstract is set across the full width of the page, above the columns the
body runs in, and it is the one block a paper opens with that is neither a
section, nor a figure, nor body text. Write it between a pair of `:::` lines
with the word `abstract` after the opener, in the front matter — above
anything else the document says.

The look decides what it looks like. The article look you are reading gives it
a centred `Abstract` label and sets it narrower than the page; the press
release look sets the same words as a standfirst under the masthead rule and
prints no label at all, because a press release has a lede rather than an
abstract. Convert this file with `template: press-release` in the frontmatter
to see both.

:::

::: keywords

typesetting; markdown; figure numbering, sectioned; C# and C++

:::

# What the block may hold

Paragraphs, and the inline constructs inside them: *emphasis*, **strong**,
`code`, links, and a formula such as $E = mc^2$. That is enough for a
structured abstract, which is written as bold run-in text — **Background.**
like this, then **Methods.**, then **Results.** — rather than as headings.

Everything else is named where you wrote it rather than quietly reshaped: a
heading, a list, a block quote, a table, a code block, an image, a display
equation and a citation are each an error inside an abstract, as is a `: `
caption line, which the block has nowhere to put.

# Where it has to stand

In the front matter, and once. Both looks lift the block out of the flow, so an
abstract written further down would appear at the top anyway, and source order
and page order would disagree about a document you can read. Keeping it above
the body makes the two the same.

The keywords block above is the other thing the front matter may hold, and the
order between the two is yours: written above the abstract, the terms are set
above it. What is refused is body content above either.

In a document written across several files, the front matter is the front
matter of the joined stream, so either block may be a section file of its own
that the master names before the rest.

# The terms a paper is indexed by

`::: keywords` holds one paragraph, and its terms are separated by `;` — the
same separator `author` and `affiliation` take in the frontmatter, and for the
same reason: a keyword may hold a comma of its own, as *figure numbering,
sectioned* above does.

A term is plain text. Emphasis, inline code, a link, a formula, a footnote, an
image, a citation and a hard break are each named where you wrote them, because
a `;` standing inside any of them is a semicolon you never wrote as a
separator. An empty term is an error too, and so is an empty block.

What the terms look like is the look's, down to the character between two of
them: `md2pdf` hands over the list and never the sentence.
