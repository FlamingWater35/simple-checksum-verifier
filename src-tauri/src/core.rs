use crate::models::{FileMetadata, Progress, TreeNode};
use blake2::Blake2b512;
use md5::Md5;
use rayon::prelude::*;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Instant, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;

fn hash_file(
    path: &Path,
    cancel_flag: &Arc<AtomicBool>,
    algorithm: &str,
    buffer_size: usize,
) -> std::io::Result<Option<String>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = vec![0; buffer_size];

    match algorithm {
        "blake3" => {
            let mut hasher = blake3::Hasher::new();
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Ok(Some(hasher.finalize().to_hex().to_string()))
        }
        "blake2b" => {
            let mut hasher = Blake2b512::new();
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Ok(Some(format!("{:x}", hasher.finalize())))
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Ok(Some(format!("{:x}", hasher.finalize())))
        }
        "md5" => {
            let mut hasher = Md5::new();
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Ok(Some(format!("{:x}", hasher.finalize())))
        }
        _ => {
            let mut hasher = Sha256::new();
            loop {
                if cancel_flag.load(Ordering::Relaxed) {
                    return Ok(None);
                }
                let count = file.read(&mut buffer)?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }
            Ok(Some(format!("{:x}", hasher.finalize())))
        }
    }
}

pub fn compute_folder_checksums(
    target_path: &Path,
    cancel: &Arc<AtomicBool>,
    app_handle: &AppHandle,
    location_label: &str,
    algorithm: &str,
    read_mode: &str,
    buffer_size: usize,
) -> Result<
    (
        HashMap<String, String>,
        HashMap<String, FileMetadata>,
        usize,
    ),
    String,
