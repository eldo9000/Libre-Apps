<script>
  import { onMount } from 'svelte';

  let { dark = $bindable(false) } = $props();

  // ── Accent editor ────────────────────────────────────────────────────────────

  const STORAGE_KEY   = 'libre-theme-lab';
  const SWATCHES_KEY  = 'libre-theme-swatches';
  const DEFAULTS = { accent: '#2884c9', deltaH: 0, deltaS: 0, deltaV: 0 };

  let saved          = $state(false);
  let savedSwatches  = $state(loadSwatches());

  let draft = $state(loadDraft());

  function hexToHsv(hex) {
    const r = parseInt(hex.slice(1, 3), 16) / 255;
    const g = parseInt(hex.slice(3, 5), 16) / 255;
    const b = parseInt(hex.slice(5, 7), 16) / 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b), d = max - min;
    let h = 0;
    if (d !== 0) {
      if      (max === r) h = ((g - b) / d + 6) % 6;
      else if (max === g) h = (b - r) / d + 2;
      else                h = (r - g) / d + 4;
      h = Math.round(h * 60);
    }
    return { h, s: max === 0 ? 0 : d / max, v: max };
  }

  function hsvToHex(h, s, v) {
    const c = v * s, x = c * (1 - Math.abs((h / 60) % 2 - 1)), m = v - c;
    let r, g, b;
    if      (h < 60)  { r = c; g = x; b = 0; }
    else if (h < 120) { r = x; g = c; b = 0; }
    else if (h < 180) { r = 0; g = c; b = x; }
    else if (h < 240) { r = 0; g = x; b = c; }
    else if (h < 300) { r = x; g = 0; b = c; }
    else              { r = c; g = 0; b = x; }
    return '#' + [r + m, g + m, b + m]
      .map(n => Math.round(n * 255).toString(16).padStart(2, '0'))
      .join('');
  }

  function getDarkAccent(accent, dH, dS, dV) {
    const { h, s, v } = hexToHsv(accent);
    return hsvToHex(
      (h + dH + 360) % 360,
      Math.max(0, Math.min(1, s + dS / 100)),
      Math.max(0, Math.min(1, v + dV / 100))
    );
  }

  const darkAccent = $derived(getDarkAccent(draft.accent, draft.deltaH, draft.deltaS, draft.deltaV));

  let { h: initH, s: initS, v: initV } = hexToHsv(draft.accent);
  let hue = $state(initH);
  let sat = $state(initS);
  let val = $state(initV);

  function onHueInput(e) { hue = +e.currentTarget.value; draft.accent = hsvToHex(hue, sat, val); }
  function onSatInput(e) { sat = +e.currentTarget.value / 100; draft.accent = hsvToHex(hue, sat, val); }
  function onValInput(e) { val = +e.currentTarget.value / 100; draft.accent = hsvToHex(hue, sat, val); }
  function onColorInput(e) { draft.accent = e.currentTarget.value; ({ h: hue, s: sat, v: val } = hexToHsv(draft.accent)); }

  function deltaTrack(d, max = 20) {
    const pct = (d + max) / (2 * max) * 100;
    const lo  = Math.min(pct, 50).toFixed(1);
    const hi  = Math.max(pct, 50).toFixed(1);
    const fill = 'color-mix(in srgb, var(--accent) 65%, transparent)';
    return `background: linear-gradient(to right, var(--border) ${lo}%, ${fill} ${lo}%, ${fill} ${hi}%, var(--border) ${hi}%)`;
  }

  function loadDraft() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) return { ...DEFAULTS, ...JSON.parse(stored) };
    } catch {}
    return { ...DEFAULTS };
  }

  function darken(hex, amount = 0.13) {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    return '#' + [r, g, b]
      .map(c => Math.max(0, Math.round(c * (1 - amount))).toString(16).padStart(2, '0'))
      .join('');
  }

  $effect(() => {
    const root = document.documentElement;
    root.style.setProperty('--accent-light',       draft.accent);
    root.style.setProperty('--accent-light-hover', darken(draft.accent));
    root.style.setProperty('--accent-dark',        darkAccent);
    root.style.setProperty('--accent-dark-hover',  darken(darkAccent));
    localStorage.setItem(STORAGE_KEY, JSON.stringify({
      accent: draft.accent, deltaH: draft.deltaH, deltaS: draft.deltaS, deltaV: draft.deltaV,
    }));
  });

  function loadSwatches() {
    try {
      const stored = localStorage.getItem(SWATCHES_KEY);
      if (stored) return JSON.parse(stored);
    } catch {}
    return [];
  }

  function handleSave() {
    const next = [draft.accent, ...savedSwatches].slice(0, 6);
    savedSwatches = next;
    localStorage.setItem(SWATCHES_KEY, JSON.stringify(next));
    saved = true;
    setTimeout(() => { saved = false; }, 1200);
  }

  function applyHistorySwatch(color) {
    draft.accent = color;
    ({ h: hue, s: sat, v: val } = hexToHsv(color));
  }

  // ── Token display ────────────────────────────────────────────────────────────

  const colorGroups = [
    {
      label: 'Surface & Chrome',
      tokens: [
        { name: '--accent',         label: 'Accent' },
        { name: '--accent-hover',   label: 'Accent Hover' },
        { name: '--titlebar-bg',    label: 'Titlebar BG' },
        { name: '--surface',        label: 'Surface' },
        { name: '--surface-raised', label: 'Surface Raised' },
        { name: '--surface-panel',  label: 'Surface Panel' },
        { name: '--surface-hint',   label: 'Surface Hint' },
        { name: '--surface-hover',  label: 'Surface Hover' },
        { name: '--surface-active', label: 'Surface Active' },
        { name: '--tab-bar-bg',     label: 'Tab Bar BG' },
        { name: '--tab-active-bg',  label: 'Tab Active BG' },
      ],
    },
    {
      label: 'Border & Text',
      tokens: [
        { name: '--border',          label: 'Border' },
        { name: '--border-subtle',   label: 'Border Subtle' },
        { name: '--text-primary',    label: 'Text Primary' },
        { name: '--text-secondary',  label: 'Text Secondary' },
        { name: '--text-muted',      label: 'Text Muted' },
      ],
    },
    {
      label: 'Status',
      tokens: [
        { name: '--color-success',    label: 'Success' },
        { name: '--color-success-bg', label: 'Success BG' },
        { name: '--color-warning',    label: 'Warning' },
        { name: '--color-warning-bg', label: 'Warning BG' },
        { name: '--color-danger',     label: 'Danger' },
        { name: '--color-danger-bg',  label: 'Danger BG' },
        { name: '--color-info',       label: 'Info' },
        { name: '--color-info-bg',    label: 'Info BG' },
      ],
    },
  ];

  const spacingTokens = [
    { name: '--space-1',  px: 4 },
    { name: '--space-2',  px: 8 },
    { name: '--space-3',  px: 12 },
    { name: '--space-4',  px: 16 },
    { name: '--space-5',  px: 20 },
    { name: '--space-6',  px: 24 },
    { name: '--space-8',  px: 32 },
    { name: '--space-10', px: 40 },
    { name: '--space-12', px: 48 },
  ];

  const radiusTokens = [
    { name: '--radius-sm',   label: 'sm',   px: 4 },
    { name: '--radius-md',   label: 'md',   px: 6 },
    { name: '--radius-lg',   label: 'lg',   px: 10 },
    { name: '--radius-full', label: 'full', px: 9999 },
  ];

  const shadowTokens = [
    { name: '--shadow-sm', label: 'sm' },
    { name: '--shadow-md', label: 'md' },
    { name: '--shadow-lg', label: 'lg' },
  ];

  const darkShadows = [
    { name: '--shadow-sm-dark', label: 'sm', value: '0 1px 3px rgba(255,255,255,0.10), 0 1px 6px rgba(255,255,255,0.07)' },
    { name: '--shadow-md-dark', label: 'md', value: '0 4px 10px rgba(255,255,255,0.09), 0 2px 16px rgba(255,255,255,0.06)' },
    { name: '--shadow-lg-dark', label: 'lg', value: '0 8px 20px rgba(255,255,255,0.09), 0 4px 32px rgba(255,255,255,0.06)' },
  ];

  const zTokens = [
    { name: '--z-dropdown', label: 'Dropdown', value: 100 },
    { name: '--z-modal',    label: 'Modal',    value: 200 },
    { name: '--z-tooltip',  label: 'Tooltip',  value: 300 },
    { name: '--z-toast',    label: 'Toast',    value: 400 },
  ];

  let hsvMap = $state({});

  function rgbToHsv(r, g, b) {
    r /= 255; g /= 255; b /= 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    const d = max - min;
    let h = 0;
    if (d !== 0) {
      if (max === r) h = ((g - b) / d + 6) % 6;
      else if (max === g) h = (b - r) / d + 2;
      else h = (r - g) / d + 4;
      h = Math.round(h * 60);
    }
    const s = max === 0 ? 0 : Math.round((d / max) * 100);
    const v = Math.round(max * 100);
    return { h, s, v };
  }

  function resolveHsv(tokenName) {
    const el = document.createElement('div');
    el.style.cssText = `position:absolute;width:1px;height:1px;background:var(${tokenName})`;
    document.body.appendChild(el);
    const raw = getComputedStyle(el).backgroundColor;
    document.body.removeChild(el);
    const m = raw.match(/\d+/g);
    if (!m) return null;
    return rgbToHsv(+m[0], +m[1], +m[2]);
  }

  onMount(() => {
    const map = {};
    for (const g of colorGroups) {
      for (const c of g.tokens) {
        map[c.name] = resolveHsv(c.name);
      }
    }
    hsvMap = map;
  });
