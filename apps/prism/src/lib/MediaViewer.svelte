<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import ImageViewer from './ImageViewer.svelte';
  import VideoPlayer from './VideoPlayer.svelte';
  import AudioPlayer from './AudioPlayer.svelte';
  import PdfViewer from './PdfViewer.svelte';
  import ModelViewer from './ModelViewer.svelte';
  import MetadataPanel from './MetadataPanel.svelte';
  import ShortcutsOverlay from './ShortcutsOverlay.svelte';

  let { onFileLoaded } = $props();

  let viewState = $state('loading'); // 'loading' | 'empty' | 'error' | 'ready'
  let errorMsg = $state('');
  let fileInfo = $state(null);

  // UI overlays
  let showInfo = $state(false);
  let showShortcuts = $state(false);
  let dragging = $state(false);

  // Metadata
  let metadata = $state(null);
  let metaLoading = $state(false);

  // File navigation siblings
  let siblingFiles = $state([]);
  let siblingIdx = $state(-1);

  function streamUrl(path) {
    const encoded = path.split('/').map(s => encodeURIComponent(s)).join('/');
    return `mvstream://localhost${encoded}`;
  }

  async function loadFile(path) {
    viewState = 'loading';
    showInfo = false;
    metadata = null;
    siblingFiles = [];
    siblingIdx = -1;

    try {
      const info = await invoke('get_file_info', { path });
      fileInfo = info;
      if (onFileLoaded) onFileLoaded(info.name, info.path);
      viewState = 'ready';

      // Track recent + compute siblings
      invoke('add_recent_file', { path: info.path }).catch(() => {});
      const dir = info.path.substring(0, info.path.lastIndexOf('/')) || '/';
      const siblings = await invoke('list_dir_files', { dir, category: info.category });
      siblingFiles = siblings;
      siblingIdx = siblings.indexOf(info.path);
    } catch (e) {
      errorMsg = String(e);
      viewState = 'error';
    }
  }

  async function loadMetadata() {
    if (!fileInfo) return;
    metaLoading = true;
    try {
      metadata = await invoke('get_metadata', { path: fileInfo.path });
    } catch {
      metadata = null;
    } finally {
      metaLoading = false;
    }
  }

  // Toggle info panel; load metadata on first open
  function toggleInfo() {
    showInfo = !showInfo;
    if (showInfo && metadata === null && fileInfo) {
      loadMetadata();
    }
  }

  // Navigate to adjacent file
  async function navigateFile(dir) {
    if (siblingFiles.length === 0 || siblingIdx < 0) return;
    const next = siblingIdx + dir;
    if (next < 0 || next >= siblingFiles.length) return;
    await loadFile(siblingFiles[next]);
  }

  // Open file dialog (Ctrl+O)
  async function openFileDialog() {
    try {
      const path = await invoke('open_file_dialog');
      if (path) await loadFile(path);
    } catch {
      // dialog cancelled or invoke failed — no-op
    }
  }

  // Keyboard handler — global shortcuts
  function handleKeydown(e) {
    // Don't intercept if a text input has focus
    const tag = document.activeElement?.tagName;
    if (tag === 'INPUT' || tag === 'TEXTAREA') return;

    switch (e.key) {
      case '?':
        e.preventDefault();
        showShortcuts = !showShortcuts;
        break;
      case 'i':
      case 'I':
        e.preventDefault();
        if (viewState === 'ready') toggleInfo();
        break;
      case 'Escape':
        if (showShortcuts) { e.preventDefault(); showShortcuts = false; }
        else if (showInfo)  { e.preventDefault(); showInfo = false; }
        break;
      case 'o':
      case 'O':
        if (e.ctrlKey || e.metaKey) {
          e.preventDefault();
          openFileDialog();
        }
        break;
      case 'ArrowLeft':
        // Only handle file nav for categories where ← is not used by the sub-viewer
        if (viewState === 'ready' && fileInfo &&
            !['video', 'audio', 'pdf'].includes(fileInfo.category)) {
          e.preventDefault();
          navigateFile(-1);
        }
        break;
      case 'ArrowRight':
        if (viewState === 'ready' && fileInfo &&
            !['video', 'audio', 'pdf'].includes(fileInfo.category)) {
          e.preventDefault();
          navigateFile(1);
        }
        break;
    }
  }

  // Drag-and-drop
  function handleDragOver(e) {
    e.preventDefault();
    dragging = true;
  }

  function handleDragLeave(e) {
    // Only clear if leaving the root element (not just a child)
    if (!e.currentTarget.contains(e.relatedTarget)) {
      dragging = false;
    }
  }

  async function handleDrop(e) {
    e.preventDefault();
    dragging = false;

    const files = e.dataTransfer?.files;
    if (!files || files.length === 0) return;

    // Tauri WebView exposes native path on the file object
    const file = files[0];
    const path = file.path || '';
    if (path) {
      await loadFile(path);
    }
  }

  onMount(async () => {
    try {
      const path = await invoke('get_initial_file');
      if (!path) {
        viewState = 'empty';
        return;
      }
      await loadFile(path);
    } catch (e) {
      errorMsg = String(e);
      viewState = 'error';
    }
  });
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Root container: relative so overlays can anchor to it -->
<div
  class="w-full h-full flex flex-col bg-white dark:bg-gray-900 relative"
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  ondrop={handleDrop}
  role="main"
