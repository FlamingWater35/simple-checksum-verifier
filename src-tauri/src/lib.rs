pub mod commands;
pub mod core;
pub mod models;
pub mod state;
pub mod utils;

use crate::models::AppSettings;
use std::fs;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                // Read settings early to apply the OS window frame theme instantly
                let mut path = utils::get_app_dir(app.handle()).unwrap_or_default();
                path.push("settings.json");
                let settings = if path.exists() {
                    let content = fs::read_to_string(&path).unwrap_or_default();
                    serde_json::from_str::<AppSettings>(&content).unwrap_or_default()
                } else {
                    AppSettings::default()
                };

                let theme = match settings.theme.as_str() {
                    "dark" => Some(tauri::Theme::Dark),
                    "light" => Some(tauri::Theme::Light),
                    _ => None,
                };
                let _ = window.set_theme(theme);
            }
            Ok(())
        })
        .manage(state::AppState {
            cancel_flag: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            commands::show_main_window,
            commands::set_window_theme,
            commands::get_settings,
            commands::save_settings,
            commands::get_app_version,
            commands::open_url,
            commands::get_folder_lists,
            commands::select_folder,
            commands::generate_checksums,
            commands::rehash_folder,
            commands::update_backups,
            commands::update_main_path,
            commands::delete_folder_list,
            commands::verify_folder_contents,
            commands::cancel_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
