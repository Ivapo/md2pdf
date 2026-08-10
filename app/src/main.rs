//! `md2pdf-app` — show the PDF while you write it.
//!
//! This binary owns the window and all file I/O. The core crate owns the
//! pipeline and touches neither, which is what lets the same crate serve a
//! terminal and a window without a rewrite.

// The Tauri template opens with a `windows_subsystem` attribute here. This
// spec is macOS only, so a line for a platform the phase cannot test is left
// out rather than carried untested.

mod document;

use std::path::PathBuf;

use tauri::menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder};
use tauri::{Emitter, Manager};

/// The label of the one window, which `tauri.conf.json` names too.
const MAIN: &str = "main";

/// The id of the Open menu item, and the event it sends the page.
///
/// The menu does not open the dialog itself. It asks the page to, so the
/// menu item and the button in the page run one code path and not two.
const OPEN: &str = "open";

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            app.set_menu(menu(app.handle())?)?;
            app.on_menu_event(|app, event| {
                if event.id() == OPEN
                    && let Some(window) = app.get_webview_window(MAIN)
                {
                    let _ = window.emit(OPEN, ());
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_document])
        .run(tauri::generate_context!())
        .expect("the app failed to start");
}

/// Compile the document the user picked, and hand the page its bytes.
///
/// The bytes cross as a `tauri::ipc::Response`, which reaches the page as an
/// `ArrayBuffer`. A returned `Vec<u8>` would serialize as a JSON array of
/// numbers instead, one number per byte.
///
/// The title is set before the compile, so the window names the document the
/// user opened whether or not it compiled.
///
/// The command is `async`, which puts the compile on the runtime's pool rather
/// than on the thread that draws the window.
#[tauri::command]
async fn open_document(
    window: tauri::Window,
    path: String,
) -> Result<tauri::ipc::Response, String> {
    let document = PathBuf::from(path);

    window
        .set_title(&document::title(&document))
        .map_err(|e| e.to_string())?;

    document::render(&document).map(tauri::ipc::Response::new)
}

/// The window's menu.
///
/// macOS draws no menu of its own, so every item the keyboard needs is named
/// here — `Cmd-O` above all, which is the accelerator this phase adds.
fn menu(app: &tauri::AppHandle) -> tauri::Result<tauri::menu::Menu<tauri::Wry>> {
    let open = MenuItemBuilder::with_id(OPEN, "Open…")
        .accelerator("CmdOrCtrl+O")
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
