/* mpdf-010 Phase 4 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The Rust half is `cargo test --workspace`, which holds all seven clauses
   about the rule and the session: the file goes and nothing else does, both
   spellings of *outside* are refused, a link out of the project loses the link
   and keeps its target, nothing at that name is refused, the main is refused,
   the pane falls back to the main after being read rather than assigned, and a
   named section leaves a marked-missing row the next compile refuses on.

   What only a window can say is two things. **The panel is refreshed by the
   command and not by the watch** — clause 2 asks `status` the instant the
   invoke resolves, with no wait at all, which is a claim no amount of
   `settled()` could make. And **the file is really in the Trash**: every Rust
   clause hands in a double, and a double proves nothing about the OS. A call
   that silently unlinked would pass every assertion in the suite.

   ONE SETUP STEP, in a terminal, before you start:

     rm -rf "$TMPDIR/mpdf-010-phase4"
     cp -R tests/fixtures/panel "$TMPDIR/mpdf-010-phase4"
     echo "$TMPDIR/mpdf-010-phase4"

   **It runs over a copy and never over the repository**, which is Phase 3's
   setup and for a reason this phase makes sharper: this is the app's first
   destructive operation and `tests/fixtures/panel/` is fully tracked. Phase 3's
   gate could note that "there is no delete in this app until Phase 4" as the
   reason nothing it did could be undone; that sentence is now false, and the
   copy is the whole of what stands between this script and a deleted fixture.

   `cp -R` implies `-P` on macOS, so `<copy>/outside` arrives as a symlink whose
   relative target does not exist under `$TMPDIR`. It contributes no rows either
   way — `kind_of("outside")` is `None` for a name with no extension — so no
   clause below depends on which it is.

   **It has no preconditions**, as Phases 1, 2, 3 and 5 had none: every clause
   is about the DOM and about `invoke('status')`'s own answer.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open <copy>/sections/text.md
     await __gate.controls()   <- clause 1
     await __gate.remove()     <- clauses 2, 3
     await __gate.fallback()   <- clause 4
     await __gate.section()    <- clauses 5, 6
     __gate.report()
     then the one claim by eye, which report() prints again

   Run this against the build before this phase and `__gate.controls()` reports
   clause 1 failed with every count at zero — `.trash` is in no row of that
   page, which is the gesture this phase adds.                                */
