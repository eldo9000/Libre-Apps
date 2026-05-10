<script>
  import { focus, setSingleFocus, toggleFocus, clearFocus } from './focus.svelte.js';

  let { id, label, sourceFile = null, component = null, children } = $props();

  const isFocused = $derived(focus.cards.some(c => c.id === id));

  let inspecting = $state(false);
  let tokens = $state([]);

  const TOKEN_NAMES = [
    '--accent', '--accent-hover', '--surface', '--surface-panel',
    '--surface-raised', '--border', '--border-subtle',
    '--text-primary', '--text-secondary', '--text-muted',
    '--radius', '--shadow',
  ];

  function handleHeaderClick(e) {
    if (e.shiftKey) {
      toggleFocus({ id, label, sourceFile, component });
    } else if (isFocused && focus.cards.length === 1) {
      clearFocus();
    } else {
      setSingleFocus({ id, label, sourceFile, component });
    }
  }

  function handleBodyShiftClick(e) {
    if (!e.shiftKey) return;
    e.preventDefault();
    inspecting = !inspecting;
    if (inspecting) {
      const style = getComputedStyle(document.documentElement);
      tokens = TOKEN_NAMES.map(name => ({
        name,
        value: style.getPropertyValue(name).trim() || '—',
      }));
    }
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="card" class:focused={isFocused}>
  <div class="card-header" onclick={handleHeaderClick} role="button" tabindex="0">
    <code class="card-id">{id}</code>
    <span class="card-name">{label}</span>
    <span class="focus-pip" class:active={isFocused} title={isFocused ? 'Focused — click to clear' : 'Click to focus'}>
      {isFocused ? '◉' : '○'}
    </span>
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="card-body" onclick={handleBodyShiftClick}>
    {@render children?.()}
  </div>
  {#if inspecting}
    <div class="inspect-panel">
      <div class="inspect-header">
        <span>CSS Tokens</span>
        <button class="inspect-close" onclick={() => inspecting = false}>✕</button>
      </div>
      {#each tokens as t}
        <div class="inspect-row">
          <span class="inspect-name">{t.name}</span>
          <span class="inspect-val">
            {#if /^#|^rgb|^hsl/.test(t.value)}
              <span class="swatch" style="background:{t.value}"></span>
            {/if}
            {t.value}
          </span>
        </div>
      {/each}
      {#if sourceFile}
        <div class="inspect-file">↗ {sourceFile}</div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .card {
    border: 1px solid var(--border);
    border-radius: 10px;
    overflow: hidden;
    background: var(--surface-raised);
    transition: border-color 0.12s;
  }
  .card.focused {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 14px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-panel);
    cursor: pointer;
    user-select: none;
    transition: background 0.1s;
  }
  .card-header:hover { background: color-mix(in srgb, var(--surface-panel) 85%, var(--accent)); }

  .card-id {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-primary);
    font-family: 'Geist Mono', 'Fira Code', monospace;
  }

  .card-name {
    font-size: 12px;
    color: var(--text-secondary);
    flex: 1;
  }

  .focus-pip {
    font-size: 11px;
    color: var(--text-muted);
    opacity: 0;
    transition: opacity 0.1s, color 0.1s;
    flex-shrink: 0;
  }
  .card-header:hover .focus-pip { opacity: 1; }
  .focus-pip.active { opacity: 1; color: var(--accent); }

  .card-body {
    padding: 28px 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 88px;
    gap: 12px;
    flex-wrap: wrap;
  }

  /* Inspect overlay */
  .inspect-panel {
    border-top: 1px solid var(--border);
    background: var(--surface-panel);
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
  }

  .inspect-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 5px 10px;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-subtle);
  }

  .inspect-close {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 10px;
    color: var(--text-muted);
    padding: 0;
    line-height: 1;
  }
  .inspect-close:hover { color: var(--text-primary); }

  .inspect-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 2px 10px;
    gap: 8px;
  }
  .inspect-row:nth-child(odd) { background: color-mix(in srgb, var(--surface-panel) 95%, #000); }

  .inspect-name { color: var(--text-muted); flex-shrink: 0; }

  .inspect-val {
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    gap: 5px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .swatch {
    width: 10px;
    height: 10px;
    border-radius: 2px;
    border: 1px solid rgba(0,0,0,0.15);
    flex-shrink: 0;
  }

  .inspect-file {
    padding: 4px 10px;
    color: var(--text-muted);
    border-top: 1px solid var(--border-subtle);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
