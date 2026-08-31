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
   fails; `--falsify` runs all eleven. That is the gate's clause 3, run rather
   than read.

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
   check that would pass on a page the scope forbids.

   **Two mutations may own one clause without either being redundant**, and
   clause 3 is where that stands: `flex-min` reaches the footer's half — the
   brand pushed out of a bar that holds one line — and `header-wraps` reaches
   the header's, a pinned box whose children have left it. `views-one-way` was
   the other such pair and was withdrawn when the header gave its copies up:
   it reached the sync between a toggle's two copies, and there is one copy. */
const OWNS = {
  'footer-last': 1,
  'flex-min': 3,
  'header-wraps': 3,
  'cell-main': 5,
  'theme-dark-attr': 7,
  'theme-click-direct': 8,
  'controls-auto-margin': 9,
  'marks-unlit': 10,
  'figure-unnamed': 11,
  'save-as-mislabelled': 12,
  'receipt-sticks': 13
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
   two bars cost the column, and clause 3 is what each bar holds while the
   window narrows — the header the box it declares, the footer its height and
   its brand. `flex-min` moves the second without moving the first, and
   `header-wraps` moves the header's half of the second without moving either
   of the others. */
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
        /* `getBoundingClientRect().height` and never `offsetHeight`: the
           header was fractional when this was written and `offsetHeight` rounds
           it, so a three-term sum overshoots `innerHeight` at some widths and
           not at others. `rules/desktop-geometry.md` has which engine's numbers
           those were. */
        const rect = (sel) => document.querySelector(sel).getBoundingClientRect()
        const header = rect('header')
        const main = rect('main')
        const footer = rect('footer')
        const brand = rect('#brand')

        /* **The header's own rule, off the CSSOM, and never `getComputedStyle`
           — and the reason is not the driver's reason.** There, on the footer,
           that call resolves to the *used* height and would grow with the
           content, holding however tall the bar got. Here the header's height
           is pinned, so it never grows: `getComputedStyle` returns `27px` under
           the mutation too, and the reading would not be wrong, it would be
           **vacuous** — and it would go on being vacuous on the day someone
           drops the `height` declaration. The sheet fails loudly instead. Last
           match wins, the harness serving a stub stylesheet of its own. */
        let declared = null
        for (const sheet of document.styleSheets) {
          for (const rule of sheet.cssRules) {
            if (rule.selectorText === 'header' && rule.style.height) declared = parseFloat(rule.style.height)
          }
        }

        const bar = document.querySelector('header')
        return {
          width: innerWidth,
          innerHeight,
          header: header.height,
          declared,
          /* The border is beside the declared height because the page sets no
             global `box-sizing`, so the 1px rule is outside the box — which is
             how `app/driver/drive.mjs` reads the footer's. */
          border: parseFloat(getComputedStyle(bar).borderBottomWidth),
          /* **The half a pin can lie about.** A flex container with an explicit
             `height` does not grow when its line wraps — its content overflows
             — so a clause reading only the height would hold on a page whose
             children had left the bar. */
          outside: [...bar.children].filter((child) => {
            const box = child.getBoundingClientRect()
            return box.top < header.top - 0.5 || box.bottom > header.bottom + 0.5
          }).length,
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
    `${r.width}px | header ${r.header} against ${r.declared} + ${r.border}, ${r.outside} children outside | ` +
    `footer ${r.footer} | sum ${r.sum} against ${r.innerHeight} | ` +
    `brand ${r.brand || '(empty)'} ${r.inside ? 'in the bar' : 'OUTSIDE the bar'}`
  for (const r of readings) note(said(r))

  ok(
    2,
    'the three boxes sum to innerHeight at every width, read off getBoundingClientRect',
    readings.every((r) => r.sum === r.innerHeight),
    readings.filter((r) => r.sum !== r.innerHeight).map(said).join('   ') || 'exact at every width'
  )

  /* **The positive control for this sweep is `flex-min`**, and naming it is
     what stops the clause proving the viewport ever narrowed by nothing. That
     mutation bites only at 240px with the 58-character name, so a run in which
     it isolates is a run that reached a narrow viewport with a full cell. It
     used to be `grew` — the header wrapping below its own threshold — and that
     control went when the header stopped wrapping at all. */
  const widest = readings[0]
  if (readings.some((r) => r.declared === null))
    throw new Error('no `header` rule declares a height — the reading this clause takes is not in the sheet')

  const off = (a, b) => Math.abs(a - b) > 0.5
  const pinned = readings.every((r) => !off(r.header, r.declared + r.border))
  const held = readings.every((r) => r.outside === 0)
  const height = readings.every((r) => r.footer === widest.footer)
  const kept = readings.every((r) => r.brand === 'Letur' && r.inside)

  ok(
    3,
    'the header is the box its own rule declares and holds every child inside it, and the footer keeps its height and its brand across a sweep to 240px with a 58-character name',
    pinned && held && height && kept && readings.every((r) => r.cell === LONG_NAME),
    `the header was ${widest.declared} + ${widest.border} everywhere: ${pinned}; ` +
      `no child outside it: ${held}; ` +
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
      and `color-scheme` the other — it is what paints the `#fit-footer` select,
      its arrow and the scrollbars, so a page whose tokens said dark while it
      said light would put a light scrollbar on a dark pane.

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
      mark, same palette, same flip. So this asserts both halves — that Rust
      moving the value alone moves the attribute, and that the click's only act
      is to ask, naming the other of the two.

      **Three presses and not one**, because the button has two positions and
      Rust three values: `system` is the unset state, and what a press from it
      must ask for is read off the *system*, not off the value. Under this
      page's light scheme that is `dark` — the same answer a press from `light`
      gives, which is why the `dark` press is here too and is the one that
      separates "the other of the two" from "always dark".

      The stub's answer is what closes the loop, so the asking half is measured
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

  /* The click, from each of the three, with the boundary read rather than the
     DOM. The page is under the default light scheme, so `system` is light in
     effect and a press from it must ask for `dark`. */
  const clicked = []
  for (const [from, wanted] of [
    ['system', 'dark'],
    ['dark', 'light'],
    ['light', 'dark']
  ]) {
    clicked.push({
      from,
      wanted,
      ...(await page.evaluate(async (from) => {
        window.__harness.set({ appearance: from })
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
      }, from))
    })
  }

  const errors = await drainErrors(page)
  await page.close()

  const misplaced = placed.filter((p) => p.attribute !== (p.appearance === 'system' ? null : p.appearance))
  const wrong = clicked.filter(
    (c) =>
      c.asked.length !== 1 ||
      c.asked[0].args.appearance !== c.wanted ||
      c.attribute !== c.wanted ||
      c.answered !== c.wanted
  )

  note(`placed: ${placed.map((p) => `${p.appearance}->${p.attribute}`).join('  ')}`)
  note(
    `clicked: ${clicked.map((c) => `${c.from} asked ${c.asked.map((i) => i.args.appearance).join('+') || 'nothing'}`).join('  ')}`
  )

  ok(
    8,
    'the cell places what Rust says, and the click only asks, for the other of the two',
    misplaced.length === 0 && wrong.length === 0,
    misplaced.length
      ? `Rust moved and the page did not: ${JSON.stringify(misplaced)}`
      : wrong.length
        ? `wrong: ${wrong.map((c) => `from ${c.from} wanted ${c.wanted}, asked ${c.asked.map((i) => i.args.appearance).join('+') || 'nothing'}`).join('; ')}`
        : `three placed with no click; three clicks each asked for the other and the attribute followed the answer`
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

      **Measured from `#controls`' own right edge, and there are two gaps to
      cross.** The clause's first draft read the theme button's edge, which was
      the group's right edge only while the group held one flush, unpadded
      child; the fit select made that false, and the separator put a third cell
      between the group and the brand. So the reading is the group to the
      separator and the separator to the brand, each one bar-gap.

      **The second of the two is what still separates the layouts.** With the
      auto margin on the group, the separator and the brand are packed at the
      right; moved to the brand, the free space opens between them — while the
      group-to-separator gap reads the bar's own under either. A clause that
      asserted only the first would falsify nothing. */
const groupSitsBesideTheBrand = async (browser, url) => {
  const page = await opened(browser, url, WIDTHS[0])

  const read = await page.evaluate(async () => {
    const footer = document.querySelector('footer')
    const controls = document.getElementById('controls')
    const sep = document.getElementById('sep-brand')
    const brand = document.getElementById('brand')

    const seen = []
    for (const appearance of ['system', 'light', 'dark']) {
      window.__harness.set({ appearance })
      window.__harness.fire('rendered')
      await new Promise((r) => setTimeout(r, 150))
      seen.push({
        appearance,
        gap: sep.getBoundingClientRect().left - controls.getBoundingClientRect().right,
        toBrand: brand.getBoundingClientRect().left - sep.getBoundingClientRect().right,
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

  const off = (n) => Math.abs(n - read.columnGap) > 0.5
  const apart = read.seen.filter((s) => off(s.gap) || off(s.toBrand))
  const brands = new Set(read.seen.map((s) => s.brand.toFixed(2)))

  const said = (s) => `${s.appearance} ${s.gap.toFixed(2)} / ${s.toBrand.toFixed(2)}`
  note(`gap group→sep / sep→brand: ${read.seen.map(said).join('  ')}`)

  ok(
    9,
    `the icon group, the separator and the brand sit one gap apart at ${WIDTHS[0]}px, in all three states`,
    apart.length === 0 && brands.size === 1 && read.last,
    apart.length
      ? `the bar's own column-gap is ${read.columnGap}; read ${apart.map(said).join(', ')}`
      : `${read.columnGap} across both, in all three, the brand still last and unmoved at ${[...brands][0]}`
  )
  return errors
}

/* 10. **Each toggle works the pane it names, and its mark says which state it
       is in.** This clause used to assert one setting behind *two* controls,
       the bar's `Files` and `Lines` duplicating the header's, and it was
       re-keyed when the header gave its copies up: a copy that cannot disagree
       cannot exist, and `views-one-way` — the mutation that reached that
       disagreement — was withdrawn with it.

       **The pane is read and not just the attribute.** A control that placed
       its own state while the panel stayed open would satisfy every ARIA
       reading and be the defect this exists to catch. */
const viewsWorkTheirPanes = async (browser, url) => {
  const page = await opened(browser, url)

  const read = () =>
    page.evaluate(() => ({
      files: document.getElementById('views-files').getAttribute('aria-expanded'),
      lines: document.getElementById('views-lines').getAttribute('aria-pressed'),
      panel: !document.getElementById('files').classList.contains('collapsed'),
      gutter: !document.getElementById('lines').hidden,
      /* **The ink each mark is wearing**, because these two are marks and a
         mark says nothing a word does not. Since they carry no text, the
         *only* visible difference between on and off is this colour — so a
         stylesheet that lost the rule would leave two identical icons and every
         ARIA reading above would still pass. Which value it is is not the
         clause; that the two states differ is. */
      ink: {
        files: getComputedStyle(document.getElementById('views-files')).color,
        lines: getComputedStyle(document.getElementById('views-lines')).color
      }
    }))

  /* **Still four presses, and that is what the header's two rows became rather
     than what is left when they go.** Each toggle is pressed twice, once each
     way, because a single press would leave its mark in one ink — and *each
     mark's two inks* is the half of this clause `marks-unlit` owns and the
     requirement that determined this rewrite. */
  const pressed = []
  for (const [selector, which] of [
    ['#views-files', 'files'],
    ['#views-lines', 'lines'],
    ['#views-files', 'files'],
    ['#views-lines', 'lines']
  ]) {
    const before = await read()
    await page.click(selector)
    /* **Off the control before the colour is read.** A click leaves the pointer
       where it landed, `:hover` paints the mark with the same ink `on` does,
       and the reading below would then say the two states match whatever the
       toggle is actually in. Measured, not reasoned about: without this the
       off state read as ink in three of the four presses. */
    await page.mouse.move(0, 0)
    await settle(page)
    const after = await read()
    pressed.push({ press: pressed.length + 1, which, before, after })
  }

  const errors = await drainErrors(page)
  await page.close()

  /* The control says what the pane says, and the pane moved. `Files` is
     expanded-when-open, `Lines` pressed-when-shown, so each is compared against
     the box it works rather than against a literal. */
  const wrong = pressed.filter((p) => {
    const said = p.which === 'files' ? p.after.files : p.after.lines
    const box = p.which === 'files' ? p.after.panel : p.after.gutter
    const moved = String(p.which === 'files' ? p.before.panel : p.before.gutter) !== String(box)
    return said !== String(box) || !moved
  })

  /* The four presses put each toggle in both states, so each mark's two inks
     are in hand without a fifth reading. */
  const inks = (which) => new Set(pressed.filter((p) => p.which === which).map((p) => p.after.ink[which]))
  const marked = inks('files').size === 2 && inks('lines').size === 2

  for (const p of pressed)
    note(
      `press ${p.press}, ${p.which}: the control ${p.after[p.which]}, ` +
        `the pane ${p.which === 'files' ? p.after.panel : p.after.gutter}, the mark ${p.after.ink[p.which]}`
    )

  ok(
    10,
    'each view toggle works the pane it names, and its mark shows which state it is in',
    wrong.length === 0 && marked,
    wrong.length || !marked
      ? `${wrong.map((p) => `press ${p.press}, ${p.which}: ${JSON.stringify(p.after)}`).join('; ')}` +
        `${marked ? '' : ` — the marks: files ${[...inks('files')].join(' / ')}, lines ${[...inks('lines')].join(' / ')}`}`
      : 'four presses, the control and the pane agreeing after every one, each mark two inks'
  )
  return errors
}

/* 11. **The cell names what the pane is holding, and a figure is not
       `edited`.** Clicking an image row opens a surface over the text and never
       moves `Status::edited` — it cannot, `edited` being the file being typed
       in — so before this the bar named a markdown file that had not been on
       screen since the click. Both surfaces are asserted: the drawn figure and
       the sentence a `.pdf` row gets, which is still a surface the pane is
       holding.

       **And the way back is half the clause.** A cell that took the figure's
       name and kept it would read correctly in exactly the reading a one-ended
       check makes, so `Escape` is pressed and the markdown name must return. */
const cellNamesTheFigure = async (browser, url) => {
  const page = await opened(browser, url)

  const cell = () => page.evaluate(() => document.getElementById('edited').textContent)
  const clickRow = async (name) => {
    await page.evaluate((name) => {
      const row = [...document.querySelectorAll('#parts li')].find((li) => li.textContent.includes(name))
      row?.querySelector('button.name')?.click()
    }, name)
    await settle(page)
  }

  const before = await cell()
  await clickRow('mark.svg')
  const figure = await cell()
  /* **That the sheet holds a picture is part of the clause, not colour.** A
     refused read reaches the same surface through `saySoInstead` and names the
     same file, so a clause that only asked whether the surface was up passed
     against a harness that could not serve a figure at all — which is what it
     did, until `serve.mjs` started copying the project's images in. This is the
     reading that says the drawn path was the one taken. */
  const drawn = await page.evaluate(
    () => !document.getElementById('viewer').hidden && !!document.querySelector('#viewer .sheet img')
  )
  await clickRow('plan.pdf')
  const said = await cell()
  await page.keyboard.press('Escape')
  await settle(page)
  const back = await cell()

  const errors = await drainErrors(page)
  await page.close()

  note(`the cell: ${before} → mark.svg gives ${figure} → plan.pdf gives ${said} → Escape gives ${back}`)

  ok(
    11,
    'the cell names the figure the pane is holding, and the edited file again when it is left',
    figure === 'mark.svg' && drawn && said === 'plan.pdf' && back === before && before !== '',
    `opened on ${JSON.stringify(before)}; the figure gave ${JSON.stringify(figure)} with a picture drawn ${drawn}; ` +
      `the pdf's sentence gave ${JSON.stringify(said)}; Escape gave ${JSON.stringify(back)}`
  )
  return errors
}

/* 12. **The header's second mark says what it does, in both of its names.**
       `mpdf-003` Phase 17 turned this button from `Save` into `Save as…`, and
       Phase 16 shipped a recorded drop — nothing in either rig read the header's
       children at all — which this ends. It is the page's only visible change in
       that phase, and a mark that named the wrong action would be the exact
       defect Phase 16 deferred the rename to avoid: a button saying what it
       would do next release.

       **Both names and not one**, `wearAppearance`'s rule and the footer's: the
       `title` is what a sighted reader hovers for and the `aria-label` is what a
       screen reader says, so a page that moved one and not the other would tell
       two readers two different things. `title` renders in the shipping
       WKWebView — read at the window, since neither rig can see a native
       tooltip. */
const theSaveMarkSaysWhatItDoes = async (browser, url) => {
  const page = await opened(browser, url)

  const read = await page.evaluate(() => {
    const button = document.getElementById('save')
    return {
      title: button.getAttribute('title'),
      label: button.getAttribute('aria-label'),
      text: button.textContent.trim()
    }
  })
  const errors = await drainErrors(page)
  await page.close()

  note(`#save: title ${JSON.stringify(read.title)}, aria-label ${JSON.stringify(read.label)}`)

  ok(
    12,
    'the header\'s second mark says `Save as…` in both of its names',
    read.title === 'Save as…' && read.label === 'Save as…' && read.text === '',
    read.title === read.label
      ? `both say ${JSON.stringify(read.title)}`
      : `title ${JSON.stringify(read.title)} against aria-label ${JSON.stringify(read.label)}`
  )
  return errors
}

/* 13. **A save says what it did, and then stops saying it.**
       `mpdf-003` Phase 19 gave the bar a fifth cell, and the sentence in it is
       Rust's: `app/src/main.rs:save` answers it and `sayReceipt` places it. So
       this drives the page's own `save` listener — the menu's event, since `⌘S`
       has no button — and asserts the cell holds exactly what the stub answered.

       **Both halves, and the second is the one that needs the wait.** A receipt
       that appeared would pass a check written for the first half alone while the
       bar carried a stale sentence for the rest of the session, which is what the
       `receipt-sticks` mutation is. **It reads the cell's emptiness and never the
       timer's length**: no interval is written here, and the wait is
       `waitForFunction`'s own default, so the four seconds could move in the page
       without touching this file.

       **`Save as…` is deliberately not driven.** `stub.mjs`'s `dialog.save`
       answers `null`, so `saveDocumentAs` returns before it reaches the command —
       a rig that made that dialog answer would be testing a panel the app does
       not have. The window gate reads that half. */
const theSaveSaysWhatItDidAndThenStops = async (browser, url) => {
  const page = await opened(browser, url)

  const before = await page.evaluate(() => document.getElementById('receipt').textContent)

  await page.evaluate(() => {
    window.__harness.forget()
    window.__harness.fire('save')
  })

  const said = await page
    .waitForFunction(() => document.getElementById('receipt').textContent || null)
    .then((held) => held.jsonValue())
    .catch(() => '')

  /* The boundary, for the reason `stub.mjs` keeps the log at all: a page that
     worded its own receipt would look identical in the DOM. */
  const asked = await page.evaluate(() => window.__harness.invokes().map((sent) => sent.name))

  const cleared = await page
    .waitForFunction(() => document.getElementById('receipt').textContent === '')
    .then(() => true)
    .catch(() => false)

  const errors = await drainErrors(page)
  await page.close()

  note(`the cell: ${JSON.stringify(before)} → the save gave ${JSON.stringify(said)} → ${cleared ? 'empty again' : 'still there'}`)
  note(`what it asked Rust for: ${asked.join(', ') || 'nothing'}`)

  ok(
    13,
    'a plain save places the receipt Rust answered, and the cell is empty again after',
    before === '' && said === 'saved' && asked.includes('save') && cleared,
    `opened on ${JSON.stringify(before)}; the save gave ${JSON.stringify(said)} ` +
      `after asking for [${asked.join(', ')}]; it cleared: ${cleared}`
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
      groupSitsBesideTheBrand,
      viewsWorkTheirPanes,
      cellNamesTheFigure,
      theSaveMarkSaysWhatItDoes,
      theSaveSaysWhatItDidAndThenStops
    ]) {
      gather(await check(browser, held.url))
    }
  } finally {
    await browser.close()
    await held.close()
  }

  /* **Named and not merely counted**, and the `ResizeObserver` class counted
     apart: that is the failure `mpdf-009` Phase 3 found twenty-one of in a
     single run, and an unrelated throw must still be visible beside it.
     **It stays last** — it is the only clause that accumulates across every
     other one, so its number moves as clauses are added and theirs do not. */
  ok(
    14,
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
