<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { WindowFrame, Titlebar } from '@libre/ui';
  import { searchLinks, softwareLinks, browsers } from './lib/brandColors.js';

  let urlInput = $state('');
  let downloadToast = $state(null); // { filename: string } | null

  onMount(async () => {
    const unlistenDownload = listen('download-complete', ({ payload }) => {
      downloadToast = { filename: payload };
      setTimeout(() => { downloadToast = null; }, 4000);
    });
    return () => { unlistenDownload.then(f => f()); };
  });

  function navigate() {
    let url = urlInput.trim();
    if (!url) return;
    if (!url.includes('.') || url.includes(' ')) {
      url = 'https://duckduckgo.com/?q=' + encodeURIComponent(url);
    } else if (!/^https?:\/\//i.test(url)) {
      url = 'https://' + url;
    }
    window.location.href = url;
  }

  function onKeydown(e) {
    if (e.key === 'Enter') navigate();
  }

  function go(url) {
    urlInput = url;
    navigate();
  }

  async function openBrowser(app) {
    await invoke('launch_app', { app });
    await invoke('close_window');
  }


</script>

<WindowFrame>
  <!-- Titlebar: Ghost icon + name LEFT, nav + address bar CENTER, controls RIGHT -->
  <Titlebar height="h-11">
    <!-- Left: Ghost icon + name -->
    <div class="flex items-center gap-2 shrink-0 pl-3 mr-2" data-tauri-drag-region>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
           class="text-[var(--accent)] shrink-0">
        <path d="M12 3a7 7 0 0 0-7 7v9l2.5-2 2.5 2 2.5-2 2.5 2 2.5-2V10a7 7 0 0 0-7-7z"/>
        <circle cx="9.5" cy="11" r="1" fill="currentColor" stroke="none"/>
        <circle cx="14.5" cy="11" r="1" fill="currentColor" stroke="none"/>
      </svg>
      <span class="text-[13px] font-medium text-gray-700 dark:text-gray-300">Ghost</span>
    </div>

    <!-- Nav controls + address bar + Go -->
    <div class="flex items-center gap-1 flex-1 min-w-0 pr-1" role="navigation" aria-label="Browser navigation">
      <button
        onclick={() => history.back()}
        class="p-1 rounded text-gray-400 dark:text-gray-500 hover:text-gray-700 dark:hover:text-gray-300
               hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-base leading-none shrink-0"
        title="Back" aria-label="Back"
      >‹</button>
      <button
        onclick={() => history.forward()}
        class="p-1 rounded text-gray-400 dark:text-gray-500 hover:text-gray-700 dark:hover:text-gray-300
               hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-base leading-none shrink-0"
        title="Forward" aria-label="Forward"
      >›</button>
      <button
        onclick={() => window.location.reload()}
        class="p-1 rounded text-gray-400 dark:text-gray-500 hover:text-gray-700 dark:hover:text-gray-300
               hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors text-sm leading-none shrink-0"
        title="Reload" aria-label="Reload"
      >↺</button>

      <input
        bind:value={urlInput}
        onkeydown={onKeydown}
        onfocus={(e) => e.target.select()}
        type="text"
        placeholder="Search or enter address…"
        class="flex-1 h-7 px-3 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600
               rounded-full text-[13px] text-gray-800 dark:text-gray-200
               focus:outline-none focus:border-[var(--accent)] focus:ring-2 focus:ring-[var(--accent)]/20
               placeholder-gray-400 dark:placeholder-gray-500 transition-all min-w-0 mx-1"
      />

      <button
        onclick={navigate}
        class="shrink-0 px-3 h-7 bg-[var(--accent)] hover:opacity-90 text-white text-xs font-medium
               rounded-full transition-opacity"
      >Go</button>
    </div>
  </Titlebar>

  <!-- New tab page -->
  <div class="flex-1 flex flex-col items-center justify-center gap-7 overflow-auto py-8" role="main">

    <!-- Header -->
    <div class="flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-[var(--accent)] flex items-center justify-center shadow-sm shrink-0">
        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="1.6"
             stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 3a7 7 0 0 0-7 7v9l2.5-2 2.5 2 2.5-2 2.5 2 2.5-2V10a7 7 0 0 0-7-7z"/>
          <circle cx="9.5" cy="11" r="1" fill="white" stroke="none"/>
          <circle cx="14.5" cy="11" r="1" fill="white" stroke="none"/>
        </svg>
      </div>
      <div>
        <div class="text-base font-semibold text-[var(--text-primary)] leading-tight">Ghost</div>
        <div class="text-xs text-[var(--text-secondary)] leading-tight">No trace. Ever.</div>
      </div>
    </div>

    <!-- Search bar -->
    <div class="flex gap-2 w-full max-w-md px-4">
      <input
        bind:value={urlInput}
        onkeydown={onKeydown}
        onfocus={(e) => e.target.select()}
        type="text"
        placeholder="Search or enter address…"
        class="flex-1 h-10 px-4 bg-[var(--surface-raised)] border border-[var(--border)] rounded-xl text-sm
               text-[var(--text-primary)] focus:outline-none focus:border-[var(--accent)] focus:ring-2
               focus:ring-[var(--accent)]/20 placeholder-gray-400 dark:placeholder-gray-500 transition-all"
      />
      <button
        onclick={navigate}
        class="px-4 h-10 bg-[var(--accent)] hover:opacity-90 text-white text-sm font-medium
               rounded-xl transition-opacity"
      >Go</button>
    </div>

    <!-- Three columns -->
    <div class="flex gap-8 px-6">

      <!-- Search engines -->
      <div class="flex flex-col gap-0.5 min-w-[130px]">
        <div class="text-[10px] font-semibold text-[var(--text-secondary)] uppercase tracking-wider px-2 mb-1.5">Search</div>
        {#each searchLinks as link}
          <button
            onclick={() => go(link.url)}
            class="flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-[var(--surface-raised)]
                   transition-colors text-left w-full"
          >
            <span class="w-5 h-5 rounded flex items-center justify-center text-white
                         text-[9px] font-bold shrink-0 leading-none"
                  style="background:{link.color};">{link.badge}</span>
            <span class="text-sm text-[var(--text-primary)] font-medium">{link.name}</span>
          </button>
        {/each}
      </div>

      <div class="w-px bg-[var(--border)] self-stretch"></div>

      <!-- Software sources -->
      <div class="flex flex-col gap-0.5 min-w-[130px]">
        <div class="text-[10px] font-semibold text-[var(--text-secondary)] uppercase tracking-wider px-2 mb-1.5">Software</div>
        {#each softwareLinks as link}
          <button
            onclick={() => go(link.url)}
            class="flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-[var(--surface-raised)]
                   transition-colors text-left w-full"
          >
            <span class="w-5 h-5 rounded flex items-center justify-center text-white
                         text-[9px] font-bold shrink-0 leading-none"
                  style="background:{link.color};">{link.badge}</span>
            <span class="text-sm text-[var(--text-primary)] font-medium">{link.name}</span>
          </button>
        {/each}
      </div>

      <div class="w-px bg-[var(--border)] self-stretch"></div>

      <!-- Launch a real browser -->
      <div class="flex flex-col gap-0.5 min-w-[110px]">
        <div class="text-[10px] font-semibold text-[var(--text-secondary)] uppercase tracking-wider px-2 mb-1.5">Open Browser</div>
        {#each browsers as b}
          <button
            onclick={() => openBrowser(b.app)}
            class="flex items-center gap-2.5 px-2 py-1.5 rounded-lg hover:bg-[var(--surface-raised)]
                   transition-colors text-left w-full"
          >
            <span class="w-5 h-5 rounded flex items-center justify-center text-white
                         text-[9px] font-bold shrink-0 leading-none"
                  style="background:{b.color};">{b.badge}</span>
            <span class="text-sm text-[var(--text-primary)] font-medium">{b.name}</span>
          </button>
        {/each}
        <div class="text-[10px] text-[var(--text-secondary)] px-2 pt-2 leading-snug opacity-60">
          Launches the app<br>and closes this window.
        </div>
      </div>

    </div>
  </div>

  <!-- Download toast -->
  {#if downloadToast}
    <div class="absolute bottom-4 right-4 z-50 flex items-center gap-3 px-4 py-3
                bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700
                rounded-xl shadow-lg text-sm max-w-xs animate-fade-in">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
           class="text-[var(--accent)] shrink-0">
        <path d="M12 5v10M7 15l5 5 5-5"/>
        <path d="M5 20h14"/>
      </svg>
      <div class="min-w-0">
        <div class="font-medium text-gray-800 dark:text-gray-200 truncate">{downloadToast.filename}</div>
        <div class="text-[11px] text-gray-500 dark:text-gray-400">Saved to Downloads</div>
      </div>
    </div>
  {/if}

</WindowFrame>
