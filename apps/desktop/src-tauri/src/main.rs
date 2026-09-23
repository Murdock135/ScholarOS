use scholar_core::{Command, Preview, Store, View};
use std::{path::Path, sync::Mutex};
use tauri::Manager;

#[tauri::command]
fn execute(store: tauri::State<Mutex<Store>>, command: Command) -> Result<View, String> {
    store.lock().map_err(|e| e.to_string())?.execute(command)
}
#[tauri::command]
fn export_backup(store: tauri::State<Mutex<Store>>, path: String) -> Result<(), String> {
    store
        .lock()
        .map_err(|e| e.to_string())?
        .export(Path::new(&path))
}
#[tauri::command]
fn preview_backup(json: String) -> Result<Preview, String> {
    Store::preview(&json)
}
#[tauri::command]
fn restore_backup(store: tauri::State<Mutex<Store>>, json: String) -> Result<String, String> {
    store
        .lock()
        .map_err(|e| e.to_string())?
        .restore(&json)
        .map(|p| p.display().to_string())
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let path = app.path().app_data_dir()?.join("scholaros.sqlite");
            let store = Store::open(&path).map_err(std::io::Error::other)?;
            app.manage(Mutex::new(store));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            execute,
            export_backup,
            preview_backup,
            restore_backup
        ])
        .run(tauri::generate_context!())
        .expect("Could not start ScholarOS");
}
