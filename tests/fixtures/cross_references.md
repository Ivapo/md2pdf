# Cross-references

A name on a caption line makes the figure that caption makes referable, and a
link with no text points at it. The number is Typst's, so it stays true when
anything moves.

![The three steps, drawn as boxes](dot.png)

: The conversion pipeline. {#fig:pipeline}

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

: The constructs and their *counters*. {#tab:counters}

```rust
fn main() {
    println!("hello");
}
```

: The entry point. {#lst:main}

As [](#fig:pipeline) shows, the emitter sits in the middle; the counters are
set out in [](#tab:counters), and the program that drives it is [](#lst:main).

A reference ends this sentence, as it does in [](#fig:pipeline). The prose
carries on afterwards, which is the shape ordinary writing puts one in.

A reference may stand above the figure it names, so [](#fig:later) resolves
against a caption further down the page.

![A check mark](mark.svg)

: The mark this paragraph named before it stood. {#fig:later}

The prefix is the author's own convention and the dialect neither requires nor
reads it. This image is named without one:

![The three steps again](dot.png)

: A figure named with no prefix at all. {#pipeline}

and this one is named with a table's:

![A check mark again](mark.svg)

: A figure named with a table's prefix. {#tab:pipeline}

Both are figures, and [](#pipeline) and [](#tab:pipeline) say so.

A link that carries text is the link it has always been, whatever its
destination: [some words](#fig:pipeline) is one, and so is [ ](#fig:pipeline),
whose text is a space rather than nothing.

A name declared inside a footnote definition is a declared name[^inside], and
[](#fig:note) reaches it from out here.

$$
a^2 + b^2 = c^2
$$

: This paragraph follows a display equation, which takes no name and no number
of its own, so the marker reaches the page as the prose it is.

[^inside]: The definition holds a figure of its own.

    ![The three steps, a third time](dot.png)

    : The figure inside the note. {#fig:note}
