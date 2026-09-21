---
title: Diagrams, sized
---

# Six bands

Six diagrams, each in a band of its own: one that fits its column at caption
size, one kept in its column by the tolerance, two that float across the page,
one that would float but has no caption to be found by, and one wider than the
page itself.

```mermaid
flowchart TB
    a[Read] --> b[Walk] --> c[Write]
```

: The walk, top to bottom.

```mermaid
graph LR
    a[Read the source] --> b[Walk the events] --> c[Write]
```

: The walk, left to right, and a little too wide for its column.

```mermaid
graph LR
    a[Read] --> b[Walk] --> c[Emit] --> d[Compile] --> e[Done]
```

: Five steps, too wide to keep in a column.

```mermaid
sequenceDiagram
    participant A as Author
    participant C as CLI
    participant K as md2pdf-core
    A->>C: md2pdf paper.md
    C->>K: image_paths(md)
    K-->>C: figures/plot.svg, line 12
    C->>K: md_to_pdf(md, assets)
    K-->>C: PDF bytes
    C-->>A: paper.pdf
```

: The CLI asks the engine twice. {#fig:sequence}

As [](#fig:sequence) shows, the images are named before anything is compiled.
The diagram below has no caption, so it stays in its column however small that
makes it.

```mermaid
flowchart LR
    a[Markdown source] --> b[pulldown-cmark parses] --> c[Typst compiles]
```

```mermaid
flowchart LR
    a[Step one of the job] --> b[Step two of the job] --> c[Step three of the job] --> d[Step four of the job] --> e[Step five of the job] --> f[Step six of the job]
```

: Six steps, wider than the page.
