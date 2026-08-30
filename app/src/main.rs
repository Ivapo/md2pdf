//! `letur` — show the PDF while you write it.
//!
//! This binary owns the window and all file I/O. The core crate owns the
//! pipeline and touches neither, which is what lets the same crate serve a
//! terminal and a window without a rewrite.

// The Tauri template opens with a `windows_subsystem` attribute here. This
// spec is macOS only, so a line for a platform the phase cannot test is left
// out rather than carried untested.

mod document;
mod preview;
mod watch;

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{Emitter, Manager};

use preview::{Appearance, Session, Status};

/// The label of the one window, which `tauri.conf.json` names too.
const MAIN: &str = "main";

/// The id of the Open menu item, and the event it sends the page.
///
/// The menu does not open the dialog itself. It asks the page to, so the
/// menu item and the button in the page run one code path and not two.
const OPEN: &str = "open";

/// The id of the Save-a-copy menu item, and the event it sends the page.
///
/// It follows [`OPEN`] exactly: the item emits, and the page owns the dialog.
const EXPORT: &str = "export";

/// The id of the Save menu item, and the event it sends the page.
///
/// It opens no dialog — the document has a path already — and **since
/// `mpdf-003` Phase 17 it has no button beside it either**, the header's one
/// floppy being [`SAVE_AS`]. It still emits rather than acting: the page hands
/// its text over before it asks for the save, so the file is the pane and not
/// the pane a moment ago, and that ordering is the page's to do.
const SAVE: &str = "save";

/// The id of the Save-as menu item, and the event it sends the page.
///
/// It follows [`OPEN`] and [`EXPORT`]: the item emits, the page owns the
/// dialog. **This one has the button beside it**, and it takes
/// `Shift+CmdOrCtrl+S` — which [`EXPORT`] gave up rather than moving to a
/// second, more obscure chord. `mpdf-003` Phase 17.
const SAVE_AS: &str = "save-as";

/// The signal the loop sends the page after every compile.
///
/// It carries no payload. The page then invokes [`current_pdf`], because an
/// event carrying the bytes would serialize them as a JSON array of numbers,
/// one per byte — the cost the `tauri::ipc::Response` boundary already refused.
const RENDERED: &str = "rendered";

/// The signal a document handed over by Finder sends the page.
///
/// It carries no payload either, and for a different reason than [`RENDERED`]:
/// the path is in [`Pending`], and the page takes it through [`pending_open`].
/// The take is what makes a cold launch and a warm one one code path — whichever
/// of the startup take and this signal's take runs second finds the slot empty
/// and does nothing, so the document cannot open twice.
const OPENED: &str = "opened";

/// The document Finder handed over, until the page comes and takes it.
///
/// The association only *launches* the app; it hands the process nothing, and a
/// bundled app is handed its document by `tauri::RunEvent::Opened` rather than
/// in `argv`. That event can arrive before the page exists, so the path waits
/// here rather than going straight into a window that may not be listening.
///
/// **The open goes through the page rather than around it**, and the page's own
/// `clear()` is why. [`Session::open`] rebuilds from `Preview::default()`, so
/// `revision` and `reloaded` restart at 0 for every document, while the page
/// resets the counters it compares them against only inside `clear()`. A path
/// straight into Rust would never reach that, and a second document opened from
/// Finder would leave both panes showing the first one under a new title.
#[derive(Default)]
struct Pending(Mutex<Option<PathBuf>>);