;(() => {
  const files = document.getElementById('files')
  const list = document.getElementById('parts')
  const pages = document.getElementById('pages')
  const problem = document.getElementById('error')
  const divergence = document.getElementById('divergence')
  const text = document.getElementById('text')
  const { invoke } = window['__TAURI__'].core

  let noise = 0
  const spoken = []
  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* A second covers the watch's own `app/src/watch.rs:DEBOUNCE` (100 ms), the
     platform's latency in front of it, and the compile behind it. It is Phase
     3's number and Phase 2's before that. */
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

  /* The panel as a reader sees it, in Phase 3's spelling: a folder heading is
     drawn as `name/`, which is the `#files li.folder .name::after` rule. */
  const drawn = () =>
    [...list.children].map((li) => {
      const name = li.querySelector('.name')?.textContent ?? ''
      return `${li.classList.contains('folder') ? name + '/' : name}@${li.dataset.depth}`
    })

  /* One file row by the name it draws, which is the path's last segment. Every
     name this fixture holds is unique, so a segment identifies a row. */
  const rowFor = (name) =>
    [...list.children].find(
      (li) => !li.classList.contains('folder') && li.querySelector('.name')?.textContent === name
    )

  /* What controls a row carries, as the reader could reach them. `visibility`
     is what hides them until hover, so they are in the DOM either way and this
     reads the DOM rather than the hover state. */
  const controlsOn = (name) => {
    const li = rowFor(name)
    if (!li) return null
    return [...li.querySelectorAll('.controls button')].map((b) => b.className)
  }

  let page = null

  window.__gate = {
    report() {
      console.log(`mpdf-010 Phase 4 gate\n${transcript.join('\n')}`)
      console.log(
        '%cBY EYE, and it is the claim no test in this repository can make:%c\n' +
          '  open the Trash in Finder. `cover.jpg`, `refs.bib`, `other.md` and `text.md`\n' +
          '  are in it. Select one, Put Back, and it returns to the project folder.\n' +
          '  A call that silently unlinked would have passed every assertion above.',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      spoken.length = 0
      page = null
      // **Named and not merely counted**, per Phase 3: a run reporting
      // "3 uncaught" and nothing else sends the next round looking blind.
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

      ok(0, 'the empty state holds no panel', files.hidden, `files.hidden ${files.hidden}`)

      console.log(
        '%carmed%c  — now open $TMPDIR/mpdf-010-phase4/sections/text.md, then run: await __gate.controls()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
    },

    async controls() {
      heading('a copy of the fixture — which rows carry which controls')
      await settled()

      const opened = await invoke('status')
      page = {
        pages: pages.children.length,
        stale: pages.classList.contains('stale'),
        failed: problem.hidden
      }
      note(`root's main is ${opened.main}, the pane holds ${opened.edited}`)
      note(`the panel holds ${drawn().length} rows: ${drawn().join(' ')}`)

      /* `<copy>/sections/text.md` roots at `<copy>` by Phase 1's climb —
         `book.md` names it — and discovery lands on `book.md`, the one master. */
      if (opened.main !== 'book.md' || opened.edited !== 'book.md') {
        ok(1, 'the controls are where they belong', false,
          `the project did not open as expected: main ${opened.main}, edited ${opened.edited}`)
        return tally('controls')
      }

      /* Clause 1. **Four rows, four different answers**, and each is a rule:
         the main draws neither button, its file being the one the delete
         refuses; a marked-missing row draws neither, naming a file the disk
         does not hold; a non-main markdown row draws both, which is the pair
         `.controls` exists to seat since OQ-5's `margin-left: auto` was
         reasoned for a row that could only ever carry one; and an image row
         draws the delete alone, nothing else being able to compile. */
      const main = controlsOn('book.md')
      const missing = controlsOn('missing.md')
      const other = controlsOn('other.md')
      const figure = controlsOn('cover.jpg')

      ok(
        1,
        'the main and the missing row carry none, a markdown row carries both, a figure carries the delete',
        main?.length === 0 &&
          missing?.length === 0 &&
          other?.join(' ') === 'set trash' &&
          figure?.join(' ') === 'trash',
        `book.md [${main}], missing.md [${missing}], other.md [${other}], cover.jpg [${figure}]`
      )

      note('now run: await __gate.remove()')
      return tally('controls')
    },

    async remove() {
      heading('the panel is refreshed by the command, not by the watch')

      const before = await invoke('status')

      /* Clause 2. **No wait at all, and that is the clause.** `cover.jpg` is a
         figure `book.md` does not name, so nothing about it is in the asset
         list and nothing about it compiles. If the panel were left to the
         watch, this status — fetched the instant the invoke resolves, before
         any `app/src/watch.rs:DEBOUNCE` could have elapsed — would still hold
         the row. It does not, because `Session::trash` re-walked with
         `document::files_under` itself, which it may because this app made the
         change and knows it. */
      await invoke('trash_file', { path: 'cover.jpg' })
      const now = await invoke('status')

      ok(
        2,
        'the row is gone from `status` with no wait, so the command re-walked rather than the watch',
        !now.entries.some((e) => e.path === 'cover.jpg') &&
          before.entries.some((e) => e.path === 'cover.jpg') &&
          now.revision === before.revision &&
          now.edited === before.edited,
        `${before.entries.length} entries → ${now.entries.length}, ` +
          `revision ${now.revision} (was ${before.revision}), edited ${now.edited}`
      )

      /* Clause 3. **Through the button a reader actually presses**, where
         clause 2 went straight to the command to control its timing. This one
         may wait: what it asserts is the wiring and the drawing, not the
         moment. `refs.bib` is a bibliography nothing names — OQ-2 leaves its
         body inert, and this is the gesture that reaches it all the same. */
      const was = drawn()
      rowFor('refs.bib')?.querySelector('.trash')?.click()
      await settled()
      const after = drawn()

      ok(
        3,
        'the button on a row removes it, and the drawn page is neither redrawn nor marked stale',
        !after.includes('refs.bib@0') &&
          was.includes('refs.bib@0') &&
          after.length === was.length - 1 &&
          pages.children.length === page.pages &&
          pages.classList.contains('stale') === page.stale &&
          problem.hidden === page.failed &&
          divergence.hidden,
        `${was.length} rows → ${after.length}; ${pages.children.length} pages (was ${page.pages}), ` +
          `stale ${pages.classList.contains('stale')} (was ${page.stale}), ` +
          `error bar hidden ${problem.hidden} (was ${page.failed})`
      )

      note('now run: await __gate.fallback()')
      return tally('remove')
    },

    async fallback() {
      heading('the pane falls back to the file that compiles')

      /* `other.md` is markdown the master does not name, so putting it in the
         pane is Phase 2's plain switch and trashing it costs the document
         nothing — which is what isolates the fall-back from the marked-missing
         row clause 5 is about. */
      rowFor('other.md')?.querySelector('.name')?.click()
      await settled()

      const holding = await invoke('status')
      if (holding.edited !== 'other.md') {
        ok(4, 'the pane falls back to the main', false,
          `the switch did not happen: edited ${holding.edited}`)
        return tally('fallback')
      }

      rowFor('other.md')?.querySelector('.trash')?.click()
      await settled()
      const fell = await invoke('status')

      /* Clause 4. **Three things, and the middle one is the clause**, exactly
         as the Rust clause 6 has it: `edited` moved, and the pane was *read*
         rather than the field assigned. A build that armed the loops without
         loading would satisfy the first and the third and hold the trashed
         file's text here — and would then write it over the master on `⌘S`. */
      ok(
        4,
        'the pane holds the main, and holds the main’s own text rather than the trashed file’s',
        fell.edited === 'book.md' &&
          fell.main === 'book.md' &&
          text.value.includes('A Book the Panel Lists') &&
          !text.value.includes('other') &&
          !fell.entries.some((e) => e.path === 'other.md'),
        `edited ${fell.edited}, ${text.value.length} chars in the pane, ` +
          `row still listed ${fell.entries.some((e) => e.path === 'other.md')}`
      )

      note('now run: await __gate.section()')
      return tally('fallback')
    },

    async section() {
      heading('a section the master names, and the refusal that follows it')

      const was = await invoke('status')
      rowFor('text.md')?.querySelector('.trash')?.click()
      await settled()
      const gone = await invoke('status')

      /* Clause 5. **The row stays and changes**, which is §2's union doing its
         one job: `document::files_under` stops finding the file, and
         `document::merge` puts the path straight back as `missing: true`
         because the master still names it. The compile then refuses — and the
         sentence is `document::read_sections_with`'s own, not
         `md2pdf_core::Error::MissingSection`, which this app never reaches:
         `?` short-circuits before `core/src/sections.rs`'s only raising site
         is called at all. */
      const row = gone.entries.find((e) => e.path === 'sections/text.md')
      const said = problem.textContent ?? ''

      ok(
        5,
        'the deleted section is now the marked-missing row, and the compile refuses by its name',
        row?.missing === true &&
          was.entries.some((e) => e.path === 'sections/text.md' && !e.missing) &&
          !problem.hidden &&
          said.includes('cannot read') &&
          said.includes('for the section') &&
          said.includes('text.md'),
        `row ${row ? `missing ${row.missing}` : 'absent'}, error bar hidden ${problem.hidden}, ` +
          `said "${said}"`
      )

      ok(6, 'no error reached the console', noise === 0,
        `${noise} uncaught${spoken.length ? ' — ' + spoken.join(' | ') : ''}`)

      note('run __gate.report() to copy the transcript back and print the by-eye step.')
      note('then check `git status` is clean — nothing here touched the repository.')
      return tally('section')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open $TMPDIR/mpdf-010-phase4/sections/text.md → controls() → remove() → fallback() → section() → report().',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
