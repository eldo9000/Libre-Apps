<script>
  import { onMount } from 'svelte';

  const colors = [
    { name: '--accent',         label: 'Accent' },
    { name: '--surface',        label: 'Surface' },
    { name: '--surface-raised', label: 'Surface Raised' },
    { name: '--surface-panel',  label: 'Surface Panel' },
    { name: '--border',         label: 'Border' },
    { name: '--border-subtle',  label: 'Border Subtle' },
    { name: '--text-primary',   label: 'Text Primary' },
    { name: '--text-secondary', label: 'Text Secondary' },
    { name: '--text-muted',     label: 'Text Muted' },
    { name: '--titlebar-bg',    label: 'Titlebar BG' },
    { name: '--tab-bar-bg',     label: 'Tab Bar BG' },
    { name: '--tab-active-bg',  label: 'Tab Active BG' },
  ];

  const spacing = [1, 2, 3, 4, 6, 8, 10, 12, 16];

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
    for (const c of colors) {
      map[c.name] = resolveHsv(c.name);
    }
    hsvMap = map;
  });
</script>

<div class="section">
  <h2 class="group-title">Color</h2>
  <div class="color-grid">
    {#each colors as c}
      <div class="swatch">
        <div
          class="swatch-preview"
          style="background: var({c.name}); border: 1px solid var(--border);"
        ></div>
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

  <h2 class="group-title">Spacing (4px base unit)</h2>
  <div class="spacing-row">
    {#each spacing as n}
      <div class="spacing-item">
        <div
          class="spacing-bar"
          style="width: {n * 4}px; height: {n * 4}px; background: var(--accent);"
        ></div>
        <span class="spacing-label">{n * 4}px</span>
      </div>
    {/each}
  </div>
</div>

<style>
  .section { max-width: 900px; }

  .group-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 32px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  .color-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 16px;
  }

  .swatch {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .swatch-preview {
    position: relative;
    height: 56px;
    border-radius: 8px;
  }

  .swatch-meta {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: flex-start;
    gap: 8px;
  }

  .swatch-names {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .swatch-token {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-secondary);
    letter-spacing: 0.02em;
  }

  .swatch-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .swatch-hsv-block {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 1px;
  }

  .swatch-hsv-row {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    gap: 3px;
  }

  .swatch-hsv-value {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-primary);
    line-height: 1.3;
  }

  .swatch-hsv-label {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    line-height: 1.3;
  }

  .spacing-row {
    display: flex;
    align-items: flex-end;
    gap: 20px;
    flex-wrap: wrap;
  }

  .spacing-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }

  .spacing-bar {
    border-radius: 2px;
    opacity: 0.7;
    min-width: 4px;
    min-height: 4px;
  }

  .spacing-label {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }
</style>
