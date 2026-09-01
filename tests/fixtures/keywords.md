---
title: A Paper With Keywords
author: Iva Po
figures: sectioned
headings: 2
---

::: abstract

The terms a paper is indexed by are metadata rather than prose, and the obvious
home for metadata is a frontmatter key. That home was measured and rejected: a
look placing the terms from its own arguments places them at a fixed point in
its own template, so keywords written above an abstract and keywords written
below one would land in the same place, whatever the author wrote.

So the terms live in the body, between a pair of `:::` lines with the word
`keywords` after the opener, and one paragraph of them separated by `;`. The
semicolon is this project's own separator for a list whose members may hold
commas, which the frontmatter's `author` and `affiliation` keys already use for
exactly that reason, and a term such as the third below carries one.

:::

::: keywords

cross-references; C# and C++; figure numbering, sectioned; markdown

:::

# Introduction

The keywords above take no number and restart no counter, so the table below is
the first table of the first section rather than the second.

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

: The only table this paper shows, and the first of its section. {#tab:one}
