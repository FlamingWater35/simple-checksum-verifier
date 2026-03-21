<script lang="ts">
  import type { FolderListSummary } from "$lib/types";

  let {
    activeBackupList,
    onAdd,
    onRemove,
    onClose,
  }: {
    activeBackupList: FolderListSummary;
    onAdd: () => void;
    onRemove: (path: string) => void;
    onClose: () => void;
  } = $props();
</script>

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
              onclick={() => onRemove(backup)}
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
        onclick={onAdd}
      >
        + Add Backup Location
      </button>
      <button
        class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
        onclick={onClose}
      >
        Close
      </button>
    </div>
  </div>
</div>
