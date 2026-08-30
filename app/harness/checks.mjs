/* What checks `app/dist/index.html` — the assertions the harness exists for.

     bun app/harness/checks.mjs [--webkit] [--headed] [--rev <sha>] [--doc <path>]
                                [--mutate <name>] [--falsify]

   **Chromium is the default, and the default decides little.** Every metric
   literal is forbidden below and `mpdf-003` Phase 12's gate requires both engines
   to pass, so this is not a fidelity choice and is not dressed as one: against the
   same page the header's rect reads 47.40625 in Playwright's WebKit, 46.75 in its
   Chromium and 47 in the window, and at the narrow widths WebKit agrees with
   Chromium and not with the window. Chromium is first because it is the engine
   every recorded run in `mpdf-009` used and the one a contributor most likely
   already has; WebKit is kept because a second engine catches what one cannot,
   not because it is truer.

   **From which follows the one rule every check obeys: assert a property, never a
   metric literal.** The sum is exact; the footer does not change height; the
   header grows below its own threshold; the cell holds the last path segment. **No
   check may encode 46.5, 46.75, 47, 66, 79, 80.5 or 627**, and
   `rules/desktop-geometry.md` carries why.

   **What this does not reach**, said here rather than left to be discovered: the
   seven behaviour defects in `rules/desktop-panes.md`'s list. The A/B that once
   justified reaching them — 0 `ResizeObserver` errors before `overflow-x: auto`,
   21 after — does not reproduce under this driver. `tests/gates/mpdf-009-phase5.js`,
   pasted into a real window's console, is still the only thing that has seen them.

   **The suite is falsified before it is trusted.** `--mutate <name>` serves a
   deliberately broken copy and judges that **exactly** the clause that owns it
   fails; `--falsify` runs all six. That is the gate's clause 3, run rather than
   read.

   **`light` is the default colour scheme and it is written down**, because one
   of the clauses below is about a page that must behave differently under each
   and Playwright's own default is not a thing to inherit silently.           */

import { chromium, webkit } from 'playwright'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { serve } from './serve.mjs'

const argv = process.argv.slice(2)
const has = (name) => argv.includes(`--${name}`)
const flag = (name, fallback = null) => {
  const at = argv.indexOf(`--${name}`)
  return at < 0 ? fallback : argv[at + 1]
}

/* Which clause each mutation owns. A mutation that fails a second clause is a
   check measuring something it does not claim to; a mutation that fails none is a
   check that would pass on a page the scope forbids. */
const OWNS = {
  'footer-last': 1,
  'flex-min': 3,
  'cell-main': 5,
  'theme-dark-attr': 7,
  'theme-click-direct': 8,
  'controls-auto-margin': 9
}

/* **58 characters, and the length is asserted rather than trusted.** The
   `flex-min` mutation only bites where `#edited`'s content overflows the bar, and
   the default fixture's longest bare name is `missing.md` — so without this name
   and the 240px floor below, that mutation falsifies nothing. **It is deliberately
   not one of the fixture's eleven entries**, so the panel marks no edited row
   during the sweep: that is why the sweep and the row click are two checks. */
const LONG_NAME = 'notes-and-sources-for-the-second-chapter-final-revision.md'

/* Descending, and it must reach 240px — the width the brand was measured to
   survive down to, and the one the mutation was measured at. The widths are
   inputs; nothing below is keyed to any of them. */
const WIDTHS = [900, 620, 500, 320, 240]
const HEIGHT = 600

/* -------------------------------------------------------------- the ledger */

let passed = 0
let failed = 0
const owned = []

const ok = (n, name, good, detail) => {
  good ? passed++ : (failed++, owned.push(n))
  console.log(`${good ? 'PASS' : 'FAIL'}  ${String(n).padStart(2)}. ${name}${detail ? '\n          ' + detail : ''}`)
}
const note = (s) => console.log(`....  ${s}`)

/* ---------------------------------------------------------------- the page */

const settle = async (page) =>
  page.evaluate(
    () => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(r, 250))))
  )

/** A page with the document open and its first compile drawn. Every check gets
    its own, so no check inherits the state another left behind. */
