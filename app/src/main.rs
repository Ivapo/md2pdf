//! `md2pdf-app` — show the PDF while you write it.
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

use std::path::PathBuf;
use std::sync::Mutex;

use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{Emitter, Manager};

use preview::{Session, Status};

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
const SAVE: &str = "save";

/// The signal the loop sends the page after every compile.
///
/// It carries no payload. The page then invokes [`current_pdf`], because an
/// event carrying the bytes would serialize them as a JSON array of numbers,
/// one per byte — the cost the `tauri::ipc::Response` boundary already refused.
const RENDERED: &str = "rendered";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.set_menu(menu(app.handle())?)?;
            app.on_menu_event(|app, event| {
                let id = event.id();
                if (id == OPEN || id == SAVE)
                    && let Some(window) = app.get_webview_window(MAIN)
                {
                    let _ = window.emit(id.as_ref(), ());
                }
            });

            // The watch loop compiles with nobody asking, so the session is
            // built with the one thing it cannot decide for itself: how to
            // tell the page that a compile happened.
            let handle = app.handle().clone();
            app.manage(Mutex::new(Session::new(move || {
                let _ = handle.emit(RENDERED, ());
            })));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_document,
            current_pdf,
            status,
            export_path,
            export
        ])
        .run(tauri::generate_context!())
        .expect("the app failed to start");
}

/// Open the document the user picked: compile it, and watch it from now on.
///
/// It returns no bytes. Phase 1's one call both compiled and returned, and the
/// watch loop splits those, because the loop compiles without being asked and
/// the page has to be able to fetch what it compiled. A compile that fails is
/// not an error here — it is a state the page draws, and it arrives through
/// [`current_pdf`] like any other.
///
/// The title is set before the compile, so the window names the document the
/// user opened whether or not it compiled.
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

    session
        .lock()
        .expect("the session lock was poisoned")
        .open(document)
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
#[tauri::command]
fn status(session: tauri::State<'_, Mutex<Session>>) -> Status {
    session
        .lock()
        .expect("the session lock was poisoned")
        .preview()
        .status()
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
/// here — `Cmd-O` and `Shift-Cmd-S`, which are the two accelerators the app has.
///
/// The export takes `Shift-Cmd-S` and not `Cmd-S` deliberately. Phase 4 puts a
/// text pane beside the preview, where `Cmd-S` means save the document, and an
/// accelerator taken now would have to be given back then.
fn menu(app: &tauri::AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    let open = MenuItemBuilder::with_id(OPEN, "Open…")
        .accelerator("CmdOrCtrl+O")
        .build(app)?;
    let save = MenuItemBuilder::with_id(SAVE, "Save a Copy…")
        .accelerator("Shift+CmdOrCtrl+S")
        .build(app)?;

    MenuBuilder::new(app)
        .items(&[
            &SubmenuBuilder::new(app, "md2pdf")
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
