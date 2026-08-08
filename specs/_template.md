---
id: <prefix>-NNN               # stable. never reused, renamed or renumbered
title: <slug>                  # descriptive — content, not identity
note: >                        # one line; this becomes the INDEX.md entry
  <what this spec covers, in one sentence>
status: draft                  # draft | accepted | superseded | withdrawn
last_updated: YYYY-MM-DD       # absolute date, bumped every revision

phases: []                     # the unit of work. Append only; never insert,
                               # renumber or delete. Each entry:
                               #   - name: "Phase 1 — <what it builds>"
                               #     reviewed: null   # date its own round converged
                               #     shipped: null    # date its exit gate passed
                               #     cut: null        # date, if removed
                               #     by: null         # the id that cut it

extends: null                  # a named kind under a framework another spec reserved
supersedes: null               # <id>, or [{id, phases: [...]}] for a partial
superseded_by: null            # set on the OLD spec when replaced whole
related: []                    # see-also. Unconstrained — not a dependency
reference: null                # external inspiration + what is out of scope from it
---

# <Title>

## 1. Goal

What this exists to produce, and for whom. **Name the observable** — the thing a
consumer of this project can see — because §7's round 0 asks whether this spec
produces it, and every phase below has to answer the same question.

### 1.1 Non-goals

What this deliberately does not do. A non-goal is what stops a review round from
turning into a scope-creep loop.

## 2. Design

The decisions, and the argument for each. Ground every claim in source and cite
`file:symbol` — never `file:line`. Where review confirms a non-obvious choice,
record it as a `(decision, recorded)` subsection so a later pass cannot quietly
regress it.

## 3. Open questions

- **OQ-1** — <the question>. *(design call | needs-input | deferred by evidence)*

## 4. Implementation phases

Strictly sequential; each is one plan-mode pass. Each states the observable it
produces — or argues explicitly why it produces none — and each carries an exit
gate concrete enough that someone else could check it.

### Phase 1 — <what it builds>
*Produces the observable: yes | no — <the argument>.*

- **Scope:** <what is built, precisely enough to build it from this alone>
- **Exit gate:** <a checkable condition, not "it works">
- **Close-out:** <the `rules/` file it seeds or updates, and what gets committed>

<!--
The review record is a sibling file, not a section: it lives at
specs/reviews/<id>.md, append-only, one heading per round. See §7.
-->