fn main() {
    // **The chain is broken here and nowhere else, and it is broken for one
    // reason:** an attribute cannot be applied to a method call, so the one
    // `#[cfg]`'d plugin needs a binding to hang off. Everything below runs
    // unbroken from `builder` to `build`, as it did before `driven` existed.
    //
    // `tauri-plugin-wdio-webdriver` starts a WebDriver server on 4445 and is
    // what `app/driver/drive.mjs` speaks to. Off by default: `app/Cargo.toml`
    // makes the dependency optional and this is the only `init()` of it, so a
    // plain build neither links the crate nor opens the port.
    let builder = tauri::Builder::default().plugin(tauri_plugin_dialog::init());

    #[cfg(feature = "driven")]
    let builder = builder.plugin(tauri_plugin_wdio_webdriver::init());

    let app = builder
        .manage(Pending::default())
        .setup(|app| {
            app.set_menu(menu(app.handle())?)?;
            app.on_menu_event(|app, event| {
                let id = event.id();
                if (id == OPEN || id == SAVE || id == SAVE_AS || id == EXPORT)
                    && let Some(window) = app.get_webview_window(MAIN)
                {
                    let _ = window.emit(id.as_ref(), ());
                }
            });

            // The watch loop compiles with nobody asking, so the session is
            // built with the one thing it cannot decide for itself: how to
            // tell the page that a compile happened.
            //
            // The second thing it cannot decide for itself is where the one
            // fact it remembers about a project lives. **Tauri's own resolver
            // answers it**, because `tauri.conf.json`'s identifier is what
            // names the directory and nothing else here should have to know it.
            // A resolver that cannot answer leaves the store beside nothing and
            // the app remembers nothing, which is the same state a first launch
            // is in.
            let support = app.path().app_data_dir().unwrap_or_default();

            // **The third thing it cannot decide for itself is which palette to
            // wear.** It is read here, in the same hook that resolves the store
            // and from the file beside it, and worn before anything else runs —
            // so the choice is on screen from the first frame rather than
            // arriving with the page's first `refresh`.
            //
            // **The window already exists by now**: the runtime builds every
            // `tauri.conf.json` window before it calls this hook, which is what
            // lets `get_webview_window` reach one here at all. So the property
            // is not an ordering against window creation — it is that the read,
            // the `set_theme` and the show all happen inside this one `Ready`
            // callback.
            //
            // **And one `Ready` callback was not enough, which is measured
            // rather than reasoned about.** `specs/desktop_app_spec.md` Phase
            // 13's window gate was run on 2026-08-29 with a stored `dark` on a
            // light system and reported a flash: the runtime does not merely
            // *build* the configured window before this hook, it puts it on
            // screen, so `set_theme` arrives a frame late however early in
            // `setup` it is called. So `tauri.conf.json` carries
            // `"visible": false` and the window is shown here instead — the
            // fallback that phase named in advance, taken because the clause
            // that tests it failed.
            //
            // **`show` is what makes the window appear at all now**, so nothing
            // between the config and this line may return early, and the focus
            // is restored with it: a window created hidden does not take it by
            // being shown, and the app's own launch used to have it.
            let settings = document::settings_file(&support);
            let appearance = document::read_appearance(&settings);
            if let Some(window) = app.get_webview_window(MAIN) {
                let _ = window.set_theme(theme(appearance));
                let _ = window.show();
                let _ = window.set_focus();
            }

            let handle = app.handle().clone();
            app.manage(Mutex::new(Session::new(
                document::store_file(&support),
                settings,
                appearance,
                move || {
                    let _ = handle.emit(RENDERED, ());
                },
            )));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_document,
            set_main,
            set_edited,
            discard,
            asset_bytes,
            create_file,
            trash_file,
            document_text,
            edit,
            save,
            save_as_path,
            save_as,
            current_pdf,
            status,
            set_appearance,
            export_path,
            export,
            pending_open
        ])
        .build(tauri::generate_context!())
        .expect("the app failed to start");

    // Phase 1 ended this `.run(generate_context!())`, which surfaces no run
    // events at all — and a run event is the only way a document opened from
    // Finder reaches the process.
    app.run(|handle, event| {
        if let tauri::RunEvent::Opened { urls } = event {
            // Finder delivers a multiple selection as one event. The app takes
            // the first and ignores the rest, which is Phase 1's "one file at a
            // time" rather than a new decision.
            //
            // The path comes from `to_file_path` and not from `Url::path`,
            // which leaves a space percent-encoded: a document named
            // `my doc.md` would arrive as `my%20doc.md` and open as nothing.
            let Some(document) = urls.first().and_then(|url| url.to_file_path().ok()) else {
                return;
            };

            *handle
                .state::<Pending>()
                .0
                .lock()
                .expect("the pending lock was poisoned") = Some(document);

            if let Some(window) = handle.get_webview_window(MAIN) {
                let _ = window.emit(OPENED, ());
            }
        }
    });
}