> {
    let entries: Vec<_> = WalkDir::new(target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let total_files = entries.len();
    let (tx, rx) = std::sync::mpsc::channel();

    let cancel_ref = cancel.clone();
    let target_path_ref = target_path.to_path_buf();
    let algo_ref = algorithm.to_string();
    let mode_ref = read_mode.to_string();

    rayon::spawn(move || {
        let process_entry = |entry: walkdir::DirEntry, sender: &std::sync::mpsc::Sender<_>| {
            if cancel_ref.load(Ordering::Relaxed) {
                return;
            }

            let file_path = entry.path();
            let relative_path = file_path
                .strip_prefix(&target_path_ref)
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace("\\", "/");

            let meta_res = fs::metadata(file_path);
            let mut is_access_denied = meta_res.is_err();
            let meta = meta_res.ok();

            let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let modified = meta
                .as_ref()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let mut hash_opt = None;
            if !is_access_denied {
                match hash_file(file_path, &cancel_ref, &algo_ref, buffer_size) {
                    Ok(Some(h)) => hash_opt = Some(h),
                    Ok(None) => {}
                    Err(_) => {
                        is_access_denied = true;
                    }
                }
            }

            let metadata = FileMetadata {
                size,
                modified,
                is_access_denied,
            };

            let _ = sender.send((relative_path, hash_opt, metadata));
        };

        if mode_ref == "sequential" {
            entries.into_iter().for_each(|e| process_entry(e, &tx));
        } else {
            entries
                .into_par_iter()
                .for_each_with(tx, |tx_ref, e| process_entry(e, tx_ref));
        }
    });

    let mut checksums = HashMap::new();
    let mut metadata_map = HashMap::new();
    let mut processed = 0;
    let mut last_emit = Instant::now();

    for (rel_path, hash_opt, meta) in rx {
        if cancel.load(Ordering::Relaxed) {
            return Err("Cancelled".into());
        }

        if let Some(hash) = hash_opt {
            checksums.insert(rel_path.clone(), hash);
        }
        metadata_map.insert(rel_path.clone(), meta);

        processed += 1;
        let now = Instant::now();
        if processed == total_files || now.duration_since(last_emit).as_millis() >= 50 {
            let _ = app_handle.emit(
                "operation_progress",
                Progress {
                    total: total_files,
                    processed,
                    current_file: rel_path,
                    current_location: location_label.to_string(),
                },
            );
            last_emit = now;
        }
    }

    Ok((checksums, metadata_map, total_files))
}

pub fn verify_single_path(
    target_path_str: &str,
    saved_hashes: &HashMap<String, HashMap<String, String>>,
    saved_metadata: &HashMap<String, FileMetadata>,
    algorithm: &str,
    verify_depth: &str,
    cancel: &Arc<AtomicBool>,
    app_handle: &AppHandle,
    location_label: &str,
    read_mode: &str,
    buffer_size: usize,
) -> Result<TreeNode, String> {
    let target_path = PathBuf::from(target_path_str);

    let entries: Vec<_> = WalkDir::new(&target_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .collect();

    let total_files_on_disk = entries.len();
    let (tx, rx) = std::sync::mpsc::channel();

    let cancel_ref = cancel.clone();
    let target_path_ref = target_path.to_path_buf();
    let algo_ref = algorithm.to_string();
    let depth_ref = verify_depth.to_string();
    let mode_ref = read_mode.to_string();

    rayon::spawn(move || {
        let process_entry = |entry: walkdir::DirEntry, sender: &std::sync::mpsc::Sender<_>| {
            if cancel_ref.load(Ordering::Relaxed) {
                return;
            }

            let file_path = entry.path();
            let relative_path = file_path
                .strip_prefix(&target_path_ref)
                .unwrap()
                .to_string_lossy()
                .to_string()
                .replace("\\", "/");

            let meta_res = fs::metadata(file_path);
            let mut live_is_access_denied = meta_res.is_err();
            let meta = meta_res.ok();

            let live_size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
            let live_modified = meta
                .as_ref()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            let mut hash_opt = None;
            if depth_ref == "deep" && !live_is_access_denied {
                match hash_file(file_path, &cancel_ref, &algo_ref, buffer_size) {
                    Ok(Some(h)) => hash_opt = Some(h),
                    Ok(None) => {}
                    Err(_) => {
                        live_is_access_denied = true;
                    }
                }
            }

            let _ = sender.send((
                relative_path,
                live_size,
                live_modified,
                live_is_access_denied,
                hash_opt,
            ));
        };

        if mode_ref == "sequential" {
            entries.into_iter().for_each(|e| process_entry(e, &tx));
        } else {
            entries
                .into_par_iter()
                .for_each_with(tx, |tx_ref, e| process_entry(e, tx_ref));
        }
    });

    let mut disk_checksums = HashMap::new();
    let mut disk_metadata = HashMap::new();
    let mut processed = 0;
    let mut last_emit = Instant::now();

    for (rel_path, size, modified, is_access_denied, hash_opt) in rx {
        if cancel.load(Ordering::Relaxed) {
            return Err("Cancelled".into());
        }

        disk_metadata.insert(
            rel_path.clone(),
            FileMetadata {
                size,
                modified,
                is_access_denied,
            },
        );
        if let Some(hash) = hash_opt {
            disk_checksums.insert(rel_path.clone(), hash);
        }

        processed += 1;
        let now = Instant::now();
        if processed == total_files_on_disk || now.duration_since(last_emit).as_millis() >= 50 {
            let _ = app_handle.emit(
                "operation_progress",
                Progress {
                    total: total_files_on_disk,
                    processed,
                    current_file: rel_path,
                    current_location: location_label.to_string(),
                },
            );
            last_emit = now;
        }
    }

    let mut all_files = std::collections::HashSet::new();
    for k in saved_hashes.keys() {
        all_files.insert(k.clone());
    }
    for k in disk_metadata.keys() {
        all_files.insert(k.clone());
    }

    #[derive(Default)]
    struct NodeBuilder {
        files: HashMap<String, String>,
        dirs: HashMap<String, NodeBuilder>,
    }

    let mut root_builder = NodeBuilder::default();

    for file_path in all_files {
        let is_tracked =
            saved_metadata.contains_key(&file_path) || saved_hashes.contains_key(&file_path);

        let status = if !disk_metadata.contains_key(&file_path) {
            "Missing"
        } else if disk_metadata.get(&file_path).unwrap().is_access_denied {
            "Access Denied"
        } else if !is_tracked {
            "Untracked"
        } else {
            let d_m = disk_metadata.get(&file_path).unwrap();

            if verify_depth == "quick" {
                if let Some(s_m) = saved_metadata.get(&file_path) {
                    if s_m.is_access_denied {
                        "Modified"
                    } else if s_m.size == d_m.size && s_m.modified == d_m.modified {
                        "Match"
                    } else {
                        "Modified"
                    }
                } else {
                    "Modified"
                }
            } else {
                let s_h = saved_hashes.get(&file_path).and_then(|m| m.get(algorithm));
                let d_h = disk_checksums.get(&file_path);

                match (s_h, d_h) {
                    (Some(s), Some(d)) if s == d => "Match",
                    _ => "Mismatch",
                }
            }
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
        let mut has_modified = false;
        let mut has_missing = false;
        let mut has_untracked = false;
        let mut has_access_denied = false;

        for (dir_name, dir_builder) in builder.dirs {
            let node = build_tree(dir_name, dir_builder);
            if let TreeNode::Directory { ref status, .. } = node {
                match status.as_str() {
                    "Mismatch" => has_mismatch = true,
                    "Modified" => has_modified = true,
                    "Missing" => has_missing = true,
                    "Untracked" => has_untracked = true,
                    "Access Denied" => has_access_denied = true,
                    _ => {}
                }
            }
            children.push(node);
        }

        for (file_name, status) in builder.files {
            match status.as_str() {
                "Mismatch" => has_mismatch = true,
                "Modified" => has_modified = true,
                "Missing" => has_missing = true,
                "Untracked" => has_untracked = true,
                "Access Denied" => has_access_denied = true,
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
        } else if has_access_denied {
            "Access Denied"
        } else if has_modified {
            "Modified"
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
