<script lang="ts">
  import type { Progress, FullVerifyResult } from "$lib/types";
  import TreeItem from "./TreeItem.svelte";

  let {
    activeVerifyId,
    currentOperation,
    operationProgress,
    verifyResult,
    selectedVerifyTab = $bindable(),
    onCancelOperation,
  }: {
    activeVerifyId: string | null;
    currentOperation: string;
    operationProgress: Progress | null;
    verifyResult: FullVerifyResult | null;
    selectedVerifyTab: number;
    onCancelOperation: () => void;
  } = $props();
</script>

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
                    (operationProgress.processed / operationProgress.total) *
                      100,
                  )
                : 0}%
            </span>
            <button
              class="bg-red-100 dark:bg-red-900/50 text-red-700 dark:text-red-400 hover:bg-red-200 dark:hover:bg-red-800 px-3 py-1 rounded text-xs font-medium transition cursor-pointer"
              onclick={onCancelOperation}
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
              ? (operationProgress.processed / operationProgress.total) * 100
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
