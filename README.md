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

The desktop app builds into a real macOS application:

```console
$ cargo tauri build
```

That writes `target/release/bundle/macos/md2pdf.app`, and a `.dmg` beside it under
`target/release/bundle/dmg/`. Drag the `.app` into `/Applications` and launch it from
there; double-clicking a `.md` file opens it too.

**The bundle is not signed.** Copy it over — a USB stick, `scp`, a shared folder — and
it runs. Download it or send it by AirDrop and macOS marks it quarantined, and
Gatekeeper refuses it until you allow it by hand in System Settings → Privacy &
Security. Signing and notarising it needs an Apple Developer account, which this build
does not have.

## Try it

`samples/article.md` is a ready-made document that exercises everything the tool
supports. Convert it and open the result:

```console
$ ./target/release/md2pdf samples/article.md
$ open samples/article.pdf
```

Then change `columns` in its frontmatter from `2` to `1` and convert it again, to see
the same text across the full width of the page.

`samples/press-release.md` is the same tool in its second look. Convert it the same way,
and compare the two pages:

```console
$ ./target/release/md2pdf samples/press-release.md
$ open samples/press-release.pdf
```

## Use

```console
$ md2pdf paper.md                 # writes paper.pdf
$ md2pdf paper.md -o report.pdf   # writes report.pdf
$ md2pdf paper.md --emit-typst    # prints the generated Typst source
```

Without `-o`, the PDF lands at the input path with a `.pdf` extension.

`--emit-typst` prints the Typst source instead of compiling it. That output imports the
look the frontmatter chose, which exists only inside the compiler's virtual filesystem, so
it serves inspection rather than a standalone `typst compile`.

## The desktop app

There is a second front end: a macOS window that shows the PDF while you write it. It
wraps the same core crate, so it converts exactly what the command converts and refuses
exactly what the command refuses, in the same words.

```console
$ cargo tauri dev
```

That opens a window. Press `⌘O`, or the Open button, and pick a markdown file: the
window puts its text in the left pane and draws the page on the right. `cargo tauri dev`
needs the Tauri CLI (`cargo install tauri-cli`); without it,
`cargo run --release -p md2pdf-app` opens the same window and skips the
rebuild-on-change.

**Type in the left pane and the page follows.** It redraws when you stop typing, and
**the PDF is what the pane says, not what the file says** — so the page shows your
unsaved work. `⌘S`, or the Save button, writes the pane back to the file. Drag the
divider to give either side more room.

**Save the file in another program and the page redraws too** — with one exception. The
window watches the folder the document sits in, so editing a figure elsewhere redraws
it as well. If the pane holds unsaved edits when the file changes underneath, the app
keeps your text and says so rather than choosing for you: save to write the pane over
the file, or open the file again to take it. It never merges the two.

A document that will not compile leaves the last good page on screen, dimmed, with the
error above it — the same sentence the command prints — and the page comes back when you
fix it.

**A redraw opens the page on the heading you are writing under.** The pane is a real PDF
view and tells the app nothing about where you scrolled it, so it follows your cursor
instead — to the nearest heading above it, which is as close as it can get without one.
Opening a file, and taking one that changed underneath, still start you at page 1.

The header says where the page stands — `current` with the time the compile took, or
`stale` when the last one failed and the page you are looking at is the older one.

**`File → Save a Copy…`, or `⇧⌘S`, writes the PDF where you ask**, offering the document's
own path with a `.pdf` extension. It writes the page on screen and compiles nothing, so
the file and the page cannot disagree, and it is byte for byte the file `md2pdf` writes
for the same document — while the pane and the file say the same thing, which they do
until you type. A page that is stale, or no page at all, is refused rather than written.

**A `.md` file double-clicked in Finder opens in the app**, once it is the handler for
that extension. macOS gives an installed editor the first claim on `.md`, so if
double-clicking still opens your editor, pick a markdown file, press `⌘I`, and set
*Open With* to md2pdf followed by *Change All*. Opening a second file this way switches
the window to it, and **unsaved edits in the pane are lost** — the same as reopening
from the Open dialog. Save first if you want to keep them.

The app opens one file at a time. The Install section above has the build command and
the one thing an unsigned bundle cannot do.

## What the markdown may contain

This release supports **headings, paragraph text, the inline constructs, the block
constructs, links, tables, images, footnotes, strikethrough, and math in both its forms**:

````markdown
# Introduction

Body text with *emphasis*, **strong emphasis**, ~~struck text~~, and `inline code`.

A formula in the running text, $\sum_{i=1}^{n} i = \frac{n(n+1)}{2}$, typesets as math,
and one written between double dollars is set as a block of its own:

