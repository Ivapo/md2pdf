---
title: Citing a source
bibliography: refs.yml
---

# Citations

A bracketed citation [@DBLP:books/lib/Knuth86a] reaches the reference list, and
the collapsed form [@DBLP:books/lib/Knuth86a][] cites the same source. The key
carries a `:` and a `/`, which is the shape Typst's own label syntax cannot
read, so it crosses as a string.

Nothing else here is a citation. An email a@b.com, a bare @thing, a prefix form
[see @k], a bracketed email [a@b.com], an [ open bracket, a ] close bracket and
an a[0] index all reach the page as the text they have always been.

A citation inside a footnote definition[^note] is set where the note is set,
which is why it travels out of that definition's own walk.

[^note]: The definition cites [@DBLP:books/lib/Knuth86a] as well.
