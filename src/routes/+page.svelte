<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TreeItem from "./TreeItem.svelte";
  import type {
    FolderListSummary,
    Progress,
    FullVerifyResult,
  } from "$lib/types";

  // Operation Control State
  type OperationType = "none" | "generating" | "verifying" | "rehashing";
  let currentOperation: OperationType = $state("none");
  let isBusy = $derived(currentOperation !== "none");
  let operationProgress: Progress | null = $state(null);

  // General State
  let folderLists: FolderListSummary[] = $state([]);
  let searchQuery: string = $state("");
  let isLoadingFolders = $state(true);

  // App Update State
  let currentVersion: string = $state("");
  let latestVersion: string = $state("");
  let releaseUrl: string = $state("");
  let updateAvailable: boolean = $state(false);
  let showUpdateDialog: boolean = $state(false);

  // Create Dialog State
  let showCreateDialog = $state(false);
  let selectedPath = $state("");
  let newFolderName = $state("");

  // Duplicate Warning Dialog State
  let showDuplicateWarningDialog = $state(false);
  let pendingFolderPath = $state("");

  // Delete Dialog State
  let showDeleteDialog = $state(false);
  let folderToDelete: FolderListSummary | null = $state(null);

  // Manage Backups Dialog State
  let showManageBackupsDialog = $state(false);
  let activeBackupList: FolderListSummary | null = $state(null);

  // Verification & Rehash State
  let verifyResult: FullVerifyResult | null = $state(null);
  let activeVerifyId: string | null = $state(null);
  let selectedVerifyTab: number = $state(0); // 0 = main, 1+ = backups

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

    await checkForUpdates();

    listen<Progress>("operation_progress", (event) => {
      operationProgress = event.payload;
    });
  });

  async function checkForUpdates() {
    try {
      currentVersion = await invoke("get_app_version");
      const res = await fetch(
        "https://api.github.com/repos/FlamingWater35/simple-checksum-verifier/releases/latest",
      );

      if (res.ok) {
        const data = await res.json();
        const tag = data.tag_name || "";
        latestVersion = tag.replace(/^v/, "");
        releaseUrl = data.html_url;

        if (latestVersion && currentVersion) {
          const curParts = currentVersion.split(".").map(Number);
          const latParts = latestVersion.split(".").map(Number);

          for (let i = 0; i < Math.max(curParts.length, latParts.length); i++) {
            const pCur = curParts[i] || 0;
            const pLat = latParts[i] || 0;
            if (pLat > pCur) {
              updateAvailable = true;
              break;
            } else if (pLat < pCur) {
              break;
            }
          }
        }
      }
    } catch (e) {
      console.warn("Failed to check for updates:", e);
    }
  }

  async function openReleasePage() {
    if (releaseUrl) {
      await invoke("open_url", { url: releaseUrl });
      showUpdateDialog = false;
    }
  }

  async function fetchFolderLists() {
    folderLists = await invoke("get_folder_lists");
  }

  async function openFolderPicker() {
    if (isBusy) return;
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

  async function cancelOperation() {
    await invoke("cancel_operation");
  }

  async function startGeneration() {
    if (!selectedPath || !newFolderName || isBusy) return;
    currentOperation = "generating";
    operationProgress = {
      total: 0,
      processed: 0,
      current_file: "Starting...",
      current_location: "Main Folder",
    };

    try {
      await invoke("generate_checksums", {
        name: newFolderName,
        targetPath: selectedPath,
      });
      showCreateDialog = false;
      await fetchFolderLists();
    } catch (e) {
      if (e !== "Cancelled") alert("Error: " + e);
    } finally {
      currentOperation = "none";
    }
  }

  function openManageBackups(list: FolderListSummary) {
    if (isBusy) return;
    activeBackupList = list;
    showManageBackupsDialog = true;
  }

  async function addBackupLocation() {
    const path = await invoke<string | null>("select_folder");
    if (path && activeBackupList) {
      if (
        !activeBackupList.backups.includes(path) &&
        path !== activeBackupList.path
      ) {
        const newBackups = [...activeBackupList.backups, path];
        await invoke("update_backups", {
          id: activeBackupList.id,
          backups: newBackups,
        });
        await fetchFolderLists();
        activeBackupList =
          folderLists.find((l) => l.id === activeBackupList!.id) || null;
      } else {
        alert(
          "This folder is already set as the main path or a backup location.",
        );
      }
    }
  }

  async function removeBackupLocation(path: string) {
    if (activeBackupList) {
      const newBackups = activeBackupList.backups.filter((b) => b !== path);
      await invoke("update_backups", {
        id: activeBackupList.id,
        backups: newBackups,
      });
      await fetchFolderLists();
      activeBackupList =
        folderLists.find((l) => l.id === activeBackupList!.id) || null;
    }
  }

  async function rehashList(id: string) {
    if (isBusy) return;
    activeVerifyId = id;
    verifyResult = null;
    selectedVerifyTab = 0;
    currentOperation = "rehashing";
    operationProgress = {
      total: 0,
      processed: 0,
      current_file: "Starting...",
      current_location: "Main Folder",
    };

    try {
      await invoke("rehash_folder", { id });
      await fetchFolderLists();
    } catch (e) {
      if (e !== "Cancelled") alert("Error: " + e);
    } finally {
      currentOperation = "none";
    }
  }

  async function verifyList(id: string) {
    if (isBusy) return;
    activeVerifyId = id;
    verifyResult = null;
    selectedVerifyTab = 0;
    currentOperation = "verifying";
    operationProgress = {
      total: 0,
      processed: 0,
      current_file: "Starting...",
      current_location: "Starting...",
    };

    try {
      verifyResult = await invoke<FullVerifyResult>("verify_folder_contents", {
        id,
      });
    } catch (e) {
      if (e !== "Cancelled") alert("Error: " + e);
    } finally {
      currentOperation = "none";
    }
  }

  function requestDelete(list: FolderListSummary) {
    if (isBusy) return;
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
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === "Escape") {
      if (showCreateDialog && currentOperation !== "generating")
        showCreateDialog = false;
      else if (showDuplicateWarningDialog) cancelDuplicateWarning();
      else if (showDeleteDialog) closeDeleteDialog();
      else if (showManageBackupsDialog) showManageBackupsDialog = false;
      else if (showUpdateDialog) showUpdateDialog = false;
    }
  }}
