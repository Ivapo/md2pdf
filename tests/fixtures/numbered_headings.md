---
figures: sectioned
headings: 2
---

# First

A heading may carry its number. `headings: 2` in the frontmatter asks for one
and says how deep the numbers go; the look decides what one looks like, exactly
as it decides a caption's separator and an equation's format.

| Construct | Number |
| --------- | :----: |
| a heading | 1      |

: The first table of the first section. {#tab:one}

## Background

A `##` sits inside the cap, so it carries a number of its own — and the table
below carries the second number of this section, because the figure counters
restart at a `#` and nowhere else.

| Construct | Number |
| --------- | :----: |
| a heading | 1.1    |

: The second table of the first section.

### Detail

A `###` sits below the cap and carries no number at all. The `##` after it still
reads 1.2, so a level above the cap costs the levels below it nothing.

## Second Background

The second subheading of the first section, numbered where the `###` above it is
not.

# Second

A `#` takes the next section number and restarts every kind's counter, so the
table below is the first table of this section rather than the third of the
document.

| Construct | Number   |
| --------- | :------: |
| a section | 2        |

: The first table of the second section.

The number a reference reads is the number its caption reads, so [](#tab:one)
names the first table of the first section.
