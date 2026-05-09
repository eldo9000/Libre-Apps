<script>
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

  const typeScale = [
    { size: '10px', weight: '400', label: 'xs / Section label muted' },
    { size: '11px', weight: '400', label: 'sm / Section labels' },
    { size: '12px', weight: '400', label: 'sm / Card labels' },
    { size: '13px', weight: '400', label: 'base- / Nav items, metadata' },
    { size: '14px', weight: '400', label: 'base / Body text' },
    { size: '14px', weight: '500', label: 'base medium / Nav active' },
    { size: '14px', weight: '600', label: 'base semibold / Button labels' },
    { size: '16px', weight: '600', label: 'lg / Dialog titles' },
    { size: '22px', weight: '600', label: 'xl / Page headings' },
  ];

  const spacing = [1, 2, 3, 4, 6, 8, 10, 12, 16];
</script>

<div class="section">
  <h1 class="page-title">Design Tokens</h1>

  <h2 class="group-title">Color</h2>
  <div class="color-grid">
    {#each colors as c}
      <div class="swatch">
        <div
          class="swatch-preview"
          style="background: var({c.name}); border: 1px solid var(--border);"
        ></div>
        <code class="swatch-token">{c.name}</code>
        <span class="swatch-label">{c.label}</span>
      </div>
    {/each}
  </div>

  <h2 class="group-title">Typography</h2>
  <div class="type-list">
    {#each typeScale as t}
      <div class="type-row">
        <span class="type-sample" style="font-size: {t.size}; font-weight: {t.weight};">
          The quick brown fox jumps
        </span>
        <span class="type-meta">{t.size} / w{t.weight} — {t.label}</span>
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

  .page-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: -0.02em;
    color: var(--text-primary);
    margin: 0 0 32px;
  }

  .group-title {
    font-size: 11px;
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
    height: 56px;
    border-radius: 8px;
  }

  .swatch-token {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--accent);
    letter-spacing: 0.02em;
  }

  .swatch-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .type-list {
    display: flex;
    flex-direction: column;
  }

  .type-row {
    display: flex;
    align-items: baseline;
    gap: 24px;
    padding: 10px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .type-sample {
    font-family: 'Geist', system-ui, sans-serif;
    color: var(--text-primary);
    min-width: 260px;
  }

  .type-meta {
    font-size: 11px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
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
