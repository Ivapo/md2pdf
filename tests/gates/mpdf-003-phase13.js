/* mpdf-003 Phase 13 exit gate — the window half. Paste into the Web Inspector
   console of a `cargo tauri dev` window.

   The other three-quarters run without a person: `cargo test --workspace` at
   338 passed / 0 failed / 2 ignored across nine binaries, `bun
   app/harness/checks.mjs` and `--webkit` at ten clauses each, `--falsify` at
   six isolated mutations, and `bun app/typecheck.mjs`. **No Rust in this phase
   reaches the compile path**, so the PDF is byte-identical across it, and
   `--paper` unmoved in all six system-by-state combinations is where the
   harness pins that.

   WHAT ONLY A WINDOW CAN SAY, and it is three things:

     * **the native title bar follows the choice.** `window.set_theme` is the
       whole reason this phase touches `app/src/main.rs`, and nothing in a
       browser has a title bar. From the page that call would reject —
       `core:default` is the window's getters and no setter — but the command
       makes it from Rust, where capabilities do not apply.
     * **`◐ ☀ ☾` render alike in WKWebView.** They measured 9.23 / 9.23 / 9.22px
       in Playwright's Chromium and WebKit, identical to the digit, with the
       brand not moving as they swap — but Phase 12's own note records that
       neither engine is this one. `specs/desktop_app_spec.md` OQ-12 is what
       this answers, and its answer may be three small inline paths instead.
     * **the launch does not flash the other palette.** That is a claim about a
       frame, and only a relaunch can see it. **This clause failed on its first
       run, on 2026-08-29, and the phase's named fallback was taken**: reading
       the choice inside `setup` was not enough, because the runtime does not
       merely build the configured window before that hook — it puts it on
       screen, so `set_theme` arrived a frame late however early it was called.
       `tauri.conf.json` now carries `"visible": false` and `setup` calls
       `show()` after `set_theme`. **So a failure here now means something else**
       — that the store was not read, or that `show` did not run — rather than
       an ordering nobody had tested.

   ONE PRECONDITION, and it is the third clause's alone: **the system must be
   set to Light** in System Settings → Appearance. Clause 3 stores `dark` and
   relaunches, and a dark system would make the two agree — a flash that could
   not happen is not a flash that did not. Clauses 1 and 2 do not care.

   NO SETUP STEP, and no document need be opened: every clause here is about the
   window's own chrome. **This gate writes one file**, `settings.json` in the
   app's own Application Support directory, which is what the feature is; it
   writes nothing into the repository and `git status` is clean before and
   after. **It leaves the appearance where it found it** — `restore()` is the
   last call in the order below, and `report()` reminds you.

   ORDER:
     __gate.forget()           <- once, at the start: clears any earlier transcript
     await __gate.arm()        <- installs the listeners, reads the state
     await __gate.chrome()     <- clauses 1, 2  (title bar, and the three marks)
     await __gate.flash()      <- stores dark, then tells you to quit
     ... quit Letur (Cmd-Q), then `cargo tauri dev` again, then RE-PASTE this ...
     await __gate.arm()        <- again, in the NEW window: it has no listeners yet
     await __gate.landed(true|false) <- clauses 3, 4; your eyes on the first frame
     await __gate.restore()    <- puts the appearance back
     __gate.report()           <- the whole run, both windows

   **The transcript is kept in `localStorage` and not in a closure**, because
   clause 3 needs a relaunch and a closure does not survive one. That is a defect
   this gate had on its first run, on 2026-08-29: it reported the launch half
   alone and the two clauses before the quit were gone with the window. `forget()`
   is the only thing that clears it, so re-pasting never loses a run — and
   `arm()` is called in **both** windows, because its error listeners live in the
   page too and clause 4 in a window that was never armed passes by counting
   nothing.

   **Clause 1 and clause 3 are judged by eye, and they say so rather than
   pretending otherwise.** A title bar's own rendering is not readable from the
   content area, and a flash is a frame. What the gate does instead is remove
   every other reason for a wrong answer: it drives the real command through the
   real bridge, it probes `getCurrentWindow().theme()` by *calling* it rather
   than by `typeof` — which is the mistake that cost Phase 11's gate two runs —
   and it reports what Rust says beside what you say, so a disagreement between
   the two is itself visible.

   Paste this into the build before this phase and the banner never prints: the
   lookups below run at paste time and `#theme` is not in that page at all,
   which is the first thing this phase adds.                                   */

