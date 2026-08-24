#import "template.typ": template, divider
#show: template.with(title: none, author: none, columns: 2, date: none, equations: "plain", figures: "flat")

= Captions

#figure(image("dot.png", alt: "The three steps, drawn as boxes"), caption: [The conversion pipeline, with the #emph[emitter] in the middle and a #raw("raw") span.])

A caption is what makes a figure, so an image with none beneath it is the bare
block it has always been. These two are uncaptioned and consecutive:

#image("mark.svg", alt: "a check mark")

#image("dot.png", alt: "The three steps again")

A small icon #box(image("mark.svg", alt: "a check mark")) sits inside this sentence, and the
paragraph below opens with the marker.

: This paragraph follows prose rather than a figure, so it is prose itself and
reaches the page as written.

#box(image("dot.png", alt: "The three steps, drawn as boxes"))
: This line has no blank line above it, so it joins the image's own paragraph
and the image stays inline.

A note whose definition ends with a standalone image#footnote[The last block of this definition is an image.

#image("dot.png", alt: "The three steps, drawn as boxes")]<fn-1>.

#image("dot.png")
