pub mod commands;
pub mod core;
pub mod models;
pub mod state;
pub mod utils;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState {
            cancel_flag: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
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
