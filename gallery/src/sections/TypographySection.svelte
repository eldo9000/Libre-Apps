<script>
  const FONT_OPTIONS = [
    { name: 'Geist',               family: 'Geist, system-ui, sans-serif',                  gf: null },
    { name: 'Inter',               family: '"Inter", system-ui, sans-serif',                 gf: 'Inter:ital,opsz,wght@0,14..32,400;0,14..32,600;0,14..32,700' },
    { name: 'DM Sans',             family: '"DM Sans", system-ui, sans-serif',               gf: 'DM+Sans:wght@400;500;600;700' },
    { name: 'Plus Jakarta Sans',   family: '"Plus Jakarta Sans", system-ui, sans-serif',     gf: 'Plus+Jakarta+Sans:wght@400;500;600;700' },
    { name: 'Sora',                family: '"Sora", system-ui, sans-serif',                  gf: 'Sora:wght@400;500;600;700' },
    { name: 'Outfit',              family: '"Outfit", system-ui, sans-serif',                gf: 'Outfit:wght@400;500;600;700' },
    { name: 'Playfair Display',    family: '"Playfair Display", Georgia, serif',             gf: 'Playfair+Display:wght@400;600;700' },
    { name: 'Fraunces',            family: '"Fraunces", Georgia, serif',                     gf: 'Fraunces:opsz,wght@9..144,400;9..144,600;9..144,700' },
  ];

  const MONO_OPTIONS = [
    { name: 'Geist Mono',   family: '"Geist Mono", monospace',   gf: null },
    { name: 'JetBrains Mono', family: '"JetBrains Mono", monospace', gf: 'JetBrains+Mono:wght@400;500;600' },
    { name: 'Fira Code',    family: '"Fira Code", monospace',    gf: 'Fira+Code:wght@400;500;600' },
    { name: 'IBM Plex Mono',family: '"IBM Plex Mono", monospace',gf: 'IBM+Plex+Mono:wght@400;500;600' },
    { name: 'Inconsolata',  family: '"Inconsolata", monospace',  gf: 'Inconsolata:wght@400;500;600' },
  ];

  const TYPO_KEY = 'libre-typography';
  function _loadTypo() {
    try { return JSON.parse(localStorage.getItem(TYPO_KEY) ?? 'null'); } catch { return null; }
  }
  const _typo = _loadTypo();

  let headingFont = $state(_typo?.headingFont ?? 'Geist');
  let monoFont    = $state(_typo?.monoFont    ?? 'Geist Mono');

  let headingFamily = $derived(
    FONT_OPTIONS.find(f => f.name === headingFont)?.family ?? 'Geist, system-ui, sans-serif'
  );
  let monoFamily = $derived(
    MONO_OPTIONS.find(f => f.name === monoFont)?.family ?? '"Geist Mono", monospace'
  );

  const loaded = new Set(['Geist', 'Geist Mono']);

  function loadGF(gfParam) {
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = `https://fonts.googleapis.com/css2?family=${gfParam}&display=swap`;
    document.head.appendChild(link);
  }

  // Re-inject any non-default Google Fonts on init
  if (_typo?.headingFont && _typo.headingFont !== 'Geist') {
    const opt = FONT_OPTIONS.find(f => f.name === _typo.headingFont);
    if (opt?.gf) { loadGF(opt.gf); loaded.add(opt.name); }
  }
  if (_typo?.monoFont && _typo.monoFont !== 'Geist Mono') {
    const opt = MONO_OPTIONS.find(f => f.name === _typo.monoFont);
    if (opt?.gf) { loadGF(opt.gf); loaded.add(opt.name); }
  }

  $effect(() => {
    localStorage.setItem(TYPO_KEY, JSON.stringify({ headingFont, monoFont }));
  });

  function pickHeadingFont(e) {
    const name = e.target.value;
    headingFont = name;
    const opt = FONT_OPTIONS.find(f => f.name === name);
    if (opt?.gf && !loaded.has(name)) { loadGF(opt.gf); loaded.add(name); }
  }

  function pickMonoFont(e) {
    const name = e.target.value;
    monoFont = name;
    const opt = MONO_OPTIONS.find(f => f.name === name);
    if (opt?.gf && !loaded.has(name)) { loadGF(opt.gf); loaded.add(name); }
  }
</script>

