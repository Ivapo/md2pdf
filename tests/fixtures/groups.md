# Figures with more than one member

A caption attaches to the construct directly above it, so no arrangement of the
`: ` marker says "these two images are one figure". A pair of `:::` lines does.
This document runs one counter past a group and back, so the number a group
takes can be read against the numbers around it.

![The three steps, drawn as boxes](dot.png)

: A single captioned image, which takes the first number.

:::

![The first of a pair](dot.png)

![The second of a pair](dot.png)

: Two images under one caption, side by side. {#fig:pair}

:::

As [](#fig:pair) shows, a group is one figure: one caption, one number, and one
name to point at it with. The image below takes the number after the group's
rather than the number after its last member, which is what a group being one
figure means on the page.

![A check mark](mark.svg)

: A second single image, which takes the third number.

::: table

![The first of another pair](dot.png)

![The second of another pair](mark.svg)

: A group opened with a word, which is still a Figure. {#fig:word}

:::

The word after an opener is the author's own convention and the dialect does
not read it: Typst takes the kind from what the members are, so the group above
is a Figure however it was opened. The two tables below are a Table, on the
same rule and with a counter of their own.

:::

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

| Construct | Counter |
| --------- | :-----: |
| a group   | one     |

: Two tables under one caption, and the *first* Table. {#tab:pair}

:::

A group holds the three constructs a caption reaches, so two code blocks make a
Listing the same way. See [](#tab:pair) for the counters they keep apart.

:::

```rust
fn first() {}
```

```rust
fn second() {}
```

: Two listings under one caption.

:::

The marker is reserved at the first text of a paragraph and nowhere else. A
line reading ::: inside a sentence is prose, and so is one standing later in a
paragraph:
::: not a group.

A fenced block is where a document that documents this syntax puts one, so the
marker reaches the page verbatim there:

````markdown
:::

![The first of a pair](pipeline.svg)

![The second of a pair](check.svg)

: Two images under one caption. {#fig:pair}

:::
````

An indented block is the same arm, so it is the same hatch:

    :::
    ![tight](dot.png)
    :::

Neither of those is a group, and neither raises anything.
