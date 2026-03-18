<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TreeItem from "./TreeItem.svelte";

  // Types
  type FolderListSummary = {
    id: string;
    name: string;
    path: string;
    created_at: number;
    total_files: number;
  };

  type Progress = {
    total: number;
    processed: number;
    current_file: string;
  };

  type TreeNode = {
    node_type: "File" | "Directory";
    name: string;
    status: string; // "Match", "Mismatch", "Missing", "Untracked"
    children?: TreeNode[];
  };

  // State
  let folderLists: FolderListSummary[] = $state([]);
  let searchQuery: string = $state("");

  // Create Dialog State
  let showCreateDialog = $state(false);
  let selectedPath = $state("");
  let newFolderName = $state("");
  let isGenerating = $state(false);
  let generateProgress: Progress | null = $state(null);

  // Verification State
  let isVerifying = $state(false);
  let verifyProgress: Progress | null = $state(null);
  let verifyResult: TreeNode | null = $state(null);
  let activeVerifyId: string | null = $state(null);

  // Computed Values
  let filteredLists = $derived(
    folderLists.filter(
      (list) =>
        list.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        list.path.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  onMount(async () => {
    await fetchFolderLists();

    // Listeners for progress bars
    listen<Progress>("generate_progress", (event) => {
      generateProgress = event.payload;
    });

    listen("generate_done", () => {
      isGenerating = false;
      showCreateDialog = false;
      fetchFolderLists();
    });

    listen<Progress>("verify_progress", (event) => {
      verifyProgress = event.payload;
    });

    listen("verify_done", () => {
      isVerifying = false;
    });
  });

  async function fetchFolderLists() {
    folderLists = await invoke("get_folder_lists");
  }

  async function openFolderPicker() {
    const path = await invoke<string | null>("select_folder");
    if (path) {
      selectedPath = path;
      const parts = path.split(/[\\/]/);
      newFolderName = parts[parts.length - 1] || "New Folder";
      showCreateDialog = true;
    }
  }

  async function startGeneration() {
    if (!selectedPath || !newFolderName) return;
    isGenerating = true;
    generateProgress = { total: 0, processed: 0, current_file: "Starting..." };
    try {
      await invoke("generate_checksums", { name: newFolderName, targetPath: selectedPath });
    } catch (e) {
      alert("Error: " + e);
      isGenerating = false;
    }
  }

  async function deleteList(id: string) {
    if (confirm("Are you sure you want to delete this folder list?")) {
      await invoke("delete_folder_list", { id });
      await fetchFolderLists();
      if (activeVerifyId === id) {
        verifyResult = null;
        activeVerifyId = null;
      }
    }
  }

  async function verifyList(id: string) {
    activeVerifyId = id;
    verifyResult = null;
    isVerifying = true;
    verifyProgress = { total: 0, processed: 0, current_file: "Starting..." };
    try {
      verifyResult = await invoke<TreeNode>("verify_folder_contents", { id });
    } catch (e) {
      alert("Error: " + e);
      isVerifying = false;
    }
  }
</script>

<div class="min-h-screen bg-gray-50 text-gray-800 p-6 flex flex-col font-sans">
  <div class="max-w-6xl mx-auto w-full flex-1 flex flex-col">
    <!-- Header -->
    <header class="flex justify-between items-center mb-8">
      <h1 class="text-3xl font-bold text-gray-900">Checksum Verifier</h1>
      <button
        class="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2.5 rounded-lg shadow font-medium transition cursor-pointer"
        onclick={openFolderPicker}
      >
        + Add Folder
      </button>
    </header>

    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-8">

      <!-- Left Panel: Folder Lists -->
      <div class="bg-white rounded-xl shadow border border-gray-100 p-6 flex flex-col">
        <div class="mb-4">
          <input
            type="text"
            placeholder="Search folders..."
            bind:value={searchQuery}
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <div class="flex-1 overflow-y-auto pr-2 space-y-3">
          {#each filteredLists as list}
            <div class="p-4 border border-gray-200 rounded-lg hover:shadow-md transition bg-gray-50 flex flex-col {activeVerifyId === list.id ? 'ring-2 ring-blue-500' : ''}">
              <div class="flex justify-between items-start mb-2">
                <div>
                  <h3 class="font-semibold text-lg text-gray-900">{list.name}</h3>
                  <p class="text-sm text-gray-500 break-all">{list.path}</p>
                </div>
                <button
                  class="text-red-500 hover:text-red-700 p-1 cursor-pointer transition"
                  onclick={() => deleteList(list.id)}
                  title="Delete"
                >
                  <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
              <div class="flex justify-between items-end mt-2">
                <span class="text-xs text-gray-400 font-medium">{list.total_files} files • {new Date(list.created_at * 1000).toLocaleDateString()}</span>
                <button
                  class="bg-gray-200 hover:bg-blue-100 hover:text-blue-700 text-gray-800 px-3 py-1 rounded text-sm font-medium transition cursor-pointer"
                  onclick={() => verifyList(list.id)}
                >
                  Verify
                </button>
              </div>
            </div>
          {/each}
          {#if filteredLists.length === 0}
            <p class="text-gray-500 text-center py-8">No folder lists found.</p>
          {/if}
        </div>
      </div>

      <!-- Right Panel: Verification Details -->
      <div class="bg-white rounded-xl shadow border border-gray-100 p-6 flex flex-col overflow-hidden max-h-[80vh]">
        {#if activeVerifyId}
          <h2 class="text-xl font-semibold mb-4 text-gray-900 border-b pb-2">Verification Results</h2>

          {#if isVerifying && verifyProgress}
            <div class="mb-6 bg-blue-50 p-4 rounded-lg">
              <p class="text-sm font-medium text-blue-800 mb-2">Verifying files...</p>
              <div class="w-full bg-blue-200 rounded-full h-2.5 mb-2">
                <div class="bg-blue-600 h-2.5 rounded-full transition-all duration-300" style="width: {verifyProgress.total ? (verifyProgress.processed / verifyProgress.total) * 100 : 0}%"></div>
              </div>
              <p class="text-xs text-blue-600 truncate">{verifyProgress.processed} / {verifyProgress.total} - {verifyProgress.current_file}</p>
            </div>
          {:else if verifyResult}
            <div class="flex-1 overflow-y-auto p-2 border border-gray-200 rounded bg-gray-50">
               <TreeItem node={verifyResult} defaultOpen={true} />
            </div>
          {:else}
            <div class="flex-1 flex items-center justify-center text-gray-400">
              <p>Waiting for verification results...</p>
            </div>
          {/if}
        {:else}
          <div class="flex-1 flex items-center justify-center text-gray-400">
            <p>Select Verify on a folder list to see results.</p>
          </div>
        {/if}
      </div>

    </div>
  </div>
</div>

<!-- Add Folder Dialog -->
{#if showCreateDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
    <div class="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
      <h2 class="text-2xl font-bold mb-4">Add Folder List</h2>

      <div class="mb-4">
        <label class="block text-sm font-medium text-gray-700 mb-1">Selected Path</label>
        <div class="p-2 bg-gray-100 rounded text-sm text-gray-600 break-all">{selectedPath}</div>
      </div>

      <div class="mb-6">
        <label class="block text-sm font-medium text-gray-700 mb-1">Name</label>
        <input
          type="text"
          bind:value={newFolderName}
          disabled={isGenerating}
          class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
        />
      </div>

      {#if isGenerating && generateProgress}
        <div class="mb-6">
          <p class="text-sm font-medium text-gray-700 mb-2">Generating checksums...</p>
          <div class="w-full bg-gray-200 rounded-full h-2.5 mb-2">
            <div class="bg-blue-600 h-2.5 rounded-full transition-all duration-300" style="width: {generateProgress.total ? (generateProgress.processed / generateProgress.total) * 100 : 0}%"></div>
          </div>
          <p class="text-xs text-gray-500 truncate">{generateProgress.processed} / {generateProgress.total} - {generateProgress.current_file}</p>
        </div>
      {/if}

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 hover:bg-gray-100 transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={() => showCreateDialog = false}
          disabled={isGenerating}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={startGeneration}
          disabled={isGenerating || !newFolderName}
        >
          {isGenerating ? 'Generating...' : 'Save & Generate'}
        </button>
      </div>
    </div>
  </div>
{/if}
