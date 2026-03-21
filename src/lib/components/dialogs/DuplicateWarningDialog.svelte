<script lang="ts">
  import { fade, fly } from "svelte/transition";

  let {
    pendingFolderPath,
    onCancel,
    onContinue,
  }: {
    pendingFolderPath: string;
    onCancel: () => void;
    onContinue: () => void;
  } = $props();
</script>

<div
  class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  transition:fade={{ duration: 150 }}
>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md p-6"
    transition:fly={{ y: 20, duration: 250 }}
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
        class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer shadow-sm"
        onclick={onCancel}
      >
        Cancel
      </button>
      <button
        class="px-4 py-2 rounded-lg bg-orange-600 hover:bg-orange-700 text-white transition font-medium cursor-pointer shadow-sm"
        onclick={onContinue}
      >
        Continue
      </button>
    </div>
  </div>
</div>
