# md2pdf

Convert one markdown file — or a master and the sections it names — into one typeset
PDF. Everything happens on your machine: no server, no SaaS, and no LaTeX toolchain.

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

**With nothing installed: [ivapo.github.io/md2pdf](https://ivapo.github.io/md2pdf/).** The
same crate compiled to WebAssembly, converting in the page — it sets out what this dialect
adds to markdown, and every example on it is one this repository's tests compile and one
click away from a PDF in your own browser, the three refusals included. Nothing is sent
anywhere, and there is no server to send it to.

Locally, **`samples/showcase/` is one document that uses every construct in the dialect** —
every inline and block form, captions, groups, names and cross-references, both forms of
math, footnotes, and citations against the fake bibliography beside it, under all eight
frontmatter keys. It is the fastest way to see the whole surface set on a page:

```console
$ ./target/release/md2pdf samples/showcase/showcase.md
$ open samples/showcase/showcase.pdf
```

`samples/article.md` is a shorter ready-made document, and the one to start from. Convert
it and open the result:

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
exactly what the command refuses, in the same words. **One exception, for now: it does
not yet open a document written in several files** — a master opened here says which
section it could not find. Convert one from the command line until it does.

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
window watches the folder the document sits in, so editing a figure or the bibliography
elsewhere redraws it as well. If the pane holds unsaved edits when the file changes
underneath, the app keeps your text and says so rather than choosing for you: save to
write the pane over the file, or open the file again to take it. It never merges the two.

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
constructs, links, cross-references, citations, include markers, tables, images, captions,
figure groups, footnotes, strikethrough, and math in both its forms**:

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

: What this release carries.

![A diagram of the three steps](figures/pipeline.svg)

: The conversion pipeline, with the *emitter* in the middle. {#fig:pipeline}

As [](#fig:pipeline) shows, the emitter sits in the middle.

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

One shape is not a link at all. `[](#name)` — **with nothing between the brackets** — is a
cross-reference to a figure or an equation you have named, and the naming section below
covers it. Any
link that has text is the link it has always been, whatever its destination, so
`[Introduction](#introduction)` still points where it always did.

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
once per line. Put `{#eq:name}` after the closing `$$` and you can point at that formula
by name, the way you point at a figure — see the section below.

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

The path is relative to the markdown file that names it, and `md2pdf` reads the file from
there — in a document written across several files, that is the section the image was drawn in
and not the master. An image alone in its paragraph is set as a block and scales down to the
column; an image with text beside it stays in the line. Eight formats work, and the extension
decides which one a file holds:

```
png  jpg  jpeg  gif  webp  svg  svgz  pdf
```

The text in the square brackets is alt text. It reaches the accessibility layer of the
PDF rather than the page, so it is never used as a caption. There is no syntax for a
width, a rotation or a crop; the image takes its natural size, bounded by the column.

An image alone in its paragraph can carry a caption, which is what turns it into a
numbered figure. The next section covers that, for a table and a code block as well.

A file that `md2pdf` cannot read is an error that names the path, the line that asked
for it, and the reason:

```console
$ md2pdf paper.md
error: cannot read figures/pipeline.svg for the image at line 12: No such file or directory (os error 2)
```

Bytes that disagree with their extension are an error too. So are four destinations:
a URL and a `data:` URI, because nothing is fetched over the network; an absolute path,
which converts on one machine only; and a path with a `..` segment, which escapes the
document's own folder — a section's included, so a section cannot reach up out of the
folder it sits in.

## Several files

A document long enough that one file has stopped being comfortable can be written as
several. A **master** carries the frontmatter and names its sections in the order they
are read:

```markdown
---
title: A Long Report
author: Iva Po
figures: sectioned
---

[](sections/introduction.md)

[](sections/method.md)

[](sections/results.md)
```

The sections are ordinary markdown files with no frontmatter of their own:

```console
$ md2pdf report.md          # writes report.pdf, out of all four files
```

**A link with no text pointing at a `.md` file is an include**, and it has to be a
paragraph of its own. That is the same shape a cross-reference uses — `[](#fig:one)` —
and the empty text is what makes it one. A link that carries text is an ordinary link
whatever it points at, so `[the method](sections/method.md)` still links.

**It is one document, not three PDFs stapled together.** The files are joined before
anything is parsed, so a figure declared in the first is *Figure 1.1*, a `[](#fig:it)`
in the third reads its number, and a footnote defined anywhere lands at the foot of the
column that cites it. Numbering, references, footnotes and citations run continuously
because nothing in the pipeline was ever written against a file.

**Only the master carries frontmatter.** That is what makes the title, the look, the
column count, the numbering scheme and the bibliography one set of answers rather than
several to reconcile. A section that opens with a `---` line is an error naming that
file, and so is a section that names a section of its own — the master is the one place
the order lives.

Every error names the file you wrote it in:

```console
$ md2pdf report.md
error: math error in sections/method.md at line 12: unsupported command '\includegraphics'
```

**A section's neighbours are its own.** An image named inside `sections/method.md` is
looked for beside *that file*, so `![A figure](figure.png)` there means
`sections/figure.png`. A chapter folder holding its own figures can be moved, copied or
shared whole — moving it means editing one line, the master's marker, which is the line
that exists to say where a section is.

## Captions

A paragraph of its own beneath an image, a table or a code block, opening `: `, is that
block's caption:

````markdown
![The three steps, drawn as boxes](figures/pipeline.svg)

: The conversion pipeline, with the *emitter* in the middle.

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

: The constructs and the counters they keep.

```rust
fn main() {}
```

: The entry point.
````

**A caption is what makes a figure.** A block with one is set as a numbered figure —
*Figure 1*, *Table 1*, *Listing 1* — and each kind counts on its own, so inserting a
table above a listing does not renumber the listing. Typst renumbers them all whenever anything moves, and
the caption sets beneath. A block without a caption is the plain block it has always been:
it takes no number and consumes none, so a document written before this release typesets
exactly as it did. The caption is ordinary markdown, so emphasis, `code`, links and
formulas all work inside one. The words "Figure", "Table" and "Listing", the number's
format and the punctuation after it belong to the look, so the two bundled ones differ.

**Leave the blank line above a caption.** Above an image it is required: without it the
two are one paragraph, and the image stays in the line with `: ` printed as text beside
it. Above a table it is required too, because markdown reads a non-blank line after the
last row as one more row, so the marker lands in a cell. Above a code block it is
optional, since a fence ends at its closing fence — but one rule for all three is easier
to keep than three.

Only a standalone image takes a caption; an image inside a sentence is not a figure. A
`: ` paragraph anywhere else — after prose, after a heading, after a list — is ordinary
text, unchanged. Two things are errors, each naming the line: a `: ` with nothing after
it, and a second caption under one block.

## Figures with more than one member

Two images side by side, under one caption and one number, are a **group**. Wrap them in
a pair of `:::` lines and put the caption last, just above the closing one:

```markdown
:::

![The first step](figures/one.svg)

![The second step](figures/two.svg)

: The two halves of the pipeline. {#fig:halves}

:::
```

That is *Figure 1* once, not twice — the members share the caption, the number and the
name, so `[](#fig:halves)` points at the pair. A group takes the three things a caption
takes: a standalone image, a table and a code block. Write as many as you like; they
sit in one row, and how far apart is the look's business.

**A word after the opener is yours, and `md2pdf` does not read it.** `::: figure` and
`::: table` are both fine and both mean the same thing, because the kind comes from what
the members are — two tables are a *Table* whatever you opened them with.

**`:::` at the start of a paragraph is reserved.** Everywhere else it is ordinary text:
inside a sentence, on the second line of a paragraph, and inside a code block, which is
how the example above is written. Leave the blank lines in — a `:::` and the image under
it with no blank line between them are one paragraph, not a group, and `md2pdf` says so
rather than guessing.

Eight things are errors, each naming the line: a group with no caption, one with no
member, a second `: ` line, a `: ` line with a member after it, a `:::` inside a group, a
group you never close, a paragraph starting `:::` that is neither an opener nor a closer,
and anything between the members that is not one of the three:

```console
$ md2pdf paper.md
error: unsupported markdown construct 'figure group with no caption' at line 12
```

## Naming a figure or an equation, and pointing at it

End a caption with `{#name}` to name the figure it makes, then write `[](#name)` to point
at it:

```markdown
![The three steps, drawn as boxes](figures/pipeline.svg)

: The conversion pipeline. {#fig:pipeline}

As [](#fig:pipeline) shows, the emitter sits in the middle.
```

That reads *"As Figure 1 shows…"* on the page, and it stays true. Insert another figure
above it and the sentence reads *"As Figure 2 shows…"* without your touching it, which is
the whole reason to number anything. The name never appears in the PDF; the supplement and
the number come from the look, as the caption's own type does.

**The brackets must be empty.** `[](#name)` is a reference; `[the diagram](#name)` is an
ordinary link, and so is `[ ](#name)`, whose text is a space rather than nothing. That is
what keeps `[Introduction](#introduction)` and every other anchor link meaning what it
always meant.

**A name may hold letters, digits, `-`, `_`, `:` and `.`**, and may not begin with `:` or
`.`. The `fig:`, `tab:` and `lst:` prefixes are a convention and nothing more — the kind
comes from what the caption sits under, so `{#pipeline}` on an image is a figure and so is
`{#tab:pipeline}`. Names beginning `fn-` followed by digits are reserved for footnotes.

Five things are errors, each naming the line: a reference to a name nothing declares, the
same name declared twice, a character outside the set, a reserved name, and a reference to
an equation in a document that did not number its equations. A name is checked here rather
than left to the typesetter, which would otherwise report a name you never typed and no
line at all:

```console
$ md2pdf paper.md
error: name error at line 24: nothing declares the name 'fig:piepline'
```

**A display equation is named on its closing fence**, since it has no caption line to carry
a name:

```markdown
$$
a^2 + b^2 = c^2
$$ {#eq:pythagoras}

As [](#eq:pythagoras) shows, the two shorter sides settle the longest one.
```

That reads *"As Equation 1 shows…"*. The group has to be the whole of what follows the
closing `$$` — `$$…$$ {#eq:one} and more` is the prose it looks like — and an inline `$…$`
takes no name, because only the block form is ever numbered.

**Pointing at an equation needs `equations: numbered` in the frontmatter**, and `md2pdf`
says so with your line if you forget:

```console
$ md2pdf paper.md
error: name error at line 24: 'eq:pythagoras' names an equation, and a document that
points at one must set 'equations: numbered'
```

Naming one is always fine — it is only the reference that needs the key, because an
unnumbered equation has no number for the sentence to say.

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

## Citing sources

Name a bibliography file in the frontmatter and cite a key with `[@key]`. The reference
list is set at the end of the document:

````markdown
---
title: Citing a source
bibliography: refs.yml
---

Typesetting was rethought from the ground up [@DBLP:books/lib/Knuth86a].
````

The file sits beside the document, as an image does, and it is the author's own — either
a Hayagriva `.yml`/`.yaml` file or a BibLaTeX `.bib` one, both of which Typst reads
directly:

```yaml
"DBLP:books/lib/Knuth86a":
  type: book
  title: "Computers and Typesetting, Volume A: The TeXbook"
  author: Knuth, Donald E.
  date: 1986
  publisher: Addison-Wesley
```

The mark and the numbering are Typst's, and the look decides what the label above the list
says and how it is set. Nothing is fetched: no key is resolved against anything on the
network. The file is read for its keys before anything is typeset, so a key it does not
hold is named where you cited it rather than reported by the compiler.

The brackets are required. A bare `@key` is not a citation, because an unbracketed `@` is
load-bearing in ordinary text — an email address is the everyday case — so `a@b.com` and
`@thing` reach the page as you typed them, and so do `[see @k]` and `[a@b.com]`.

Eight things are errors, and each names the line you wrote it on. Four are about the
citation: Pandoc's `[@a; @b]`, `[@k, p. 33]` and `[-@k]` are not in this dialect and are
named rather than guessed at, and a citation in a document that names no bibliography is
an error rather than text on the page:

```
error: citation error at line 7: '@smith2020' is cited and the frontmatter names no bibliography
```

Two more are about the keys. A key the bibliography does not hold is one:

```
error: citation error at line 7: '@smith2020' is cited and the bibliography does not hold it
```

and so is a name your document and your bibliography both use, because Typst's labels are
one namespace. Naming a figure `{#smith2020}` beside a bibliography holding `smith2020` is
fine on its own; it is `[](#smith2020)` that has no way to mean one of them rather than
the other:

```
error: citation error at line 12: 'smith2020' names something in this document and a key in the bibliography, and one reference cannot mean both
```

The last two are about the file, and both name the frontmatter line that declared it: a
bibliography that does not parse, and one whose extension is neither `.yml`, `.yaml` nor
`.bib`.

## Frontmatter

A leading `---` block controls the layout. It takes eight keys, all optional:

```markdown
---
title: A Minimal Example
author: Iva Po
date: 10 August 2026        # a free string, typeset as you wrote it
template: article           # article or press-release
columns: 1                  # 1 or 2
equations: numbered         # numbered or plain
figures: sectioned          # sectioned or flat
bibliography: refs.yml      # a Hayagriva .yml/.yaml or BibLaTeX .bib file
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
page, and is what lets a sentence point at one by name. `plain` is the default, so a file
that says nothing gets no numbers and reads exactly as it did before. You say *whether*;
the look says *how* — the format, and where on the line it sits. Both bundled looks write
`(1)`.

`figures: sectioned` gives a figure, a table and a listing the number of the section it
stands in — *Figure 1.1*, *Table 1.2*, *Table 2.1* — restarting each kind at every `#`
heading and at no other level. The headings themselves stay unnumbered. `flat` is the
default, one counter per kind down the whole document, so a file that says nothing reads
exactly as it did before. You say *whether*; the look says *how*, as with `equations`. A
reference reads whatever the caption reads, so `[](#tab:one)` says *Table 1.1* in a
sectioned document. A display equation is not a figure: under `equations: numbered` it
keeps its `(1)` and takes no section.

Without `title`, `author` and `date` together, the PDF gets no title block. Without the
frontmatter altogether, it gets every default.

A key outside the eight, a `columns` value other than `1` or `2`, or a `template`,
`equations` or `figures` name outside its set, is an error that names the key and its
line:

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
`author`, `columns`, `date`, `equations` and `figures` before its trailing document
argument. `md2pdf` names all six on every call.

Everything else a look decides, it decides over Typst's own elements with `show` and `set`
rules, taking no argument at all. A table's header row, a code block's font, a figure's
caption, the space between a group's members and the left edge a captioned code block sits
on all reach a look that way — which is why neither a caption nor a group widened the call
at all. An argument is added only where the *author* has something to ask for, which is
what `equations` and `figures` are: the two questions a look cannot answer on its own,
because the answer is a fact about the document rather than about the house style.

## Licence

The code is MIT. Most of the bundled fonts are under the SIL Open Font Licence —
Libertinus Serif in four faces for body text, and Libertinus Mono for code; see
`core/assets/fonts/OFL.txt`. The math font, NewCMMath-Regular, is under the GUST Font
Licence; see `core/assets/fonts/GUST-FONT-LICENSE.txt`.