const opened = async (browser, url, width = WIDTHS[0], colorScheme = 'light') => {
  const page = await browser.newPage({
    viewport: { width, height: HEIGHT },
    deviceScaleFactor: 2,
    colorScheme
  })
  await page.goto(url, { waitUntil: 'load' })
  await page.waitForFunction(() => typeof window.__harness === 'object')
  await page.evaluate(() => {
    window.__harness.open()
    window.__harness.fire('rendered')
  })
  /* The pane has drawn when a wrapper exists: `openPdf` measured the pane and
     rasterised, so a geometry reading taken after this is taken against a page
     and not an empty column. */
  await page.waitForFunction(() => document.getElementById('pages').children.length > 0, null, {
    timeout: 30000
  })
  await settle(page)
  return page
}

/** What a page's error listener saw. Read before the page is closed, because the
    listener lives in the page — `page.on('pageerror')` does not see a
    `ResizeObserver` loop error, which is measured and is why `stub.mjs` installs
    one of its own in `<head>`. */
const drainErrors = async (page) => page.evaluate(() => window.__harness.errors)

/* -------------------------------------------------------------- the checks */

/* 1. **All three, and the position is satisfiable by and only by one
      placement.** `body`'s element children are `HEADER, MAIN, FOOTER, SCRIPT`:
      "after `</main>`" and "the last element of `body`" name two different
      places — the second being that script, and at runtime a hidden canvas
      `pdf.js` appends — so a clause asserting only one of them would pass on a
      page the scope forbids. */
const elementOrder = async (browser, url) => {
  const page = await opened(browser, url)
  const read = await page.evaluate(() => {
    const main = document.querySelector('main')
    const footer = document.querySelector('footer')
    /* The page's own module, not the stub's: the harness injects a second
       `script[type=module]`, and it lives in the head. */
    const module = document.body.querySelector('script[type="module"]')
    return {
      nextIsFooter: main.nextElementSibling === footer,
      thenModule: footer.nextElementSibling === module,
      notLast: document.body.lastElementChild !== footer,
      order: [...document.body.children].map((e) => e.tagName).join(', ')
    }
  })
  const errors = await drainErrors(page)
  await page.close()

  ok(
    1,
    "the footer is main's next sibling and the last element before the module script",
    read.nextIsFooter && read.thenModule && read.notLast,
    `body holds ${read.order}`
  )
  return errors
}

/* 2 and 3 share one sweep. **They are still two clauses**: the sum is what the
   bar costs the column, the brand is what keeps it in the bar, and the
   `flex-min` mutation moves the second without moving the first. */