$$
\int_{-\infty}^{\infty} e^{-x^2}\,dx = \sqrt{\pi}
$$

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

A task list item, `- [ ] a`, is an error for now.

A formula takes one of two forms. `$…$` sets in the running text, and `$$…$$` sets as a
block of its own — wherever you write it, so a `$$…$$` in the middle of a sentence breaks
that sentence around it, which is what the double dollars ask for. If you meant a dollar
sign rather than a formula, write `\$`: the backslash stops the span, and the dollar reaches
the page as itself.

A document that refers to its own formulas can number them, with `equations: numbered` in
the frontmatter. The numbers run `(1)`, `(2)` down the page and land on the display form
only, so an inline `$…$` never takes one. One `$$…$$` span is one equation whatever it
holds: a multi-line `aligned` derivation is numbered once, against its lines, rather than
once per line. There is no way to *label* an equation and refer to it by name — "see
equation (1)" is prose you keep true yourself.

A formula is **LaTeX**, and the dialect accepts a bounded subset of it — the Greek
letters, the relations and operators, sums, products and integrals, `\frac`, `\sqrt`,
`\binom`, `\left`/`\right`, the accents, the font commands, the named operators, and the
`matrix`, `pmatrix`, `bmatrix`, `vmatrix`, `cases` and `aligned` environments. A command
outside that list is an error naming the command and its line, rather than a formula that
quietly loses part of itself:

```console
$ md2pdf paper.md
error: math error at line 12: unsupported command '\text'
```

Two things that follow from it. `\text{…}` is not on the list yet, so prose inside a
formula is written beside it instead — `$100\%$ of the sample` rather than
`$100\% \text{ of the sample}$`. And a `%` inside a formula must be written `\%`, because
LaTeX reads a bare one as a comment and would drop the rest of the line.

The formula is LaTeX and not Typst's own math syntax, which matters because Typst's is
close enough to look like it should work: `$frac(a,b)$` is not an error, it just sets as
the letters `f r a c (a, b)`. Write `$\frac{a}{b}$`.

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

A leading `---` block controls the layout. It takes six keys, all optional:

```markdown
---
title: A Minimal Example
author: Iva Po
date: 10 August 2026        # a free string, typeset as you wrote it
template: article           # article or press-release
columns: 1                  # 1 or 2
equations: numbered         # numbered or plain
---
```

`template` picks the look:

| Name | The look | Columns without a `columns` key |
| --- | --- | :---: |
| `article` | the default: a centred title block, and the date under the author | 2 |
| `press-release` | a dateline above a flush-left title, over a rule | 1 |

Each look brings its own column count, so a press release is a single column without
saying so. A `columns` key of your own beats it.

`date` is your text and nothing else. `md2pdf` never reads a clock, so the same file
makes the same PDF on every machine and on any day.

`equations: numbered` numbers the document's display formulas, `(1)`, `(2)`, down the
page. `plain` is the default, so a file that says nothing gets no numbers and reads
exactly as it did before. You say *whether*; the look says *how* — the format, and where
on the line it sits. Both bundled looks write `(1)`.

Without `title`, `author` and `date` together, the PDF gets no title block. Without the
frontmatter altogether, it gets every default.

A key outside the six, a `columns` value other than `1` or `2`, or a `template` or
`equations` name outside its set, is an error that names the key and its line:

```console
$ md2pdf paper.md
error: frontmatter error at line 3: unknown key 'subtitle'
$ md2pdf release.md
error: frontmatter error at line 3: key 'template' takes article or press-release, not 'ieee'
```

The block is a small YAML subset, not full YAML: one `key: value` pair per line, blank
lines and `#` comments skipped, and one pair of quotes stripped from a value. Nesting
and lists are errors.

## Styling

A look owns all styling: the page setup, the text font, the code font, the heading style,
the rule a thematic break draws, the title block, and the column count. Two ship —
`core/assets/template.typ` is `article` and `core/assets/press-release.typ` is
`press-release`. Change one of those files to change how its look reads. The parser and
the emitter do not need to know.

A third look is a third `.typ` file plus one name in `core/src/frontmatter.rs`. It has one
contract to meet: export `template` and `divider`, and let `template` take `title`,
`author`, `columns`, `date` and `equations` before its trailing document argument.
`md2pdf` names all five on every call.

## Licence

The code is MIT. Most of the bundled fonts are under the SIL Open Font Licence —
Libertinus Serif in four faces for body text, and Libertinus Mono for code; see
`core/assets/fonts/OFL.txt`. The math font, NewCMMath-Regular, is under the GUST Font
Licence; see `core/assets/fonts/GUST-FONT-LICENSE.txt`.
