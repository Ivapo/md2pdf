/* mpdf-010 Phase 1 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   **It has no preconditions.** Every clause below is about what is in the DOM,
   so unlike `mpdf-009-phase5.js` there is nothing here that turns on
   `devicePixelRatio`, on the pane's width, or on the machine's scrollbar
   setting — and no failure it reports can be about the size of your window.

   The Rust half is `cargo test --workspace`, which holds clauses 1 to 7 of the
   phase's gate. What only a window can say is that the panel Rust computed is
   the panel the reader sees, which is what this checks: **the rows are compared
   against `invoke('status')`'s own answer**, not against a list written here.

   **It is re-runnable.** `setMain()` puts the fixture's main back where it found
   it, so a second run starts where the first did — which matters because the
   store outlives the window and clause 1 asserts a particular main.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open tests/fixtures/panel/sections/text.md
     await __gate.project()    <- clauses 1, 2, 3, 4
     await __gate.setMain()    <- clauses 5, 6
     open samples/article.md
     await __gate.article()    <- clauses 7, 8, 9
     __gate.report()

   Clause 7 is `mpdf-008` Phase 4's gate case (2) inverted deliberately: that
   phase asserted `samples/article.md` draws NO panel. Run this script against
   the build before this phase and clause 7 is the one that fails.            */
