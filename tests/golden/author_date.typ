#import "template.typ": template, divider
#show: template.with(title: "Reading in the sentence", author: none, affiliation: none, columns: 2, date: none, equations: "plain", figures: "flat", headings: "plain", citations: "author-date")

= Author and date

The parenthetical mark names the source and its year #cite(label("one")), and the same key
cited in the collapsed form #cite(label("two")) reaches the same entry.

A citation may read in the sentence. As #cite(label("two"), form: "prose") showed, the method holds, and
the year alone follows a name the prose already carries: Postigo argued this
in #cite(label("one"), form: "year").

Two sources share one parenthesis when their keys share one bracket with a
semicolon between them #cite(label("three"))#cite(label("one")), and a source with four authors is
shortened by the style, as #cite(label("four"), form: "prose") found.

#bibliography("author_date.yml", title: none)
