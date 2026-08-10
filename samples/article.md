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

Headings, paragraph text, the inline constructs, the block constructs, links,
tables, images, and footnotes. Heading levels 1 to 6 map to Typst headings of
the same level, so the two below are real headings, not bold paragraphs.

## A second level heading

Text under a second level heading. A single newline inside a paragraph is a
soft break, and it does not start a new paragraph. This sentence began on a
new line in the markdown source, and it still belongs to the paragraph above
it.

### A third level heading

Text under a third level heading. A construct outside the list above is an
error rather than a silent omission, and support arrives construct by
construct.

# The inline constructs

A word in *emphasis* comes out italic, a word in **strong emphasis** comes out
bold, and ***both markers at once*** give you bold italic. Each one is written
as a Typst function call rather than as Typst's own markup, because Typst
reads its markers only at a word boundary while markdown does not: that is
what lets emphasis land mid*word*here.

A phrase between two tildes comes out ~~struck through~~, and one tilde on
each side does the same to ~this phrase~. Typst has no markup for a strike at
all, so the function call is the only form here.

Text in `backticks` becomes inline code, and it reaches the page exactly as
you typed it. Nothing inside a pair of backticks is escaped and nothing needs
to be, because the content travels to Typst as a string rather than as markup.
So `#5`, `$5` and `C:\path` all keep their characters.

End a line with a backslash to break it without starting a paragraph,\
like this. Three or more dashes on a line of their own draw a rule across the
column, as below.

---

# The block constructs

A bullet list uses a dash, and an ordered list a number and a dot:

- the first item
- the second item, which nests a list of its own
  - a nested item
  - another nested item

3. an ordered list that starts at three
4. and keeps counting from there

Items that sit on adjacent lines make a tight list, and items separated by a
blank line make a loose one, which is set with more space between the items.
That is markdown's own distinction, and it reaches the page unchanged:

- a loose item

- a second loose item, which holds two paragraphs

  This is the second paragraph of that item. It is indented under the item's
  marker, so it belongs to the item rather than to the list.

A fenced code block keeps every character you typed, and its language tag
tells Typst how to highlight it:

```rust
fn main() {
    println!("hello, world");
}
```

Four spaces of indentation make a code block too. It carries no language
tag, so nothing is highlighted:

    $ md2pdf paper.md -o paper.pdf

A line beginning with a greater-than sign makes a block quote:

> The observable this project produces is the typeset PDF that Typst
> compiles from your markdown.

A row of pipes makes a table. The colons in the second row set each column's
alignment, and the header row comes out in bold:

| Column | Meaning     | Width |
| ------ | :---------- | ----: |
| first  | left        |    10 |
| second | *emphasis*  |   200 |
| third  | `code`      |  3000 |

A row with too few cells is padded with empty ones, and a row with too many
loses the extra. That is what GitHub-flavoured markdown does, and the emitter
follows it rather than deciding anything of its own.

# Links

A link written inline points at [the Typst website](https://typst.app), and a
[reference link][commonmark] points at the CommonMark specification through a
definition at the foot of this file. Both reach the PDF as real links that
your reader can follow.

An address between angle brackets becomes a link on its own, so
<https://github.com/> and <ivapo@example.com> both resolve. The email address
gets a mailto destination, which the markdown itself does not carry.

A destination travels to Typst as a string rather than as markup, so it keeps
every character you typed, a # fragment included. The text of the link is
ordinary body text, and it is escaped like any other.

Two shapes are errors rather than links. A link with an empty destination has
nothing to resolve to, and a link that carries a title holds something neither
Typst nor the PDF can show, so passing it on would drop it in silence.

# Images

Name a file that sits beside this one, and it reaches the page:

![The three steps, drawn as boxes](pipeline.svg)

That image sits alone in its paragraph, so it is set as a block of its own and
it scales down to the column. An image with text beside it stays in the line
instead, the way this check mark ![a check mark](check.svg) sits in this
sentence.

The path is relative to this file, and it stays inside this file's directory.
A URL is an error rather than a download, because nothing is fetched over the
network. So are an absolute path and a path that climbs out with a `..`
segment, because a document and its figures travel as one folder.

The text in the square brackets is alt text. It reaches the accessibility layer
of the PDF rather than the page, so it is not a caption and nothing numbers it.
Leave it empty and the image carries none.

Eight formats work: png, jpg, jpeg, gif, webp, svg, svgz, and pdf. The
extension decides which one a file holds, so bytes that disagree with the name
are an error that names both.

# Footnotes

A reference in the text puts its note at the foot of the column that holds
it[^note], and Typst numbers the notes in the order they appear on the page.
Cite the same note a second time and both markers carry the same number[^note].

[^note]: The note itself. It may hold *emphasis*, `inline code` and more than
    one paragraph, and it may sit anywhere in the file, above or below the
    reference that cites it.

    This second paragraph belongs to the same note, because it is indented
    under the definition that opened it.

Three shapes are errors rather than notes, for the reason every other error
here exists. A definition that no reference cites would reach no page, and
content that vanishes is what this tool refuses to ship. A second definition
for one label would lose a body, and choosing between two bodies is a guess. A
reference inside a definition would put a footnote inside a footnote, which
this dialect does not carry.

A reference whose definition is missing altogether is not an error. It stays
on the page as the text you typed, the way an unresolved link reference does.

# Characters you do not have to think about

Typst reads several characters as markup, and the emitter escapes all of them
for you before the compiler ever sees them. So a price of $5 stays five
dollars and never opens math mode, issue #5 stays an issue number, a lone *
star stays a star, and a snake_case word keeps its underscore.

The same holds for an @ sign, a < less than, a > greater than, an [ open
bracket, a ] close bracket, a ~ tilde, a + plus, and a // double slash. You
write what you mean, and it reaches the page.

# Where the look lives

One file owns every visual decision: the page size, the margins, the body
font, the code font, the heading style, the rule a thematic break draws, the
title block, and the column count. That file is core/assets/template.typ, and
the fonts ship inside the binary beside it.

Nothing is fetched over the network at any point, and no font is read from
your operating system. The same markdown therefore compiles to the same PDF
on every machine, which is what makes the golden-file tests worth having.

[commonmark]: https://spec.commonmark.org/0.31.2/
