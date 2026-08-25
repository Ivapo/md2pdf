# Notes and sources

## Footnotes

A reference in the text puts its note at the foot of the column that holds
it[^note], and a label is matched without regard to case, so the same note
cited again[^NOTE] carries one number rather than two.

The definition may sit anywhere in the document, above or below the reference
and in any of its files, and it may hold more than one paragraph[^long].

## Citations

Name a bibliography in the frontmatter and cite a key with `[@key]`. The list
at the end is set from the file, and the mark and the numbering are the
typesetter's [@quill2019].

The brackets are required, because an unbracketed `@` is load-bearing in
ordinary text. Markers and the documents that hold them have been argued over
before [@arden2021], as has the business of numbering things that move
[@olsson2023], and a caption is a subject of its own [@harlow2024].

Nothing is fetched. No key is resolved against anything on the network, and the
file is read for its keys before anything is typeset — so a key it does not
hold is named at the line you cited it on rather than reported by the compiler.

[^note]: The note itself, which may hold *emphasis*, `code` and a
    [link](https://typst.app) like any other text.

[^long]: A note may run to a second paragraph.

    This is that second paragraph, and it is indented under the definition
    rather than marked in any other way.

[typst]: https://typst.app