;(() => {
  const files = document.getElementById('files')
  const list = document.getElementById('parts')
  const toggle = document.getElementById('toggle')
  const { invoke } = window['__TAURI__'].core

  /* **The window's title is not `document.title`.** `set_title` sets the native
     title bar; the page's own `<title>` says `Letur` and never changes, so a
     clause reading it reported a failure about the wrong string. This is the one
     Tauri call here beyond `invoke`, and it costs no capability: `allow-title`
     is already in `core:window`'s default set, which `core:default` grants. */
  const titled = async () => {
    try {
      return await window['__TAURI__'].window.getCurrentWindow().title()
    } catch (problem) {
      return `<unreadable: ${problem}>`
    }
  }

  let noise = 0
  const spoken = []

  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* The panel is drawn off a status, and a status arrives after a compile. One
     second is some thirty times the compile times `rules/desktop.md` records,
     and this script is read by a person rather than by a timer. */
  const settled = () => wait(1000)

  let pass = 0
  let fail = 0
  /* Every line is kept as well as logged, because Safari's Web Inspector copies
     one console entry at a time and this script writes one per clause. Run
     `__gate.report()` at the end and the whole run comes back as one entry. */
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

  /* The rows as drawn: folder headings and files, in document order, each with
     the depth the page put on it. This is what a reader sees, read back. */
  const drawn = () =>
    [...list.children].map((li) => ({
      folder: li.classList.contains('folder'),
      here: li.classList.contains('here'),
      missing: li.classList.contains('missing'),
      depth: Number(li.dataset.depth),
      name: li.querySelector('.name').textContent,
      set: li.querySelector('.set') !== null
    }))

  /* The rows the entries Rust sent would draw, derived here the way the page
     derives them — which is what makes clause 1 a comparison of two
     independent answers rather than of the page with itself. */
  const expected = (entries, main) => {
    const rows = []
    let folder = []
    for (const entry of entries) {
      const segments = entry.path.split('/')
      const here = segments.slice(0, -1)
      let shared = 0
      while (shared < here.length && here[shared] === folder[shared]) shared++
      for (let at = shared; at < here.length; at++) {
        rows.push({ folder: true, here: false, missing: false, depth: at, name: here[at], set: false })
      }
      folder = here
      rows.push({
        folder: false,
        here: entry.path === main,
        missing: entry.missing,
        depth: here.length,
        name: segments[segments.length - 1],
        set: entry.path !== main && entry.kind === 'markdown' && !entry.missing
      })
    }
    return rows
  }

  const same = (a, b) => JSON.stringify(a) === JSON.stringify(b)

  window.__gate = {
    /* The whole run as one console entry, for copying back in one gesture. */
    report() {
      console.log(`mpdf-010 Phase 1 gate\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      spoken.length = 0
      // **Named and not merely counted.** A run that reports "3 uncaught" and
      // nothing else sends the next round looking for something it cannot see.
      const say = (what) => { noise++; if (spoken.length < 8) spoken.push(what) }
      addEventListener('error', (e) =>
        say(`error: ${e.message || e.error} @ ${e.filename || '?'}:${e.lineno || '?'}`))
      addEventListener('unhandledrejection', (e) =>
        say(`rejection: ${(e.reason && (e.reason.message || e.reason.name)) || String(e.reason)}`))

      ok(0, 'the empty state draws no panel and no toggle',
        files.hidden && toggle.hidden && list.children.length === 0,
        `files.hidden ${files.hidden}, toggle.hidden ${toggle.hidden}, ${list.children.length} rows`)

      console.log('%carmed%c  — now open tests/fixtures/panel/sections/text.md, then run: await __gate.project()',
        'font-weight:bold;color:#1a73e8', 'color:inherit')
    },

    async project() {
      heading('tests/fixtures/panel/sections/text.md — opened from a section')
      await settled()
      const state = await invoke('status')
      const title = await titled()

      /* Clause 1. The observable: the section was opened and the master
         compiled. `main` is what Rust landed on, and the title bar says so. */
      ok(1, 'opening a section landed on the master above it',
        state.main === 'book.md' && title === 'book.md' && state.state === 'current',
        `main ${state.main}, title "${title}", state ${state.state}`)

      /* Clause 2. The panel is the Rust listing, drawn — every row, in order,
         with the folders derived and nothing invented. */
      const rows = drawn()
      const want = expected(state.entries, state.main)
      ok(2, 'the panel holds the rows Rust returned, in that order',
        same(rows, want),
        `${rows.length} rows drawn, ${want.length} expected` +
        (same(rows, want) ? '' : `\n  drawn:    ${JSON.stringify(rows)}\n  expected: ${JSON.stringify(want)}`))

      const marked = rows.filter((row) => row.here)
      ok(3, 'exactly one row is marked, and it is the main',
        marked.length === 1 && marked[0].name === 'book.md',
        `${marked.length} marked: ${marked.map((row) => row.name).join(', ')}`)

      const missing = rows.filter((row) => row.missing)
      ok(4, 'the fold hides the panel and gives it back, keeping the toggle',
        (() => {
          toggle.click()
          const away = files.classList.contains('collapsed') &&
            toggle.getAttribute('aria-expanded') === 'false' && !toggle.hidden
          toggle.click()
          const back = !files.classList.contains('collapsed') &&
            toggle.getAttribute('aria-expanded') === 'true'
          return away && back
        })(),
        `toggle reads "${toggle.textContent.trim()}"`)

      note(`the panel drew ${rows.length} rows, ${missing.length} of them named-and-absent`)
      note('now run: await __gate.setMain()')
      return tally('panel')
    },

    async setMain() {
      heading('the gesture that sets which file compiles')
      const before = await invoke('status')

      const row = [...list.children].find((li) => li.querySelector('.set') !== null)
      if (row === undefined) {
        ok(5, 'a row offers the gesture', false, 'no row carries a set-main button')
        return tally('set main')
      }
      const name = row.querySelector('.name').textContent
      row.querySelector('.set').click()
      await settled()
      await settled()

      const after = await invoke('status')
      const title = await titled()
      const marked = drawn().filter((r) => r.here).map((r) => r.name)
      ok(5, 'the gesture moves the main, the mark and the pane',
        after.main !== before.main &&
        after.main.endsWith(name) &&
        marked.length === 1 && marked[0] === name &&
        title === name,
        `${before.main} -> ${after.main}, marked ${marked.join(',')}, title "${title}"`)

      /* **Put it back**, and check the round trip while doing it. The store
         outlives the window, so a gate that left the fixture altered would fail
         its own clause 1 the second time it was run. */
      const back = [...list.children].find(
        (li) => li.querySelector('.name').textContent === before.main &&
                li.querySelector('.set') !== null
      )
      if (back === undefined) {
        note(`could not restore ${before.main} — the fixture's main is now ${after.main}`)
        return tally('set main')
      }
      back.querySelector('.set').click()
      await settled()
      await settled()

      ok(6, 'and it goes back, so this gate can be run twice',
        (await invoke('status')).main === before.main,
        `${after.main} -> ${(await invoke('status')).main}`)

      note('now open samples/article.md, then run: await __gate.article()')
      return tally('set main')
    },

    async article() {
      heading('samples/article.md — a document that names no section')
      await settled()
      const state = await invoke('status')
      const rows = drawn()

      /* Clause 6. `mpdf-008` Phase 4's gate case (2), inverted. That phase
         asserted this document draws no panel at all. */
      ok(7, 'a document naming no section draws a panel, where it drew none',
        !files.hidden && !toggle.hidden && rows.length > 0,
        `${rows.length} rows, main ${state.main}`)

      /* **`samples/` is the case a tidy fixture could not have caught**: a
         single-file document sits there beside the whole `showcase/` project, so
         a discovery that recursed found `showcase/showcase.md`, called it the
         one master, and compiled it for an author who opened `article.md`. A
         master is never below its own sections, so a `.md` in a subdirectory is
         another project's. */
      ok(8, 'the file that was opened is the file that compiles',
        state.main === 'article.md' &&
        rows.some((row) => row.name === 'article.md' && row.here) &&
        rows.some((row) => row.name === 'pipeline.svg') &&
        rows.some((row) => row.name === 'check.svg'),
        `main ${state.main} — ${rows.map((row) => row.name).join(' · ')}`)

      ok(9, 'no error reached the console', noise === 0,
        `${noise} uncaught${spoken.length ? ' — ' + spoken.join(' | ') : ''}`)

      note('run __gate.report() to copy the whole transcript back in one gesture.')
      return tally('article.md')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open tests/fixtures/panel/sections/text.md → project() → setMain() → ' +
      'open samples/article.md → article() → report().',
    'font-weight:bold;color:#1a73e8', 'color:inherit')
})()
