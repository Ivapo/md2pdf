---
title: Diagrams, Drawn
author: Iva Po
columns: 1
---

# Introduction

This file holds every kind of diagram md2pdf draws, with a little text between
them. Convert it, open the PDF, then change a diagram here and convert it again.

Each diagram is a fenced block tagged `mermaid`, the same spelling GitHub and
GitLab draw, so this file reads the same way on either of them. Five kinds are
drawn: flowcharts, sequence diagrams, class diagrams, state diagrams and
entity-relationship diagrams. Everything is drawn on your machine, inside
md2pdf, with no browser and no network.

A caption on the line after a diagram makes it a figure: numbered with the
images, and referenceable by the name its caption declares. [](#fig:pipeline)
is the first one below, and this sentence found its number by that name.

# Flowcharts

A flowchart is written `flowchart` and a direction: `LR` for left to right,
`TB` for top to bottom. Nodes take their shape from their brackets, and an edge
can carry a label.

```mermaid
flowchart LR
    md[Markdown source] --> parse[pulldown-cmark parses]
    parse --> emit{Emitter maps events}
    emit -->|Typst markup| typst[Typst compiles]
    typst --> pdf[(PDF)]
    emit -.->|rejected| err[Error naming the line]
```

: The pipeline, left to right. {#fig:pipeline}

The look decides how big a diagram is. Its labels set at the size of the
captions, and a diagram is never enlarged. [](#fig:pipeline) is too wide for a
column at that size, so it floats across the whole page, to the top or the foot
of one. A long left-to-right chain gets small that way. When one does, lay it
out top to bottom, as [](#fig:pipeline-down) does, and the same chain stays one
column wide.

```mermaid
flowchart TB
    md[Markdown source] --> parse[pulldown-cmark parses]
    parse --> emit{Emitter maps events}
    emit -->|Typst markup| typst[Typst compiles]
    emit -.->|rejected| err[Error naming the line]
    typst --> pdf[(PDF)]
```
: The same pipeline, top to bottom. {#fig:pipeline-down}

`graph` is the older spelling of `flowchart`, and draws the same way. The
diagram below has no caption, so it is not a figure: it has no number, it sits
exactly where it is written, and it never floats.

```mermaid
graph TD
    start([Open the file]) --> read[/Read the frontmatter/]
    read --> keys{Any keys?}
    keys -->|yes| parse[Parse each key]
    keys -->|no| defaults[Take the defaults]
    parse --> walk[[Walk the body]]
    defaults --> walk
```

# Sequence diagrams

A sequence diagram is written `sequenceDiagram`. Each participant is a column,
each message an arrow between two of them, and a note or an `alt` block says
what the arrows alone cannot.

```mermaid
sequenceDiagram
    participant A as Author
    participant C as CLI
    participant K as md2pdf-core
    A->>C: md2pdf paper.md
    C->>K: image_paths(md)
    K-->>C: figures/plot.svg, line 12
    Note over C: reads each image from disk
    C->>K: md_to_pdf(md, assets)
    alt in the dialect
        K-->>C: PDF bytes
        C-->>A: paper.pdf
    else it is not
        K-->>C: an error naming the line
        C-->>A: exits non-zero
    end
```

: The CLI asks the engine twice. {#fig:sequence}

As [](#fig:sequence) shows, the images are named before anything is compiled,
so the CLI knows which files to read before it asks for the PDF.

# Class diagrams

A class diagram is written `classDiagram`. A generic is written with tildes,
`Vec~u8~`, because an angle bracket would open HTML where Mermaid is drawn in a
browser, and it is typeset with its angle brackets, as `Vec<u8>`.

```mermaid
classDiagram
    class Asset {
        +String path
        +Vec~u8~ bytes
    }
    class ImageRef {
        +String path
        +Location location
    }
    class Location {
        +Option~String~ file
        +usize line
    }
    Asset <.. ImageRef : names
    ImageRef *-- Location
```

: What the caller supplies, and what names it. {#fig:assets}

`classDiagram-v2` is the same diagram under a second keyword, and so is the
figure below. It draws an interface and the two looks that meet it.

```mermaid
classDiagram-v2
    class Look {
        <<interface>>
        +template(doc)
        +divider()
        +diagram(svg, width, label)
    }
    class Article
    class PressRelease
    Look <|.. Article
    Look <|.. PressRelease
```

: The call contract, and the two looks that meet it. {#fig:looks}

# State diagrams

A state diagram is written `stateDiagram`, or `stateDiagram-v2`, which is the
same thing. `[*]` is where it starts and where it ends, and a state can hold
states of its own.

```mermaid
stateDiagram-v2
    [*] --> Walking
    state Walking {
        [*] --> Text
        Text --> Parked: an image opens
        Parked --> Text: the next event settles it
    }
    Walking --> Refused: a construct outside the dialect
    Walking --> Compiled: the last event
    Refused --> [*]
    Compiled --> [*]
```

: The walk, and the two ways it ends. {#fig:walk}

In [](#fig:walk) the walk parks an image because its form is known one event
late: whether it stands alone in its paragraph is only known once the next
event arrives.

# Entity-relationship diagrams

An entity-relationship diagram is written `erDiagram`. Each line joins two
entities, and the marks at each end say how many of each there can be. An
entity can list its attributes, and a key is marked `PK`.

```mermaid
erDiagram
    MASTER ||--o{ SECTION : names
    MASTER ||--o| BIBLIOGRAPHY : names
    SECTION ||--o{ IMAGE : names
    MASTER {
        string title
        int columns
    }
    SECTION {
        string path PK
    }
    IMAGE {
        string path PK
        string alt
    }
```

: What a master names, and what its sections do. {#fig:master}

The relationship labels in [](#fig:master) set a little smaller than the entity
names, because Mermaid draws them smaller.

# Showing Mermaid source instead

To show Mermaid source as code rather than draw it, tag the fence anything
other than `mermaid`, such as `text`, and it sets as a listing:

```text
flowchart LR
    a[Read] --> b[Write]
```

The tag must be exactly `mermaid`, in lower case. A block may not carry its own
configuration: an `init` directive anywhere in it, or front matter opening it,
is an error naming its line, because the look owns how a diagram looks. A
diagram stands at the top level of a file, not inside a list item, a block
quote, a footnote or a figure group.
