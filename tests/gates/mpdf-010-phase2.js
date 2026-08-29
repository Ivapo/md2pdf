/* mpdf-010 Phase 2 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The Rust half is `cargo test --workspace`, which holds clauses 1 to 6 of the
   phase's gate. What only a window can say is the phase's own observable: **the
   page shows the whole compiled document while the pane holds one section of
   it, and the caret's own page is right** — which is the caret, the anchors, the
   compile and the panel all agreeing at once, and no unit test sees all four.

   **It has no preconditions**, as Phase 1's had none: every clause is about the
   DOM and about `invoke('status')`'s own answer, so no failure it reports can be
   about the size of your window or its pixel ratio.

   **It writes to a tracked file and puts it back.** Clause 4 types one character
   into `samples/showcase/sections/notes-and-sources.md`, because `caretPage` is
   consulted only on a status carrying a new `revision` — a caret move alone
   scrolls nothing, so the gate has to say what makes the redraw happen. Clause 6
   then takes the discard the switch's refusal names, which both restores the
   file and exercises the second way out. **Run `git status` when you are done**:
   it must be clean.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open samples/showcase/showcase.md
     await __gate.opened()     <- clauses 1, 2
     await __gate.click()      <- clauses 3, 4, 5
     await __gate.refuse()     <- clauses 6, 7
     __gate.report()

   Run this against the build before this phase and clause 3 is the one that
   fails first: a row's body was inert, so there is nothing to click.        */
