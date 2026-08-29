/* mpdf-003 Phase 11 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The Rust half is `cargo test --workspace`, which holds exactly one clause
   about this phase — `preview.rs`'s new test, that the brand cell says what
   `tauri.conf.json` calls the product — plus the 333 that must not move,
   because **no Rust reaches the compile path at all** and the PDF is
   byte-identical across the phase. `bun app/typecheck.mjs` is the third half
   and is not part of either, for the reason its own header states.

   What only a window can say is **where the bar sits, what it names, and what
   it costs**: that the footer is `main`'s next sibling and the last element
   before the module script, that the left cell is the bare name of `edited` and
   not of `main`, that the three boxes still sum to the viewport, and that
   nothing in the page's `ResizeObserver` starts looping now that a box in flow
   has joined the column.

   NO SETUP STEP. It opens `tests/fixtures/panel/book.md` in the repository
   itself, the way `mpdf-010` Phase 1's, Phase 2's and Phase 5's gates open
   theirs — a path a person can reach in the Open dialog without typing one.

   **The `$TMPDIR` copy Phases 3 and 4 use would be ceremony here, and the
   reason is checkable rather than asserted.** Those two created and deleted
   files. **This gate makes exactly one gesture** — a click on a panel row — and
   `app/src/preview.rs:set_edited` answers it without touching disk: it moves
   `edited`, reloads the buffer and re-arms the watch, and that is all. There is
   no save, no export, no create and no delete anywhere in the run, and the only
   thing this app writes outside a document is the remembered main, which lives
   in Application Support. `git status` clean before and after is the check, not
   the copy.

   **`book.md` and not `sections/text.md`.** It is the master, so `edited` and
   `main` are equal at the open and the row click is what makes them differ —
   which is the transition the last behavioural clause is about, and it cannot
   be read from a session that started with them already apart.

   TWO PRECONDITIONS, and both are on clause 6 rather than on the run:

     * the widths must be taken **with the fixture open and a compile behind
       you**. `#toggle` carries `hidden` until a document is open and `#status`
       is empty before one compiles, so a header armed before the open is
       narrower than 627px demands and will not grow — a false negative on the
       positive control, read as a pass.
     * the widths must be read off **`innerWidth`, not off what was asked for**.
       A window resize can be clamped, or otherwise answered with a viewport
       wider than requested; an implementer who resizes and does not read the
       answer back has measured nothing. That cost this phase's own prototype a
       wrong literal — in Chromium, and it is not a claim about this WKWebView.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open tests/fixtures/panel/book.md
     await __gate.bar()        <- clauses 1, 2, 3, 4
     await __gate.pick()       <- clause 5
     await __gate.widths()     <- clauses 6, 7, 8, 9
     __gate.report()

   **`widths()` may well report that it cannot drive the window, and that is
   expected rather than a fault.** `core:default` grants the window *getters*
   and not `allow-set-size`, so `setSize` rejects at the IPC; the gate probes it
   and says so rather than asking for the capability, a permission widened to
   run a check being one the shipped app would carry for ever. Then resize the
   window by hand — one width above 627px and two different ones below — calling
   `await __gate.sample()` after each drag, and `__gate.widthsDone()` at the
   end. `sample()` says what is still wanted. The judging is the same either way.

   Paste this into the build before this phase and the banner never prints:
   the lookups below run at paste time and `arm()` throws
   `TypeError: null is not an object (evaluating 'cell.textContent')` — `#edited`
   is not in that page at all, which is the first thing this phase adds.       */
