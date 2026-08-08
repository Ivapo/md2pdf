---
title: <subsystem-slug>
sources:                       # regenerate by re-reading exactly these.
  - path/to/source.ext         # [] ⇒ hand-maintained, deliberately — this is
                               # the signal, not `generated` below
covers: >                      # what this file is FOR — the regeneration target.
  <the behaviour this rule documents, in one line>   # null where hand-maintained
max_lines: 50                  # the cap. Body lines only; the frontmatter is free
generated: YYYY-MM-DD          # when the loop last ran. null ⇒ never
---

# <Subsystem>

**What is true right now.** This file tracks the code, so it is corrected freely —
there is no audit trail to protect here and no dated correction note. Freely of
ceremony, not from a blank file: the regeneration loop verifies what is here
against `sources` and corrects what has drifted, so a fact no source states — an
outcome, a closed question, a measurement — survives the pass. If it disagrees
with the code, the file is wrong.

Keep it under the cap. A cap holds only where something regenerates against it,
which is what `sources` and `covers` are for: they let the regeneration loop
re-derive this file without knowing anything about this project in advance.

There are three managed states, and **`sources` is what tells them apart**:
non-empty with a `generated` date is *generated*; non-empty with `generated: null`
is *declared but never regenerated*, which is what adopting this in an existing
project produces; and `sources: []` with `covers: null` is **declared**
hand-maintained — a different thing from silently unmaintained, and the whole
reason the keys are required rather than optional. A file with no frontmatter at
all is a fourth thing, *unmanaged*, and that is an error.
