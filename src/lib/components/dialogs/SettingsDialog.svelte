<script lang="ts">
  import type { AppSettings } from "$lib/types";
  import { fade, fly } from "svelte/transition";
  import { Select } from "bits-ui";

  let {
    settings = $bindable(),
    onClose,
    onUpdate,
  }: {
    settings: AppSettings;
    onClose: () => void;
    onUpdate: () => void;
  } = $props();

  const themes = [
    { value: "auto", label: "Automatic (System)" },
    { value: "light", label: "Light" },
    { value: "dark", label: "Dark" },
  ];

  const algorithms = [
    { value: "sha256", label: "SHA-256 (Compatibility)" },
    { value: "blake2b", label: "BLAKE2b (Fast 64-bit)" },
    { value: "blake3", label: "BLAKE3 (Ultra Fast)" },
    { value: "sha1", label: "SHA-1 (Legacy)" },
    { value: "md5", label: "MD5 (Legacy/Fast)" },
  ];

  const verifyDepths = [
    { value: "quick", label: "Quick (Check Date & Size)" },
    { value: "deep", label: "Deep (Read Entire File)" },
  ];

  const readModes = [
    { value: "parallel", label: "Parallel (NVMe / SSD Fast)" },
    { value: "sequential", label: "Sequential (HDD Safe)" },
  ];

  const bufferSizes = [
    { value: "131072", label: "128 KB" },
    { value: "262144", label: "256 KB (Default)" },
    { value: "524288", label: "512 KB" },
  ];

  let bufferSizeStr = $state(settings.buffer_size.toString());

  function onBufferSizeChange(v: string) {
    settings.buffer_size = parseInt(v, 10);
    onUpdate();
  }

  const selectedThemeLabel = $derived(
    themes.find((t) => t.value === settings.theme)?.label,
  );
  const selectedAlgoLabel = $derived(
    algorithms.find((a) => a.value === settings.algorithm)?.label,
  );
  const selectedDepthLabel = $derived(
    verifyDepths.find((d) => d.value === settings.verify_depth)?.label,
  );
  const selectedReadModeLabel = $derived(
    readModes.find((r) => r.value === settings.read_mode)?.label,
  );
  const selectedBufferSizeLabel = $derived(
    bufferSizes.find((b) => b.value === bufferSizeStr)?.label,
  );
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 dark:bg-black/70"
  transition:fade={{ duration: 150 }}
