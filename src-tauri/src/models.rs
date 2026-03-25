use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppSettings {
    pub theme: String,
    pub algorithm: String,
    pub verify_depth: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "auto".to_string(),
            algorithm: "sha256".to_string(),
            verify_depth: "deep".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct FileMetadata {
    pub size: u64,
    pub modified: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderList {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
    pub total_files: usize,
    #[serde(default)]
    pub checksums: HashMap<String, String>,
    #[serde(default)]
    pub hashes: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub metadata: HashMap<String, FileMetadata>,
    #[serde(default)]
    pub backups: Vec<String>,
}

impl FolderList {
    pub fn migrate(&mut self) {
        if !self.checksums.is_empty() && self.hashes.is_empty() {
            for (path, hash) in &self.checksums {
                let mut algo_map = HashMap::new();
                algo_map.insert("sha256".to_string(), hash.clone());
                self.hashes.insert(path.clone(), algo_map);
            }
            self.checksums.clear();
        }
    }

    pub fn get_available_algorithms(&self) -> Vec<String> {
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
pub struct FolderListSummary {
    pub id: String,
    pub name: String,
    pub path: String,
    pub created_at: u64,
    pub total_files: usize,
    pub backups: Vec<String>,
    pub available_algorithms: Vec<String>,
}

#[derive(Clone, Serialize)]
pub struct Progress {
    pub total: usize,
    pub processed: usize,
    pub current_file: String,
    pub current_location: String,
}

#[derive(Serialize, Clone)]
#[serde(tag = "node_type")]
pub enum TreeNode {
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
pub struct BackupVerifyResult {
    pub path: String,
    pub tree: TreeNode,
}

#[derive(Serialize)]
pub struct FullVerifyResult {
    pub main: TreeNode,
    pub backups: Vec<BackupVerifyResult>,
}
