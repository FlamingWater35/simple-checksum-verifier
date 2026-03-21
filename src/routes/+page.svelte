<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import type {
    FolderListSummary,
    Progress,
    FullVerifyResult,
    AppSettings,
  } from "$lib/types";

  // Components
  import Header from "$lib/components/Header.svelte";
  import FolderList from "$lib/components/FolderList.svelte";
  import VerificationResult from "$lib/components/VerificationResult.svelte";
  import SettingsDialog from "$lib/components/dialogs/SettingsDialog.svelte";
  import UpdateDialog from "$lib/components/dialogs/UpdateDialog.svelte";
  import ManageBackupsDialog from "$lib/components/dialogs/ManageBackupsDialog.svelte";
  import DuplicateWarningDialog from "$lib/components/dialogs/DuplicateWarningDialog.svelte";
  import CreateFolderDialog from "$lib/components/dialogs/CreateFolderDialog.svelte";
  import DeleteConfirmDialog from "$lib/components/dialogs/DeleteConfirmDialog.svelte";
  import ErrorDialog from "$lib/components/dialogs/ErrorDialog.svelte";

  // Operation Control State
  type OperationType = "none" | "generating" | "verifying" | "rehashing";
  let currentOperation: OperationType = $state("none");
  let isBusy = $derived(currentOperation !== "none");
  let operationProgress: Progress | null = $state(null);

  // Settings State
  let settings: AppSettings = $state({ theme: "auto", algorithm: "sha256" });
  let showSettingsDialog = $state(false);
  let themeQueryMedia: MediaQueryList | null = null;

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

  // Error Dialog State
  let showErrorMessage: string | null = $state(null);

  // Delete Dialog State
  let showDeleteDialog = $state(false);
  let folderToDelete: FolderListSummary | null = $state(null);

  // Manage Backups Dialog State
  let showManageBackupsDialog = $state(false);
  let activeBackupList: FolderListSummary | null = $state(null);

  // Verification & Rehash State
  let verifyResult: FullVerifyResult | null = $state(null);
  let activeVerifyId: string | null = $state(null);
  let selectedVerifyTab: number = $state(0);

  let filteredLists = $derived(
    folderLists.filter(
      (list) =>
        list.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        list.path.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  onMount(async () => {
    settings = await invoke("get_settings");
    applyTheme();

    themeQueryMedia = window.matchMedia("(prefers-color-scheme: dark)");
    themeQueryMedia.addEventListener("change", applyTheme);

    await fetchFolderLists();
    isLoadingFolders = false;

    await checkForUpdates();

    listen<Progress>("operation_progress", (event) => {
      operationProgress = event.payload;
    });
  });

  onDestroy(() => {
    if (themeQueryMedia) {
      themeQueryMedia.removeEventListener("change", applyTheme);
    }
  });

  function applyTheme() {
    if (!window) return;
    localStorage.setItem("app-theme", settings.theme);

    const isDark =
      settings.theme === "dark" ||
      (settings.theme === "auto" &&
        window.matchMedia("(prefers-color-scheme: dark)").matches);

    if (isDark) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }

  async function updateSettings() {
    applyTheme();
    await invoke("save_settings", { settings });
  }

  function showError(msg: string) {
    showErrorMessage = msg;
  }

  function closeError() {
    showErrorMessage = null;
  }

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
        algorithm: settings.algorithm,
      });
      showCreateDialog = false;
      await fetchFolderLists();
    } catch (e) {
      if (e !== "Cancelled") showError("Error: " + e);
    } finally {
      currentOperation = "none";
    }
  }

  async function changeMainPath(list: FolderListSummary, event: Event) {
    if (isBusy) return;

    if (event?.currentTarget instanceof HTMLElement) {
      event.currentTarget.blur();
    }

    const newPath = await invoke<string | null>("select_folder");
    if (newPath) {
      if (list.backups.includes(newPath)) {
        showError(
          "The selected path is already a backup location. Please remove it from backups first.",
        );
        return;
      }
      try {
        await invoke("update_main_path", { id: list.id, newPath });
        await fetchFolderLists();
        if (activeBackupList?.id === list.id) {
          activeBackupList = folderLists.find((l) => l.id === list.id) || null;
        }
      } catch (e) {
        showError("Error: " + e);
      }
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
        try {
          await invoke("update_backups", {
            id: activeBackupList.id,
            backups: newBackups,
          });
          await fetchFolderLists();
          activeBackupList =
            folderLists.find((l) => l.id === activeBackupList!.id) || null;
        } catch (e) {
          showError("Error: " + e);
        }
      } else {
        showError(
          "This folder is already set as the main path or a backup location.",
        );
      }
    }
  }

  async function removeBackupLocation(path: string) {
    if (activeBackupList) {
      const newBackups = activeBackupList.backups.filter((b) => b !== path);
      try {
        await invoke("update_backups", {
          id: activeBackupList.id,
          backups: newBackups,
        });
        await fetchFolderLists();
        activeBackupList =
          folderLists.find((l) => l.id === activeBackupList!.id) || null;
      } catch (e) {
        showError("Error: " + e);
      }
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
      await invoke("rehash_folder", { id, algorithm: settings.algorithm });
      await fetchFolderLists();
    } catch (e) {
      if (e !== "Cancelled") showError("Error: " + e);
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
        algorithm: settings.algorithm,
      });
    } catch (e) {
      if (e !== "Cancelled") showError("Error: " + e);
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
      try {
        await invoke("delete_folder_list", { id: folderToDelete.id });
        await fetchFolderLists();
        if (activeVerifyId === folderToDelete.id) {
          verifyResult = null;
          activeVerifyId = null;
        }
      } catch (e) {
        showError("Error: " + e);
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
      if (showErrorMessage) closeError();
      else if (showCreateDialog && currentOperation !== "generating")
        showCreateDialog = false;
      else if (showDuplicateWarningDialog) cancelDuplicateWarning();
      else if (showDeleteDialog) closeDeleteDialog();
      else if (showManageBackupsDialog) showManageBackupsDialog = false;
      else if (showUpdateDialog) showUpdateDialog = false;
      else if (showSettingsDialog) showSettingsDialog = false;
    }
  }}
