/* mpdf-009 exit gate — Phase 5's clauses and Phase 3's. Paste into the Web
   Inspector console of a `cargo tauri dev` window, with the machine set to
   always-show scrollbars.

   Phase 3's clauses are labelled `P3·n`; Phase 5's are bare numbers. The two
   phases number their own gates from 1 and this is one script, which is the
   whole reason for the prefix.

   ORDER:
     __gate.arm()          <- BEFORE opening anything, from the empty state
     open tests/fixtures/long.md
     await __gate.long()   <- P5 clauses 2, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15
     await __gate.fits()   <- P3 clauses 1 (width, manual), 3, 4, 5, 6, 7, 8
     widen the window until pages.clientWidth exceeds __gate.boundary()
     await __gate.wide()   <- P3 clauses 1 (page), 2, 5, 7 — then narrow it back
     open tests/fixtures/near.md
     await __gate.near()   <- P5 clause 11, P3 clause 9
     open samples/showcase/showcase.md
     await __gate.showcase()  <- P5 clause 1's geometry, P3 clause 10
   P5 clauses 12 and 13 are driven by __gate.long() and named at the end.
   P3 clause 11 is `cargo test --workspace`, outside this script.            */
;(() => {
  const pages = document.getElementById('pages')
  const text = document.getElementById('text')
  const fitEl = document.getElementById('fit')
  const P = () => window.__pane
  let noise = 0
  let records = null
  let observer = null

  const wait = (ms) => new Promise((r) => setTimeout(r, ms))
  const frame = () => new Promise((r) => requestAnimationFrame(r))
  const MiB = (n) => +(n / 2 ** 20).toFixed(2)

  const idle = async () => {
    let last = -1
    for (let i = 0; i < 120; i++) {
      await wait(150)
      if (P().renders === last) return
      last = P().renders
    }
  }
  const held = () =>
    [...pages.querySelectorAll('canvas')].reduce((n, c) => n + c.width * c.height * 4, 0)
  const whole = () => {
    const r = devicePixelRatio
    return [...pages.children].reduce(
      (n, w) => n + Math.floor(w.logical.w * r) * Math.floor(w.logical.h * r) * 4,
      0
    )
  }
  const at = () => {
    const k = pages.children
    const top = pages.scrollTop
    let i = k.length - 1
    for (let n = 0; n < k.length; n++) {
      if (k[n].offsetTop + k[n].offsetHeight > top) { i = n; break }
    }
    return { page: i + 1, fraction: (top - k[i].offsetTop) / (k[i].offsetHeight || 1) }
  }
  const sharp = () =>
    [...pages.children].every((w) => {
      const c = w.querySelector('canvas')
      return c === null || c.width === Math.floor(w.logical.w * devicePixelRatio)
    })
  const layersMatch = () => {
    const c = pages.querySelectorAll('canvas').length
    return (
      c === pages.querySelectorAll('.textLayer').length &&
      c === pages.querySelectorAll('.annotationLayer').length &&
      [...pages.children].every((w) => {
        const has = w.querySelector('canvas') !== null
        return (
          (w.querySelector('.textLayer') !== null) === has &&
          (w.querySelector('.annotationLayer') !== null) === has
        )
      })
    )
  }
  const width = async (px) => {
    for (let i = 0; i < 40 && pages.clientWidth !== px; i++) {
      const b = parseFloat(getComputedStyle(text).flexBasis) || text.clientWidth
      text.style.flexBasis = `${b + (pages.clientWidth - px)}px`
      await frame()
    }
    await wait(900)
    await idle()
  }
  const compile = async () => {
    const before = text.selectionStart
    text.value = `${text.value} `
    text.setSelectionRange(before, before)
    text.dispatchEvent(new Event('input', { bubbles: true }))
    await wait(1200)
    await idle()
  }

  /* ---- Phase 3's own instruments -------------------------------------- */

  // Change the fit the way a reader does. The app's handler is on `change`,
  // and setting `.value` fires nothing by itself.
  const setFit = async (value) => {
    fitEl.value = value
    fitEl.dispatchEvent(new Event('change', { bubbles: true }))
    await wait(500)
    await idle()
  }

  /*
    The pane width at which fit-page stops being fit-width. **Rounded and not
    floored**: `layoutPages` rounds the CSS box it lays out, so a floor misses
    it by a pixel at heights where the fraction lands above a half. It moves
    with the height, which is why it is an expression and not a number — and
    it is read under a fit that overflows nothing sideways, so the horizontal
    track a pinned scale adds is not in it.
  */
  const boundary = () => Math.round((pages.clientHeight * 595.28) / 841.89)

  // A page's own CSS box, which is what a fit writes and the one number a
  // pinned scale must hold to the pixel.
  const cssW = (i = 0) => Math.round(parseFloat(pages.children[i].style.width))
  const cssH = (i = 0) => Math.round(parseFloat(pages.children[i].style.height))
  const count = () => pages.querySelectorAll('canvas').length
  const one = (i = 0) =>
    Math.floor(cssW(i) * devicePixelRatio) * Math.floor(cssH(i) * devicePixelRatio) * 4

  /*
    A fit change with the canvas count sampled on **every frame through it**,
    not just at its end. Every other clause reads a settled pane, which is
    exactly where a transition that empties itself hides — the sampling blind
    spot Phase 5's post-ship fix documents, given a clause of its own here.
  */
  const through = async (value) => {
    let fewest = count()
    let running = true
    const sample = () => {
      fewest = Math.min(fewest, count())
      if (running) requestAnimationFrame(sample)
    }
    requestAnimationFrame(sample)
    await setFit(value)
    running = false
    return fewest
  }

  // A fit change, and whether it carried the reader.
  const carried = async (value, name) => {
    const from = at()
    const fewest = await through(value)
    const to = at()
    return {
      name,
      ok: to.page === from.page && Math.abs(to.fraction - from.fraction) < 0.01,
      fewest,
      say: `${name} p${from.page}/${from.fraction.toFixed(3)}→p${to.page}/${to.fraction.toFixed(3)}`
    }
  }

  // Put the caret a fraction of the way through the buffer, so the compile
  // after it is OQ-2 case 2 landing where the caret is rather than at page 1.
  const caretAt = (fraction) => {
    const n = Math.floor(text.value.length * fraction)
    text.focus()
    text.setSelectionRange(n, n)
  }

  // Drag the divider to a pane width in steps, sampling on every frame, so
  // what a fit does *during* a gesture is readable and not only what it
  // settles at.
  const drag = async (to, sample) => {
    const from = pages.clientWidth
    for (let i = 1; i <= 12; i++) {
      const want = Math.round(from + ((to - from) * i) / 12)
      const b = parseFloat(getComputedStyle(text).flexBasis) || text.clientWidth
      text.style.flexBasis = `${b + (pages.clientWidth - want)}px`
      await frame()
      if (sample) sample()
    }
    await wait(900)
    await idle()
  }

  const pre3 = (wide) => {
    let good = true
    const w = pages.clientWidth
    const b = boundary()
    if (devicePixelRatio !== 2) {
      good = false
      note(`devicePixelRatio is ${devicePixelRatio}, not 2 — every MiB literal below is void`)
    }
    if (wide && w <= b) {
      good = false
      note(`pane is ${w}, at or under the fit-page boundary ${b} — widen the window until it is above, or fit-page is fit-width and these clauses test nothing`)
    }
    if (!wide && w >= b) {
      good = false
      note(`pane is ${w}, at or above the boundary ${b} — narrow it; these clauses want the geometry where fit-page and fit-width agree`)
    }
    ok('P3·0', 'preconditions', good, `dpr ${devicePixelRatio}, pane ${w}×${pages.clientHeight}, boundary ${b}`)
    return good
  }

  let pass = 0
  let fail = 0
  /*
    Every line is kept as well as logged, because Safari's Web Inspector copies
    one console entry at a time and this script writes one per clause. Run
    `__gate.report()` at the end and the whole run comes back as a single entry,
    which is what actually gets pasted back.
  */
  const transcript = []
  const ok = (n, name, good, detail) => {
    good ? pass++ : fail++
    transcript.push(
      `${good ? 'PASS' : 'FAIL'}  ${String(n).padStart(2)}. ${name}${detail ? '  —  ' + detail : ''}`
    )
    console.log(
      `%c${good ? 'PASS' : 'FAIL'}%c  ${String(n).padStart(2)}. ${name}${detail ? '  —  ' + detail : ''}`,
      `font-weight:bold;color:${good ? '#137333' : '#c5221f'}`,
      'color:inherit'
    )
  }
  const note = (s) => {
    transcript.push(`····  ${s}`)
    console.log(`%c····%c  ${s}`, 'color:#888', 'color:#888')
  }
  const heading = (s) => {
    transcript.push('', `== ${s}`)
    console.log(`%c\n${s}\n`, 'font-weight:bold')
  }
  const tally = (what) => {
    transcript.push(`${what}: ${pass} passed, ${fail} failed`)
    console.log(
      `%c${what}: ${pass} passed, ${fail} failed`,
      `font-weight:bold;color:${fail ? '#c5221f' : '#137333'}`
    )
    const answer = { passed: pass, failed: fail }
    pass = 0
    fail = 0
    return answer
  }

  const pre = () => {
    let good = true
    const dpr = devicePixelRatio
    const w = pages.clientWidth
    const h = pages.clientHeight
    if (dpr !== 2) { good = false; note(`devicePixelRatio is ${dpr}, not 2 — every MiB literal below is void`) }
    if (w !== 520) { good = false; note(`pane is ${w} px, not 520 — 535 means overlay scrollbars; set the machine to always show them`) }
    if (h < 1001 || h > 1126) { good = false; note(`pane height is ${h}, outside 1001–1126 — the band is not 3 at the ends and 5–6 in the middle`) }
    ok(0, 'preconditions', good, `dpr ${dpr}, pane ${w}×${h}`)
    return good
  }

  window.__gate = {
    /*
      The whole run as one console entry, for copying back in one gesture. Run
      it last, after every entry point that this session is going to run.
    */
    report() {
      console.log(`mpdf-009 gate — dpr ${devicePixelRatio}\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      addEventListener('error', () => noise++)
      addEventListener('unhandledrejection', () => noise++)
      records = []
      observer?.disconnect()
      observer = new MutationObserver((rs) => {
        for (const r of rs) records.push({ added: r.addedNodes.length, removed: r.removedNodes.length })
      })
      observer.observe(pages, { childList: true })
      console.log('%carmed%c  — now open tests/fixtures/long.md, then run: await __gate.long()',
        'font-weight:bold;color:#1a73e8', 'color:inherit')
    },

    async long() {
      heading('long.md — 71 pages')
      await idle()
      if (!pre()) note('clauses below still run; read their numbers as reports, not results')

      const n = pages.children.length
      const one = whole() / (n || 1)

      // 4 — the sizing pass commits once, not once per page. Keyed to the
      // records that ADD pages: `clear` leaves a removal record of its own, and
      // a re-open over a document of the same length legitimately adds nothing,
      // because the pass reuses the pages already there.
      const adds = (records || []).filter((r) => r.added > 0)
      ok(4, 'the sizing pass commits once', records !== null && adds.length <= 1 &&
        adds.every((r) => r.added === n),
        records === null ? 'not armed — run __gate.arm() before the open'
          : adds.length === 0 ? `no page was added: the pass reused all ${n}, so this ran over a re-open rather than from the empty state`
          : `${adds.length} adding record(s), ${adds[0].added} pages at once, wanted ${n}`)

      // 5 — the extent is exact from layout alone.
      ok(5, 'the extent is exact from layout', pages.scrollHeight === 53337,
        `scrollHeight ${pages.scrollHeight}, wanted 53337`)

      // 2 — retention is the band, not the document.
      pages.scrollTop = 0; await wait(500); await idle()
      const top = held()
      pages.scrollTop = pages.scrollHeight / 2; await wait(500); await idle()
      const mid = held()
      pages.scrollTop = pages.scrollHeight; await wait(500); await idle()
      const end = held()
      ok(2, 'retention is the band and not the whole', P().mode === 'band' &&
        MiB(top) < MiB(whole()) / 4 && MiB(end) < MiB(whole()) / 4,
        `top ${MiB(top)}, mid ${MiB(mid)}, end ${MiB(end)} MiB against ${MiB(whole())} whole; ` +
        `spec: 17.5 / 29.16–34.99 / 17.5 against 414`)

      // 3 — nothing is retained outside #pages.
      ok(3, 'nothing is retained outside #pages',
        P().made - P().released === pages.querySelectorAll('canvas').length,
        `made−released ${P().made - P().released}, canvases ${pages.querySelectorAll('canvas').length}`)

      // 9 — layers track canvases, six round trips.
      let layers = true
      for (let i = 0; i < 6; i++) {
        pages.scrollTop = pages.scrollHeight * (i % 2 ? 0.1 : 0.8)
        await wait(400); await idle()
        if (!layersMatch()) layers = false
      }
      ok(9, 'layers track canvases across release and re-entry', layers)

      // 6 — the reader does not move when canvases are released.
      pages.scrollTop = pages.scrollHeight / 2; await wait(500); await idle()
      const parked = at()
      pages.scrollTop = 0; await wait(500); await idle()
      const k = pages.children[parked.page - 1]
      pages.scrollTop = k.offsetTop + parked.fraction * k.offsetHeight
      await wait(500); await idle()
      const back = at()
      ok(6, 'the reader does not move when canvases are released',
        back.page === parked.page && Math.abs(back.fraction - parked.fraction) < 0.01,
        `page ${parked.page}→${back.page}, fraction ${parked.fraction.toFixed(4)}→${back.fraction.toFixed(4)}`)

      // 7 — a throw does not thrash.
      pages.scrollTop = 0; await wait(500); await idle()
      const before = P().renders
      const step = pages.scrollHeight / 40
      for (let i = 1; i <= 40; i++) { pages.scrollTop = step * i; await frame() }
      await idle()
      ok(7, 'a throw does not thrash', P().renders - before <= 8,
        `${P().renders - before} renders, at most 8 wanted, 43 without the per-page rest`)

      // 8 — a cross-reference into an unrendered page lands at its coordinate.
      pages.scrollTop = 0; await wait(500); await idle()
      const link = pages.children[0].querySelector('.annotationLayer a')
      const drawnBefore = pages.children[63].querySelector('canvas') !== null
      if (link) {
        link.click()
        await wait(500); await idle()
        const landed = at()
        ok(8, 'a cross-reference into an unrendered page lands at its coordinate',
          landed.page === 64 && Math.abs(landed.fraction - 0.620) < 0.01 && !drawnBefore,
          `page ${landed.page} fraction ${landed.fraction.toFixed(3)}, wanted 64 / 0.620; ` +
          `page 64 held a canvas beforehand: ${drawnBefore}`)
      } else {
        ok(8, 'a cross-reference into an unrendered page lands at its coordinate', false,
          'no <a> in page 1 — the link was filtered or the layer is missing')
      }

      // 10 — a width gesture at three widths, and the reader across it.
      let gesture = true
      const seen = []
      pages.scrollTop = pages.scrollHeight / 2; await wait(500); await idle()
      const held0 = at()
      for (const px of [400, 520, 700]) {
        await width(px)
        const here = at()
        const good = sharp() && P().mode === 'band' &&
          here.page === held0.page && Math.abs(here.fraction - held0.fraction) < 0.01
        if (!good) gesture = false
        seen.push(`${pages.clientWidth}px ${P().mode} ${sharp() ? 'sharp' : 'SOFT'} p${here.page}/${here.fraction.toFixed(3)}`)
      }
      ok(10, 'a width gesture leaves every page sharp and the reader in place', gesture, seen.join(' · '))

      /*
        10.1 — the rest is invisible, sampled every frame *through* it rather
        than after it. Every other clause here reads a settled pane, which is
        exactly where a pane that empties itself for the length of a scroll rest
        hides: a forced pass that runs before its observer has delivered reads an
        empty set, draws nothing, and sweeps the whole document to bare paper.
      */
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600); await idle()
      const floor = pages.querySelectorAll('canvas').length
      const grab = parseFloat(getComputedStyle(text).flexBasis) || text.clientWidth
      text.style.flexBasis = `${grab + 60}px`
      let fewest = floor
      for (let i = 0; i < 90; i++) {
        await frame()
        fewest = Math.min(fewest, pages.querySelectorAll('canvas').length)
      }
      await wait(900); await idle()
      ok(10.1, 'the rest never empties the pane', fewest > 0,
        `fewest canvases on any frame through the rest: ${fewest}, holding ${floor} before it`)

      // 14 — two paths do not overlap.
      const paths = P().renders
      const b = parseFloat(getComputedStyle(text).flexBasis) || text.clientWidth
      for (let i = 0; i < 20; i++) {
        text.style.flexBasis = `${b + i * 8}px`
        pages.scrollTop += 120
        await frame()
      }
      await width(520)
      ok(14, 'a drag and a scroll do not overlap',
        P().renders - paths <= 24 && layersMatch() && sharp(),
        `${P().renders - paths} renders, layers ${layersMatch()}, sharp ${sharp()}`)

      // 12/13 — a compile under the reader.
      pages.scrollTop = pages.scrollHeight / 2; await wait(600); await idle()
      const kept = [...pages.children]
      const stood = at()
      const beforeCompiles = P().renders
      for (let i = 0; i < 10; i++) await compile()
      const same = kept.length === pages.children.length && kept.every((w, i) => w === pages.children[i])
      const cost = P().renders - beforeCompiles
      ok(12, 'ten compiles cost the band and not the document',
        cost > 0 && cost < 71 && same && pages.children.length === 71 && layersMatch(),
        cost === 0 ? 'no compile happened — the ten edits produced no new bytes, so this clause tested nothing'
          : `${cost} renders over ten compiles, same elements: ${same}`)
      // 13 — the reader must land on a drawn page, caught on the frame they are
      // moved. Read a second later it passes either way: a pane that jumped them
      // onto a placeholder has had its scroll rest fire and fill it in by then.
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600); await idle()
      const from = at().page
      const start = text.selectionStart
      text.value = `${text.value} `
      text.setSelectionRange(start, start)
      text.dispatchEvent(new Event('input', { bubbles: true }))
      /*
        Sampled at the moment the open resolves, which is the first time the pass
        goes quiet after the edit. Not the frame the reader is moved: the design
        positions them and *then* draws, so a placeholder under them for one
        render is the recorded cost rather than the defect. And not a second
        later, which passes either way — by then the 120 ms scroll rest has
        filled in the page a wrong implementation jumped them onto.
      */
      const opened = P().renders
      let drawnSoFar = opened
      let quiet = 0
      let landing = null
      for (let i = 0; i < 900 && landing === null; i++) {
        await frame()
        if (P().renders !== drawnSoFar) { drawnSoFar = P().renders; quiet = 0; continue }
        if (++quiet >= 5 && drawnSoFar > opened) {
          const page = at().page
          landing = { page, drawn: pages.children[page - 1].querySelector('canvas') !== null }
        }
      }
      await wait(600); await idle()
      ok(13, 'the compile lands the reader on a page that holds a raster',
        landing !== null && landing.drawn && landing.page !== from,
        landing === null ? 'the compile started no render — this clause tested nothing'
          : `moved from ${from} to ${landing.page}, drawn when the open resolved: ${landing.drawn} ` +
            `(${drawnSoFar - opened} renders)`)

      ok(3.1, 'nothing retained outside #pages, after the scrolling and the compiles',
        P().made - P().released === pages.querySelectorAll('canvas').length,
        `made−released ${P().made - P().released}, canvases ${pages.querySelectorAll('canvas').length}`)

      ok(15, 'no error reached the console', noise === 0, `${noise} uncaught`)
      note(`15. engine readings — sizingMs ${P().sizingMs} (Chromium 3.4, failing above 250) · ` +
        `deliveryMs ${P().deliveryMs} (Chromium 0.4–6.8) · renderMs ${P().renderMs} (about 8.5 predicted at 520 px)`)
      note('a disagreement on those three is a finding; on every other clause it is a failure')
      return tally('long.md')
    },

    // What to widen the window past before running `wide()`.
    boundary,

    /*
      Phase 3, at the 520 pane — where fit-width and fit-page agree, so the
      fits that are distinct here are width and manual.
    */
    async fits() {
      heading('long.md — the three fits, at the 520 pane')
      await setFit('width')
      await width(520)
      pages.scrollTop = 0
      await wait(500)
      await idle()
      if (!pre3(false)) note('clauses below still run; read their numbers as reports, not results')

      const pinned = Math.round(pages.children[0].natural.w * 4) // 2381 for A4

      // P3·8 — the cap is the control's own edge, and one page at it fits the
      // budget alone. Above about 410% it would not, which is what 400% is.
      const offered = [...fitEl.options].map((o) => o.value)
      await setFit('4')
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600)
      await idle()
      const page400 = one()
      ok('P3·8', 'the cap is the control’s own edge', 
        offered.includes('4') && !offered.some((v) => Number(v) > 4) &&
        page400 <= 128 * 2 ** 20 && held() <= 2 * page400,
        `offers ${offered.join('/')} · one page ${MiB(page400)} MiB against a 128 budget ` +
        `(122.4 at dpr 2) · retained ${MiB(held())} over ${count()} canvases (at most 244.7)`)

      // P3·1 — each fit holds across a divider drag and a window resize. The
      // manual half is the one a width-keyed implementation fails: the box may
      // not move with the pane, during the drag or after it.
      let moved = null
      await drag(400, () => { if (cssW() !== pinned) moved = cssW() })
      const during = moved
      const after400 = cssW()
      await drag(520, () => { if (cssW() !== pinned) moved = cssW() })
      ok('P3·1a', 'a pinned scale holds across a drag, during it and after',
        during === null && moved === null && after400 === pinned && cssW() === pinned,
        `wanted ${pinned} px throughout; during the drag ${during ?? 'held'}, at rest ${after400}/${cssW()}`)

      await setFit('width')
      await width(520)
      const w520 = [cssW(), cssH()]
      await width(400)
      const w400 = [cssW(), cssH()]
      await width(520)
      ok('P3·1b', 'fit-width re-derives with the pane', 
        w520[0] === 520 && w400[0] === 400 && cssW() === 520,
        `520→${w520.join('×')} · 400→${w400.join('×')} · back to ${cssW()}×${cssH()}`)

      // P3·4 — a page wider than the pane is reachable, and one that fits
      // overflows nothing.
      await setFit('4')
      const wide = [pages.scrollWidth, pages.clientWidth]
      pages.scrollLeft = pages.scrollWidth
      await frame()
      const edge = Math.round(pages.scrollLeft + pages.clientWidth)
      await setFit('width')
      ok('P3·4', 'a page wider than the pane is reachable',
        wide[0] === pinned && wide[1] < pinned && edge === wide[0] &&
        pages.scrollWidth === pages.clientWidth,
        `at 400% scrollWidth ${wide[0]} against clientWidth ${wide[1]}, right edge reached at ${edge}; ` +
        `at fit-width ${pages.scrollWidth} === ${pages.clientWidth}`)

      // P3·5 and P3·7 — a fit change carries the reader, and the pane is never
      // empty on any frame of the transition. Width ↔ manual only: at this
      // pane a width ↔ page change is the identity and is not evidence.
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600)
      await idle()
      const trips = [await carried('2', 'width→200%'), await carried('width', '200%→width')]
      ok('P3·5', 'a fit change carries the reader, both ways',
        trips.every((t) => t.ok), trips.map((t) => t.say).join(' · '))
      ok('P3·7', 'the pane is never empty on any frame of a transition',
        trips.every((t) => t.fewest > 0),
        trips.map((t) => `${t.name} fewest ${t.fewest}`).join(' · '))

      // P3·6 — Phase 5's clauses 2, 3, 9 and 13, re-run at a pinned 200%.
      await setFit('2')
      const per = one()
      pages.scrollTop = 0
      await wait(600)
      await idle()
      const top = { n: count(), b: held() }
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600)
      await idle()
      const mid = { n: count(), b: held() }
      ok('P3·6a', 'retention at 200% is the band, and it is canvases × one page',
        P().mode === 'band' && top.b === top.n * per && mid.b === mid.n * per &&
        top.n >= 1 && top.n <= 3 && mid.n >= 2 && mid.n <= 4,
        `one page ${MiB(per)} MiB (30.6 at dpr 2) · top ${top.n}/${MiB(top.b)} · ` +
        `mid ${mid.n}/${MiB(mid.b)} · a stale canvas kept across the fit change breaks this equality`)
      ok('P3·6b', 'nothing is retained outside #pages at 200%',
        P().made - P().released === count(), `made−released ${P().made - P().released}, canvases ${count()}`)
      let layers3 = true
      for (let i = 0; i < 6; i++) {
        pages.scrollTop = pages.scrollHeight * (i % 2 ? 0.1 : 0.8)
        await wait(400)
        await idle()
        if (!layersMatch()) layers3 = false
      }
      ok('P3·6c', 'layers track canvases at 200%, six release-and-re-entry cycles', layers3)

      // P3·3 and P3·6d — a compile under the reader at a pinned scale. The
      // first compile is what puts the reader on the caret's own page; the
      // second is the measured one, because only then is the edit under them.
      await setFit('4')
      caretAt(0.5)
      await compile()
      await wait(400)
      await idle()
      const stood = at()
      const before = cssW()
      await compile()
      const landed = at()
      const drawn = pages.children[landed.page - 1].querySelector('canvas') !== null
      ok('P3·3', 'a pinned scale survives a compile, and the reader holds',
        before === pinned && cssW() === pinned && landed.page === stood.page && drawn,
        `page ${before}→${cssW()} px (wanted ${pinned}) · reader p${stood.page}/` +
        `${stood.fraction.toFixed(3)}→p${landed.page}/${landed.fraction.toFixed(3)} · drawn ${drawn}`)
      ok('P3·6d', 'a compile at a pinned scale lands the reader on a drawn page',
        drawn && landed.page > 1, `p${landed.page}, holding a raster: ${drawn}`)

      await setFit('width')
      await width(520)
      ok('P3·2', 'fit-page is not distinct at this pane, and is not tested here', true,
        `pane ${pages.clientWidth} against boundary ${boundary()} — run __gate.wide() with the window widened`)
      return tally('long.md — the fits')
    },

    /*
      Phase 3, with the pane above the fit-page boundary — the only geometry
      where fit-page is a different answer from fit-width, and so the only one
      where its clauses mean anything.
    */
    async wide() {
      heading('long.md — fit-page, above the boundary')
      await setFit('width')
      pages.scrollTop = 0
      await wait(500)
      await idle()
      if (!pre3(true)) {
        note('these clauses need a pane above the boundary; nothing below is a result until it is')
      }

      const b = boundary()
      const byWidth = [cssW(), cssH()]
      await setFit('page')
      ok('P3·2', 'fit-page is distinct above the boundary, and is boundary × clientHeight',
        cssW() === b && cssH() === Math.round(pages.clientHeight) && cssW() < byWidth[0],
        `page ${cssW()}×${cssH()}, wanted ${b}×${pages.clientHeight}; fit-width is ${byWidth.join('×')}`)
      ok('P3·10b', 'a page narrower than the pane is centred',
        pages.children[0].offsetLeft ===
          Math.round((pages.clientWidth - cssW()) / 2) && pages.children[0].offsetLeft > 0,
        `offsetLeft ${pages.children[0].offsetLeft}, wanted ${Math.round((pages.clientWidth - cssW()) / 2)}`)

      /*
        P3·1c — fit-page re-derives on a height-only change, which is the clause
        a width-keyed comparison cannot pass: a height-bound scale has no width
        to key on. The height is moved from the page rather than by resizing the
        window, because `#error` is `flex: none` above `#pages` in the same
        column — it takes height out of the pane and no width, and it is put
        back. A window resized by its height alone is the same event.
      */
      const err = document.getElementById('error')
      const wasHidden = err.hidden
      const wasText = err.textContent
      const tall = [cssW(), cssH(), pages.clientHeight]
      err.textContent = 'gate: a height-only change of the pane'
      err.hidden = false
      await wait(1200)
      await idle()
      const short = [cssW(), cssH(), pages.clientHeight]
      err.hidden = wasHidden
      err.textContent = wasText
      await wait(1200)
      await idle()
      ok('P3·1c', 'fit-page re-derives on a height-only change, and comes back',
        short[2] < tall[2] && short[0] < tall[0] && short[1] === Math.round(short[2]) &&
        cssW() === tall[0] && cssH() === tall[1],
        `${tall[0]}×${tall[1]} at a ${tall[2]} pane → ${short[0]}×${short[1]} at ${short[2]} ` +
        `→ ${cssW()}×${cssH()} at ${pages.clientHeight}`)

      // P3·5 and P3·7 — the four directions that are real only here.
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600)
      await idle()
      await setFit('width')
      const trips = [
        await carried('page', 'width→page'),
        await carried('2', 'page→200%'),
        await carried('page', '200%→page'),
        await carried('width', 'page→width')
      ]
      ok('P3·5w', 'a fit change carries the reader, in the four directions this pane makes real',
        trips.every((t) => t.ok), trips.map((t) => t.say).join(' · '))
      ok('P3·7w', 'the pane is never empty on any frame of those transitions',
        trips.every((t) => t.fewest > 0), trips.map((t) => `${t.name} ${t.fewest}`).join(' · '))
      return tally('long.md — fit-page')
    },

    async near() {
      heading('near.md — 20 pages, at the budget')
      await idle()
      await width(520)
      pages.scrollTop = 0; await wait(500); await idle()
      const a = { mode: P().mode, n: pages.querySelectorAll('canvas').length, mib: MiB(held()) }
      await width(700)
      pages.scrollTop = 0; await wait(500); await idle()
      const b = { mode: P().mode, n: pages.querySelectorAll('canvas').length, mib: MiB(held()) }
      await width(520)
      pages.scrollTop = 0; await wait(500); await idle()
      const c = { mode: P().mode, n: pages.querySelectorAll('canvas').length, mib: MiB(held()) }
      ok(11, 'the budget is crossed in both directions',
        a.mode === 'whole' && b.mode === 'band' && c.mode === 'whole' && sharp(),
        `520 ${a.mode} ${a.n}/${a.mib} · 700 ${b.mode} ${b.n}/${b.mib} · 520 ${c.mode} ${c.n}/${c.mib}; ` +
        `spec: whole 20/116.64 · band 3/31.72 · whole 20/116.64`)
      ok(3, 'nothing is retained outside #pages',
        P().made - P().released === pages.querySelectorAll('canvas').length)

      /*
        P3·9 — the budget crossed by the fit alone, both ways, with the width
        never touched. This is the clause an implementation that re-derives the
        scale without re-deciding what is held fails and passes every other:
        it reads 20 canvases at 200%, four times the cost of the whole document
        at fit-width, on a budget of 128 MiB.
      */
      await setFit('width')
      await width(520)
      pages.scrollTop = 0
      await wait(500)
      await idle()
      const wideFit = { mode: P().mode, n: count(), b: held(), px: pages.clientWidth }
      await setFit('2')
      pages.scrollTop = pages.scrollHeight / 2
      await wait(600)
      await idle()
      const zoomed = { mode: P().mode, n: count(), b: held(), px: pages.clientWidth, per: one() }
      await setFit('width')
      pages.scrollTop = 0
      await wait(500)
      await idle()
      const back = { mode: P().mode, n: count(), b: held(), px: pages.clientWidth }
      ok('P3·9', 'the budget is crossed by the fit alone, both ways, the width untouched',
        wideFit.mode === 'whole' && zoomed.mode === 'band' && back.mode === 'whole' &&
        wideFit.n === back.n && wideFit.b === back.b && zoomed.n < wideFit.n &&
        zoomed.b === zoomed.n * zoomed.per &&
        wideFit.px === zoomed.px && zoomed.px === back.px && sharp(),
        `width ${wideFit.mode} ${wideFit.n}/${MiB(wideFit.b)} · 200% ${zoomed.mode} ` +
        `${zoomed.n}/${MiB(zoomed.b)} · width ${back.mode} ${back.n}/${MiB(back.b)}; ` +
        `pane ${wideFit.px}/${zoomed.px}/${back.px} px throughout; ` +
        `spec at dpr 2: whole 20/116.64 · band 2–3/61.2–91.8 · whole 20/116.64`)
      ok('P3·9b', 'nothing is retained outside #pages after the crossings',
        P().made - P().released === count(),
        `made−released ${P().made - P().released}, canvases ${count()}`)
      return tally('near.md')
    },

    async showcase() {
      heading('showcase.md — 6 pages, unchanged')
      await idle()
      const k = pages.children
      const gaps = [k[0].offsetTop]
      for (let i = 0; i + 1 < k.length; i++) {
        gaps.push(k[i + 1].offsetTop - (k[i].offsetTop + k[i].offsetHeight))
      }
      const trailing = pages.scrollHeight - (k[k.length - 1].offsetTop + k[k.length - 1].offsetHeight)
      // Phase 4's clause 1, re-run against the wrapper. All three read 16; a
      // trailing 32 is the signature of `margin: 16px 0` written for
      // `margin-top: 16px`, which the first two expressions wave through.
      ok(1, 'the showcase is whole, and Phase 4 clause 1 still reads 16 throughout',
        P().mode === 'whole' && k.length === 6 &&
        gaps.every((g) => g === 16) && trailing === 16 && layersMatch() && sharp(),
        `mode ${P().mode}, ${k.length} pages, gaps ${[...new Set(gaps)].join('/')}, trailing ${trailing}` +
        `${trailing === 32 ? ' (32 means margin: 16px 0)' : ''}, pane ${pages.clientWidth}px`)
      /*
        P3·10 — fit-width is visually unchanged. Phase 4's clause 1 is the one
        above; what this adds is that the ring's side pixels are painted ink
        rather than scrollable overflow, so at fit-width they are clipped by
        the scrollport and grow no track: `scrollWidth` is `clientWidth` to the
        pixel and the page sits flush at x = 0.
      */
      ok('P3·10', 'fit-width is visually unchanged, and the ring shows no side pixel',
        pages.scrollWidth === pages.clientWidth && k[0].offsetLeft === 0 &&
        cssW() === pages.clientWidth &&
        getComputedStyle(k[0]).boxShadow.includes('1px'),
        `scrollWidth ${pages.scrollWidth} === clientWidth ${pages.clientWidth}, ` +
        `page ${cssW()} px at offsetLeft ${k[0].offsetLeft}`)
      note('P3·10 by eye, at a pane above __gate.boundary() under Fit page: the page sits centred')
      note('on --ground with the hairline drawing all four of its edges, not two.')
      note('Phase 2 clauses 1–8 and Phase 1 clause 7 are by eye: select and copy page 1, follow a')
      note('cross-reference, confirm no external link is followable, enter empty/failed/stale, open a second document')
      return tally('showcase.md')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Phase 5: arm → long() → near() → showcase().  ' +
      'Phase 3: fits() after long(), wide() with the window widened past ' +
      'boundary(), then near() and showcase() carry theirs.',
    'font-weight:bold;color:#1a73e8', 'color:inherit')
})()
