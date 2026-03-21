<script lang="ts">
  import type { AppSettings } from "$lib/types";

  let {
    settings = $bindable(),
    onClose,
    onUpdate,
  }: {
    settings: AppSettings;
    onClose: () => void;
    onUpdate: () => void;
  } = $props();
</script>

<div
  class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-md p-6"
  >
    <h2
      class="text-xl font-bold mb-4 text-gray-900 dark:text-white flex items-center"
    >
      <svg
        class="w-6 h-6 mr-2 text-gray-500"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
        ></path>
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
        ></path>
      </svg>
      App Settings
    </h2>

    <div class="space-y-6 mb-6">
      <div>
        <label
          for="theme-select"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >Theme</label
        >
        <select
          id="theme-select"
          bind:value={settings.theme}
          onchange={onUpdate}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
        >
          <option value="auto">Automatic (System)</option>
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
      </div>

      <div>
        <label
          for="algo-select"
          class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >Hashing Algorithm</label
        >
        <select
          id="algo-select"
          bind:value={settings.algorithm}
          onchange={onUpdate}
          class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
        >
          <option value="sha256">SHA-256 (Default)</option>
          <option value="blake2b">BLAKE2b (Faster on 64-bit CPUs)</option>
        </select>
        <p
          class="mt-2 text-xs text-gray-500 dark:text-gray-400 leading-relaxed"
        >
          Note: Changing the algorithm applies to future operations. Folders
          without hashes for the newly selected algorithm will display a warning
          badge until you click Update.
        </p>
      </div>
    </div>

    <div class="flex justify-end">
      <button
        class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-800 dark:text-gray-200 transition font-medium cursor-pointer"
        onclick={onClose}
      >
        Close
      </button>
    </div>
  </div>
</div>