const sweep = async (browser, url) => {
  if (LONG_NAME.length < 58) throw new Error(`the sweep's name is ${LONG_NAME.length} characters, wanted 58`)

  const page = await opened(browser, url)
  /* **Both fields carry the long name, and that is what keeps this clause off
     clause 5's ground.** This one is about the bar's geometry — that a name too
     long for the cell does not push the brand out — and it must not be keyed to
     *which* `Status` field the cell is wired to, which is the only thing clause 5
     asserts. Setting `edited` alone, the `cell-main` mutation emptied the cell,
     the sweep went vacuous and this clause failed for a reason it does not own.
     Measured, not reasoned about: that is what the falsification run reported
     before this line was written. */
  await page.evaluate((name) => {
    window.__harness.set({ edited: name, main: name })
    window.__harness.fire('rendered')
  }, LONG_NAME)
  await settle(page)

  const readings = []
  for (const width of WIDTHS) {
    await page.setViewportSize({ width, height: HEIGHT })
    await settle(page)
    readings.push(
      await page.evaluate(() => {
        /* `getBoundingClientRect().height` and never `offsetHeight`: the header
           is fractional, so `offsetHeight` rounds it and a three-term sum
           overshoots `innerHeight` at some widths and not at others. */
        const rect = (sel) => document.querySelector(sel).getBoundingClientRect()
        const header = rect('header')
        const main = rect('main')
        const footer = rect('footer')
        const brand = rect('#brand')
        return {
          width: innerWidth,
          innerHeight,
          header: header.height,
          main: main.height,
          footer: footer.height,
          sum: header.height + main.height + footer.height,
          inside: brand.left >= footer.left && brand.right <= footer.right,
          brand: document.getElementById('brand').textContent.trim(),
          cell: document.getElementById('edited').textContent
        }
      })
    )
  }
  const errors = await drainErrors(page)
  await page.close()

  const said = (r) =>
    `${r.width}px | header ${r.header} | footer ${r.footer} | sum ${r.sum} against ${r.innerHeight} | ` +
    `brand ${r.brand || '(empty)'} ${r.inside ? 'in the bar' : 'OUTSIDE the bar'}`
  for (const r of readings) note(said(r))

  ok(
    2,
    'the three boxes sum to innerHeight at every width, read off getBoundingClientRect',
    readings.every((r) => r.sum === r.innerHeight),
    readings.filter((r) => r.sum !== r.innerHeight).map(said).join('   ') || 'exact at every width'
  )

  /* The positive control is inside this clause rather than beside it: a sweep in
     which the header never grew was never narrow, and a brand that survived it
     has survived nothing. The header's own threshold is not encoded — that it
     grows below it is the property. */
  const widest = readings[0]
  const narrowest = readings[readings.length - 1]
  const grew = narrowest.header > widest.header
  const height = readings.every((r) => r.footer === widest.footer)
  const kept = readings.every((r) => r.brand === 'Letur' && r.inside)

  ok(
    3,
    'the footer keeps its height and its brand across a sweep to 240px, with a 58-character name',
    grew && height && kept && readings.every((r) => r.cell === LONG_NAME),
    `the header grew ${widest.header} to ${narrowest.header}: ${grew}; ` +
      `the footer held ${widest.footer}: ${height}; the brand stayed in the bar: ${kept}; ` +
      `the cell held the long name: ${readings.every((r) => r.cell === LONG_NAME)}`
  )
  return errors
}

/* 4. **What "places and never composes" is assertable as.** The sentence is
      worded in `app/src/preview.rs` out of `state` and `time`; the page places
      it. So: take those two values out of what the line holds, and what is left
      must carry no word of its own — a separator and nothing else. And the line
      must not move when the values the page *could* fold into it do. */
const statusPlaces = async (browser, url) => {
  const page = await opened(browser, url)

  const STATES = [
    { state: 'empty', time: null, error: null, page: false },
    { state: 'current', time: '31 ms', error: null, page: true },
    { state: 'stale', time: '12 ms', error: 'the harness put a sentence here', page: true },
    { state: 'failed', time: null, error: 'the harness put a sentence here', page: false }
  ]

  const read = []
  for (const patch of STATES) {
    read.push(
      await page.evaluate(async (patch) => {
        window.__harness.set(patch)
        window.__harness.fire('rendered')
        await new Promise((r) => setTimeout(r, 150))
        const status = document.getElementById('status')
        return { asked: patch, text: status.textContent, className: status.className }
      }, patch)
    )
  }

  /* The invariance half: the same two values, everything else moved. A page that
     folded a file name or a row count into the line would move here. */
  const before = read[1].text
  const after = await page.evaluate(async () => {
    window.__harness.set({
      state: 'current',
      time: '31 ms',
      edited: 'somewhere/else-entirely.md',
      main: 'another.md',
      entries: [{ path: 'another.md', kind: 'markdown', missing: false }]
    })
    window.__harness.fire('rendered')
    await new Promise((r) => setTimeout(r, 150))
    return document.getElementById('status').textContent
  })

  const errors = await drainErrors(page)
  await page.close()

  const composed = read.filter((r) => {
    const left = r.text.replace(r.asked.state, '').replace(r.asked.time ?? ' ', '')
    return /[\p{L}\p{N}]/u.test(left) || !r.text.startsWith(r.asked.state) || r.className !== r.asked.state
  })
  for (const r of read) note(`${r.asked.state}: ${JSON.stringify(r.text)} class ${JSON.stringify(r.className)}`)

  ok(
    4,
    'the status line carries no value the page chose, in any of the four states',
    composed.length === 0 && after === before,
    composed.length
      ? `composed: ${composed.map((r) => JSON.stringify(r.text)).join(', ')}`
      : `and did not move when edited, main and entries did: ${JSON.stringify(before)}`
  )
  return errors
}

