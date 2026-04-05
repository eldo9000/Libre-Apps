<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import ImageViewer from './ImageViewer.svelte';
  import VideoPlayer from './VideoPlayer.svelte';
  import AudioPlayer from './AudioPlayer.svelte';
  import PdfViewer from './PdfViewer.svelte';
  import ModelViewer from './ModelViewer.svelte';

  let { onFileLoaded } = $props();

  let viewState = $state('loading'); // 'loading' | 'empty' | 'error' | 'ready'
  let errorMsg = $state('');
  let fileInfo = $state(null);

  function streamUrl(path) {
    const encoded = path.split('/').map(s => encodeURIComponent(s)).join('/');
    return `mvstream://localhost${encoded}`;
  }

  onMount(async () => {
    try {
      const path = await invoke('get_initial_file');
      if (!path) {
        viewState = 'empty';
        return;
      }
      const info = await invoke('get_file_info', { path });
      fileInfo = info;
      if (onFileLoaded) onFileLoaded(info.name);
      viewState = 'ready';
    } catch (e) {
      errorMsg = String(e);
      viewState = 'error';
    }
  });
</script>

<div class="w-full h-full flex flex-col bg-white dark:bg-gray-900">
  {#if viewState === 'loading'}
    <!-- Loading state -->
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 rounded-full border-2 border-gray-200 dark:border-gray-700 border-t-[#0066cc] animate-spin"></div>
        <span class="text-gray-500 dark:text-gray-400 text-sm">Loading…</span>
      </div>
    </div>

  {:else if viewState === 'empty'}
    <!-- Empty / no file state -->
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-4 text-center px-8">
        <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-gray-300 dark:text-gray-600">
          <circle cx="12" cy="12" r="10"/>
          <path d="M10 8l6 4-6 4V8z"/>
        </svg>
        <div>
          <p class="text-gray-700 dark:text-gray-300 font-medium text-base">No file open</p>
          <p class="text-gray-500 text-sm mt-1">Open a file from the command line or drag one here</p>
          <p class="text-gray-400 dark:text-gray-600 text-xs mt-2 font-mono">prism /path/to/file</p>
        </div>
      </div>
    </div>

  {:else if viewState === 'error'}
    <!-- Error state -->
    <div class="flex-1 flex items-center justify-center">
      <div class="flex flex-col items-center gap-3 text-center px-8">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor" class="text-red-500 dark:text-red-400">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
        <div>
          <p class="text-red-500 dark:text-red-400 font-medium">Could not open file</p>
          <p class="text-gray-500 text-sm mt-1">{errorMsg}</p>
        </div>
      </div>
    </div>

  {:else if viewState === 'ready' && fileInfo}
    <!-- Route to the right sub-viewer -->
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
      <!-- Unknown file type fallback -->
      <div class="flex-1 flex items-center justify-center">
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
  {/if}
</div>