</script>

<div class="section">

  <!-- ── Accent Editor ─────────────────────────────────────────── -->
  <div class="accent-editor">
    <div class="ae-header">
      <span class="ae-title">Accent Color</span>
      <div class="ae-swatches">
        <span class="ae-dot" style="background:{draft.accent}" title="Light accent"></span>
        <span class="ae-dot" style="background:{darkAccent}" title="Dark accent"></span>
      </div>
    </div>

    <div class="ae-body">
      <!-- Left: light mode controls -->
      <div class="ae-col">
        <div class="ae-col-label">Light</div>
        <div class="ae-slider-row">
          <span class="ae-lbl">H</span>
          <input type="range" class="ae-slider" min="0" max="359" value={hue} oninput={onHueInput}
            style="background: linear-gradient(to right,
              hsl(0,100%,45%),hsl(20,100%,45%),hsl(40,100%,45%),hsl(60,100%,45%),
              hsl(80,100%,45%),hsl(100,100%,45%),hsl(120,100%,35%),hsl(140,100%,35%),
              hsl(160,100%,35%),hsl(180,100%,38%),hsl(200,100%,42%),hsl(220,100%,45%),
              hsl(240,100%,50%),hsl(260,100%,50%),hsl(280,100%,45%),hsl(300,100%,40%),
              hsl(320,100%,40%),hsl(340,100%,42%),hsl(360,100%,45%))"
          />
          <span class="ae-val">{hue}°</span>
        </div>
        <div class="ae-slider-row">
          <span class="ae-lbl">S</span>
          <input type="range" class="ae-slider" min="0" max="100" value={Math.round(sat * 100)} oninput={onSatInput}
            style="background: linear-gradient(to right, {hsvToHex(hue, 0, Math.max(val, 0.15))}, {hsvToHex(hue, 1, Math.max(val, 0.15))})"
          />
          <span class="ae-val">{Math.round(sat * 100)}%</span>
        </div>
        <div class="ae-slider-row">
          <span class="ae-lbl">V</span>
          <input type="range" class="ae-slider" min="0" max="100" value={Math.round(val * 100)} oninput={onValInput}
            style="background: linear-gradient(to right, #000, {hsvToHex(hue, sat, 1)})"
          />
          <span class="ae-val">{Math.round(val * 100)}%</span>
        </div>
      </div>

      <!-- Divider -->
      <div class="ae-divider"></div>

      <!-- Right: dark delta controls -->
      <div class="ae-col">
        <div class="ae-col-label">Dark Δ</div>
        <div class="ae-slider-row">
          <span class="ae-lbl">H</span>
          <input type="range" class="ae-slider" min="-20" max="20" step="1"
            value={draft.deltaH} oninput={e => draft.deltaH = +e.currentTarget.value}
            style={deltaTrack(draft.deltaH)}
          />
          <span class="ae-val">{draft.deltaH > 0 ? '+' : ''}{draft.deltaH}°</span>
        </div>
        <div class="ae-slider-row">
          <span class="ae-lbl">S</span>
          <input type="range" class="ae-slider" min="-20" max="20" step="1"
            value={draft.deltaS} oninput={e => draft.deltaS = +e.currentTarget.value}
            style={deltaTrack(draft.deltaS)}
          />
          <span class="ae-val">{draft.deltaS > 0 ? '+' : ''}{draft.deltaS}%</span>
        </div>
        <div class="ae-slider-row">
          <span class="ae-lbl">V</span>
          <input type="range" class="ae-slider" min="-20" max="20" step="1"
            value={draft.deltaV} oninput={e => draft.deltaV = +e.currentTarget.value}
            style={deltaTrack(draft.deltaV)}
          />
          <span class="ae-val">{draft.deltaV > 0 ? '+' : ''}{draft.deltaV}%</span>
        </div>
      </div>

      <!-- Right: swatch + actions -->
      <div class="ae-actions-col">
        <div class="ae-swatch-row">
          <input type="color" class="ae-color-swatch" value={draft.accent} oninput={onColorInput} />
          <div class="ae-hex-stack">
            <span class="ae-hex">{draft.accent}</span>
            <span class="ae-hex ae-hex-dark">{darkAccent}</span>
          </div>
        </div>
        <div class="ae-history-row">
          {#each { length: 6 } as _, i}
            {#if savedSwatches[i]}
              <button
                class="ae-history-swatch"
                style="background:{savedSwatches[i]}"
                onclick={() => applyHistorySwatch(savedSwatches[i])}
                title={savedSwatches[i]}
              ></button>
            {:else}
              <div class="ae-history-empty"></div>
            {/if}
          {/each}
          <button class="ae-btn-save" onclick={handleSave}>
            {saved ? '✓' : 'Save'}
          </button>
        </div>
      </div>
    </div>
  </div>

  <!-- ── Color ──────────────────────────────────────────────── -->
  {#each colorGroups as group}
    <h2 class="group-title">{group.label}</h2>
    <div class="color-grid">
      {#each group.tokens as c}
        <div class="swatch">
          <div class="swatch-preview" style="background: var({c.name}); border: 1px solid var(--border);"></div>
          <div class="swatch-meta">
            <div class="swatch-names">
              <code class="swatch-token">{c.name}</code>
              <span class="swatch-label">{c.label}</span>
            </div>
            {#if hsvMap[c.name]}
              <div class="swatch-hsv-block">
                {#each [['h','H'],['s','S'],['v','V']] as [key, ltr]}
                  <div class="swatch-hsv-row">
                    <span class="swatch-hsv-value">{hsvMap[c.name][key]}</span>
                    <span class="swatch-hsv-label">{ltr}</span>
                  </div>
                {/each}
              </div>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {/each}

  <!-- ── Overlay ─────────────────────────────────────────────── -->
  <h2 class="group-title">Overlay</h2>
  <div class="overlay-demo">
    <div class="overlay-surface">
      <div class="overlay-scrim"></div>
      <span class="overlay-label">--overlay-bg</span>
    </div>
    <code class="overlay-code">rgb(0 0 0 / 50%)</code>
  </div>

  <!-- ── Spacing ─────────────────────────────────────────────── -->
  <h2 class="group-title">Spacing</h2>
  <div class="spacing-row">
    {#each spacingTokens as t}
      <div class="spacing-item">
        <div class="spacing-bar" style="width: {t.px}px; height: {t.px}px;"></div>
        <code class="spacing-token">{t.name}</code>
        <span class="spacing-label">{t.px}px</span>
      </div>
    {/each}
  </div>

  <!-- ── Border Radius ───────────────────────────────────────── -->
  <h2 class="group-title">Border Radius</h2>
  <div class="radius-row">
    {#each radiusTokens as t}
      <div class="radius-item">
        <div class="radius-box" style="border-radius: var({t.name});"></div>
        <code class="radius-token">{t.name}</code>
        <span class="radius-label">{t.px >= 9999 ? '∞' : t.px + 'px'}</span>
      </div>
    {/each}
  </div>

  <!-- ── Shadows ─────────────────────────────────────────────── -->
  <h2 class="group-title">Shadows</h2>
  <div class="shadow-variants">
    <div class="shadow-variant">
      <span class="shadow-mode-label">Light</span>
      <div class="shadow-row">
        {#each shadowTokens as t}
          <div class="shadow-item">
            <div class="shadow-box" style="box-shadow: var({t.name});"></div>
            <code class="shadow-token">{t.name}</code>
          </div>
        {/each}
      </div>
    </div>
    <div class="shadow-variant">
      <span class="shadow-mode-label">Dark</span>
      <div class="shadow-row">
        {#each darkShadows as t}
          <div class="shadow-item">
            <div class="shadow-box shadow-box-dark" style="box-shadow: {t.value};"></div>
            <code class="shadow-token">{t.name}</code>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- ── Z-index ─────────────────────────────────────────────── -->
  <h2 class="group-title">Z-Index</h2>
  <div class="z-row">
    {#each zTokens as t}
      <div class="z-item" style="--z-h: {(t.value / 400) * 64}px;">
        <div class="z-bar"></div>
        <code class="z-token">{t.name}</code>
        <span class="z-value">{t.value}</span>
        <span class="z-label">{t.label}</span>
      </div>
    {/each}
  </div>

</div>

<style>
  /* Theme switching — higher specificity (0-1-1) beats tokens.css :root (0-0-1) */
  :global(html:not(.dark)) {
    --accent:       var(--accent-light,       #2884c9);
    --accent-hover: var(--accent-light-hover, #2373b0);
  }
  :global(html.dark) {
    --accent:       var(--accent-dark,        #2884c9);
    --accent-hover: var(--accent-dark-hover,  #2373b0);
  }

  .section { max-width: 1125px; }

  /* ── Accent Editor ── */
  .accent-editor {
    margin-bottom: 8px;
    border: 1px solid var(--border);
    border-radius: 10px;
    background: var(--surface-raised);
    overflow: hidden;
  }

  .ae-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 16px 9px;
    border-bottom: 1px solid var(--border);
  }

  .ae-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    flex: 1;
  }

  .ae-swatches { display: flex; gap: 5px; align-items: center; }

  .ae-dot {
    display: inline-block;
    width: 11px;
    height: 11px;
    border-radius: 50%;
    border: 1px solid var(--border);
  }

  .ae-body {
    display: flex;
    align-items: stretch;
    gap: 0;
    padding: 14px 16px;
    gap: 20px;
  }

  .ae-col {
    display: flex;
    flex-direction: column;
    gap: 7px;
    flex: 1;
    min-width: 0;
  }

  .ae-col-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 2px;
  }

  .ae-divider {
    width: 1px;
    background: var(--border);
    flex-shrink: 0;
    align-self: stretch;
    margin: 0 4px;
  }

  .ae-slider-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .ae-lbl {
    font-size: 9px;
    font-weight: 700;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    width: 10px;
    flex-shrink: 0;
    text-align: center;
  }

  .ae-slider {
    -webkit-appearance: none;
    appearance: none;
    flex: 1;
    height: 6px;
    border-radius: 3px;
    border: 1px solid var(--border);
    cursor: pointer;
    outline: none;
  }
  .ae-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #fff;
    border: 1.5px solid rgba(0,0,0,0.28);
    box-shadow: 0 1px 3px rgba(0,0,0,0.35);
    cursor: pointer;
  }
  .ae-slider::-moz-range-thumb {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: #fff;
    border: 1.5px solid rgba(0,0,0,0.28);
    box-shadow: 0 1px 3px rgba(0,0,0,0.35);
    cursor: pointer;
  }

  .ae-val {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    width: 34px;
    text-align: right;
    flex-shrink: 0;
  }

  .ae-actions-col {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    gap: 12px;
    flex-shrink: 0;
  }

  .ae-swatch-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .ae-color-swatch {
    appearance: none;
    -webkit-appearance: none;
    width: 36px;
    height: 36px;
    border-radius: 8px;
    border: 1px solid var(--border);
    cursor: pointer;
    padding: 0;
    background: none;
    flex-shrink: 0;
  }
  .ae-color-swatch::-webkit-color-swatch-wrapper { padding: 0; }
  .ae-color-swatch::-webkit-color-swatch { border: none; border-radius: 7px; }

  .ae-hex-stack {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .ae-hex {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-secondary);
  }

  .ae-hex-dark {
    color: var(--text-muted);
    opacity: 0.7;
  }

  .ae-history-row {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .ae-history-swatch {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px solid var(--border);
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
    transition: transform 0.1s, border-color 0.1s;
  }
  .ae-history-swatch:hover { transform: scale(1.15); border-color: var(--text-muted); }

  .ae-history-empty {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px dashed color-mix(in srgb, var(--border) 60%, transparent);
    flex-shrink: 0;
  }

  .ae-btn-save {
    padding: 5px 14px;
    font-size: 11px;
    font-family: inherit;
    background: var(--accent);
    border: 1px solid var(--accent);
    border-radius: 5px;
    cursor: pointer;
    color: #fff;
    font-weight: 500;
    transition: opacity 0.1s;
    white-space: nowrap;
  }
  .ae-btn-save:hover:not(:disabled) { opacity: 0.88; }
  .ae-btn-save:disabled { opacity: 0.5; cursor: default; }

  /* ── Group titles ── */
  .group-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 36px 0 14px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  /* ── Color swatches ── */
  .color-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 16px;
  }
  .swatch { display: flex; flex-direction: column; gap: 6px; }
  .swatch-preview { height: 56px; border-radius: 8px; }
  .swatch-meta { display: flex; flex-direction: row; justify-content: space-between; align-items: flex-start; gap: 8px; }
  .swatch-names { display: flex; flex-direction: column; gap: 6px; }
  .swatch-token { font-size: 10px; font-family: 'Geist Mono', monospace; color: var(--text-secondary); letter-spacing: 0.02em; }
  .swatch-label { font-size: 12px; color: var(--text-secondary); }
  .swatch-hsv-block { display: flex; flex-direction: column; align-items: flex-end; gap: 1px; }
  .swatch-hsv-row { display: flex; flex-direction: row; align-items: baseline; gap: 3px; }
  .swatch-hsv-value { font-size: 10px; font-family: 'Geist Mono', monospace; color: var(--text-primary); line-height: 1.3; }
  .swatch-hsv-label { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); line-height: 1.3; }

  /* ── Overlay ── */
  .overlay-demo { display: flex; align-items: center; gap: 20px; }
  .overlay-surface {
    position: relative;
    width: 120px;
    height: 56px;
    border-radius: 8px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    overflow: hidden;
  }
  .overlay-scrim { position: absolute; inset: 0; background: var(--overlay-bg); }
  .overlay-label {
    position: relative;
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: #fff;
    text-align: center;
    display: block;
    padding-top: 20px;
  }
  .overlay-code { font-size: 11px; font-family: 'Geist Mono', monospace; color: var(--text-secondary); }

  /* ── Spacing ── */
  .spacing-row { display: flex; align-items: flex-end; gap: 20px; flex-wrap: wrap; }
  .spacing-item { display: flex; flex-direction: column; align-items: center; gap: 6px; }
  .spacing-bar {
    background: var(--accent);
    border-radius: 2px;
    opacity: 0.7;
    min-width: 4px;
    min-height: 4px;
  }
  .spacing-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }
  .spacing-label { font-size: 10px; font-family: 'Geist Mono', monospace; color: var(--text-secondary); }

  /* ── Border radius ── */
  .radius-row { display: flex; align-items: flex-end; gap: 28px; flex-wrap: wrap; }
  .radius-item { display: flex; flex-direction: column; align-items: center; gap: 8px; }
  .radius-box {
    width: 56px;
    height: 56px;
    background: var(--surface-raised);
    border: 2px solid var(--accent);
  }
  .radius-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }
  .radius-label { font-size: 11px; font-family: 'Geist Mono', monospace; color: var(--text-secondary); }

  /* ── Shadows ── */
  .shadow-variants { display: flex; flex-direction: column; gap: 20px; }
  .shadow-variant { display: flex; align-items: center; gap: 20px; }
  .shadow-mode-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    width: 28px;
    flex-shrink: 0;
  }
  .shadow-row { display: flex; gap: 32px; flex-wrap: wrap; align-items: flex-end; }
  .shadow-item { display: flex; flex-direction: column; align-items: center; gap: 12px; }
  .shadow-box {
    width: 72px;
    height: 48px;
    border-radius: 8px;
    background: var(--surface-raised);
  }
  .shadow-box-dark {
    background: #1a1a1f;
  }
  .shadow-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }

  /* ── Z-index ── */
  .z-row { display: flex; align-items: flex-end; gap: 24px; }
  .z-item { display: flex; flex-direction: column; align-items: center; gap: 6px; }
  .z-bar {
    width: 40px;
    height: var(--z-h, 16px);
    background: color-mix(in srgb, var(--accent) 60%, transparent);
    border-radius: 4px 4px 0 0;
    border: 1px solid var(--accent);
    border-bottom: none;
  }
  .z-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }
  .z-value { font-size: 11px; font-family: 'Geist Mono', monospace; color: var(--text-primary); font-weight: 600; }
  .z-label { font-size: 10px; color: var(--text-secondary); }
</style>
