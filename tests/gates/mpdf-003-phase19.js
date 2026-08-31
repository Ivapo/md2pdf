/* mpdf-003 Phase 19 exit gate — clause 9, the reading left to a person. Paste
   into the Web Inspector console of a `cargo tauri dev` window.

   **Everything else in this phase's gate runs without a person** and is recorded
   here so a reader knows what this file is *not* covering: `cargo test
   --workspace`, `bun app/typecheck.mjs`, `bun app/harness/checks.mjs` and
   `--webkit` at fourteen clauses each, `--falsify` at eleven isolated mutations
   in both engines, and `bun app/driver/drive.mjs` and `--falsify` unchanged. What
   is left is three gestures, and two of them open a **native save panel** that
   neither rig can drive — `app/harness/stub.mjs`'s `dialog.save` answers `null`
   deliberately, and the WebDriver session the driver speaks to cannot reach an
   `NSSavePanel` either.

   **So the person performs the gesture and this judges the reading**, which is
   the difference between a gate and a list of things to look at. Nothing below
   is answered by eye:

     * the receipt is read off `#receipt` by a `MutationObserver`, so a sentence
       that appeared and went is captured even if you take longer than its own
       four seconds to run the next call;
     * **whether you saved inside or outside is not asked, it is deduced.** A
       receipt appeared, so the dialog was not cancelled; the pane moved, so the
       destination was in the project. Those two together are the whole
       distinction this phase is about, and neither is a question you can answer
       backwards.

   **One reading is deliberately a note and not a clause**, and it was a clause in
   this file's first draft, which failed on it: that an outside save **compiles
   nothing**. The claim is true and
   `app/src/preview.rs:a_save_as_outside_the_project_compiles_nothing` is where it
   is asserted, with nothing in the way. It cannot be read at the window, because
   `saveDocumentAs` sends the pane's text over with `invoke('edit')` **before** it
   opens the dialog and `Session::edit` kicks the 300ms typing debounce — so a
   compile falls due while the native panel is still open, on both paths, for a
   reason that has nothing to do with the save. **A gate that reads a number it
   cannot attribute is worse than one that does not read it**, and this is the
   shape of that.

   **It reads the cell's emptiness and never the timer's length.** `RECEIPT_MS`
   is `app/dist/index.html`'s and could move without touching this file: what is
   asserted is that the sentence appeared and that the cell went back to empty on
   its own, waited for under a bound generous enough not to be a measurement.

   NO PRECONDITION beyond a document being open, and **it writes two files** —
   the two copies you save, one inside the project and one outside it. Put the
   outside one somewhere you do not mind, `~/Desktop` for instance. **It writes
   nothing into the repository** if the open document is not in one; if you drive
   it on this repo's own `samples/`, delete the inside copy afterwards, or
   `git status` will not be clean and clause 10 is the gate's own.

   ORDER:
     __gate.forget()          <- once, at the start: clears any earlier transcript
     await __gate.arm()       <- installs the observer and reads the starting state
     ... press ⌘S ...
     await __gate.saved()     <- clause 9a
     ... press ⇧⌘S and save INSIDE the project (the panel already opens there) ...
     await __gate.inside()    <- clause 9b
     ... press ⇧⌘S and save OUTSIDE the project (navigate out first) ...
     await __gate.outside()   <- clause 9c
     __gate.report()          <- the whole run as one console entry

   **The transcript is kept in `localStorage`**, `tests/gates/mpdf-003-phase13.js`'s
   answer to Safari's Web Inspector copying one entry at a time. No clause here
   needs a relaunch, but `report()` giving the run back in one piece is worth the
   same six lines.

   Paste this into the build before this phase and `arm()` refuses: `#receipt` is
   not in that page at all, which is the first thing this phase adds.          */