/* 5. **The behaviour a reader could most reasonably expect to be the other
      way.** The bar names the file being typed in, which from this click is not
      the file the page beside it came from. Asserted against `main` as well as
      against `edited`: the two are equal at the open, so a cell wired to `main`
      passes everything until here. */
const cellFollowsThePane = async (browser, url) => {
  const page = await opened(browser, url)

  const atOpen = await page.evaluate(() => ({
    cell: document.getElementById('edited').textContent,
    status: window.__harness.status()
  }))

  const clicked = await page.evaluate(async () => {
    const row = [...document.getElementById('parts').children].find(
      (li) => !li.classList.contains('folder') && li.querySelector('.name')?.textContent === 'text.md'
    )
    const button = row?.querySelector('button.name')
    button?.click()
    await new Promise((r) => setTimeout(r, 500))
    return {
      clicked: !!button,
      cell: document.getElementById('edited').textContent,
      status: window.__harness.status()
    }
  })

  const errors = await drainErrors(page)
  await page.close()

  const last = (path) => (path === null ? '' : path.split('/').pop())
  note(`at the open: cell ${JSON.stringify(atOpen.cell)}, edited ${atOpen.status.edited}, main ${atOpen.status.main}`)
  note(
    `after the click: cell ${JSON.stringify(clicked.cell)}, edited ${clicked.status.edited}, main ${clicked.status.main}`
  )

  ok(
    5,
    'the cell is the last segment of `edited`, and after a row click that is not `main`',
    atOpen.cell === last(atOpen.status.edited) &&
      clicked.clicked &&
      clicked.status.edited !== clicked.status.main &&
      clicked.cell === last(clicked.status.edited) &&
      clicked.cell !== last(clicked.status.main) &&
      !clicked.cell.includes('/'),
    `clicked ${clicked.clicked}; wanted ${JSON.stringify(last(clicked.status.edited))}, ` +
      `got ${JSON.stringify(clicked.cell)}, and main's is ${JSON.stringify(last(clicked.status.main))}`
  )
  return errors
}

/* 6. Rust sends flat entries, already ordered, and never a directory: a heading
      belongs exactly where a path's leading segments differ from the last one's,
      one per newly entered segment, and the depth is how many segments precede
      the name. **The expectation is derived here from the entries** rather than
      read off the page, so this compares two derivations and not a page with
      itself. */
const panelDrawsTheEntries = async (browser, url) => {
  const page = await opened(browser, url)

  const read = await page.evaluate(() => ({
    rows: [...document.getElementById('parts').children].map((li) => ({
      folder: li.classList.contains('folder'),
      name: li.querySelector('.name')?.textContent ?? '',
      depth: li.dataset.depth
    })),
    entries: window.__harness.config.entries
  }))
  const errors = await drainErrors(page)
  await page.close()

  const wanted = []
  let folder = []
  for (const entry of read.entries) {
    const segments = entry.path.split('/')
    const here = segments.slice(0, -1)
    let shared = 0
    while (shared < here.length && here[shared] === folder[shared]) shared++
    for (let at = shared; at < here.length; at++) {
      wanted.push({ folder: true, name: here[at], depth: String(Math.min(at, 5)) })
    }
    folder = here
    wanted.push({ folder: false, name: segments[segments.length - 1], depth: String(Math.min(here.length, 5)) })
  }

  const same = JSON.stringify(read.rows) === JSON.stringify(wanted)
  note(`${read.entries.length} entries drew ${read.rows.length} rows, ${wanted.length} wanted`)

  ok(
    6,
    'the panel draws one row per entry, in order, with the folders derived',
    same,
    same
      ? read.rows.map((r) => `${r.folder ? '[dir] ' : ''}${r.name}@${r.depth}`).join('  ')
      : `got     ${JSON.stringify(read.rows)}\n          wanted  ${JSON.stringify(wanted)}`
  )
  return errors
}


