<script>
  import { WindowFrame, Titlebar } from '@libre/ui';
  import MediaViewer from './lib/MediaViewer.svelte';

  let filename = $state('Prism');
  let filepath = $state('');

  function onFileLoaded(name, path) {
    filename = name;
    filepath = path || '';
  }
</script>

<WindowFrame>
  <!-- Titlebar: icon + filename left, controls right -->
  <Titlebar>
    <div class="flex items-center gap-2 flex-1 min-w-0 pl-3" data-tauri-drag-region>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
           stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
           class="text-[var(--accent)] shrink-0">
        <path d="M9 15h-3a3 3 0 0 1 -3 -3v-6a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v3" />
        <path d="M9 12a3 3 0 0 1 3 -3h6a3 3 0 0 1 3 3v6a3 3 0 0 1 -3 3h-6a3 3 0 0 1 -3 -3l0 -6" />
        <path d="M3 12l2.296 -2.296a2.41 2.41 0 0 1 3.408 0l.296 .296" />
        <path d="M14 13.5v3l2.5 -1.5l-2.5 -1.5" />
        <path d="M7 6v.01" />
      </svg>
      <span
        class="text-[13px] font-medium text-gray-700 dark:text-gray-300 truncate"
        title={filepath || filename}
      >{filename}</span>
      {#if filepath}
        <span
          class="text-[11px] text-gray-400 dark:text-gray-500 truncate max-w-[240px] hidden sm:block"
          title={filepath}
          data-tauri-drag-region
        >{filepath}</span>
      {/if}
    </div>
  </Titlebar>

  <!-- Media viewer -->
  <div class="flex-1 min-h-0" role="main">
    <MediaViewer {onFileLoaded} />
  </div>
</WindowFrame>
