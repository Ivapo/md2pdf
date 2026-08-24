# The showcase

One document that uses **every construct md2pdf supports**, so the whole surface
can be read on a page at once instead of assembled from the README.

```console
$ md2pdf showcase.md
$ open showcase.pdf
```

| File | What it is |
| --- | --- |
| `showcase.md` | the document — every construct, under all eight frontmatter keys |
| `refs.bib` | the bibliography its citations resolve against, entirely invented |
| `pipeline.svg`, `parse.svg`, `emit.svg`, `mark.svg` | the four figures it names |

Everything here sits in this folder. A path in this dialect may not be a URL, an
absolute path, or reach through a `..` segment — a document and the files it
names travel together — so the figures and the bibliography are beside the
document rather than shared with the samples above.

**Nothing in `refs.bib` is real.** The people, titles, journals and publishers
are invented, so that a sample bibliography makes no claim about any actual
work.

Change `template: article` to `template: press-release` in the frontmatter and
convert again to read the same text in the other bundled look.
