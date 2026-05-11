<script>
  import { focus, clearFocus } from './focus.svelte.js';

  let { collapsed = false } = $props();
</script>

<aside class="panels" class:panels-collapsed={collapsed}>
  <div class="panel-body" class:panel-body-hidden={collapsed}>
    {#if focus.cards.length > 0}
      <div class="focus-bar">
        {#if focus.cards.length === 1}
          <div class="focus-info">
            <code class="focus-id">{focus.cards[0].id}</code>
            <span class="focus-label">{focus.cards[0].label}</span>
          </div>
          {#if focus.cards[0].sourceFile}
            <span class="focus-file">{focus.cards[0].sourceFile}</span>
          {/if}
          <button class="focus-clear" onclick={clearFocus} title="Clear focus">✕</button>
        {:else}
          <div class="focus-info">
            <code class="focus-id">{focus.cards.length} selected</code>
            <span class="focus-label">{focus.cards.map(c => c.id).join(', ')}</span>
          </div>
          <button class="focus-clear" onclick={clearFocus} title="Clear all">✕</button>
        {/if}
      </div>
    {/if}
  </div>
</aside>

<style>
  .panels {
    display: flex;
    flex-direction: row;
    flex-shrink: 0;
    width: 304px;
    border-left: 1px solid var(--border);
    background: var(--surface-panel);
    transition: width 0.18s ease;
    overflow: hidden;
  }

  .panels-collapsed {
    width: 0;
  }

  .panel-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .panel-body-hidden {
    display: none;
  }

  .focus-bar {
    padding: 6px 10px;
    border-bottom: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
    min-height: 32px;
  }

  .focus-info {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    min-width: 0;
  }

  .focus-id {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text-primary);
    flex-shrink: 0;
  }

  .focus-label {
    font-size: 10px;
    color: var(--text-secondary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .focus-file {
    font-size: 8px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
    display: block;
  }

  .focus-clear {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 10px;
    color: var(--text-muted);
    padding: 0;
    flex-shrink: 0;
    transition: color 0.1s;
  }
  .focus-clear:hover { color: var(--text-primary); }
</style>
