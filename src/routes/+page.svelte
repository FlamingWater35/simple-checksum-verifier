<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TreeItem from "./TreeItem.svelte";
  import type { FolderListSummary, Progress, TreeNode } from "$lib/types";

  // State
  let folderLists: FolderListSummary[] = $state([]);
  let searchQuery: string = $state("");
  let isLoadingFolders = $state(true);

  // Create Dialog State
  let showCreateDialog = $state(false);
  let selectedPath = $state("");
  let newFolderName = $state("");
  let isGenerating = $state(false);
  let generateProgress: Progress | null = $state(null);

  // Duplicate Warning Dialog State
  let showDuplicateWarningDialog = $state(false);
  let pendingFolderPath = $state("");

  // Delete Dialog State
  let showDeleteDialog = $state(false);
  let folderToDelete: FolderListSummary | null = $state(null);

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
        list.path.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  onMount(async () => {
    await fetchFolderLists();
    isLoadingFolders = false;

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
      const isDuplicate = folderLists.some((list) => list.path === path);

      if (isDuplicate) {
        pendingFolderPath = path;
        showDuplicateWarningDialog = true;
      } else {
        prepareCreateDialog(path);
      }
    }
  }

  function prepareCreateDialog(path: string) {
    selectedPath = path;
    const parts = path.split(/[\\/]/);
    newFolderName = parts[parts.length - 1] || "New Folder";
    showCreateDialog = true;
  }

  function continueToCreate() {
    showDuplicateWarningDialog = false;
    prepareCreateDialog(pendingFolderPath);
    pendingFolderPath = "";
  }

  function cancelDuplicateWarning() {
    showDuplicateWarningDialog = false;
    pendingFolderPath = "";
  }

  async function startGeneration() {
    if (!selectedPath || !newFolderName) return;
    isGenerating = true;
    generateProgress = { total: 0, processed: 0, current_file: "Starting..." };
    try {
      await invoke("generate_checksums", {
        name: newFolderName,
        targetPath: selectedPath,
      });
    } catch (e) {
      alert("Error: " + e);
      isGenerating = false;
    }
  }

  function requestDelete(list: FolderListSummary) {
    folderToDelete = list;
    showDeleteDialog = true;
  }

  async function confirmDelete() {
    if (folderToDelete) {
      await invoke("delete_folder_list", { id: folderToDelete.id });
      await fetchFolderLists();
      if (activeVerifyId === folderToDelete.id) {
        verifyResult = null;
        activeVerifyId = null;
      }
    }
    closeDeleteDialog();
  }

  function closeDeleteDialog() {
    showDeleteDialog = false;
    folderToDelete = null;
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

<div
  class="h-screen w-full overflow-hidden bg-gray-50 dark:bg-gray-900 text-gray-800 dark:text-gray-200 p-6 flex flex-col font-sans transition-colors duration-200"
>
  <div class="max-w-6xl mx-auto w-full flex-1 flex flex-col min-h-0">
    <!-- Header -->
    <header class="flex justify-between items-center mb-6 shrink-0">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        Checksum Verifier
      </h1>
      <button
        class="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2.5 rounded-lg shadow font-medium transition cursor-pointer"
        onclick={openFolderPicker}
      >
        + Add Folder
      </button>
    </header>

    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-8 min-h-0">
      <!-- Left Panel: Folder Lists -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl shadow border border-gray-100 dark:border-gray-700 p-6 flex flex-col h-full min-h-0"
      >
        <div class="mb-4 shrink-0">
          <input
            type="text"
            placeholder="Search folders..."
            aria-label="Search folders"
            bind:value={searchQuery}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>

        <div class="flex-1 overflow-y-auto pr-2 space-y-3 min-h-0 relative">
          {#if isLoadingFolders}
            <div class="absolute inset-0 flex items-center justify-center">
              <svg
                class="animate-spin h-8 w-8 text-blue-600 dark:text-blue-500"
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
              >
                <circle
                  class="opacity-25"
                  cx="12"
                  cy="12"
                  r="10"
                  stroke="currentColor"
                  stroke-width="4"
                ></circle>
                <path
                  class="opacity-75"
                  fill="currentColor"
                  d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                ></path>
              </svg>
            </div>
          {:else if filteredLists.length === 0}
            <p class="text-gray-500 dark:text-gray-400 text-center py-8">
              No folder lists found.
            </p>
          {:else}
            {#each filteredLists as list}
              <div
                class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:shadow-md transition bg-gray-50 dark:bg-gray-800/50 flex flex-col {activeVerifyId ===
                list.id
                  ? 'ring-2 ring-blue-500'
                  : ''}"
              >
                <div class="flex justify-between items-start mb-2">
                  <div>
                    <h3
                      class="font-semibold text-lg text-gray-900 dark:text-white"
                    >
                      {list.name}
                    </h3>
                    <p
                      class="text-sm text-gray-500 dark:text-gray-400 break-all"
                    >
                      {list.path}
                    </p>
                  </div>
                  <button
                    class="text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 cursor-pointer transition shrink-0"
                    onclick={() => requestDelete(list)}
                    title="Delete"
                  >
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      class="h-5 w-5"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                      />
                    </svg>
                  </button>
                </div>
                <div class="flex justify-between items-end mt-2">
                  <span
                    class="text-xs text-gray-500 dark:text-gray-400 font-medium"
                    >{list.total_files} files • {new Date(
                      list.created_at * 1000,
                    ).toLocaleDateString()}</span
                  >
                  <button
                    class="bg-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900 hover:text-blue-700 dark:hover:text-blue-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer shrink-0"
                    onclick={() => verifyList(list.id)}
                  >
                    Verify
                  </button>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <!-- Right Panel: Verification Details -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl shadow border border-gray-100 dark:border-gray-700 p-6 flex flex-col h-full min-h-0"
      >
        {#if activeVerifyId}
          <h2
            class="text-xl font-semibold mb-4 shrink-0 text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700 pb-2"
          >
            Verification Results
          </h2>

          {#if isVerifying && verifyProgress}
            <div
              class="mb-6 bg-blue-50 dark:bg-blue-900/30 p-4 rounded-lg shrink-0"
            >
              <p
                class="text-sm font-medium text-blue-800 dark:text-blue-300 mb-2"
              >
                Verifying files...
              </p>
              <div
                class="w-full bg-blue-200 dark:bg-blue-900/50 rounded-full h-2.5 mb-2"
              >
                <div
                  class="bg-blue-600 dark:bg-blue-500 h-2.5 rounded-full transition-all duration-300"
                  style="width: {verifyProgress.total
                    ? (verifyProgress.processed / verifyProgress.total) * 100
                    : 0}%"
                ></div>
              </div>
              <p class="text-xs text-blue-600 dark:text-blue-400 truncate">
                {verifyProgress.processed} / {verifyProgress.total} - {verifyProgress.current_file}
              </p>
            </div>
          {:else if verifyResult}
            <div
              class="flex-1 overflow-y-auto p-2 border border-gray-200 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-900/50 min-h-0"
            >
              <TreeItem node={verifyResult} defaultOpen={true} />
            </div>
          {:else}
            <div
              class="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500"
            >
              <p>Waiting for verification results...</p>
            </div>
          {/if}
        {:else}
          <div
            class="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500"
          >
            <p>Select Verify on a folder list to see results.</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Duplicate Folder Warning Dialog -->
{#if showDuplicateWarningDialog}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md p-6"
    >
      <h2
        class="text-xl font-bold mb-4 text-orange-600 dark:text-orange-400 flex items-center"
      >
        <svg
          class="w-6 h-6 mr-2"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
          ></path>
        </svg>
        Duplicate Folder Detected
      </h2>

      <p class="text-gray-600 dark:text-gray-300 mb-2">
        The selected folder is already present in your saved folder lists.
      </p>

      <div
        class="p-2 bg-gray-100 dark:bg-gray-700 rounded text-sm text-gray-600 dark:text-gray-300 break-all mb-4"
      >
        {pendingFolderPath}
      </div>

      <p class="text-sm text-gray-500 dark:text-gray-400 mb-6">
        Adding it again will create a new, separate snapshot. Do you want to
        continue?
      </p>

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
          onclick={cancelDuplicateWarning}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-orange-600 hover:bg-orange-700 text-white transition font-medium cursor-pointer"
          onclick={continueToCreate}
        >
          Continue
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Add Folder Dialog -->
{#if showCreateDialog}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md p-6"
    >
      <h2 class="text-2xl font-bold mb-4 text-gray-900 dark:text-white">
        Add Folder List
      </h2>

      <div class="mb-4">
        <span
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >Selected Path</span
        >
        <div
          class="p-2 bg-gray-100 dark:bg-gray-700 rounded text-sm text-gray-600 dark:text-gray-300 break-all"
        >
          {selectedPath}
        </div>
      </div>

      <div class="mb-6">
        <label
          for="folder-name-input"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
          >Name</label
        >
        <input
          id="folder-name-input"
          type="text"
          bind:value={newFolderName}
          disabled={isGenerating}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white disabled:opacity-50"
        />
      </div>

      {#if isGenerating && generateProgress}
        <div class="mb-6">
          <p class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Generating checksums...
          </p>
          <div
            class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5 mb-2"
          >
            <div
              class="bg-blue-600 dark:bg-blue-500 h-2.5 rounded-full transition-all duration-300"
              style="width: {generateProgress.total
                ? (generateProgress.processed / generateProgress.total) * 100
                : 0}%"
            ></div>
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
            {generateProgress.processed} / {generateProgress.total} - {generateProgress.current_file}
          </p>
        </div>
      {/if}

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={() => (showCreateDialog = false)}
          disabled={isGenerating}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={startGeneration}
          disabled={isGenerating || !newFolderName}
        >
          {isGenerating ? "Generating..." : "Save & Generate"}
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Delete Confirmation Dialog -->
{#if showDeleteDialog}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-sm p-6"
    >
      <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">
        Confirm Deletion
      </h2>

      <p class="text-gray-600 dark:text-gray-300 mb-6">
        Are you sure you want to delete the folder list <span
          class="font-semibold text-gray-900 dark:text-white"
          >"{folderToDelete?.name}"</span
        >? This action cannot be undone.
      </p>

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
          onclick={closeDeleteDialog}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-red-600 hover:bg-red-700 text-white transition font-medium cursor-pointer"
          onclick={confirmDelete}
        >
          Delete
        </button>
      </div>
    </div>
  </div>
{/if}
