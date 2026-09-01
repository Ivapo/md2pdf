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
with the word `abstract` after the opener, before anything else in the
document.

The look decides what it looks like. The article look you are reading gives it
a centred `Abstract` label and sets it narrower than the page; the press
release look sets the same words as a standfirst under the masthead rule and
prints no label at all, because a press release has a lede rather than an
abstract. Convert this file with `template: press-release` in the frontmatter
to see both.

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

First, and once. Both looks lift the block out of the flow, so an abstract
written further down would appear at the top anyway, and source order and page
order would disagree about a document you can read. Requiring it first makes
the two the same.

In a document written across several files, first means first in the joined
stream, so the abstract may be a section file of its own that the master names
before the rest.