;(() => {
  const footer = document.querySelector('footer')
  const main = document.querySelector('main')
  const header = document.querySelector('header')
  const module = document.querySelector('script[type="module"]')
  const cell = document.getElementById('edited')
  const brand = document.getElementById('brand')
  const list = document.getElementById('parts')
  const status = document.getElementById('status')
  const toggle = document.getElementById('toggle')
  const { invoke } = window['__TAURI__'].core

  let noise = 0
  /* **Counted apart from the rest, and that is the clause rather than a
     nicety.** `ResizeObserver loop completed with undelivered notifications` is
     the failure class `mpdf-009` Phase 3 found twenty-one of in a single run,
     off one `overflow-x: auto`; a footer is a new box in flow in the column
     that observer watches, so it is the specific thing this phase could have
     disturbed. `noise` stays the total so an unrelated throw is still seen. */
  let loops = 0
  const spoken = []
  const wait = (ms) => new Promise((r) => setTimeout(r, ms))
  const frame = () => new Promise((r) => requestAnimationFrame(r))

  /* One `app/src/watch.rs:DEBOUNCE` (100 ms) plus the platform's own latency
     plus a compile, as every gate here sizes it. */
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

  /* **`getBoundingClientRect().height` and not `offsetHeight`**, and this is a
     correctness point rather than a style one. The header is 46.5px at
     13px/1.5, so `offsetHeight` rounds it to 47 and a three-term `offsetHeight`
     sum overshoots `innerHeight` by one at some widths and not at others. Every
     other gate in this directory reaches for `offsetHeight`, so the natural
     implementation of clause 4 fails on correct code. Measured, not reasoned
     about: at a 616px viewport the rects are 46.5 + 545.5 + 24 = 616 exactly,
     and the same three `offsetHeight`s give 617. */
  const tall = (el) => el.getBoundingClientRect().height
  const boxes = () => ({
    header: tall(header),
    main: tall(main),
    footer: tall(footer),
    innerHeight
  })
  const sums = () => tall(header) + tall(main) + tall(footer) === innerHeight
  const columns = () => {
    const b = boxes()
    return `header ${b.header} + main ${b.main} + footer ${b.footer} = ` +
      `${b.header + b.main + b.footer} against innerHeight ${b.innerHeight}`
  }

  /* The panel's rows as a reader sees them, and the one gesture this gate
     makes: a click on a row's own `.name` button, which is what moves the pane
     and is the only thing here that makes `edited` differ from `main`. */
  const rowFor = (name) =>
    [...list.children].find(
      (li) => !li.classList.contains('folder') && li.querySelector('.name')?.textContent === name
    )
  const clickRow = (name) => {
    const button = rowFor(name)?.querySelector('button.name')
    button?.click()
    return button ?? null
  }

  /* What a width reading is: the viewport actually achieved, and the three
     boxes at it. Never the width that was asked for. */
  const samples = []
  const reading = () => ({
    width: innerWidth,
    header: tall(header),
    footer: tall(footer),
    brand: brand.getBoundingClientRect().width,
    said: brand.textContent.trim(),
    sums: sums()
  })
  const said = (s) =>
    `${s.width}px · header ${s.header} · footer ${s.footer} · brand ${s.said || '(empty)'} ` +
    `at ${Math.round(s.brand)}px · sum ${s.sums ? 'exact' : 'OFF'}`

  window.__gate = {
    report() {
      console.log(`mpdf-003 Phase 11 gate\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      loops = 0
      spoken.length = 0
      samples.length = 0
      // **Named and not merely counted.** A run reporting "3 uncaught" and
      // nothing else sends the next round looking for something it cannot see.
      const say = (what) => {
        noise++
        if (spoken.length < 8) spoken.push(what)
      }
      addEventListener('error', (e) => {
        const what = `${e.message || e.error}`
        if (what.includes('ResizeObserver')) loops++
        say(`error: ${what} @ ${e.filename || '?'}:${e.lineno || '?'}`)
      })
      addEventListener('unhandledrejection', (e) =>
        say(`rejection: ${(e.reason && (e.reason.message || e.reason.name)) || String(e.reason)}`)
      )

      /* **The empty state is the one this phase leaves the product name alone
         in.** Before the first open the window title is `Letur` too, so the name
         is on screen twice — which resolves itself at `⌘O`, and is exactly the
         case Phase 10 argued from. What is asserted here is the other half: the
         left cell names nothing, because there is nothing to name. */
      ok(
        0,
        'the empty state leaves the cell empty and the brand standing',
        cell.textContent === '' && brand.textContent.trim() === 'Letur' &&
          brand.getBoundingClientRect().width > 0,
        `cell ${JSON.stringify(cell.textContent)}, brand ${JSON.stringify(brand.textContent)} ` +
          `at ${Math.round(brand.getBoundingClientRect().width)}px`
      )

      console.log(
        '%carmed%c  — now open tests/fixtures/panel/book.md, then run: await __gate.bar()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
    },

    async bar() {
      heading('the fixture — the bar with a document open')
      await settled()

      const state = await invoke('status')
      note(`the root's main is ${state.main}, the pane holds ${state.edited}`)
      note(columns())

      /* **All three, and the position is satisfiable by and only by one
         placement.** `body`'s element children are `HEADER, MAIN, FOOTER,
         SCRIPT`: "after `</main>`" and "the last element of `body`" name two
         different places, the second being that script, and a clause asserting
         only one of them would pass on a page the scope forbids. */
      ok(
        1,
        'the footer is main’s next sibling and the last element before the module script',
        main.nextElementSibling === footer &&
          footer.nextElementSibling === module &&
          document.body.lastElementChild !== footer,
        `body holds ${[...document.body.children].map((e) => e.tagName).join(', ')}`
      )

      /* **The bare name of `edited`, and `edited` is not `main`.** The two are
         equal here, at the open, which is why clause 5 exists — but the string
         is read against `edited` even so, so a cell wired to `main` fails there
         and not here for a reason this clause has already named. */
      const wanted = state.edited === null ? '' : state.edited.split('/').pop()
      ok(
        2,
        'the cell is exactly the bare name of `edited`, and carries no path',
        cell.textContent === wanted && !cell.textContent.includes('/') &&
          cell.textContent === 'book.md',
        `cell ${JSON.stringify(cell.textContent)} against ${JSON.stringify(wanted)} ` +
          `from edited ${JSON.stringify(state.edited)}`
      )

      /* The Rust half holds this against `tauri.conf.json`; this holds it
         against the window that is actually running. */
      ok(
        3,
        'the brand reads Letur',
        brand.textContent.trim() === 'Letur',
        `brand ${JSON.stringify(brand.textContent)}`
      )

      ok(
        4,
        'the three boxes sum to the viewport, read off getBoundingClientRect',
        sums(),
        `${columns()} · the same three offsetHeights give ` +
          `${header.offsetHeight + main.offsetHeight + footer.offsetHeight}`
      )

      console.log(
        '%c—%c  now run: await __gate.pick()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return tally('bar')
    },

    async pick() {
      heading('a row click — the cell follows the pane, not the compile')

      const before = await invoke('status')
      const button = clickRow('text.md')
      await settled()
      const after = await invoke('status')

      note(`before: edited ${before.edited}, main ${before.main}`)
      note(`after:  edited ${after.edited}, main ${after.main}`)

      /* **The behaviour a reader could most reasonably expect to be the other
         way.** The bar names the file being typed in, which from this click is
         not the file the page beside it came from, and nothing in the bar marks
         that — the panel does, marking one row `main` and lighting the edited
         one. Asserted against `main` as well as against `edited`, because a
         cell wired to `main` passes clause 2 and fails only here. */
      ok(
        5,
        'after the click the cell names the clicked file, not the one that compiles',
        button !== null && after.edited === 'sections/text.md' && after.main === 'book.md' &&
          after.edited !== after.main && cell.textContent === 'text.md' &&
          !cell.textContent.includes('/'),
        `clicked ${button !== null}, cell ${JSON.stringify(cell.textContent)}, ` +
          `edited ${JSON.stringify(after.edited)}, main ${JSON.stringify(after.main)}`
      )

      console.log(
        '%c—%c  now run: await __gate.widths()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return tally('pick')
    },

    /* One reading at whatever width the window is now. Called by `widths()`
       three times, and by hand as many times as you like if the scripted
       resize is not available. */
    async sample() {
      await wait(700)
      const s = reading()
      samples.push(s)
      note(`sample ${samples.length}: ${said(s)}`)
      // What is still wanted, so a hand-driven sweep knows when it is done
      // rather than finding out from a failed clause 6.
      const wide = samples.filter((x) => x.width > 627).length
      const narrow = new Set(samples.filter((x) => x.width < 627).map((x) => x.width)).size
      note(
        `      have ${wide} above 627px and ${narrow} distinct below it; want 1 and 2. ` +
          (wide >= 1 && narrow >= 2 ? 'Run __gate.widthsDone().' : 'Drag again, then sample again.')
      )
      return s
    },

    async widths() {
      heading('the widths — what the bar costs and what it disturbs')

      if (toggle.hidden || status.textContent === '') {
        ok(
          6,
          'the widths were taken with a document open and a compile behind them',
          false,
          `#toggle hidden ${toggle.hidden}, #status ${JSON.stringify(status.textContent)} — ` +
            'the header cannot grow from here, so the control below would pass on nothing'
        )
        return tally('widths')
      }

      const api = window['__TAURI__']
      const LogicalSize = (api.dpi && api.dpi.LogicalSize) || (api.window && api.window.LogicalSize)
      const win = api.window && api.window.getCurrentWindow && api.window.getCurrentWindow()

      /* **Whether this window will resize itself is settled by trying, not by
         looking, and the difference cost a run.** `setSize` is on the object
         whatever the bundle permits — Tauri v2 gates the window API in
         `app/capabilities/default.json`, and this app grants `core:default`,
         whose window set is `allow-scale-factor`, `allow-inner-size`,
         `allow-title` and the rest of the *getters*, with no `allow-set-size`
         among them. So a `typeof` check reports a function that rejects at the
         IPC, and the first version of this gate threw out of the method after
         printing its heading and nothing else.

         The probe is a `setSize` to the size the window already has: a no-op
         where it is allowed, and the refusal itself where it is not.

         **This gate does not ask for the permission**, and that is the decision
         rather than an oversight: a capability widened to run a check is a
         capability the shipped app carries for ever, and the hand path below
         measures the same thing. */
      let scale = 1
      let back = null
      let refusal = ''
      if (!LogicalSize || !win || typeof win.setSize !== 'function') {
        refusal = 'this window exposes no setSize at all'
      } else {
        try {
          scale = await win.scaleFactor()
          back = await win.innerSize()
          await win.setSize(back)
        } catch (problem) {
          refusal = `${(problem && (problem.message || problem)) || 'it refused'}`
          back = null
        }
      }

      if (back === null) {
        note(`the window will not resize itself — ${refusal}`)
        note('so take the widths by hand: drag the window wide, then narrow, then narrower —')
        note('one width above 627px and two different ones below it — calling')
        note('`await __gate.sample()` after each drag, then `__gate.widthsDone()`.')
        note('Nothing is skipped: widthsDone() is the same judging either way.')
        return { passed: 0, failed: 0 }
      }

      const height = Math.round(back.height / scale)

      /* **Asked for, then read back.** `setSize` is a request: a window manager
         may clamp it, and the viewport that answers is the only width any
         clause below may be keyed to. `innerWidth` settling is what is waited
         for, not a fixed delay. */
      const goTo = async (px) => {
        await win.setSize(new LogicalSize(px, height))
        let last = -1
        for (let i = 0; i < 60 && (innerWidth !== last || innerWidth === 0); i++) {
          last = innerWidth
          await frame()
        }
        await wait(900)
      }

      try {
        for (const px of [900, 620, 500]) {
          await goTo(px)
          if (innerWidth !== px) note(`asked for ${px}px and the viewport answered ${innerWidth}px`)
          await this.sample()
        }
      } finally {
        // The restore is a courtesy and must not replace the run's own result
        // with its own failure, which is what an unguarded reject would do.
        try {
          await win.setSize(back)
          await wait(900)
          note(`restored to ${innerWidth}px`)
        } catch (problem) {
          note(`could not put the window back: ${(problem && problem.message) || problem}`)
        }
      }

      return this.widthsDone()
    },

    widthsDone() {
      const wide = samples.filter((s) => s.width > 627)
      const narrow = samples.filter((s) => s.width < 627)
      const narrowWidths = new Set(narrow.map((s) => s.width))

      /* **The positive control, and the clause that decides whether the rest of
         this method tested anything.** 627px is the header's own derived
         threshold — its seven visible children measure 68+62+86+73+59+114+93 =
         555px, plus six 8px gaps and 24px of padding. It is `flex-wrap: nowrap`
         and so never wraps *as a row*; its items' own text wraps inside them and
         the bar grows taller: 47px above the threshold, 66px at 620 and 81px at
         500, read as `offsetHeight` for the whole numbers' sake against rects of
         47.25, 66 and 80.5. **This phase does not fix that** — it is shipped
         behaviour. It is used here as evidence that the viewport really was
         narrow, so a run in which the header never grew has not tested the two
         clauses after it and must not read as a pass. */
      const grew = wide.length > 0 && narrow.length > 0 &&
        narrow.every((s) => s.header > Math.min(...wide.map((w) => w.header)))
      ok(
        6,
        'the sweep really was narrow — three widths, and the header grew below 627px',
        wide.length >= 1 && narrowWidths.size >= 2 && grew,
        `${samples.length} widths: ${samples.map((s) => `${s.width}px→header ${s.header}`).join(', ')}`
      )

      /* The bar's own two rules, `min-width: 0` on the cell and `flex: none` on
         the brand, are what this is about: without them a long name pushes the
         brand out and the footer stops being 24px tall. */
      const kept = samples.filter((s) => s.footer === 24 && s.brand > 0 && s.said === 'Letur')
      ok(
        7,
        'the footer kept its 24px and its brand at every width, and the sum stayed exact',
        samples.length > 0 && kept.length === samples.length && samples.every((s) => s.sums),
        samples.map(said).join('  |  ')
      )

      ok(
        8,
        'no ResizeObserver error at any width',
        loops === 0,
        `${loops} of them${loops ? ' — ' + spoken.filter((s) => s.includes('ResizeObserver')).join(' | ') : ''}`
      )

      ok(
        9,
        'no other error reached the console',
        noise - loops === 0,
        `${noise} uncaught, ${loops} of them ResizeObserver` +
          `${spoken.length ? ' — ' + spoken.join(' | ') : ''}`
      )

      note('run __gate.report() to copy the whole transcript back, then check `git status` is clean.')
      return tally('widths')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open tests/fixtures/panel/book.md → await bar() → await pick() → ' +
      'await widths() → report().',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
