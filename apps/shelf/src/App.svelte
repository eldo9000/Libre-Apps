<script>
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import QuickFiles from './lib/QuickFiles.svelte';

  const appWindow = getCurrentWindow();

  let quickFilesRef;
  let dualPane = false;

  onMount(async () => {
    // Read theme from shared ~/.config/librewin/theme — works across app processes.
    const theme = await invoke('get_theme');
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    document.documentElement.classList.toggle('dark',
      theme === 'dark' || (theme === 'system' && mq.matches));
    // Read accent from shared ~/.config/librewin/accent and apply to CSS variable.
    const accent = await invoke('get_accent');
    document.documentElement.style.setProperty('--accent', accent);
    // Live update: fires when KDE applies a new color scheme (portal → WebKit).
    const handler = async (e) => {
      const t = await invoke('get_theme');
      if (t === 'system') document.documentElement.classList.toggle('dark', e.matches);
    };
    mq.addEventListener('change', handler);
    return () => mq.removeEventListener('change', handler);
  });
</script>

<!-- Frameless window: custom titlebar + file manager -->
<div class="relative flex flex-col h-full bg-white dark:bg-gray-900 overflow-hidden">

  <!-- Top resize strip — 4px invisible hit-target at the window top edge.
       data-tauri-drag-region on the titlebar below steals mousedown before the OS
       resize handler fires, so the top edge moves the window instead of resizing.
       This strip is above the drag region in z-order and explicitly calls
       startResizeDragging so dragging the very top edge works correctly. -->
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
    <!-- Left: icon + title -->
    <div class="flex items-center gap-2 flex-1 min-w-0 pl-3" data-tauri-drag-region>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="text-[var(--accent)] shrink-0">
        <path d="M9 3h3l2 2h5a2 2 0 0 1 2 2v7a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2" />
        <path d="M17 16v2a2 2 0 0 1 -2 2h-10a2 2 0 0 1 -2 -2v-9a2 2 0 0 1 2 -2h2" />
      </svg>
      <span class="text-[13px] font-medium text-gray-700 dark:text-gray-300 truncate">Shelf</span>
    </div>

    <!-- Dual-pane toggle -->
    <button
      onclick={() => quickFilesRef?.toggleDualPane()}
      class="flex items-center gap-1.5 px-2 py-0.5 rounded text-[11px] transition-colors mr-1
        {dualPane ? 'bg-[var(--accent)]/15 text-[var(--accent)]' : 'text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700'}"
      title="Toggle dual pane"
      aria-label="Toggle dual pane"
    >
      <svg width="14" height="10" viewBox="0 0 14 10" fill="none" stroke="currentColor" stroke-width="1.5">
        <rect x="0.5" y="0.5" width="5.5" height="9" rx="1"/>
        <rect x="8" y="0.5" width="5.5" height="9" rx="1"/>
      </svg>
    </button>

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

  <!-- File manager -->
  <div class="flex-1 min-h-0" role="main">
    <QuickFiles bind:this={quickFilesRef} bind:dualPane />
  </div>

</div>