;(() => {
  const { invoke } = window['__TAURI__'].core

  const receipt = document.getElementById('receipt')
  const editedCell = document.getElementById('edited')

  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* **A bound on the wiring and not a measurement of it.** The page clears the
     cell after four seconds; this waits far longer before calling it stuck, so
     the interval can move in `app/dist/index.html` without this file noticing —
     which is the same rule `app/harness/checks.mjs` states for its own clause. */
  const PATIENCE_MS = 20000

  let pass = 0
  let fail = 0
  let armed = false
  let noise = 0
  let loops = 0
  const spoken = []

  const KEEP = 'mpdf-003-phase19-gate'
  const transcript = (() => {
    try {
      return JSON.parse(localStorage.getItem(KEEP) ?? '[]')
    } catch {
      return []
    }
  })()
  const keep = () => {
    try {
      localStorage.setItem(KEEP, JSON.stringify(transcript))
    } catch {
      /* A gate that cannot persist still runs; only `report()` is narrower. */
    }
  }
  const ok = (n, name, good, detail) => {
    good ? pass++ : fail++
    transcript.push(`${good ? 'PASS' : 'FAIL'}  ${n}. ${name}${detail ? '  —  ' + detail : ''}`)
    keep()
    console.log(
      `%c${good ? 'PASS' : 'FAIL'}%c  ${n}. ${name}${detail ? '  —  ' + detail : ''}`,
      `font-weight:bold;color:${good ? '#137333' : '#c5221f'}`,
      'color:inherit'
    )
  }
  const note = (s) => {
    transcript.push(`····  ${s}`)
    keep()
    console.log(`%c····%c  ${s}`, 'color:#888', 'color:#888')
  }
  const heading = (s) => {
    transcript.push('', `== ${s}`)
    keep()
    console.log(`%c\n${s}\n`, 'font-weight:bold')
  }
  const tell = (s) => {
    transcript.push(`>>>>  ${s}`)
    keep()
    console.log(`%c>>>>%c  ${s}`, 'font-weight:bold;color:#1a73e8', 'color:inherit')
  }

  /* ------------------------------------------------------------ the recorder */

  /* Every sentence the cell has held since the last `mark()`, and whether it has
     gone back to empty. **The observer is what makes the timing irrelevant**: a
     receipt that appeared and cleared while you were typing the next call is
     still in `seen`, where a poll of `textContent` would have missed it. */
  let seen = []
  let emptied = false
  let watcher = null

  const mark = () => {
    seen = []
    emptied = false
  }

  /** The cell's own emptiness, waited for rather than timed. */
  const untilEmpty = async () => {
    const deadline = Date.now() + PATIENCE_MS
    while (Date.now() < deadline) {
      if (receipt.textContent === '' && seen.length) return true
      await wait(100)
    }
    return receipt.textContent === '' && seen.length > 0
  }

  /** What the bar's left cell says: a bare file name, `namePaneFile`'s own. */
  const pane = () => editedCell.textContent

  /** What Rust says, for the fields no cell carries. */
  const asked = async () => {
    try {
      const state = await invoke('status')
      return { edited: state.edited, revision: state.revision, how: 'status' }
    } catch (problem) {
      return { edited: null, revision: null, how: `status refused: ${problem}` }
    }
  }

  /* `saved as <name> in <folder>`, taken apart so the two halves can be judged
     separately. **Split on the last ` in `**, because a file name may hold one
     and a folder certainly may. */
  const readReceipt = (sentence) => {
    const said = String(sentence ?? '')
    if (!said.startsWith('saved as ')) return null
    const rest = said.slice('saved as '.length)
    const at = rest.lastIndexOf(' in ')
    if (at < 0) return null
    return { name: rest.slice(0, at), folder: rest.slice(at + ' in '.length) }
  }

  /* --------------------------------------------------------------- the gate */

  window.__gate = {
    /** Clear the kept transcript. Once, at the start. */
    forget() {
      transcript.length = 0
      keep()
      console.log('%cforgotten%c  — the transcript is empty', 'font-weight:bold', 'color:inherit')
      return 'run __gate.arm() next'
    },

    /** Install the observer and the error listeners, and read the starting state.
        **It seeds both readings the later clauses compare against**, so a run
        that skips a clause fails on that clause rather than on a missing
        baseline. */
    async arm() {
      if (!receipt) {
        console.log(
          '%crefused%c  — this page has no #receipt, so it is older than mpdf-003 Phase 19',
          'font-weight:bold;color:#c5221f',
          'color:inherit'
        )
        return 'wrong build'
      }

      watcher?.disconnect()
      watcher = new MutationObserver(() => {
        const said = receipt.textContent
        if (said) {
          seen.push(said)
          emptied = false
        } else if (seen.length) {
          emptied = true
        }
      })
      watcher.observe(receipt, { childList: true, characterData: true, subtree: true })

      if (!armed) {
        addEventListener('error', (e) => {
          noise++
          if (`${e.message}`.includes('ResizeObserver')) loops++
          if (spoken.length < 12) spoken.push(`error: ${e.message}`)
        })
        addEventListener('unhandledrejection', (e) => {
          noise++
          if (spoken.length < 12) spoken.push(`rejection: ${(e.reason && e.reason.message) || e.reason}`)
        })
      }
      armed = true
      mark()

      const state = await asked()
      this._before = pane()
      this._edited = state.edited
      this._revision = state.revision

      heading('mpdf-003 Phase 19 — the pane stays in the project, and a save says so')
      note(`the bar names ${JSON.stringify(pane())}; the cell is ${JSON.stringify(receipt.textContent)}`)
      note(`Rust's edited is ${JSON.stringify(state.edited)} at revision ${state.revision} (${state.how})`)
      tell('press ⌘S, then run:  await __gate.saved()')
      return 'armed'
    },

    /** Clause 9a: `⌘S` shows `saved`, and it goes. */
    async saved() {
      heading('9a — a plain save says so, and stops saying it')

      const said = seen[seen.length - 1] ?? ''
      const cleared = await untilEmpty()

      note(`the cell held ${JSON.stringify(seen)}, and is now ${JSON.stringify(receipt.textContent)}`)

      ok(
        '9a',
        '`⌘S` shows `saved`, and the cell is empty again after',
        seen.length === 1 && said === 'saved' && cleared,
        seen.length === 0
          ? 'NOTHING APPEARED — either ⌘S was never pressed, or `save` answered no sentence'
          : `the cell held ${JSON.stringify(seen)}; it cleared: ${cleared}`
      )

      mark()
      tell('press ⇧⌘S and save INSIDE the project — the panel already opens there — then run:  await __gate.inside()')
      return 'clause 9a recorded'
    },

    /** Clause 9b: `⇧⌘S` into the project moves the pane. */
    async inside() {
      heading('9b — a save inside the project moves the pane')

      const was = this._before ?? null
      const now = await asked()
      const said = seen[seen.length - 1] ?? ''
      const parts = readReceipt(said)
      const cleared = await untilEmpty()

      note(`the cell held ${JSON.stringify(said)}`)
      note(`the bar names ${JSON.stringify(pane())}; Rust's edited is ${JSON.stringify(now.edited)}`)
      if (was) note(`it named ${JSON.stringify(was)} before the save`)

      /* **The pane moved is what says the destination was in the project**, and
         a receipt having appeared is what says the dialog was not cancelled.
         Neither is a question, so neither can be answered backwards. */
      const moved = parts !== null && pane() === parts.name && now.edited !== null
      const relative = parts !== null && !String(now.edited).startsWith('/')

      ok(
        '9b',
        '`⇧⌘S` into the project writes the file, moves the pane to it, and names the folder',
        moved && relative && parts.folder.startsWith('/') && cleared,
        parts === null
          ? `NO RECEIPT of the right shape: ${JSON.stringify(said)} — cancelled, or the sentence changed`
          : `named ${JSON.stringify(parts.name)} in ${JSON.stringify(parts.folder)}; ` +
            `the bar now says ${JSON.stringify(pane())}, Rust ${JSON.stringify(now.edited)}; it cleared: ${cleared}`
      )

      this._before = pane()
      this._edited = now.edited
      this._revision = now.revision
      mark()
      tell(
        'press ⇧⌘S, NAVIGATE OUT of the project — ~/Desktop will do — and save there, then run:  await __gate.outside()'
      )
      return 'clause 9b recorded'
    },

    /** Clause 9c: `⇧⌘S` outside it writes the file and leaves the pane. */
    async outside() {
      heading('9c — a save outside the project is a copy, and the receipt says so')

      const was = this._before ?? null
      const wasEdited = this._edited ?? null
      const wasRevision = this._revision ?? null
      const now = await asked()
      const said = seen[seen.length - 1] ?? ''
      const parts = readReceipt(said)
      const cleared = await untilEmpty()

      note(`the cell held ${JSON.stringify(said)}`)
      note(`the bar names ${JSON.stringify(pane())}, where it named ${JSON.stringify(was)} before`)

      /* **A note and not a clause, and the first draft of this gate had it as a
         clause and failed on it.** `Session::save_as` compiles nothing on the
         outside path — `app/src/preview.rs:a_save_as_outside_the_project_compiles_nothing`
         is where that is asserted, with nothing in the way. It cannot be read
         *here*: `saveDocumentAs` sends the pane's text over with `invoke('edit')`
         **before** it opens the dialog, and `Session::edit` kicks the 300ms
         typing debounce — so a compile falls due while the native panel is still
         open, on both paths, for a reason that has nothing to do with the save.
         A revision that moved across this gesture is the typing loop and is
         printed as such. */
      note(
        `revision ${wasRevision} → ${now.revision}` +
          ` — the typing loop's, not the save's: the page's own pre-dialog \`edit\` arms it`
      )

      /* A receipt appeared and the pane did **not** move: the only way both are
         true is a destination outside the project. Cancelling would have left no
         receipt; an inside destination would have moved the pane. */
      /* **Not "the name differs"**, which would misfire on the ordinary case of
         copying a file out under its own name. What says the pane did not move is
         the pane: the bar's cell and Rust's own `edited`, both unchanged. */
      const stayed =
        parts !== null && was !== null && pane() === was && wasEdited !== null && now.edited === wasEdited
      const named = parts !== null && parts.folder.startsWith('/')

      ok(
        '9c',
        '`⇧⌘S` outside the project writes the file, leaves the pane where it was, and names the folder',
        stayed && named && cleared,
        parts === null
          ? `NO RECEIPT of the right shape: ${JSON.stringify(said)} — cancelled, or the sentence changed`
          : `named ${JSON.stringify(parts.name)} in ${JSON.stringify(parts.folder)}; ` +
            `the bar ${stayed ? 'stayed on' : 'MOVED TO'} ${JSON.stringify(pane())}, ` +
            `Rust ${stayed ? 'kept' : 'MOVED TO'} ${JSON.stringify(now.edited)}; it cleared: ${cleared}`
      )

      ok(
        '9d',
        'no error reached the console since this window was armed',
        armed && noise === 0,
        armed
          ? `${noise} uncaught, ${loops} of them ResizeObserver${spoken.length ? ' — ' + spoken.join(' | ') : ''}`
          : 'THIS WINDOW WAS NEVER ARMED — run __gate.arm() first, or this clause passes by counting nothing'
      )

      tell('run __gate.report() for the whole run as one entry.')
      return 'clause 9c recorded'
    },

    /** The whole run, as one console entry Safari's Inspector can copy. */
    report() {
      const lines = [...transcript, '', `clause 9: ${pass} passed, ${fail} failed`]
      console.log(lines.join('\n'))
      return { passed: pass, failed: fail }
    }
  }

  /* The two baselines the clauses compare against. `arm()` fills them in; these
     exist so a call made out of order reads `null` and says so rather than
     throwing. */
  window.__gate._before = null
  window.__gate._edited = null
  window.__gate._revision = null

  console.log(
    '%cmpdf-003 Phase 19 gate loaded%c — run __gate.forget() then await __gate.arm()',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
