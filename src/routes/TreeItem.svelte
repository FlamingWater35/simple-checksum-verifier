<script lang="ts">
  import type { TreeNode } from "$lib/types";
  import TreeItem from "./TreeItem.svelte";

  let { node, defaultOpen = false }: { node: TreeNode; defaultOpen?: boolean } =
    $props();

  // Use an IIFE closure to suppress the `state_referenced_locally` warning
  // This tells Svelte we intentionally only want to capture the initial value.
  let isOpen = $state((() => defaultOpen)());

  function toggle() {
    if (node.node_type === "Directory") {
      isOpen = !isOpen;
    }
  }

  // Color coding mapped to the backend statuses
  function getStatusColor(status: string) {
    switch (status) {
      case "Match":
        return "text-green-700 bg-green-50 border-green-200";
      case "Mismatch":
        return "text-red-700 bg-red-50 border-red-200";
      case "Missing":
        return "text-orange-700 bg-orange-50 border-orange-200";
      case "Untracked":
        return "text-purple-700 bg-purple-50 border-purple-200";
      default:
        return "text-gray-700 bg-gray-50 border-gray-200";
    }
  }

  function getStatusDot(status: string) {
    switch (status) {
      case "Match":
        return "bg-green-500";
      case "Mismatch":
        return "bg-red-500";
      case "Missing":
        return "bg-orange-500";
      case "Untracked":
        return "bg-purple-500";
      default:
        return "bg-gray-500";
    }
  }
</script>

<div class="font-mono text-[13px] select-none">
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="flex items-center py-1.5 px-2 hover:bg-gray-100 rounded transition-colors {node.node_type ===
    'Directory'
      ? 'cursor-pointer font-medium'
      : 'ml-4'}"
    onclick={toggle}
  >
    <!-- Expand/Collapse Chevron -->
    {#if node.node_type === "Directory"}
      <span
        class="mr-1 w-4 text-gray-500 inline-block text-center shrink-0"
      >
        {isOpen ? "▼" : "▶"}
      </span>
      <svg
        class="w-4 h-4 mr-2 text-blue-400 shrink-0"
        fill="currentColor"
        viewBox="0 0 20 20"
      >
        <path d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H2V6z" />
        <path d="M2 11v3a2 2 0 002 2h12a2 2 0 002-2v-3H2z" />
      </svg>
    {:else}
      <svg
        class="w-4 h-4 mr-2 text-gray-400 shrink-0"
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

    <span class="flex-1 truncate text-gray-700">
      {node.name}
    </span>

    <!-- Status Tag -->
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
    <div class="ml-2.5 pl-3 border-l border-gray-300">
      {#each node.children as child}
        <TreeItem node={child} />
      {/each}
    </div>
  {/if}
</div>
