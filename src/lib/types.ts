export type FolderListSummary = {
  id: string;
  name: string;
  path: string;
  created_at: number;
  total_files: number;
};

export type Progress = {
  total: number;
  processed: number;
  current_file: string;
};

export type TreeNode = {
  node_type: "File" | "Directory";
  name: string;
  status: string; // "Match", "Mismatch", "Missing", "Untracked"
  children?: TreeNode[];
};
