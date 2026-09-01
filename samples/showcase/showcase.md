---
title: Everything the Dialect Carries
author: Ivan Postigo
date: 23 August 2026
template: article
columns: 1
equations: numbered
figures: sectioned
headings: 2
bibliography: refs.bib
---

# What this is

This is one document that uses every construct md2pdf supports, so that you can
see all of them set on a page at once and read the markdown that produced them
side by side. It is written across six files, because being written across
several files is itself one of the constructs. Convert it, open the PDF, then
change something and convert it again:

    md2pdf showcase.md

Everything below is inside the dialect, so it compiles as it stands. Nothing in
this folder reaches outside it — the bibliography sits beside this file and the
four figures beside the sections that draw them, which is the only shape a path
may take. A document and the files it names travel as one folder.

## The frontmatter, nine of the ten keys

The block at the top of this file carries nine of the ten, and all ten are
optional. The tenth is `affiliation`, which samples/authors.md carries instead.
Delete the whole block and the PDF still compiles, with no title block at all.

`title`, `author` and `date` become the block at the top of the first page, and
`date` is your text and nothing else — md2pdf never reads a clock, so this file
makes the same PDF on every machine and on any day.

`template` picks the look and takes `article` or `press-release`. This file
names the first; change that one word and convert again to read all of this in
the other look, where a caption's number is separated by a dash rather than a
full stop and the title block sets flush left over a rule. `columns` takes `1`
or `2`; leave it out and you get the count your look brings, which is 2 for an
article. This file says 1, which is why you are reading one wide column rather
than two narrow ones.

`equations: numbered` puts a number on every display formula, which is what
lets a sentence point at one. `figures: sectioned` gives every figure, table
and listing the number of the section it stands in, which is why every caption
below carries two numbers rather than one. `headings: 2` numbers the headings
themselves, down to the second level — which is why the line above this
paragraph reads *1.1* and why the third-level headings under *Headings* carry
no number at all. `bibliography` names the file the citations at the end are
resolved against.

You say *whether* in all four cases, and for `headings` you say *how deep* as
well: there is no `headings: numbered`, because a depth is the part a yes-or-no
key cannot say. The look says *how*: what a number looks like, what word stands
before it, and what punctuation follows.

## Several files, and the marker that joins them

What you are reading is the **master**. It carries the frontmatter, says what
the whole thing is, and names its sections in the order they are read. A name is
an empty-text link pointing at a markdown file, alone in its own paragraph — the
five below are the real ones, and this is what one looks like:

    [](sections/text.md)

**That is the other half of the shape the naming section uses.** `[](#name)`
points at a figure; `[](sections/text.md)` reads in a file. The empty brackets
are what make either one, so a link that carries text is an ordinary link
whatever it points at.

**It is one document and not five PDFs stapled together.** The files are joined
before anything is parsed, so the figure numbers, the cross-references, the
footnotes and the citations run continuously through all six. The reference link
in `sections/text.md` resolves against a definition at the foot of
`sections/notes-and-sources.md`, and nothing in the pipeline notices, because
nothing in it was ever written against a file.

**Only the master carries frontmatter.** That is what makes the title, the look,
the column count and the numbering one set of answers rather than several to
reconcile. A section that opened with a `---` line would be refused, naming that
file — and so would a section that named a section of its own.

**A section's neighbours are its own.** The four figures sit in `sections/`,
beside the files that draw them, so the `![a mark](mark.svg)` written in
`sections/text.md` means `sections/mark.svg`. A chapter folder holding its own
figures can be moved or shared whole; moving it means editing one line here, the
marker, which is the line that exists to say where a section is. The
bibliography stays beside this file, because it is this file's frontmatter that
names it.

[](sections/text.md)

[](sections/blocks.md)

[](sections/figures.md)

[](sections/mathematics.md)

[](sections/notes-and-sources.md)
