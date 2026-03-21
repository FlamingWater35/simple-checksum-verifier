<script lang="ts">
  import type { Progress } from "$lib/types";

  let {
    selectedPath,
    newFolderName = $bindable(),
    currentOperation,
    operationProgress,
    onCancel,
    onGenerate,
    onStop,
  }: {
    selectedPath: string;
    newFolderName: string;
    currentOperation: string;
    operationProgress: Progress | null;
    onCancel: () => void;
    onGenerate: () => void;
    onStop: () => void;
  } = $props();
</script>

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
              onclick={onStop}
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
        onclick={onCancel}
        disabled={currentOperation === "generating"}
      >
        Cancel
      </button>
      <button
        class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium disabled:opacity-50 cursor-pointer"
        onclick={onGenerate}
        disabled={currentOperation === "generating" || !newFolderName}
      >
        {currentOperation === "generating"
          ? "Generating..."
          : "Save & Generate"}
      </button>
    </div>
  </div>
</div>
