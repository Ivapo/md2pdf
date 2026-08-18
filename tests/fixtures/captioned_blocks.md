# Captions on blocks

One mechanism over three constructs, each keeping a counter of its own. This
document carries one of each, so the three numbers can be read apart.

![The three steps, drawn as boxes](dot.png)

: The conversion pipeline.

| Construct | Counter |
| --------- | :-----: |
| a table   | its own |

: The constructs and the *counters* they keep.

```rust
fn main() {
    println!("hello");
}
```

: The entry point, with a `raw` span in its caption.

A caption reaches the construct above it and no other. The table below carries
none and stays the bare block it has always been; the listing after it takes
the caption beneath them both.

| Uncaptioned | Table |
| ----------- | ----- |
| stays       | bare  |

```rust
fn second() {}
```

: The listing above this line, and not the table above that.
