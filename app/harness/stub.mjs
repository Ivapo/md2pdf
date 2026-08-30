/* The `window.__TAURI__` the page is handed when it is driven outside a window.
   `app/harness/serve.mjs` injects this into `<head>` of a *copy* of
   `app/dist/index.html`; nothing here is shipped and nothing here is imported by
   the page.

   **Its point is fidelity to what the app refuses, not only to what it answers.**
   `getCurrentWindow().setSize` is present and *rejects*, in the words
   `app/capabilities/default.json`'s `core:default` produces. A stub that omits
   what the real thing forbids tests the wrong half — which is the finding this
   file exists to make permanent: `mpdf-003` Phase 11's gate shipped a
   `typeof win.setSize === 'function'` probe that cannot see a capability denial,
   and it cost two runs by a person at a window before it was corrected.

   **It is a module, and the position it is injected at is load-bearing.** The
   page reads `window['__TAURI__']` at module top level
   (`app/dist/index.html`, `const { open, save } = window['__TAURI__'].dialog`),
   and module scripts execute in document order — so this one, in `<head>`, runs
   first. A `<script>` injected into `<body>` would sit between `</main>` and the
   module script and change the very element order the first check asserts.

   The harness's own control surface is `window.__harness`; everything else here
   answers as `app/src/main.rs`'s commands do.                                */

/* ------------------------------------------------------------------ errors */

/* **Installed before anything of the page runs**, which is the whole reason
   this file is in `<head>`. `page.on('pageerror')` does not see a
   `ResizeObserver` loop error — measured — so the count has to come from a
   listener inside the page, exactly as `tests/gates/*.js` do it.

   `ResizeObserver` errors are counted apart from the rest. That class is the one
   `mpdf-009` Phase 3 found twenty-one of in a single run, and an unrelated throw
   must still be visible beside it. **Named and not merely counted**: a run
   reporting "3 uncaught" and nothing else sends the next round looking for
   something it cannot see. */
const errors = { total: 0, loops: 0, spoken: [] }
const say = (what) => {
  errors.total++
  if (`${what}`.includes('ResizeObserver')) errors.loops++
  if (errors.spoken.length < 12) errors.spoken.push(what)
}
addEventListener('error', (e) => say(`error: ${e.message || e.error} @ ${e.filename || '?'}:${e.lineno || '?'}`))
addEventListener('unhandledrejection', (e) =>
  say(`rejection: ${(e.reason && (e.reason.message || e.reason.name)) || String(e.reason)}`)
)

/* ------------------------------------------------------------------- state */

/* What `serve.mjs` measured about the document in play, written into the copy
   beside this file. `entries`/`main` are the panel's, `pdf` is the compiled
   file's name in the scratch directory.

   The fallback is the empty project, so this file still loads if it is opened
   without a served config — a broken stub that throws at parse time would take
   the page's own module with it and every check would report the same thing. */
const CONFIG = window.__HARNESS_CONFIG ?? { entries: [], main: null, pdf: 'document.pdf', text: '' }

/* **Both field generations ride in every status, and that is not tidiness.**
   `mpdf-010` Phase 1 renamed the panel's fields: a page older than it reads
   `state.sections` (a `Vec<String>`) and `state.master` (an `Option<String>`),
   where today's reads `entries`, `main` and `edited`. `serve.mjs --rev` serves
   named revisions on both sides of that rename — it is what the A/B needs — so
   the stub sends all five and each page ignores the ones it does not know.

   **`sections` is derived from the entries and is not what a master names**, which
   is a stated approximation rather than an oversight: reading that off the markdown
   would put a copy of `md2pdf_core::section_paths` in here. It costs nothing for
   what the older revision is served for — the A/B runs on a single-file document,
   whose section list is empty either way — and no check here reads it. */
const derived = (state) => ({
  ...state,
  sections: state.entries.filter((e) => e.kind === 'markdown' && e.path !== state.main).map((e) => e.path),
  master: state.main === null ? null : state.main.split('/').pop()
})

/** The four states of `app/src/preview.rs:State`, so a check can drive
    `app/dist/index.html:report` through each of them. */
const EMPTY = {
  state: 'empty', time: null, error: null, page: false, divergence: null,
  revision: 0, reloaded: 0, anchors: [], entries: [], main: null, edited: null,
  appearance: 'system'
}
const OPEN = {
  state: 'current', time: '31 ms', error: null, page: true, divergence: null,
  revision: 1, reloaded: 1, anchors: [{ line: 5, page: 1 }],
  entries: CONFIG.entries, main: CONFIG.main, edited: CONFIG.main,
  appearance: 'system'
}

let status = { ...EMPTY }
let text = ''
const listeners = {}

/* **Every command the page sends, in order.** The page is supposed to ask Rust
   for things rather than do them itself, and a check that reads only the DOM
   cannot tell the two apart: a toggle that set `data-theme` on its own would
   look identical. So the boundary is recorded and asserted against. */
const invokes = []

const fire = (name, payload = null) => {
  for (const fn of listeners[name] ?? []) fn({ event: name, id: 0, payload })
}

/* ------------------------------------------------------------------ invoke */

/* Every command `app/dist/index.html` sends, answered the way
   `app/src/main.rs` answers it. **An unknown name returns `null` rather than
   throwing**: the page's `fail` would put a sentence in the error bar and the
   status checks would then be reading the harness's own complaint. */
