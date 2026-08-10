# Strikethrough

A sentence with ~~struck text~~ in it, and ~one tilde~ strikes the same way,
because the parser reads a delimiter run of one as well as one of two.

Nesting works either way round: *emphasis around ~~a struck phrase~~*, and a
struck link ~~[Typst](https://typst.app)~~.

A struck phrase inside alt text flattens to its own words:
![a ~~struck~~ caption](dot.png)

Two things stay prose beside it. Inline code carries `a ~~ pair` verbatim,
and a \$5 to \$10 range keeps both dollars.
