# Figures

## A caption is what makes a figure

A paragraph of its own beneath an image, a table or a code block, opening `: `,
is that block's caption. Leave the blank line above it.

![The three steps, drawn as boxes](pipeline.svg)

: The conversion pipeline, with the *emitter* in the middle. {#fig:pipeline}

A block with no caption is the plain block it has always been: it takes no
number and consumes none. The image below has none, so nothing on the page
counts it.

![The same three steps, uncaptioned](pipeline.svg)

A code block takes a caption the same way. Both bundled looks stand a block of
code 2em off the edge this sentence begins on, captioned and uncaptioned alike,
so the same code reads the same wherever it stands:

```rust
fn convert(md: &str) -> Result<Vec<u8>> {
    md_to_pdf(md, &[])
}
```

: The whole of the pipeline, as one call. {#lst:convert}

## More than one member under one caption

A pair of `:::` lines makes several blocks into one figure, with one caption,
one number and one name. Put the caption last, just above the closing line:

:::

![The parser, which reads the markdown](parse.svg)

![The emitter, which writes the Typst](emit.svg)

: The two halves the diagram above draws as boxes. {#fig:halves}

:::

That is one figure, not two. md2pdf reads two words after the opener,
`abstract` and `keywords`, and every other one is yours — a group's kind comes
from what the members are, so the two tables below are a *Table* however the
group was opened:

::: table

| Step | Reads    |
| ---- | -------- |
| one  | markdown |

| Step | Writes |
| ---- | ------ |
| two  | Typst  |

: Two tables under one caption, and one number between them. {#tab:steps}

:::

Everywhere but the start of a paragraph, `:::` is ordinary text — inside a
sentence, on a later line of a paragraph, and inside a code block, which is how
the examples in the README are written.

## Naming a figure and pointing at it

End a caption with `{#name}` to name the figure it makes, then write `[](#name)`
to point at it. As [](#fig:pipeline) shows, the emitter sits in the middle; the
kinds it reaches are set out in [](#tab:kinds), the call that drives it is
[](#lst:convert), and the two halves are drawn separately in [](#fig:halves)
and counted once in [](#tab:steps).

The number in each of those sentences is the typesetter's, so it stays true
when anything moves. Insert a figure above one and the sentence renumbers
itself without your touching it, which is the whole reason to number anything.

The `fig:`, `tab:` and `lst:` prefixes are a convention and nothing more. The
kind comes from what the caption sits under, so the image below is a *Figure*
even though it is named without a prefix at all, and [](#unprefixed) says so.

![A mark, standing alone](mark.svg)

: A figure named with no prefix. {#unprefixed}
