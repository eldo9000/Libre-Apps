<script>
  import Card from '../lib/Card.svelte';

  // ── Settings panel state ───────────────────────────────────────────────────
  let activeCategory = $state('appearance');
  let darkMode        = $state(false);
  let accentHue       = $state(206);
  let accentSat       = $state(80);
  let accentVal       = $state(79);
  let fontSize        = $state('medium');
  let density         = $state('compact');
  let wallpaper       = $state('gradient-blue');

  // ── Apps panel state ───────────────────────────────────────────────────────
  let selectedApp     = $state('shelf');

  const categories = [
    { id: 'appearance',   label: 'Appearance',    icon: '◑' },
    { id: 'apps',         label: 'Applications',  icon: '⊞' },
    { id: 'display',      label: 'Display',       icon: '▣' },
    { id: 'sound',        label: 'Sound',         icon: '◈' },
    { id: 'network',      label: 'Network',       icon: '⊕' },
    { id: 'privacy',      label: 'Privacy',       icon: '⊛' },
    { id: 'updates',      label: 'Updates',       icon: '↻' },
    { id: 'about',        label: 'About',         icon: 'ⓘ' },
  ];

  const wallpapers = [
    { id: 'gradient-blue',   label: 'Ocean',   color: 'linear-gradient(135deg,#1a3a5c,#2884c9)' },
    { id: 'gradient-purple', label: 'Dusk',    color: 'linear-gradient(135deg,#2d1b4e,#7c3aed)' },
    { id: 'gradient-slate',  label: 'Slate',   color: 'linear-gradient(135deg,#1e293b,#334155)' },
    { id: 'gradient-forest', label: 'Forest',  color: 'linear-gradient(135deg,#14532d,#16a34a)' },
    { id: 'solid-dark',      label: 'Noir',    color: '#0a0a0a' },
    { id: 'solid-sand',      label: 'Sand',    color: 'linear-gradient(135deg,#78350f,#d97706)' },
  ];

  const apps = [
    { id: 'shelf',     name: 'Shelf',     role: 'File Manager',       version: '1.2.0', status: 'running', icon: '⊟' },
    { id: 'stack',     name: 'Stack',     role: 'Markdown Editor',    version: '0.9.4', status: 'idle',    icon: '⊕' },
    { id: 'prism',     name: 'Prism',     role: 'Media Viewer',       version: '1.0.1', status: 'idle',    icon: '◈' },
    { id: 'fade',      name: 'Fade',      role: 'Media Converter',    version: '0.8.2', status: 'running', icon: '◑' },
    { id: 'avalanche', name: 'Avalanche', role: 'Privacy Browser',    version: '0.5.0', status: 'idle',    icon: '⊞' },
    { id: 'flicker',   name: 'Flicker',   role: 'Screen Capture',     version: '1.1.3', status: 'running', icon: '⊛' },
  ];

  const accentColor = $derived(() => {
    const h = accentHue;
    const s = accentSat / 100;
    const v = accentVal / 100;
    const f = (n) => {
      const k = (n + h / 60) % 6;
      return v - v * s * Math.max(0, Math.min(k, 4 - k, 1));
    };
    const r = Math.round(f(5) * 255);
    const g = Math.round(f(3) * 255);
    const b = Math.round(f(1) * 255);
    return `rgb(${r},${g},${b})`;
  });

  const selectedAppData = $derived(apps.find(a => a.id === selectedApp));
</script>

