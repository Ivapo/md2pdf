---
figures: sectioned
---

# The scheme

A figure number may carry the section it stands in. `figures: sectioned` in the
frontmatter asks for it, and the look decides what one looks like, exactly as it
decides a caption's separator and an equation's format.

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

: The first table of the first section. {#tab:one}

## A subheading takes no number and restarts nothing

The counters restart at a `#` and nowhere else, so the table below carries the
second number of this section rather than the first of a new one.

| Construct | Counter |
| --------- | :-----: |
| a heading | none    |

: The second table of the first section.

Each kind keeps a counter of its own, so the image below is the first figure of
this section however many tables stand above it.

![The three steps, drawn as boxes](dot.png)

: The first figure of the first section.

# The second section

A `#` restarts every kind at once, so the table below is the first table of this
section rather than the third of the document.

| Construct | Counter  |
| --------- | :------: |
| a section | restarts |

: The first table of the second section.

The number a reference reads is the number its caption reads, so [](#tab:one)
names the first table of the first section and carries both halves of it.