/* 7. **Six readings, and the point is the two that a one-scheme suite would
      miss.** The palette has to win in *both* directions: `dark` chosen under a
      light system and `light` chosen under a dark one. The tokens are one half
      and `color-scheme` the other — it is what paints the `#fit` select, its
      arrow and the scrollbars, so a page whose tokens said dark while it said
      light would put a light scrollbar on a dark pane.

      **And `--paper` is unchanged in all six**, which is the clause that keeps
      `specs/desktop_app_spec.md` §1.1's narrowing honest: this app themes its
      own chrome and nothing about the document, the page Typst compiles being
      white in either palette. Every value is read against the page's own other
      readings — the system's own dark and its own light — so no colour literal
      is written here. */
const paletteTurnsBothWays = async (browser, url) => {
  const read = []
  const errors = { total: 0, loops: 0, spoken: [] }

  for (const system of ['light', 'dark']) {
    const page = await opened(browser, url, WIDTHS[0], system)
    for (const appearance of ['system', 'light', 'dark']) {
      read.push({
        system,
        appearance,
        ...(await page.evaluate(async (appearance) => {
          window.__harness.set({ appearance })
          window.__harness.fire('rendered')
          await new Promise((r) => setTimeout(r, 150))
          const root = document.documentElement
          const style = getComputedStyle(root)
          return {
            attribute: root.getAttribute('data-theme'),
            scheme: style.colorScheme,
            ground: style.getPropertyValue('--ground').trim(),
            ink: style.getPropertyValue('--ink').trim(),
            paper: style.getPropertyValue('--paper').trim()
          }
        }, appearance))
      })
    }
    const seen = await drainErrors(page)
    errors.total += seen.total
    errors.loops += seen.loops
    errors.spoken.push(...seen.spoken)
    await page.close()
  }

  const at = (system, appearance) => read.find((r) => r.system === system && r.appearance === appearance)

  /* The two the page has always had, and every other reading is compared to one
     of them rather than to a literal. */
  const lightGround = at('light', 'light').ground
  const darkGround = at('dark', 'dark').ground
  const wants = (r) => (r.appearance === 'system' ? r.system : r.appearance)

  const wrong = read.filter((r) => {
    const wanted = wants(r)
    const ground = wanted === 'dark' ? darkGround : lightGround
    const attribute = r.appearance === 'system' ? null : r.appearance
    return (
      r.attribute !== attribute ||
      r.ground !== ground ||
      !r.scheme.includes(wanted) ||
      (wanted === 'dark' ? r.scheme === 'light' : r.scheme === 'dark')
    )
  })

  const paper = new Set(read.map((r) => r.paper))
  for (const r of read) {
    note(`system ${r.system} + ${r.appearance}: attr ${r.attribute} scheme ${r.scheme} ground ${r.ground}`)
  }

  ok(
    7,
    'the palette turns both ways, and --paper turns in neither',
    wrong.length === 0 && paper.size === 1 && lightGround !== darkGround,
    wrong.length
      ? `wrong: ${wrong.map((r) => `${r.system}+${r.appearance}`).join(', ')}`
      : `six readings, --paper ${[...paper][0]} in all of them; the two grounds differ: ${lightGround} / ${darkGround}`
  )
  return errors
}

/* 8. **The boundary, which nothing read off the DOM alone can see.** A toggle
      that set the attribute itself would look identical from the page: same
      mark, same palette, same cycle. So this asserts both halves — that Rust
      moving the value alone moves the attribute, and that the click's only act
      is to ask, naming the next of the three.

      The stub's answer is what closes the loop, so the second half is measured
      after it: a page that had already moved the attribute before the answer
      arrived would be deciding. */