;(() => {
  const themeButton = document.getElementById('theme')
  const brand = document.getElementById('brand')
  const footer = document.querySelector('footer')
  const { invoke } = window['__TAURI__'].core
  const { getCurrentWindow } = window['__TAURI__'].window

  const APPEARANCES = ['system', 'light', 'dark']
  const MARK = { system: '◐', light: '☀', dark: '☾' }

  let noise = 0
  let loops = 0
  let armed = false
  const spoken = []
  const wait = (ms) => new Promise((r) => setTimeout(r, ms))

  /* No compile happens here — `set_appearance` announces without compiling, and
     the page's `refresh` guards on the revision — so this is the IPC round trip
     and a paint, not `watch.rs:DEBOUNCE` plus a compile. */
  const settled = () => wait(400)

  let pass = 0
  let fail = 0
  /* Every line is kept as well as logged: Safari's Web Inspector copies one
     console entry at a time, and `__gate.report()` gives the whole run back as
     one entry.

     **It is kept in `localStorage`, which is what survives the relaunch clause 3
     needs.** The origin is `tauri://localhost` and does not move across a quit.
     Both accessors are guarded: a gate that threw here would take the run with
     it, and the run is the thing being reported. */
  const KEEP = 'mpdf-003-phase13-gate'
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
    transcript.push(
      `${good ? 'PASS' : 'FAIL'}  ${String(n).padStart(2)}. ${name}${detail ? '  —  ' + detail : ''}`
    )
    keep()
    console.log(
      `%c${good ? 'PASS' : 'FAIL'}%c  ${String(n).padStart(2)}. ${name}${detail ? '  —  ' + detail : ''}`,
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
  const ask = (s) => {
    transcript.push(`????  ${s}`)
    keep()
    console.log(`%c????%c  ${s}`, 'font-weight:bold;color:#1a73e8', 'color:inherit')
  }
  const tally = (what) => {
    transcript.push(`${what}: ${pass} passed, ${fail} failed`)
    keep()
    console.log(
      `%c${what}: ${pass} passed, ${fail} failed`,
      `font-weight:bold;color:${fail ? '#c5221f' : '#137333'}`
    )
    const answer = { passed: pass, failed: fail }
    pass = 0
    fail = 0
    return answer
  }

  /* **`getBoundingClientRect()` and not `offsetWidth`**, for the reason
     `tests/gates/mpdf-003-phase11.js` states at length: a 10px glyph is a
     fraction of a pixel wide and `offsetWidth` rounds it, which is exactly the
     difference this clause is looking for. */
  const wide = (el) => el.getBoundingClientRect().width
  const left = (el) => el.getBoundingClientRect().left

  /** What Rust says the window is wearing, if it will say. */
  const asked = async () => {
    try {
      return { held: await invoke('status').then((s) => s.appearance), how: 'status' }
    } catch (problem) {
      return { held: null, how: `status refused: ${problem}` }
    }
  }

  /** What the window itself says its theme is — **probed by calling it**.
      `core:window:default` is the getters, and whether `theme` is among them is
      a question to settle by asking the IPC, never by `typeof`. */
  const windowTheme = async () => {
    try {
      return String(await getCurrentWindow().theme())
    } catch (problem) {
      return `refused: ${problem}`
    }
  }

  /** Ask Rust for an appearance and let it come back round through `rendered`. */
  const wear = async (appearance) => {
    await invoke('set_appearance', { appearance })
    await settled()
  }

  /** What this session found the appearance at, so it can be put back. */
  let found = null

  window.__gate = {
    /** The whole run, both windows, and its own tally derived from the lines
        rather than from a counter the relaunch reset. */
    report() {
      const passed = transcript.filter((l) => l.startsWith('PASS')).length
      const failed = transcript.filter((l) => l.startsWith('FAIL')).length
      console.log(
        `mpdf-003 Phase 13 gate\n${transcript.join('\n')}\n\n` +
          `WHOLE RUN: ${passed} passed, ${failed} failed`
      )
      return `${passed} passed, ${failed} failed — run __gate.restore() if you have not`
    },

    /** Clear the kept transcript. The only thing that does, so a re-paste after
        the relaunch adds to the run rather than starting a new one. */
    forget() {
      transcript.length = 0
      keep()
      console.log(
        '%cforgotten%c  — now run: await __gate.arm()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return 'cleared'
    },

    async arm() {
      noise = 0
      loops = 0
      spoken.length = 0
      pass = 0
      fail = 0
      armed = true

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

      const state = await asked()
      found = state.held
      note(`the window came up wearing ${JSON.stringify(found)} (${state.how})`)
      note(`the window's own theme reads ${await windowTheme()}`)
      note(`the mark on the bar is ${JSON.stringify(themeButton.textContent)}`)

      console.log(
        '%carmed%c  — now run: await __gate.chrome()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return found
    },

    /* Clauses 1 and 2. One pass over the three states, reading everything each
       of them can be read for, so the title bar and the marks are one walk of
       the cycle rather than two. */
    async chrome() {
      heading('the three states — the title bar, and the marks')

      const seen = []
      for (const appearance of APPEARANCES) {
        await wear(appearance)
        seen.push({
          appearance,
          mark: themeButton.textContent,
          title: themeButton.title,
          label: themeButton.getAttribute('aria-label'),
          attribute: document.documentElement.getAttribute('data-theme'),
          width: wide(themeButton),
          brand: left(brand),
          theme: await windowTheme(),
          said: (await asked()).held
        })
        note(
          `${appearance}: mark ${JSON.stringify(seen[seen.length - 1].mark)} ` +
            `at ${seen[seen.length - 1].width.toFixed(2)}px, ` +
            `attribute ${seen[seen.length - 1].attribute}, ` +
            `window theme ${seen[seen.length - 1].theme}, ` +
            `status says ${seen[seen.length - 1].said}`
        )
        ask(`LOOK AT THE TITLE BAR NOW — is it ${appearance === 'system' ? 'following the system' : appearance}?`)
        await wait(1500)
      }

      /* **What Rust reports is a precondition of the eye clause, not the clause
         itself.** If the value did not even move, the title bar was never asked
         to; saying so here keeps a "no" from being read as `set_theme` failing
         when it was `set_appearance` that did. */
      const moved = seen.every((s) => s.said === s.appearance)
      const placed = seen.every(
        (s) =>
          s.mark === MARK[s.appearance] &&
          s.attribute === (s.appearance === 'system' ? null : s.appearance) &&
          s.title === s.label &&
          /appearance/i.test(s.label || '')
      )

      ok(
        1,
        'the choice reached Rust and the window in all three states — YOUR EYES DECIDE the title bar',
        moved && placed,
        `values ${moved ? 'moved' : 'DID NOT MOVE'}, page ${placed ? 'placed' : 'MISPLACED'}; ` +
          `window theme read ${seen.map((s) => `${s.appearance}→${s.theme}`).join(', ')}. ` +
          'If the title bar did NOT follow above, mark this FAIL in your notes and say which state.'
      )

      /* Clause 2. The three widths against each other, and against the brand.
         **No literal**: Playwright's two engines both said 9.23 / 9.23 / 9.22,
         but that is a reading from elsewhere and this clause is about here, so
         what it asserts is that the three agree with one another and that the
         bar does not move as they swap. */
      const widths = seen.map((s) => s.width)
      const spread = Math.max(...widths) - Math.min(...widths)
      const brands = new Set(seen.map((s) => s.brand.toFixed(2)))
      const drawn = widths.every((w) => w > 4 && w < 20)

      ok(
        2,
        'the three marks render alike in WKWebView, and the brand does not move as they swap',
        spread < 1 && brands.size === 1 && drawn,
        `widths ${widths.map((w) => w.toFixed(2)).join(' / ')} — spread ${spread.toFixed(2)}px; ` +
          `brand at ${[...brands].join(', ')}. ` +
          'ALSO LOOK: are all three legible at this size, and does each read as what it means? ' +
          'If any is a tofu box, a colour emoji or unreadable, that is OQ-12 answered "inline SVG".'
      )

      note(`${noise} uncaught so far, ${loops} of them ResizeObserver`)
      console.log(
        '%cnow run%c  __gate.flash()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return tally('chrome')
    },

    /* Clause 3, first half: leave the window in the state the relaunch has to
       start from, and say what to do. */
    async flash() {
      heading('the launch — what to do next')
      await wear('dark')
      note(`the appearance is now ${(await asked()).held}, and settings.json holds it`)
      console.log(
        '%cNOW:%c\n' +
          '  1. Check System Settings → Appearance is set to LIGHT. If it is not, this\n' +
          '     clause proves nothing — a dark system and a stored dark cannot disagree.\n' +
          '  2. Quit Letur entirely (⌘Q). Not just close the window.\n' +
          '  3. Run `cargo tauri dev` again and WATCH THE WINDOW AS IT APPEARS.\n' +
          '     The question is whether it comes up dark, or comes up light and\n' +
          '     turns dark a frame or two later.\n' +
          '  4. Re-paste this gate into the new window, then:\n' +
          '       await __gate.arm()        (the new window has no listeners yet)\n' +
          '       await __gate.landed(true)   if it came up dark with no flash of light\n' +
          '       await __gate.landed(false)  if you saw the light palette first\n' +
          '     The transcript from THIS window survives the quit and report() has it all.',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return 'quit and relaunch, then __gate.landed(true|false)'
    },

    /* Clause 3, second half. Run in the RELAUNCHED window. */
    async landed(clean) {
      heading('the launch — the first frame')

      const state = await asked()
      const attribute = document.documentElement.getAttribute('data-theme')

      note(`this window came up wearing ${JSON.stringify(state.held)}, attribute ${attribute}`)
      note(`the window's own theme reads ${await windowTheme()}`)

      /* Remembering it is the precondition; not flashing is the clause. A
         window that came up `system` never had a stored value to apply, and a
         "no flash" from it would be worth nothing. */
      const remembered = state.held === 'dark' && attribute === 'dark'

      ok(
        3,
        'a stored dark is worn from the first frame, with no flash of the other palette',
        remembered && clean === true,
        remembered
          ? clean === true
            ? 'remembered across the quit, and worn from the first visible frame'
            : 'remembered, but YOU STILL SAW A FLASH — the "visible": false fallback is ' +
              'already in tauri.conf.json and setup already shows the window after set_theme, ' +
              'so this is a NEW finding rather than the one that fallback answers'
          : `NOT REMEMBERED: came up ${JSON.stringify(state.held)} — settings.json was not read, ` +
            'so the flash question was never asked'
      )

      /* **Since this window was armed**, which is narrower than "through any of
         it" and is said so rather than implied: the listeners live in the page,
         so they cannot see a throw that happened before the paste. Errors at
         launch are in the console above regardless. */
      ok(
        4,
        'no error reached the console since this window was armed',
        armed && noise === 0,
        armed
          ? `${noise} uncaught, ${loops} of them ResizeObserver` +
            `${spoken.length ? ' — ' + spoken.join(' | ') : ''}`
          : 'THIS WINDOW WAS NEVER ARMED — run await __gate.arm() first, or this clause ' +
            'passes by counting nothing, which is what it did on the run of 2026-08-29'
      )

      note('run __gate.restore() to put the appearance back, then __gate.report().')
      return tally('launch')
    },

    /** Put the appearance back where `arm()` found it. */
    async restore() {
      const back = found ?? 'system'
      await wear(back)
      note(`the appearance is back at ${JSON.stringify((await asked()).held)}`)
      return back
    }
  }

  console.log(
    `%c__gate ready%c  —  set System Settings → Appearance to LIGHT first (clause 3 needs it).\n` +
      `${transcript.length} lines already kept${transcript.length ? ' — __gate.forget() to start over' : ''}.\n` +
      'FIRST WINDOW:  __gate.forget() → await __gate.arm() → await __gate.chrome() →\n' +
      '               await __gate.flash()\n' +
      'THEN quit (⌘Q), `cargo tauri dev` again, re-paste, and in the NEW WINDOW:\n' +
      '               await __gate.arm() → await __gate.landed(true|false) →\n' +
      '               await __gate.restore() → __gate.report()',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
