# The showcase

One document that uses **every construct md2pdf supports**, so the whole surface
can be read on a page at once instead of assembled from the README. It is
written across six files, because that is one of the constructs.

```console
$ md2pdf showcase.md
$ open showcase.pdf
```

| File | What it is |
| --- | --- |
| `showcase.md` | the **master** — all eleven frontmatter keys, the abstract, the keywords beside it, what the document is, and five markers |
| `sections/*.md` | the five sections, in the order the master names them |
| `sections/*.svg` | the four figures, beside the sections that draw them |
| `refs.bib` | the bibliography its citations resolve against, entirely invented |

Everything here sits in this folder. A path in this dialect may not be a URL, an
absolute path, or reach through a `..` segment — a document and the files it
names travel together — so the figures and the bibliography are beside the
document rather than shared with the samples above.

**A section's neighbours are its own**, which is why the figures sit one level
down: `![a mark](mark.svg)` written in `sections/text.md` means
`sections/mark.svg`, so `sections/` could be moved or shared whole. The
bibliography stays at the top, because it is the master's frontmatter that names
it and only the master carries frontmatter.

**Nothing in `refs.bib` is real.** The people, titles, journals and publishers
are invented, so that a sample bibliography makes no claim about any actual
work.

Change `template: article` to `template: press-release` in the master's
frontmatter and convert again to read the same text in the other bundled look.
