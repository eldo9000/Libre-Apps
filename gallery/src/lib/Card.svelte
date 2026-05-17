<script>
  import { fly } from 'svelte/transition';
  import { cubicOut } from 'svelte/easing';
  import { focus, setSingleFocus, toggleFocus, clearFocus } from './focus.svelte.js';
  import { canvas, PATTERNS } from './stores/canvas.svelte.js';

  let { id, label, sourceFile = null, component = null, effects = null, flush = false, children } = $props();

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
    const btnRect = e.currentTarget.getBoundingClientRect();
    brightnessPos = {
      top:  btnRect.bottom + 6,
      left: btnRect.left + btnRect.width / 2,
    };
    showBrightness = true;
  }

  function hsvToRgb(h, s, v) {
    s /= 100; v /= 100;
    const k = (n) => (n + h / 60) % 6;
    const f = (n) => v - v * s * Math.max(0, Math.min(k(n), 4 - k(n), 1));
    return [Math.round(f(5) * 255), Math.round(f(3) * 255), Math.round(f(1) * 255)];
  }

  // ── Generic effects always available on every card ──────────────
  // bg-*        → applied to the canvas layer behind the component
  // content-*   → applied to the wrapper around the component itself
  // frame-shadow → box-shadow on the tight frame around the component
  const GENERIC_EFFECTS = [
    { label: 'Blur', _generic: true, type: 'bg-filter',
      filterFn: (p) => `blur(${p.radius}px)`,
      params: [{ label: 'Radius', key: 'radius', unit: 'px', min: 0, max: 10, step: 0.5, value: 0 }] },
    { label: 'Overlay', _generic: true, type: 'bg-opacity',
      params: [
        { label: 'Amount', key: 'amount', unit: '%', min: 0, max: 100, step: 1,   value: 50  },
        { label: 'Hue',    key: 'hue',   unit: '°', min: 0, max: 360, step: 1,   value: 0   },
        { label: 'Sat',    key: 'sat',   unit: '%', min: 0, max: 100, step: 1,   value: 0   },
        { label: 'Val',    key: 'val',   unit: '%', min: 0, max: 100, step: 1,   value: 100 },
      ] },
    { label: 'Outer shadow', _generic: true, type: 'frame-outer',
      params: [
        { label: 'Distance', key: 'y',       unit: 'px', min: 0, max: 30,  step: 1, value: 4  },
        { label: 'Blur',     key: 'blur',    unit: 'px', min: 0, max: 40,  step: 1, value: 12 },
        { label: 'Opacity',  key: 'opacity', unit: '%',  min: 0, max: 100, step: 1, value: 30 },
      ] },
    { label: 'Border glow', _generic: true, type: 'frame-shadow',
      params: [
        { label: 'Blur',    key: 'blur',    unit: 'px', min: 0, max: 40,  step: 1, value: 12 },
        { label: 'Spread',  key: 'spread',  unit: 'px', min: 0, max: 10,  step: 1, value: 0  },
        { label: 'Opacity', key: 'opacity', unit: '%',  min: 0, max: 100, step: 1, value: 60 },
      ] },
  ];

  // Effects sidebar
  let showEffects = $state(false);

  function initEffects() {
    const base = [
      ...(effects ?? []).map(e => ({
        ...e, enabled: true,
        paramValues: Object.fromEntries((e.params ?? []).map(p => [p.key, p.value])),
      })),
      ...GENERIC_EFFECTS.map(e => ({
        ...e, enabled: false,
        paramValues: Object.fromEntries(e.params.map(p => [p.key, p.value])),
      })),
    ];
    try {
      const saved = JSON.parse(localStorage.getItem(`libre-eff-${id}`) ?? 'null');
      if (saved) {
        for (const e of base) {
          const s = saved.find(s => s.label === e.label);
          if (s) { e.enabled = s.enabled; e.paramValues = { ...e.paramValues, ...s.paramValues }; }
        }
      }
    } catch {}
    return base;
  }

  let effectStates = $state(initEffects());

  // Persist effect states whenever they change
  $effect(() => {
    const toSave = effectStates.map(e => ({
      label: e.label, enabled: e.enabled, paramValues: { ...e.paramValues },
    }));
    try { localStorage.setItem(`libre-eff-${id}`, JSON.stringify(toSave)); } catch {}
  });

  // CSS vars only — cascade to children for card-specific shadow effects
  const cssVarStyle = $derived.by(() => {
    return effectStates
      .filter(e => e.cssVar)
      .map(e => {
        if (!e.enabled) return `${e.cssVar}: ${e.cssOff ?? '0 0 0 0 transparent'}`;
        return e.template
          ? `${e.cssVar}: ${e.template(e.paramValues)}`
          : `${e.cssVar}: ${e.cssOn}`;
      })
      .join('; ');
  });

  // Background layer: canvas pattern + blur + color overlay (applied behind the component)
  const cardBgStyle = $derived.by(() => {
    const parts   = [cardBodyStyle].filter(Boolean);
    const filters = [];
    for (const e of effectStates) {
      if (!e.enabled) continue;
      if (e.type === 'bg-filter') filters.push(e.filterFn(e.paramValues));
      if (e.type === 'bg-opacity') {
        const [r, g, b] = hsvToRgb(e.paramValues.hue, e.paramValues.sat, e.paramValues.val);
        parts.push(`background-color: rgba(${r},${g},${b},${e.paramValues.amount / 100})`);
      }
    }
    if (filters.length) parts.push(`filter: ${filters.join(' ')}`);
    return parts.filter(Boolean).join('; ');
  });

  // Frame: compose border glow + outer drop shadow on the tight wrapper around the component
  const cardFrameStyle = $derived.by(() => {
    const shadows = [];
    const g = effectStates.find(e => e.type === 'frame-shadow' && e.enabled);
    if (g) {
      const v = g.paramValues;
      shadows.push(`0 0 ${v.blur}px ${v.spread}px color-mix(in srgb, var(--accent) ${v.opacity}%, transparent)`);
    }
    const o = effectStates.find(e => e.type === 'frame-outer' && e.enabled);
    if (o) {
      const v = o.paramValues;
      shadows.push(`0 ${v.y}px ${v.blur}px rgba(0,0,0,${v.opacity / 100})`);
    }
    return shadows.length ? `box-shadow: ${shadows.join(', ')}` : '';
  });

  function onEffectsClick(e) {
    e.stopPropagation();
    showEffects = !showEffects;
  }
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
    <button
      class="effects-btn"
      class:effects-btn-active={showEffects}
      onclick={onEffectsClick}
      aria-label="View visual effects"
    >
      <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <circle cx="12" cy="16" r="0.5" fill="currentColor"/>
      </svg>
    </button>
    <span class="focus-pip" class:active={isFocused} title={isFocused ? 'Focused — click to clear' : 'Click to focus'}>
      {isFocused ? '◉' : '○'}
    </span>
  </div>
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div class="card-body" class:effects-open={showEffects} style={cssVarStyle} onclick={handleBodyShiftClick}>
    <div class="card-bg" style={cardBgStyle}></div>
    <div class="card-content" class:flush>
      <div class="card-frame" class:flush style={cardFrameStyle}>
        {@render children?.()}
      </div>
    </div>
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

  {#if showEffects}
    <div
      class="effects-sidebar"
      transition:fly={{ x: 220, duration: 220, easing: cubicOut }}
    >
      {#each effectStates as effect, i}
        {#if effect._generic}
          <label class="effects-row">
            <input
              type="checkbox"
              class="effects-check"
              bind:checked={effectStates[i].enabled}
            />
            <span class="effects-label">{effect.label}</span>
          </label>
          {#if effect.enabled && effect.params?.length}
            <div class="effects-params">
              {#each effect.params as param}
                {@const pct = ((effectStates[i].paramValues[param.key] - param.min) / (param.max - param.min)) * 100}
                <div class="eff-param-row">
                  <span class="eff-param-lbl">{param.label}</span>
                  <input
                    type="range"
                    min={param.min}
                    max={param.max}
                    step={param.step ?? 1}
                    bind:value={effectStates[i].paramValues[param.key]}
                    class="eff-slider"
                    style="--pct: {pct}%"
                  />
                  <span class="eff-param-val">{effectStates[i].paramValues[param.key]}{param.unit ?? ''}</span>
                </div>
              {/each}
            </div>
          {/if}
        {/if}
      {/each}
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
    overflow: hidden;
  }
  .card.focused {
    border-color: var(--accent);
    box-shadow: var(--focus-ring, 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent));
  }

  .effects-divider {
    height: 1px;
    background: var(--border);
    margin: 3px 0;
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
    position: fixed;
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

  /* ── Effects info button & sidebar ── */
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
  .effects-btn-active { color: var(--accent); }

  .card-body.effects-open {
    min-height: 152px;
  }

  .effects-sidebar {
    position: absolute;
    top: 39px;
    right: 0;
    height: calc(100% - 39px);
    width: 220px;
    background: var(--surface-panel);
    border-left: 1px solid var(--border);
    z-index: 10;
    display: flex;
    flex-direction: column;
    overflow-y: scroll;
    scrollbar-width: thin;
    scrollbar-color: var(--border) transparent;
  }

  .effects-sidebar::-webkit-scrollbar { width: 4px; }
  .effects-sidebar::-webkit-scrollbar-track { background: transparent; }
  .effects-sidebar::-webkit-scrollbar-thumb { background: var(--border); border-radius: 2px; }

  .effects-none {
    font-size: 11px;
    color: var(--text-muted);
    padding: 10px 12px;
    font-style: italic;
  }

  .effects-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 7px 12px;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
    cursor: pointer;
    transition: background 80ms;
  }
  .effects-row:last-child { border-bottom: none; }
  .effects-row:hover { background: color-mix(in srgb, var(--surface-panel) 85%, black); }

  .effects-check {
    width: 13px;
    height: 13px;
    accent-color: var(--accent);
    cursor: pointer;
    flex-shrink: 0;
    margin: 0;
  }

  .effects-label {
    font-size: 11px;
    color: var(--text-secondary);
    flex: 1;
  }

  .effects-val {
    font-size: 10px;
    font-family: 'Geist Mono', 'Fira Code', monospace;
    color: var(--text-muted);
    text-align: right;
    white-space: nowrap;
  }

  .effects-params {
    padding: 2px 12px 8px 35px;
    display: flex;
    flex-direction: column;
    gap: 5px;
    border-bottom: 1px solid color-mix(in srgb, var(--border) 50%, transparent);
  }

  .eff-param-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .eff-param-lbl {
    font-size: 10px;
    color: var(--text-muted);
    width: 44px;
    flex-shrink: 0;
  }

  .eff-slider {
    flex: 1;
    height: 2px;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(to right, var(--accent) var(--pct), var(--border) var(--pct));
    border-radius: 2px;
    cursor: pointer;
    outline: none;
    min-width: 0;
  }

  .eff-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px;
    height: 10px;
    border-radius: 3px;
    background: var(--text-muted);
    border: 2px solid var(--surface-panel);
    cursor: pointer;
    transition: background 80ms;
  }

  .eff-slider:hover::-webkit-slider-thumb { background: #fff; }

  .eff-param-val {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    width: 30px;
    text-align: right;
    flex-shrink: 0;
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
    position: relative;
    min-height: 88px;
    border-radius: 0 0 10px 10px;
    overflow: hidden;
  }

  .card-bg {
    position: absolute;
    inset: 0;
    border-radius: 0 0 10px 10px;
    pointer-events: none;
    z-index: 0;
  }

  .card-content {
    position: relative;
    z-index: 1;
    padding: 28px 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    box-sizing: border-box;
  }

  .card-frame {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    flex-wrap: wrap;
    border-radius: 10px;
    width: 100%;
  }

  .card-content.flush {
    padding: 0;
    align-items: stretch;
  }
  .card-frame.flush {
    width: 100%;
    border-radius: 0;
    gap: 0;
    flex-wrap: nowrap;
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
