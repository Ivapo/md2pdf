/* mpdf-010 Phase 5 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The Rust half is `cargo test --workspace`, which holds the three clauses
   about the read: the bytes come back as the disk holds them, and a path that
   leaves the project is refused by name. What only a window can say is that
   **the figure is a view** — it is drawn over the text pane's own column, it
   follows that column wherever it moves, and the pane goes on holding its file
   while it is up.

   **It has no preconditions**, as Phase 1's and Phase 2's had none: every
   clause is about the DOM and about `invoke('status')`'s own answer, so no
   failure it reports can be about the size of your window or its pixel ratio.

   **It writes nothing**, where Phase 2's gate wrote to a tracked file and put
   it back. It moves the divider and puts it back, and it moves `edited` inside
   the fixture project, which the next open resets. `git status` is clean before
   and after.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open tests/fixtures/panel/sections/text.md
     await __gate.figure()     <- clauses 1, 2, 3
     await __gate.geometry()   <- clause 4
     await __gate.ways()       <- clause 5
     open samples/showcase/showcase.md
     await __gate.fit()        <- clauses 6, 7
     __gate.report()

   The first open is the fixture and not a sample deliberately: it roots at
   `tests/fixtures/panel/` by Phase 1's climb, and it is the only project in
   this repository holding a `.pdf` a fresh clone really has — clause 3 has
   nothing to click otherwise.

   Run this against the build before this phase and it does not reach a clause
   at all: `__gate.arm()` throws `TypeError: Cannot read properties of null
   (reading 'hidden')`, because `#viewer` is not in that page. Measured against
   `git show <this commit>^:app/dist/index.html`, not reasoned about. The first
   clause that could fail is 1 — an image row's name was a `<span>`, so there is
   nothing to click.                                                          */
