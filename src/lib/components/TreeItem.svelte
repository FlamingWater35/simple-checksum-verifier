<script lang="ts">
  import type { TreeNode } from "$lib/types";
  import TreeItem from "./TreeItem.svelte";

  let { node, defaultOpen = false }: { node: TreeNode; defaultOpen?: boolean } =
    $props();

  let isOpen = $state((() => defaultOpen)());

  function toggle() {
    if (node.node_type === "Directory") {
      isOpen = !isOpen;
    }
  }

  function getStatusColor(status: string) {
    switch (status) {
      case "Match":
        return "text-green-700 bg-green-50 border-green-200 dark:text-green-300 dark:bg-green-900/30 dark:border-green-800";
      case "Mismatch":
        return "text-red-700 bg-red-50 border-red-200 dark:text-red-300 dark:bg-red-900/30 dark:border-red-800";
      case "Modified":
        return "text-yellow-700 bg-yellow-50 border-yellow-200 dark:text-yellow-300 dark:bg-yellow-900/30 dark:border-yellow-800";
      case "Missing":
        return "text-orange-700 bg-orange-50 border-orange-200 dark:text-orange-300 dark:bg-orange-900/30 dark:border-orange-800";
      case "Untracked":
        return "text-purple-700 bg-purple-50 border-purple-200 dark:text-purple-300 dark:bg-purple-900/30 dark:border-purple-800";
      default:
        return "text-gray-700 bg-gray-50 border-gray-200 dark:text-gray-300 dark:bg-gray-800 dark:border-gray-700";
    }
  }

  function getStatusDot(status: string) {
    switch (status) {
      case "Match":
        return "bg-green-500 dark:bg-green-400";
      case "Mismatch":
        return "bg-red-500 dark:bg-red-400";
      case "Modified":
        return "bg-yellow-500 dark:bg-yellow-400";
      case "Missing":
        return "bg-orange-500 dark:bg-orange-400";
      case "Untracked":
        return "bg-purple-500 dark:bg-purple-400";
      default:
        return "bg-gray-500 dark:bg-gray-400";
    }
  }
</script>

<div class="font-mono text-[13px] select-none">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="flex items-center py-1.5 px-2 hover:bg-gray-100 dark:hover:bg-gray-700/50 rounded transition-colors {node.node_type ===
    'Directory'
      ? 'cursor-pointer font-medium'
      : 'ml-4'}"
    onclick={toggle}
  >
    {#if node.node_type === "Directory"}
      <span
        class="mr-1 w-4 text-gray-500 dark:text-gray-400 inline-block text-center shrink-0"
      >
        {isOpen ? "▼" : "▶"}
      </span>
      <svg
        class="w-4 h-4 mr-2 text-blue-500 dark:text-blue-400 shrink-0"
        fill="currentColor"
        viewBox="0 0 20 20"
      >
        <path d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H2V6z" />
        <path d="M2 11v3a2 2 0 002 2h12a2 2 0 002-2v-3H2z" />
      </svg>
    {:else}
      <svg
        class="w-4 h-4 mr-2 text-gray-400 dark:text-gray-500 shrink-0"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
        />
      </svg>
    {/if}

    <span class="flex-1 truncate text-gray-700 dark:text-gray-200">
      {node.name}
    </span>

    <span
      class="ml-2 px-2 py-0.5 rounded text-[11px] font-sans border {getStatusColor(
        node.status,
      )} flex items-center shadow-sm"
    >
      <span class="w-2 h-2 rounded-full mr-1 {getStatusDot(node.status)}"
      ></span>
      {node.status}
    </span>
  </div>

  {#if isOpen && node.children}
    <div class="ml-2.5 pl-3 border-l border-gray-300 dark:border-gray-600">
      {#each node.children as child}
        <TreeItem node={child} />
      {/each}
    </div>
  {/if}
</div>