>
  {#if viewState === 'loading'}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 rounded-full border-2 border-gray-200 dark:border-gray-700 border-t-[var(--accent)] animate-spin"></div>
        <span class="text-gray-500 dark:text-gray-400 text-sm">Loading…</span>
      </div>
    </div>

  {:else if viewState === 'empty'}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-4 text-center px-8">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-gray-300 dark:text-gray-600">
          <circle cx="12" cy="12" r="10"/>
          <path d="M10 8l6 4-6 4V8z"/>
        </svg>
        <div>
          <p class="text-gray-700 dark:text-gray-300 font-medium text-base">No file open</p>
          <p class="text-gray-500 text-sm mt-1">Open a file with <kbd class="px-1.5 py-0.5 text-xs rounded bg-gray-100 dark:bg-gray-800 font-mono">Ctrl+O</kbd> or drag one here</p>
          <p class="text-gray-400 dark:text-gray-600 text-xs mt-2 font-mono">prism /path/to/file</p>
        </div>
        <button
          onclick={openFileDialog}
          class="mt-2 px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity"
        >
          Open file…
        </button>
      </div>
    </div>

  {:else if viewState === 'error'}
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3 text-center px-8">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor" class="text-red-500 dark:text-red-400">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
        <div>
          <p class="text-red-500 dark:text-red-400 font-medium">Could not open file</p>
          <p class="text-gray-500 text-sm mt-1">{errorMsg}</p>
        </div>
        <button
          onclick={openFileDialog}
          class="mt-2 px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity"
        >
          Open another file…
        </button>
      </div>
    </div>

  {:else if viewState === 'ready' && fileInfo}
    <!-- Sub-viewer area + metadata panel side by side -->
    <div class="flex-1 min-h-0 flex relative">
      <!-- Sub-viewer -->
      <div class="flex-1 min-w-0 min-h-0">
        {#if fileInfo.category === 'image' || fileInfo.category === 'image_convert'}
          <ImageViewer
            path={fileInfo.path}
            ext={fileInfo.ext}
            category={fileInfo.category}
            {streamUrl}
          />
        {:else if fileInfo.category === 'video'}
          <VideoPlayer
            path={fileInfo.path}
            ext={fileInfo.ext}
            {streamUrl}
          />
        {:else if fileInfo.category === 'audio'}
          <AudioPlayer
            path={fileInfo.path}
            ext={fileInfo.ext}
            name={fileInfo.name}
            size={fileInfo.size}
            {streamUrl}
          />
        {:else if fileInfo.category === 'pdf'}
          <PdfViewer
            path={fileInfo.path}
            {streamUrl}
          />
        {:else if fileInfo.category === 'model'}
          <ModelViewer
            path={fileInfo.path}
            ext={fileInfo.ext}
            {streamUrl}
          />
        {:else}
          <div class="flex-1 flex items-center justify-center h-full">
            <div class="flex flex-col items-center gap-3 text-center px-8">
              <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-gray-300 dark:text-gray-600">
                <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9l-7-7z"/>
                <polyline points="13 2 13 9 20 9"/>
              </svg>
              <div>
                <p class="text-gray-700 dark:text-gray-300 font-medium">{fileInfo.name}</p>
                <span class="inline-block mt-2 px-2 py-0.5 rounded bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 text-xs uppercase tracking-wide">{fileInfo.ext || fileInfo.category}</span>
                <p class="text-gray-500 text-sm mt-2">This file type is not supported for preview</p>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Info panel -->
      {#if showInfo}
        {#if metaLoading}
          <!-- Loading state while fetching metadata -->
          <div class="absolute top-0 right-0 h-full w-72 bg-[var(--surface,#1f2937)] border-l border-[var(--border,#374151)] flex items-center justify-center z-20">
            <div class="flex flex-col items-center gap-3">
              <div class="w-6 h-6 rounded-full border-2 border-gray-600 border-t-[var(--accent)] animate-spin"></div>
              <span class="text-gray-400 text-xs">Loading metadata…</span>
            </div>
          </div>
        {:else}
          <MetadataPanel {metadata} onClose={() => { showInfo = false; }} />
        {/if}
      {/if}
    </div>
  {/if}

  <!-- Drop zone overlay -->
  {#if dragging}
    <div class="absolute inset-0 z-30 flex items-center justify-center bg-[var(--accent,#0066cc)]/20 border-4 border-dashed border-[var(--accent,#0066cc)] rounded-lg pointer-events-none">
      <div class="flex flex-col items-center gap-3 text-[var(--accent,#0066cc)]">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="17 8 12 3 7 8"/>
          <line x1="12" y1="3" x2="12" y2="15"/>
        </svg>
        <span class="text-lg font-semibold">Drop to open</span>
      </div>
    </div>
  {/if}

  <!-- Shortcuts overlay (portal-like, sits on top of everything) -->
  {#if showShortcuts}
    <ShortcutsOverlay onClose={() => { showShortcuts = false; }} />
  {/if}
</div>