;(() => {
  const list = document.getElementById('parts')
  const text = document.getElementById('text')
  const pages = document.getElementById('pages')
  const viewer = document.getElementById('viewer')
  const divider = document.getElementById('divider')
  const toggle = document.getElementById('toggle')
  const numbers = document.getElementById('numbers')
  const problem = document.getElementById('error')
  const { invoke } = window['__TAURI__'].core

  let noise = 0
  const spoken = []
  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* A blob is minted and an `<img>` decodes it, both inside one task; 400 ms is
     some hundreds of times what either costs and this script is read by a
     person rather than by a timer. The two-second wait after an open is the
     compile's, matching Phase 2's `settled`. */
  const shown = () => wait(400)
  const settled = () => wait(1000)

  let pass = 0
  let fail = 0
  /* Every line is kept as well as logged: Safari's Web Inspector copies one
     console entry at a time, and `__gate.report()` gives the whole run back as
     one entry. */
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

  const row = (name) =>
    [...list.children].find((li) => li.querySelector('.name').textContent === name)

  /* Click a row's own body, which is what the reader clicks. `null` when the
     row is not there or is not a button, so a clause can say which. */
  const clickRow = (name) => {
    const button = row(name)?.querySelector('button.name')
    button?.click()
    return button ?? null
  }

  /* **The surface is compared by offsets and not by rectangles.**
     `placeViewer` writes `text.offsetLeft` and `text.offsetWidth`, which are
     integers, so a `getBoundingClientRect` comparison is off by whatever the
     fraction was and would fail on a subpixel window for a reason that is not
     this phase's. */
  const overText = () =>
    viewer.offsetLeft === text.offsetLeft && viewer.offsetWidth === text.offsetWidth
  const columns = () =>
    `viewer ${viewer.offsetLeft}+${viewer.offsetWidth}, text ${text.offsetLeft}+${text.offsetWidth}`

  /* Drag the divider by `dx` and let go.

     **One method of the app's is stubbed for the length of the drag, and only
     one**: a synthesised `PointerEvent` carries no live pointer, so the real
     `setPointerCapture` throws `NotFoundError` and the handler never gets past
     its first line. Everything after that line is the app's own code running on
     the app's own listeners. */
  const drag = (dx) => {
    const capture = divider.setPointerCapture
    divider.setPointerCapture = () => {}
    try {
      const from = divider.getBoundingClientRect().left
      const at = (type, x) =>
        divider.dispatchEvent(
          new PointerEvent(type, { pointerId: 1, clientX: x, clientY: 300, bubbles: true })
        )
      at('pointerdown', from)
      at('pointermove', from + dx)
      at('pointerup', from + dx)
    } finally {
      divider.setPointerCapture = capture
    }
  }

  /* The sheet's content box, which is what the image's `max-height: 100%`
     resolves against, and what it must fit inside. */
  const sheetBox = () => {
    const sheet = viewer.querySelector('.sheet')
    const style = getComputedStyle(sheet)
    return {
      width: sheet.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight),
      height: sheet.clientHeight - parseFloat(style.paddingTop) - parseFloat(style.paddingBottom),
      overflow: sheet.scrollHeight - sheet.clientHeight
    }
  }

  let held = ''
  let before = null
  let basis = ''

  window.__gate = {
    report() {
      console.log(`mpdf-010 Phase 5 gate\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      spoken.length = 0
      held = ''
      before = null
      basis = text.style.flexBasis
      // **Named and not merely counted.** A run reporting "3 uncaught" and
      // nothing else sends the next round looking for something it cannot see.
      const say = (what) => {
        noise++
        if (spoken.length < 8) spoken.push(what)
      }
      addEventListener('error', (e) =>
        say(`error: ${e.message || e.error} @ ${e.filename || '?'}:${e.lineno || '?'}`)
      )
      addEventListener('unhandledrejection', (e) =>
        say(`rejection: ${(e.reason && (e.reason.message || e.reason.name)) || String(e.reason)}`)
      )

      ok(0, 'the empty state holds no figure', viewer.hidden, `viewer.hidden ${viewer.hidden}`)

      console.log(
        '%carmed%c  — now open tests/fixtures/panel/sections/text.md, then run: await __gate.figure()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
    },

    async figure() {
      heading('tests/fixtures/panel/ — a figure over the pane, and a PDF that says so')
      await settled()
      before = await invoke('status')
      held = text.value
      basis = text.style.flexBasis
      note(`root's main is ${before.main}, the pane holds ${before.edited}`)

      /* Clause 1. The picture is up, and it is over the text pane's own
         column — which is the whole reason it is positioned rather than
         placed in the flow. */
      if (clickRow('mark.svg') === null) {
        ok(1, 'an image row shows the figure', false, 'no row for mark.svg carries a name button')
        return tally('figure')
      }
      await shown()

      const picture = viewer.querySelector('img')
      ok(
        1,
        'the image row drew the figure over the text pane, in that pane’s own column',
        !viewer.hidden && picture !== null && overText(),
        `${picture ? `img alt "${picture.alt}"` : 'no img'}, ${columns()}`
      )

      /* Clause 2. **It is a view.** Nothing that decides what compiles, what
         the page shows or what `⌘S` writes has moved. */
      const during = await invoke('status')
      ok(
        2,
        'the pane kept its file: main, edited and the revision are all where they were',
        during.main === before.main &&
          during.edited === before.edited &&
          during.revision === before.revision &&
          text.value === held,
        `main ${during.main}, edited ${during.edited}, revision ${during.revision} ` +
          `(was ${before.revision}), ${text.value.length} chars in the pane`
      )

      /* Clause 3. A PDF is a legal figure in this dialect and `<img>` will not
         draw one, so the row says so — **and the page does not go through the
         command or the error bar to say it**, either of which would mark the
         compiled page out of date for a click that compiled nothing. */
      /* Both are read before the click and compared after it, rather than
         asserted absolutely: a document that will not compile is a state this
         gate has no business having an opinion about, and the claim is that
         **this click changed neither**. */
      const wasStale = pages.classList.contains('stale')
      const wasFailed = problem.hidden
      if (clickRow('plan.pdf') === null) {
        ok(3, 'a PDF row says so and draws nothing', false, 'no row for plan.pdf carries a name button')
        return tally('figure')
      }
      await shown()

      const said = viewer.querySelector('.said')
      ok(
        3,
        'a PDF row draws no image, says so, and does not mark the page stale',
        !viewer.hidden &&
          viewer.querySelector('img') === null &&
          said !== null &&
          said.textContent.length > 0 &&
          pages.classList.contains('stale') === wasStale &&
          problem.hidden === wasFailed,
        `said "${said ? said.textContent : ''}", stale ${pages.classList.contains('stale')} ` +
          `(was ${wasStale}), error bar hidden ${problem.hidden} (was ${wasFailed})`
      )

      note('now run: await __gate.geometry()')
      return tally('figure')
    },

    async geometry() {
      heading('the three things that move the text pane’s column')
      clickRow('mark.svg')
      await shown()

      const started = [text.offsetLeft, text.offsetWidth]

      drag(90)
      await wait(200)
      const dragged = overText()
      const wide = [text.offsetLeft, text.offsetWidth]

      toggle.click()
      await wait(200)
      const folded = overText()
      const left = text.offsetLeft
      toggle.click()
      await wait(200)

      numbers.click()
      await wait(200)
      const gutter = overText()
      const shifted = text.offsetLeft
      numbers.click()
      await wait(200)

      // Put the pane back where it was found.
      text.style.flexBasis = basis
      await wait(200)

      /* Clause 4. All three move `#text`'s own box, and an enumeration that
         dropped either of the last two would pass a drag-only check. */
      ok(
        4,
        'the surface followed the divider, the Files fold and the Lines toggle',
        dragged &&
          folded &&
          gutter &&
          wide[1] !== started[1] &&
          left !== wide[0] &&
          shifted !== wide[0],
        `drag ${started[1]}→${wide[1]} px wide (${dragged}), fold left ${wide[0]}→${left} ` +
          `(${folded}), lines left ${wide[0]}→${shifted} (${gutter})`
      )

      note('now run: await __gate.ways()')
      return tally('geometry')
    },

    async ways() {
      heading('the three ways back to the text')
      clickRow('mark.svg')
      await shown()
      const up = !viewer.hidden

      viewer.querySelector('#close-viewer').click()
      await wait(120)
      const byButton = viewer.hidden

      clickRow('mark.svg')
      await shown()
      dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
      await wait(120)
      const byKey = viewer.hidden

      /* **The re-show is what makes the third claim reachable after the
         second**, and it is also the claim that the surface can be opened
         again rather than once per document. */
      clickRow('mark.svg')
      await shown()
      const again = !viewer.hidden

      const moved = clickRow('text.md')
      await settled()
      const state = await invoke('status')

      /* Clause 5. A markdown row already means *put that file in the pane*,
         so it must not leave a picture over it — and it must still move the
         pane, which is Phase 2's behaviour unchanged. */
      ok(
        5,
        'the control, Escape and a markdown row each put the text back, and the row still moved the pane',
        up &&
          byButton &&
          byKey &&
          again &&
          moved !== null &&
          viewer.hidden &&
          state.edited === 'sections/text.md' &&
          state.main === before.main,
        `control ${byButton}, escape ${byKey}, re-shown ${again}, ` +
          `edited ${state.edited}, main ${state.main}`
      )

      note('now open samples/showcase/showcase.md, then run: await __gate.fit()')
      return tally('ways')
    },

    async fit() {
      heading('samples/showcase/ — a figure with a shape, in a column narrow enough to bind')
      await settled()
      basis = text.style.flexBasis

      if (clickRow('emit.svg') === null) {
        ok(6, 'a figure is drawn inside the sheet', false, 'no row for emit.svg carries a name button')
        return tally('fit')
      }
      await shown()

      /* **The pane is narrowed first, and that is the clause rather than
         setup.** `emit.svg` is 120 × 72, so in a pane at its default width it
         is drawn at its natural size and `max-width` has nothing to do — a
         containment check there passes under a stylesheet with no fit rule at
         all. Dragging to the floor puts the sheet's content box under 120 px,
         which is where the rule is the only thing keeping the figure in. */
      drag(-2000)
      await wait(300)

      const picture = viewer.querySelector('img')
      const box = sheetBox()
      const drawn = picture ? picture.getBoundingClientRect() : null

      ok(
        6,
        'the figure is drawn no wider and no taller than the sheet, and the sheet does not overflow',
        picture !== null &&
          drawn.width <= box.width + 1 &&
          drawn.height <= box.height + 1 &&
          drawn.width < picture.naturalWidth &&
          box.overflow === 0,
        picture
          ? `natural ${picture.naturalWidth}×${picture.naturalHeight}, ` +
            `drawn ${Math.round(drawn.width)}×${Math.round(drawn.height)}, ` +
            `sheet ${Math.round(box.width)}×${Math.round(box.height)}, overflow ${box.overflow}`
          : 'no img'
      )

      // Put the pane back where it was found.
      text.style.flexBasis = basis
      dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape', bubbles: true }))
      await wait(200)

      ok(7, 'no error reached the console', noise === 0,
        `${noise} uncaught${spoken.length ? ' — ' + spoken.join(' | ') : ''}`)

      note('run __gate.report() to copy the whole transcript back, then check `git status` is clean.')
      return tally('fit')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open tests/fixtures/panel/sections/text.md → figure() → geometry() → ways() →\n' +
      'open samples/showcase/showcase.md → fit() → report().',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
