/* mpdf-010 Phase 3 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The Rust half is `cargo test --workspace`, which holds all five clauses about
   the rule: the file appears and nothing else does, both spellings of *outside*
   are refused by name, a link out of the project is refused, an existing file
   keeps its bytes, and the kinds are the pipeline's own. What only a window can
   say is that **the row arrives on its own** — nothing tells the panel about
   the create, the file lands under the watched root and comes back as
   `watch::Change::Tree` one debounce later — and that **a create compiles
   nothing**, so the drawn page does not move and is not marked out of date.

   ONE SETUP STEP, in a terminal, before you start:

     rm -rf "$TMPDIR/mpdf-010-phase3"
     cp -R tests/fixtures/panel "$TMPDIR/mpdf-010-phase3"
     echo "$TMPDIR/mpdf-010-phase3"

   **It runs over a copy and never over the repository.**
   `tests/fixtures/panel/` is fully tracked and enumerated to eleven rows by
   `app/src/document.rs:the_listing_is_the_disk_and_what_the_master_names`, so a
   twelfth row would fail that test on the next `cargo test`;
   `samples/showcase/` is tracked too; and **there is no delete in this app
   until Phase 4**, so nothing here could undo either. Remaking the copy is what
   makes the run re-runnable, which Phase 2's gate made an explicit property and
   this one keeps. `git status` is clean before and after.

   `cp -R` implies `-P` on macOS, so `<copy>/outside` arrives as a symlink whose
   relative target does not exist under `$TMPDIR`. It contributes no rows either
   way — `kind_of("outside")` is `None` for a name with no extension — so no
   clause below depends on which it is.

   **It has no preconditions**, as Phases 1, 2 and 5 had none: every clause is
   about the DOM and about `invoke('status')`'s own answer, so no failure it
   reports can be about the size of your window or its pixel ratio.

   ORDER:
     __gate.arm()              <- BEFORE opening anything, from the empty state
     open <copy>/sections/text.md
     await __gate.create()     <- clauses 1, 2
     await __gate.refuse()     <- clauses 3, 4
     __gate.report()

   Run this against the build before this phase and `__gate.arm()` throws
   `TypeError: Cannot read properties of null (reading 'hidden')` — `#new-file`
   is not in that page at all, which is the first thing this phase adds.      */
