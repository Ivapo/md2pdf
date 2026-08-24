---
title: Everything the Dialect Carries
author: Iva Po
date: 23 August 2026
template: article
columns: 1
equations: numbered
figures: sectioned
bibliography: refs.bib
---

# What this file is

This is one document that uses every construct md2pdf supports, so that you can
see all of them set on a page at once and read the markdown that produced them
side by side. Convert it, open the PDF, then change something here and convert
it again:

    md2pdf showcase.md

Everything below is inside the dialect, so it compiles as it stands. Nothing in
this folder reaches outside it — the four figures and the bibliography sit
beside this file, which is the only shape a path may take. A document and the
files it names travel as one folder.

## The frontmatter, all eight keys

The block at the top of this file carries every key there is, and all eight are
optional. Delete the whole block and the PDF still compiles: it arrives with no
title block, two columns, no numbers on anything, and no reference list.

`title`, `author` and `date` become the block at the top of the first page.
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
below carries two numbers rather than one. `bibliography` names the file the
citations at the end are resolved against.

You say *whether* in all three cases. The look says *how*: what a number looks
like, what word stands before it, and what punctuation follows.

# Text

## Headings

Levels 1 to 6 map to headings of the same level, so the four below are real
headings rather than bold paragraphs. Neither bundled look numbers them, and
under `figures: sectioned` they still do not — the number goes on the figure,
not on the heading.

### A third level heading

Text under a third level heading.

#### A fourth level heading

Text under a fourth level heading.

##### A fifth level heading

Text under a fifth level heading.

###### A sixth level heading

Text under a sixth level heading, which is as deep as markdown goes.

## The inline constructs

Body text with *emphasis*, **strong emphasis**, ~~struck text~~ and
`inline code`. A single tilde strikes as well as a double one, so ~this~ is
struck too. Emphasis works inside a word, so mid*dle*ware sets the way you
wrote it — which is why these reach Typst as function calls rather than as its
own markup, whose delimiters are sensitive to word boundaries.

A single newline inside a paragraph is a soft break, and it does not start a
new paragraph. This sentence began on a new line in the source and still
belongs to the paragraph above it. A backslash at the end of a line is a hard
break instead,\
and this line follows it inside the same paragraph.

A formula in the running text, $\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$, typesets
as mathematics rather than as letters.

An image may sit inside a sentence, like this ![a mark](mark.svg) one, and an
image in a line is not a figure: it takes no caption, no number and no name.
Only an image standing alone in its own paragraph can become one.

## Links

An [inline link](https://typst.app), a [reference link][typst] whose
destination is defined at the foot of this file, a bare autolink
<https://typst.app>, and an email address at <you@example.com>. All three link
forms reach the page the same way; the email one gets its `mailto:` added for
it.

A link that carries text is a link whatever its destination, so
[this one](#fig:pipeline) points at a figure below and is still an ordinary
link rather than a cross-reference. It is the *empty* brackets that make a
reference, and the section on naming below has them.

## What reaches the page as itself

Characters Typst would otherwise read as its own syntax are escaped for you, so
a `#` alone is a hash, a `$5` is five dollars, an `a@b.com` is an address and
an `@thing` is a thing. None of them is a citation, a formula or a function
call. A ~ tilde on its own is a tilde, and 2026. at the start of a line is a
year and a full stop rather than a numbered list.

# Blocks

## Lists

- a bullet list
- whose second item nests one
  - like this
  - and this
- and whose last item does not

3. an ordered list that starts at three
4. and keeps counting from there
5. because every item carries its own number

A list with blank lines between its items is a loose one, and it sets further
apart:

- the first item, standing on its own

- the second item, standing on its own

Tightness passes through structurally, so md2pdf decides nothing about the
spacing — the blank lines you left are what set it.

## Quotes and rules

> A block quote, which may hold *emphasis* and `code` like any other text.

A thematic break draws the rule below, and the look decides its weight and its
spacing:

---

## Code

A fenced block takes the first word of its info string as the language:

```rust
fn main() {
    println!("hello");
}
```

An indented block takes none, and is set the same way:

    the four spaces in front of this line
    are what make it a code block

## Tables

The delimiter row sets each column's alignment, and the header row is repeated
across a page break:

| Construct | In the dialect | Counter  |
| --------- | :------------: | -------: |
| a table   | yes            | its own  |
| an image  | yes            | its own  |
| a listing | yes            | its own  |
| raw HTML  | no             | none     |

: The three kinds a caption reaches, and the counter each one keeps.
  {#tab:kinds}

That caption is what makes the table above a *Table*, and the number it carries
is the section's, because this file asks for `figures: sectioned`.

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

A code block takes a caption the same way, and both bundled looks send a
captioned listing back to the left edge that its uncaptioned twin stands on:

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

That is one figure, not two. A word after the opener is yours and md2pdf does
not read it — the kind comes from what the members are, so the two tables below
are a *Table* however the group was opened:

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

# Mathematics

An inline span sets in the running text, as $e^{i\pi} + 1 = 0$ does. A span
between double dollars is set as a block of its own, and `equations: numbered`
is why it carries a number:

$$
\int_{-\infty}^{\infty} e^{-x^2}\,dx = \sqrt{\pi}
$$

A display equation is named on its closing fence, since it has no caption line
to carry a name:

$$
a^2 + b^2 = c^2
$$ {#eq:pythagoras}

As [](#eq:pythagoras) shows, the two shorter sides settle the longest one. A
reference to an equation is the one that needs `equations: numbered` in the
frontmatter, because an unnumbered equation has no number for a sentence to
say.

The dialect accepts a closed subset of LaTeX — the Greek letters, the
relations, the operators, the set and logic commands, the large operators, the
accents, the font commands, the named operators, and six environments. A
command outside it is named with your line rather than passed through:

$$
\begin{aligned}
  \nabla \cdot \mathbf{E} &= \frac{\rho}{\varepsilon_0} \\
  \nabla \times \mathbf{B} &= \mu_0 \mathbf{J}
\end{aligned}
$$

A matrix is two more of the six, and a case split is the last:

$$
M = \begin{pmatrix} \alpha & \beta \\ \gamma & \delta \end{pmatrix}
$$

$$
f(x) = \begin{cases} 1, & x \geq 0 \\ 0, & x < 0 \end{cases}
$$

One `$$…$$` span is one equation whatever it holds, so the derivation above
takes a single number against its lines rather than one for each of them.

# Notes and sources

## Footnotes

A reference in the text puts its note at the foot of the column that holds
it[^note], and a label is matched without regard to case, so the same note
cited again[^NOTE] carries one number rather than two.

The definition may sit anywhere in the file, above or below the reference, and
it may hold more than one paragraph[^long].

## Citations

Name a bibliography in the frontmatter and cite a key with `[@key]`. The list
at the end is set from the file, and the mark and the numbering are the
typesetter's [@quill2019].

The brackets are required, because an unbracketed `@` is load-bearing in
ordinary text. Markers and the documents that hold them have been argued over
before [@arden2021], as has the business of numbering things that move
[@olsson2023], and a caption is a subject of its own [@harlow2024].

Nothing is fetched. No key is resolved against anything on the network, and the
file is read for its keys before anything is typeset — so a key it does not
hold is named at the line you cited it on rather than reported by the compiler.

[^note]: The note itself, which may hold *emphasis*, `code` and a
    [link](https://typst.app) like any other text.

[^long]: A note may run to a second paragraph.

    This is that second paragraph, and it is indented under the definition
    rather than marked in any other way.

[typst]: https://typst.app
