/* What drives the real window — the clauses a person used to read off the glass.

     bun app/driver/drive.mjs [--falsify] [--mutate <name>]

   **A second rig, not a change to the first.** `app/harness/` stubs Rust and
   drives `app/dist/index.html` in Playwright, which is where geometry, the panel
   and the palette are checked. This drives the **shipped binary** and the real
   WKWebView — `browserName: webkit`, the engine that ships — over
   `tauri-plugin-wdio-webdriver`, an in-process W3C WebDriver server on
   `127.0.0.1:4445` that `app/Cargo.toml`'s `driven` feature is the only way to
   get. The two answer different questions and neither replaces the other: a
   claim about the page belongs to the harness, a claim about *the window* — the
   IPC, the stored file, the OS resize — belongs here.

   **Plain `fetch` and no npm client.** `@wdio/tauri-service` is the intended
   client and buys nothing this needs; the routes used are `/status`, `/session`,
   `/session/<id>/execute/sync`, `/session/<id>/execute/async` and
   `/session/<id>/window/rect`. **Both execute routes**, because `invoke` returns
   a promise and the async route is the one with a W3C completion callback to
   settle it on.

   **It launches `target/debug/letur` itself and kills it**, and not through the
   Tauri CLI: `app/Cargo.toml`'s own comment records that `cargo tauri dev`
   rewrites the crate's feature lists on every run, which would dirty the tree in
   the same file this feature was added to. **And it restores what it moved** —
   `set_appearance` writes `settings.json` in the app's Application Support
   directory, so this reads that file before the launch and puts it back after
   the kill. A gate that left the appearance where it happened to stop is one of
   the four instrument defects `specs/desktop_app_spec.md` Phase 14 is built on.

   **Falsified before it is trusted.** `--mutate <name>` installs a deliberate
   defect into the live session and judges that **exactly** the clause that owns
   it fails; `--falsify` runs both, one child process each. The page is compiled
   into the binary — `generate_context!` walks `frontendDist` in — so there is no
   served copy to edit the way `app/harness/serve.mjs` builds one, and a mutation
   is injected instead. **It cannot be the page's own function**: the page has one
   `<script type="module">` and `execute/sync` evaluates a classic script in the
   *global* context, which cannot rebind a module-local name. What it can reach is
   the DOM, so each mutation replaces a method or accessor on one of the two
   objects `wearAppearance` writes through — reached by property lookup at call
   time, so an own property on the instance shadows the prototype and takes.

   **What this does not reach.** The title bar's own pixels and the launch flash:
   W3C `Take Screenshot` is the viewport and not the OS window, and the session
   only exists after the window is up. Those stay in
   `tests/gates/mpdf-003-phase13.js`, with a person. It makes no claim about the
   seven behaviour defects `rules/desktop-panes.md` lists — the 2026-08-29 spike
   measured the real engine under a driver and got the same 0 Playwright gave.

   **One thing a run needs that a headless browser does not: the window has to be
   being rendered.** A `ResizeObserver` fires on a rendering opportunity, and a
   macOS window nothing can see gets none — clause 3 read 0 out of a flat
   four-second wait once, for that reason and not for a blind listener. It polls
   now, and it says which of the two a 0 was.                                   */

