/* mpdf-003 Phase 13 exit gate — the window half, as Phase 14 left it. Paste into
   the Web Inspector console of a `cargo tauri dev` window.

   **Phase 14 took two of this gate's four clauses away, and this file is where
   the two numberings are reconciled.** The spec's table names four; three are
   left here and they are renumbered 1, 2, 3, because a gate that prints a gap
   reads like something was lost. The map, once:

     old 1, the plumbing and the placement  ->  `bun app/driver/drive.mjs` clause 1
     old 1, the title bar                   ->  clause 1 HERE, with a boolean of its own
     old 2, the marks and the brand         ->  `bun app/driver/drive.mjs` clause 2
     old 3, the launch                      ->  clause 2 HERE
     old 4, the errors since arming         ->  clause 3 HERE

   Old clause 1 read `moved && placed`, which is *entirely* the half that moved,
   so left alone it would have asserted nothing the driver had not already. What
   is left of it is the eye's half, and it now has an answer of its own to be
   recorded by.

   The rest runs without a person: `cargo test --workspace` at 338 passed / 0
   failed / 2 ignored across nine binaries, `bun app/driver/drive.mjs` and
   `--falsify` at three clauses and two isolated mutations in the real window,
   `bun app/harness/checks.mjs` and `--webkit` at ten clauses each, `--falsify` at
   six isolated mutations, and `bun app/typecheck.mjs`. **No Rust in this phase
   reaches the compile path**, so the PDF is byte-identical across it, and
   `--paper` unmoved in all six system-by-state combinations is where the harness
   pins that.

   WHAT ONLY A WINDOW CAN SAY, and after Phase 14 it is two things:

     * **the native title bar follows the choice.** `window.set_theme` is the
       whole reason this phase touches `app/src/main.rs`, and nothing in a
       browser has a title bar. From the page that call would reject —
       `core:default` is the window's getters and no setter — but the command
       makes it from Rust, where capabilities do not apply. **W3C `Take
       Screenshot` does not reach it either**: that is the viewport, not the OS
       window, which is why the driver did not take this clause with the other
       half of old clause 1.
     * **the launch does not flash the other palette.** That is a claim about a
       frame, and only a relaunch can see it — the driver's session begins after
       the window is already up. **This clause failed on its first run, on
       2026-08-29, and the phase's named fallback was taken**: reading the choice
       inside `setup` was not enough, because the runtime does not merely build
       the configured window before that hook — it puts it on screen, so
       `set_theme` arrived a frame late however early it was called.
       `tauri.conf.json` now carries `"visible": false` and `setup` calls
       `show()` after `set_theme`. **So a failure here now means something else**
       — that the store was not read, or that `show` did not run — rather than
       an ordering nobody had tested.

   **`☀ ☾` rendering alike in WKWebView is no longer asked here**, having moved
   whole to the driver, which measures them at a width it sets. What has no
   machine form and stays is the *legibility* prompt: whether either is a tofu
   box is a thing only an eye answers, and `chrome()` below asks it as a note
   rather than a clause.

   ONE PRECONDITION, and it is the launch clause's alone: **the system must be
   set to Light** in System Settings → Appearance. Clause 2 stores `dark` and
   relaunches, and a dark system would make the two agree — a flash that could
   not happen is not a flash that did not. Clause 1 does not care.

   NO SETUP STEP, and no document need be opened: every clause here is about the
   window's own chrome. **This gate writes one file**, `settings.json` in the
   app's own Application Support directory, which is what the feature is; it
   writes nothing into the repository and `git status` is clean before and
   after. **It leaves the appearance where it found it** — `restore()` is the
   last call in the order below, and `report()` reminds you.

   ORDER:
     __gate.forget()          <- once, at the start: clears any earlier transcript
     await __gate.arm()       <- installs the listeners, reads the state
     await __gate.chrome()    <- walks the three states so you can watch the title bar
     __gate.titleFollowed()   <- clause 1 — or __gate.titleDidNot('<state>') if it did not
     await __gate.flash()     <- stores dark, then tells you to quit
     ... quit Letur (Cmd-Q), then `cargo tauri dev` again, then RE-PASTE this ...
     await __gate.arm()       <- again, in the NEW window: it has no listeners yet
     await __gate.noFlash()   <- clauses 2, 3 — or __gate.flashed() if it did
     await __gate.restore()   <- puts the appearance back
     __gate.report()          <- the whole run, both windows

   **The transcript is kept in `localStorage` and not in a closure**, because
   clause 2 needs a relaunch and a closure does not survive one. That is a defect
   this gate had on its first run, on 2026-08-29: it reported the launch half
   alone and the two clauses before the quit were gone with the window. `forget()`
   is the only thing that clears it, so re-pasting never loses a run — and
   `arm()` is called in **both** windows, because its error listeners live in the
   page too and clause 3 in a window that was never armed passes by counting
   nothing.

   **Every clause left here is judged by eye, and they say so rather than
   pretending otherwise.** A title bar's own rendering is not readable from the
   content area, and a flash is a frame. What the gate does instead is remove
   every other reason for a wrong answer: it drives the real command through the
   real bridge, it probes `getCurrentWindow().theme()` by *calling* it rather
   than by `typeof` — which is the mistake that cost Phase 11's gate two runs —
   and it reports what Rust says beside what you say, so a disagreement between
   the two is itself visible. **What Rust says is a note here and not a clause**,
   because the driver asserts it; a second assertion of it would be two gates
   claiming one reading.

   Paste this into the build before this phase and the banner never prints: the
   lookups below run at paste time and `#theme` is not in that page at all,
   which is the first thing this phase adds.                                   */

