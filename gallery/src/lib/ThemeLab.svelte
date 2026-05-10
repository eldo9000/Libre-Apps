<script>
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';

  let { dark = $bindable(false) } = $props();

  const STORAGE_KEY = 'libre-theme-lab';
  const DEFAULTS = { accent: '#003f7d', deltaH: 0, deltaS: 0, deltaV: 0 };

  let saving    = $state(false);
  let loading   = $state(false);
  let saved     = $state(false);
  let collapsed = $state(localStorage.getItem('libre-theme-lab-collapsed') === 'true');

  function toggleCollapsed() {
    collapsed = !collapsed;
    localStorage.setItem('libre-theme-lab-collapsed', String(collapsed));
  }

  let draft = $state(loadDraft());

  const isDirty = $derived(
    draft.accent !== DEFAULTS.accent ||
    draft.deltaH !== 0 || draft.deltaS !== 0 || draft.deltaV !== 0
  );

  // ── HSV helpers ────────────────────────────────────────────────────────────

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

  // ── Dark accent derivation ─────────────────────────────────────────────────

  function getDarkAccent(accent, dH, dS, dV) {
    const { h, s, v } = hexToHsv(accent);
    return hsvToHex(
      (h + dH + 360) % 360,
      Math.max(0, Math.min(1, s + dS / 100)),
      Math.max(0, Math.min(1, v + dV / 100))
    );
  }

  const darkAccent = $derived(getDarkAccent(draft.accent, draft.deltaH, draft.deltaS, draft.deltaV));

  // ── Light HSV state ────────────────────────────────────────────────────────

  let { h: initH, s: initS, v: initV } = hexToHsv(draft.accent);
  let hue = $state(initH);
  let sat = $state(initS);
  let val = $state(initV);

  function onHueInput(e) {
    hue = +e.currentTarget.value;
    draft.accent = hsvToHex(hue, sat, val);
  }

  function onSatInput(e) {
    sat = +e.currentTarget.value / 100;
    draft.accent = hsvToHex(hue, sat, val);
  }

  function onValInput(e) {
    val = +e.currentTarget.value / 100;
    draft.accent = hsvToHex(hue, sat, val);
  }

  function onColorInput(e) {
    draft.accent = e.currentTarget.value;
    ({ h: hue, s: sat, v: val } = hexToHsv(draft.accent));
  }

  // ── Delta track background ─────────────────────────────────────────────────

  function deltaTrack(d, max = 20) {
    const pct  = (d + max) / (2 * max) * 100;
    const lo   = Math.min(pct, 50).toFixed(1);
    const hi   = Math.max(pct, 50).toFixed(1);
    const fill = 'color-mix(in srgb, var(--accent) 65%, transparent)';
    return `background: linear-gradient(to right, var(--border) ${lo}%, ${fill} ${lo}%, ${fill} ${hi}%, var(--border) ${hi}%)`;
  }

  // ── Persistence ─────────────────────────────────────────────────────────────

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
      accent: draft.accent,
      deltaH: draft.deltaH,
      deltaS: draft.deltaS,
      deltaV: draft.deltaV,
    }));
  });

  async function handleSave() {
    saving = true;
    try {
      const path = await save({
        title: 'Save Theme Preset',
        defaultPath: 'libre-theme.md',
        filters: [{ name: 'Markdown', extensions: ['md'] }],
      });
      if (!path) return;
      const payload = { accent: draft.accent, deltaH: draft.deltaH, deltaS: draft.deltaS, deltaV: draft.deltaV };
      const content = `# Libre Theme Preset\n\n\`\`\`json\n${JSON.stringify(payload, null, 2)}\n\`\`\`\n`;
      await invoke('write_preset', { path, content });
      await invoke('save_theme', { accent: draft.accent, accentHover: darken(draft.accent) });
      saved = true;
      setTimeout(() => { saved = false; }, 2000);
    } catch (e) {
      console.error('save failed:', e);
    } finally {
      saving = false;
    }
  }

  async function handleLoad() {
    loading = true;
    try {
      const path = await open({
        title: 'Load Theme Preset',
        filters: [{ name: 'Markdown', extensions: ['md'] }],
        multiple: false,
      });
      if (!path) return;
      const content = await invoke('read_preset', { path });
      const match = content.match(/```json\s*([\s\S]*?)```/);
      if (!match) throw new Error('No JSON block found');
      const parsed = JSON.parse(match[1]);
      if (parsed.accent) {
        draft.accent = parsed.accent;
        ({ h: hue, s: sat, v: val } = hexToHsv(draft.accent));
      }
      if (parsed.deltaH !== undefined) draft.deltaH = parsed.deltaH;
      if (parsed.deltaS !== undefined) draft.deltaS = parsed.deltaS;
      if (parsed.deltaV !== undefined) draft.deltaV = parsed.deltaV;
    } catch (e) {
      console.error('load failed:', e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="lab">
  <button class="lab-header" onclick={toggleCollapsed} aria-expanded={!collapsed}>
    <span class="lab-title">Theme Lab</span>
    <svg class="lab-chev" class:lab-chev-up={collapsed}
         width="12" height="12" viewBox="0 0 24 24" fill="none"
         stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <polyline points="6 9 12 15 18 9"/>
    </svg>
  </button>
  {#if !collapsed}
  <div class="lab-body">

    <!-- Dark mode delta -->
    <div class="delta-section">
      <div class="delta-header">
        <span class="delta-title">Dark Δ</span>
        <div class="delta-swatches">
          <span class="dot-swatch" style="background:{draft.accent}" title="Light accent"></span>
          <span class="dot-swatch" style="background:{darkAccent}" title="Dark accent"></span>
        </div>
      </div>
      <div class="hsv-row">
        <span class="hsv-lbl">H</span>
        <input type="range" class="hsv-slider" min="-20" max="20" step="1"
          value={draft.deltaH}
          oninput={e => draft.deltaH = +e.currentTarget.value}
          style={deltaTrack(draft.deltaH)}
        />
        <span class="delta-val">{draft.deltaH > 0 ? '+' : ''}{draft.deltaH}°</span>
      </div>
      <div class="hsv-row">
        <span class="hsv-lbl">S</span>
        <input type="range" class="hsv-slider" min="-20" max="20" step="1"
          value={draft.deltaS}
          oninput={e => draft.deltaS = +e.currentTarget.value}
          style={deltaTrack(draft.deltaS)}
        />
        <span class="delta-val">{draft.deltaS > 0 ? '+' : ''}{draft.deltaS}%</span>
      </div>
      <div class="hsv-row">
        <span class="hsv-lbl">V</span>
        <input type="range" class="hsv-slider" min="-20" max="20" step="1"
          value={draft.deltaV}
          oninput={e => draft.deltaV = +e.currentTarget.value}
          style={deltaTrack(draft.deltaV)}
        />
        <span class="delta-val">{draft.deltaV > 0 ? '+' : ''}{draft.deltaV}%</span>
      </div>
    </div>

    <!-- Light mode HSV sliders -->
    <div class="hsv-sliders">
      <div class="hsv-row">
        <span class="hsv-lbl">H</span>
        <input type="range" class="hsv-slider" min="0" max="359" value={hue} oninput={onHueInput}
          style="background: linear-gradient(to right,
            hsl(0,100%,45%),hsl(20,100%,45%),hsl(40,100%,45%),hsl(60,100%,45%),
            hsl(80,100%,45%),hsl(100,100%,45%),hsl(120,100%,35%),hsl(140,100%,35%),
            hsl(160,100%,35%),hsl(180,100%,38%),hsl(200,100%,42%),hsl(220,100%,45%),
            hsl(240,100%,50%),hsl(260,100%,50%),hsl(280,100%,45%),hsl(300,100%,40%),
            hsl(320,100%,40%),hsl(340,100%,42%),hsl(360,100%,45%))"
        />
      </div>
      <div class="hsv-row">
        <span class="hsv-lbl">S</span>
        <input type="range" class="hsv-slider" min="0" max="100" value={Math.round(sat * 100)} oninput={onSatInput}
          style="background: linear-gradient(to right, {hsvToHex(hue, 0, Math.max(val, 0.15))}, {hsvToHex(hue, 1, Math.max(val, 0.15))})"
        />
      </div>
      <div class="hsv-row">
        <span class="hsv-lbl">V</span>
        <input type="range" class="hsv-slider" min="0" max="100" value={Math.round(val * 100)} oninput={onValInput}
          style="background: linear-gradient(to right, #000, {hsvToHex(hue, sat, 1)})"
        />
      </div>
    </div>

    <!-- Accent swatches -->
    <div class="accent-row">
      <button
        class="dark-toggle"
        onclick={() => (dark = !dark)}
        data-tooltip="Toggle theme — L"
        aria-label="Toggle theme"
      >{dark ? '☀' : '☾'}</button>
      <input
        type="color"
        class="swatch"
        value={draft.accent}
        oninput={onColorInput}
      />
      <span class="hex">{draft.accent}</span>
      <span class="hex derived">{darkAccent}</span>
      {#if isDirty}
        <span class="unsaved" title="Unsaved changes"></span>
      {/if}
    </div>

    <!-- Actions -->
    <div class="actions">
      <button class="btn-load" onclick={handleLoad} disabled={loading}>
        {loading ? '···' : 'Load'}
      </button>
      <button class="btn-save" onclick={handleSave} disabled={saving}>
        {saving ? '···' : saved ? '✓' : 'Save'}
      </button>
    </div>

  </div>
  {/if}
</div>

<style>
  /* Theme switching — higher specificity (0-1-1) beats tokens.css :root (0-0-1) */
  :global(html:not(.dark)) {
    --accent:       var(--accent-light,       #003f7d);
    --accent-hover: var(--accent-light-hover, #003060);
  }
  :global(html.dark) {
    --accent:       var(--accent-dark,        #003f7d);
    --accent-hover: var(--accent-dark-hover,  #003060);
  }

  .lab {
    border-top: 1px solid var(--border);
  }

  .lab-header {
    width: 100%;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    background: none;
    border: none;
    border-bottom: 1px solid transparent;
    cursor: pointer;
    font-family: inherit;
    color: inherit;
    transition: background 0.1s;
  }
  .lab-header:hover { background: color-mix(in srgb, var(--surface-panel) 85%, var(--text-primary)); }

  .lab-title {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .lab-chev {
    color: var(--text-muted);
    transition: transform 0.2s ease;
  }
  .lab-chev-up { transform: rotate(180deg); }

  .lab-body {
    padding: 10px 12px 10px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  /* Delta section */
  .delta-section {
    display: flex;
    flex-direction: column;
    gap: 5px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .delta-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1px;
  }

  .delta-title {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .delta-swatches {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .dot-swatch {
    display: inline-block;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    border: 1px solid var(--border);
  }

  .delta-val {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    width: 28px;
    text-align: right;
    flex-shrink: 0;
  }

  /* HSV sliders */
  .hsv-sliders {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .hsv-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .hsv-lbl {
    font-size: 9px;
    font-weight: 700;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    width: 10px;
    flex-shrink: 0;
    text-align: center;
  }

  .hsv-slider {
    -webkit-appearance: none;
    appearance: none;
    flex: 1;
    height: 6px;
    border-radius: 3px;
    border: 1px solid var(--border);
    cursor: pointer;
    outline: none;
  }

  .hsv-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #fff;
    border: 1.5px solid rgba(0, 0, 0, 0.3);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    cursor: pointer;
  }

  .hsv-slider::-moz-range-thumb {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #fff;
    border: 1.5px solid rgba(0, 0, 0, 0.3);
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.4);
    cursor: pointer;
  }

  /* Accent row */
  .accent-row {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .dark-toggle {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0;
    font-size: 12px;
    line-height: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    transition: border-color 0.1s, color 0.1s;
  }
  .dark-toggle:hover { border-color: var(--accent); color: var(--text-primary); }

  .swatch {
    appearance: none;
    -webkit-appearance: none;
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px solid var(--border);
    cursor: pointer;
    padding: 0;
    background: none;
    flex-shrink: 0;
  }
  .swatch::-webkit-color-swatch-wrapper { padding: 0; }
  .swatch::-webkit-color-swatch { border: none; border-radius: 4px; }

  .hex {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }
  .derived { opacity: 0.6; }

  .unsaved {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent);
    margin-left: auto;
    flex-shrink: 0;
  }

  /* Actions */
  .actions { display: flex; gap: 6px; }

  .btn-load {
    flex: 1;
    padding: 5px 0;
    font-size: 11px;
    font-family: inherit;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 5px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: color 0.1s, border-color 0.1s;
  }
  .btn-load:hover:not(:disabled) { color: var(--text-primary); border-color: var(--text-muted); }
  .btn-load:disabled { opacity: 0.4; cursor: default; }

  .btn-save {
    flex: 1;
    padding: 5px 0;
    font-size: 11px;
    font-family: inherit;
    background: var(--accent);
    border: 1px solid var(--accent);
    border-radius: 5px;
    cursor: pointer;
    color: #fff;
    font-weight: 500;
    transition: opacity 0.1s;
  }
  .btn-save:hover:not(:disabled) { opacity: 0.88; }
  .btn-save:disabled { opacity: 0.5; cursor: default; }
</style>