>
  <div
    class="w-full max-w-lg p-6 bg-white shadow-xl dark:bg-gray-800 rounded-xl max-h-[90vh] flex flex-col"
    transition:fly={{ y: 20, duration: 250 }}
  >
    <h2
      class="flex items-center mb-6 text-xl font-bold text-gray-900 dark:text-white shrink-0"
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

    <div
      class="space-y-6 overflow-y-auto pr-3 -mr-2 pl-2 -ml-2 py-2 -my-2 custom-scrollbar"
    >
      <div class="space-y-2">
        <label
          for="theme-trigger"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >Theme Preference</label
        >
        <Select.Root
          type="single"
          bind:value={settings.theme}
          onValueChange={onUpdate}
          items={themes}
        >
          <Select.Trigger
            id="theme-trigger"
            class="inline-flex items-center justify-between w-full px-4 py-2.5 text-sm transition-all border border-gray-300 rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700/50 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span class="text-gray-900 dark:text-gray-100"
              >{selectedThemeLabel}</span
            >
            <svg
              class="w-4 h-4 text-gray-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              /></svg
            >
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="z-50 w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) p-1.5 bg-white border border-gray-200 shadow-xl dark:bg-gray-800 dark:border-gray-700 rounded-lg outline-none"
              sideOffset={4}
            >
              <Select.Viewport>
                {#each themes as theme}
                  <Select.Item
                    value={theme.value}
                    label={theme.label}
                    class="relative flex items-center w-full py-2.5 px-3 pr-10 text-sm transition-colors rounded-md cursor-pointer select-none outline-none data-highlighted:bg-blue-50 dark:data-highlighted:bg-blue-900/30 text-gray-700 dark:text-gray-300 data-highlighted:text-blue-700 dark:data-highlighted:text-blue-300"
                  >
                    {#snippet children({ selected })}
                      {theme.label}
                      {#if selected}
                        <div
                          class="absolute right-3 flex items-center justify-center"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            ><path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="3"
                              d="M5 13l4 4L19 7"
                            /></svg
                          >
                        </div>
                      {/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      <div class="space-y-2">
        <label
          for="algo-trigger"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >Deep Hashing Algorithm</label
        >
        <Select.Root
          type="single"
          bind:value={settings.algorithm}
          onValueChange={onUpdate}
          items={algorithms}
        >
          <Select.Trigger
            id="algo-trigger"
            class="inline-flex items-center justify-between w-full px-4 py-2.5 text-sm transition-all border border-gray-300 rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700/50 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span class="text-gray-900 dark:text-gray-100"
              >{selectedAlgoLabel}</span
            >
            <svg
              class="w-4 h-4 text-gray-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              /></svg
            >
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="z-50 w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) p-1.5 bg-white border border-gray-200 shadow-xl dark:bg-gray-800 dark:border-gray-700 rounded-lg outline-none"
              sideOffset={4}
            >
              <Select.Viewport>
                {#each algorithms as algo}
                  <Select.Item
                    value={algo.value}
                    label={algo.label}
                    class="relative flex items-center w-full py-2.5 px-3 pr-10 text-sm transition-colors rounded-md cursor-pointer select-none outline-none data-highlighted:bg-blue-50 dark:data-highlighted:bg-blue-900/30 text-gray-700 dark:text-gray-300 data-highlighted:text-blue-700 dark:data-highlighted:text-blue-300"
                  >
                    {#snippet children({ selected })}
                      {algo.label}
                      {#if selected}
                        <div
                          class="absolute right-3 flex items-center justify-center"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            ><path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="3"
                              d="M5 13l4 4L19 7"
                            /></svg
                          >
                        </div>
                      {/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      <div class="space-y-2">
        <label
          for="depth-trigger"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >Verification Depth</label
        >
        <Select.Root
          type="single"
          bind:value={settings.verify_depth}
          onValueChange={onUpdate}
          items={verifyDepths}
        >
          <Select.Trigger
            id="depth-trigger"
            class="inline-flex items-center justify-between w-full px-4 py-2.5 text-sm transition-all border border-gray-300 rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700/50 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span class="text-gray-900 dark:text-gray-100"
              >{selectedDepthLabel}</span
            >
            <svg
              class="w-4 h-4 text-gray-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              /></svg
            >
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="z-50 w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) p-1.5 bg-white border border-gray-200 shadow-xl dark:bg-gray-800 dark:border-gray-700 rounded-lg outline-none"
              sideOffset={4}
            >
              <Select.Viewport>
                {#each verifyDepths as depth}
                  <Select.Item
                    value={depth.value}
                    label={depth.label}
                    class="relative flex items-center w-full py-2.5 px-3 pr-10 text-sm transition-colors rounded-md cursor-pointer select-none outline-none data-highlighted:bg-blue-50 dark:data-highlighted:bg-blue-900/30 text-gray-700 dark:text-gray-300 data-highlighted:text-blue-700 dark:data-highlighted:text-blue-300"
                  >
                    {#snippet children({ selected })}
                      {depth.label}
                      {#if selected}
                        <div
                          class="absolute right-3 flex items-center justify-center"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            ><path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="3"
                              d="M5 13l4 4L19 7"
                            /></svg
                          >
                        </div>
                      {/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      <hr class="border-gray-200 dark:border-gray-700 my-4" />
      <h3 class="text-md font-semibold text-gray-800 dark:text-gray-200">
        Advanced Performance Options
      </h3>

      <div class="space-y-2">
        <label
          for="mode-trigger"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >Concurrency Mode</label
        >
        <Select.Root
          type="single"
          bind:value={settings.read_mode}
          onValueChange={onUpdate}
          items={readModes}
        >
          <Select.Trigger
            id="mode-trigger"
            class="inline-flex items-center justify-between w-full px-4 py-2.5 text-sm transition-all border border-gray-300 rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700/50 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span class="text-gray-900 dark:text-gray-100"
              >{selectedReadModeLabel}</span
            >
            <svg
              class="w-4 h-4 text-gray-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              /></svg
            >
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="z-50 w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) p-1.5 bg-white border border-gray-200 shadow-xl dark:bg-gray-800 dark:border-gray-700 rounded-lg outline-none"
              sideOffset={4}
            >
              <Select.Viewport>
                {#each readModes as mode}
                  <Select.Item
                    value={mode.value}
                    label={mode.label}
                    class="relative flex items-center w-full py-2.5 px-3 pr-10 text-sm transition-colors rounded-md cursor-pointer select-none outline-none data-highlighted:bg-blue-50 dark:data-highlighted:bg-blue-900/30 text-gray-700 dark:text-gray-300 data-highlighted:text-blue-700 dark:data-highlighted:text-blue-300"
                  >
                    {#snippet children({ selected })}
                      {mode.label}
                      {#if selected}
                        <div
                          class="absolute right-3 flex items-center justify-center"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            ><path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="3"
                              d="M5 13l4 4L19 7"
                            /></svg
                          >
                        </div>
                      {/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>

      <div class="space-y-2">
        <label
          for="buffer-trigger"
          class="block text-sm font-semibold text-gray-700 dark:text-gray-300"
          >I/O Buffer Size</label
        >
        <Select.Root
          type="single"
          bind:value={bufferSizeStr}
          onValueChange={onBufferSizeChange}
          items={bufferSizes}
        >
          <Select.Trigger
            id="buffer-trigger"
            class="inline-flex items-center justify-between w-full px-4 py-2.5 text-sm transition-all border border-gray-300 rounded-lg shadow-sm bg-gray-50 dark:bg-gray-700/50 dark:border-gray-600 hover:bg-gray-100 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <span class="text-gray-900 dark:text-gray-100"
              >{selectedBufferSizeLabel}</span
            >
            <svg
              class="w-4 h-4 text-gray-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              ><path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M19 9l-7 7-7-7"
              /></svg
            >
          </Select.Trigger>
          <Select.Portal>
            <Select.Content
              class="z-50 w-(--bits-select-anchor-width) min-w-(--bits-select-anchor-width) p-1.5 bg-white border border-gray-200 shadow-xl dark:bg-gray-800 dark:border-gray-700 rounded-lg outline-none"
              sideOffset={4}
            >
              <Select.Viewport>
                {#each bufferSizes as bSize}
                  <Select.Item
                    value={bSize.value}
                    label={bSize.label}
                    class="relative flex items-center w-full py-2.5 px-3 pr-10 text-sm transition-colors rounded-md cursor-pointer select-none outline-none data-highlighted:bg-blue-50 dark:data-highlighted:bg-blue-900/30 text-gray-700 dark:text-gray-300 data-highlighted:text-blue-700 dark:data-highlighted:text-blue-300"
                  >
                    {#snippet children({ selected })}
                      {bSize.label}
                      {#if selected}
                        <div
                          class="absolute right-3 flex items-center justify-center"
                        >
                          <svg
                            class="w-4 h-4"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                            ><path
                              stroke-linecap="round"
                              stroke-linejoin="round"
                              stroke-width="3"
                              d="M5 13l4 4L19 7"
                            /></svg
                          >
                        </div>
                      {/if}
                    {/snippet}
                  </Select.Item>
                {/each}
              </Select.Viewport>
            </Select.Content>
          </Select.Portal>
        </Select.Root>
      </div>
    </div>

    <div
      class="flex justify-end pt-5 border-t border-gray-100 dark:border-gray-700 mt-2"
    >
      <button
        class="px-5 py-2 font-medium text-gray-800 transition bg-gray-200 rounded-lg shadow-sm cursor-pointer dark:bg-gray-700 dark:hover:bg-gray-600 dark:text-gray-200 hover:bg-gray-300"
        onclick={onClose}
      >
        Close
      </button>
    </div>
  </div>
</div>