import { spawn, spawnSync } from 'node:child_process'
import { existsSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { homedir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const argv = process.argv.slice(2)
const has = (name) => argv.includes(`--${name}`)
const flag = (name, fallback = null) => {
  const at = argv.indexOf(`--${name}`)
  return at < 0 ? fallback : argv[at + 1]
}

const HERE = dirname(fileURLToPath(import.meta.url))
const REPO = resolve(HERE, '..', '..')
const BINARY = join(REPO, 'target', 'debug', 'letur')
const CONFIG = join(REPO, 'app', 'tauri.conf.json')

/* The plugin's own default, and it is not configurable from here: nothing but
   `TAURI_WEBDRIVER_PORT` moves it, and moving it would make the number in
   `rules/desktop.md` a lie. */
const PORT = 4445
const BASE = `http://127.0.0.1:${PORT}`

const APPEARANCES = ['system', 'light', 'dark']

/* The width clause 2 measures at. **An input and not a reading** — nothing below
   is keyed to it, and the driver converges on it by reading `innerWidth` back
   rather than by arithmetic, `window/rect` being physical pixels where
   `tauri.conf.json`'s `width` is logical. */
const WIDTH = 600

const wait = (ms) => new Promise((r) => setTimeout(r, ms))

/* The IPC round trip and a paint, which is what `set_appearance` costs: it
   announces without compiling and the page's `refresh` guards on the revision,
   so this is not `watch.rs:DEBOUNCE` plus a compile. The figure is
   `tests/gates/mpdf-003-phase13.js`'s, measured in the window. */
const settled = () => wait(400)

/* Exit 2 is the rig, exit 1 is a clause — `app/harness/serve.mjs` and
   `app/typecheck.mjs` both spell it this way. */
const die = (why) => {
  console.error(`drive: ${why}`)
  process.exit(2)
}

/* -------------------------------------------------------------- the ledger */

let passed = 0
let failed = 0
const owned = []

const ok = (n, name, good, detail) => {
  good ? passed++ : (failed++, owned.push(n))
  console.log(`${good ? 'PASS' : 'FAIL'}  ${String(n).padStart(2)}. ${name}${detail ? '\n          ' + detail : ''}`)
}
const note = (s) => console.log(`....  ${s}`)

/* ------------------------------------------------------------- the mutants */

/* Which clause each mutation owns. A mutation that fails a second clause is a
   check measuring something it does not claim to; a mutation that fails none is a
   check that would pass on a page the scope forbids.

   Clause 1 does not assert the mark and clause 2 does not assert the attribute,
   which is the sentence that buys these two their isolation. */
const OWNS = {
  'attribute-always-set': 1,
  'third-mark': 2
}

/* **Each mutation counts its own invocations**, which is `serve.mjs`'s rule that
   a replacement matching nothing has falsified nothing, in this rig's terms: the
   counter is a page global, reset where the mutation is installed, and one
   mutation is live per run — each `--falsify` child launches its own app. A
   counter still at zero when the run ends is the instrument broken, not a clause
   failed, so it is `die` and not `ok`. */
const MUTATIONS = {
  /* Owns clause 1. `wearAppearance`'s only `removeAttribute` is the `system`
     branch, and its only `setAttribute` for this attribute is the other one.

     **It writes a value rather than doing nothing**, and the difference is not
     cosmetic: doing nothing leaves whatever was already there, so a walk reaching
     `system` while the attribute happens to be absent — the state a launch on a
     stored `system` produces — would find clause 1's assertion holding and this
     mutation passing, with its counter incremented so the counter rule would not
     catch it either.

     **The value is a sentinel outside the appearance vocabulary.** Writing
     `"system"` is the one choice that collides with what clause 1 asserts about,
     and a permissive reading of that clause would let it through. A value no
     appearance is named by cannot be read either way.

     It fails clause 1 alone. `data-theme="mutated"` matches neither
     `:root[data-theme='dark']` nor `:root[data-theme='light']` and does match the
     media query's `:not([data-theme='light'])` — the page carries no bare
     `[data-theme]` presence selector at all — so it is palette-identical to the
     attribute being absent; and independently the mark is
     `APPEARANCE_MARK[inEffect(next)]`, which reads `matchMedia`, which no
     `data-theme` affects. Clause 2 stands on either argument alone. */
  'attribute-always-set': `
    window.__mutation = { name: 'attribute-always-set', count: 0 }
    const root = document.documentElement
    const original = Element.prototype.removeAttribute
    root.removeAttribute = function (name) {
      if (name !== 'data-theme') return original.call(this, name)
      window.__mutation.count++
      return this.setAttribute('data-theme', 'mutated')
    }
    return true
  `,

  /* Owns clause 2. An own accessor on the button shadows `Node.prototype`'s, and
     `wearAppearance` assigns `textContent` by property lookup at call time, so it
     takes.

     **It delegates to the original setter and actually puts the glyph in the
     DOM.** Clause 2 measures with `getBoundingClientRect`, and an accessor that
     lied about what it wrote would change no rendered width and falsify nothing.

     It fails clause 2's "exactly two distinct glyphs" alone: `title` and
     `aria-label` are written on their own lines from `APPEARANCE_SAYS` and are
     untouched, and clause 1 does not assert the mark. */
  'third-mark': `
    window.__mutation = { name: 'third-mark', count: 0 }
    const button = document.getElementById('theme')
    const original = Object.getOwnPropertyDescriptor(Node.prototype, 'textContent')
    Object.defineProperty(button, 'textContent', {
      configurable: true,
      get() {
        return original.get.call(this)
      },
      set(value) {
        if (document.documentElement.hasAttribute('data-theme')) return original.set.call(this, value)
        window.__mutation.count++
        return original.set.call(this, '◐')
      }
    })
    return true
  `
}

/* ------------------------------------------------------------- the endpoint */

const call = async (method, path, body) => {
  const answer = await fetch(`${BASE}${path}`, {
    method,
    headers: { 'Content-Type': 'application/json' },
    body: body === undefined ? undefined : JSON.stringify(body)
  })
  const said = await answer.json()
  if (said.value && said.value.error) throw new Error(`${said.value.error}: ${said.value.message}`)
  return said.value
}

/** Is anything answering on the port at all? */
const answering = async () => {
  try {
    await fetch(`${BASE}/status`, { signal: AbortSignal.timeout(500) })
    return true
  } catch {
    return false
  }
}

/** A session over one window, and the two script routes it needs.

    **`execute/sync` for anything the DOM can answer at once, `execute/async` for
    anything that awaits.** Both take a *function body*, so a value wants an
    explicit `return`; the async one is handed a `done(result, error)` as its last
    argument, which is where a promise is settled. */
const session = async () => {
  const made = await call('POST', '/session', {
    capabilities: { alwaysMatch: { browserName: 'tauri' } }
  })
  const id = made.sessionId
  return {
    id,
    engine: `${made.capabilities.browserName} ${made.capabilities.browserVersion} on ${made.capabilities.platformName}`,
    setWindowRect: made.capabilities.setWindowRect === true,
    sync: (script, args = []) => call('POST', `/session/${id}/execute/sync`, { script, args }),
    async: (script, args = []) => call('POST', `/session/${id}/execute/async`, { script, args }),
    rect: () => call('GET', `/session/${id}/window/rect`),
    resize: (width) => call('POST', `/session/${id}/window/rect`, { width })
  }
}

/* ------------------------------------------------------------------ the app */

/** Where the one thing this app remembers about its appearance lives. The
    identifier is read rather than written down, so a rename in
    `tauri.conf.json` moves this too instead of silently restoring nothing. */
const settingsFile = () => {
  const identifier = JSON.parse(readFileSync(CONFIG, 'utf8')).identifier
  if (!identifier) die(`${CONFIG} names no identifier`)
  return join(homedir(), 'Library', 'Application Support', identifier, 'settings.json')
}

/** The app, up and answering. Preflight first, because both failures below are
    silent otherwise: a plain build opens no port at all, and a stranger already
    on 4445 would be driven instead of this one. */
const launch = async () => {
  if (!existsSync(BINARY)) die(`no ${BINARY} — run: cargo build -p letur --features driven`)

  /* A sibling `--falsify` child may still be letting go of the port, so this
     waits before it complains. */
  for (let i = 0; i < 6 && (await answering()); i++) await wait(500)
  if (await answering()) die(`something is already answering on ${PORT} — quit it, or this drives the wrong window`)

  const app = spawn(BINARY, [], { stdio: ['ignore', 'pipe', 'pipe'] })
  const log = []
  app.stdout.on('data', (d) => log.push(String(d)))
  app.stderr.on('data', (d) => log.push(String(d)))

  /* **`ready` and not a 200.** The server answers as soon as it is up and reports
     `ready: false` until a webview window exists, so a session created on the
     status code alone races the window into being. */
  for (let i = 0; i < 60; i++) {
    try {
      const said = await (await fetch(`${BASE}/status`, { signal: AbortSignal.timeout(1000) })).json()
      if (said.value?.ready) {
        return { app, log }
      }
    } catch {
      /* Not up yet. The loop is the timeout. */
    }
    await wait(500)
  }

  app.kill('SIGKILL')
  die(`nothing became ready on ${PORT} in 30s — is this a --features driven build?\n${log.join('').slice(-2000)}`)
}

/* -------------------------------------------------------------- the reading */

/** Everything the bar can be read for, in one round trip.

    **Every field is copied out by name.** The server serializes own enumerable
    properties, and a `DOMRect`'s are all on its prototype — returned whole it
    comes back as an empty object. */
const BAR = `
  const button = document.getElementById('theme')
  const brand = document.getElementById('brand')
  const mark = button.getBoundingClientRect()
  const name = brand.getBoundingClientRect()
  return {
    mark: button.textContent,
    title: button.title,
    label: button.getAttribute('aria-label'),
    attribute: document.documentElement.getAttribute('data-theme'),
    width: mark.width,
    brandLeft: name.left,
    innerWidth: window.innerWidth,
    ratio: window.devicePixelRatio
  }
`

const ASK_STATUS = `
  const done = arguments[arguments.length - 1]
  window['__TAURI__'].core.invoke('status').then(
    (state) => done(state.appearance),
    (problem) => done(null, String(problem))
  )
`

const SET_APPEARANCE = `
  const done = arguments[arguments.length - 1]
  window['__TAURI__'].core.invoke('set_appearance', { appearance: arguments[0] }).then(
    () => done(true),
    (problem) => done(null, String(problem))
  )
`

/** Ask Rust for an appearance, wait for Rust to say it has it, let the page
    place it, and read the bar. The wait is on `status` rather than on the DOM
    deliberately: the DOM is what the clauses judge, so waiting on it would be
    waiting for the answer. */
const wear = async (held, appearance) => {
  await held.async(SET_APPEARANCE, [appearance])

  let said = null
  for (let i = 0; i < 15; i++) {
    said = await held.async(ASK_STATUS)
    if (said === appearance) break
    await wait(200)
  }
  await settled()

  return { appearance, said, ...(await held.sync(BAR)) }
}

const walk = async (held) => {
  const seen = []
  for (const appearance of APPEARANCES) seen.push(await wear(held, appearance))
  return seen
}

/* ------------------------------------------------------------- the clauses */

/* 1. **The value reaches Rust and the page places it.** All three appearances
      through the real command over the real bridge.

      `data-theme` is spelled **absent for `system`, equal to the appearance
      otherwise** — that way round and not as "the value or absent", which is
      permissive enough for a mutation writing `data-theme="system"` to satisfy.

      **It does not assert the mark**, which is clause 2's. Two clauses asserting
      one reading is two clauses that cannot be told apart by a mutation. */
const valueReachesRustAndThePagePlacesIt = async (held) => {
  const seen = await walk(held)

  for (const state of seen) {
    note(
      `${state.appearance}: status says ${JSON.stringify(state.said)}, ` +
        `attribute ${JSON.stringify(state.attribute)}, ` +
        `title ${JSON.stringify(state.title)}, label ${JSON.stringify(state.label)}`
    )
  }

  const moved = seen.every((s) => s.said === s.appearance)
  const misplaced = seen.filter(
    (s) =>
      s.attribute !== (s.appearance === 'system' ? null : s.appearance) ||
      s.title !== s.label ||
      !/appearance/i.test(s.label || '')
  )

  ok(
    1,
    'the value reaches Rust and the page places it, in all three appearances',
    moved && misplaced.length === 0,
    `values ${moved ? 'moved' : 'DID NOT MOVE'}` +
      (misplaced.length
        ? `; MISPLACED: ${misplaced
            .map(
              (s) =>
                `${s.appearance} had attribute ${JSON.stringify(s.attribute)} ` +
                `wanted ${s.appearance === 'system' ? 'none' : JSON.stringify(s.appearance)}, ` +
                `title ${JSON.stringify(s.title)} label ${JSON.stringify(s.label)}`
            )
            .join(' | ')}`
        : '; every attribute and label as spelled')
  )
}

/* 2. **The marks, measured at a width the driver set.** `tests/gates/mpdf-003-phase13.js`'s
      clause 2, moved whole — the two glyphs agreeing with each other and the
      brand not moving as they swap, with no metric literal anywhere.

      **The resize is part of the clause and not a setup step.** Moving the window
      is the reason this is a driver rather than a script: the page cannot do it,
      `core:default` granting the window's getters and no setter. A rig that
      shipped `setWindowRect` structurally present and never exercised would be
      claiming the thing it did not check, so the conjunction below carries it. */
const marksAtAWidthTheDriverSet = async (held) => {
  if (!held.setWindowRect) die('the session does not offer setWindowRect — this rig has nothing to stand on')

  /* **Converged by reading back, not by arithmetic.** `window/rect` is physical
     pixels where `tauri.conf.json`'s `width` is logical, and the window's own
     chrome is in between; 700 physical gave `innerWidth` 350 at a
     `devicePixelRatio` of 2 when this was measured. */
  let inner = await held.sync('return window.innerWidth')
  for (let i = 0; i < 12 && inner !== WIDTH; i++) {
    const ratio = await held.sync('return window.devicePixelRatio')
    const rect = await held.rect()
    await held.resize(Math.round(rect.width + (WIDTH - inner) * ratio))
    await wait(400)
    inner = await held.sync('return window.innerWidth')
  }
  note(`the driver moved the window to an innerWidth of ${inner}, asked for ${WIDTH}`)

  const seen = await walk(held)

  const widths = seen.map((s) => s.width)
  const spread = Math.max(...widths) - Math.min(...widths)
  const marks = new Set(seen.map((s) => s.mark))
  const brands = new Set(seen.map((s) => s.brandLeft.toFixed(2)))
  const drawn = widths.every((w) => w > 4 && w < 20)
  const sized = inner === WIDTH

  ok(
    2,
    'the marks are alike at a width the driver set, and the brand does not move as they swap',
    sized && spread < 1 && brands.size === 1 && drawn && marks.size === 2,
    `${sized ? 'sized' : 'NOT SIZED'} to ${inner}; ` +
      `widths ${widths.map((w) => w.toFixed(2)).join(' / ')} — spread ${spread.toFixed(2)}px; ` +
      `${marks.size} distinct marks ${JSON.stringify([...marks])}; brand at ${[...brands].join(', ')}`
  )
}

/* 3. **The instrument can see what it reports the absence of.** A deliberate
      `ResizeObserver` loop — a callback that resizes the box it observes — must
      be caught. The 2026-08-29 spike is why this is a clause and not a habit: its
      0 meant nothing until 242 caught errors made it a measurement.

      **It runs last and tears itself down.** The loop is self-perpetuating by
      construction, and one left firing would sit under every measurement taken
      after it. */
const theInstrumentSeesALoop = async (held) => {
  await held.sync(`
    window.__loop = { errors: 0, callbacks: 0, spoken: [] }
    window.__loopHeard = (e) => {
      const what = String(e.message || e.error || '')
      if (!what.includes('ResizeObserver')) return
      window.__loop.errors++
      if (window.__loop.spoken.length < 3) window.__loop.spoken.push(what.slice(0, 120))
    }
    addEventListener('error', window.__loopHeard)

    const box = document.createElement('div')
    box.style.cssText = 'position:fixed;left:-9999px;top:0;width:10px;height:10px'
    document.body.appendChild(box)
    window.__loopRo = new ResizeObserver(() => {
      window.__loop.callbacks++
      if (window.__loop.callbacks < 400) box.style.width = 10 + (window.__loop.callbacks % 50) + 'px'
    })
    window.__loopRo.observe(box)
    window.__loopBox = box
    return true
  `)

  /* **Polled to a ceiling rather than waited out flat**, and the callback count
     is read beside the errors. A `ResizeObserver` fires on a rendering
     opportunity, and the loop wants a few hundred of them; a flat wait reports 0
     for "the window was not being rendered" and for "the listener is blind" in
     the same words, which is exactly the ambiguity this clause exists to remove. */
  let heard = { errors: 0, callbacks: 0, spoken: [] }
  for (let i = 0; i < 40; i++) {
    await wait(500)
    heard = await held.sync('return window.__loop')
    if (heard.errors > 0) break
  }

  await held.sync(`
    window.__loopRo.disconnect()
    window.__loopBox.remove()
    removeEventListener('error', window.__loopHeard)
    return true
  `)

  ok(
    3,
    'the instrument catches a ResizeObserver loop it made itself',
    heard.errors > 0,
    `${heard.errors} caught in ${heard.callbacks} observer callbacks` +
      (heard.spoken.length ? ` — ${heard.spoken.join(' | ')}` : '') +
      (heard.errors > 0
        ? ''
        : heard.callbacks === 0
          ? '; the observer never ran at all, so the window was not being rendered — ' +
            'bring it to the front and run this again'
          : '; the loop ran and the listener heard nothing, so a 0 from this rig would mean nothing')
  )
}

/* ----------------------------------------------------------------- the run */

const run = async ({ mutate }) => {
  passed = 0
  failed = 0
  owned.length = 0

  if (mutate && !MUTATIONS[mutate]) die(`no such mutation: ${mutate} — ${Object.keys(MUTATIONS).join(', ')}`)

  const settings = settingsFile()
  const before = existsSync(settings) ? readFileSync(settings) : null

  const { app, log } = await launch()
  let fired = null
  /* **Kept and rethrown after the `finally`, never `process.exit`ed inside the
     `try`.** An exit there terminates the process before the block below runs,
     and the block below is what puts the author's appearance back. */
  let broke = null

  try {
    const held = await session()
    console.log(`\n${held.engine} | ${BINARY.replace(`${REPO}/`, '')}${mutate ? ` | MUTATED ${mutate}` : ''}\n`)

    if (mutate) {
      await held.sync(MUTATIONS[mutate])
      note(`${mutate} installed, and it owns clause ${OWNS[mutate]}`)
    }

    await valueReachesRustAndThePagePlacesIt(held)
    await marksAtAWidthTheDriverSet(held)
    await theInstrumentSeesALoop(held)

    if (mutate) fired = (await held.sync('return window.__mutation')).count
  } catch (problem) {
    broke = problem
  } finally {
    app.kill('SIGKILL')
    /* The port is the next child's preflight, so it is let go of before this
       process is. */
    await wait(500)

    if (before === null) rmSync(settings, { force: true })
    else writeFileSync(settings, before)
  }

  if (broke) {
    console.error(log.join('').slice(-2000))
    die(broke.message)
  }

  const back = existsSync(settings) ? JSON.parse(readFileSync(settings, 'utf8')).appearance : null
  note(`settings.json is back at ${JSON.stringify(back)}, where the run found it`)

  if (mutate && !fired) {
    die(`the mutation ${mutate} was never invoked — it has falsified nothing, whatever the clauses said`)
  }

  console.log(`\n${passed + failed} clauses: ${passed} passed, ${failed} failed`)
  return { passed, failed, owned: [...owned], fired }
}

/* ----------------------------------------------------------------- the CLI */

if (has('falsify')) {
  /* Each mutation must fail exactly the clause that owns it and no other.

     **One child process per mutation, and one app per child.** The mutations
     patch the live DOM and there is no uninstall worth trusting, so a second
     mutation in the same window would be running under the first. Each child is
     this file with `--mutate`, so the forked path and the single-mutation path
     are one path, and its exit code already means "isolated". */
  const rest = argv.filter((a) => a !== '--falsify')
  const verdicts = Object.keys(OWNS).map((mutate) => {
    const child = spawnSync(process.execPath, [fileURLToPath(import.meta.url), ...rest, '--mutate', mutate], {
      stdio: 'inherit'
    })
    return { mutate, clause: OWNS[mutate], isolated: child.status === 0 }
  })
  console.log('\nfalsification in the window:')
  for (const v of verdicts) {
    console.log(`  ${v.isolated ? 'ISOLATED    ' : 'NOT ISOLATED'}  ${v.mutate} owns clause ${v.clause}`)
  }
  process.exit(verdicts.every((v) => v.isolated) ? 0 : 1)
}

const mutate = flag('mutate')
const answer = await run({ mutate })

if (mutate) {
  /* **Clause 3 is not a mutation and is not optional.** A run where the
     deliberate loop went uncaught puts 3 in `owned` and is not isolated, whatever
     the mutation's own clause did. */
  const isolated = answer.owned.length === 1 && answer.owned[0] === OWNS[mutate]
  console.log(
    `${isolated ? 'ISOLATED' : 'NOT ISOLATED'}  ${mutate} owns clause ${OWNS[mutate]}, ` +
      `failed [${answer.owned.join(', ')}], fired ${answer.fired}×`
  )
  process.exit(isolated ? 0 : 1)
}

process.exit(answer.failed === 0 ? 0 : 1)