/>

<div
  class="h-screen w-full overflow-hidden bg-gray-50 dark:bg-gray-900 text-gray-800 dark:text-gray-200 p-6 flex flex-col font-sans transition-colors duration-200"
>
  <div class="max-w-6xl mx-auto w-full flex-1 flex flex-col min-h-0">
    <Header
      {updateAvailable}
      {isBusy}
      onOpenUpdate={() => (showUpdateDialog = true)}
      onOpenSettings={() => (showSettingsDialog = true)}
      onAddFolder={openFolderPicker}
    />

    <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-8 min-h-0">
      <FolderList
        bind:searchQuery
        {isBusy}
        {isLoadingFolders}
        {filteredLists}
        {activeVerifyId}
        {settings}
        onChangeMainPath={changeMainPath}
        onRequestDelete={requestDelete}
        onOpenManageBackups={openManageBackups}
        onRehashList={rehashList}
        onVerifyList={verifyList}
      />

      <VerificationResult
        {activeVerifyId}
        {currentOperation}
        {operationProgress}
        {verifyResult}
        bind:selectedVerifyTab
        onCancelOperation={cancelOperation}
      />
    </div>
  </div>
</div>

{#if showSettingsDialog}
  <SettingsDialog
    bind:settings
    onClose={() => (showSettingsDialog = false)}
    onUpdate={updateSettings}
  />
{/if}

{#if showUpdateDialog}
  <UpdateDialog
    {currentVersion}
    {latestVersion}
    onClose={() => (showUpdateDialog = false)}
    onRelease={openReleasePage}
  />
{/if}

{#if showManageBackupsDialog && activeBackupList}
  <ManageBackupsDialog
    {activeBackupList}
    onClose={() => (showManageBackupsDialog = false)}
    onAdd={addBackupLocation}
    onRemove={removeBackupLocation}
  />
{/if}

{#if showDuplicateWarningDialog}
  <DuplicateWarningDialog
    {pendingFolderPath}
    onCancel={cancelDuplicateWarning}
    onContinue={continueToCreate}
  />
{/if}

{#if showCreateDialog}
  <CreateFolderDialog
    {selectedPath}
    bind:newFolderName
    {currentOperation}
    {operationProgress}
    onCancel={() => (showCreateDialog = false)}
    onGenerate={startGeneration}
    onStop={cancelOperation}
  />
{/if}

{#if showDeleteDialog && folderToDelete}
  <DeleteConfirmDialog
    folderName={folderToDelete.name}
    onCancel={closeDeleteDialog}
    onConfirm={confirmDelete}
  />
{/if}

{#if showErrorMessage}
  <ErrorDialog message={showErrorMessage} onClose={closeError} />
{/if}
