/* What checks `dist/index.html`. Run it from anywhere: `bun app/typecheck.mjs`.
   `--strict` turns `strictNullChecks` and `noImplicitAny` back on, which is how
   the 243 in `tsconfig.json`'s comment is re-derived.

   **The page is checked through a mirror, because tsc cannot be handed HTML.**
   The mirror does two things and no more, and both are stated here because a
   generator with an unstated rule is a generator nobody can reproduce:

     1. every line outside `<script type="module">` becomes an empty line;
     2. the single relative `pdfjs` specifier becomes the bare name
        `pdfjs-dist`, which `paths` resolves and a relative specifier cannot be
        given types for.

   Both are line-preserving, so the mirror is the same length as the page and
   every error tsc reports cites a real `dist/index.html` line. That is asserted
   below rather than trusted: the line counts must match and every line inside
   the script must be identical, the rewritten specifier alone excepted.

   `new URL('./pdfjs/pdf.worker.min.mjs', import.meta.url)` is left alone — it is
   a string tsc never resolves, and only one of the two vendored modules is
   imported as a module at all.

   **Not `cargo test`.** bun is not a prerequisite of this workspace, and making
   the Rust suite depend on one would charge the whole app's build a node
   toolchain in order to check a file that suite does not otherwise touch. The
   Rust half of this phase — `preview.rs`'s typedef test — does run there, and
   costs nothing.                                                              */

import { mkdirSync, readFileSync, writeFileSync } from 'node:fs'
import { spawnSync } from 'node:child_process'
import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'

const HERE = dirname(fileURLToPath(import.meta.url))
const PAGE = join(HERE, 'dist', 'index.html')
const MIRROR_DIR = join(HERE, '.mirror')
const MIRROR = join(MIRROR_DIR, 'index.mjs')
const PROJECT = join(HERE, 'tsconfig.json')

// Pinned, and the whole reason this script exists rather than a bare `bunx tsc`
// in the workflow: that resolves to whatever is current — 7.x today.
const TYPESCRIPT = 'typescript@5.9.3'

const OPEN = '<script type="module">'
const CLOSE = '</script>'
const RELATIVE = "'./pdfjs/pdf.min.mjs'"
const BARE = "'pdfjs-dist'"

const die = (why) => {
  console.error(`typecheck: ${why}`)
  process.exit(2)
}

const page = readFileSync(PAGE, 'utf8').split('\n')

const only = (what, test) => {
  const found = page.flatMap((line, at) => (test(line) ? [at] : []))
  if (found.length !== 1) die(`expected one ${what}, found ${found.length}`)
  return found[0]
}

const open = only(OPEN, (line) => line.trim() === OPEN)
const close = only(CLOSE, (line) => line.trim() === CLOSE)
if (close < open) die('the script closes before it opens')

// Rule 1. The tags themselves are outside the script and go with everything
// else, so the region kept is strictly between them.
const mirror = page.map((line, at) => (at > open && at < close ? line : ''))

// Rule 2.
const specifier = only(RELATIVE, (line) => line.includes(RELATIVE))
if (specifier <= open || specifier >= close) die('the import is outside the script')
mirror[specifier] = page[specifier].replace(RELATIVE, BARE)

// The rules are line-preserving, and this is where that is a fact rather than a
// claim — it is exit-gate clause 3, run on every invocation.
if (mirror.length !== page.length) {
  die(`the mirror is ${mirror.length} lines against the page's ${page.length}`)
}
for (let at = open + 1; at < close; at++) {
  if (at !== specifier && mirror[at] !== page[at]) die(`line ${at + 1} drifted`)
}

mkdirSync(MIRROR_DIR, { recursive: true })
writeFileSync(MIRROR, mirror.join('\n'))

// A trailing newline leaves a last empty element that `wc -l` does not count,
// and the count quoted in the spec is `wc -l`'s.
const lines = (of) => of.length - (of[of.length - 1] === '' ? 1 : 0)

const middle = open + Math.floor((close - open) / 2)
console.log(`mirror: ${lines(mirror)} lines, as the page has ${lines(page)}`)
console.log(`mirror: line ${specifier + 1} rewritten, ${RELATIVE} -> ${BARE}`)
console.log(`mirror: line ${middle + 1} identical — ${page[middle].trim()}`)

// bun is what the measurement was made with; npx is here because a contributor
// with node and no bun should still be able to run the check, and it pins the
// same way. Neither is a prerequisite of building this app.
const runner = ['bunx', 'npx'].find(
  (cmd) => !spawnSync(cmd, ['--version'], { stdio: 'ignore' }).error
)
if (!runner) die('neither bunx nor npx is on PATH')

const flags = ['-p', PROJECT, '--pretty', 'false']
if (process.argv.includes('--strict')) {
  flags.push('--strictNullChecks', '--noImplicitAny')
}

const pin = runner === 'bunx' ? ['--package', TYPESCRIPT] : ['--yes', '--package', TYPESCRIPT]
console.log(`tsc:    ${runner} ${TYPESCRIPT} ${flags.slice(4).join(' ')}`.trimEnd())
const tsc = spawnSync(runner, [...pin, 'tsc', ...flags], { stdio: 'inherit' })
process.exit(tsc.status ?? 2)
