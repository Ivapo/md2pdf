#import "template.typ": template, divider, checklist
#show: template.with(title: none, author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", citations: "numeric")

= Task list

#checklist(tight: true, (checked: false, body: [
  an unchecked task
]), (checked: true, body: [
  a checked task
]))

A loose checklist follows.

#checklist(tight: false, (checked: false, body: [
  a loose unchecked task
]), (checked: true, body: [
  a loose task checked with a capital
]))

A plain list nesting a checklist follows.

- a plain item
- a plain item holding a checklist
  #checklist(tight: true, (checked: false, body: [
    a nested unchecked task
  ]), (checked: true, body: [
    a nested checked task
  ]))

A checklist nesting a plain list follows.

#checklist(tight: true, (checked: false, body: [
  a task holding a plain list
  - a nested plain item
  - another nested plain item
]))

A checklist whose body needs escaping follows.

#checklist(tight: true, (checked: false, body: [
  a body carrying \# and \] and #emph[emph]
]))

A checklist whose body wraps follows.

#checklist(tight: true, (checked: true, body: [
  a long task whose single sentence runs on for well over sixty words so that it wraps at least twice in the article's two\-column measure and in the press release's wider one, which is what lets a reader of the page image see whether the wrapped lines hang under the first line of the body or slide back under the box that the look draws in front of it, and that is the whole point
]))