<div class="section">

  <!-- Heading Hierarchy -->
  <div class="group-header">
    <h2 class="group-title">Heading Hierarchy</h2>
    <div class="font-picker">
      <span class="font-picker-label">Font</span>
      <span class="font-name-tag">{headingFont}</span>
      <div class="select-wrap">
        <select class="font-select" value={headingFont} onchange={pickHeadingFont} aria-label="Choose heading font">
          {#each FONT_OPTIONS as f}
            <option value={f.name}>{f.name}</option>
          {/each}
        </select>
        <svg class="select-chev" width="9" height="9" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </div>
    </div>
  </div>

  <div class="type-block">
    <div class="heading-row">
      <span class="heading-sample" style="font-size:32px;font-weight:700;letter-spacing:-0.03em;font-family:{headingFamily}">Display — The quick brown fox</span>
      <span class="type-meta">32px / 700 / −0.03em</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:22px;font-weight:600;letter-spacing:-0.02em;font-family:{headingFamily}">Heading 1 — The quick brown fox jumps</span>
      <span class="type-meta">22px / 600 / −0.02em</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:18px;font-weight:600;letter-spacing:-0.01em;font-family:{headingFamily}">Heading 2 — The quick brown fox jumps over</span>
      <span class="type-meta">18px / 600 / −0.01em</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:16px;font-weight:600;font-family:{headingFamily}">Heading 3 — The quick brown fox jumps over the lazy dog</span>
      <span class="type-meta">16px / 600</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:14px;font-weight:600;font-family:{headingFamily}">Heading 4 — The quick brown fox jumps over the lazy dog</span>
      <span class="type-meta">14px / 600</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:13px;font-weight:600;font-family:{headingFamily}">Heading 5 — The quick brown fox jumps over the lazy dog</span>
      <span class="type-meta">13px / 600</span>
    </div>
    <div class="heading-row">
      <span class="heading-sample" style="font-size:11px;font-weight:700;letter-spacing:0.08em;text-transform:uppercase;font-family:{headingFamily}">Heading 6 — Section label style</span>
      <span class="type-meta">11px / 700 / +0.08em / uppercase</span>
    </div>
  </div>

  <!-- Body & UI Scale -->
  <h2 class="group-title">Body &amp; UI Scale</h2>
  <div class="type-block">
    {#each [
      { size: '10px', weight: '400', label: 'Caption / section label muted'   },
      { size: '11px', weight: '400', label: 'Label small / metadata'           },
      { size: '12px', weight: '400', label: 'Label / card names'               },
      { size: '13px', weight: '400', label: 'Body small / nav items'           },
      { size: '14px', weight: '400', label: 'Body / default'                   },
      { size: '14px', weight: '500', label: 'Body medium / active nav'         },
      { size: '14px', weight: '600', label: 'Body semibold / button labels'    },
      { size: '16px', weight: '600', label: 'Large / dialog titles'            },
    ] as t}
      <div class="body-row">
        <span class="body-sample" style="font-size:{t.size};font-weight:{t.weight};">
          The quick brown fox jumps over the lazy dog
        </span>
        <span class="type-meta">{t.size} / w{t.weight} — {t.label}</span>
      </div>
    {/each}
  </div>

  <!-- Paragraph / Reading -->
  <h2 class="group-title">Paragraph &amp; Reading Text</h2>
  <div class="para-grid">
    <div class="para-block">
      <p class="para-label">14px / 400 / 1.5 line-height</p>
      <p class="para-text">
        Geist is the typeface used across all Libre apps. At 14px with a 1.5 line-height, body text is optimized for compact UI reading rather than long-form prose. Antialiasing is applied at the root level for consistency across platforms.
      </p>
      <p class="para-text">
        A second paragraph shows how spacing between blocks reads. The 4px base unit governs all margins — paragraph gap is typically 12–16px depending on density.
      </p>
    </div>
    <div class="para-block">
      <p class="para-label">13px / 400 / 1.5 — compact density</p>
      <p class="para-text" style="font-size:13px;">
        Compact variant used in sidebars, panels, and inspector UIs. Slightly tighter but retains the 1.5 line-height for readability. This size is used in the TurboTalk settings panel, Flicker inspector labels, and other dense control surfaces.
      </p>
      <p class="para-text" style="font-size:13px;">
        Metadata, descriptions, and supporting copy typically live at this size with <span style="color:var(--text-secondary)">--text-secondary</span> or <span style="color:var(--text-muted)">--text-muted</span> coloring.
      </p>
    </div>
  </div>

  <!-- Monospace -->
  <div class="group-header">
    <h2 class="group-title">Monospace / Code</h2>
    <div class="font-picker">
      <span class="font-picker-label">Font</span>
      <span class="font-name-tag">{monoFont}</span>
      <div class="select-wrap">
        <select class="font-select" value={monoFont} onchange={pickMonoFont} aria-label="Choose monospace font">
          {#each MONO_OPTIONS as f}
            <option value={f.name}>{f.name}</option>
          {/each}
        </select>
        <svg class="select-chev" width="9" height="9" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </div>
    </div>
  </div>

  <div class="type-block">
    {#each [
      { size: '9px',  sample: '--surface-raised: #1a1a1a',          label: 'Token labels, card IDs'     },
      { size: '10px', sample: 'color-mix(in srgb, var(--accent) 18%, var(--surface-panel))', label: 'Inspector values, metadata' },
      { size: '11px', sample: 'fn handle_event(event: TauriEvent) -> Result<()>',            label: 'Inline code'               },
      { size: '12px', sample: "import { invoke } from '@tauri-apps/api/core'",               label: 'Code block body'           },
      { size: '13px', sample: 'The quick brown fox → 0123456789',   label: 'Tabular / display'          },
    ] as t}
      <div class="mono-row">
        <code class="mono-sample" style="font-size:{t.size};font-family:{monoFamily};">{t.sample}</code>
        <span class="type-meta">{t.size} — {t.label}</span>
      </div>
    {/each}
  </div>

  <!-- Text Colors -->
  <h2 class="group-title">Text Colors</h2>
  <div class="color-text-grid">
    {#each [
      { token: '--text-primary',   label: 'Primary',         desc: 'Main content, headings, values'         },
      { token: '--text-secondary', label: 'Secondary',       desc: 'Supporting labels, descriptions'        },
      { token: '--text-muted',     label: 'Muted',           desc: 'Hints, metadata, disabled states'       },
      { token: '--accent',         label: 'Accent',          desc: 'Interactive, links, highlights'         },
    ] as c}
      <div class="color-text-row">
        <div class="color-text-swatch" style="background:var({c.token})"></div>
        <div class="color-text-body">
          <span class="color-text-sample" style="color:var({c.token});">
            The quick brown fox jumps over the lazy dog — {c.label}
          </span>
          <span class="color-text-meta">
            <code>var({c.token})</code> — {c.desc}
          </span>
        </div>
      </div>
    {/each}

    <!-- color-mix intermediates -->
    <div class="color-mix-divider">color-mix() intermediates</div>

    {#each [
      {
        expr: 'color-mix(in srgb, var(--text-secondary) 65%, var(--text-primary))',
        label: 'Between secondary → primary',
        desc: 'One step up from secondary; auto-shifts in both modes',
      },
      {
        expr: 'color-mix(in srgb, var(--text-muted) 65%, var(--text-secondary))',
        label: 'Between muted → secondary',
        desc: 'One step up from muted; useful for label text on panels',
      },
      {
        expr: 'color-mix(in srgb, var(--accent) 60%, var(--text-primary))',
        label: 'Accent dimmed',
        desc: 'Softened accent for secondary interactive text',
      },
      {
        expr: 'color-mix(in srgb, var(--accent) 15%, var(--text-primary))',
        label: 'Accent tinted primary',
        desc: 'Near-primary with a hint of brand color',
      },
    ] as c}
      <div class="color-text-row">
        <div class="color-text-swatch" style="background:{c.expr}"></div>
        <div class="color-text-body">
          <span class="color-text-sample" style="color:{c.expr};">
            The quick brown fox jumps over the lazy dog — {c.label}
          </span>
          <span class="color-text-meta">
            <code>{c.expr}</code>
          </span>
        </div>
      </div>
    {/each}
  </div>

  <!-- Label Patterns -->
  <h2 class="group-title">Label &amp; UI Text Patterns</h2>
  <div class="pattern-grid">
    <div class="pattern-item">
      <span class="pat-label">Section title</span>
      <span class="pat-sample pat-section-title">Section Title</span>
      <code class="pat-spec">9px / 700 / uppercase / +0.1em</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Nav item</span>
      <span class="pat-sample" style="font-size:13px;color:var(--text-primary);">Navigation Item</span>
      <code class="pat-spec">13px / 400 / text-primary</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Nav item active</span>
      <span class="pat-sample" style="font-size:13px;font-weight:500;color:var(--text-primary);">Active Nav Item</span>
      <code class="pat-spec">13px / 500 / text-primary</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Button label</span>
      <span class="pat-sample" style="font-size:13px;font-weight:600;color:var(--text-primary);">Button Label</span>
      <code class="pat-spec">13px / 600 / text-primary</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Card ID</span>
      <code class="pat-sample" style="font-size:10px;font-weight:600;letter-spacing:0.08em;text-transform:uppercase;color:var(--text-primary);font-family:'Geist Mono',monospace;">BTN-1</code>
      <code class="pat-spec">10px / 600 / mono / uppercase / +0.08em</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Inspector label</span>
      <span class="pat-sample" style="font-size:10px;color:var(--text-secondary);">Field label</span>
      <code class="pat-spec">10px / 400 / text-secondary</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Tooltip / hint</span>
      <span class="pat-sample" style="font-size:11px;color:var(--text-muted);">Press ⌘K to open</span>
      <code class="pat-spec">11px / 400 / text-muted</code>
    </div>
    <div class="pattern-item">
      <span class="pat-label">Monospace value</span>
      <code class="pat-sample" style="font-size:10px;color:var(--accent);font-family:'Geist Mono',monospace;font-variant-numeric:tabular-nums;">1920 × 1080</code>
      <code class="pat-spec">10px / mono / tabular-nums / accent</code>
    </div>
  </div>

