<script>
  import { focus, setSingleFocus, toggleFocus, clearFocus } from './focus.svelte.js';
  import { canvas, PATTERNS } from './stores/canvas.svelte.js';

  let { id, label, sourceFile = null, component = null, effects = null, children } = $props();

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

  const onFoundation  = $derived(canvas.activeTab === 'foundation');
  const cardBodyStyle = $derived.by(() => {
    if (!onFoundation) return '';
    const b       = canvas.bgBrightness;
    const pattern = PATTERNS[canvas.bgPattern];
    const oc      = b < 0 ? `rgba(0,0,0,${Math.abs(b)/100})` : b > 0 ? `rgba(255,255,255,${b/100})` : null;
    const overlay = oc ? `linear-gradient(${oc}, ${oc})` : null;
    const images    = [overlay, pattern?.image].filter(Boolean).join(', ');
    const sizes     = [overlay ? 'auto' : null, pattern?.size].filter(Boolean).join(', ');
    const positions = [overlay ? '0 0' : null, pattern?.position].filter(Boolean).join(', ');
    if (!images) return '';
    return `background-image: ${images}; background-size: ${sizes};${positions ? ` background-position: ${positions};` : ''}`;
  });

  // Brightness slider popup — click to open at cursor, mouseleave to close
  let showBrightness  = $state(false);
  let brightnessPos   = $state({ top: 0, left: 0 });

  function onBrightnessClick(e) {
    e.stopPropagation();
    if (showBrightness) { showBrightness = false; return; }
    // Position relative to .card (position:relative), corrected for CSS zoom scale
    const btnRect  = e.currentTarget.getBoundingClientRect();
    const cardEl   = e.currentTarget.closest('.card');
    const cardRect = cardEl.getBoundingClientRect();
    const scale    = cardRect.width / cardEl.offsetWidth;
    brightnessPos  = {
      top:  (btnRect.bottom - cardRect.top + 6) / scale,
      left: (btnRect.left + btnRect.width / 2 - cardRect.left) / scale,
    };
    showBrightness = true;
  }

  // Effects popover
  let showEffects     = $state(false);
  let effectsHideTimer;
  let effectsBtnEl    = $state(null);
  let effectsPos      = $state({ top: 0, left: 0 });

  function onEffectsEnter() {
    clearTimeout(effectsHideTimer);
    if (effectsBtnEl) {
      const btnRect  = effectsBtnEl.getBoundingClientRect();
      const cardEl   = effectsBtnEl.closest('.card');
      const cardRect = cardEl.getBoundingClientRect();
      const scale    = cardRect.width / cardEl.offsetWidth;
      effectsPos = {
        top:  (btnRect.bottom - cardRect.top + 4) / scale,
        left: (btnRect.left + btnRect.width / 2 - cardRect.left) / scale,
      };
    }
    showEffects = true;
  }
  function onEffectsLeave()  { effectsHideTimer = setTimeout(() => showEffects = false, 140); }
  function onPopoverEnter()  { clearTimeout(effectsHideTimer); }
  function onPopoverLeave()  { effectsHideTimer = setTimeout(() => showEffects = false, 140); }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="card" class:focused={isFocused}>
  <div class="card-header" onclick={handleHeaderClick} role="button" tabindex="0">
    <code class="card-id">{id}</code>
    <span class="card-name">{label}</span>
    {#if onFoundation}
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div class="card-controls" onclick={e => e.stopPropagation()}>
        <button
          class="pat-btn"
          class:pat-active={canvas.bgBrightness !== 0 || showBrightness}
          onclick={onBrightnessClick}
          title="Canvas brightness"
        >
          <svg width="12" height="12" viewBox="0 0 12 12" aria-hidden="true">
            <circle cx="6" cy="6" r="5" stroke="currentColor" stroke-width="1" fill="none"/>
            <path d="M6 1a5 5 0 0 1 0 10Z" fill="currentColor"/>
          </svg>
        </button>
        <div class="canvas-sep"></div>
        <div class="os-toggle-inline">
          <button class="os-btn" class:os-active={canvas.osMode === 'macos'} onclick={() => canvas.osMode = 'macos'}>macOS</button>
          <button class="os-btn" class:os-active={canvas.osMode === 'windows'} onclick={() => canvas.osMode = 'windows'}>Windows</button>
        </div>
        <div class="canvas-sep"></div>
        <button class="pat-btn" class:pat-active={canvas.bgPattern === 'dots'} onclick={() => canvas.bgPattern = canvas.bgPattern === 'dots' ? 'none' : 'dots'} title="Dots">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
            <circle cx="2" cy="2" r="1"/><circle cx="6" cy="2" r="1"/><circle cx="10" cy="2" r="1"/>
            <circle cx="2" cy="6" r="1"/><circle cx="6" cy="6" r="1"/><circle cx="10" cy="6" r="1"/>
            <circle cx="2" cy="10" r="1"/><circle cx="6" cy="10" r="1"/><circle cx="10" cy="10" r="1"/>
          </svg>
        </button>
        <button class="pat-btn" class:pat-active={canvas.bgPattern === 'grid'} onclick={() => canvas.bgPattern = canvas.bgPattern === 'grid' ? 'none' : 'grid'} title="Grid">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1.2" stroke-linecap="square" aria-hidden="true">
            <line x1="4" y1="0" x2="4" y2="12"/><line x1="8" y1="0" x2="8" y2="12"/>
            <line x1="0" y1="4" x2="12" y2="4"/><line x1="0" y1="8" x2="12" y2="8"/>
          </svg>
        </button>
        <button class="pat-btn" class:pat-active={canvas.bgPattern === 'checker'} onclick={() => canvas.bgPattern = canvas.bgPattern === 'checker' ? 'none' : 'checker'} title="Checker">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
            <rect x="0" y="0" width="6" height="6"/><rect x="6" y="6" width="6" height="6"/>
          </svg>
        </button>
      </div>
    {/if}
    {#if effects !== null}
      <button
        bind:this={effectsBtnEl}
        class="effects-btn"
        onmouseenter={onEffectsEnter}
        onmouseleave={onEffectsLeave}
        onclick={e => e.stopPropagation()}
        aria-label="View visual effects"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="8" x2="12" y2="12"/>
          <circle cx="12" cy="16" r="0.5" fill="currentColor"/>
        </svg>
      </button>
    {/if}
    <span class="focus-pip" class:active={isFocused} title={isFocused ? 'Focused — click to clear' : 'Click to focus'}>
      {isFocused ? '◉' : '○'}
    </span>
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="card-body" style={cardBodyStyle} onclick={handleBodyShiftClick}>
    {@render children?.()}
  </div>
  {#if showBrightness && onFoundation}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="brightness-popup"
      style="top: {brightnessPos.top}px; left: {brightnessPos.left}px"
      role="tooltip"
      onmouseleave={() => showBrightness = false}
    >
      <span class="brightness-edge-label">lighter</span>
      <input
        type="range"
        class="brightness-slider"
        min="-100"
        max="100"
        step="5"
        value={canvas.bgBrightness}
        oninput={e => canvas.bgBrightness = Number(e.currentTarget.value)}
      />
      <span class="brightness-edge-label">darker</span>
      <button class="brightness-reset" onclick={() => canvas.bgBrightness = 0} title="Reset">
        {canvas.bgBrightness > 0 ? '+' : ''}{canvas.bgBrightness}%
      </button>
    </div>
  {/if}

  {#if showEffects && effects !== null}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="effects-popover"
      style="top: {effectsPos.top}px; left: {effectsPos.left}px"
      role="tooltip"
      onmouseenter={onPopoverEnter}
      onmouseleave={onPopoverLeave}
    >
      <div class="effects-hd">Visual Effects</div>
      {#if effects.length}
        {#each effects as e}
          <div class="effects-row">
            <span class="effects-label">{e.label}</span>
            {#if e.value}<code class="effects-val">{e.value}</code>{/if}
          </div>
        {/each}
      {:else}
        <div class="effects-none">No additional effects</div>
      {/if}
    </div>
  {/if}

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
    position: relative;
    border: 1px solid var(--border);
    border-radius: 10px;
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
    border-radius: 10px 10px 0 0;
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

  /* ── Canvas controls (foundation tab) ── */
  .card-controls {
    display: flex;
    align-items: center;
    gap: 4px;
    margin-right: 8px;
  }

  .os-toggle-inline { display: flex; align-items: center; }

  .os-btn {
    padding: 0 10px;
    height: 22px;
    font-size: 11px;
    font-family: inherit;
    border: 1px solid var(--border);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 80ms, color 80ms;
    line-height: 1;
  }
  .os-btn:first-child { border-radius: 4px 0 0 4px; }
  .os-btn:last-child  { border-radius: 0 4px 4px 0; border-left: none; }
  .os-active { background: var(--surface-raised); color: var(--text-primary); font-weight: 500; }

  .canvas-sep {
    width: 1px;
    height: 14px;
    background: var(--border);
    margin: 0 6px;
    flex-shrink: 0;
  }

  .pat-btn {
    width: 22px;
    height: 22px;
    border: 1px solid transparent;
    border-radius: 4px;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    transition: background 80ms, color 80ms, border-color 80ms;
  }
  .pat-btn:hover  { background: var(--surface-hover); color: var(--text-secondary); }
  .pat-active     { border-color: var(--border); background: var(--surface-raised); color: var(--text-primary); }

  /* ── Brightness slider popup ── */
  .brightness-popup {
    position: absolute;
    z-index: 9999;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 8px 10px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0 0 0 / 0.16), 0 1px 4px rgba(0 0 0 / 0.10);
    transform: translateX(-50%);
  }

  .brightness-edge-label {
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    user-select: none;
  }

  .brightness-slider {
    writing-mode: vertical-lr;
    direction: rtl;
    width: 4px;
    height: 100px;
    cursor: ns-resize;
    accent-color: var(--accent);
  }

  .brightness-reset {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 3px;
    transition: color 80ms, background 80ms;
    min-width: 36px;
    text-align: center;
  }
  .brightness-reset:hover { color: var(--text-primary); background: var(--surface-hover); }

  /* ── Effects info button & popover ── */
  .effects-btn {
    width: 22px;
    height: 22px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    padding: 0;
    flex-shrink: 0;
    transition: color 80ms;
  }
  .effects-btn:hover { color: var(--accent); }

  .effects-popover {
    position: absolute;
    z-index: 9999;
    min-width: 280px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 20px rgba(0 0 0 / 0.16), 0 1px 4px rgba(0 0 0 / 0.10);
    overflow: hidden;
    transform: translateX(-50%);
  }

  .effects-none {
    font-size: 11px;
    color: var(--text-muted);
    padding: 8px 10px;
    font-style: italic;
  }

  .effects-hd {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 7px 10px 5px;
    border-bottom: 1px solid var(--border);
  }

  .effects-row {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: 12px;
    padding: 5px 10px;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
  }
  .effects-row:last-child { border-bottom: none; }

  .effects-label {
    font-size: 11px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .effects-val {
    font-size: 10px;
    font-family: 'Geist Mono', 'Fira Code', monospace;
    color: var(--text-muted);
    text-align: right;
    white-space: nowrap;
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
    border-radius: 0 0 10px 10px;
    overflow: hidden;
  }

  /* Inspect overlay */
  .inspect-panel {
    border-top: 1px solid var(--border);
    background: var(--surface-panel);
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    border-radius: 0 0 10px 10px;
    overflow: hidden;
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
