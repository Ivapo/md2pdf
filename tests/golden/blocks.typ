#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain")

= Block constructs

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

#raw(block: true, lang: "rust", "fn main() {\n    println!(\"hello, \\\"world\\\"\");\n}")

An indented code block, which carries no language tag:

#raw(block: true, "#raw text $ that no escape touches\na second line")

A block quote:

#quote(block: true)[
  A quoted paragraph.

  A second quoted paragraph.
]