;(() => {
  const themeButton = document.getElementById('theme')
  const { invoke } = window['__TAURI__'].core
  const { getCurrentWindow } = window['__TAURI__'].window

  const APPEARANCES = ['system', 'light', 'dark']
  const MARK = { light: '☀', dark: '☾' }
  /* What the *page* should be showing for a given value, which for the unset
     state is whatever the machine is giving. Read the same way the page reads
     it, so a disagreement is the page's and not this gate's arithmetic.

     **It must be sampled beside the reading it judges and never after the
     walk.** `set_theme` moves the app-wide appearance, so `prefers-color-scheme`
     answers differently at each step: a `system` reading taken while the system
     was light, judged later against a media query the walk has since put in
     dark, is wrong about correct code. That is what it did on the run of
     2026-08-29 — the page had `☀`, `null` and a light window theme, all three
     right, and this said MISPLACED. It is a note now rather than a clause, and
     the hazard is the same either way: a note nobody can trust is worse than no
     note. */
  const inEffect = (a) =>
    a === 'system' ? (matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light') : a

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

     **It is kept in `localStorage`, which is what survives the relaunch clause 2
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

  /** What the FIRST window found the appearance at, so it can be put back.
      **Kept beside the transcript**, because `restore()` runs after the relaunch
      and the relaunched window finds the value `flash()` deliberately stored —
      which on the run of 2026-08-29 put the appearance back at `dark` rather
      than where the run started. */
  const FOUND = 'mpdf-003-phase13-found'
  let found = (() => {
    try {
      return localStorage.getItem(FOUND)
    } catch {
      return null
    }
  })()

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
      found = null
      keep()
      try {
        localStorage.removeItem(FOUND)
      } catch {
        /* The in-memory reset above is the one that matters. */
      }
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
      // Only the first arming records it; the second is after `flash()` stored
      // its own value and would overwrite the thing being preserved.
      if (found === null) {
        found = state.held
        try {
          localStorage.setItem(FOUND, found)
        } catch {
          /* `restore()` then falls back to `system`, and says what it did. */
        }
      }
      note(`this window came up wearing ${JSON.stringify(state.held)} (${state.how}); the run started at ${JSON.stringify(found)}`)
      note(`the window's own theme reads ${await windowTheme()}`)
      note(`the mark on the bar is ${JSON.stringify(themeButton.textContent)}`)

      console.log(
        '%carmed%c  — now run: await __gate.chrome()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return found
    },

    /* The walk clause 1 needs, and **no clause of its own**. It drives the three
       states so there is something to look at, and reports what Rust and the page
       did beside it — but the driver asserts both of those, so here they are
       notes. What this ends with is a question, and the answer is the clause. */
    async chrome() {
      heading('the three states — watch the title bar')

      const seen = []
      for (const appearance of APPEARANCES) {
        await wear(appearance)
        seen.push({
          appearance,
          // Sampled here, with the reading, for the reason `inEffect` states.
          effective: inEffect(appearance),
          mark: themeButton.textContent,
          label: themeButton.getAttribute('aria-label'),
          attribute: document.documentElement.getAttribute('data-theme'),
          theme: await windowTheme(),
          said: (await asked()).held
        })
        const last = seen[seen.length - 1]
        note(
          `${appearance}: mark ${JSON.stringify(last.mark)}, ` +
            `attribute ${last.attribute}, window theme ${last.theme}, status says ${last.said}`
        )
        ask(
          `LOOK AT THE TITLE BAR NOW — is it ${
            appearance === 'system' ? `following the system (so, ${inEffect('system')})` : appearance
          }?`
        )
        await wait(1500)
      }

      /* **A note, because `bun app/driver/drive.mjs` clause 1 asserts exactly
         this.** It is still printed, and it still earns its place: if the value
         did not move at all, the title bar was never asked to, and a "no" above
         would be `set_appearance` failing rather than `set_theme`. */
      const moved = seen.every((s) => s.said === s.appearance)
      const misplaced = seen.filter(
        (s) => s.mark !== MARK[s.effective] || s.attribute !== (s.appearance === 'system' ? null : s.appearance)
      )
      note(
        `values ${moved ? 'moved' : 'DID NOT MOVE'}, page ${misplaced.length === 0 ? 'placed' : 'MISPLACED'} — ` +
          'the driver is what asserts this; a disagreement here means run it and read the reason'
      )
      ask(
        'ALSO LOOK at the two marks the walk just showed: are both legible at this size, and ' +
          'does each read as what it means? If either is a tofu box, a colour emoji or ' +
          'unreadable, that is OQ-12 re-opened and the answer is "inline SVG". ' +
          'The driver measures their widths; it cannot read them.'
      )

      console.log(
        '%cnow say which you saw:%c\n' +
          '    __gate.titleFollowed()        the title bar followed in all three states\n' +
          "    __gate.titleDidNot('system')  it did not — name the state it failed in",
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return 'answer with __gate.titleFollowed() or __gate.titleDidNot(state)'
    },

    /* **A boolean argument was a mistake once and is not made twice.** On
       2026-08-29 the operator reported "no flash" in words and passed `false`
       both times, so the launch clause grew two named calls; clause 1 is now
       answered the same way, and this refuses. */
    title() {
      console.log(
        '%crefused%c  — say which you saw, in words:\n' +
          '    __gate.titleFollowed()        it followed in all three states\n' +
          "    __gate.titleDidNot('light')   it did not — name the state",
        'font-weight:bold;color:#c5221f',
        'color:inherit'
      )
      return 'use __gate.titleFollowed() or __gate.titleDidNot(state)'
    },

    /** Clause 1: the title bar followed. Run after `chrome()`. */
    titleFollowed() {
      return this._title(true, null)
    },

    /** Clause 1: it did not, in the state you name. Run after `chrome()`. */
    titleDidNot(state) {
      return this._title(false, state ?? 'a state you did not name')
    },

    _title(followed, where) {
      heading('the title bar — your eyes')

      ok(
        1,
        'the native title bar followed the choice in all three states — YOUR EYES DECIDE',
        followed === true,
        followed === true
          ? 'watched through system, light and dark, and it followed each'
          : `IT DID NOT FOLLOW in ${where}. The driver's clause 1 says whether the value ` +
            'reached Rust at all; if that passes and this fails, `set_theme` is the half at fault'
      )

      console.log(
        '%cnow run%c  await __gate.flash()',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return tally('the title bar')
    },

    /* Clause 2, first half: leave the window in the state the relaunch has to
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
          '       await __gate.arm()       (the new window has no listeners yet)\n' +
          '       await __gate.noFlash()   if it came up dark, with no flash of light\n' +
          '       await __gate.flashed()   if you saw the light palette first\n' +
          '     Say which you saw in words — there is no true/false to get backwards.\n' +
          '     The transcript from THIS window survives the quit and report() has it all.',
        'font-weight:bold;color:#1a73e8',
        'color:inherit'
      )
      return 'quit and relaunch, then __gate.noFlash() or __gate.flashed()'
    },

    /* **`landed` took a boolean and that was a mistake**, made twice on
       2026-08-29: the operator reported "no flash" in words and passed `false`
       both times, which is the answer a gate should make impossible to give by
       accident rather than one it should record. The two named calls below say
       what they mean at the call site and this one now refuses. */
    landed() {
      console.log(
        '%crefused%c  — say which you saw, in words:\n' +
          '    await __gate.noFlash()   the window came up dark, with no flash of light\n' +
          '    await __gate.flashed()   you saw the light palette first',
        'font-weight:bold;color:#c5221f',
        'color:inherit'
      )
      return 'use __gate.noFlash() or __gate.flashed()'
    },

    /** Clause 2, second half: it came up right. Run in the RELAUNCHED window. */
    noFlash() {
      return this._landed(true)
    },

    /** Clause 2, second half: it flashed. Run in the RELAUNCHED window. */
    flashed() {
      return this._landed(false)
    },

    async _landed(clean) {
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
        2,
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
         launch are in the console above regardless.

         **It is not deleted along with the clauses that moved**, and that is
         deliberate: it guards the half of this gate that is still run by hand,
         and removing it would re-open by hand exactly the defect it was given
         its `armed` guard for — a clause passing by counting nothing. */
      ok(
        3,
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

    /** Put the appearance back where the run's FIRST window found it. */
    async restore() {
      const back = found ?? 'system'
      await wear(back)
      try {
        localStorage.removeItem(FOUND)
      } catch {
        /* Nothing depends on the removal; the next `forget()` is the reset. */
      }
      note(`the appearance is back at ${JSON.stringify((await asked()).held)}, where the run started`)
      return back
    }
  }

  console.log(
    `%c__gate ready%c  —  set System Settings → Appearance to LIGHT first (clause 2 needs it).\n` +
      `${transcript.length} lines already kept${transcript.length ? ' — __gate.forget() to start over' : ''}.\n` +
      'Two of the four clauses this gate had are now `bun app/driver/drive.mjs`; three are here.\n' +
      'FIRST WINDOW:  __gate.forget() → await __gate.arm() → await __gate.chrome() →\n' +
      '               __gate.titleFollowed() (or .titleDidNot(state)) → await __gate.flash()\n' +
      'THEN quit (⌘Q), `cargo tauri dev` again, re-paste, and in the NEW WINDOW:\n' +
      '               await __gate.arm() → await __gate.noFlash() (or .flashed()) →\n' +
      '               await __gate.restore() → __gate.report()',
    'font-weight:bold;color:#1a73e8',
    'color:inherit'
  )
})()
