<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  let { path, ext, category, streamUrl } = $props();

  let imgSrc = $state('');
  let imgState = $state('loading'); // 'loading' | 'ready' | 'error'
  let errorMsg = $state('');

  let zoom = $state(1.0);       // 1.0 = 100%
  let rotation = $state(0);     // degrees, multiples of 90

  const MIN_ZOOM = 0.1;
  const MAX_ZOOM = 8.0;
  const ZOOM_STEP = 0.15;

  onMount(async () => {
    if (category === 'image') {
      imgSrc = streamUrl(path);
      imgState = 'ready';
    } else {
      // image_convert — invoke backend to convert via ImageMagick
      try {
        const tmpPath = await invoke('convert_image_to_png', { path });
        imgSrc = streamUrl(tmpPath);
        imgState = 'ready';
      } catch (e) {
        errorMsg = String(e);
        imgState = 'error';
      }
    }
  });

  function zoomIn() {
    zoom = Math.min(MAX_ZOOM, zoom + ZOOM_STEP);
  }
  function zoomOut() {
    zoom = Math.max(MIN_ZOOM, zoom - ZOOM_STEP);
  }
  function resetZoom() {
    zoom = 1.0;
  }
  function rotateLeft() {
    rotation = (rotation - 90 + 360) % 360;
  }
  function rotateRight() {
    rotation = (rotation + 90) % 360;
  }

  function handleWheel(e) {
    e.preventDefault();
    if (e.deltaY < 0) {
      zoomIn();
    } else {
      zoomOut();
    }
  }

  function handleKeydown(e) {
    switch (e.key) {
      case '+':
      case '=':
        e.preventDefault();
        zoomIn();
        break;
      case '-':
        e.preventDefault();
        zoomOut();
        break;
      case '0':
        e.preventDefault();
        resetZoom();
        break;
      case 'r':
      case 'R':
        e.preventDefault();
        rotateRight();
        break;
      case 'l':
      case 'L':
        e.preventDefault();
        rotateLeft();
        break;
    }
  }

  let zoomPct = $derived(Math.round(zoom * 100));
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex flex-col h-full bg-gray-900">
  <!-- Image area -->
  <div
    class="flex-1 min-h-0 flex items-center justify-center overflow-auto bg-gray-950 relative"
    onwheel={handleWheel}
  >
    {#if imgState === 'loading'}
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 rounded-full border-2 border-gray-700 border-t-[var(--accent)] animate-spin"></div>
        <span class="text-gray-400 text-sm">
          {category === 'image_convert' ? 'Converting image…' : 'Loading…'}
        </span>
      </div>
    {:else if imgState === 'error'}
      <div class="flex flex-col items-center gap-3 text-center px-8">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor" class="text-red-400">
          <path d="M21 19V5c0-1.1-.9-2-2-2H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2zM8.5 13.5l2.5 3.01L14.5 12l4.5 6H5l3.5-4.5z"/>
        </svg>
        <div>
          <p class="text-red-400 font-medium">Could not load image</p>
          <p class="text-gray-500 text-sm mt-1">{errorMsg}</p>
        </div>
      </div>
    {:else}
      <div class="flex items-center justify-center p-8" style="min-width: 100%; min-height: 100%;">
        <img
          src={imgSrc}
          alt={path}
          style="transform: scale({zoom}) rotate({rotation}deg); transform-origin: center; transition: transform 0.1s ease;"
          class="max-w-none block"
          draggable="false"
        />
      </div>
    {/if}
  </div>

  <!-- Bottom toolbar -->
  <div class="h-10 bg-gray-800 border-t border-gray-700 flex items-center px-3 gap-2 shrink-0">
    <!-- Zoom controls -->
    <button
      onclick={zoomOut}
      class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors text-base font-bold"
      title="Zoom Out (-)"
    >−</button>
    <span class="text-gray-400 text-xs w-10 text-center tabular-nums">{zoomPct}%</span>
    <button
      onclick={zoomIn}
      class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors text-base font-bold"
      title="Zoom In (+)"
    >+</button>
    <button
      onclick={resetZoom}
      class="px-2 h-6 flex items-center justify-center rounded text-gray-400 hover:bg-gray-700 hover:text-white transition-colors text-[11px] border border-gray-600"
      title="Reset Zoom (0)"
    >1:1</button>

    <div class="w-px h-5 bg-gray-700 mx-1"></div>

    <!-- Rotation controls -->
    <button
      onclick={rotateLeft}
      class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors"
      title="Rotate Left (L)"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
        <path d="M3 3v5h5"/>
      </svg>
    </button>
    <button
      onclick={rotateRight}
      class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors"
      title="Rotate Right (R)"
    >
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M21 12a9 9 0 1 1-9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/>
        <path d="M21 3v5h-5"/>
      </svg>
    </button>

    <!-- Spacer -->
    <div class="flex-1"></div>

    <!-- File info -->
    <span class="text-gray-400 text-xs truncate max-w-[260px]">
      {path.split('/').pop()}
    </span>
    {#if ext}
      <span class="px-1.5 py-0.5 rounded bg-gray-700 text-gray-400 text-[10px] uppercase tracking-wide shrink-0">{ext}</span>
    {/if}
  </div>
</div>