</div>

<style>
  .section { max-width: 1075px; }

  /* Section header row — title + font picker aligned on one line */
  .group-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin: 40px 0 0;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  .group-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 0;
    flex-shrink: 0;
  }

  /* standalone group-title (not inside group-header) */
  h2.group-title {
    margin: 40px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  /* Font picker pill */
  .font-picker {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .font-picker-label {
    font-size: 10px;
    color: var(--text-muted);
    font-weight: 500;
    letter-spacing: 0.04em;
  }

  .font-name-tag {
    font-size: 11px;
    font-weight: 500;
    color: var(--accent);
    font-family: 'Geist Mono', monospace;
    background: color-mix(in srgb, var(--accent) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent) 25%, transparent);
    border-radius: 4px;
    padding: 1px 6px;
    white-space: nowrap;
  }

  .select-wrap {
    position: relative;
    display: flex;
    align-items: center;
  }

  .font-select {
    appearance: none;
    -webkit-appearance: none;
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    border: 1px solid var(--border);
    border-radius: 5px;
    color: var(--text-secondary);
    font-size: 11px;
    font-family: inherit;
    padding: 3px 22px 3px 8px;
    cursor: pointer;
    outline: none;
    transition: border-color 0.1s, background 0.1s, color 0.1s;
  }
  .font-select:hover {
    border-color: color-mix(in srgb, var(--text-primary) 30%, transparent);
    color: var(--text-primary);
  }
  .font-select:focus {
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .select-chev {
    position: absolute;
    right: 6px;
    pointer-events: none;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  /* Heading hierarchy */
  .type-block {
    display: flex;
    flex-direction: column;
    margin-top: 16px;
  }

  /* When type-block follows group-header (no spacing from margin-bottom: 16px on group-title) */
  .group-header + .type-block {
    margin-top: 16px;
  }

  .heading-row {
    display: flex;
    align-items: baseline;
    gap: 20px;
    padding: 10px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .heading-sample {
    color: var(--text-primary);
    line-height: 1.2;
    flex: 1;
    min-width: 0;
  }

  /* Body scale */
  .body-row {
    display: flex;
    align-items: baseline;
    gap: 24px;
    padding: 8px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .body-sample {
    color: var(--text-primary);
    font-family: Geist, system-ui, sans-serif;
    min-width: 320px;
    flex-shrink: 0;
  }

  .type-meta {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Paragraph */
  .para-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 32px;
  }

  .para-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .para-label {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    margin: 0 0 8px;
  }

  .para-text {
    font-size: 14px;
    line-height: 1.5;
    color: var(--text-primary);
    margin: 0 0 12px;
  }

  /* Monospace */
  .mono-row {
    display: flex;
    align-items: baseline;
    gap: 24px;
    padding: 8px 0;
    border-bottom: 1px solid var(--border-subtle);
  }

  .mono-sample {
    color: var(--text-primary);
    min-width: 380px;
    flex-shrink: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* Text colors */
  .color-text-grid {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .color-text-row {
    display: flex;
    align-items: center;
    gap: 14px;
    padding: 10px 12px;
    border-radius: 8px;
    background: var(--surface-raised);
    border: 1px solid var(--border-subtle);
  }

  .color-text-swatch {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    flex-shrink: 0;
    border: 1px solid var(--border-subtle);
  }

  .color-text-body {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .color-text-sample {
    font-size: 13px;
    line-height: 1.4;
  }

  .color-text-meta {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  .color-text-meta code {
    font-family: inherit;
  }

  .color-mix-divider {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 16px 0 6px;
  }

  /* Label patterns */
  .pattern-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 12px;
  }

  .pattern-item {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: 14px;
    background: var(--surface-raised);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
  }

  .pat-label {
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .pat-sample {
    font-family: Geist, system-ui, sans-serif;
    line-height: 1.4;
  }

  .pat-section-title {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-secondary);
  }

  .pat-spec {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    margin-top: 2px;
  }
</style>
