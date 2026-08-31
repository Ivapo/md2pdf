/* Serves a *copy* of the desktop front end with `stub.mjs` in its head, so a
   driver can open the real `app/dist/index.html` outside a window.

     bun app/harness/serve.mjs [--rev <sha>] [--doc <path>] [--port <n>]
                               [--mutate <name>] [--quiet]

   `app/harness/checks.mjs` imports `serve()` below; the CLI is what an A/B is
   run by hand from, two of them on two ports.

   **It copies rather than edits, and that is a hard rule.**
   `app/typecheck.mjs` dies unless `app/dist/index.html` holds exactly one
   `<script type="module">` line and one `</script>` line, so a stub written into
   the real file breaks the check this repository already runs in CI. `git status`
   clean after a full run is `mpdf-003` Phase 12's exit gate, clause 4.

   **The scratch directory is outside `app/dist/`.** `generate_context!` walks
   `frontendDist` recursively into the shipped binary, so anything written under
   `dist/` is embedded in the app. The precedent is `app/.mirror/`, which
   `typecheck.mjs` writes for the same reason.                                */

import { createServer } from 'node:http'
import { spawnSync } from 'node:child_process'
import { existsSync, mkdirSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs'
import { dirname, extname, join, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const HERE = dirname(fileURLToPath(import.meta.url))
const APP = dirname(HERE)
const REPO = dirname(APP)
const SCRATCH = join(APP, '.harness')

const die = (why) => {
  console.error(`serve: ${why}`)
  process.exit(2)
}

/* ------------------------------------------------------------ the document */

/* Which files the pipeline reads, as `app/src/document.rs:kind_of` decides it:
   `.md`, the three bibliography spellings, and `md2pdf_core::IMAGE_EXTENSIONS`
   re-exported from `core/src/emit.rs`. Kept as one table here rather than three
   scattered tests so a new extension is one edit. */
const IMAGES = ['png', 'jpg', 'jpeg', 'gif', 'webp', 'svg', 'svgz', 'pdf']
const kindOf = (path) => {
  const name = path.split('/').pop()
  const at = name.lastIndexOf('.')
  if (at < 0) return null
  const extension = name.slice(at + 1)
  if (extension.toLowerCase() === 'md') return 'markdown'
  if (['bib', 'yml', 'yaml'].includes(extension.toLowerCase())) return 'bibliography'
  if (IMAGES.includes(extension)) return 'image'
  return null
}

/* **The default fixture's eleven rows are a committed literal, transcribed from
   `tests/fixtures/panel-manifest.txt`, and that is not laziness.** One of them —
   `sections/missing.md` — is a path the master names that the disk does not
   hold, and `book.md` deliberately does not name it: Rust's `document.rs:listing`
   is handed it. **No walk can produce that row.** The manifest is where the
   eleven are pinned and where `mpdf-010` Phase 1's own gate reads them, so it is
   the source here too. */
const PANEL = 'tests/fixtures/panel'
const PANEL_ENTRIES = [
  { path: 'book.md', kind: 'markdown', missing: false },
  { path: 'cover.jpg', kind: 'image', missing: false },
  { path: 'other.md', kind: 'markdown', missing: false },
  { path: 'plan.pdf', kind: 'image', missing: false },
  { path: 'refs.bib', kind: 'bibliography', missing: false },
  { path: 'refs.yml', kind: 'bibliography', missing: false },
  { path: 'loose/orphan.md', kind: 'markdown', missing: false },
  { path: 'parts/ch1/deep.md', kind: 'markdown', missing: false },
  { path: 'sections/mark.svg', kind: 'image', missing: false },
  { path: 'sections/missing.md', kind: 'markdown', missing: true },
  { path: 'sections/text.md', kind: 'markdown', missing: false }
]

/* `mpdf-010` §2's order: within each directory, files byte-wise alphabetically
   first, then subdirectories byte-wise alphabetically, each expanded where it
   sits. Symlinks are not followed — `tests/fixtures/panel/outside` is a symlink
   into a decoy directory, and a walk with no confinement would list it. */
const walk = (root, at = '') => {
  const here = join(root, at)
  let names
  try {
    names = readdirSync(here, { withFileTypes: true })
  } catch {
    return []
  }
  const files = names
    .filter((e) => e.isFile())
    .map((e) => (at ? `${at}/${e.name}` : e.name))
    .filter((path) => kindOf(path) !== null)
    .sort()
    .map((path) => ({ path, kind: kindOf(path), missing: false }))
  const folders = names
    .filter((e) => e.isDirectory())
    .map((e) => e.name)
    .sort()
  return [...files, ...folders.flatMap((name) => walk(root, at ? `${at}/${name}` : name))]
}

/* **`--doc` takes a directory or a file**, which is not looseness but the app's
   own project model: `mpdf-010`'s one level of climb resolves either into a root
   and a document.

   **The climb itself is deliberately not reimplemented here.**
   `app/src/document.rs:project_root` answers "does a markdown file above name
   this one" by reading text through `md2pdf_core::section_paths`, and a
   JavaScript copy of that is exactly the drift a committed rig must not add.
   Naming the *directory* is how you get the climb's answer; naming a file roots
   at its own parent, which is `watch::root` unchanged and is every single-file
   document. */
const project = (doc) => {
  const path = resolve(REPO, doc)
  if (!existsSync(path)) die(`no such document: ${doc}`)

  if (statSync(path).isDirectory()) {
    const relative_ = relative(REPO, path)
    const entries = relative_ === PANEL ? PANEL_ENTRIES : walk(path)
    const main = entries.find((e) => e.kind === 'markdown' && !e.missing)
    if (!main) die(`${doc} holds no markdown to compile`)
    return { root: path, document: join(path, main.path), main: main.path, entries }
  }

  const root = dirname(path)
  const main = path.slice(root.length + 1)
  return { root, document: path, main, entries: [{ path: main, kind: 'markdown', missing: false }] }
}

/* ------------------------------------------------------------- the mutants */

/* `mpdf-003` Phase 12's exit gate, clause 3: the suite is falsified before it is
   trusted. Each of these is applied to the *copy*, and each is asserted to apply
   exactly once — a mutation that silently matched nothing would make the clause
   pass on a page nothing was done to.

   The clause each one owns is declared in `checks.mjs`, beside the checks. */
const MUTATIONS = {
  /* Moves the footer past the module script, which is the one placement the
     element-order check forbids. A `<script>` is `display: none`, so this
     changes no geometry — it fails that check and nothing else. */
  'footer-last': (page) => {
    const footer = page.match(/\n( *)<footer>[\s\S]*?<\/footer>\n/)
    if (!footer) die('the mutation footer-last found no <footer> block')
    const without = page.replace(footer[0], '\n')
    const close = '\n    </script>\n'
    if (without.split(close).length !== 2) die('the mutation footer-last found no single </script>')
    return without.replace(close, `${close}${footer[0].slice(1)}`)
  },

  /* **Both declarations, because dropping either alone changes nothing.** Each
     independently zeroes `#edited`'s flex automatic minimum size, so they are
     redundant with each other — measured at a 240px viewport with a
     58-character name, identical in both engines. Phase 11's claim that
     `min-width: 0` was load-bearing by itself is what that falsified. */
  'flex-min': (page) => {
    const rule = page.match(/\n( *)#edited \{\n[\s\S]*?\n\1\}\n/)
    if (!rule) die('the mutation flex-min found no #edited rule')
    const dropped = rule[0]
      .replace(/^ *min-width: 0;\n/m, '')
      .replace(/^ *overflow: hidden;\n/m, '')
    if (dropped === rule[0]) die('the mutation flex-min dropped neither declaration')
    return page.replace(rule[0], dropped)
  },

  /* **A pinned header whose children have left it.** `flex-wrap: wrap` and one
     child too wide for the line, and the header still reads its own 28: a flex
     container with an explicit `height` does not grow when its line wraps, its
     content overflows instead. That is exactly what the height half of clause 3
     cannot see and the second half is there for — measured in the shipping
     WKWebView, `#status` 34px below the bar's own bottom edge and the wide child
     14px, where the unmutated header has no child outside it at all.

     Two edits, because either alone is inert: the wrap without a wide child has
     nothing to wrap, and the wide child without the wrap makes one long line. */
  'header-wraps': (page) => {
    const rule = page.match(/\n( *)header \{\n[\s\S]*?\n\1\}\n/)
    if (!rule) die('the mutation header-wraps found no header rule')
    const wrapped = rule[0].replace(/^( *)display: flex;\n/m, '$1display: flex;\n$1flex-wrap: wrap;\n')
    if (wrapped === rule[0]) die('the mutation header-wraps found no display: flex to wrap')

    const status = '      <span id="status"></span>\n'
    if (page.split(status).length !== 2) die('the mutation header-wraps found no single #status')
    return page
      .replace(rule[0], wrapped)
      .replace(status, '      <span id="wide" style="flex: none; width: 900px; height: 14px"></span>\n' + status)
  },

  /* The header's second mark says one thing to a sighted reader and another to
     a screen reader: `title` keeps `Save as…` while `aria-label` goes back to
     the `Save` this button said before `mpdf-003` Phase 17 renamed it. **The
     half of the clause a single-name check would miss**, and the shape of the
     defect Phase 16 deferred the rename to avoid — a button naming an action it
     does not perform. */
  'save-as-mislabelled': (page) => {
    const both = '<button id="save" type="button" title="Save as…" aria-label="Save as…">'
    if (page.split(both).length !== 2) die('the mutation save-as-mislabelled found no single #save button')
    return page.replace(both, '<button id="save" type="button" title="Save as…" aria-label="Save">')
  },

  /* Rewires the footer's cell to the file that compiles rather than the file
     the pane is holding. The two are equal at every open, so this passes
     everything until a row is clicked. */
  'cell-main': (page) => {
    const line = 'editedPath = state.edited ?? null'
    if (page.split(line).length !== 2) die('the mutation cell-main found no single cell line')
    return page.replace(line, 'editedPath = state.main ?? null')
  },

  /* Drops the attribute block that opts *in* under a light system, leaving the
     media query's `:not([data-theme='light'])` to carry both directions — which
     it cannot. Under a dark system nothing changes, which is the half a suite
     driven in one colour scheme would miss. */
  'theme-dark-attr': (page) => {
    const rule = page.match(/\n( *):root\[data-theme='dark'\] \{\n[\s\S]*?\n\1\}\n/)
    if (!rule) die('the mutation theme-dark-attr found no :root[data-theme=dark] block')
    return page.replace(rule[0], '\n')
  },

  /* Lets the page decide instead of asking. The attribute still moves and the
     mark still follows, so everything a check reads off the DOM alone is
     unchanged — the boundary is the only thing that moved. */
  'theme-click-direct': (page) => {
    const call = `invoke('set_appearance', {
          appearance: inEffect(worn) === 'dark' ? 'light' : 'dark'
        }).catch(fail)`
    if (page.split(call).length !== 2) die('the mutation theme-click-direct found no single invoke')
    return page.replace(
      call,
      `Promise.resolve(wearAppearance(inEffect(worn) === 'dark' ? 'light' : 'dark'))`
    )
  },

  /* The bar's marks paint the same in both states — on and off alike — which
     is the failure an icon invites and a word does not: every ARIA reading
     still agrees, the pane still follows, and a reader cannot see which way
     either toggle is set. **It owns clause 10 alone**, since the header gave
     its worded copies up and `views-one-way` — the sync between two controls
     that can no longer disagree — went with them. */
  'marks-unlit': (page) => {
    const block =
      "      #views button:hover,\n" +
      "      #views button[aria-expanded='true'],\n" +
      "      #views button[aria-pressed='true'] {\n"
    if (page.split(block).length !== 2) die('the mutation marks-unlit found no single mark rule')
    return page.replace(block, '      #views button:hover {\n')
  },

  /* A figure goes up and the bar goes on naming the markdown file — which is
     the shipped behaviour this changed, so the mutation is the page as it was.
     The `saySoInstead` write is left alone: one of the two is enough, and
     removing both would not say which. */
  'figure-unnamed': (page) => {
    const block = '        figureInPane = path\n        namePaneFile()\n\n        viewer.hidden = false\n'
    if (page.split(block).length !== 2) die('the mutation figure-unnamed found no single figure naming')
    return page.replace(block, '        viewer.hidden = false\n')
  },

  /* The receipt appears and never leaves. **The failure a clause reading only
     "it appeared" would miss**, which is why that clause reads the cell's
     emptiness afterwards and not the timer's length: with the re-arm dropped the
     sentence is correct, placed in the right cell, and permanent — a bar that
     says what the last save did for the rest of the session.

     Both statements go, the `clearTimeout` with the `setTimeout` it was paired
     with, because the clear alone would leave a timer nothing cancels and a
     second save could then blank the first's sentence — a different defect, and
     not this one. */
  'receipt-sticks': (page) => {
    const block =
      '        clearTimeout(receiptClock)\n' +
      '        receiptClock = setTimeout(() => {\n' +
      "          receiptCell.textContent = ''\n" +
      '        }, RECEIPT_MS)\n'
    if (page.split(block).length !== 2) die('the mutation receipt-sticks found no single re-arm')
    return page.replace(block, '')
  },

  /* Puts the auto margin back where Phase 11 had it. **This changes nothing
     about `#brand`'s own rect** — an auto margin absorbs exactly the free space
     in total, so a last child with no right margin cannot move, which is what
     falsified this mutation's first draft. What moves is the group. */
  'controls-auto-margin': (page) => {
    const controls = page.match(/\n( *)#controls \{\n[\s\S]*?\n\1\}\n/)
    const brand = page.match(/\n( *)#brand \{\n[\s\S]*?\n\1\}\n/)
    if (!controls) die('the mutation controls-auto-margin found no #controls rule')
    if (!brand) die('the mutation controls-auto-margin found no #brand rule')

    const moved = controls[0].replace(/^ *margin-left: auto;\n/m, '')
    if (moved === controls[0]) die('the mutation controls-auto-margin moved no margin')

    return page
      .replace(controls[0], moved)
      .replace(brand[0], brand[0].replace('flex: none;', 'flex: none;\n        margin-left: auto;'))
  }
}

/* ------------------------------------------------------------- the scratch */

const git = (args) => {
  const run = spawnSync('git', args, { cwd: REPO, maxBuffer: 1 << 28 })
  if (run.status !== 0) die(`git ${args.join(' ')} — ${run.stderr}`)
  return run.stdout
}

/** The page and the renderer beside it, from the working tree or from a named
    revision. **`--rev` is what an A/B between two revisions needs**, and it is
    the one thing this rig has historically been used for. */
const takePage = (scratch, rev) => {
  if (!rev) {
    writeFileSync(join(scratch, 'index.html'), readFileSync(join(APP, 'dist', 'index.html')))
    mkdirSync(join(scratch, 'pdfjs'), { recursive: true })
    for (const name of readdirSync(join(APP, 'dist', 'pdfjs'))) {
      writeFileSync(join(scratch, 'pdfjs', name), readFileSync(join(APP, 'dist', 'pdfjs', name)))
    }
    return readFileSync(join(scratch, 'index.html'), 'utf8')
  }

  const page = git(['show', `${rev}:app/dist/index.html`]).toString('utf8')
  writeFileSync(join(scratch, 'index.html'), page)
  mkdirSync(join(scratch, 'pdfjs'), { recursive: true })
  const listed = git(['ls-tree', '-r', '--name-only', rev, '--', 'app/dist/pdfjs'])
    .toString('utf8')
    .split('\n')
    .filter(Boolean)
  if (listed.length === 0) die(`${rev} carries no app/dist/pdfjs — the page cannot render there`)
  for (const path of listed) {
    writeFileSync(join(scratch, 'pdfjs', path.split('/').pop()), git(['show', `${rev}:${path}`]))
  }
  return page
}

/* ---------------------------------------------------------------- the copy */

const HEAD = '</head>'

export async function serve({ rev = null, doc = PANEL, port = 0, mutate = null, quiet = false } = {}) {
  if (mutate && !MUTATIONS[mutate]) die(`no such mutation: ${mutate} — ${Object.keys(MUTATIONS).join(', ')}`)

  const slug = [rev ? rev.replace(/[^\w.^~-]/g, '_') : 'tree', doc.replace(/[^\w-]/g, '_'), mutate ?? 'clean'].join('-')
  const scratch = join(SCRATCH, slug)
  rmSync(scratch, { recursive: true, force: true })
  mkdirSync(scratch, { recursive: true })

  const here = project(doc)

  /* **`-o` into the scratch directory, and the flag is not optional.**
     `cli/src/main.rs:default_output` writes beside its input and `.gitignore`
     covers PDFs under `/samples/` and nowhere else, so a compile without it leaves
     an untracked PDF in `tests/fixtures/` and fails the gate's own "`git status` is
     clean". The crate is `md2pdf-cli`; the binary it builds is `md2pdf`. */
  const pdf = join(scratch, 'document.pdf')
  const compile = spawnSync(
    'cargo',
    ['run', '--quiet', '-p', 'md2pdf-cli', '--', here.document, '-o', pdf],
    { cwd: REPO, stdio: quiet ? ['ignore', 'ignore', 'pipe'] : 'inherit' }
  )
  if (compile.status !== 0) die(`the CLI would not compile ${here.document}\n${compile.stderr ?? ''}`)

  let page = takePage(scratch, rev)
  if (mutate) page = MUTATIONS[mutate](page)

  /* **Into `<head>`, and the position is load-bearing.** The page reads
     `window['__TAURI__']` at module top level, so the stub must run first — and
     a `<script>` injected into `<body>` would sit between `</main>` and the
     module script, changing the very element order the first check asserts.

     The config is a classic inline script and so runs at parse time; the stub is
     a module and so runs after parsing but before the page's own module, module
     scripts executing in document order. */
  if (page.split(HEAD).length !== 2) die('the page does not hold exactly one </head>')
  /* **The project's figures, copied in so a figure can actually be drawn.**
     `asset_bytes` used to be refused outright — the stub's own comment said
     nothing in the checks clicked an image row, which stopped being true — and
     a refusal reaches the surface through `saySoInstead`, so every image row
     landed on a sentence and the drawn-figure path was unreachable. A clause
     about a figure on screen has to be able to get one there.

     **Copied rather than served from the tree**, for the server's one rule:
     nothing outside the scratch directory is served, however the URL is
     spelled. The missing row is skipped for the same reason Rust would refuse
     it, and the map is what the stub refuses an unknown path against. */
  const assets = {}
  mkdirSync(join(scratch, 'assets'), { recursive: true })
  for (const entry of here.entries) {
    if (entry.kind !== 'image' || entry.missing) continue
    const from = join(here.root, entry.path)
    if (!existsSync(from)) continue
    const to = join(scratch, 'assets', entry.path)
    mkdirSync(dirname(to), { recursive: true })
    writeFileSync(to, readFileSync(from))
    assets[entry.path] = `assets/${entry.path}`
  }

  const config = {
    entries: here.entries,
    main: here.main,
    pdf: 'document.pdf',
    assets,
    text: readFileSync(here.document, 'utf8')
  }
  const inject =
    `    <script>window.__HARNESS_CONFIG = ${JSON.stringify(config)}</script>\n` +
    `    <script type="module" src="./stub.mjs"></script>\n  ${HEAD}`
  writeFileSync(join(scratch, 'index.html'), page.replace(HEAD, inject))
  writeFileSync(join(scratch, 'stub.mjs'), readFileSync(join(HERE, 'stub.mjs')))

  const TYPES = {
    '.html': 'text/html; charset=utf-8',
    '.mjs': 'text/javascript; charset=utf-8',
    '.js': 'text/javascript; charset=utf-8',
    '.pdf': 'application/pdf',
    '.svg': 'image/svg+xml',
    '.jpg': 'image/jpeg',
    '.png': 'image/png'
  }

  const server = createServer((request, answer) => {
    const path = new URL(request.url, 'http://localhost').pathname
    const file = join(scratch, path === '/' ? 'index.html' : path.replace(/^\/+/, ''))
    // Nothing outside the scratch directory is served, however the URL is spelled.
    if (!file.startsWith(scratch) || !existsSync(file) || statSync(file).isDirectory()) {
      answer.writeHead(404).end('not here')
      return
    }
    answer.writeHead(200, { 'content-type': TYPES[extname(file)] ?? 'application/octet-stream' })
    answer.end(readFileSync(file))
  })

  await new Promise((ready) => server.listen(port, '127.0.0.1', ready))
  const url = `http://127.0.0.1:${server.address().port}/`
  if (!quiet) {
    console.log(`serve: ${url}`)
    console.log(`serve: ${relative(REPO, scratch)}${rev ? `  ·  ${rev}` : ''}${mutate ? `  ·  ${mutate}` : ''}`)
    console.log(`serve: ${relative(REPO, here.document)}, ${here.entries.length} entries, main ${here.main}`)
  }

  return {
    url,
    scratch,
    project: here,
    close: () => new Promise((done) => server.close(done))
  }
}

/* ----------------------------------------------------------------- the CLI */

const flag = (name, fallback = null) => {
  const at = process.argv.indexOf(`--${name}`)
  return at < 0 ? fallback : process.argv[at + 1]
}

if (process.argv[1] && resolve(process.argv[1]) === resolve(fileURLToPath(import.meta.url))) {
  const held = await serve({
    rev: flag('rev'),
    doc: flag('doc', PANEL),
    port: Number(flag('port', 0)),
    mutate: flag('mutate')
  })
  console.log('serve: ^C to stop')
  process.on('SIGINT', async () => {
    await held.close()
    process.exit(0)
  })
}
