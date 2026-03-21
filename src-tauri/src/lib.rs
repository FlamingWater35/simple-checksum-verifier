use blake2::Blake2b512;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tauri::{AppHandle, Emitter, Manager, State};
use walkdir::WalkDir;

struct AppState {
    cancel_flag: Arc<AtomicBool>,
}

#[derive(Serialize, Deserialize, Clone)]
struct AppSettings {
    theme: String,
    algorithm: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            algorithm: "sha256".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FolderList {
    id: String,
    name: String,
    path: String,
    created_at: u64,
    total_files: usize,
    #[serde(default)]
    checksums: HashMap<String, String>,
    #[serde(default)]
    hashes: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    backups: Vec<String>,
}

impl FolderList {
    fn migrate(&mut self) {
        if !self.checksums.is_empty() && self.hashes.is_empty() {
            for (path, hash) in &self.checksums {
                let mut algo_map = HashMap::new();
                algo_map.insert("sha256".to_string(), hash.clone());
                self.hashes.insert(path.clone(), algo_map);
            }
            self.checksums.clear();
        }
    }

    fn get_available_algorithms(&self) -> Vec<String> {
        let mut algos = std::collections::HashSet::new();
        if let Some(first_file) = self.hashes.values().next() {
            for k in first_file.keys() {
                algos.insert(k.clone());
            }
        } else if !self.checksums.is_empty() {
            algos.insert("sha256".to_string());
        }
        algos.into_iter().collect()
    }
}

#[derive(Serialize)]
struct FolderListSummary {
    id: String,
    name: String,
    path: String,
    created_at: u64,
    total_files: usize,
    backups: Vec<String>,
    available_algorithms: Vec<String>,
}

#[derive(Clone, Serialize)]
struct Progress {
    total: usize,
    processed: usize,
    current_file: String,
    current_location: String,
}

#[derive(Serialize, Clone)]
#[serde(tag = "node_type")]
enum TreeNode {
    File {
        name: String,
        status: String,
    },
    Directory {
        name: String,
        status: String,
        children: Vec<TreeNode>,
    },
}

#[derive(Serialize)]
struct BackupVerifyResult {
    path: String,
    tree: TreeNode,
}

#[derive(Serialize)]
struct FullVerifyResult {
    main: TreeNode,
    backups: Vec<BackupVerifyResult>,
}

fn get_app_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

fn get_lists_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = get_app_dir(app)?;
    path.push("folder_lists");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<AppSettings, String> {
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
fn save_settings(app: AppHandle, settings: AppSettings) -> Result<(), String> {
    let mut path = get_app_dir(&app)?;
    path.push("settings.json");
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

fn hash_file(
    path: &Path,
    cancel_flag: &Arc<AtomicBool>,
    algorithm: &str,
) -> std::io::Result<Option<String>> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 65536];

    if algorithm == "blake2b" {
        let mut hasher = Blake2b512::new();
        loop {
            if cancel_flag.load(Ordering::Relaxed) {
                return Ok(None);
            }
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        Ok(Some(format!("{:x}", hasher.finalize())))
    } else {
        let mut hasher = Sha256::new();
        loop {
            if cancel_flag.load(Ordering::Relaxed) {
                return Ok(None);
            }
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        Ok(Some(format!("{:x}", hasher.finalize())))
    }
}

#[tauri::command]
fn get_app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_folder_lists(app: AppHandle) -> Result<Vec<FolderListSummary>, String> {
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
fn select_folder() -> Option<String> {
    rfd::FileDialog::new()
        .pick_folder()
        .map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
fn cancel_operation(state: State<'_, AppState>) {
    state.cancel_flag.store(true, Ordering::Relaxed);
}

fn compute_folder_checksums(
    target_path: &Path,
    cancel: &Arc<AtomicBool>,
    app_handle: &AppHandle,
    location_label: &str,
    algorithm: &str,
) -> Result<(HashMap<String, String>, usize), String> {
    let mut total_files = 0;
    for entry in WalkDir::new(target_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_files += 1;
        }
    }

    let mut checksums = HashMap::new();
    let mut processed = 0;
    let mut last_emit = Instant::now();

    for entry in WalkDir::new(target_path).into_iter().filter_map(|e| e.ok()) {
        if cancel.load(Ordering::Relaxed) {
            return Err("Cancelled".into());
        }

        if entry.file_type().is_file() {
            let file_path = entry.path();
            let relative_path = file_path
                .strip_prefix(target_path)
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace("\\", "/");

            match hash_file(file_path, cancel, algorithm) {
                Ok(Some(hash)) => {
                    checksums.insert(relative_path.clone(), hash);
                }
                Ok(None) => return Err("Cancelled".into()),
                Err(_) => {}
            }

            processed += 1;
            let now = Instant::now();

            if processed == total_files || now.duration_since(last_emit).as_millis() >= 50 {
                let _ = app_handle.emit(
                    "operation_progress",
                    Progress {
                        total: total_files,
                        processed,
                        current_file: relative_path,
                        current_location: location_label.to_string(),
                    },
                );
                last_emit = now;
            }
        }
    }
    Ok((checksums, total_files))
}

#[tauri::command]
async fn generate_checksums(
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
        let (new_hashes, total_files) =
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
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            total_files,
            checksums: HashMap::new(),
            hashes: structured_hashes,
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
async fn rehash_folder(
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
        let (new_hashes, total_files) = compute_folder_checksums(
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
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            total_files,
            checksums: HashMap::new(),
            hashes: updated_hashes,
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
fn update_backups(app: AppHandle, id: String, backups: Vec<String>) -> Result<(), String> {
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
fn update_main_path(app: AppHandle, id: String, new_path: String) -> Result<(), String> {
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
fn delete_folder_list(app: AppHandle, id: String) -> Result<(), String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn verify_single_path(
    target_path_str: &str,
    saved_hashes: &HashMap<String, HashMap<String, String>>,
    algorithm: &str,
    cancel: &Arc<AtomicBool>,
    app_handle: &AppHandle,
    location_label: &str,
) -> Result<TreeNode, String> {
    let target_path = PathBuf::from(target_path_str);
    let mut total_files_on_disk = 0;

    if target_path.exists() {
        for entry in WalkDir::new(&target_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                total_files_on_disk += 1;
            }
        }
    }

    let mut disk_checksums = HashMap::new();
    let mut processed = 0;
    let mut last_emit = Instant::now();

    if target_path.exists() {
        for entry in WalkDir::new(&target_path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if cancel.load(Ordering::Relaxed) {
                return Err("Cancelled".into());
            }

            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = file_path
                    .strip_prefix(&target_path)
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .replace("\\", "/");

                match hash_file(file_path, cancel, algorithm) {
                    Ok(Some(hash)) => {
                        disk_checksums.insert(relative_path.clone(), hash);
                    }
                    Ok(None) => return Err("Cancelled".into()),
                    Err(_) => {}
                }

                processed += 1;
                let now = Instant::now();
                if processed == total_files_on_disk
                    || now.duration_since(last_emit).as_millis() >= 50
                {
                    let _ = app_handle.emit(
                        "operation_progress",
                        Progress {
                            total: total_files_on_disk,
                            processed,
                            current_file: relative_path,
                            current_location: location_label.to_string(),
                        },
                    );
                    last_emit = now;
                }
            }
        }
    }

    let mut all_files = std::collections::HashSet::new();
    for k in saved_hashes.keys() {
        all_files.insert(k.clone());
    }
    for k in disk_checksums.keys() {
        all_files.insert(k.clone());
    }

    #[derive(Default)]
    struct NodeBuilder {
        files: HashMap<String, String>,
        dirs: HashMap<String, NodeBuilder>,
    }

    let mut root_builder = NodeBuilder::default();

    for file_path in all_files {
        let saved_hash = saved_hashes.get(&file_path).and_then(|m| m.get(algorithm));
        let disk_hash = disk_checksums.get(&file_path);

        let status = match (saved_hash, disk_hash) {
            (Some(s), Some(d)) if s == d => "Match",
            (Some(_), Some(_)) => "Mismatch",
            (Some(_), None) => "Missing",
            (None, Some(_)) => "Untracked",
            _ => unreachable!(),
        };

        let parts: Vec<&str> = file_path.split('/').collect();
        let mut current = &mut root_builder;
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                current.files.insert(part.to_string(), status.to_string());
            } else {
                current = current.dirs.entry(part.to_string()).or_default();
            }
        }
    }

    fn build_tree(name: String, builder: NodeBuilder) -> TreeNode {
        let mut children = Vec::new();
        let mut has_mismatch = false;
        let mut has_missing = false;
        let mut has_untracked = false;

        for (dir_name, dir_builder) in builder.dirs {
            let node = build_tree(dir_name, dir_builder);
            if let TreeNode::Directory { ref status, .. } = node {
                match status.as_str() {
                    "Mismatch" => has_mismatch = true,
                    "Missing" => has_missing = true,
                    "Untracked" => has_untracked = true,
                    _ => {}
                }
            }
            children.push(node);
        }

        for (file_name, status) in builder.files {
            match status.as_str() {
                "Mismatch" => has_mismatch = true,
                "Missing" => has_missing = true,
                "Untracked" => has_untracked = true,
                _ => {}
            }
            children.push(TreeNode::File {
                name: file_name,
                status,
            });
        }

        children.sort_by(|a, b| {
            let (a_is_dir, a_name) = match a {
                TreeNode::Directory { name, .. } => (true, name),
                TreeNode::File { name, .. } => (false, name),
            };
            let (b_is_dir, b_name) = match b {
                TreeNode::Directory { name, .. } => (true, name),
                TreeNode::File { name, .. } => (false, name),
            };
            if a_is_dir == b_is_dir {
                a_name.cmp(b_name)
            } else {
                b_is_dir.cmp(&a_is_dir)
            }
        });

        let status = if has_mismatch {
            "Mismatch"
        } else if has_missing {
            "Missing"
        } else if has_untracked {
            "Untracked"
        } else {
            "Match"
        };

        TreeNode::Directory {
            name,
            status: status.to_string(),
            children,
        }
    }

    Ok(build_tree("Root".to_string(), root_builder))
}

#[tauri::command]
async fn verify_folder_contents(
    app: AppHandle,
    id: String,
    algorithm: String,
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
            &algorithm,
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
                &algorithm,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            cancel_flag: Arc::new(AtomicBool::new(false)),
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            get_app_version,
            open_url,
            get_folder_lists,
            select_folder,
            generate_checksums,
            rehash_folder,
            update_backups,
            update_main_path,
            delete_folder_list,
            verify_folder_contents,
            cancel_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
