<script lang="ts">
  import type { FolderListSummary, AppSettings } from "$lib/types";

  let {
    searchQuery = $bindable(),
    isBusy,
    isLoadingFolders,
    filteredLists,
    activeVerifyId,
    settings,
    onChangeMainPath,
    onRequestDelete,
    onOpenManageBackups,
    onRehashList,
    onVerifyList,
  }: {
    searchQuery: string;
    isBusy: boolean;
    isLoadingFolders: boolean;
    filteredLists: FolderListSummary[];
    activeVerifyId: string | null;
    settings: AppSettings;
    onChangeMainPath: (list: FolderListSummary, e: Event) => void;
    onRequestDelete: (list: FolderListSummary) => void;
    onOpenManageBackups: (list: FolderListSummary) => void;
    onRehashList: (id: string) => void;
    onVerifyList: (id: string) => void;
  } = $props();
</script>

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

                <div class="flex items-center group mt-1">
                  <p class="text-sm text-gray-500 dark:text-gray-400 break-all">
                    {list.path}
                  </p>
                  <button
                    class="ml-2 text-blue-500 hover:text-blue-700 dark:hover:text-blue-400 opacity-0 group-hover:opacity-100 transition-opacity p-1 focus-visible:opacity-100 cursor-pointer disabled:opacity-0"
                    onclick={(e) => onChangeMainPath(list, e)}
                    title="Change Main Folder Path"
                    disabled={isBusy}
                  >
                    <svg
                      class="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z"
                      />
                    </svg>
                  </button>
                </div>

                {#if !list.available_algorithms.includes(settings.algorithm) && list.total_files > 0}
                  <div
                    class="text-orange-600 dark:text-orange-400 text-xs mt-1.5 flex items-center font-medium"
                  >
                    <svg
                      class="w-3.5 h-3.5 mr-1"
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
                    Requires Update ({settings.algorithm.toUpperCase()})
                  </div>
                {/if}
              </div>
              <button
                class="text-red-500 hover:text-red-700 dark:hover:text-red-400 p-1 cursor-pointer transition shrink-0 disabled:opacity-30 disabled:cursor-not-allowed"
                onclick={() => onRequestDelete(list)}
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

            <div class="flex flex-wrap justify-between items-center gap-3 mt-2">
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
                  onclick={() => onOpenManageBackups(list)}
                  disabled={isBusy}
                  title="Manage Backups"
                >
                  Backups ({list.backups?.length || 0})
                </button>
                <button
                  class="bg-gray-200 dark:bg-gray-700 hover:bg-green-100 dark:hover:bg-green-900 hover:text-green-700 dark:hover:text-green-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                  onclick={() => onRehashList(list.id)}
                  disabled={isBusy}
                  title="Update Checksums"
                >
                  Update
                </button>
                <button
                  class="bg-gray-200 dark:bg-gray-700 hover:bg-blue-100 dark:hover:bg-blue-900 hover:text-blue-700 dark:hover:text-blue-300 text-gray-800 dark:text-gray-200 px-3 py-1 rounded text-sm font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                  onclick={() => onVerifyList(list.id)}
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
