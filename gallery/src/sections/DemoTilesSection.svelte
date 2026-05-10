<script>
  let brightness = $state(78);
  let contrast   = $state(62);
  let profile    = $state('p3');
  let applied    = $state(false);

  function applySettings() {
    applied = true;
    setTimeout(() => applied = false, 1800);
  }
</script>

<div class="section">
  <div class="demo-tiles">

    <!-- Tile 1: article / release note -->
    <div class="demo-tile">
      <div class="demo-eyebrow">
        <span class="demo-tag">What's new</span>
        <span class="demo-version">v2.4.0</span>
      </div>
      <h2 class="demo-h2">Geist is now the system typeface</h2>
      <p class="demo-sub">Typography, spacing, and motion — unified across all Libre apps.</p>
      <p class="demo-body">
        All apps now share a single token layer for color, type, and spacing. The 4px base unit
        governs every margin and gap. Motion is functional only — 120ms fade for overlays,
        200ms fly for panels. Nothing moves unless it helps the user understand a transition.
      </p>
      <div class="demo-footer">
        <span class="demo-link">Read the changelog →</span>
        <span class="demo-muted">2 minutes ago</span>
      </div>
    </div>

    <!-- Tile 2: data / status panel -->
    <div class="demo-tile">
      <div class="demo-section-label">System Status</div>
      <h3 class="demo-h3">Everything is running normally</h3>
      <p class="demo-caption">Last checked 2 minutes ago · Auto-refreshes every 60s</p>

      <div class="demo-rows">
        <div class="demo-row">
          <span class="demo-row-key">Storage</span>
          <span class="demo-row-bar"><span class="demo-row-fill" style="width:19%"></span></span>
          <span class="demo-row-val">48.2 <span class="demo-row-unit">GB / 256</span></span>
        </div>
        <div class="demo-row">
          <span class="demo-row-key">Memory</span>
          <span class="demo-row-bar"><span class="demo-row-fill" style="width:39%"></span></span>
          <span class="demo-row-val">3.1 <span class="demo-row-unit">GB used</span></span>
        </div>
        <div class="demo-row">
          <span class="demo-row-key">CPU</span>
          <span class="demo-row-bar"><span class="demo-row-fill" style="width:8%"></span></span>
          <span class="demo-row-val demo-row-accent">0.8<span class="demo-row-unit">%</span></span>
        </div>
        <div class="demo-row">
          <span class="demo-row-key">Uptime</span>
          <span class="demo-row-bar"></span>
          <span class="demo-row-val demo-row-accent">14d 6h <span class="demo-row-unit">23m</span></span>
        </div>
      </div>

      <p class="demo-note">
        <span class="demo-dot-ok"></span>
        All services nominal — <span style="color:var(--text-secondary)">no alerts in the last 30 days</span>
      </p>
    </div>

    <!-- Tile 3: controls / settings -->
    <div class="demo-tile">
      <div class="demo-section-label">Display Settings</div>
      <h3 class="demo-h3">Adjust your workspace</h3>
      <p class="demo-caption">Changes apply to all Libre apps on this device.</p>

      <div class="ctrl-fields">
        <div class="ctrl-field">
          <div class="ctrl-label-row">
            <span class="ctrl-label">Brightness</span>
            <span class="ctrl-val">{brightness}%</span>
          </div>
          <input type="range" class="ctrl-slider" min="0" max="100" bind:value={brightness} />
        </div>
        <div class="ctrl-field">
          <div class="ctrl-label-row">
            <span class="ctrl-label">Contrast</span>
            <span class="ctrl-val">{contrast}%</span>
          </div>
          <input type="range" class="ctrl-slider" min="0" max="100" bind:value={contrast} />
        </div>
        <div class="ctrl-field">
          <div class="ctrl-label-row">
            <span class="ctrl-label">Color Profile</span>
          </div>
          <select class="ctrl-select" bind:value={profile}>
            <option value="srgb">sRGB — Standard</option>
            <option value="p3">Display P3 — Wide gamut</option>
            <option value="adobe">Adobe RGB — Print</option>
          </select>
        </div>
      </div>

      <div class="ctrl-actions">
        <button class="ctrl-ghost" onclick={() => { brightness = 78; contrast = 62; profile = 'p3'; }}>Reset</button>
        <button class="ctrl-primary" class:ctrl-applied={applied} onclick={applySettings}>
          {applied ? '✓ Applied' : 'Apply Changes'}
        </button>
      </div>
    </div>

    <!-- Tile 4: activity feed -->
    <div class="demo-tile">
      <div class="feed-header">
        <div class="demo-section-label">Recent Activity</div>
        <span class="feed-badge">4 new</span>
      </div>

      <div class="feed-list">
        <div class="feed-item">
          <div class="feed-avatar feed-av-blue">E</div>
          <div class="feed-body">
            <div class="feed-line">
              <span class="feed-name">Elliott</span>
              <span class="feed-action">merged</span>
              <span class="feed-ref">feat/token-refactor</span>
            </div>
            <span class="feed-time">3 minutes ago</span>
          </div>
          <span class="feed-dot"></span>
        </div>

        <div class="feed-item">
          <div class="feed-avatar feed-av-green">C</div>
          <div class="feed-body">
            <div class="feed-line">
              <span class="feed-name">Claude</span>
              <span class="feed-action">updated</span>
              <span class="feed-ref">tokens.css</span>
            </div>
            <div class="feed-sub">Removed --ir-* vars, switched to global tokens</div>
            <span class="feed-time">12 minutes ago</span>
          </div>
          <span class="feed-dot"></span>
        </div>

        <div class="feed-item">
          <div class="feed-avatar feed-av-blue">E</div>
          <div class="feed-body">
            <div class="feed-line">
              <span class="feed-name">Elliott</span>
              <span class="feed-action">opened</span>
              <span class="feed-ref">PR #48</span>
            </div>
            <div class="feed-sub">Typography section — demo tiles + full hierarchy</div>
            <span class="feed-time">1 hour ago</span>
          </div>
          <span class="feed-dot"></span>
        </div>

        <div class="feed-item feed-item-read">
          <div class="feed-avatar feed-av-gray">L</div>
          <div class="feed-body">
            <div class="feed-line">
              <span class="feed-name">Lint</span>
              <span class="feed-action">passed</span>
              <span class="feed-ref">ci/stylelint</span>
            </div>
            <span class="feed-time">2 hours ago</span>
          </div>
        </div>
      </div>

      <button class="feed-more">View all activity →</button>
    </div>

  </div>
