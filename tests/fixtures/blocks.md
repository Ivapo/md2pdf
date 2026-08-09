# Block constructs

A bullet list, with one item holding a nested list:

- first item
- second item
  - a nested item
  - another nested item

An ordered list that starts at three:

3. third
4. fourth

A loose list, whose first item holds two paragraphs:

- The first paragraph of a loose item.

  The second paragraph of the same item, which the blank line above makes a
  paragraph rather than a continuation.

- A second item.

A fenced code block, with a language tag:

```rust
fn main() {
    println!("hello, \"world\"");
}
```

An indented code block, which carries no language tag:

    #raw text $ that no escape touches
    a second line

A block quote:

> A quoted paragraph.
>
> A second quoted paragraph.
