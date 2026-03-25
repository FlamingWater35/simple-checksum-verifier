use crate::core::{compute_folder_checksums, verify_single_path};
use crate::models::{
    AppSettings, BackupVerifyResult, FolderList, FolderListSummary, FullVerifyResult,
};
use crate::state::AppState;
use crate::utils::{get_app_dir, get_lists_dir};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
    let mut path = get_app_dir(&app)?;
    path.push("settings.json");
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(serde_json::from_str(&content).unwrap_or_default())
    } else {
        Ok(AppSettings::default())
    }
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let mut path = get_app_dir(&app)?;
    path.push("settings.json");
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
pub fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_folder_lists(app: AppHandle) -> Result<Vec<FolderListSummary>, String> {
    let dir = get_lists_dir(&app)?;
    let mut summaries = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry
                .path()
                .extension()
                .map(|e| e == "json")
                .unwrap_or(false)
            {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    if let Ok(mut list) = serde_json::from_str::<FolderList>(&content) {
                        list.migrate();
                        let available_algorithms = list.get_available_algorithms();
                        summaries.push(FolderListSummary {
                            id: list.id,
                            name: list.name,
                            path: list.path,
                            created_at: list.created_at,
                            total_files: list.total_files,
                            backups: list.backups,
                            available_algorithms,
                        });
                    }
                }
            }
        }
    }
    summaries.sort_by_key(|s| std::cmp::Reverse(s.created_at));
    Ok(summaries)
}

#[tauri::command]
pub fn select_folder() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub fn cancel_operation(state: State<'_, AppState>) {
    state.cancel_flag.store(true, Ordering::Relaxed);
}

#[tauri::command]
pub async fn generate_checksums(
    app: AppHandle,
    name: String,
    target_path: String,
    algorithm: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let path = PathBuf::from(&target_path);
    if !path.is_dir() {
        return Err("Path is not a directory".into());
    }

    state.cancel_flag.store(false, Ordering::Relaxed);
    let cancel = state.cancel_flag.clone();
    let app_handle = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let (new_hashes, metadata, total_files) =
            compute_folder_checksums(&path, &cancel, &app_handle, "Main Folder", &algorithm)?;

        let mut structured_hashes = HashMap::new();
        for (p, h) in new_hashes {
            let mut map = HashMap::new();
            map.insert(algorithm.clone(), h);
            structured_hashes.insert(p, map);
        }

        let id = uuid::Uuid::new_v4().to_string();
        let list = FolderList {
            id: id.clone(),
            name,
            path: target_path,
            created_at: std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            total_files,
            checksums: HashMap::new(),
            hashes: structured_hashes,
            metadata,
            backups: Vec::new(),
        };

        let mut out_path = get_lists_dir(&app_handle)?;
        out_path.push(format!("{}.json", list.id));
        let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
        fs::write(out_path, json).map_err(|e| e.to_string())?;

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn rehash_folder(
    app: AppHandle,
    id: String,
    algorithm: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut saved_list: FolderList = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    saved_list.migrate();

    let target_path_buf = PathBuf::from(&saved_list.path);
    if !target_path_buf.is_dir() {
        return Err("Path is not a directory or is missing".into());
    }

    state.cancel_flag.store(false, Ordering::Relaxed);
    let cancel = state.cancel_flag.clone();
    let app_handle = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let (new_hashes, metadata, total_files) = compute_folder_checksums(
            &target_path_buf,
            &cancel,
            &app_handle,
            "Main Folder",
            &algorithm,
        )?;

        let mut updated_hashes = HashMap::new();
        for (p, h) in new_hashes {
            let mut file_hashes = saved_list.hashes.get(&p).cloned().unwrap_or_default();
            file_hashes.insert(algorithm.clone(), h);
            updated_hashes.insert(p, file_hashes);
        }

        let list = FolderList {
            id: id.clone(),
            name: saved_list.name.clone(),
            path: saved_list.path.clone(),
            created_at: std::time::SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            total_files,
            checksums: HashMap::new(),
            hashes: updated_hashes,
            metadata,
            backups: saved_list.backups.clone(),
        };

        let mut out_path = get_lists_dir(&app_handle)?;
        out_path.push(format!("{}.json", id));
        let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
        fs::write(out_path, json).map_err(|e| e.to_string())?;

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub fn update_backups(app: AppHandle, id: String, backups: Vec<String>) -> Result<(), String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut list: FolderList = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    list.backups = backups;
    let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_main_path(app: AppHandle, id: String, new_path: String) -> Result<(), String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut list: FolderList = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    list.path = new_path;
    let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_folder_list(app: AppHandle, id: String) -> Result<(), String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn verify_folder_contents(
    app: AppHandle,
    id: String,
    algorithm: String,
    verify_depth: String,
    state: State<'_, AppState>,
) -> Result<FullVerifyResult, String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut saved_list: FolderList = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    saved_list.migrate();

    state.cancel_flag.store(false, Ordering::Relaxed);
    let cancel = state.cancel_flag.clone();
    let app_handle = app.clone();

    tokio::task::spawn_blocking(move || -> Result<FullVerifyResult, String> {
        let main_tree = verify_single_path(
            &saved_list.path,
            &saved_list.hashes,
            &saved_list.metadata,
            &algorithm,
            &verify_depth,
            &cancel,
            &app_handle,
            "Main Folder",
        )?;

        let mut backups_results = Vec::new();
        for (idx, backup_path) in saved_list.backups.iter().enumerate() {
            let label = format!("Backup {}", idx + 1);
            let tree = verify_single_path(
                backup_path,
                &saved_list.hashes,
                &saved_list.metadata,
                &algorithm,
                &verify_depth,
                &cancel,
                &app_handle,
                &label,
            )?;
            backups_results.push(BackupVerifyResult {
                path: backup_path.clone(),
                tree,
            });
        }

        Ok(FullVerifyResult {
            main: main_tree,
            backups: backups_results,
        })
    })
    .await
    .map_err(|e| e.to_string())?
}
