/* mpdf-009 Phase 5 — exit gate. Paste into the Web Inspector console of a
   `cargo tauri dev` window, with the machine set to always-show scrollbars.

   ORDER:
     __gate.arm()          <- BEFORE opening anything, from the empty state
     open tests/fixtures/long.md
     await __gate.long()   <- clauses 2, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15
     open tests/fixtures/near.md
     await __gate.near()   <- clause 11
     open samples/showcase/showcase.md
     await __gate.showcase()  <- clause 1's geometry (its 1-8 are Phase 2's, by eye)
   Clauses 12 and 13 are driven by __gate.long() and named at the end.        */
;(() => {
  const pages = document.getElementById('pages')
  const text = document.getElementById('text')
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

  let pass = 0
  let fail = 0
  const ok = (n, name, good, detail) => {
    good ? pass++ : fail++
    console.log(
      `%c${good ? 'PASS' : 'FAIL'}%c  ${String(n).padStart(2)}. ${name}${detail ? '  —  ' + detail : ''}`,
      `font-weight:bold;color:${good ? '#137333' : '#c5221f'}`,
      'color:inherit'
    )
  }
  const note = (s) => console.log(`%c····%c  ${s}`, 'color:#888', 'color:#888')
  const tally = (what) => {
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
      console.log('%c\nlong.md — 71 pages\n', 'font-weight:bold')
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

    async near() {
      console.log('%c\nnear.md — 20 pages, at the budget\n', 'font-weight:bold')
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
      return tally('near.md')
    },

    async showcase() {
      console.log('%c\nshowcase.md — 6 pages, unchanged\n', 'font-weight:bold')
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
      note('Phase 2 clauses 1–8 and Phase 1 clause 7 are by eye: select and copy page 1, follow a')
      note('cross-reference, confirm no external link is followable, enter empty/failed/stale, open a second document')
      return tally('showcase.md')
    }
  }

  console.log('%c__gate ready%c  —  run __gate.arm() now, before opening anything.',
    'font-weight:bold;color:#1a73e8', 'color:inherit')
})()
