/* mpdf-003 Phase 20 exit gate — clause 7, the reading left to a person. Paste
   into the Web Inspector console of a `cargo tauri dev` window.

   **Everything else in this phase's gate runs without a person** and is recorded
   here so a reader knows what this file is *not* covering: `bun app/typecheck.mjs`,
   `cargo test --workspace` unchanged, `bun app/harness/checks.mjs` and `--webkit`
   at fifteen clauses each, `--falsify` at twelve isolated mutations in both
   engines, and `bun app/driver/drive.mjs` and `--falsify` unchanged. What is left
   is **one gesture and one keystroke**, and the phase asks for them here rather
   than in a driver because a synthesised drag is not a hand on a trackpad: the
   harness clause moves the pointer in five programmatic steps, and what the author
   reported was a hand.

   **So the person performs the gesture and this judges the reading**, which is the
   difference between a gate and a list of things to look at. Nothing below is
   answered by eye — not even the ring, which is read off `getComputedStyle` with
   the pane's focus checked first, because a ring reading taken on an unfocused
   textarea passes without asserting anything at all.

   **The drag is one continuous gesture, left and then back right**, which is how
   it was reported and is also the shape that carries both defects: leftward the
   press cost the pane its focus, rightward it moved the caret. The keystroke is
   the second call because a swallowed keystroke is the damage the focus loss
   actually does — a reading of `activeElement` alone would be the mechanism, and
   this asks for the consequence.

   **The ring is read twice, and `report()` fails if it was not.** Its colour is
   `--ink`; a reader who checks only light has checked the easier one. Switch
   appearance from the footer bar between the two calls — this file does not press
   that control itself, because the control is not what is under test.

   **The flex basis is asserted to have *changed*, not to hold a value.** A drag
   that goes left and comes back lands near where it started, so a before/after
   width would be a coin toss; the inline `flexBasis` the handler writes is unset
   until the first drag and is a subpixel string after, which a hand cannot land on
   twice. That is what stops this clause being passed by a divider that does
   nothing.

   **Every clause is guarded against the hand doing something else, and this
   file's own first run is why.** That run failed 7a with the focus on `#pages`
   — which carries `tabindex="0"` so a keyboard reader can scroll the preview —
   and the drag turned out to be innocent. A probe of the real window recorded
   `pointerdown` on `#divider` marked `[defaultPrevented]`, **no compat
   `mousedown` at all**, and **no focus event of any kind**, across two gestures
   including one run hard into the `room - 160` clamp with the pointer ending
   95px past the divider and over `#pages`. The focus had been moved by a stray
   press between two console calls. So: `arm()` records every press from that
   moment on, and a reading taken across a press on anything but the divider is
   reported **invalid** rather than blamed on the gesture. A gate asserts only
   what it can attribute.

   NO PRECONDITION beyond a document being open. **It writes no file** — the one
   keystroke dirties the buffer in the pane, exactly as typing does, and nothing
   here presses `⌘S`. Undo it with `⌘Z` when you are done, or leave it; the file on
   disk is untouched either way.

   **Hand the keyboard back by pressing the DIVIDER once, not the title bar and
   not the pane.** Keys typed while the Web Inspector holds focus never reach the
   page at all, so the web content has to be made first responder somehow. A
   click in the pane does it and moves the caret, which is the press that
   invalidates 7b. **The title bar does not do it** — it focuses the window and
   leaves the keyboard where it was, measured, and two readings of this clause
   were wasted on it. A press on the divider does: it is the one press whose
   default action this phase cancels, so it reaches the view without taking the
   focus off the pane. The drag itself is fine to start cold, for the same reason.

   **A key that never reached the page is reported as the rig and not the
   defect.** 7b records what the page saw, so a keystroke the Inspector kept and
   a keystroke the pane dropped are told apart instead of both reading as a
   buffer that did not grow.

   ORDER:
     __gate.forget()          <- once, at the start: clears any earlier transcript
     await __gate.arm()       <- focuses the pane, sets a caret, reads the baseline
     ... drag the divider LEFT past the middle and back RIGHT, one gesture ...
     __gate.dragged()         <- clause 7a
     ... press the DIVIDER once, then type ONE character ...
     __gate.typed()           <- clause 7b
     __gate.ring()            <- clause 7c, in the appearance you are in
     ... switch appearance from the footer bar ...
     __gate.ring()            <- clause 7c again, in the other
     __gate.report()          <- clauses 7d and 7e, and the whole run as one entry

   Paste this into the build before this phase and it does not refuse — it fails.
   That is deliberate: neither half of this phase adds an element, so there is
   nothing for `arm()` to find missing. `dragged()` reports the focus on `body`,
   `typed()` reports a swallowed keystroke, and `ring()` reports `auto`.        */