const cellPlacesAndDoesNotDecide = async (browser, url) => {
  const page = await opened(browser, url)

  /* Rust moves it, nobody clicks. */
  const placed = await page.evaluate(async () => {
    const seen = []
    for (const appearance of ['dark', 'light', 'system']) {
      window.__harness.set({ appearance })
      window.__harness.fire('rendered')
      await new Promise((r) => setTimeout(r, 150))
      seen.push({ appearance, attribute: document.documentElement.getAttribute('data-theme') })
    }
    return seen
  })

  /* The click, from a known state, with the boundary read rather than the DOM. */
  const clicked = await page.evaluate(async () => {
    window.__harness.set({ appearance: 'system' })
    window.__harness.fire('rendered')
    await new Promise((r) => setTimeout(r, 150))

    window.__harness.forget()
    document.getElementById('theme').click()
    await new Promise((r) => setTimeout(r, 150))

    return {
      asked: window.__harness.invokes().filter((i) => i.name === 'set_appearance'),
      attribute: document.documentElement.getAttribute('data-theme'),
      answered: window.__harness.status().appearance
    }
  })

  const errors = await drainErrors(page)
  await page.close()

  const misplaced = placed.filter((p) => p.attribute !== (p.appearance === 'system' ? null : p.appearance))
  /* system -> light is the cycle's first step, and the mark that follows is the
     stub's answer rather than the click. */
  const asked = clicked.asked.length === 1 && clicked.asked[0].args.appearance === 'light'
  const followed = clicked.attribute === 'light' && clicked.answered === 'light'

  note(`placed: ${placed.map((p) => `${p.appearance}->${p.attribute}`).join('  ')}`)
  note(`clicked: asked ${JSON.stringify(clicked.asked.map((i) => i.args))}, attribute ${clicked.attribute}`)

  ok(
    8,
    'the cell places what Rust says, and the click only asks',
    misplaced.length === 0 && asked && followed,
    misplaced.length
      ? `Rust moved and the page did not: ${JSON.stringify(misplaced)}`
      : `three placed with no click; one click asked for ${JSON.stringify(clicked.asked.map((i) => i.args.appearance))} and the attribute followed the answer`
  )
  return errors
}

/* 9. **Keyed to the group and not to the brand, because the brand cannot move.**
      An auto margin absorbs exactly the free space in total, so a last child
      with no right margin reads the same x under either layout — which is what
      falsified this clause's first draft. What separates them is the distance
      from the group to the brand, and it must equal the bar's own gap.

      **Read off the stylesheet, never written as a number**, per this file's
      one rule; and **taken at the sweep's widest width**, which is part of the
      clause rather than incidental: at 240px the 58-character name has filled
      `#edited` and left no free space for an auto margin to absorb, so both
      layouts read the gap and the clause would falsify nothing.

      The exact equality depends on `#controls` holding one flush, unpadded
      child. A second cell in that group would want measuring from `#controls`'
      own right edge instead. */
const groupSitsBesideTheBrand = async (browser, url) => {
  const page = await opened(browser, url, WIDTHS[0])

  const read = await page.evaluate(async () => {
    const footer = document.querySelector('footer')
    const theme = document.getElementById('theme')
    const brand = document.getElementById('brand')

    const seen = []
    for (const appearance of ['system', 'light', 'dark']) {
      window.__harness.set({ appearance })
      window.__harness.fire('rendered')
      await new Promise((r) => setTimeout(r, 150))
      seen.push({
        appearance,
        gap: brand.getBoundingClientRect().left - theme.getBoundingClientRect().right,
        brand: brand.getBoundingClientRect().left
      })
    }

    return {
      seen,
      columnGap: parseFloat(getComputedStyle(footer).columnGap),
      last: footer.lastElementChild === brand
    }
  })

  const errors = await drainErrors(page)
  await page.close()

  const apart = read.seen.filter((s) => Math.abs(s.gap - read.columnGap) > 0.5)
  const brands = new Set(read.seen.map((s) => s.brand.toFixed(2)))

  note(`gap ${read.seen.map((s) => `${s.appearance} ${s.gap.toFixed(2)}`).join('  ')}`)

  ok(
    9,
    `the icon group sits one gap from the brand at ${WIDTHS[0]}px, in all three states`,
    apart.length === 0 && brands.size === 1 && read.last,
    apart.length
      ? `the bar's own column-gap is ${read.columnGap}; read ${apart.map((s) => `${s.appearance} ${s.gap.toFixed(2)}`).join(', ')}`
      : `${read.columnGap} in all three, the brand still last and unmoved at ${[...brands][0]}`
  )
  return errors
}

