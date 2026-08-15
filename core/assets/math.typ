// The symbols `mitex` writes that Typst does not define, and nothing else.
//
// This file is this project's own, written against measured converter output
// rather than copied from the `@preview/mitex` package: that package resolves
// through Typst's registry, which is a network fetch, and it defines ~168 names
// for the whole of what MiTeX can emit. The dialect accepts a closed list of
// LaTeX commands instead, and this is the remainder that list needs.
//
// The membership is derived, not chosen: convert every command the dialect
// allows, take the multi-character heads of the output, subtract what Typst's
// global, `math` and `sym` scopes define, and define what is left. The set moves
// with the Typst version — `sect` and `diff` are here only because mitex 0.2.4
// writes the pre-0.13 spellings of `inter` and `partial` — so the fixture that
// sets one formula per allowed command is what proves this file complete, and a
// Typst upgrade that changes the answer fails that test rather than a reader's
// document.
//
// The emitter imports these by name, and only into a document that has math.

// The matrix environments. mitex writes the cells positionally with `;` between
// rows, which is the shape `mat` already takes; only the delimiter differs.
#let matrix(..rows) = math.mat(delim: none, ..rows)
#let pmatrix(..rows) = math.mat(delim: "(", ..rows)
#let bmatrix(..rows) = math.mat(delim: "[", ..rows)
#let vmatrix(..rows) = math.mat(delim: "|", ..rows)

// `aligned` carries alignment points and line breaks that Typst already reads
// inside an equation, so the environment contributes grouping and nothing else.
#let aligned(body) = body

// `\sqrt{x}` converts to one argument and `\sqrt[n]{x}` to two, the index first.
#let mitexsqrt(..args) = if args.pos().len() == 2 {
  math.root(args.pos().at(0), args.pos().at(1))
} else {
  math.sqrt(..args)
}

// `\mathbf` is bold upright, which is what LaTeX sets and what Typst's own
// `bold` alone would not give.
#let mitexmathbf(body) = math.bold(math.upright(body))

// Version skew rather than MiTeX helpers: these two look like ordinary Typst
// symbols, which is exactly why a hand-written prelude omits them.
#let sect = math.inter
#let diff = math.partial

// `\!`, LaTeX's negative thin space: three eighteenths of an em, backwards.
#let negthinspace = h(-0.16667em)
