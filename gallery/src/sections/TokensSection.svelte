<script>
  import { onMount } from 'svelte';
  import BezierEditor from '@libre/ui/src/components/BezierEditor.svelte';

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

  const motionTokens = [
    { name: '--ease-linear', label: 'Linear'      },
    { name: '--ease-hard',   label: 'Hard Ease'   },
    { name: '--ease-out',    label: 'Gentle Ease' },
  ];

  const zTokens = [
    { name: '--z-dropdown', label: 'Dropdown', value: 100 },
    { name: '--z-modal',    label: 'Modal',    value: 200 },
    { name: '--z-tooltip',  label: 'Tooltip',  value: 300 },
    { name: '--z-toast',    label: 'Toast',    value: 400 },
  ];

  const MOTION_STORAGE_KEY = 'libre-motion-tokens';

  const BEZIER_DEFAULTS = {
    '--ease-linear': [1/3,  1/3,  2/3,  2/3 ],
    '--ease-hard':   [0.9,  0,    0.1,  1   ],
    '--ease-out':    [0.25, 0.46, 0.45, 0.94],
  };
  const DURATION_DEFAULTS = {
    '--ease-linear': 600,
    '--ease-hard':   600,
    '--ease-out':    600,
  };

  function loadMotionStorage() {
    try {
      const raw = localStorage.getItem(MOTION_STORAGE_KEY);
      if (!raw) return null;
      return JSON.parse(raw);
    } catch { return null; }
  }

  function saveMotionStorage() {
    localStorage.setItem(MOTION_STORAGE_KEY, JSON.stringify({
      beziers:   Object.fromEntries(Object.entries(savedBeziers).map(([k, v]) => [k, [...v]])),
      durations: { ...durations },
    }));
  }

  const _stored = loadMotionStorage();

  let hsvMap = $state({});

  // Duration in ms per token — editable via the number input
  let durations = $state(_stored?.durations ?? { ...DURATION_DEFAULTS });

  // Live bezier values — editable via handle drag
  let beziers = $state(
    _stored?.beziers
      ? Object.fromEntries(Object.entries(_stored.beziers).map(([k, v]) => [k, [...v]]))
      : Object.fromEntries(Object.entries(BEZIER_DEFAULTS).map(([k, v]) => [k, [...v]]))
  );

  // Saved baseline — values committed via the Save button. Edits are
  // "dirty" until saved; Cancel reverts to this snapshot.
  let savedBeziers = $state(
    _stored?.beziers
      ? Object.fromEntries(Object.entries(_stored.beziers).map(([k, v]) => [k, [...v]]))
      : Object.fromEntries(Object.entries(BEZIER_DEFAULTS).map(([k, v]) => [k, [...v]]))
  );

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
  <div class="shadow-row">
    {#each shadowTokens as t}
      <div class="shadow-item">
        <div class="shadow-box" style="box-shadow: var({t.name});"></div>
        <code class="shadow-token">{t.name}</code>
      </div>
    {/each}
  </div>

  <!-- ── Motion ──────────────────────────────────────────────── -->
  <h2 class="group-title">Motion</h2>
  <div class="motion-row">
    {#each motionTokens as t}
      <BezierEditor
        name={t.name}
        bind:value={beziers[t.name]}
        bind:savedValue={savedBeziers[t.name]}
        bind:duration={durations[t.name]}
        onSave={saveMotionStorage}
      />
    {/each}
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
  .section { max-width: 900px; }

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
  .overlay-scrim {
    position: absolute;
    inset: 0;
    background: var(--overlay-bg);
  }
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
  .shadow-row { display: flex; gap: 32px; flex-wrap: wrap; align-items: flex-end; }
  .shadow-item { display: flex; flex-direction: column; align-items: center; gap: 12px; }
  .shadow-box {
    width: 72px;
    height: 48px;
    border-radius: 8px;
    background: var(--surface-raised);
  }
  .shadow-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }

  /* ── Motion ── */
  .motion-row {
    display: flex;
    gap: 32px;
    align-items: flex-start;
    width: max-content;
    max-width: none;
    flex-wrap: nowrap;
  }

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