/* ----------------------------------------------------------------- the run */

const run = async ({ engine, headed, rev, doc, mutate }) => {
  passed = 0
  failed = 0
  owned.length = 0

  const held = await serve({ rev, doc, mutate, quiet: true })
  const browser = await (engine === 'webkit' ? webkit : chromium).launch({ headless: !headed })
  const errors = { total: 0, loops: 0, spoken: [] }
  const gather = (seen) => {
    errors.total += seen.total
    errors.loops += seen.loops
    errors.spoken.push(...seen.spoken)
  }

  console.log(
    `\n${engine}${headed ? ', headed' : ''} | ${rev ?? 'the working tree'} | ${doc}` +
      `${mutate ? ` | MUTATED ${mutate}` : ''}\n`
  )

  try {
    for (const check of [
      elementOrder,
      sweep,
      statusPlaces,
      cellFollowsThePane,
      panelDrawsTheEntries,
      paletteTurnsBothWays,
      cellPlacesAndDoesNotDecide,
      groupSitsBesideTheBrand
    ]) {
      gather(await check(browser, held.url))
    }
  } finally {
    await browser.close()
    await held.close()
  }

  /* 10. **Named and not merely counted**, and the `ResizeObserver` class counted
         apart: that is the failure `mpdf-009` Phase 3 found twenty-one of in a
         single run, and an unrelated throw must still be visible beside it.
         **It stays last** — it is the only clause that accumulates across every
         other one, so its number moves as clauses are added and theirs do not. */
  ok(
    10,
    'no uncaught error reached the console through any of it',
    errors.total === 0,
    `${errors.total} uncaught, ${errors.loops} of them ResizeObserver` +
      (errors.spoken.length ? ` — ${errors.spoken.join(' | ')}` : '')
  )

  console.log(`\n${passed + failed} clauses: ${passed} passed, ${failed} failed`)
  return { passed, failed, owned: [...owned] }
}

const engine = has('webkit') ? 'webkit' : 'chromium'
const shared = { engine, headed: has('headed'), rev: flag('rev'), doc: flag('doc', 'tests/fixtures/panel') }

if (has('falsify')) {
  /* The gate's clause 3, judged rather than read: each mutation must fail exactly
     the clause that owns it and no other.

     **One child process per mutation, and that is measured rather than tidy.** A
     second or third `chromium.launch()` in the same process hangs — no browser
     alive, the driver waiting on a promise that never settles — which cost this
     phase two runs and is the same thing the A/B driver forks around. Each child
     is this file with `--mutate`, so the forked path and the single-mutation path
     are one path, and its exit code already means "isolated". */
  const rest = argv.filter((a) => a !== '--falsify')
  const verdicts = Object.keys(OWNS).map((mutate) => {
    const child = spawnSync(process.execPath, [fileURLToPath(import.meta.url), ...rest, '--mutate', mutate], {
      stdio: 'inherit'
    })
    return { mutate, clause: OWNS[mutate], isolated: child.status === 0 }
  })
  console.log(`\nfalsification in ${engine}:`)
  for (const v of verdicts) {
    console.log(`  ${v.isolated ? 'ISOLATED    ' : 'NOT ISOLATED'}  ${v.mutate} owns clause ${v.clause}`)
  }
  process.exit(verdicts.every((v) => v.isolated) ? 0 : 1)
}

const mutate = flag('mutate')
const answer = await run({ ...shared, mutate })

if (mutate) {
  const isolated = answer.owned.length === 1 && answer.owned[0] === OWNS[mutate]
  console.log(
    `${isolated ? 'ISOLATED' : 'NOT ISOLATED'}  ${mutate} owns clause ${OWNS[mutate]}, failed [${answer.owned.join(', ')}]`
  )
  process.exit(isolated ? 0 : 1)
}

process.exit(answer.failed === 0 ? 0 : 1)
