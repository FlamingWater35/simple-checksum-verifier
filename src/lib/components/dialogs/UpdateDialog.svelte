<script lang="ts">
  import { fade, fly } from "svelte/transition";

  let {
    currentVersion,
    latestVersion,
    onClose,
    onRelease,
  }: {
    currentVersion: string;
    latestVersion: string;
    onClose: () => void;
    onRelease: () => void;
  } = $props();
</script>

<div
  class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
  transition:fade={{ duration: 150 }}
>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-sm p-6"
    transition:fly={{ y: 20, duration: 250 }}
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
        class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer shadow-sm"
        onclick={onClose}
      >
        Close
      </button>
      <button
        class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-700 text-white transition font-medium cursor-pointer shadow-sm"
        onclick={onRelease}
      >
        View Release
      </button>
    </div>
  </div>
</div>