;(() => {
  const files = document.getElementById('files')
  const list = document.getElementById('parts')
  const pages = document.getElementById('pages')
  const problem = document.getElementById('error')
  const divergence = document.getElementById('divergence')
  const newToggle = document.getElementById('new-toggle')
  const newFile = document.getElementById('new-file')
  const newName = document.getElementById('new-name')
  const newRefusal = document.getElementById('new-refusal')
  const { invoke } = window['__TAURI__'].core

  let noise = 0
  const spoken = []
  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* The panel is refreshed by the watch and not by the command's return, so the
     row lands one `app/src/watch.rs:DEBOUNCE` (100 ms) after the write plus
     whatever the platform took to report it. A second is some tens of times
     that, and this script is read by a person rather than by a timer. The same
     wait after an open is the compile's, matching Phase 2's `settled`. */
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

  /* The panel as a reader sees it: every row in order, each as the text it
     draws plus the depth it sits at, so a clause can say *where* a row is and
     not only that it exists. A folder heading is drawn as `name/`, which is the
     `#files li.folder .name::after` rule, so it is spelled that way here. */
  const drawn = () =>
    [...list.children].map((li) => {
      const name = li.querySelector('.name')?.textContent ?? ''
      return `${li.classList.contains('folder') ? name + '/' : name}@${li.dataset.depth}`
    })

  /* Ask for a file the way the reader does: reveal the field, type the whole
     root-relative path, submit the form. `requestSubmit` and not a click on the
     button, so the input's own `required` still runs — a submit dispatched by
     hand would skip it. */
  const askFor = async (path) => {
    if (newFile.hidden) newToggle.click()
    newName.value = path
    newFile.requestSubmit()
    await settled()
  }

  let before = null
  let page = null

  window.__gate = {
    report() {
      console.log(`mpdf-010 Phase 3 gate\n${transcript.join('\n')}`)
      return `${transcript.length} lines`
    },

    arm() {
      noise = 0
      spoken.length = 0
      before = null
      page = null
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

      ok(
        0,
        'the empty state holds no panel and no name field',
        files.hidden && newFile.hidden,
        `files.hidden ${files.hidden}, new-file.hidden ${newFile.hidden}`
      )

      console.log(
        '%carmed%c  — now open $TMPDIR/mpdf-010-phase3/sections/text.md, then run: await __gate.create()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
    },

    async create() {
      heading('a copy of the fixture — a file made from the panel')
      await settled()

      before = await invoke('status')
      page = {
        pages: pages.children.length,
        stale: pages.classList.contains('stale'),
        failed: problem.hidden
      }
      note(`root's main is ${before.main}, the pane holds ${before.edited}`)
      note(`the panel holds ${drawn().length} rows: ${drawn().join(' ')}`)

      /* `<copy>/sections/text.md` roots at `<copy>` by Phase 1's climb —
         `book.md` names it — so `sections/` is a folder the panel already draws
         and the new row has somewhere to land that is not the root. */
      if (before.edited !== 'book.md' || before.main !== 'book.md') {
        ok(1, 'a create puts a row in the panel', false,
          `the project did not open as expected: main ${before.main}, edited ${before.edited}`)
        return tally('create')
      }

      const was = drawn()
      await askFor('sections/discussion.md')
      const now = drawn()

      /* Clause 1. **The row arrives on its own and in Rust's order.** Nothing
         told the panel about the create: the file landed under the watched
         root, came back as `Change::Tree`, and the panel was rebuilt off the
         status that followed. `discussion.md` sorts byte-wise before
         `mark.svg`, so it is the first row under the `sections` heading — which
         is the order asserted where it can be wrong, rather than the row merely
         being present somewhere. */
      const at = now.indexOf('discussion.md@1')
      ok(
        1,
        'the create put one row under the sections heading, in Rust’s own order',
        at > 0 &&
          now[at - 1] === 'sections/@0' &&
          now[at + 1] === 'mark.svg@1' &&
          now.length === was.length + 1,
        `${was.length} rows → ${now.length}; around it: ${now.slice(Math.max(at - 1, 0), at + 2).join(' ')}`
      )

      /* Clause 2. **A create compiles nothing.** `revision` standing still is
         what "does not compile" means as an assertion; `edited` unmoved is
         §2's *"Phase 3 creates the file and stops"* — the new file does not go
         into the pane; and the drawn page is the one that was there, neither
         redrawn nor marked out of date. */
      const after = await invoke('status')
      ok(
        2,
        'nothing compiled: the revision, the pane and the drawn page are all where they were',
        after.revision === before.revision &&
          after.edited === before.edited &&
          after.main === before.main &&
          pages.children.length === page.pages &&
          pages.classList.contains('stale') === page.stale,
        `revision ${after.revision} (was ${before.revision}), edited ${after.edited}, ` +
          `${pages.children.length} pages (was ${page.pages}), ` +
          `stale ${pages.classList.contains('stale')} (was ${page.stale})`
      )

      note('now run: await __gate.refuse()')
      return tally('create')
    },

    async refuse() {
      heading('the refusal a reader meets by typing')

      const was = drawn()
      await askFor('../escape.md')

      /* Clause 3. **The sentence lands beside the field**, and neither of the
         two routes this spec already refused was taken: `fail` would have
         marked the compiled page stale for a gesture that compiled nothing —
         Phase 5 round 1's blocking finding, now a written rule — and the
         divergence bar would have drawn a `Discard` button beside a sentence
         that names nothing to discard.

         `stale` and the error bar are read before the run and compared after,
         rather than asserted absolutely: a document that will not compile is a
         state this gate has no business having an opinion about, and the claim
         is that **this gesture changed neither**. */
      const said = newRefusal.textContent ?? ''
      ok(
        3,
        'the refusal is beside the field, the page is not marked stale, and there is no Discard',
        !newRefusal.hidden &&
          said.length > 0 &&
          pages.classList.contains('stale') === page.stale &&
          problem.hidden === page.failed &&
          divergence.hidden &&
          drawn().join(' ') === was.join(' '),
        `said "${said}", stale ${pages.classList.contains('stale')} (was ${page.stale}), ` +
          `error bar hidden ${problem.hidden} (was ${page.failed}), ` +
          `divergence hidden ${divergence.hidden}, ${drawn().length} rows (was ${was.length})`
      )

      ok(4, 'no error reached the console', noise === 0,
        `${noise} uncaught${spoken.length ? ' — ' + spoken.join(' | ') : ''}`)

      note('run __gate.report() to copy the whole transcript back, then check `git status` is clean.')
      note('and check the copy: `ls "$TMPDIR/mpdf-010-phase3/sections"` holds discussion.md,')
      note('while `ls "$TMPDIR/escape.md"` holds nothing.')
      return tally('refuse')
    }
  }

  console.log(
    '%c__gate ready%c  —  run __gate.arm() now, before opening anything.\n' +
      'Then: open $TMPDIR/mpdf-010-phase3/sections/text.md → create() → refuse() → report().',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
