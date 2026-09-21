---
title: Two state diagrams
---

# The pending slot

An image's form is known one event late, so the walk parks it and the next event
settles it. The same machine is drawn twice, once under each keyword.

```mermaid
stateDiagram
    [*] --> Walking
    Walking --> Parked: an image
    Parked --> Walking: the next event
    Walking --> [*]
```

: The walk, under `stateDiagram`.

```mermaid
stateDiagram-v2
    [*] --> Walking
    Walking --> Parked: an image
    Parked --> Walking: the next event
    Walking --> [*]
```

: The same walk, under `stateDiagram-v2`.