;(() => {
  const files = document.getElementById('files')
  const list = document.getElementById('parts')
  const text = document.getElementById('text')
  const pages = document.getElementById('pages')
  const bar = document.getElementById('divergence')
  const said = document.getElementById('divergence-text')
  const { invoke } = window['__TAURI__'].core

  /* The window's title is not `document.title` — `set_title` sets the native
     title bar, where the page's own `<title>` says `Letur` and never changes.
     Phase 1's gate learned this the hard way and it is the one Tauri call here
     beyond `invoke`. */
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

  /* A compile falls due one typing debounce (300 ms) after the last keystroke,
     and the panel is drawn off the status that follows it. A second is some
     thirty times the compile times `rules/desktop.md` records, and this script
     is read by a person rather than by a timer. */
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

  /* The rows as drawn, with both marks read back. `holding` is this phase's
     own; `here` is Phase 1's and must not have moved. */
  const drawn = () =>
    [...list.children].map((li) => ({
      folder: li.classList.contains('folder'),
      here: li.classList.contains('here'),
      holding: li.classList.contains('holding'),
      name: li.querySelector('.name').textContent,
      opens: li.querySelector('button.name') !== null
    }))

  const row = (name) => [...list.children].find((li) => li.querySelector('.name').textContent === name)

  /* Which page is at the top of the pane. Each wrapper carries the number
     `pdf.js` gave it, so this reads the reader's place off the pane rather than
     off anything the page was told to do. **It is reported and not asserted on**
     — see `reached` below. */
  const showing = () => {
    const top = pages.scrollTop
    const at = [...pages.children].find((w) => w.offsetTop + w.offsetHeight > top)
    return at ? at.number : null
  }

  /* Where the pane would sit if it opened on this page, **clamped the way the
     browser clamps it**.

     `openPdf` scrolls with `applyAnchor({ page: page - 1, fraction: 0 })`, which
     writes `scrollTop = kids[i].offsetTop`; a browser then clamps that to
     `scrollHeight - clientHeight`. `showcase.md` is six pages and the heading
     under test is on the sixth, so on any window whose pane is taller than one
     page the clamp leaves the *fifth* page's tail at the top of the viewport —
     the app having scrolled as far as the document allows. Reading back "which
     page is at the top" would therefore fail on a tall window and pass on a
     short one, which is a claim about the reader's monitor and not about this
     phase. This compares the scroll the app actually took against the anchor's
     own page, both read off the live DOM. */
  const reached = (page) => {
    const at = pages.children[page - 1]
    if (at === undefined) return null
    const room = pages.scrollHeight - pages.clientHeight
    return Math.min(at.offsetTop, Math.max(room, 0))
  }

  /* Put the caret at the end of a line, counted the way `caretPage` counts —
     newlines before `selectionStart` — so the gate and the page agree. */
  const caretOnLine = (line) => {
    const upto = text.value.split('\n').slice(0, line).join('\n').length
    text.focus()
    text.setSelectionRange(upto, upto)
  }

  /* One character, typed the way a keyboard types it: the page listens for
     `input`, which `value = …` alone does not raise. */
  const type = (what) => {
    const at = text.selectionStart
    text.setRangeText(what, at, at, 'end')
    text.dispatchEvent(new Event('input', { bubbles: true }))
  }

  let held = ''
  let wanted = null

  window.__gate = {
    report() {
      console.log(`mpdf-010 Phase 2 gate\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      spoken.length = 0
      held = ''
      wanted = null
      // **Named and not merely counted.** A run reporting "3 uncaught" and
      // nothing else sends the next round looking for something it cannot see.
      const say = (what) => { noise++; if (spoken.length < 8) spoken.push(what) }
      addEventListener('error', (e) =>
        say(`error: ${e.message || e.error} @ ${e.filename || '?'}:${e.lineno || '?'}`))
      addEventListener('unhandledrejection', (e) =>
        say(`rejection: ${(e.reason && (e.reason.message || e.reason.name)) || String(e.reason)}`))

      ok(0, 'the empty state draws no panel', files.hidden && list.children.length === 0,
        `files.hidden ${files.hidden}, ${list.children.length} rows`)

      console.log('%carmed%c  — now open samples/showcase/showcase.md, then run: await __gate.opened()',
        'font-weight:bold;color:#1a73e8', 'color:inherit')
    },

    async opened() {
      heading('samples/showcase/showcase.md — opened, so the two files are one')
      await settled()
      const state = await invoke('status')

      /* Clause 1. An open still puts the main in the pane, which is Phase 1's
         behaviour and is what this phase does *not* change. */
      ok(1, 'an open leaves the pane holding the file that compiles',
        state.main === 'showcase.md' && state.edited === 'showcase.md' && state.state === 'current',
        `main ${state.main}, edited ${state.edited}, state ${state.state}`)

      const rows = drawn()
      const marked = rows.filter((r) => r.here)
      const holding = rows.filter((r) => r.holding)
      ok(2, 'one row carries both marks, and the sections offer to open',
        marked.length === 1 && holding.length === 1 &&
        marked[0].name === 'showcase.md' && holding[0].name === 'showcase.md' &&
        rows.some((r) => r.name === 'notes-and-sources.md' && r.opens) &&
        !rows.some((r) => r.name === 'refs.bib' && r.opens),
        `here ${marked.map((r) => r.name)}, holding ${holding.map((r) => r.name)}, ` +
        `${rows.filter((r) => r.opens).length} rows open`)

      note('now run: await __gate.click()')
      return tally('opened')
    },

    async click() {
      heading('sections/notes-and-sources.md — clicked, and the caret followed')
      const before = await invoke('status')
      const target = row('notes-and-sources.md')
      if (target === undefined || target.querySelector('button.name') === null) {
        ok(3, 'the row body opens', false, 'no row for notes-and-sources.md carries a name button')
        return tally('click')
      }

      target.querySelector('button.name').click()
      await settled()
      await settled()

      const state = await invoke('status')
      const title = await titled()
      const rows = drawn()

      /* Clause 3. The pane moved and the compile did not. */
      ok(3, 'the row put that file in the pane and left the main compiling',
        state.edited === 'sections/notes-and-sources.md' &&
        state.main === 'showcase.md' &&
        state.state === 'current' &&
        title === 'notes-and-sources.md',
        `main ${state.main}, edited ${state.edited}, title "${title}", state ${state.state}`)

      /* Clause 4. Two marks on two rows, which is the state §1 sketches. */
      const marked = rows.filter((r) => r.here).map((r) => r.name)
      const holding = rows.filter((r) => r.holding).map((r) => r.name)
      ok(4, 'the panel marks the two files separately',
        marked.length === 1 && marked[0] === 'showcase.md' &&
        holding.length === 1 && holding[0] === 'notes-and-sources.md',
        `here ${marked.join(',')}, holding ${holding.join(',')}`)

      held = text.value
      /* Clause 5. The pane holds that file's own text, and the anchors are its
         own headings — the master's first is line 12, so an anchor at line 1
         could not be `showcase.md`'s. */
      const anchors = state.anchors.map((a) => a.line)
      ok(5, 'the pane holds that file, and the anchors are its headings',
        held.startsWith('# Notes and sources') &&
        anchors.length === 3 && anchors[0] === 1 &&
        state.revision > before.revision,
        `${held.split('\n').length} lines in the pane, anchor lines ${anchors.join(',')}`)

      wanted = state.anchors.find((a) => a.line === 12)
      note(`its "## Citations" heading landed on page ${wanted ? wanted.page : '?'} of the whole document`)
      note('now run: await __gate.refuse()')
      return tally('click')
    },

    async refuse() {
      heading('the caret, and the two ways out of a dirty buffer')
      if (wanted === undefined || wanted === null) {
        ok(6, 'the caret opens the page its heading landed on', false,
          'no anchor at line 12 — run __gate.click() first')
        return tally('refuse')
      }

      /* **Type a character.** `caretPage` is consulted only on a status carrying
         a new `revision`, so moving the caret alone scrolls nothing. */
      caretOnLine(13)
      type(' ')
      await settled()
      await settled()

      /* Clause 6, the phase's observable. The heading is a dozen lines into a
         section the master names, and the page it landed on is the *document's*
         and not that file's — so a pane still at the top is the old behaviour,
         which is what `wanted.page > 1` and a non-zero scroll rule out
         together. */
      const want = reached(wanted.page)
      const got = pages.scrollTop
      ok(6, 'the caret opened the page that heading landed on in the whole document',
        want !== null && wanted.page > 1 && want > 0 && Math.abs(got - want) <= 2,
        `scrollTop ${Math.round(got)}, page ${wanted.page} of ${pages.children.length} ` +
        `is at ${want === null ? '?' : Math.round(want)} (clamped), showing page ${showing()}`)

      /* Clause 7. The buffer is dirty now, so the switch refuses in a sentence
         that does not claim the file moved — and the way out is beside it. */
      const other = row('mathematics.md')
      if (other === undefined || other.querySelector('button.name') === null) {
        ok(7, 'the switch refuses and names the way out', false, 'no row for mathematics.md')
        return tally('refuse')
      }
      other.querySelector('button.name').click()
      await settled()

      const refused = await invoke('status')
      const sentence = said.textContent
      ok(7, 'the switch refuses over unsaved work, and the way out is beside it',
        refused.edited === 'sections/notes-and-sources.md' &&
        !bar.hidden &&
        sentence.length > 0 &&
        !sentence.includes('changed on disk') &&
        document.getElementById('discard') !== null,
        `edited ${refused.edited}, bar "${sentence}"`)

      /* **Put the file back**, which is a clause rather than tidiness: the
         character above went into a tracked file. Taking the discard restores it
         and exercises the second way out in one gesture. */
      document.getElementById('discard').click()
      await settled()
      await settled()

      const clean = await invoke('status')
      ok(8, 'the discard puts the file back, so this gate can be run twice',
        clean.divergence === null && text.value === held && bar.hidden,
        `${text.value.length} chars in the pane, ${held.length} before typing`)

      ok(9, 'no error reached the console', noise === 0,
        `${noise} uncaught${spoken.length ? ' — ' + spoken.join(' | ') : ''}`)

      note('run __gate.report() to copy the whole transcript back, then check `git status` is clean.')
      return tally('refuse')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open samples/showcase/showcase.md → opened() → click() → refuse() → report().',
    'font-weight:bold;color:#1a73e8', 'color:inherit')
})()
