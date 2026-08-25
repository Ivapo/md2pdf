# Blocks

## Lists

- a bullet list
- whose second item nests one
  - like this
  - and this
- and whose last item does not

3. an ordered list that starts at three
4. and keeps counting from there
5. because every item carries its own number

A list with blank lines between its items is a loose one, and it sets further
apart:

- the first item, standing on its own

- the second item, standing on its own

Tightness passes through structurally, so md2pdf decides nothing about the
spacing — the blank lines you left are what set it.

## Quotes and rules

> A block quote, which may hold *emphasis* and `code` like any other text.

A thematic break draws the rule below, and the look decides its weight and its
spacing:

---

## Code

A fenced block takes the first word of its info string as the language:

```rust
fn main() {
    println!("hello");
}
```

An indented block takes none, and is set the same way:

    the four spaces in front of this line
    are what make it a code block

## Tables

The delimiter row sets each column's alignment, and the header row is repeated
across a page break:

| Construct | In the dialect | Counter  |
| --------- | :------------: | -------: |
| a table   | yes            | its own  |
| an image  | yes            | its own  |
| a listing | yes            | its own  |
| raw HTML  | no             | none     |

: The three kinds a caption reaches, and the counter each one keeps.
  {#tab:kinds}

That caption is what makes the table above a *Table*, and the number it carries
is the section's, because the master asks for `figures: sectioned`.