const invoke = async (name, args = {}) => {
  invokes.push({ name, args: JSON.parse(JSON.stringify(args)) })
  switch (name) {
    case 'status':
      return JSON.parse(JSON.stringify(derived(status)))

    /* **Real compiled bytes.** `pdf.js` is not stubbed and the pane must
       actually rasterise, so this is the file `serve.mjs` produced with
       `cargo run -p md2pdf-cli -- <doc> -o <scratch>/…`. */
    case 'current_pdf': {
      const answer = await fetch(`./${CONFIG.pdf}`)
      if (!answer.ok) throw new Error(`the harness has no ${CONFIG.pdf}`)
      return new Uint8Array(await answer.arrayBuffer())
    }

    case 'document_text':
      return text

    /* `app/src/preview.rs:set_edited` moves `edited`, reloads the buffer and
       re-arms the watch — and touches no disk. `main` does not move, which is
       the transition the cell check is about. */
    case 'set_edited':
      status.edited = args.path
      text = `# ${args.path}\n\nThe harness's stand-in text for this file.\n`
      status.reloaded += 1
      fire('rendered')
      return null

    case 'set_main':
      status.main = args.path
      status.edited = args.path
      status.revision += 1
      status.reloaded += 1
      fire('rendered')
      return null

    case 'pending_open':
      return null
    case 'export_path':
      return `${CONFIG.main ?? 'document'}.pdf`
    case 'edit':
      text = args.text
      return null
    case 'open_document':
    case 'save':
    case 'export':
    case 'discard':
    case 'create_file':
    case 'trash_file':
      return null

    /* The figure viewer reads bytes for an image row. **`current_pdf`'s route
       exactly**: Rust answers both with a `tauri::ipc::Response`, so both
       arrive as bytes and the page turns them into a blob — and a stub that
       answered this one in some other shape would be checking a page that does
       not ship.

       A path the project does not hold is refused rather than answered with a
       `null` the page would draw as a broken picture, which is the sentence a
       reader gets for a figure that has gone. */
    case 'asset_bytes': {
      const at = CONFIG.assets?.[args.path]
      const answer = at ? await fetch(`./${at}`) : null
      if (!answer?.ok) throw new Error(`the harness serves no asset bytes for ${args.path}`)
      return new Uint8Array(await answer.arrayBuffer())
    }

    /* **The whole of the Rust half, as the page can see it.** The real command
       also writes `settings.json` and calls `window.set_theme`, neither of
       which a browser has — `specs/desktop_app_spec.md` Phase 13's own window
       clause is where those two get eyes. What matters here is the shape: the
       value moves in Rust and comes back through the compile signal, so the
       page places it rather than deciding it. */
    case 'set_appearance':
      status = { ...status, appearance: args.appearance }
      fire('rendered')
      return null

    default:
      return null
  }
}

/* ------------------------------------------------------------------ window */

window.__TAURI__ = {
  core: { invoke },
  dialog: { open: async () => null, save: async () => null },
  event: {
    listen: async (name, fn) => {
      ;(listeners[name] ??= []).push(fn)
      return () => {}
    },
    emit: async () => {}
  },
  dpi: {
    LogicalSize: class { constructor(w, h) { this.width = w; this.height = h } },
    PhysicalSize: class { constructor(w, h) { this.width = w; this.height = h } }
  },
  window: {
    getCurrentWindow: () => ({
      label: 'main',
      title: async () => 'Letur',
      setTitle: async () => {},
      scaleFactor: async () => devicePixelRatio,
      innerSize: async () => ({ width: innerWidth * devicePixelRatio, height: innerHeight * devicePixelRatio }),

      /* **The refusal, in the shipped app's own words.** `core:default` carries
         the window *getters* — `allow-scale-factor`, `allow-inner-size`,
         `allow-title`, the `is-*` family — and no `allow-set-size`, so this
         command is on the object and rejects at the IPC. The sentence is
         `tauri-2.11.5/src/ipc/authority.rs`'s
         `"{command} not allowed. {permission_error_detail}"`, and it is the one
         `rules/desktop.md` records the window answering.

         **Do not make this resolve to let a check drive the viewport.** A gate
         that needs a width sets it through the driver, and a capability widened
         to run a check is one the shipped app would carry for ever. */
      setSize: async () => {
        throw new Error(
          'window.set_size not allowed. Permissions associated with this command: core:window:allow-set-size'
        )
      }
    })
  }
}

/* ----------------------------------------------------------------- harness */

/** What `app/harness/checks.mjs` drives the page through. */
window.__harness = {
  errors,
  config: CONFIG,

  /** The status as the stub holds it, before `derived` widens it. */
  status: () => JSON.parse(JSON.stringify(status)),

  /** Merge into the status. `revision` is left alone unless it is named, so a
      state change redraws the report without asking for bytes again. */
  set(patch) {
    status = { ...status, ...patch }
    return this.status()
  },

  /** Move to the open state — one document, its entries, `edited` = `main`. */
  open() {
    status = { ...OPEN, entries: CONFIG.entries, main: CONFIG.main, edited: CONFIG.main }
    text = CONFIG.text
    return this.status()
  },

  reset() {
    status = { ...EMPTY }
    text = ''
    return this.status()
  },

  fire,

  /** The names `listen` has been called with, so a check can say the page
      registered before it was driven rather than assuming it. */
  listening: () => Object.keys(listeners),

  /** Every command the page has sent, oldest first, as a copy. */
  invokes: () => JSON.parse(JSON.stringify(invokes)),

  /** Forget them, so a check can say "since here" rather than "ever". */
  forget: () => {
    invokes.length = 0
  }
}
