use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Clone)]
struct FolderList {
    id: String,
    name: String,
    path: String,
    created_at: u64,
    total_files: usize,
    checksums: HashMap<String, String>,
}

#[derive(Serialize)]
struct FolderListSummary {
    id: String,
    name: String,
    path: String,
    created_at: u64,
    total_files: usize,
}

#[derive(Clone, Serialize)]
struct Progress {
    total: usize,
    processed: usize,
    current_file: String,
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

// Stores the config lists in %LOCALAPPDATA%\simple-checksum-verifier\folder_lists\
fn get_lists_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    path.push("folder_lists");
    fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

fn hash_file(path: &Path) -> std::io::Result<String> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
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
                    if let Ok(list) = serde_json::from_str::<FolderList>(&content) {
                        summaries.push(FolderListSummary {
                            id: list.id,
                            name: list.name,
                            path: list.path,
                            created_at: list.created_at,
                            total_files: list.total_files,
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
async fn generate_checksums(
    app: AppHandle,
    name: String,
    target_path: String,
) -> Result<(), String> {
    let path = PathBuf::from(&target_path);
    if !path.is_dir() {
        return Err("Path is not a directory".into());
    }

    let mut total_files = 0;
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            total_files += 1;
        }
    }

    let app_handle = app.clone();

    tokio::task::spawn_blocking(move || -> Result<(), String> {
        let mut checksums = HashMap::new();
        let mut processed = 0;

        for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = file_path
                    .strip_prefix(&path)
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .replace("\\", "/");

                let hash = hash_file(file_path).map_err(|e| e.to_string())?;
                checksums.insert(relative_path.clone(), hash);

                processed += 1;
                // Throttle event emission slightly for massive folders
                if processed % 10 == 0 || processed == total_files {
                    let _ = app_handle.emit(
                        "generate_progress",
                        Progress {
                            total: total_files,
                            processed,
                            current_file: relative_path,
                        },
                    );
                }
            }
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
            checksums,
        };

        let mut out_path = get_lists_dir(&app_handle)?;
        out_path.push(format!("{}.json", list.id));
        let json = serde_json::to_string(&list).map_err(|e| e.to_string())?;
        fs::write(out_path, json).map_err(|e| e.to_string())?;

        let _ = app_handle.emit("generate_done", ());

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
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

#[tauri::command]
async fn verify_folder_contents(app: AppHandle, id: String) -> Result<TreeNode, String> {
    let mut path = get_lists_dir(&app)?;
    path.push(format!("{}.json", id));
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let saved_list: FolderList = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let target_path = PathBuf::from(&saved_list.path);
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

    let app_handle = app.clone();

    tokio::task::spawn_blocking(move || -> Result<TreeNode, String> {
        let mut disk_checksums = HashMap::new();
        let mut processed = 0;

        if target_path.exists() {
            for entry in WalkDir::new(&target_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    let file_path = entry.path();
                    let relative_path = file_path
                        .strip_prefix(&target_path)
                        .unwrap()
                        .to_string_lossy()
                        .to_string()
                        .replace("\\", "/");

                    if let Ok(hash) = hash_file(file_path) {
                        disk_checksums.insert(relative_path.clone(), hash);
                    }

                    processed += 1;
                    if processed % 10 == 0 || processed == total_files_on_disk {
                        let _ = app_handle.emit(
                            "verify_progress",
                            Progress {
                                total: total_files_on_disk,
                                processed,
                                current_file: relative_path,
                            },
                        );
                    }
                }
            }
        }

        let mut all_files = std::collections::HashSet::new();
        for k in saved_list.checksums.keys() {
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
            let saved_hash = saved_list.checksums.get(&file_path);
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

            // Sort directories first, then alphabetical
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

        let root_node = build_tree("Root".to_string(), root_builder);

        let _ = app_handle.emit("verify_done", ());
        Ok(root_node)
    })
    .await
    .map_err(|e| e.to_string())?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_folder_lists,
            select_folder,
            generate_checksums,
            delete_folder_list,
            verify_folder_contents
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
