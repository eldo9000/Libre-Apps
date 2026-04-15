<script>
  import { SHORTCUT_GROUPS } from './shortcuts.js';

  let { onClose } = $props();

  function handleKeydown(e) {
    if (e.key === 'Escape') {
      e.preventDefault();
      onClose();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Backdrop -->
<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
  onclick={onClose}
  onkeydown={(e) => { if (e.key === 'Escape') onClose(); }}
  role="dialog"
  aria-modal="true"
  aria-label="Keyboard shortcuts"
  tabindex="-1"
>
  <!-- Panel — stop click propagation so clicking inside doesn't close -->
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="relative bg-[var(--surface,#111827)] border border-[var(--border,#374151)] rounded-xl shadow-2xl w-[560px] max-h-[80vh] overflow-y-auto"
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Header -->
    <div class="flex items-center justify-between px-6 py-4 border-b border-[var(--border,#374151)] sticky top-0 bg-[var(--surface,#111827)] rounded-t-xl">
      <span class="text-[var(--text-primary,#f9fafb)] font-semibold text-sm">Keyboard Shortcuts</span>
      <button
        onclick={onClose}
        class="w-7 h-7 flex items-center justify-center rounded-lg text-[var(--text-secondary,#9ca3af)] hover:text-[var(--text-primary,#f9fafb)] hover:bg-white/10 transition-colors"
        aria-label="Close shortcuts overlay"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
          <path d="M18 6 6 18M6 6l12 12"/>
        </svg>
      </button>
    </div>

    <!-- Shortcut groups -->
    <div class="px-6 py-4 grid grid-cols-2 gap-x-8 gap-y-6">
      {#each SHORTCUT_GROUPS as group}
        <div>
          <h3 class="text-[var(--accent,#0066cc)] text-[11px] font-semibold uppercase tracking-widest mb-2.5">
            {group.group}
          </h3>
          <div class="flex flex-col gap-1.5">
            {#each group.items as item}
              <div class="flex items-center justify-between gap-4">
                <span class="text-[var(--text-secondary,#9ca3af)] text-[12px] leading-snug">
                  {item.description}
                </span>
                <kbd class="shrink-0 px-2 py-0.5 rounded bg-[var(--border,#374151)] text-[var(--text-primary,#f9fafb)] text-[11px] font-mono whitespace-nowrap border border-white/10">
                  {item.key}
                </kbd>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>

    <!-- Footer hint -->
    <div class="px-6 py-3 border-t border-[var(--border,#374151)] text-center">
      <span class="text-[var(--text-secondary,#6b7280)] text-[11px]">Press <kbd class="px-1.5 py-0.5 rounded bg-[var(--border,#374151)] text-[10px] font-mono">Escape</kbd> or click outside to close</span>
    </div>
  </div>
</div>
