export type FolderListSummary = {
  id: string;
  name: string;
  path: string;
  created_at: number;
  total_files: number;
  backups: string[];
  available_algorithms: string[];
};

export type Progress = {
  total: number;
  processed: number;
  current_file: string;
  current_location: string;
};

export type TreeNode = {
  node_type: "File" | "Directory";
  name: string;
  status: string; // "Match", "Mismatch", "Missing", "Untracked", "Modified"
  children?: TreeNode[];
};

export type BackupVerifyResult = {
  path: string;
  tree: TreeNode;
};

export type FullVerifyResult = {
  main: TreeNode;
  backups: BackupVerifyResult[];
};

export type AppSettings = {
  theme: "auto" | "light" | "dark";
  algorithm: "sha256" | "blake2b" | "blake3";
  verify_depth: "quick" | "deep";
};