/// Open the document the user picked: compile it, and watch it from now on.
///
/// It returns no bytes. Phase 1's one call both compiled and returned, and the
/// watch loop splits those, because the loop compiles without being asked and
/// the page has to be able to fetch what it compiled. A compile that fails is
/// not an error here — it is a state the page draws, and it arrives through
/// [`current_pdf`] like any other.
///
/// **The title is set twice, and both are needed.** Once before the compile
/// with the path the user picked, so the window names something whether or not
/// the compile succeeded — which is what it has always done — and once after,
/// because `Session::open` now opens the file the *project* compiles rather
/// than the file it was handed. A double-click on a section titles the window
/// `text.md` for the length of one compile and `showcase.md` after it, which is
/// the truth in both instants.
///
/// The command is `async`, which puts the compile on the runtime's pool rather
/// than on the thread that draws the window.
#[tauri::command]
async fn open_document(
    window: tauri::Window,
    session: tauri::State<'_, Mutex<Session>>,
    path: String,
) -> Result<(), String> {
    let document = PathBuf::from(path);

    window
        .set_title(&document::title(&document))
        .map_err(|e| e.to_string())?;

    let opened = {
        let mut session = session.lock().expect("the session lock was poisoned");
        session.open(document)?;
        session.preview().document().map(document::title)
    };

    if let Some(name) = opened {
        window.set_title(&name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Set which file under the open project compiles, and remember it.
///
/// The window is retitled for [`open_document`]'s reason: this routes through
/// the open, so the pane is now holding the file it named.
#[tauri::command]
async fn set_main(
    window: tauri::Window,
    session: tauri::State<'_, Mutex<Session>>,
    path: String,
) -> Result<(), String> {
    let opened = {
        let mut session = session.lock().expect("the session lock was poisoned");
        session.set_main(path)?;
        session.preview().document().map(document::title)
    };

    if let Some(name) = opened {
        window.set_title(&name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Put another of the project's files in the pane, leaving the main alone.
///
/// The window is retitled for [`open_document`]'s reason, which is now the
/// whole of what the title says: it names the file the pane holds, and that is
/// no longer the file that compiles.
///
/// **A refusal does not come back through this `Err`.** A path outside the
/// project does; a pane holding unsaved edits does not, because that is a
/// status the page places in the divergence bar rather than an error — see
/// `preview::Session::refused_while_dirty`.
#[tauri::command]
async fn set_edited(
    window: tauri::Window,
    session: tauri::State<'_, Mutex<Session>>,
    path: String,
) -> Result<(), String> {
    let opened = {
        let mut session = session.lock().expect("the session lock was poisoned");
        session.set_edited(path)?;
        session.preview().document().map(document::title)
    };

    if let Some(name) = opened {
        window.set_title(&name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Throw the pane's unsaved edits away and take the file as it stands.
///
/// It is the second way out both refusals name — a switch the pane's own work
/// blocked, and a file that moved under it — and it is the only command in this
/// app that discards anything. It needs no dialog and no capability: the button
/// that reaches it is drawn only beside the sentence that asks for it.
#[tauri::command]
fn discard(session: tauri::State<'_, Mutex<Session>>) {
    session
        .lock()
        .expect("the session lock was poisoned")
        .discard();
}

/// The bytes of one of the project's files, for the page to draw.
///
/// **The rule is [`document::asset_bytes`]'s and none of it is here**, which is
/// this file's own division: the confinement is testable where it lives and
/// would be reachable by nothing if it were written into a command.
///
/// The bytes cross as a `tauri::ipc::Response` for [`current_pdf`]'s reason,
/// and the page turns them into a blob. That route needs no capability and no
/// `app/tauri.conf.json` change, where Tauri's asset protocol would want both.
#[tauri::command]
fn asset_bytes(
    session: tauri::State<'_, Mutex<Session>>,
    path: String,
) -> Result<tauri::ipc::Response, String> {
    let root = {
        let session = session.lock().expect("the session lock was poisoned");
        session
            .preview()
            .root()
            .map(Path::to_path_buf)
            .ok_or_else(|| "no document is open".to_string())?
    };

    // **The lock is dropped before the read**, which is what cloning the root
    // out was for. `std::fs::read` on a figure over a slow volume is unbounded
    // in a way nothing else this command does is, and holding the session
    // across it stalls `status`, `save`, the watch and the compile behind one
    // reader looking at a picture.
    document::asset_bytes(&root, &path).map(tauri::ipc::Response::new)
}

/// Make one empty file in the project, named from the panel.
///
/// **The rule is [`document::create_file`]'s and none of it is here**, for
/// [`asset_bytes`]'s reason — and this is the first time this app writes to a
/// path the author did not choose in a native dialog, which is the reason that
/// division matters most here.
///
/// **Nothing is announced.** The file lands under the watched root, so
/// `crate::watch::classify` answers `Change::Tree` and
/// `crate::preview::Session::on_change` refreshes the listing without
/// compiling. Creation and an external `touch` reach the window by one path.
#[tauri::command]
fn create_file(session: tauri::State<'_, Mutex<Session>>, path: String) -> Result<(), String> {
    let root = {
        let session = session.lock().expect("the session lock was poisoned");
        session
            .preview()
            .root()
            .map(Path::to_path_buf)
            .ok_or_else(|| "no document is open".to_string())?
    };

    // The lock is dropped before the write, for [`asset_bytes`]'s reason: a
    // create on a slow volume is unbounded in a way nothing else here is.
    document::create_file(&root, &path)
}

/// Move one of the project's files to the Trash, named from a row.
///
/// **The rule is `document::trash_file`'s and the session's is
/// `preview::Session::trash`'s, and none of either is here.** This app's first
/// *destructive* operation, so the division matters more here than anywhere:
/// what a command holds is what no test in this repository can reach.
///
/// **Not `std::fs::remove_file`.** There is no undo anywhere in this app — not
/// for an edit, not for a save, not for an export — and the Trash is the
/// platform's own undo for exactly this operation. That is also why nothing
/// asks twice: a confirmation stands in for an undo where there is none, and
/// Finder does not confirm a move to the Trash for the same reason.
///
/// `document::move_to_trash` is the real call, handed in here, because
/// `preview::Session::trash` takes it as a parameter so that the suite can
/// exercise every clause without leaving a file in anybody's Trash.
///
/// **Nothing is announced from here.** The session announces, having refreshed
/// the panel itself — which it must, since a deleted section classifies as
/// `crate::watch::Change::Asset` and never reaches the listing refresh a create
/// rides on.
#[tauri::command]
fn trash_file(session: tauri::State<'_, Mutex<Session>>, path: String) -> Result<(), String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .trash(path, document::move_to_trash)
}

/// The document Finder handed over, if one is waiting.
///
/// **It takes rather than reads**, and that is the whole mechanism. The page
/// calls this at startup and again on every [`OPENED`] signal, so a document
/// that arrived before the page's listener existed is collected by the first
/// and one that arrives after by the second. Whichever runs second finds the
/// slot empty, so the document opens once.
///
/// The page then invokes [`open_document`] with the path exactly as the dialog
/// does, after the same `clear()`. One open path in the page keeps its counters
/// honest.
#[tauri::command]
fn pending_open(pending: tauri::State<'_, Pending>) -> Option<String> {
    pending
        .0
        .lock()
        .expect("the pending lock was poisoned")
        .take()
        .map(|path| path.to_string_lossy().into_owned())
}

/// The text the pane should be holding.
///
/// The page asks for this when the status says the buffer was replaced from
/// disk, and at no other time — a fetch on every compile would race the
/// keystrokes still in flight and could put an older text back in the pane.
#[tauri::command]
fn document_text(session: tauri::State<'_, Mutex<Session>>) -> String {
    session
        .lock()
        .expect("the session lock was poisoned")
        .preview()
        .text()
        .to_string()
}

/// Take what the author has typed.
///
/// The compile is not here. It falls due one quiet interval later, on the
/// session's own typing loop, which is what keeps a keystroke from costing a
/// compile.
#[tauri::command]
fn edit(session: tauri::State<'_, Mutex<Session>>, text: String) {
    session
        .lock()
        .expect("the session lock was poisoned")
        .edit(text);
}

/// Write the pane's text to the document's own path.
///
/// It needs no dialog and so no capability: the path is the one the window
/// already names. The save's own filesystem event arrives a moment later and
/// compiles nothing, because the file and the buffer then say the same thing.
#[tauri::command]
fn save(session: tauri::State<'_, Mutex<Session>>) -> Result<(), String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .save()
}

/// Where the Save-as panel should open, or why it should not open.
///
/// **A mirror of [`export_path`] and for its reason**: the page cannot build an
/// absolute path of its own, and a panel given no default opens wherever macOS
/// last was rather than where the author is working. It answers with the file
/// the pane is holding, which is what a Save-as defaults to everywhere, and it
/// refuses **before** the dialog rather than after it.
///
/// **Two sentences here were corrected by `mpdf-003` Phase 18 and the code was
/// not.** `Status` no longer carries root-relative spellings *only* —
/// `Preview::edited_relative` falls back to an absolute path for a pane outside
/// the root — and [`crate::document::save_file`] no longer confines, so a panel
/// opening elsewhere is no longer a refusal waiting to happen. The default is
/// still worth answering: it opens where the author is, not where macOS was.
///
/// It names the file in the pane and not the file that compiles, which is the
/// opposite of [`export_path`]'s choice and right for the opposite reason: the
/// bytes about to be written are the pane's.
#[tauri::command]
fn save_as_path(session: tauri::State<'_, Mutex<Session>>) -> Result<String, String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .preview()
        .document()
        .map(|path| path.to_string_lossy().into_owned())
        .ok_or_else(|| "no document is open".to_string())
}

/// Write the pane where the author asked, and hold that file after.
///
/// **It sets the title, which is the fourth writer of it in this file** —
/// [`open_document`], [`set_main`] and [`set_edited`] each do the same, the
/// window being the command's to retitle and not the session's.
#[tauri::command]
async fn save_as(
    window: tauri::Window,
    session: tauri::State<'_, Mutex<Session>>,
    path: String,
) -> Result<(), String> {
    let opened = {
        let mut session = session.lock().expect("the session lock was poisoned");
        session.save_as(path)?;
        session.preview().document().map(document::title)
    };

    if let Some(name) = opened {
        window.set_title(&name).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// What the pane should be showing now.
///
/// The bytes cross as a `tauri::ipc::Response`, which reaches the page as an
/// `ArrayBuffer`. A returned `Vec<u8>` would serialize as a JSON array of
/// numbers instead, one number per byte.
///
/// `Err` is the failed compile, and it is not an accident: the page keeps the
/// page it has, draws this sentence above it, and marks the pane stale.
#[tauri::command]
fn current_pdf(session: tauri::State<'_, Mutex<Session>>) -> Result<tauri::ipc::Response, String> {
    let session = session.lock().expect("the session lock was poisoned");
    let preview = session.preview();

    // A stale pane keeps its bytes and gets the message instead, and the page
    // draws the one over the other. Phase 3's export reads the same flag, so
    // the file it writes and the page on screen cannot disagree.
    if preview.is_stale() {
        let error = preview.error().unwrap_or("the page is out of date");
        return Err(error.to_string());
    }

    preview
        .pdf()
        .map(|pdf| tauri::ipc::Response::new(pdf.to_vec()))
        .ok_or_else(|| "no document is open".to_string())
}

/// What the window should say about the last compile.
///
/// This is a second command beside [`current_pdf`] rather than a wider return
/// from it, and both answer the same payload-less `rendered` signal: the bytes
/// cross as a raw `tauri::ipc::Response` and a status does not.
///
/// **It asks the session and not the preview**, one of them knowing a field the
/// other does not. This line is outside every test in this repository, by the
/// division this file records for itself: `preview::Session::status` is where
/// the composition is checked, and the window gate is where the call site is.
#[tauri::command]
fn status(session: tauri::State<'_, Mutex<Session>>) -> Status {
    session
        .lock()
        .expect("the session lock was poisoned")
        .status()
}

/// Which palette the window wears, as the footer's toggle asks for it.
///
/// **Two halves, and the split is what the phase is for.**
/// [`preview::Session::set_appearance`] writes the settings file, moves the
/// value and announces it through `rendered` — all three, because the announce
/// is the page's only route to the footer's mark. Then the native title bar
/// follows, which nothing in the content area can do and which is why this
/// command exists at all rather than the page keeping the preference itself.
///
/// **From the page `set_theme` would reject** — `capabilities/default.json`
/// grants `core:default`, which is the window's getters and no setter — but
/// **from Rust capabilities do not apply**, which is the route
/// [`set_edited`]'s `set_title` already takes. No capability is added for this.
#[tauri::command]
fn set_appearance(
    window: tauri::Window,
    session: tauri::State<'_, Mutex<Session>>,
    appearance: Appearance,
) -> Result<(), String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .set_appearance(appearance)?;

    window
        .set_theme(theme(appearance))
        .map_err(|e| e.to_string())
}

/// An [`Appearance`] as Tauri spells it.
///
/// `tauri::Theme` is `Light | Dark` and the option is the third state:
/// **`None` is *follow the system***, which maps onto
/// [`Appearance::System`] exactly and is why the three states need no fourth
/// call. It lives here rather than beside the enum so that `preview.rs` — which
/// the tests are written against — never imports `tauri`.
fn theme(appearance: Appearance) -> Option<tauri::Theme> {
    match appearance {
        Appearance::System => None,
        Appearance::Light => Some(tauri::Theme::Light),
        Appearance::Dark => Some(tauri::Theme::Dark),
    }
}

/// Where the Save-a-copy dialog should open, or why it should not open.
///
/// The page asks this before it asks the user, so an export the pane cannot
/// serve is refused without a dialog the answer would throw away.
#[tauri::command]
fn export_path(session: tauri::State<'_, Mutex<Session>>) -> Result<String, String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .preview()
        .export_path()
        .map(|path| path.to_string_lossy().into_owned())
}

/// Write the page's own bytes to the path the user picked.
///
/// It compiles nothing. The file is the bytes the pane is showing, which is
/// what keeps the two from disagreeing.
#[tauri::command]
fn export(session: tauri::State<'_, Mutex<Session>>, path: String) -> Result<(), String> {
    session
        .lock()
        .expect("the session lock was poisoned")
        .preview()
        .export(&PathBuf::from(path))
}

/// The window's menu.
///
/// macOS draws no menu of its own, so every item the keyboard needs is named
/// here — `Cmd-O`, `Cmd-S` and `Shift-Cmd-S`, which are the three accelerators
/// the app has.
///
/// `Cmd-S` saves the document and `Shift-Cmd-S` writes a copy of the PDF.
/// Phase 3 gave the export the second of those and reserved the first for the
/// text pane, so this phase spends the accelerator rather than taking one back.
fn menu(app: &tauri::AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    let open = MenuItemBuilder::with_id(OPEN, "Open…")
        .accelerator("CmdOrCtrl+O")
        .build(app)?;
    let save = MenuItemBuilder::with_id(SAVE, "Save")
        .accelerator("CmdOrCtrl+S")
        .build(app)?;
    let save_as = MenuItemBuilder::with_id(SAVE_AS, "Save as…")
        .accelerator("Shift+CmdOrCtrl+S")
        .build(app)?;
    // **The export gives the chord up rather than moving to a second one.**
    // Phase 16 withdrew its button on the argument that a reader who wants it
    // knows to look in `File`, and an accelerator nobody guesses is not better
    // than the item it duplicates. `mpdf-003` Phase 17.
    let export = MenuItemBuilder::with_id(EXPORT, "Save a Copy…").build(app)?;

    MenuBuilder::new(app)
        .items(&[
            &SubmenuBuilder::new(app, "Letur")
                .about(None)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .show_all()
                .separator()
                .quit()
                .build()?,
            &SubmenuBuilder::new(app, "File")
                .item(&open)
                .item(&save)
                .item(&save_as)
                .item(&export)
                .separator()
                .close_window()
                .build()?,
            &SubmenuBuilder::new(app, "Edit")
                .undo()
                .redo()
                .separator()
                .cut()
                .copy()
                .paste()
                .select_all()
                .build()?,
            &SubmenuBuilder::new(app, "Window")
                .minimize()
                .fullscreen()
                .separator()
                .close_window()
                .build()?,
        ])
        .build()
}
