<script>
  import { onMount } from 'svelte';
  import * as pdfjsLib from 'pdfjs-dist';
  import pdfWorker from 'pdfjs-dist/build/pdf.worker.min.mjs?url';

  pdfjsLib.GlobalWorkerOptions.workerSrc = pdfWorker;

  let { path, streamUrl } = $props();

  let canvasEl = $state(null);
  let pdfDoc = $state(null);
  let currentPage = $state(1);
  let pageCount = $state(0);
  let scale = $state(1.5);
  let rendering = $state(false);
  let loadState = $state('loading'); // 'loading' | 'ready' | 'error'
  let errorMsg = $state('');

  const MIN_SCALE = 0.5;
  const MAX_SCALE = 3.0;
  const SCALE_STEP = 0.25;

  onMount(async () => {
    try {
      const url = streamUrl(path);
      const doc = await pdfjsLib.getDocument({ url }).promise;
      pdfDoc = doc;
      pageCount = doc.numPages;
      loadState = 'ready';
      await renderPage(currentPage);
    } catch (e) {
      errorMsg = String(e);
      loadState = 'error';
    }
  });

  async function renderPage(num) {
    if (!pdfDoc || !canvasEl) return;
    rendering = true;
    try {
      const page = await pdfDoc.getPage(num);
      const viewport = page.getViewport({ scale });
      canvasEl.width = viewport.width;
      canvasEl.height = viewport.height;
      const ctx = canvasEl.getContext('2d');
      await page.render({ canvasContext: ctx, viewport }).promise;
    } catch (e) {
      // ignore render errors silently — canvas may have been remounted
    } finally {
      rendering = false;
    }
  }

  async function goToPage(num) {
    const n = Math.max(1, Math.min(pageCount, num));
    currentPage = n;
    await renderPage(n);
  }

  async function prevPage() {
    if (currentPage > 1) await goToPage(currentPage - 1);
  }

  async function nextPage() {
    if (currentPage < pageCount) await goToPage(currentPage + 1);
  }

  async function zoomIn() {
    if (scale < MAX_SCALE) {
      scale = Math.min(MAX_SCALE, parseFloat((scale + SCALE_STEP).toFixed(2)));
      await renderPage(currentPage);
    }
  }

  async function zoomOut() {
    if (scale > MIN_SCALE) {
      scale = Math.max(MIN_SCALE, parseFloat((scale - SCALE_STEP).toFixed(2)));
      await renderPage(currentPage);
    }
  }

  async function resetScale() {
    scale = 1.0;
    await renderPage(currentPage);
  }

  async function handleKeydown(e) {
    switch (e.key) {
      case 'ArrowLeft':
        e.preventDefault();
        await prevPage();
        break;
      case 'ArrowRight':
        e.preventDefault();
        await nextPage();
        break;
      case '+':
      case '=':
        e.preventDefault();
        await zoomIn();
        break;
      case '-':
        e.preventDefault();
        await zoomOut();
        break;
    }
  }

  let scalePct = $derived(Math.round(scale * 100));
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex flex-col h-full bg-gray-800">
  <!-- Top toolbar -->
  {#if loadState === 'ready'}
    <div class="h-10 bg-gray-900 border-b border-gray-700 flex items-center px-3 gap-2 shrink-0">
      <!-- Page navigation -->
      <button
        onclick={prevPage}
        disabled={currentPage <= 1}
        class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
        title="Previous page (←)"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
          <path d="M15.41 7.41 14 6l-6 6 6 6 1.41-1.41L10.83 12z"/>
        </svg>
      </button>
      <span class="text-gray-400 text-xs tabular-nums whitespace-nowrap">
        Page {currentPage} of {pageCount}
      </span>
      <button
        onclick={nextPage}
        disabled={currentPage >= pageCount}
        class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors disabled:opacity-40 disabled:cursor-not-allowed"
        title="Next page (→)"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
          <path d="M10 6 8.59 7.41 13.17 12l-4.58 4.59L10 18l6-6z"/>
        </svg>
      </button>

      <div class="w-px h-5 bg-gray-700 mx-1"></div>

      <!-- Zoom controls -->
      <button
        onclick={zoomOut}
        disabled={scale <= MIN_SCALE}
        class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors disabled:opacity-40 disabled:cursor-not-allowed text-base font-bold"
        title="Zoom Out (-)"
      >−</button>
      <span class="text-gray-400 text-xs w-10 text-center tabular-nums">{scalePct}%</span>
      <button
        onclick={zoomIn}
        disabled={scale >= MAX_SCALE}
        class="w-7 h-7 flex items-center justify-center rounded text-gray-300 hover:bg-gray-700 hover:text-white transition-colors disabled:opacity-40 disabled:cursor-not-allowed text-base font-bold"
        title="Zoom In (+)"
      >+</button>
      <button
        onclick={resetScale}
        class="px-2 h-6 flex items-center justify-center rounded text-gray-400 hover:bg-gray-700 hover:text-white transition-colors text-[11px] border border-gray-600"
        title="Reset zoom"
      >Reset</button>

      {#if rendering}
        <div class="ml-2 w-4 h-4 rounded-full border-2 border-gray-600 border-t-[var(--accent)] animate-spin"></div>
      {/if}
    </div>
  {/if}

  <!-- Page canvas area -->
  <div class="flex-1 min-h-0 overflow-auto flex justify-center p-6 bg-gray-700">
    {#if loadState === 'loading'}
      <div class="flex flex-col items-center justify-center gap-3 self-center">
        <div class="w-8 h-8 rounded-full border-2 border-gray-600 border-t-[var(--accent)] animate-spin"></div>
        <span class="text-gray-400 text-sm">Loading PDF…</span>
      </div>
    {:else if loadState === 'error'}
      <div class="flex flex-col items-center justify-center gap-3 text-center self-center">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor" class="text-red-400">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/>
        </svg>
        <div>
          <p class="text-red-400 font-medium">Could not load PDF</p>
          <p class="text-gray-500 text-sm mt-1">{errorMsg}</p>
        </div>
      </div>
    {:else}
      <canvas
        bind:this={canvasEl}
        class="shadow-2xl bg-white"
        style="max-width: 100%;"
      ></canvas>
    {/if}
  </div>
</div>
