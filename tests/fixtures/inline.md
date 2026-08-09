# Inline constructs

A sentence with *emphasis*, **strong emphasis**, ***both at once***, and
`inline code` in it.

Typst's own delimiters are word-boundary sensitive, so intraword emphasis
like foo*bar*baz needs the function form. A foo_bar_baz word keeps both
underscores, because CommonMark does not read those as emphasis.

A hard line break ends this line,\
and this line follows it in the same paragraph.

---

Text after the thematic break.
