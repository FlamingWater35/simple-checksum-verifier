<script lang="ts">
  let {
    message,
    onClose,
  }: {
    message: string;
    onClose: () => void;
  } = $props();
</script>

<div
  class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-60"
>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-sm p-6 border-l-4 border-red-500"
  >
    <h2
      class="text-xl font-bold mb-3 text-red-600 dark:text-red-400 flex items-center"
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
          d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        ></path>
      </svg>
      Error
    </h2>

    <p class="text-gray-600 dark:text-gray-300 mb-6 text-sm wrap-break-word">
      {message}
    </p>

    <div class="flex justify-end">
      <button
        class="px-4 py-2 rounded-lg bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600 text-gray-800 dark:text-gray-200 transition font-medium cursor-pointer"
        onclick={onClose}
      >
        OK
      </button>
    </div>
  </div>
</div>
