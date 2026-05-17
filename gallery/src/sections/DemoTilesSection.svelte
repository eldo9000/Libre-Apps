<script>
  let brightness = $state(78);
  let contrast   = $state(62);
  let profile    = $state('p3');
  let applied    = $state(false);

  function applySettings() {
    applied = true;
    setTimeout(() => applied = false, 1800);
  }

  const bars = [34, 58, 45, 72, 89, 63, 91, 78, 105, 88, 116, 94];
  const barMax = 116;
  const barMonths = ['Jun','Jul','Aug','Sep','Oct','Nov','Dec','Jan','Feb','Mar','Apr','May'];
</script>

<div class="section">

  <!-- ── Original 2-col tiles ─────────────────────────────────────────── -->
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

  <!-- ── Horizontal layouts ────────────────────────────────────────────── -->
  <div class="tiles-group-label">Horizontal Layouts</div>
  <div class="demo-tiles-wide">

    <!-- Banner: app header strip -->
    <div class="demo-tile bnn-tile">
      <div class="bnn-icon">◈</div>
      <div class="bnn-info">
        <div class="bnn-name">Shelf</div>
        <div class="bnn-sub">File manager · Running</div>
      </div>
      <div class="bnn-stats">
        <div class="bnn-stat">
          <span class="bnn-val">2,847</span>
          <span class="bnn-key">Files indexed</span>
        </div>
        <div class="bnn-divider"></div>
        <div class="bnn-stat">
          <span class="bnn-val">5</span>
          <span class="bnn-key">Locations</span>
        </div>
        <div class="bnn-divider"></div>
        <div class="bnn-stat">
          <span class="bnn-val">Just now</span>
          <span class="bnn-key">Last sync</span>
        </div>
        <div class="bnn-divider"></div>
        <div class="bnn-stat">
          <span class="bnn-val bnn-green">Healthy</span>
          <span class="bnn-key">Status</span>
        </div>
      </div>
      <div class="bnn-actions">
        <button class="ctrl-ghost">Settings</button>
        <button class="ctrl-primary">Open Shelf →</button>
      </div>
    </div>

    <!-- Installer window -->
    <div class="demo-tile inst-tile">
      <div class="inst-sidebar">
        <div class="inst-sidebar-label">LibreWin OS 1.0</div>
        <div class="inst-step inst-done">
          <span class="inst-num">✓</span>
          <span class="inst-label">License</span>
        </div>
        <div class="inst-step inst-done">
          <span class="inst-num">✓</span>
          <span class="inst-label">Destination</span>
        </div>
        <div class="inst-step inst-active">
          <span class="inst-num">3</span>
          <span class="inst-label">Installation</span>
        </div>
        <div class="inst-step inst-pending">
          <span class="inst-num">4</span>
          <span class="inst-label">Summary</span>
        </div>
        <div class="inst-sidebar-spacer"></div>
        <button class="inst-cancel">Cancel</button>
      </div>
      <div class="inst-main">
        <div class="inst-main-title">Installing LibreWin OS</div>
        <div class="inst-main-sub">Copying system files and core applications to your volume</div>
        <div class="inst-progress-wrap">
          <div class="inst-progress-track">
            <div class="inst-progress-fill" style="width: 65%"></div>
          </div>
          <div class="inst-progress-meta">
            <span class="inst-pct">65%</span>
            <span class="inst-size">847 MB of 1.3 GB</span>
          </div>
        </div>
        <div class="inst-log">
          <div class="inst-log-line"><span class="inst-dot-ok">●</span> librewin-common 2.4.1 — done</div>
          <div class="inst-log-line"><span class="inst-dot-ok">●</span> shelf 1.2.0 — done</div>
          <div class="inst-log-line inst-log-active"><span class="inst-dot-spin">◌</span> prism 1.1.0 — installing…</div>
          <div class="inst-log-line inst-log-dim"><span class="inst-dot-pend">○</span> stack 1.3.0</div>
          <div class="inst-log-line inst-log-dim"><span class="inst-dot-pend">○</span> fade 0.9.0</div>
        </div>
        <div class="inst-eta">Estimated time remaining: <strong>42 seconds</strong></div>
      </div>
    </div>

  </div>

  <!-- ── Column layouts (narrow, vertical) ────────────────────────────── -->
  <div class="tiles-group-label">Column Layouts</div>
  <div class="demo-tiles-3col">

    <!-- Column 1: File browser -->
    <div class="demo-tile col-tile">
      <div class="demo-section-label">File Browser</div>
      <div class="tree-path">~/Downloads/Github</div>
      <div class="tree-list">
        <div class="tree-item tree-folder tree-open">
          <span class="tree-caret">▾</span>
          <span class="tree-name">Libre-Apps</span>
          <span class="tree-count">12</span>
        </div>
        <div class="tree-item tree-sub tree-folder tree-open">
          <span class="tree-caret">▾</span>
          <span class="tree-name">common-js</span>
        </div>
        <div class="tree-item tree-sub2 tree-file">
          <span class="tree-caret tree-no-caret"></span>
          <span class="tree-name">Button.svelte</span>
          <span class="tree-size">4.2 KB</span>
        </div>
        <div class="tree-item tree-sub2 tree-file">
          <span class="tree-caret tree-no-caret"></span>
          <span class="tree-name">Dialog.svelte</span>
          <span class="tree-size">8.1 KB</span>
        </div>
        <div class="tree-item tree-sub2 tree-file tree-selected">
          <span class="tree-caret tree-no-caret"></span>
          <span class="tree-name">tokens.css</span>
          <span class="tree-size">3.6 KB</span>
        </div>
        <div class="tree-item tree-sub tree-folder">
          <span class="tree-caret">▸</span>
          <span class="tree-name">gallery</span>
        </div>
        <div class="tree-item tree-sub tree-folder">
          <span class="tree-caret">▸</span>
          <span class="tree-name">docs</span>
        </div>
        <div class="tree-item tree-folder">
          <span class="tree-caret">▸</span>
          <span class="tree-name">Shelf-App</span>
          <span class="tree-count">8</span>
        </div>
        <div class="tree-item tree-folder">
          <span class="tree-caret">▸</span>
          <span class="tree-name">Stack-App</span>
          <span class="tree-count">7</span>
        </div>
        <div class="tree-item tree-folder">
          <span class="tree-caret">▸</span>
          <span class="tree-name">Prism-App</span>
          <span class="tree-count">9</span>
        </div>
      </div>
      <div class="tree-footer">
        <span class="tree-footer-info">10 items · 3 selected</span>
        <span class="tree-footer-size">1.2 GB</span>
      </div>
    </div>

    <!-- Column 2: Inbox / notifications -->
    <div class="demo-tile col-tile">
      <div class="inbox-header">
        <div class="demo-section-label">Inbox</div>
        <span class="inbox-unread">5 unread</span>
      </div>
      <div class="inbox-groups">
        <div class="inbox-group">
          <div class="inbox-group-head">
            <span class="inbox-app-name">Shelf</span>
            <span class="inbox-badge">2</span>
          </div>
          <div class="inbox-item inbox-item-unread">
            <span class="inbox-dot"></span>
            <div class="inbox-body">
              <div class="inbox-msg">Index complete — 2,847 files found</div>
              <div class="inbox-time">2 min ago</div>
            </div>
          </div>
          <div class="inbox-item inbox-item-unread">
            <span class="inbox-dot"></span>
            <div class="inbox-body">
              <div class="inbox-msg">Low storage warning · 48 GB used</div>
              <div class="inbox-time">1 hr ago</div>
            </div>
          </div>
        </div>
        <div class="inbox-group">
          <div class="inbox-group-head">
            <span class="inbox-app-name">Stack</span>
            <span class="inbox-badge">1</span>
          </div>
          <div class="inbox-item inbox-item-unread">
            <span class="inbox-dot"></span>
            <div class="inbox-body">
              <div class="inbox-msg">Auto-saved "M3 roadmap.md"</div>
              <div class="inbox-time">14 min ago</div>
            </div>
          </div>
        </div>
        <div class="inbox-group">
          <div class="inbox-group-head">
            <span class="inbox-app-name">Prism</span>
            <span class="inbox-badge">2</span>
          </div>
          <div class="inbox-item inbox-item-unread">
            <span class="inbox-dot"></span>
            <div class="inbox-body">
              <div class="inbox-msg">14 new items in watchlist</div>
              <div class="inbox-time">3 hr ago</div>
            </div>
          </div>
          <div class="inbox-item inbox-item-read">
            <span class="inbox-dot inbox-dot-read"></span>
            <div class="inbox-body">
              <div class="inbox-msg">Conversion complete · 6 files</div>
              <div class="inbox-time">5 hr ago</div>
            </div>
          </div>
        </div>
      </div>
      <button class="inbox-clear">Mark all read</button>
    </div>

    <!-- Column 3: Task list -->
    <div class="demo-tile col-tile">
      <div class="task-header">
        <div class="demo-section-label">Tasks</div>
        <span class="task-count">5 remaining</span>
      </div>
      <div class="task-list">
        <div class="task-item">
          <div class="task-check"></div>
          <div class="task-content">
            <div class="task-title">Token audit — remove --ir-* vars</div>
            <div class="task-meta">
              <span class="task-pri task-high">HIGH</span>
              <span class="task-due task-due-today">Due today</span>
            </div>
          </div>
        </div>
        <div class="task-item">
          <div class="task-check"></div>
          <div class="task-content">
            <div class="task-title">Sync Shelf-App tokens snapshot</div>
            <div class="task-meta">
              <span class="task-pri task-med">MED</span>
              <span class="task-due">Due Mon</span>
            </div>
          </div>
        </div>
        <div class="task-item task-item-done">
          <div class="task-check task-check-done">✓</div>
          <div class="task-content">
            <div class="task-title task-title-done">Gallery horizontal banner section</div>
            <div class="task-meta">
              <span class="task-pri task-med">MED</span>
              <span class="task-due">Done</span>
            </div>
          </div>
        </div>
        <div class="task-item">
          <div class="task-check"></div>
          <div class="task-content">
            <div class="task-title">Update docs/ROADMAP.md — M3</div>
            <div class="task-meta">
              <span class="task-pri task-low">LOW</span>
              <span class="task-due">Due Wed</span>
            </div>
          </div>
        </div>
        <div class="task-item">
          <div class="task-check"></div>
          <div class="task-content">
            <div class="task-title">Write M3 milestone doc for Prism</div>
            <div class="task-meta">
              <span class="task-pri task-low">LOW</span>
              <span class="task-due">No date</span>
            </div>
          </div>
        </div>
        <div class="task-item">
          <div class="task-check"></div>
          <div class="task-content">
            <div class="task-title">Review signing spec — Fade release</div>
            <div class="task-meta">
              <span class="task-pri task-high">HIGH</span>
              <span class="task-due task-due-today">Due today</span>
            </div>
          </div>
        </div>
      </div>
      <button class="task-add">+ Add task</button>
    </div>

  </div>

  <!-- ── Full-page layouts ─────────────────────────────────────────────── -->
  <div class="tiles-group-label">Full Page Layouts</div>
  <div class="demo-tiles-fullpage">

    <!-- Full page 1: Article / editorial -->
    <div class="demo-tile art-tile">
      <div class="art-header">
        <div class="art-eyebrow">
          <span class="demo-tag">Design Systems</span>
          <span class="art-read-time">4 min read</span>
        </div>
        <h1 class="art-title">Designing with constraints: how the 4px grid shipped across five desktop apps</h1>
        <div class="art-meta">
          <div class="art-author-row">
            <div class="art-avatar">E</div>
            <div class="art-author-info">
              <span class="art-author-name">Elliott Donofrio</span>
              <span class="art-date">April 2026 · Libre Engineering Blog</span>
            </div>
          </div>
        </div>
      </div>

      <p class="art-lead">
        When you're building a family of apps that share a foundation, constraints become
        features. The 4px base unit wasn't an arbitrary choice — it emerged from the recognition
        that consistent spacing makes visual rhythm predictable without requiring designers to
        think about every gap.
      </p>

      <p class="art-body">
        The Libre family — Shelf, Stack, Prism, Fade, and Avalanche — each started as independent
        tools. When we decided to unify them under a shared token layer, the first challenge
        wasn't the components. It was the grid. Every app had its own rhythm, its own instinct
        for padding. Some used 8px base units. Some mixed 4px and 6px in the same panel. A few
        used neither — spacing was whatever felt right at the time.
      </p>

      <p class="art-body">
        The migration took two milestones. The first unified colors and typography — the visible,
        obvious layer. The second was spacing. We introduced a lint rule that flags any value
        not divisible by 4. Three weeks in, it surfaced 47 violations across the five codebases.
        That number dropping to zero was the moment the system felt real.
      </p>

      <blockquote class="art-quote">
        Consistency isn't about making everything look the same. It's about making everything
        feel like it came from the same mind.
      </blockquote>

      <h4 class="art-h4">Motion as last resort</h4>

      <p class="art-body">
        Once the grid was settled, motion became the next target. The rule was blunt: nothing
        moves unless it helps the user understand a transition. 120ms for overlays appearing.
        200ms fly for panels sliding in. That's the whole spec. Anything more complex gets cut.
        There were debates about cubic-bezier curves, about spring physics, about matching
        macOS system animations. All of it was rejected. Complexity in motion is debt.
      </p>

      <p class="art-body">
        The hardest part of shipping a constraint isn't defining it — it's defending it six
        months later when someone wants an exception. The lint rule helped. The shared
        <code class="art-code">tokens.css</code> file helped more. When a constraint lives in
        one place and downstream repos vendor it, the cost of violating it is explicit.
        You have to consciously override it. Most of the time, you don't.
      </p>

      <h4 class="art-h4">What's next</h4>

      <p class="art-body">
        The next shared constraint is elevation — how surfaces stack, how shadows communicate
        depth without skeuomorphism. We're prototyping it in the gallery now. If it passes
        the same test the grid passed — if removing it feels worse than keeping it — it ships.
      </p>

      <div class="art-footer">
        <div class="art-tags">
          <span class="art-tag">design-systems</span>
          <span class="art-tag">tokens</span>
          <span class="art-tag">desktop-apps</span>
          <span class="art-tag">tauri</span>
          <span class="art-tag">svelte-5</span>
        </div>
        <div class="art-footer-actions">
          <span class="demo-link">Share this post →</span>
        </div>
      </div>
    </div>

    <!-- Full page 2: Dashboard / graphical -->
    <div class="demo-tile dash-tile">
      <div class="dash-header">
        <div class="dash-header-left">
          <div class="dash-title">Release Dashboard</div>
          <div class="dash-subtitle">May 2026 · All apps · All platforms</div>
        </div>
        <div class="dash-header-right">
          <select class="ctrl-select dash-select">
            <option>Last 12 months</option>
            <option>Last 6 months</option>
            <option>This year</option>
          </select>
        </div>
      </div>

      <!-- Metric cards -->
      <div class="dash-metrics">
        <div class="dash-metric">
          <div class="dash-metric-val">14,847</div>
          <div class="dash-metric-label">Total Downloads</div>
          <div class="dash-metric-change dash-up">↑ 12% this month</div>
        </div>
        <div class="dash-metric">
          <div class="dash-metric-val">3,291</div>
          <div class="dash-metric-label">Active Installs</div>
          <div class="dash-metric-change dash-up">↑ 5% vs last month</div>
        </div>
        <div class="dash-metric">
          <div class="dash-metric-val">18m</div>
          <div class="dash-metric-label">Avg. Session</div>
          <div class="dash-metric-change dash-up">↑ 2m vs last month</div>
        </div>
        <div class="dash-metric">
          <div class="dash-metric-val">7</div>
          <div class="dash-metric-label">Open Issues</div>
          <div class="dash-metric-change dash-down">↓ 3 since last week</div>
        </div>
      </div>

      <!-- Charts row -->
      <div class="dash-charts-row">
        <!-- Bar chart -->
        <div class="dash-chart-panel">
          <div class="dash-chart-title">Monthly Downloads</div>
          <div class="dash-barchart">
            <div class="dash-bars">
              {#each bars as val, i}
                <div class="dash-bar">
                  <div class="dash-bar-fill" style="height: {Math.round((val / barMax) * 96)}px"></div>
                  <div class="dash-bar-label">{barMonths[i]}</div>
                </div>
              {/each}
            </div>
          </div>
        </div>

        <!-- Donut chart -->
        <div class="dash-chart-panel">
          <div class="dash-chart-title">Downloads by App</div>
          <div class="dash-donut-wrap">
            <div class="dash-donut"></div>
            <div class="dash-donut-legend">
              <div class="dash-legend-item"><span class="dli-dot dli-accent"></span><span class="dli-label">Shelf</span><span class="dli-val">42%</span></div>
              <div class="dash-legend-item"><span class="dli-dot dli-blue2"></span><span class="dli-label">Stack</span><span class="dli-val">26%</span></div>
              <div class="dash-legend-item"><span class="dli-dot dli-green"></span><span class="dli-label">Prism</span><span class="dli-val">18%</span></div>
              <div class="dash-legend-item"><span class="dli-dot dli-orange"></span><span class="dli-label">Fade</span><span class="dli-val">9%</span></div>
              <div class="dash-legend-item"><span class="dli-dot dli-gray"></span><span class="dli-label">Others</span><span class="dli-val">5%</span></div>
            </div>
          </div>
        </div>
      </div>

      <!-- App breakdown table -->
      <div class="dash-table-wrap">
        <div class="dash-table-title">App Breakdown</div>
        <div class="dash-table">
          <div class="dash-table-head">
            <span class="dth dth-app">App</span>
            <span class="dth dth-ver">Version</span>
            <span class="dth dth-dl">Downloads</span>
            <span class="dth dth-installs">Installs</span>
            <span class="dth dth-release">Last Release</span>
            <span class="dth dth-status">Build</span>
          </div>
          <div class="dash-table-row">
            <span class="dtd dth-app"><span class="dash-app-dot" style="background:var(--accent)"></span>Shelf</span>
            <span class="dtd dth-ver">1.2.0</span>
            <span class="dtd dth-dl">6,235</span>
            <span class="dtd dth-installs">1,382</span>
            <span class="dtd dth-release">May 8</span>
            <span class="dtd dth-status"><span class="dash-build dash-build-pass">passing</span></span>
          </div>
          <div class="dash-table-row">
            <span class="dtd dth-app"><span class="dash-app-dot" style="background:#5b8dee"></span>Stack</span>
            <span class="dtd dth-ver">1.3.0</span>
            <span class="dtd dth-dl">3,861</span>
            <span class="dtd dth-installs">854</span>
            <span class="dtd dth-release">May 12</span>
            <span class="dtd dth-status"><span class="dash-build dash-build-pass">passing</span></span>
          </div>
          <div class="dash-table-row">
            <span class="dtd dth-app"><span class="dash-app-dot" style="background:#3aaa6a"></span>Prism</span>
            <span class="dtd dth-ver">1.1.0</span>
            <span class="dtd dth-dl">2,671</span>
            <span class="dtd dth-installs">591</span>
            <span class="dtd dth-release">Apr 29</span>
            <span class="dtd dth-status"><span class="dash-build dash-build-pass">passing</span></span>
          </div>
          <div class="dash-table-row">
            <span class="dtd dth-app"><span class="dash-app-dot" style="background:#e08040"></span>Fade</span>
            <span class="dtd dth-ver">0.9.0</span>
            <span class="dtd dth-dl">1,337</span>
            <span class="dtd dth-installs">326</span>
            <span class="dtd dth-release">Apr 18</span>
            <span class="dtd dth-status"><span class="dash-build dash-build-warn">warning</span></span>
          </div>
          <div class="dash-table-row">
            <span class="dtd dth-app"><span class="dash-app-dot" style="background:var(--text-muted)"></span>Avalanche</span>
            <span class="dtd dth-ver">0.4.1</span>
            <span class="dtd dth-dl">743</span>
            <span class="dtd dth-installs">138</span>
            <span class="dtd dth-release">Mar 30</span>
            <span class="dtd dth-status"><span class="dash-build dash-build-fail">failing</span></span>
          </div>
        </div>
      </div>

    </div>

  </div>
</div>

<style>
  .section { max-width: 1075px; }

  /* ── Group labels ── */
  .tiles-group-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 28px 0 10px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  /* ── Grid containers ── */
  .demo-tiles {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .demo-tiles-wide {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .demo-tiles-3col {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    gap: 16px;
  }

  .demo-tiles-fullpage {
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  /* ── Base tile ── */
  .demo-tile {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 20px 22px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  /* ── Shared primitives ── */
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

  /* ── Banner tile ── */
  .bnn-tile {
    flex-direction: row;
    align-items: center;
    gap: 20px;
    padding: 18px 24px;
  }

  .bnn-icon {
    width: 44px;
    height: 44px;
    border-radius: 10px;
    background: var(--accent);
    color: #fff;
    font-size: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .bnn-info { flex-shrink: 0; }
  .bnn-name { font-size: 15px; font-weight: 600; color: var(--text-primary); }
  .bnn-sub  { font-size: 11px; color: var(--text-muted); margin-top: 2px; }

  .bnn-stats {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0;
  }

  .bnn-stat {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 0 24px;
  }

  .bnn-val {
    font-size: 16px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    color: var(--text-primary);
    font-family: 'Geist Mono', monospace;
  }

  .bnn-key {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-top: 2px;
  }

  .bnn-green { color: #3aaa5a; }

  .bnn-divider {
    width: 1px;
    height: 28px;
    background: var(--border);
    flex-shrink: 0;
  }

  .bnn-actions { display: flex; gap: 8px; flex-shrink: 0; }

  /* ── Installer tile ── */
  .inst-tile {
    flex-direction: row;
    padding: 0;
    overflow: hidden;
    gap: 0;
  }

  .inst-sidebar {
    width: 160px;
    flex-shrink: 0;
    background: color-mix(in srgb, var(--surface-raised) 50%, var(--surface-panel));
    border-right: 1px solid var(--border);
    padding: 20px 14px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .inst-sidebar-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 10px;
  }

  .inst-step {
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 6px 8px;
    border-radius: 6px;
  }

  .inst-done  { opacity: 0.5; }
  .inst-active {
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    color: var(--accent);
  }
  .inst-pending { opacity: 0.4; }

  .inst-num {
    font-size: 12px;
    font-weight: 700;
    font-family: 'Geist Mono', monospace;
    width: 16px;
    text-align: center;
    flex-shrink: 0;
  }
  .inst-active .inst-num { color: var(--accent); }

  .inst-label { font-size: 12px; color: var(--text-primary); }
  .inst-active .inst-label { font-weight: 600; color: var(--accent); }

  .inst-sidebar-spacer { flex: 1; }

  .inst-cancel {
    background: none;
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 5px 10px;
    font-size: 11px;
    font-family: inherit;
    color: var(--text-muted);
    cursor: pointer;
    text-align: center;
  }

  .inst-main {
    flex: 1;
    padding: 24px 28px;
    display: flex;
    flex-direction: column;
    gap: 14px;
  }

  .inst-main-title {
    font-size: 17px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .inst-main-sub {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: -8px;
  }

  .inst-progress-wrap { display: flex; flex-direction: column; gap: 6px; }

  .inst-progress-track {
    height: 14px;
    background: var(--border);
    border-radius: 7px;
    overflow: hidden;
  }

  .inst-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, var(--accent), color-mix(in srgb, var(--accent) 80%, #fff));
    border-radius: 7px;
  }

  .inst-progress-meta {
    display: flex;
    justify-content: space-between;
  }

  .inst-pct {
    font-size: 13px;
    font-weight: 700;
    font-family: 'Geist Mono', monospace;
    color: var(--accent);
  }

  .inst-size { font-size: 11px; color: var(--text-muted); }

  .inst-log {
    display: flex;
    flex-direction: column;
    gap: 3px;
    background: var(--surface-panel);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    padding: 8px 12px;
  }

  .inst-log-line {
    display: flex;
    align-items: center;
    gap: 7px;
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-secondary);
  }

  .inst-log-active { color: var(--text-primary); }
  .inst-log-dim    { opacity: 0.45; }

  .inst-dot-ok   { color: #3aaa5a; font-size: 9px; }
  .inst-dot-spin { color: var(--accent); font-size: 9px; }
  .inst-dot-pend { color: var(--text-muted); font-size: 9px; }

  .inst-eta {
    font-size: 11px;
    color: var(--text-muted);
  }
  .inst-eta strong { color: var(--text-secondary); }

  /* ── Column tiles ── */
  .col-tile { gap: 8px; }

  /* File tree */
  .tree-path {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent) 20%, transparent);
    border-radius: 4px;
    padding: 3px 8px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .tree-list {
    display: flex;
    flex-direction: column;
    gap: 0;
    border-top: 1px solid var(--border-subtle);
    padding-top: 4px;
    flex: 1;
  }

  .tree-item {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 6px;
    border-radius: 4px;
    font-size: 12px;
    cursor: default;
  }
  .tree-item:hover { background: color-mix(in srgb, var(--surface) 50%, var(--surface-raised)); }

  .tree-sub  { padding-left: 18px; }
  .tree-sub2 { padding-left: 34px; }

  .tree-folder { font-weight: 500; color: var(--text-primary); }
  .tree-file   { color: var(--text-secondary); font-weight: 400; }

  .tree-selected {
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    color: var(--accent);
    font-weight: 500;
  }

  .tree-caret { font-size: 9px; color: var(--text-muted); width: 10px; flex-shrink: 0; }
  .tree-no-caret { visibility: hidden; }

  .tree-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

  .tree-count {
    font-size: 9px;
    color: var(--text-muted);
    background: var(--border);
    border-radius: 8px;
    padding: 1px 5px;
    flex-shrink: 0;
  }

  .tree-size {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .tree-footer {
    display: flex;
    justify-content: space-between;
    padding-top: 8px;
    border-top: 1px solid var(--border-subtle);
  }

  .tree-footer-info,
  .tree-footer-size { font-size: 10px; color: var(--text-muted); }

  /* Inbox */
  .inbox-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .inbox-unread {
    font-size: 9px;
    font-weight: 700;
    color: #fff;
    background: var(--accent);
    border-radius: 8px;
    padding: 1px 6px;
  }

  .inbox-groups {
    display: flex;
    flex-direction: column;
    gap: 0;
    border-top: 1px solid var(--border-subtle);
    flex: 1;
  }

  .inbox-group { padding-bottom: 4px; }

  .inbox-group-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 0 4px;
  }

  .inbox-app-name {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .inbox-badge {
    font-size: 9px;
    font-weight: 700;
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    border-radius: 8px;
    padding: 1px 6px;
  }

  .inbox-item {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 5px 0;
    border-bottom: 1px solid var(--border-subtle);
  }
  .inbox-item:last-child { border-bottom: none; }
  .inbox-item-read { opacity: 0.5; }

  .inbox-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
    margin-top: 4px;
  }
  .inbox-dot-read { background: var(--border); }

  .inbox-body { flex: 1; display: flex; flex-direction: column; gap: 1px; }
  .inbox-msg  { font-size: 11px; color: var(--text-primary); line-height: 1.4; }
  .inbox-time { font-size: 10px; color: var(--text-muted); }

  .inbox-clear {
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

  /* Task list */
  .task-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .task-count { font-size: 10px; color: var(--text-muted); }

  .task-list {
    display: flex;
    flex-direction: column;
    border-top: 1px solid var(--border-subtle);
    flex: 1;
  }

  .task-item {
    display: flex;
    align-items: flex-start;
    gap: 9px;
    padding: 8px 0;
    border-bottom: 1px solid var(--border-subtle);
  }
  .task-item:last-child { border-bottom: none; }
  .task-item-done { opacity: 0.55; }

  .task-check {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    border: 1.5px solid var(--border);
    flex-shrink: 0;
    margin-top: 1px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 9px;
  }

  .task-check-done {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }

  .task-content { flex: 1; display: flex; flex-direction: column; gap: 3px; }

  .task-title { font-size: 12px; color: var(--text-primary); line-height: 1.3; }
  .task-title-done { text-decoration: line-through; color: var(--text-muted); }

  .task-meta { display: flex; align-items: center; gap: 6px; }

  .task-pri {
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    border-radius: 3px;
    padding: 1px 5px;
  }

  .task-high { color: #c04040; background: color-mix(in srgb, #c04040 12%, transparent); }
  .task-med  { color: #b07020; background: color-mix(in srgb, #b07020 12%, transparent); }
  .task-low  { color: var(--text-muted); background: var(--border); }

  .task-due { font-size: 10px; color: var(--text-muted); }
  .task-due-today { color: #c04040; font-weight: 500; }

  .task-add {
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

  /* ── Article tile ── */
  .art-tile { gap: 0; padding: 32px 40px; }

  .art-header {
    padding-bottom: 20px;
    border-bottom: 1px solid var(--border-subtle);
    margin-bottom: 4px;
  }

  .art-eyebrow {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 12px;
  }

  .art-read-time { font-size: 10px; color: var(--text-muted); }

  .art-title {
    font-size: 26px;
    font-weight: 700;
    letter-spacing: -0.025em;
    line-height: 1.2;
    color: var(--text-primary);
    margin: 0 0 16px;
    max-width: 680px;
  }

  .art-meta { display: flex; align-items: center; }

  .art-author-row { display: flex; align-items: center; gap: 10px; }

  .art-avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--accent);
    color: #fff;
    font-size: 11px;
    font-weight: 700;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .art-author-info { display: flex; flex-direction: column; gap: 1px; }
  .art-author-name { font-size: 12px; font-weight: 600; color: var(--text-primary); }
  .art-date        { font-size: 10px; color: var(--text-muted); }

  .art-lead {
    font-size: 15px;
    font-weight: 500;
    line-height: 1.65;
    color: var(--text-primary);
    margin: 20px 0 0;
    max-width: 700px;
  }

  .art-body {
    font-size: 13px;
    line-height: 1.7;
    color: var(--text-primary);
    margin: 16px 0 0;
    max-width: 700px;
  }

  .art-quote {
    border-left: 3px solid var(--accent);
    margin: 24px 0 0;
    padding: 4px 0 4px 20px;
    font-size: 14px;
    font-style: italic;
    color: var(--text-secondary);
    line-height: 1.6;
    max-width: 600px;
  }

  .art-h4 {
    font-size: 13px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 24px 0 0;
    letter-spacing: -0.005em;
  }

  .art-code {
    font-family: 'Geist Mono', monospace;
    font-size: 12px;
    background: color-mix(in srgb, var(--accent) 8%, transparent);
    color: var(--accent);
    border-radius: 3px;
    padding: 1px 5px;
  }

  .art-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-top: 28px;
    padding-top: 16px;
    border-top: 1px solid var(--border-subtle);
  }

  .art-tags { display: flex; gap: 6px; flex-wrap: wrap; }

  .art-tag {
    font-size: 10px;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--surface) 60%, var(--surface-panel));
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 7px;
  }

  /* ── Dashboard tile ── */
  .dash-tile { gap: 0; padding: 28px 32px; }

  .dash-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .dash-title    { font-size: 16px; font-weight: 600; color: var(--text-primary); }
  .dash-subtitle { font-size: 11px; color: var(--text-muted); margin-top: 3px; }

  .dash-select { width: 160px; }

  .dash-metrics {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
    margin-top: 20px;
  }

  .dash-metric {
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px 18px;
  }

  .dash-metric-val {
    font-size: 26px;
    font-weight: 700;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    line-height: 1;
  }

  .dash-metric-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-top: 6px;
  }

  .dash-metric-change { font-size: 10px; margin-top: 6px; font-weight: 500; }
  .dash-up   { color: #3aaa5a; }
  .dash-down { color: #5b9cee; }

  .dash-charts-row {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: 16px;
    margin-top: 20px;
  }

  .dash-chart-panel {
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 16px 18px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .dash-chart-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
  }

  .dash-barchart { flex: 1; display: flex; align-items: flex-end; }

  .dash-bars {
    display: flex;
    align-items: flex-end;
    gap: 5px;
    width: 100%;
    height: 120px;
  }

  .dash-bar {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    justify-content: flex-end;
    height: 100%;
  }

  .dash-bar-fill {
    width: 100%;
    background: var(--accent);
    border-radius: 3px 3px 0 0;
    opacity: 0.75;
    min-height: 4px;
    transition: opacity 0.1s;
  }
  .dash-bar:hover .dash-bar-fill { opacity: 1; }

  .dash-bar-label {
    font-size: 8px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .dash-donut-wrap {
    display: flex;
    align-items: center;
    gap: 18px;
  }

  .dash-donut {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    flex-shrink: 0;
    background: conic-gradient(
      var(--accent) 0% 42%,
      #5b8dee 42% 68%,
      #3aaa6a 68% 86%,
      #e08040 86% 95%,
      var(--border) 95% 100%
    );
    position: relative;
  }
  .dash-donut::after {
    content: '';
    position: absolute;
    inset: 18px;
    border-radius: 50%;
    background: var(--surface-panel);
  }

  .dash-donut-legend { display: flex; flex-direction: column; gap: 5px; }

  .dash-legend-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .dli-dot {
    width: 8px;
    height: 8px;
    border-radius: 2px;
    flex-shrink: 0;
  }
  .dli-accent { background: var(--accent); }
  .dli-blue2  { background: #5b8dee; }
  .dli-green  { background: #3aaa6a; }
  .dli-orange { background: #e08040; }
  .dli-gray   { background: var(--border); }

  .dli-label { font-size: 11px; color: var(--text-secondary); flex: 1; }
  .dli-val   { font-size: 11px; font-weight: 600; color: var(--text-primary); font-family: 'Geist Mono', monospace; }

  /* Dashboard table */
  .dash-table-wrap { margin-top: 20px; }

  .dash-table-title {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: 8px;
  }

  .dash-table {
    width: 100%;
    display: flex;
    flex-direction: column;
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }

  .dash-table-head {
    display: flex;
    align-items: center;
    gap: 0;
    padding: 8px 16px;
    background: color-mix(in srgb, var(--surface-panel) 70%, var(--border));
    border-bottom: 1px solid var(--border);
  }

  .dash-table-row {
    display: flex;
    align-items: center;
    gap: 0;
    padding: 9px 16px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .dash-table-row:last-child { border-bottom: none; }
  .dash-table-row:hover { background: color-mix(in srgb, var(--surface) 40%, var(--surface-raised)); }

  .dth {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .dtd {
    font-size: 12px;
    color: var(--text-primary);
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .dth-app      { flex: 1.6; }
  .dth-ver      { flex: 1; }
  .dth-dl       { flex: 1; }
  .dth-installs { flex: 1; }
  .dth-release  { flex: 1; }
  .dth-status   { flex: 0.8; }

  .dash-app-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .dash-build {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    border-radius: 4px;
    padding: 2px 7px;
  }

  .dash-build-pass { background: color-mix(in srgb, #3aaa5a 15%, transparent); color: #2a8a4a; }
  .dash-build-warn { background: color-mix(in srgb, #b07020 15%, transparent); color: #a06010; }
  .dash-build-fail { background: color-mix(in srgb, #c04040 15%, transparent); color: #a03030; }
</style>
