<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { initTheme } from '../../common-js/theme.js';
  import MediaViewer from './lib/MediaViewer.svelte';

  const appWindow = getCurrentWindow();

  let filename = $state('Prism');

  function onFileLoaded(name) {
    filename = name;
  }

  onMount(() => initTheme(invoke));
</script>

<!-- Frameless window: custom titlebar + media viewer -->
<div class="relative flex flex-col h-full bg-white dark:bg-gray-900 overflow-hidden">

  <!-- Top resize strip — see shelf App.svelte for explanation -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="absolute top-[8px] left-0 right-0 h-[4px] z-50 cursor-n-resize"
    onmousedown={(e) => { e.preventDefault(); appWindow.startResizeDragging('North'); }}
  ></div>

  <!-- Custom titlebar (draggable) — title left, controls right (Windows style) -->
  <div
    data-tauri-drag-region
    class="h-8 bg-[var(--titlebar-bg)] border-b border-gray-200 dark:border-gray-700
           flex items-center shrink-0 select-none"
  >
    <!-- Left: icon + filename -->
    <div class="flex items-center gap-2 flex-1 min-w-0 pl-3" data-tauri-drag-region>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="text-[var(--accent)] shrink-0">
        <path d="M9 15h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" />
        <path d="M9 12a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3l0 -6" />
        <path d="M3 12l2.296 -2.296a2.41 2.41 0 0 1 3.408 0l.296 .296" />
        <path d="M14 13.5v3l2.5 -1.5l-2.5 -1.5" />
        <path d="M7 6v.01" />
      </svg>
      <span class="text-[13px] font-medium text-gray-700 dark:text-gray-300 truncate">{filename}</span>
    </div>

    <!-- Right: Windows-style controls — minimize, maximize, close -->
    <div class="flex items-center shrink-0 h-full">
      <button
        onclick={() => appWindow.minimize()}
        class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
               hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-[16px] leading-none"
        title="Minimize"
        aria-label="Minimize"
      >─</button>
      <button
        onclick={() => appWindow.toggleMaximize()}
        class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
               hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-[13px]"
        title="Maximize"
        aria-label="Maximize"
      >□</button>
      <button
        onclick={() => appWindow.close()}
        class="w-11 h-full flex items-center justify-center text-gray-500 dark:text-gray-400
               hover:bg-red-500 hover:text-white transition-colors text-[18px]"
        title="Close"
        aria-label="Close"
      >×</button>
    </div>
  </div>

  <!-- Media viewer -->
  <div class="flex-1 min-h-0" role="main">
    <MediaViewer {onFileLoaded} />
  </div>

</div>
