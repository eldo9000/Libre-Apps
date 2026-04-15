<script>
  let { queue, onadd, onremove, onclear } = $props();


  function statusColor(status) {
    switch (status) {
      case 'done': return 'text-green-500';
      case 'error': return 'text-red-500';
      case 'converting': return 'text-[var(--accent)]';
      default: return 'text-[var(--text-secondary)]';
    }
  }

  function statusIcon(status) {
    switch (status) {
      case 'done': return '✓';
      case 'error': return '✕';
      case 'converting': return '↻';
      default: return '·';
    }
  }

  // Drag-and-drop onto the queue panel itself
  let dropActive = $state(false);

  function onDragover(e) {
    e.preventDefault();
    e.stopPropagation();
    dropActive = true;
  }
  function onDragleave(e) {
    e.stopPropagation();
    dropActive = false;
  }
  function onDrop(e) {
    e.preventDefault();
    e.stopPropagation();
    dropActive = false;
    const paths = Array.from(e.dataTransfer?.files ?? []).map(f => f.path ?? f.name);
    if (paths.length) onadd?.(paths);
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
  class="flex flex-col h-full {dropActive ? 'ring-2 ring-inset ring-[var(--accent)]' : ''}"
  ondragover={onDragover}
  ondragleave={onDragleave}
  ondrop={onDrop}
>

  <!-- Header -->
  <div class="flex items-center justify-between px-3 py-2 border-b border-[var(--border)] shrink-0">
    <span class="text-[12px] font-medium text-[var(--text-secondary)] uppercase tracking-wide">
      Queue ({queue.length})
    </span>
    {#if queue.length > 0}
      <button
        onclick={() => onclear?.()}
        class="text-[11px] text-[var(--text-secondary)] hover:text-red-500 transition-colors"
        aria-label="Clear queue"
      >Clear</button>
    {/if}
  </div>

  <!-- File list -->
  <div class="flex-1 min-h-0 overflow-y-auto" role="list" aria-label="Files in queue">
    {#if queue.length === 0}
      <div class="flex flex-col items-center justify-center h-full gap-2 p-4 text-center">
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="1.5" class="text-[var(--border)]">
          <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
          <polyline points="17 8 12 3 7 8"/>
          <line x1="12" y1="3" x2="12" y2="15"/>
        </svg>
        <p class="text-[12px] text-[var(--text-secondary)] leading-relaxed">
          Drop files here<br>or drag from anywhere
        </p>
      </div>
    {:else}
      {#each queue as item (item.id)}
        <div
          role="listitem"
          class="flex items-center gap-2 px-3 py-2 border-b border-[var(--border)] group"
        >
          <!-- Status icon -->
          <span class="text-[13px] shrink-0 {statusColor(item.status)}
                       {item.status === 'converting' ? 'animate-spin' : ''}">
            {statusIcon(item.status)}
          </span>

          <!-- File name + progress -->
          <div class="flex-1 min-w-0">
            <p class="text-[12px] text-[var(--text-primary)] truncate leading-tight"
               title={item.path}>{item.name}</p>
            {#if item.status === 'converting'}
              <div class="mt-0.5 h-0.5 rounded-full bg-[var(--border)] overflow-hidden">
                <div class="h-full bg-[var(--accent)] transition-all duration-300"
                     style="width: {item.percent}%"></div>
              </div>
            {:else if item.status === 'error'}
              <p class="text-[11px] text-red-500 truncate">{item.error}</p>
            {:else if item.status === 'done'}
              <p class="text-[11px] text-green-500">Converted</p>
            {:else}
              <p class="text-[11px] text-[var(--text-secondary)] capitalize">{item.mediaType}</p>
            {/if}
          </div>

          <!-- Remove button -->
          <button
            onclick={() => onremove?.(item.id)}
            class="shrink-0 w-5 h-5 flex items-center justify-center rounded
                   text-[var(--text-secondary)] opacity-0 group-hover:opacity-100
                   hover:bg-red-100 dark:hover:bg-red-900/30 hover:text-red-500
                   transition-all text-[14px]"
            aria-label="Remove {item.name}"
          >×</button>
        </div>
      {/each}
    {/if}
  </div>

</div>
