---
title: A Sample Article
author: Iva Po
columns: 2
---

# Introduction

This file exists so you can see md2pdf work without writing anything first.
Convert it, open the PDF, then change something here and convert it again.

Everything in this document is inside the supported dialect, so it compiles
as it stands. Change the columns key in the block above from 2 to 1, convert
it again, and the same text runs across the full width of the page instead.

# What the frontmatter controls

The block at the top of this file carries three keys, and all three are
optional. The title and the author become the block centred at the top of the
first page. That block spans every column, whatever the column count is,
because it is placed outside the column grid rather than inside the first
column.

The columns key takes 1 or 2, and 2 is what you get when the key is absent.
Delete the whole frontmatter block and the PDF still compiles: it simply
arrives with no title block and two columns.

A key outside those three is an error rather than a silent omission. So is a
columns value of 3. The tool names the key and the line, and exits with code
1, because a PDF that quietly ignored half of its own frontmatter would be
lying about its source.

# What the body may contain

Headings and paragraph text, and that is all for now. Heading levels 1 to 6
map to Typst headings of the same level, so the two below are real headings,
not bold paragraphs.

## A second level heading

Text under a second level heading. A single newline inside a paragraph is a
soft break, and it does not start a new paragraph. This sentence began on a
new line in the markdown source, and it still belongs to the paragraph above
it.

### A third level heading

Text under a third level heading. Anything else — a bullet list, emphasis, a
link, a code block, a table, a block quote — is an error today, and support
arrives construct by construct.

# Characters you do not have to think about

Typst reads several characters as markup, and the emitter escapes all of them
for you before the compiler ever sees them. So a price of $5 stays five
dollars and never opens math mode, issue #5 stays an issue number, a lone *
star stays a star, and a snake_case word keeps its underscore.

The same holds for an @ sign, a < less than, a > greater than, an [ open
bracket, a ] close bracket, a ~ tilde, a + plus, and a // double slash. You
write what you mean, and it reaches the page.

# Where the look lives

One file owns every visual decision: the page size, the margins, the font,
the heading style, the title block, and the column count. That file is
core/assets/template.typ, and the fonts ship inside the binary beside it.

Nothing is fetched over the network at any point, and no font is read from
your operating system. The same markdown therefore compiles to the same PDF
on every machine, which is what makes the golden-file tests worth having.