/>

<div
  class="h-screen w-full overflow-hidden bg-gray-50 dark:bg-gray-900 text-gray-800 dark:text-gray-200 p-6 flex flex-col font-sans transition-colors duration-200"
>
  <div class="max-w-6xl mx-auto w-full flex-1 flex flex-col min-h-0">
    <!-- Header -->
    <header class="flex justify-between items-center mb-6 shrink-0">
      <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
        Checksum Verifier
      </h1>
      <div class="flex items-center">
        {#if updateAvailable}
          <button
            class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 mr-4 p-2 rounded-full hover:bg-blue-100 dark:hover:bg-gray-800 transition-colors cursor-pointer relative"
            onclick={() => (showUpdateDialog = true)}
            title="Update Available"
          >
            <svg
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
              ></path>
            </svg>
            <span class="absolute top-1.5 right-1.5 flex h-2.5 w-2.5">
              <span
                class="animate-ping absolute inline-flex h-full w-full rounded-full bg-blue-400 opacity-75"
              ></span>
              <span
                class="relative inline-flex rounded-full h-2.5 w-2.5 bg-blue-500"
              ></span>
            </span>
          </button>
        {/if}
        <button
          class="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2.5 rounded-lg shadow font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
          onclick={openFolderPicker}
          disabled={isBusy}
        >
          + Add Folder
        </button>
      </div>
    </header>

    <!-- Main Content Grid -->
    <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-8 min-h-0">
      <!-- Left Panel: Folder Lists -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl shadow border border-gray-100 dark:border-gray-700 p-6 flex flex-col h-full min-h-0 relative"
      >
        {#if isBusy}
          <div
            class="absolute inset-0 bg-white/40 dark:bg-gray-900/40 z-10 rounded-xl"
          ></div>
        {/if}

        <div class="mb-4 shrink-0 z-20">
          <input
            type="text"
            placeholder="Search folders..."
            aria-label="Search folders"
            bind:value={searchQuery}
            disabled={isBusy}
            class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50"
          />
        </div>

        <div class="flex-1 overflow-y-auto pr-1 min-h-0 relative z-20">
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
            <div class="space-y-3 p-1">
              {#each filteredLists as list}
                <div
                  class="p-4 border border-gray-200 dark:border-gray-700 rounded-lg hover:shadow-md transition bg-gray-50 dark:bg-gray-800/50 flex flex-col {activeVerifyId ===
                  list.id
                    ? 'ring-2 ring-blue-500'
                    : ''}"
                >
                  <div class="flex justify-between items-start mb-2 gap-2">
                    <div class="min-w-0 flex-1">
                      <h3
                        class="font-semibold text-lg text-gray-900 dark:text-white truncate"
                        title={list.name}
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
                      class="text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 cursor-pointer transition shrink-0 disabled:opacity-30 disabled:cursor-not-allowed"
                      onclick={() => requestDelete(list)}
                      title="Delete"
                      disabled={isBusy}
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

                  <div
                    class="flex flex-wrap justify-between items-center gap-3 mt-2"
                  >
                    <span
                      class="text-xs text-gray-500 dark:text-gray-400 font-medium whitespace-nowrap"
                    >
                      {list.total_files.toLocaleString()} files • {new Date(
                        list.created_at * 1000,
                      ).toLocaleDateString(undefined, { dateStyle: "medium" })}
                    </span>
                    <div class="flex flex-wrap gap-2">
                      <button
                        class="bg-gray-200 dark:bg-gray-700 hover:bg-orange-100 dark:hover:bg-orange-900 hover:text-orange-700 dark:hover:text-orange-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                        onclick={() => openManageBackups(list)}
                        disabled={isBusy}
                        title="Manage Backups"
                      >
                        Backups ({list.backups?.length || 0})
                      </button>
                      <button
                        class="bg-gray-200 dark:bg-gray-700 hover:bg-green-100 dark:hover:bg-green-900 hover:text-green-700 dark:hover:text-green-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                        onclick={() => rehashList(list.id)}
                        disabled={isBusy}
                        title="Update Checksums"
                      >
                        Update
                      </button>
                      <button
                        class="bg-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900 hover:text-blue-700 dark:hover:text-blue-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                        onclick={() => verifyList(list.id)}
                        disabled={isBusy}
                      >
                        Verify
                      </button>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>

      <!-- Right Panel: Verification / Update Details -->
      <div
        class="bg-white dark:bg-gray-800 rounded-xl shadow border border-gray-100 dark:border-gray-700 p-6 flex flex-col h-full min-h-0"
      >
        {#if activeVerifyId}
          <h2
            class="text-xl font-semibold mb-4 shrink-0 text-gray-900 dark:text-white border-b border-gray-200 dark:border-gray-700 pb-2"
          >
            {#if currentOperation === "rehashing"}
              Snapshot Update
            {:else}
              Verification Results
            {/if}
          </h2>

          {#if (currentOperation === "verifying" || currentOperation === "rehashing") && operationProgress}
            <div
              class="mb-6 bg-blue-50 dark:bg-blue-900/30 p-4 rounded-lg shrink-0 border border-blue-100 dark:border-blue-800"
            >
              <div class="flex justify-between items-center mb-2">
                <p class="text-sm font-medium text-blue-800 dark:text-blue-300">
                  {currentOperation === "rehashing"
                    ? "Updating checksums..."
                    : `Verifying ${operationProgress.current_location}...`}
                </p>
                <div class="flex items-center space-x-3">
                  <span
                    class="text-xs font-semibold text-blue-600 dark:text-blue-400"
                  >
                    {operationProgress.total > 0
                      ? Math.round(
                          (operationProgress.processed /
                            operationProgress.total) *
                            100,
                        )
                      : 0}%
                  </span>
                  <button
                    class="bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-400 hover:bg-red-200 dark:hover:bg-red-800 px-3 py-1 rounded text-xs font-medium transition cursor-pointer"
                    onclick={cancelOperation}
                  >
                    Stop
                  </button>
                </div>
              </div>
              <div
                class="w-full bg-blue-200 dark:bg-blue-900/50 rounded-full h-2.5 mb-2"
              >
                <div
                  class="bg-blue-600 dark:bg-blue-500 h-2.5 rounded-full transition-all duration-300"
                  style="width: {operationProgress.total
                    ? (operationProgress.processed / operationProgress.total) *
                      100
                    : 0}%"
                ></div>
              </div>
              <p class="text-xs text-blue-600 dark:text-blue-400 truncate">
                {operationProgress.processed.toLocaleString()} / {operationProgress.total.toLocaleString()}
                - {operationProgress.current_file}
              </p>
            </div>
          {:else if verifyResult}
            <div class="flex flex-col flex-1 min-h-0">
              <!-- Tab Bar for Locations -->
              <div
                class="flex border-b border-gray-200 dark:border-gray-700 mb-2 overflow-x-auto hide-scrollbar shrink-0"
              >
                <button
                  class="px-4 py-2 font-medium text-sm transition-colors cursor-pointer whitespace-nowrap {selectedVerifyTab ===
                  0
                    ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
                  onclick={() => (selectedVerifyTab = 0)}
                >
                  Main Folder
                </button>
                {#each verifyResult.backups as backup, i}
                  <button
                    class="px-4 py-2 font-medium text-sm transition-colors cursor-pointer whitespace-nowrap max-w-37.5 truncate {selectedVerifyTab ===
                    i + 1
                      ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                      : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300'}"
                    onclick={() => (selectedVerifyTab = i + 1)}
                    title={backup.path}
                  >
                    Backup {i + 1}
                  </button>
                {/each}
              </div>

              <!-- Tree Output -->
              <div
                class="flex-1 overflow-y-auto p-2 border border-gray-200 dark:border-gray-700 rounded bg-gray-50 dark:bg-gray-900/50 min-h-0"
              >
                {#if selectedVerifyTab === 0}
                  <TreeItem node={verifyResult.main} defaultOpen={true} />
                {:else if verifyResult.backups[selectedVerifyTab - 1]}
                  <TreeItem
                    node={verifyResult.backups[selectedVerifyTab - 1].tree}
                    defaultOpen={true}
                  />
                {/if}
              </div>
            </div>
          {:else}
            <div
              class="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500"
            >
              <p>Ready. Select Verify or Update on a folder list.</p>
            </div>
          {/if}
        {:else}
          <div
            class="flex-1 flex items-center justify-center text-gray-400 dark:text-gray-500"
          >
            <p>Select Verify or Update on a folder list to see results.</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>

<!-- Update Available Dialog -->
{#if showUpdateDialog}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-sm p-6"
    >
      <h2
        class="text-xl font-bold mb-4 text-gray-900 dark:text-white flex items-center"
      >
        <svg
          class="w-6 h-6 mr-2 text-blue-500"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M13 10V3L4 14h7v7l9-11h-7z"
          ></path>
        </svg>
        Update Available
      </h2>

      <div class="mb-6 text-gray-600 dark:text-gray-300 space-y-3">
        <p class="text-sm">
          A new version of Simple Checksum Verifier is available for download.
        </p>

        <div class="space-y-2 text-sm mt-4">
          <div
            class="flex justify-between items-center bg-gray-50 dark:bg-gray-700 p-2.5 rounded-lg border border-gray-200 dark:border-gray-600"
          >
            <span class="font-medium text-gray-600 dark:text-gray-400"
              >Installed Version:</span
            >
            <span class="font-mono text-gray-800 dark:text-gray-200"
              >v{currentVersion}</span
            >
          </div>
          <div
            class="flex justify-between items-center bg-green-50 dark:bg-green-900/30 p-2.5 rounded-lg border border-green-200 dark:border-green-800"
          >
            <span class="font-medium text-green-700 dark:text-green-400"
              >Latest Version:</span
            >
            <span class="font-mono font-bold text-green-700 dark:text-green-400"
              >v{latestVersion}</span
            >
          </div>
        </div>
      </div>

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
          onclick={() => (showUpdateDialog = false)}
        >
          Close
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium cursor-pointer"
          onclick={openReleasePage}
        >
          View Release
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Manage Backups Dialog -->
{#if showManageBackupsDialog && activeBackupList}
  <div
    class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  >
    <div
      class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-lg p-6 flex flex-col max-h-[80vh]"
    >
      <h2 class="text-xl font-bold mb-2 text-gray-900 dark:text-white">
        Manage Backups
      </h2>
      <p
        class="text-sm text-gray-500 dark:text-gray-400 mb-4 truncate"
        title={activeBackupList.path}
      >
        For: {activeBackupList.name}
      </p>

      <div
        class="flex-1 overflow-y-auto mb-4 border border-gray-200 dark:border-gray-700 rounded-lg p-2 bg-gray-50 dark:bg-gray-900/50 space-y-2"
      >
        {#if activeBackupList.backups && activeBackupList.backups.length > 0}
          {#each activeBackupList.backups as backup}
            <div
              class="flex justify-between items-center p-2 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded shadow-sm"
            >
              <span
                class="text-sm text-gray-700 dark:text-gray-300 truncate mr-2"
                title={backup}>{backup}</span
              >
              <button
                class="text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 cursor-pointer transition shrink-0"
                onclick={() => removeBackupLocation(backup)}
                title="Remove Backup"
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
          {/each}
        {:else}
          <div
            class="h-full flex items-center justify-center text-gray-400 dark:text-gray-500 text-sm py-4"
          >
            No backup locations configured.
          </div>
        {/if}
      </div>

      <div class="flex justify-between space-x-3 mt-auto">
        <button
          class="px-4 py-2 rounded-lg bg-green-600 hover:bg-green-700 text-white transition font-medium cursor-pointer"
          onclick={addBackupLocation}
        >
          + Add Backup Location
        </button>
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
          onclick={() => (showManageBackupsDialog = false)}
        >
          Close
        </button>
      </div>
    </div>
  </div>
{/if}

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
          disabled={currentOperation === "generating"}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white disabled:opacity-50"
        />
      </div>

      {#if currentOperation === "generating" && operationProgress}
        <div
          class="mb-6 p-4 bg-gray-50 dark:bg-gray-700/50 rounded-lg border border-gray-200 dark:border-gray-600"
        >
          <div class="flex justify-between items-center mb-2">
            <p class="text-sm font-medium text-gray-700 dark:text-gray-300">
              Generating checksums...
            </p>
            <div class="flex items-center space-x-3">
              <span
                class="text-xs font-semibold text-blue-600 dark:text-blue-400"
              >
                {operationProgress.total > 0
                  ? Math.round(
                      (operationProgress.processed / operationProgress.total) *
                        100,
                    )
                  : 0}%
              </span>
              <button
                class="bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-400 hover:bg-red-200 dark:hover:bg-red-800 px-3 py-1 rounded text-xs font-medium transition cursor-pointer"
                onclick={cancelOperation}
              >
                Stop
              </button>
            </div>
          </div>
          <div
            class="w-full bg-gray-200 dark:bg-gray-600 rounded-full h-2.5 mb-2"
          >
            <div
              class="bg-blue-600 dark:bg-blue-500 h-2.5 rounded-full transition-all duration-300"
              style="width: {operationProgress.total
                ? (operationProgress.processed / operationProgress.total) * 100
                : 0}%"
            ></div>
          </div>
          <p class="text-xs text-gray-500 dark:text-gray-400 truncate">
            {operationProgress.processed.toLocaleString()} / {operationProgress.total.toLocaleString()}
            - {operationProgress.current_file}
          </p>
        </div>
      {/if}

      <div class="flex justify-end space-x-3">
        <button
          class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={() => (showCreateDialog = false)}
          disabled={currentOperation === "generating"}
        >
          Cancel
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium disabled:opacity-50 cursor-pointer"
          onclick={startGeneration}
          disabled={currentOperation === "generating" || !newFolderName}
        >
          {currentOperation === "generating"
            ? "Generating..."
            : "Save & Generate"}
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

<style>
  :global(html) {
    color-scheme: light dark;
  }

  .hide-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .hide-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  :global(::-webkit-scrollbar) {
    width: 8px;
    height: 8px;
  }

  :global(::-webkit-scrollbar-track) {
    background: transparent;
  }

  :global(::-webkit-scrollbar-thumb) {
    background-color: #cbd5e1;
    border-radius: 4px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background-color: #94a3b8;
  }

  @media (prefers-color-scheme: dark) {
    :global(::-webkit-scrollbar-thumb) {
      background-color: #475569;
    }
    :global(::-webkit-scrollbar-thumb:hover) {
      background-color: #64748b;
    }
  }
</style>
