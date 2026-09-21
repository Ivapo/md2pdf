---
title: An entity-relationship diagram
---

# What a master names

```mermaid
erDiagram
    MASTER ||--o{ SECTION : names
    MASTER ||--o| BIBLIOGRAPHY : names
    SECTION ||--o{ IMAGE : names
```

: What a master names, and what its sections do.
