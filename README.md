# md2pdf

Convert one markdown file into one typeset PDF. Everything happens on your machine —
no server, no SaaS, and no LaTeX toolchain.

`pulldown-cmark` parses the markdown, a small emitter maps it to [Typst](https://typst.app)
markup, and an embedded Typst compiler produces the PDF. The fonts ship inside the binary,
so the same markdown compiles to the same PDF on every machine.

## Install

```console
$ cargo build --release
$ ./target/release/md2pdf --help
```

## Try it

`samples/article.md` is a ready-made document that exercises everything the tool
supports. Convert it and open the result:

```console
$ ./target/release/md2pdf samples/article.md
$ open samples/article.pdf
```

Then change `columns` in its frontmatter from `2` to `1` and convert it again, to see
the same text across the full width of the page.

## Use

```console
$ md2pdf paper.md                 # writes paper.pdf
$ md2pdf paper.md -o report.pdf   # writes report.pdf
$ md2pdf paper.md --emit-typst    # prints the generated Typst source
```

Without `-o`, the PDF lands at the input path with a `.pdf` extension.

`--emit-typst` prints the Typst source instead of compiling it. That output imports
`template.typ`, which exists only inside the compiler's virtual filesystem, so it serves
inspection rather than a standalone `typst compile`.

## What the markdown may contain

This release supports **headings, paragraph text, the inline constructs, the block
constructs, links, tables, images, footnotes, and strikethrough**:

````markdown
# Introduction

Body text with *emphasis*, **strong emphasis**, ~~struck text~~, and `inline code`.

A hard line break ends this line,\
and this line follows it in the same paragraph.

An [inline link](https://typst.app), a bare autolink <https://typst.app>, and
an email address at <you@example.com>.

- a bullet list
- whose second item nests one
  - like this

3. an ordered list that starts at three
4. and keeps counting

```rust
fn main() {}
```

> A block quote.

| Construct | Supported |
| --------- | :-------: |
| a table   | yes       |

![A diagram of the three steps](figures/pipeline.svg)

A claim that needs a source[^1].

[^1]: The source, at the foot of the column.

---

## Background

Text after a thematic break.
````

Heading levels 1 to 6 map to Typst headings of the same level. A list whose items are
separated by blank lines is set with more space between them than one whose items are
not, which is the distinction markdown itself draws.

A table's header row is set in bold, and its columns take the alignment the delimiter row
gives them. A row with too few cells is padded with empty ones, and a row with too many
loses the extra, which is what GitHub-flavoured markdown does.

A reference link works too, and an email autolink becomes a `mailto:` destination. Two
link shapes do not: a link with an empty destination, `[text]()`, and a link that carries
a title, `[text](url "a title")`. Neither Typst nor the PDF can hold a title, and an empty
destination has nothing to resolve to.

Strikethrough takes one tilde on each side as well as two, so `~struck~` strikes exactly
as `~~struck~~` does. A single tilde with a space beside it is still a tilde.

**Every other construct is an error.** A block of raw HTML makes `md2pdf` exit
with code 1 and print the construct and its line:

```console
$ md2pdf notes.md
error: unsupported markdown construct 'raw HTML block' at line 5
```

That is deliberate. Dropping or flattening content would ship a PDF that lies about its
source, so the tool names what it cannot yet handle. Support arrives construct by
construct.

Math is an error in both its forms, `$x$` and `$$x$$`, and a task list item, `- [ ] a`,
is one too. If you meant a dollar sign rather than a formula, write `\$`: the backslash
stops the span, and the dollar reaches the page as itself.

```console
$ md2pdf paper.md
error: unsupported markdown construct 'math' at line 12
```

Body text reaches the PDF verbatim. Characters that Typst would otherwise interpret are
escaped for you, so a `$5` that markdown does not read as a formula stays five dollars:

``` 
\  #  $  *  _  `  @  <  >  [  ]  ~  -  +  =  /
```

Code reaches the PDF verbatim too, by a different route: its content travels as a string
rather than as markup, so nothing inside a pair of backticks or a fence is escaped and
nothing needs to be. ``` `` #5 $5 \ `x` `` ``` prints exactly those characters. A fence's
language tag is carried through, so Typst highlights the block. A link's destination takes
that same route, so a `#` fragment in a URL arrives intact.

## Images

An image points at a file that travels with the document:

```markdown
![The three steps, drawn as boxes](figures/pipeline.svg)

A small icon ![a check mark](check.svg) sits inside this sentence.
```

The path is relative to the markdown file, and `md2pdf` reads the file from there. An
image alone in its paragraph is set as a block and scales down to the column; an image
with text beside it stays in the line. Eight formats work, and the extension decides
which one a file holds:

```
png  jpg  jpeg  gif  webp  svg  svgz  pdf
```

The text in the square brackets is alt text. It reaches the accessibility layer of the
PDF rather than the page, so it is not a caption and nothing numbers it. There is no
syntax for a width, a rotation or a crop; the image takes its natural size, bounded by
the column.

A file that `md2pdf` cannot read is an error that names the path, the line that asked
for it, and the reason:

```console
$ md2pdf paper.md
error: cannot read figures/pipeline.svg for the image at line 12: No such file or directory (os error 2)
```

Bytes that disagree with their extension are an error too. So are four destinations:
a URL and a `data:` URI, because nothing is fetched over the network; an absolute path,
which converts on one machine only; and a path with a `..` segment, which escapes the
document's own folder.

## Footnotes

A reference in the text puts its note at the foot of the column that holds it:

```markdown
A claim that needs a source[^src], and the same source again[^SRC].

[^src]: The note itself, which may hold *emphasis*, `code` and a second
    paragraph.
```

The definition may sit anywhere in the file, above or below the reference. A label is
matched without regard to case, so `[^SRC]` and `[^src]` are one note, and the numbers
are Typst's: they run in the order the notes appear on the page, and a note cited twice
carries one number.

Three shapes are errors. A definition that no reference cites would reach no page. A
second definition for one label would lose a body. A reference inside a definition would
put a footnote inside a footnote. A reference whose definition is missing altogether is
not an error: it stays on the page as the text you typed, the way an unresolved link
reference does.

## Frontmatter

A leading `---` block controls the layout. It takes three keys, all optional:

```markdown
---
title: A Minimal Example
author: Iva Po
columns: 1        # 1 or 2; the default is 2
---
```

Without a `title` and an `author`, the PDF gets no title block. Without `columns`, it
gets two columns. Without the block altogether, it gets both defaults.

A key outside those three, or a `columns` value other than `1` or `2`, is an error that
names the key and its line:

```console
$ md2pdf paper.md
error: frontmatter error at line 3: unknown key 'subtitle'
```

The block is a small YAML subset, not full YAML: one `key: value` pair per line, blank
lines and `#` comments skipped, and one pair of quotes stripped from a value. Nesting
and lists are errors.

## Styling

`core/assets/template.typ` owns all styling: the page setup, the text font, the code
font, the heading style, the rule a thematic break draws, the title block, and the column
count. Change that file to change the look. The parser and the emitter do not need to
know.

## Licence

The code is MIT. The bundled fonts are under the SIL Open Font Licence — Libertinus Serif
in four faces for body text, and Libertinus Mono for code. See
`core/assets/fonts/OFL.txt`.
