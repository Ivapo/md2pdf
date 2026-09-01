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

How much you have to say depends on how many affiliations you wrote, and there
are three answers.

Write **no** `affiliation` at all and the markers are dropped: `author: Iva Po^1`
converts, and the byline reads `Iva Po`. That is the file you are still drafting
— the key commented out, or the markers typed before you had the labs to hand —
and it converts rather than stopping. The cost is that if you meant to add the
affiliations and forgot, the PDF simply has none in it. That is not a defect you
have to hunt for; it is the whole block missing from the page.

With exactly **one** affiliation you may leave the markers out entirely, and
every author belongs to it. That is the commonest paper there is: one lab,
several names, and no need to write a `1` after every one of them. Write one
anyway and it is honoured, because at one affiliation it is true.

From **two** up the relation has to be stated, so an `affiliation` no author
points at is an error naming its line. So is a marker pointing at an affiliation
you did not write — `^3` against two labs sets a superscript that points at
nothing, in a byline where the others point at something, and nobody reads a
byline looking for that.

Three things are errors however many affiliations you wrote, because they are
mistakes in what you typed rather than relations that might not exist: a marker
that is not a number, an empty entry in either list — a trailing `;` with nothing
after it — and a `^` with no name in front of it.

An author with no marker beside authors that have one is not an error. Three
people from one lab and a fourth from none is a real document, and refusing it
would break something nobody asked to have broken.