</div>

<style>
  .section { max-width: 860px; }

  .demo-tiles {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .demo-tile {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 20px 22px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  /* Shared tile primitives */
  .demo-section-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .demo-h2 {
    font-size: 18px;
    font-weight: 600;
    letter-spacing: -0.01em;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.25;
  }

  .demo-h3 {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
    line-height: 1.3;
  }

  .demo-sub {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    margin: 0;
    line-height: 1.4;
  }

  .demo-body {
    font-size: 13px;
    color: var(--text-primary);
    line-height: 1.55;
    margin: 0;
  }

  .demo-caption {
    font-size: 11px;
    color: var(--text-muted);
    margin: 0;
  }

  .demo-muted { font-size: 11px; color: var(--text-muted); }

  /* Tile 1 */
  .demo-eyebrow {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .demo-tag {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent) 30%, transparent);
    border-radius: 4px;
    padding: 2px 6px;
  }

  .demo-version {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .demo-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 2px;
    padding-top: 10px;
    border-top: 1px solid var(--border-subtle);
  }

  .demo-link {
    font-size: 12px;
    font-weight: 500;
    color: var(--accent);
    cursor: default;
  }

  /* Tile 2 */
  .demo-rows {
    display: flex;
    flex-direction: column;
    gap: 7px;
    padding: 10px 0;
    border-top: 1px solid var(--border-subtle);
    border-bottom: 1px solid var(--border-subtle);
  }

  .demo-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .demo-row-key {
    font-size: 11px;
    color: var(--text-secondary);
    width: 52px;
    flex-shrink: 0;
  }

  .demo-row-bar {
    flex: 1;
    height: 4px;
    background: var(--border);
    border-radius: 2px;
    overflow: hidden;
    position: relative;
  }

  .demo-row-fill {
    position: absolute;
    inset: 0 auto 0 0;
    background: var(--accent);
    border-radius: 2px;
    opacity: 0.7;
  }

  .demo-row-val {
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-primary);
    width: 72px;
    text-align: right;
    flex-shrink: 0;
  }

  .demo-row-unit { color: var(--text-muted); font-size: 10px; }
  .demo-row-accent { color: var(--accent); }

  .demo-note {
    font-size: 11px;
    color: var(--text-muted);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .demo-dot-ok {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #3a3;
    flex-shrink: 0;
  }

  /* Tile 3 */
  .ctrl-fields {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px 0;
    border-top: 1px solid var(--border-subtle);
    border-bottom: 1px solid var(--border-subtle);
  }

  .ctrl-field { display: flex; flex-direction: column; gap: 5px; }

  .ctrl-label-row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .ctrl-label { font-size: 11px; color: var(--text-secondary); }

  .ctrl-val {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--accent);
  }

  .ctrl-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 5px;
    border-radius: 3px;
    background: var(--border);
    cursor: pointer;
    outline: none;
    accent-color: var(--accent);
  }

  .ctrl-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 13px;
    height: 13px;
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid var(--surface);
    box-shadow: 0 0 0 1px var(--accent);
    cursor: pointer;
  }

  .ctrl-select {
    width: 100%;
    font-size: 11px;
    font-family: inherit;
    color: var(--text-primary);
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 5px 8px;
    cursor: pointer;
    outline: none;
  }

  .ctrl-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: 2px;
  }

  .ctrl-ghost {
    padding: 5px 12px;
    font-size: 12px;
    font-family: inherit;
    background: none;
    border: 1px solid var(--border);
    border-radius: 5px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: color 0.1s, border-color 0.1s;
  }
  .ctrl-ghost:hover { color: var(--text-primary); border-color: var(--text-muted); }

  .ctrl-primary {
    padding: 5px 14px;
    font-size: 12px;
    font-weight: 500;
    font-family: inherit;
    background: var(--accent);
    border: 1px solid var(--accent);
    border-radius: 5px;
    color: #fff;
    cursor: pointer;
    transition: opacity 0.1s, background 0.15s, border-color 0.15s;
  }
  .ctrl-primary:hover { opacity: 0.88; }
  .ctrl-applied { background: #2a7a2a; border-color: #2a7a2a; }

  /* Tile 4 */
  .feed-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .feed-badge {
    font-size: 9px;
    font-weight: 700;
    color: #fff;
    background: var(--accent);
    border-radius: 10px;
    padding: 1px 7px;
  }

  .feed-list {
    display: flex;
    flex-direction: column;
    border-top: 1px solid var(--border-subtle);
    border-bottom: 1px solid var(--border-subtle);
  }

  .feed-item {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 9px 0;
    border-bottom: 1px solid var(--border-subtle);
  }
  .feed-item:last-child { border-bottom: none; }
  .feed-item-read { opacity: 0.55; }

  .feed-avatar {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    font-weight: 700;
    flex-shrink: 0;
    color: #fff;
  }
  .feed-av-blue  { background: var(--accent); }
  .feed-av-green { background: #2a7a4a; }
  .feed-av-gray  { background: var(--text-muted); }

  .feed-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .feed-line {
    display: flex;
    align-items: baseline;
    gap: 4px;
    flex-wrap: wrap;
  }

  .feed-name   { font-size: 12px; font-weight: 600; color: var(--text-primary); }
  .feed-action { font-size: 12px; color: var(--text-muted); }

  .feed-ref {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 10%, transparent);
    border-radius: 3px;
    padding: 0 4px;
  }

  .feed-sub {
    font-size: 11px;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .feed-time { font-size: 10px; color: var(--text-muted); }

  .feed-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
    margin-top: 5px;
  }

  .feed-more {
    background: none;
    border: none;
    font-size: 11px;
    font-weight: 500;
    color: var(--accent);
    cursor: pointer;
    padding: 6px 0 0;
    font-family: inherit;
    text-align: left;
  }
  .feed-more:hover { opacity: 0.8; }
</style>
