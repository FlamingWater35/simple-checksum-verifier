<script lang="ts">
  let {
    folderName,
    onCancel,
    onConfirm,
  }: {
    folderName: string;
    onCancel: () => void;
    onConfirm: () => void;
  } = $props();
</script>

<div
  class="fixed inset-0 bg-black/50 dark:bg-black/70 flex items-center justify-center p-4 z-50"
>
  <div
    class="bg-white dark:bg-gray-800 rounded-xl shadow-xl w-full max-w-sm p-6"
  >
    <h2 class="text-xl font-bold mb-4 text-gray-900 dark:text-white">
      Confirm Deletion
    </h2>

    <p class="text-gray-600 dark:text-gray-300 mb-6">
      Are you sure you want to delete the folder list <span
        class="font-semibold text-gray-900 dark:text-white">"{folderName}"</span
      >? This action cannot be undone.
    </p>

    <div class="flex justify-end space-x-3">
      <button
        class="px-4 py-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 transition font-medium cursor-pointer"
        onclick={onCancel}
      >
        Cancel
      </button>
      <button
        class="px-4 py-2 rounded-lg bg-red-600 hover:bg-red-700 text-white transition font-medium cursor-pointer"
        onclick={onConfirm}
      >
        Delete
      </button>
    </div>
  </div>
</div>
