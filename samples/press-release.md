---
title: md2pdf 1.0 Converts Markdown to PDF With No Toolchain
author: Iva Po
template: press-release
date: 10 August 2026
---

The frontmatter above names the press-release look, and that one line is the
whole difference between this document and the article beside it. The markdown
is the same dialect. The date is a string this file wrote, so the tool reads no
clock and the same file makes the same PDF on any day.

This look names no column count, so it takes the one its own convention gives:
a single column. Write columns: 2 here and you would get two instead.

# What shipped

md2pdf converts one markdown file into one typeset PDF, entirely on the local
machine. There is no server, no account, and no LaTeX installation. The fonts
ship inside the binary, so the same markdown compiles to the same PDF on every
machine.

The body may hold everything the article sample demonstrates. *Emphasis*,
**strong emphasis**, ~~struck text~~ and `inline code` all reach this look
through the same emitter:

- a bullet list, in a single column
- and a second item under it

| Look | Columns |
| --- | :---: |
| article | 2 |
| press-release | 1 |

> A block quote, set the way this look sets one.

A claim that wants a source[^1] carries a footnote, and the note lands at the
foot of the column that holds the reference.

[^1]: The source, at the foot of the page.

---

# Availability

md2pdf is available today. The thematic break above draws the same rule this
look puts under its masthead, because every look exports that rule under one
name and the emitter decides nothing about how it looks.

# Contact

Write to <you@example.com>, or read the documentation at
[the project README](https://github.com/ivapo/md2pdf).
