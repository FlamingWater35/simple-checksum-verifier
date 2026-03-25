use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn get_app_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

pub fn get_lists_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = get_app_dir(app)?;
    path.push("folder_lists");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}
