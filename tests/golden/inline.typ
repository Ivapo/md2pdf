#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat")

= Inline constructs

A sentence with #emph[emphasis], #strong[strong emphasis], #emph[#strong[both at once]], and
#raw("inline code") in it.

Typst's own delimiters are word\-boundary sensitive, so intraword emphasis
like foo#emph[bar]baz needs the function form. A foo\_bar\_baz word keeps both
underscores, because CommonMark does not read those as emphasis.

A hard line break ends this line,\
and this line follows it in the same paragraph.

#divider()

Text after the thematic break.
