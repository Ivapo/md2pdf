# md2pdf

Convert one markdown file into one typeset PDF. Everything happens on your machine —
no server, no SaaS, and no LaTeX toolchain.

`pulldown-cmark` parses the markdown, a small emitter maps it to Typst markup, and an
embedded Typst compiler produces the PDF. The fonts ship inside the binary, so the same
markdown compiles to the same PDF on every machine.

## Install

```console
$ cargo build --release
$ ./target/release/md2pdf --help
```

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

This release supports **headings and paragraph text**:

```markdown
# Introduction

Body text in an article look.

## Background

More text.
```

Heading levels 1 to 6 map to Typst headings of the same level.

**Every other construct is an error.** A list, emphasis, a code block, a link, an image,
a table, or a block quote makes `md2pdf` exit with code 1 and print the construct and its
line:

```console
$ md2pdf notes.md
error: unsupported markdown construct 'bullet list' at line 5
```

That is deliberate. Dropping or flattening content would ship a PDF that lies about its
source, so the tool names what it cannot yet handle. Support arrives construct by
construct.

Body text reaches the PDF verbatim. Characters that Typst would otherwise interpret are
escaped for you, so `$5` stays five dollars and never opens math mode:

``` 
\  #  $  *  _  `  @  <  >  [  ]  ~  -  +  =  /
```

A leading `---` frontmatter block is **ignored**, with a warning on stderr. The next
release parses it and uses `title`, `author`, and `columns` to control the layout.

## Styling

`core/assets/template.typ` owns all styling: the page setup, the text font, and the
heading style. Change that file to change the look. The parser and the emitter do not
need to know.

## Licence

The code is MIT. The bundled Libertinus Serif fonts are under the SIL Open Font Licence —
see `core/assets/fonts/OFL.txt`.
