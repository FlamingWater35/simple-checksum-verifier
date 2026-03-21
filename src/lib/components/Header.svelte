<script lang="ts">
  let {
    updateAvailable,
    isBusy,
    onOpenUpdate,
    onOpenSettings,
    onAddFolder,
  }: {
    updateAvailable: boolean;
    isBusy: boolean;
    onOpenUpdate: () => void;
    onOpenSettings: () => void;
    onAddFolder: () => void;
  } = $props();
</script>

<header class="flex justify-between items-center mb-6 shrink-0">
  <h1 class="text-3xl font-bold text-gray-900 dark:text-white">
    Checksum Verifier
  </h1>
  <div class="flex items-center space-x-4">
    {#if updateAvailable}
      <button
        class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300 p-2 rounded-full hover:bg-blue-100 dark:hover:bg-gray-800 transition-colors cursor-pointer relative"
        onclick={onOpenUpdate}
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
      class="text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors cursor-pointer"
      onclick={onOpenSettings}
      disabled={isBusy}
      title="Settings"
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
          d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
        ></path>
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
        ></path>
      </svg>
    </button>

    <button
      class="bg-blue-600 hover:bg-blue-700 text-white px-5 py-2.5 rounded-lg shadow font-medium transition cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
      onclick={onAddFolder}
      disabled={isBusy}
    >
      + Add Folder
    </button>
  </div>
</header>