<div class="section">

  <!-- LW-1: Full settings panel -->
  <Card id="LW-1" label="Settings" sourceFile="gallery/src/sections/LibreWinSection.svelte">
    <div class="lw-settings">
      <!-- sidebar -->
      <div class="lw-sidebar">
        <div class="lw-sidebar-header">
          <span class="lw-logo">◈</span>
          <span class="lw-title">LibreWin</span>
        </div>
        <nav class="lw-nav">
          {#each categories as cat}
            <button
              class="lw-nav-item {activeCategory === cat.id ? 'lw-nav-active' : ''}"
              onclick={() => activeCategory = cat.id}
            >
              <span class="lw-nav-icon">{cat.icon}</span>
              <span class="lw-nav-label">{cat.label}</span>
            </button>
          {/each}
        </nav>
      </div>

      <!-- pane -->
      <div class="lw-pane">
        {#if activeCategory === 'appearance'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Appearance</h2>
            <p class="lw-pane-sub">Visual style applied across all Libre apps.</p>
          </div>

          <div class="lw-section-label">Theme</div>
          <div class="lw-theme-row">
            <button
              class="lw-theme-btn {!darkMode ? 'lw-theme-active' : ''}"
              onclick={() => darkMode = false}
            >
              <div class="lw-theme-preview lw-preview-light">
                <div class="lw-preview-bar"></div>
                <div class="lw-preview-content">
                  <div class="lw-preview-line" style="width:60%"></div>
                  <div class="lw-preview-line" style="width:40%"></div>
                </div>
              </div>
              <span class="lw-theme-label">Light</span>
            </button>
            <button
              class="lw-theme-btn {darkMode ? 'lw-theme-active' : ''}"
              onclick={() => darkMode = true}
            >
              <div class="lw-theme-preview lw-preview-dark">
                <div class="lw-preview-bar lw-preview-bar-dark"></div>
                <div class="lw-preview-content">
                  <div class="lw-preview-line lw-preview-line-dark" style="width:60%"></div>
                  <div class="lw-preview-line lw-preview-line-dark" style="width:40%"></div>
                </div>
              </div>
              <span class="lw-theme-label">Dark</span>
            </button>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Accent Color</div>
          <div class="lw-accent-preview-row">
            <div class="lw-accent-dot" style="background:{accentColor()}"></div>
            <span class="lw-accent-hex">{accentColor()}</span>
          </div>
          <div class="lw-slider-group">
            <div class="lw-slider-row">
              <span class="lw-slider-label">Hue</span>
              <input type="range" class="lw-slider" min="0" max="360" bind:value={accentHue} />
              <span class="lw-slider-val">{accentHue}°</span>
            </div>
            <div class="lw-slider-row">
              <span class="lw-slider-label">Saturation</span>
              <input type="range" class="lw-slider" min="0" max="100" bind:value={accentSat} />
              <span class="lw-slider-val">{accentSat}%</span>
            </div>
            <div class="lw-slider-row">
              <span class="lw-slider-label">Brightness</span>
              <input type="range" class="lw-slider" min="0" max="100" bind:value={accentVal} />
              <span class="lw-slider-val">{accentVal}%</span>
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Wallpaper</div>
          <div class="lw-wallpaper-grid">
            {#each wallpapers as wp}
              <button
                class="lw-wp-btn {wallpaper === wp.id ? 'lw-wp-active' : ''}"
                onclick={() => wallpaper = wp.id}
                title={wp.label}
              >
                <div class="lw-wp-swatch" style="background:{wp.color}"></div>
                <span class="lw-wp-label">{wp.label}</span>
              </button>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Interface</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Font Size</div>
                <div class="lw-field-hint">Base size for all app text.</div>
              </div>
              <div class="lw-seg">
                {#each ['small','medium','large'] as sz}
                  <button
                    class="lw-seg-btn {fontSize === sz ? 'lw-seg-on' : ''}"
                    onclick={() => fontSize = sz}
                  >{sz}</button>
                {/each}
              </div>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Density</div>
                <div class="lw-field-hint">Controls padding and row height.</div>
              </div>
              <div class="lw-seg">
                {#each ['compact','default','relaxed'] as d}
                  <button
                    class="lw-seg-btn {density === d ? 'lw-seg-on' : ''}"
                    onclick={() => density = d}
                  >{d}</button>
                {/each}
              </div>
            </div>
          </div>

        {:else if activeCategory === 'apps'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Applications</h2>
            <p class="lw-pane-sub">Installed Libre apps and their status.</p>
          </div>

          <div class="lw-apps-layout">
            <div class="lw-app-list">
              {#each apps as app}
                <button
                  class="lw-app-row {selectedApp === app.id ? 'lw-app-row-active' : ''}"
                  onclick={() => selectedApp = app.id}
                >
                  <span class="lw-app-icon">{app.icon}</span>
                  <div class="lw-app-meta">
                    <span class="lw-app-name">{app.name}</span>
                    <span class="lw-app-role">{app.role}</span>
                  </div>
                  <span class="lw-status-dot lw-status-{app.status}"></span>
                </button>
              {/each}
            </div>

            {#if selectedAppData}
              <div class="lw-app-detail">
                <div class="lw-app-detail-icon">{selectedAppData.icon}</div>
                <div class="lw-app-detail-name">{selectedAppData.name}</div>
                <div class="lw-app-detail-role">{selectedAppData.role}</div>
                <div class="lw-app-detail-rows">
                  <div class="lw-detail-row">
                    <span class="lw-detail-key">Version</span>
                    <span class="lw-detail-val">{selectedAppData.version}</span>
                  </div>
                  <div class="lw-detail-row">
                    <span class="lw-detail-key">Status</span>
                    <span class="lw-detail-val lw-detail-{selectedAppData.status}">{selectedAppData.status}</span>
                  </div>
                  <div class="lw-detail-row">
                    <span class="lw-detail-key">Binary</span>
                    <span class="lw-detail-val lw-detail-mono">/usr/local/bin/{selectedAppData.id}</span>
                  </div>
                  <div class="lw-detail-row">
                    <span class="lw-detail-key">Config</span>
                    <span class="lw-detail-val lw-detail-mono">~/.config/librewin/</span>
                  </div>
                </div>
                <div class="lw-app-actions">
                  <button class="lw-btn-secondary">Open</button>
                  <button class="lw-btn-secondary">Permissions</button>
                  <button class="lw-btn-danger">Uninstall</button>
                </div>
              </div>
            {/if}
          </div>

        {:else}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">{categories.find(c => c.id === activeCategory)?.label}</h2>
            <p class="lw-pane-sub">This panel is a placeholder — not yet designed.</p>
          </div>
          <div class="lw-placeholder">
            <span class="lw-placeholder-icon">{categories.find(c => c.id === activeCategory)?.icon}</span>
            <span class="lw-placeholder-label">Coming soon</span>
          </div>
        {/if}
      </div>
    </div>
  </Card>

</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    gap: 24px;
    padding: 24px;
  }

  /* ── Shell ─────────────────────────────────────────────────────────────── */
  .lw-settings {
    display: flex;
    height: 520px;
    border-radius: 8px;
    overflow: hidden;
    border: 1px solid var(--border);
    background: var(--surface);
  }

  /* ── Sidebar ───────────────────────────────────────────────────────────── */
  .lw-sidebar {
    width: 192px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface) 60%, var(--text-primary) 5%);
    display: flex;
    flex-direction: column;
  }

  .lw-sidebar-header {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 16px 14px 12px;
    border-bottom: 1px solid var(--border);
  }

  .lw-logo {
    font-size: 16px;
    color: var(--accent);
  }

  .lw-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .lw-nav {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px 6px;
    flex: 1;
    overflow-y: auto;
  }

  .lw-nav-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 8px;
    border-radius: 6px;
    border: none;
    background: none;
    cursor: pointer;
    text-align: left;
    color: var(--text-secondary);
    font-size: 13px;
    transition: background 120ms, color 120ms;
  }

  .lw-nav-item:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
    color: var(--text-primary);
  }

  .lw-nav-active {
    background: color-mix(in srgb, var(--accent) 15%, transparent) !important;
    color: var(--accent) !important;
    font-weight: 500;
  }

  .lw-nav-icon {
    font-size: 14px;
    width: 18px;
    text-align: center;
    flex-shrink: 0;
  }

  .lw-nav-label {
    font-size: 12.5px;
  }

  /* ── Pane ──────────────────────────────────────────────────────────────── */
  .lw-pane {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
    display: flex;
    flex-direction: column;
  }

  .lw-pane-header {
    margin-bottom: 20px;
  }

  .lw-pane-title {
    font-size: 17px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 4px;
    letter-spacing: -0.02em;
  }

  .lw-pane-sub {
    font-size: 12px;
    color: var(--text-muted);
    margin: 0;
  }

  .lw-section-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 8px;
  }

  /* ── Theme picker ──────────────────────────────────────────────────────── */
  .lw-theme-row {
    display: flex;
    gap: 12px;
  }

  .lw-theme-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    background: none;
    border: 2px solid var(--border);
    border-radius: 8px;
    padding: 8px;
    cursor: pointer;
    transition: border-color 120ms;
  }

  .lw-theme-btn:hover {
    border-color: color-mix(in srgb, var(--border) 40%, var(--text-muted));
  }

  .lw-theme-active {
    border-color: var(--accent) !important;
  }

  .lw-theme-preview {
    width: 80px;
    height: 52px;
    border-radius: 5px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .lw-preview-light { background: #f5f5f5; }
  .lw-preview-dark  { background: #1a1a1a; }

  .lw-preview-bar {
    height: 10px;
    background: #e0e0e0;
    flex-shrink: 0;
  }

  .lw-preview-bar-dark { background: #2a2a2a; }

  .lw-preview-content {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 6px;
  }

  .lw-preview-line {
    height: 6px;
    border-radius: 3px;
    background: #d0d0d0;
  }

  .lw-preview-line-dark { background: #3a3a3a; }

  .lw-theme-label {
    font-size: 11.5px;
    color: var(--text-secondary);
    font-weight: 500;
  }

  /* ── Accent ────────────────────────────────────────────────────────────── */
  .lw-accent-preview-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .lw-accent-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .lw-accent-hex {
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .lw-slider-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .lw-slider-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .lw-slider-label {
    font-size: 12px;
    color: var(--text-secondary);
    width: 72px;
    flex-shrink: 0;
  }

  .lw-slider {
    flex: 1;
    accent-color: var(--accent);
    cursor: pointer;
  }

  .lw-slider-val {
    font-size: 11.5px;
    color: var(--text-muted);
    width: 36px;
    text-align: right;
    font-variant-numeric: tabular-nums;
  }

  /* ── Wallpaper ─────────────────────────────────────────────────────────── */
  .lw-wallpaper-grid {
    display: flex;
    gap: 10px;
    flex-wrap: wrap;
  }

  .lw-wp-btn {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
    background: none;
    border: 2px solid var(--border);
    border-radius: 7px;
    padding: 6px;
    cursor: pointer;
    transition: border-color 120ms;
  }

  .lw-wp-btn:hover { border-color: color-mix(in srgb, var(--border) 40%, var(--text-muted)); }
  .lw-wp-active    { border-color: var(--accent) !important; }

  .lw-wp-swatch {
    width: 52px;
    height: 36px;
    border-radius: 4px;
  }

  .lw-wp-label {
    font-size: 10.5px;
    color: var(--text-muted);
  }

  /* ── Field rows ────────────────────────────────────────────────────────── */
  .lw-field-rows {
    display: flex;
    flex-direction: column;
    gap: 1px;
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }

  .lw-field-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
    border-bottom: 1px solid var(--border);
  }

  .lw-field-row:last-child { border-bottom: none; }

  .lw-field-label {
    font-size: 12.5px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .lw-field-hint {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 2px;
  }

  /* ── Segmented ─────────────────────────────────────────────────────────── */
  .lw-seg {
    display: flex;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }

  .lw-seg-btn {
    padding: 4px 10px;
    font-size: 11.5px;
    border: none;
    background: var(--surface);
    color: var(--text-secondary);
    cursor: pointer;
    border-right: 1px solid var(--border);
    transition: background 120ms, color 120ms;
  }

  .lw-seg-btn:last-child { border-right: none; }

  .lw-seg-btn:hover {
    background: color-mix(in srgb, var(--surface) 90%, var(--text-primary));
    color: var(--text-primary);
  }

  .lw-seg-on {
    background: var(--accent) !important;
    color: #fff !important;
    font-weight: 500;
  }

  /* ── Apps panel ────────────────────────────────────────────────────────── */
  .lw-apps-layout {
    display: flex;
    gap: 16px;
    flex: 1;
  }

  .lw-app-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 200px;
    flex-shrink: 0;
  }

  .lw-app-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    border-radius: 7px;
    border: none;
    background: none;
    cursor: pointer;
    text-align: left;
    transition: background 120ms;
  }

  .lw-app-row:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
  }

  .lw-app-row-active {
    background: color-mix(in srgb, var(--accent) 12%, transparent) !important;
  }

  .lw-app-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .lw-app-meta {
    display: flex;
    flex-direction: column;
    gap: 1px;
    flex: 1;
    min-width: 0;
  }

  .lw-app-name {
    font-size: 12.5px;
    font-weight: 500;
    color: var(--text-primary);
  }

  .lw-app-role {
    font-size: 11px;
    color: var(--text-muted);
  }

  .lw-status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .lw-status-running { background: #22c55e; }
  .lw-status-idle    { background: var(--border); }

  /* ── App detail ────────────────────────────────────────────────────────── */
  .lw-app-detail {
    flex: 1;
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 20px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }

  .lw-app-detail-icon {
    font-size: 28px;
    color: var(--accent);
    margin-bottom: 6px;
  }

  .lw-app-detail-name {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .lw-app-detail-role {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 16px;
  }

  .lw-app-detail-rows {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 1px;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    margin-bottom: 16px;
  }

  .lw-detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 7px 12px;
    background: var(--surface);
    border-bottom: 1px solid var(--border);
    font-size: 12px;
  }

  .lw-detail-row:last-child { border-bottom: none; }

  .lw-detail-key { color: var(--text-muted); }
  .lw-detail-val { color: var(--text-primary); font-weight: 500; }

  .lw-detail-running { color: #22c55e !important; }
  .lw-detail-idle    { color: var(--text-muted) !important; }

  .lw-detail-mono {
    font-family: 'Geist Mono', monospace;
    font-size: 11px !important;
  }

  .lw-app-actions {
    display: flex;
    gap: 8px;
    margin-top: auto;
  }

  .lw-btn-secondary {
    padding: 5px 12px;
    font-size: 12px;
    border: 1px solid var(--border);
    border-radius: 5px;
    background: var(--surface);
    color: var(--text-primary);
    cursor: pointer;
    transition: background 120ms;
  }

  .lw-btn-secondary:hover {
    background: color-mix(in srgb, var(--surface) 90%, var(--text-primary));
  }

  .lw-btn-danger {
    padding: 5px 12px;
    font-size: 12px;
    border: 1px solid color-mix(in srgb, #ef4444 40%, transparent);
    border-radius: 5px;
    background: color-mix(in srgb, #ef4444 8%, transparent);
    color: #ef4444;
    cursor: pointer;
    transition: background 120ms;
  }

  .lw-btn-danger:hover {
    background: color-mix(in srgb, #ef4444 16%, transparent);
  }

  /* ── Placeholder ───────────────────────────────────────────────────────── */
  .lw-placeholder {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    opacity: 0.35;
  }

  .lw-placeholder-icon  { font-size: 32px; color: var(--text-muted); }
  .lw-placeholder-label { font-size: 13px; color: var(--text-muted); }
</style>
