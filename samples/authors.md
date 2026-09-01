---
title: Three Authors and Two Labs
author: Po, Iva^1; Someone Else^2; A Third Person^1, 2
affiliation: Anthropic, San Francisco; MIT, Cambridge
date: 31 August 2026
columns: 1
---

# Why this file exists

The other samples beside this one carry a single author. This one carries three,
and the two affiliations their markers point at. Convert it, open the PDF, and
the title block reads three names on one line with a superscript each, and the
two affiliations directly beneath them.

# The two lists

`author` and `affiliation` are both lists, and both are separated by a
semicolon. A comma is not a separator: the first name in the block above is
`Po, Iva`, which is one person, and a tool that split on commas would silently
make two of them.

A `^` after a name points at an affiliation, numbered from 1 in the order you
wrote them. A name may point at more than one — the third author above writes
`^1, 2` and reads with both numbers as a single superscript.

# What you may leave out

With exactly one affiliation you may leave the markers out entirely, and every
author belongs to it. That is the commonest paper there is: one lab, several
names, and no need to write a `1` after every one of them.

From two affiliations up the relation has to be stated, so an `affiliation` no
author points at is an error naming its line. So is a marker pointing at an
affiliation you did not write, a marker that is not a number, and an empty entry
in either list — a trailing `;` with nothing after it.

An author with no marker beside authors that have one is not an error. Three
people from one lab and a fourth from none is a real document, and refusing it
would break something nobody asked to have broken.