;(() => {
  const text = document.getElementById('text')
  const divider = document.getElementById('divider')

  let pass = 0
  let fail = 0
  let armed = false
  let noise = 0
  let loops = 0
  const spoken = []

  /* **The transcript is kept in `localStorage`**, `tests/gates/mpdf-003-phase13.js`'s
     answer to Safari's Web Inspector copying one entry at a time. */
  const KEEP = 'mpdf-003-phase20-gate'
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
  const tell = (s) => {
    transcript.push(`>>>>  ${s}`)
    keep()
    console.log(`%c>>>>%c  ${s}`, 'font-weight:bold;color:#1a73e8', 'color:inherit')
  }

  /* ---------------------------------------------------------- what is read */

  /* The ground rather than the setting. `system` is one button position and two
     appearances, so what decides whether both were seen is the colour actually
     painted, not the value the footer holds. */
  const ground = () => getComputedStyle(document.body).backgroundColor

  const paneState = () => ({
    holding: document.activeElement === text ? 'text' : document.activeElement?.id || document.activeElement?.tagName?.toLowerCase() || 'nothing',
    selection: String(document.getSelection()),
    from: text.selectionStart,
    to: text.selectionEnd,
    basis: text.style.flexBasis || 'unset',
    length: text.value.length
  })

  const seenGrounds = []

  /* **What pressed what, and when the focus moved.** Without these a reading of
     `activeElement` after the fact cannot say whether the drag moved it or a
     stray click did, and the clause would report the second as the first. */
  const presses = []
  const focusMoves = []
  const keys = []
  let watching = false

  const named = (n) =>
    !n ? 'nothing' : n === document.body ? 'body' : n.id ? '#' + n.id : n.nodeName ? n.nodeName.toLowerCase() : String(n)

  const watch = () => {
    if (watching) return
    watching = true
    window.addEventListener('pointerdown', (e) => presses.push(named(e.target)), true)
    window.addEventListener('focusin', (e) => focusMoves.push(`${named(e.relatedTarget)} → ${named(e.target)}`))
    /* **Whether the key reached the page at all**, which is a different question
       from where it landed. Without this, a key the Web Inspector kept and a key
       the pane dropped read identically — as a buffer that did not grow. */
    window.addEventListener('keydown', (e) => keys.push({ where: named(e.target), key: e.key }), true)
  }

  /** The presses since the given mark, and a sentence naming the stray ones. */
  const strayFrom = (mark) => presses.slice(mark).filter((where) => where !== '#divider')

  /* ---------------------------------------------------------------- the run */

  window.__gate = {
    /** Clears the kept transcript. Run once, before `arm()`. */
    forget() {
      transcript.length = 0
      pass = 0
      fail = 0
      seenGrounds.length = 0
      keep()
      console.log('%cforgotten%c — run await __gate.arm() next', 'font-weight:bold;color:#1a73e8', 'color:inherit')
      return 'cleared'
    },

    /** Focuses the pane, puts a caret mid-document, and reads the baseline. */
    async arm() {
      if (!text || !divider) return 'NO PANE — #text or #divider is not in this page'

      window.addEventListener('error', (e) => {
        const said = String(e.message ?? '')
        said.includes('ResizeObserver') ? loops++ : spoken.push(said)
        noise++
      })
      window.addEventListener('unhandledrejection', (e) => {
        noise++
        spoken.push(String(e.reason))
      })

      watch()
      presses.length = 0
      focusMoves.length = 0
      keys.length = 0

      text.focus()
      const caret = Math.floor(text.value.length / 2)
      text.setSelectionRange(caret, caret)
      /* One frame, so the caret the engine paints is the one just set. */
      await new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r)))

      const before = paneState()
      this._before = before
      this._mark = presses.length
      armed = before.holding === 'text'

      note(`armed: focus ${before.holding}, caret ${before.from}, basis ${before.basis}, ${before.length} characters`)
      if (!armed) {
        tell('THE PANE DID NOT TAKE FOCUS — click in it once and run await __gate.arm() again.')
        return 'not armed'
      }
      tell('now drag the divider LEFT past the middle and back RIGHT, one gesture, then run __gate.dragged()')
      return 'armed'
    },

    /** Clause 7a — what the gesture left behind. */
    dragged() {
      const was = this._before
      if (!was) return 'NOT ARMED — run await __gate.arm() first'

      const now = paneState()
      const stray = strayFrom(this._mark)
      const dragged = presses.slice(this._mark).includes('#divider')

      if (stray.length || !dragged) {
        note(`presses since arming: ${presses.slice(this._mark).join(', ') || 'none'}`)
        note(`focus moves since arming: ${focusMoves.join(', ') || 'none'}`)
        ok(
          '7a',
          'a hand drag of the divider resizes the pane, highlights nothing, keeps the focus and leaves the caret where it was',
          false,
          stray.length
            ? `INVALID READING — you also pressed on ${[...new Set(stray)].join(' and ')} after arming, and this clause cannot tell that press's effects from the drag's. Run await __gate.arm() again and touch nothing but the divider.`
            : 'INVALID READING — no press on #divider was recorded, so there was no drag to judge. Run await __gate.arm() again and drag the divider.'
        )
        tell('run await __gate.arm() and try again, touching nothing but the divider.')
        return 'clause 7a invalid — re-arm'
      }

      this._after = now
      this._markAfter = presses.length
      this._keyMark = keys.length

      const resized = now.basis !== was.basis
      const kept = now.holding === 'text'
      const still = now.from === was.from && now.to === was.from
      const quiet = now.selection === ''

      note(`after the drag: focus ${now.holding}, caret ${was.from} → ${now.from}${now.to === now.from ? '' : '–' + now.to}, basis ${was.basis} → ${now.basis}, selection ${JSON.stringify(now.selection)}`)

      ok(
        '7a',
        'a hand drag of the divider resizes the pane, highlights nothing, keeps the focus and leaves the caret where it was',
        resized && kept && still && quiet,
        [
          resized ? null : `THE PANE DID NOT RESIZE — basis is still ${now.basis}; did the gesture reach the divider?`,
          kept ? null : `the focus left the pane for ${now.holding}`,
          still ? null : `the caret moved from ${was.from} to ${now.from}${now.to === now.from ? '' : '–' + now.to}`,
          quiet ? null : `it selected ${JSON.stringify(now.selection)}`,
          kept && still && quiet ? null : `focus moves recorded: ${focusMoves.join(', ') || 'none'}`
        ]
          .filter(Boolean)
          .join('; ') || `resized ${was.basis} → ${now.basis}, focus kept, caret still ${now.from}, nothing selected`
      )

      tell('now press the DIVIDER once — not the pane — then type ONE character, then run __gate.typed()')
      return 'clause 7a recorded'
    },

    /** Clause 7b — the keystroke the focus loss swallows. */
    typed() {
      const was = this._after
      if (!was) return 'NO DRAG READING — run __gate.dragged() first'

      /* A press on the divider is allowed here and is in fact the recommended
         way to hand the keyboard back: it is the one press whose default this
         phase cancels, so it makes the web content first responder without
         taking focus off the pane. Any other press moves the caret itself. */
      const stray = strayFrom(this._markAfter)
      if (stray.length) {
        ok(
          '7b',
          'the keystroke after the drag goes into the pane, at the caret',
          false,
          `INVALID READING — you pressed on ${[...new Set(stray)].join(' and ')} after the drag, which moves the caret itself. The swallowed keystroke this clause is about only shows on a pane nobody clicked back into. Run await __gate.arm() again.`
        )
        tell('run await __gate.arm() again, drag, then press the DIVIDER once and type.')
        return 'clause 7b invalid — re-arm'
      }

      const now = paneState()
      const arrived = keys.slice(this._keyMark)

      note(`after the keystroke: ${was.length} → ${now.length} characters, caret ${was.from} → ${now.from}; keys the page saw: ${arrived.map((k) => `${k.key} on ${k.where}`).join(', ') || 'none'}`)

      /* **No key reached the page at all is the rig, not the defect**, and the
         two are told apart rather than guessed at. The Web Inspector holds the
         keyboard until the web content is made first responder, and clicking the
         window's title bar does not do that — measured, and it is what made this
         clause's first two readings worthless. The pane keeping its focus
         throughout is what licenses calling it the rig: a swallowed keystroke is
         a keystroke the page *saw* and the pane did not get. */
      if (arrived.length === 0) {
        ok(
          '7b',
          'the keystroke after the drag goes into the pane, at the caret',
          false,
          `NO KEY REACHED THE PAGE — the pane still holds the focus${focusMoves.length ? '' : ' and never lost it'}, so this is the Web Inspector keeping the keyboard rather than a swallowed keystroke. Press the DIVIDER once — the one press whose default this phase cancels — then type, and run __gate.typed() again.`
        )
        tell('press the DIVIDER once, type ONE character, then run __gate.typed() again.')
        return 'clause 7b inconclusive — the key never arrived'
      }

      const landed = now.length === was.length + 1
      const atCaret = now.from === was.from + 1

      ok(
        '7b',
        'the keystroke after the drag goes into the pane, at the caret',
        landed && atCaret,
        landed
          ? atCaret
            ? `one character, at ${now.from}`
            : `the character landed, but at ${now.from} rather than ${was.from + 1}`
          : now.length === was.length
            ? `THE KEYSTROKE WAS SWALLOWED — the page saw ${arrived.map((k) => `${k.key} on ${k.where}`).join(', ')} and the buffer is the length it was; this is the defect`
            : `the buffer moved by ${now.length - was.length} characters, not one — the page saw ${arrived.length} keys`
      )

      tell('now run __gate.ring(), then switch appearance in the footer bar and run it again')
      return 'clause 7b recorded'
    },

    /** Clause 7c — the ring, in whichever appearance the window is wearing. */
    ring() {
      text.focus()
      const focused = document.activeElement === text
      const worn = ground()
      const style = getComputedStyle(text)
      seenGrounds.push(worn)

      note(`ground ${worn}: outline-style ${style.outlineStyle}, outline-width ${style.outlineWidth}`)

      ok(
        `7c on ${worn}`,
        'no ring is drawn around the focused text pane',
        focused && style.outlineStyle === 'none',
        focused
          ? style.outlineStyle === 'none'
            ? 'outline-style none, with the pane focused'
            : `outline-style ${style.outlineStyle} at ${style.outlineWidth} — the ring is still there`
          : 'THE PANE IS NOT FOCUSED — this reading asserts nothing; click in the pane and run __gate.ring() again'
      )

      tell(
        seenGrounds.length < 2
          ? 'now switch appearance in the footer bar and run __gate.ring() again'
          : 'run __gate.report() for the whole run as one entry.'
      )
      return 'clause 7c recorded'
    },

    /** Clauses 7d and 7e, and the whole run as one entry Safari can copy. */
    report() {
      const grounds = [...new Set(seenGrounds)]

      ok(
        '7d',
        'the ring was read in both appearances and not just the easier one',
        grounds.length >= 2,
        grounds.length >= 2
          ? `two grounds read: ${grounds.join(' and ')}`
          : `only ${grounds.length === 1 ? 'one ground, ' + grounds[0] : 'no ground'} — switch appearance in the footer bar and run __gate.ring() again`
      )

      ok(
        '7e',
        'no error reached the console since this window was armed',
        armed && noise === 0,
        armed
          ? `${noise} uncaught, ${loops} of them ResizeObserver${spoken.length ? ' — ' + spoken.join(' | ') : ''}`
          : 'THIS WINDOW WAS NEVER ARMED — run await __gate.arm() first, or this clause passes by counting nothing'
      )

      const lines = [...transcript, '', `clause 7: ${pass} passed, ${fail} failed`]
      console.log(lines.join('\n'))
      return { passed: pass, failed: fail }
    }
  }

  /* The two baselines the clauses compare against. `arm()` fills the first in and
     `dragged()` the second; these exist so a call made out of order reads `null`
     and says so rather than throwing. */
  window.__gate._before = null
  window.__gate._after = null
  window.__gate._mark = 0
  window.__gate._markAfter = 0
  window.__gate._keyMark = 0

  console.log(
    '%cmpdf-003 Phase 20 gate loaded%c — run __gate.forget() then await __gate.arm()',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
