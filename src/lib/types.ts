export type FolderListSummary = {
  id: string;
  name: string;
  path: string;
  created_at: number;
  total_files: number;
  backups: string[];
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
  status: string; // "Match", "Mismatch", "Missing", "Untracked"
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
