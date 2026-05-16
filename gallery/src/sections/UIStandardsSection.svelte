<script>
  import { GlobalTabs } from '@libre/ui';
  import Card from '../lib/Card.svelte';
  import { canvas } from '../lib/stores/canvas.svelte.js';

  let settingsTab = $state('general');

  const settingsTabs = [
    { id: 'general',    label: 'General'    },
    { id: 'appearance', label: 'Appearance' },
    { id: 'shortcuts',  label: 'Shortcuts'  },
    { id: 'advanced',   label: 'Advanced'   },
  ];

  const recentProjects = [
    { name: 'Product Launch',  modified: '2 hours ago' },
    { name: 'Q1 Campaign',     modified: 'Yesterday'   },
    { name: 'Brand Assets',    modified: '3 days ago'  },
    { name: 'Archive 2024',    modified: 'Jan 15'      },
  ];

  let miniPmProject = $state('Libre UI');
  let fullPmSelected = $state(0);
  let fullPmFolders  = $state({ active: true, apps: true, archive: false });

  let fullTbTabLeft  = $state('foundation');
  let fullTbTabRight = $state('applications');
  let fullTbTheme    = $state('auto');
  const ftbTabsFoundation = [
    { id: 'foundation', label: 'Tab 1' },
    { id: 'components', label: 'Tab 2' },
  ];
  const ftbTabsApps = [
    { id: 'applications', label: 'Tab 3' },
    { id: 'overview',     label: 'Tab 4' },
  ];
</script>

<div class="section">

  <!-- ── Titlebar ───────────────────────────────────────────────── -->
  <h2 class="group-title">Titlebar</h2>
  <p class="group-desc">
    38px drag region. Traffic lights left, app name centered, optional toolbar right.
    <code>--titlebar-bg</code> background, <code>--border</code> bottom edge.
  </p>
  <Card id="UIS-1" label="Titlebar" effects={[
    { label: 'Drop shadow',    value: '0 4px 16px / 10% opacity' },
    { label: 'Ambient shadow', value: '0 1px 4px / 8% opacity'   },
  ]}>
    <div class="demo-block-padded">
      <div class="titlebar-frame">
        {#if canvas.osMode === 'macos'}
          <div class="titlebar">
            <div class="titlebar-left">
              <div class="tl-dot tl-close"></div>
              <div class="tl-dot tl-min"></div>
              <div class="tl-dot tl-max"></div>
            </div>
            <div class="titlebar-center">
              <span class="titlebar-appname">Shelf</span>
            </div>
            <div class="titlebar-right">
              <button class="tb-icon-btn" aria-label="Search">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
                </svg>
              </button>
              <button class="tb-icon-btn" aria-label="Settings">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <circle cx="12" cy="12" r="3"/>
                  <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
                </svg>
              </button>
            </div>
          </div>
        {:else}
          <div class="titlebar" style="padding-right: 0;">
            <div class="win-left">
              <div class="win-appicon"></div>
              <span class="titlebar-appname">Shelf</span>
            </div>
            <div class="win-controls">
              <button class="win-btn" aria-label="Minimize">
                <svg width="10" height="1" viewBox="0 0 10 1" fill="none"><rect width="10" height="1" fill="currentColor"/></svg>
              </button>
              <button class="win-btn" aria-label="Maximize">
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none"><rect x="0.75" y="0.75" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.5"/></svg>
              </button>
              <button class="win-btn win-close" aria-label="Close">
                <svg width="10" height="10" viewBox="0 0 10 10" fill="none"><line x1="1" y1="1" x2="9" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><line x1="9" y1="1" x2="1" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
              </button>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </Card>

  <!-- ── Titlebar — Full ───────────────────────────────────────── -->
  <h2 class="group-title">Titlebar — Full</h2>
  <p class="group-desc">
    44px top bar used across the Libre family. Logo + doc picker + save controls left,
    dual <code>GlobalTabs</code> groups centered absolutely, theme toggle + nav controls right.
    Tabs switch active state; all other controls are cosmetic hover-animate only.
  </p>
  <Card id="UIS-2" label="Titlebar — Full" effects={[]}>
    <div class="ftb-bar">

      <!-- LEFT: doc picker · save -->
      <div class="ftb-left">
        <button class="ftb-doc-btn">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" class="ftb-doc-ico">
            <rect x="2" y="6" width="20" height="12" rx="2"/>
            <line x1="2" y1="10" x2="22" y2="10"/>
            <line x1="2" y1="14" x2="22" y2="14"/>
            <line x1="7" y1="6" x2="7" y2="18"/>
            <line x1="17" y1="6" x2="17" y2="18"/>
          </svg>
          <span class="ftb-doc-name">Libre UI</span>
          <svg width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="ftb-doc-chev">
            <polyline points="6 9 12 15 18 9"/>
          </svg>
        </button>
        <div class="ftb-divider"></div>
        <button class="ftb-save-as">Save As</button>
        <button class="ftb-save" aria-label="Save">
          <div class="ftb-save-inner">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
              <polyline points="17 21 17 13 7 13 7 21"/>
              <polyline points="7 3 7 8 15 8"/>
            </svg>
            <span class="ftb-unsaved-dot" aria-hidden="true"></span>
          </div>
        </button>
      </div>

      <!-- CENTER: dual GlobalTabs — absolutely centered -->
      <div class="ftb-center">
        <div class="ftb-tab-groups">
          <GlobalTabs tabs={ftbTabsFoundation} bind:active={fullTbTabLeft} />
          <div class="ftb-sep"></div>
          <GlobalTabs tabs={ftbTabsApps} bind:active={fullTbTabRight} color="#e07a2f" />
        </div>
      </div>

      <!-- RIGHT: theme · home · gear · sidebar -->
      <div class="ftb-right">
        <button
          class="ftb-icon-btn"
          class:ftb-theme-light={fullTbTheme === 'light'}
          class:ftb-theme-dark={fullTbTheme === 'dark'}
          onclick={() => fullTbTheme = fullTbTheme === 'auto' ? 'light' : fullTbTheme === 'light' ? 'dark' : 'auto'}
          aria-label="Cycle theme"
        >
          {#if fullTbTheme === 'light'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="4"/>
              <line x1="12" y1="2"  x2="12" y2="5"/>
              <line x1="12" y1="19" x2="12" y2="22"/>
              <line x1="4.22" y1="4.22"  x2="6.34" y2="6.34"/>
              <line x1="17.66" y1="17.66" x2="19.78" y2="19.78"/>
              <line x1="2"  y1="12" x2="5"  y2="12"/>
              <line x1="19" y1="12" x2="22" y2="12"/>
              <line x1="4.22" y1="19.78" x2="6.34" y2="17.66"/>
              <line x1="17.66" y1="6.34" x2="19.78" y2="4.22"/>
            </svg>
          {:else if fullTbTheme === 'dark'}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
            </svg>
          {:else}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M12 5 A7 7 0 0 0 12 19 Z" fill="currentColor" stroke="none"/>
              <circle cx="12" cy="12" r="7"/>
              <line x1="12" y1="5" x2="12" y2="19"/>
            </svg>
          {/if}
        </button>
        <button class="ftb-icon-btn" aria-label="Project Manager">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
            <polyline points="9 22 9 12 15 12 15 22"/>
          </svg>
        </button>
        <button class="ftb-icon-btn" aria-label="Settings">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>
        <button class="ftb-icon-btn" aria-label="Toggle right panel">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <rect x="3" y="3" width="18" height="18" rx="2"/>
            <line x1="15" y1="3" x2="15" y2="21"/>
          </svg>
        </button>
      </div>
    </div>
  </Card>

  <!-- ── WindowFrame ────────────────────────────────────────────── -->
  <h2 class="group-title">WindowFrame</h2>
  <p class="group-desc">
    Full window shell. 10px border-radius, 1px <code>--border</code> edge, drop shadow.
    <code>decorations: false</code> + <code>transparent: true</code> in <code>tauri.conf.json</code> — all four config values must be set together.
    Titlebar at top, optional status bar at bottom, content fills the middle.
  </p>
  <Card id="UIS-3" label="WindowFrame" effects={[
    { label: 'Drop shadow',         value: '0 8px 32px / 18% opacity' },
    { label: 'Ambient shadow',      value: '0 2px 8px / 10% opacity'  },
  ]}>
    <div class="wf-mock">
      {#if canvas.osMode === 'macos'}
        <div class="wf-titlebar">
          <div class="tl-dot tl-close"></div>
          <div class="tl-dot tl-min"></div>
          <div class="tl-dot tl-max"></div>
          <span class="wf-appname">Shelf</span>
        </div>
      {:else}
        <div class="wf-titlebar wf-titlebar-win">
          <div class="win-left">
            <div class="win-appicon"></div>
            <span class="wf-appname">Shelf</span>
          </div>
          <div class="win-controls">
            <button class="win-btn" aria-label="Minimize">
              <svg width="10" height="1" viewBox="0 0 10 1" fill="none"><rect width="10" height="1" fill="currentColor"/></svg>
            </button>
            <button class="win-btn" aria-label="Maximize">
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none"><rect x="0.75" y="0.75" width="8.5" height="8.5" rx="1" stroke="currentColor" stroke-width="1.5"/></svg>
            </button>
            <button class="win-btn win-close" aria-label="Close">
              <svg width="10" height="10" viewBox="0 0 10 10" fill="none"><line x1="1" y1="1" x2="9" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><line x1="9" y1="1" x2="1" y2="9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/></svg>
            </button>
          </div>
        </div>
      {/if}
      <div class="wf-body">
        <span class="wf-label">main content area</span>
      </div>
      <div class="wf-statusbar">
        <div class="status-dot status-dot-ok"></div>
        <span class="wf-status-text">Ready</span>
        <span class="wf-status-sep"></span>
        <span class="wf-status-text">14 items</span>
      </div>
    </div>
  </Card>

  <!-- ── Status Bar ─────────────────────────────────────────────── -->
  <h2 class="group-title">Status Bar</h2>
  <p class="group-desc">
    24px footer strip. Status text left, context info center, secondary actions right.
    <code>--surface-raised</code> background, <code>--border</code> top edge.
  </p>
  <Card id="UIS-4" label="Status Bar" effects={[]}>
    <div class="statusbar">
      <div class="statusbar-section">
        <div class="status-dot status-dot-ok"></div>
        <span class="statusbar-text">Ready</span>
      </div>
      <div class="statusbar-sep"></div>
      <div class="statusbar-section">
        <span class="statusbar-text">14 items</span>
      </div>
      <div class="statusbar-sep"></div>
      <div class="statusbar-section statusbar-section-fill">
        <span class="statusbar-text statusbar-path">/Users/eldo/Documents/Projects</span>
      </div>
      <div class="statusbar-sep"></div>
      <div class="statusbar-section">
        <span class="statusbar-text">macOS</span>
      </div>
    </div>
  </Card>

  <!-- ── About ──────────────────────────────────────────────────── -->
  <h2 class="group-title">About</h2>
  <p class="group-desc">
    Centered panel. App identity, version, attribution, legal links.
    Standard size: 300px wide.
  </p>
  <Card id="UIS-5" label="About" effects={[]}>
    <div class="about-panel">
      <div class="about-icon">
        <svg width="32" height="32" viewBox="0 0 40 40" fill="none">
          <rect width="40" height="40" rx="10" fill="var(--accent)" opacity="0.15"/>
          <rect width="40" height="40" rx="10" fill="none" stroke="var(--accent)" stroke-width="1.5" opacity="0.4"/>
          <text x="20" y="27" text-anchor="middle" font-size="18" fill="var(--accent)" font-family="system-ui">◈</text>
        </svg>
      </div>
      <div class="about-name">Shelf</div>
      <div class="about-version">Version 1.0.0 (build 42)</div>
      <div class="about-family">Part of the Libre family of software.</div>
      <div class="about-by">By <strong>Iron Tree Software</strong></div>
      <div class="about-divider"></div>
      <div class="about-links">
        <button class="about-link-btn about-gh-btn">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
            <path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
          </svg>
          GitHub
        </button>
        <button class="about-link-btn">Check for Updates</button>
      </div>
    </div>
  </Card>
  <p class="about-behavior">
    Dismissed by clicking outside the panel — no close button.
    The two action buttons navigate in-app without closing it.
  </p>

  <!-- ── Settings ───────────────────────────────────────────────── -->
  <h2 class="group-title">Settings</h2>
  <p class="group-desc">
    Two-column panel. Category sidebar left (140px), content area right.
    Shared layout across all apps — only the content varies.
  </p>
  <Card id="UIS-6" label="Settings" effects={[]}>
    <div class="settings-panel">
      <div class="settings-hd">
        <span class="settings-hd-title">Settings</span>
        <div class="settings-hd-actions">
          <button class="settings-hd-btn" title="Save">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
              <polyline points="17 21 17 13 7 13 7 21"/>
              <polyline points="7 3 7 8 15 8"/>
            </svg>
          </button>
          <button class="settings-hd-btn" title="Load">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
          </button>
          <button class="settings-hd-btn" title="Reset to Defaults">
            <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="1 4 1 10 7 10"/>
              <path d="M3.51 15a9 9 0 1 0 .49-4.95"/>
            </svg>
          </button>
        </div>
      </div>
      <div class="settings-body">
      <div class="settings-sidebar">
        {#each settingsTabs as tab}
          <button
            class="settings-tab"
            class:settings-tab-active={settingsTab === tab.id}
            onclick={() => settingsTab = tab.id}
          >{tab.label}</button>
        {/each}
      </div>
      <div class="settings-content">
        {#if settingsTab === 'general'}
          <div class="settings-group">
            <div class="settings-row">
              <label class="settings-label">Open on launch</label>
              <select class="settings-select">
                <option>Last project</option>
                <option>New project</option>
                <option>Project manager</option>
              </select>
            </div>
            <div class="settings-row">
              <label class="settings-label">Auto-save</label>
              <div class="settings-toggle settings-toggle-on"></div>
            </div>
            <div class="settings-row">
              <label class="settings-label">Save interval</label>
              <div class="settings-stepper">
                <button class="step-btn">−</button>
                <span class="step-val">30s</span>
                <button class="step-btn">+</button>
              </div>
            </div>
          </div>
        {:else if settingsTab === 'appearance'}
          <div class="settings-group">
            <div class="settings-row">
              <label class="settings-label">Theme</label>
              <select class="settings-select">
                <option>System</option>
                <option>Light</option>
                <option>Dark</option>
              </select>
            </div>
            <div class="settings-row">
              <label class="settings-label">Density</label>
              <select class="settings-select">
                <option>Compact</option>
                <option>Default</option>
              </select>
            </div>
          </div>
        {:else if settingsTab === 'shortcuts'}
          <div class="settings-placeholder">
            <span>Keyboard shortcuts</span>
          </div>
        {:else}
          <div class="settings-placeholder">
            <span>Advanced settings</span>
          </div>
        {/if}
      </div>
      </div>
    </div>
  </Card>

  <!-- ── Project Manager — Picker ─────────────────────────────── -->
  <h2 class="group-title">Project Manager — Picker</h2>
  <p class="group-desc">
    Top-bar document switcher. Trigger shows the current project name; dropdown opens with current project, new/open actions, and a recents list. Dismissed by clicking outside or switching projects.
  </p>
  <Card id="UIS-7" label="Project Manager — Picker" effects={[
    { label: 'Dropdown shadow', value: '0 4px 16px / 12% opacity' },
    { label: 'Ambient shadow',  value: '0 1px 4px / 8% opacity'   },
  ]}>
    <div class="dp-stage">
      <button class="dp-trigger dp-trigger-open">
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <rect x="2" y="6" width="20" height="12" rx="2"/>
          <line x1="2" y1="10" x2="22" y2="10"/>
          <line x1="2" y1="14" x2="22" y2="14"/>
          <line x1="7" y1="6" x2="7" y2="18"/>
          <line x1="17" y1="6" x2="17" y2="18"/>
        </svg>
        <span class="dp-name">{miniPmProject}</span>
        <svg width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="dp-chev-open">
          <polyline points="6 9 12 15 18 9"/>
        </svg>
      </button>
      <div class="dp-menu">
        <div class="dp-menu-label">Current Project</div>
        <div class="dp-current">
          <span class="dp-current-name">{miniPmProject}</span>
          <button class="dp-rename-btn" aria-label="Rename">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
              <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
            </svg>
          </button>
        </div>
        <div class="dp-sep"></div>
        <button class="dp-action">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
          </svg>
          New Project
        </button>
        <button class="dp-action">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
          </svg>
          Open Project…
        </button>
        <div class="dp-sep"></div>
        <div class="dp-menu-label">Recent</div>
        {#each recentProjects as project}
          <button class="dp-recent" onclick={() => { miniPmProject = project.name; }}>
            {project.name}
          </button>
        {/each}
      </div>
    </div>
  </Card>

  <!-- ── Project Manager — Full ─────────────────────────────────── -->
  <h2 class="group-title">Project Manager — Full</h2>
  <p class="group-desc">
    Full-size project manager. Opened via the home icon in the top-bar right group.
    Two-panel layout: project list left, project detail right. Backdrop-dismissed.
  </p>
  <Card id="UIS-8" label="Project Manager — Full" effects={[
    { label: 'Backdrop scrim', value: 'rgba 0,0,0 / 40% opacity' },
  ]}>
    <div class="full-pm">
      <div class="full-pm-hd">
        <span class="full-pm-title">Project Manager</span>
        <button class="full-pm-close" aria-label="Close">
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>
      <div class="full-pm-body">
        <div class="full-pm-sidebar">
          <div class="full-pm-sb-hd">
            <span class="full-pm-sb-label">Projects</span>
            <button class="full-pm-new" aria-label="New project">
              <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
            </button>
          </div>
          {#each [
            { key: 'active',  label: 'Active', projects: [
              { name: 'Libre UI',       mod: 'Today',       idx: 0 },
              { name: 'Component Lab',  mod: 'Yesterday',   idx: 1 },
              { name: 'Token Sandbox',  mod: '3 days ago',  idx: 2 },
            ]},
            { key: 'apps',    label: 'Apps', projects: [
              { name: 'Fade Converter', mod: 'Last week',   idx: 3 },
              { name: 'Prism Viewer',   mod: 'Last week',   idx: 4 },
              { name: 'Stack Editor',   mod: '2 weeks ago', idx: 5 },
            ]},
            { key: 'archive', label: 'Archive', projects: [
              { name: 'Motion Demos',       mod: 'Last month', idx: 6 },
              { name: 'Accessibility Pass', mod: 'Last month', idx: 7 },
            ]},
          ] as folder}
            <div class="full-pm-folder">
              <button class="full-pm-folder-hd" onclick={() => { fullPmFolders = { ...fullPmFolders, [folder.key]: !fullPmFolders[folder.key] }; }}>
                <svg class="full-pm-chevron" class:full-pm-chevron-open={fullPmFolders[folder.key]}
                     width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                     stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="9 18 15 12 9 6"/>
                </svg>
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                     stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
                <span class="full-pm-folder-name">{folder.label}</span>
                <span class="full-pm-folder-count">{folder.projects.length}</span>
              </button>
              {#if fullPmFolders[folder.key]}
                {#each folder.projects as p}
                  <button class="full-pm-row full-pm-row-indented" class:full-pm-row-active={fullPmSelected === p.idx} onclick={() => fullPmSelected = p.idx}>
                    <div class="full-pm-ico">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                        <line x1="3" y1="9" x2="21" y2="9"/>
                        <line x1="9" y1="21" x2="9" y2="9"/>
                      </svg>
                    </div>
                    <div class="full-pm-meta">
                      <span class="full-pm-pname">{p.name}</span>
                      <span class="full-pm-pmod">{p.mod}</span>
                    </div>
                  </button>
                {/each}
              {/if}
            </div>
          {/each}
        </div>
        <div class="full-pm-detail">
          <div class="full-pm-hero">
            <div class="full-pm-glyph">◈</div>
            <div class="full-pm-hero-text">
              <h3 class="full-pm-dname">Libre UI</h3>
              <span class="full-pm-dpath">~/Downloads/Github/Libre-Apps/gallery</span>
            </div>
          </div>
          <div class="full-pm-meta-grid">
            <div class="full-pm-mitem"><span class="full-pm-mkey">Last opened</span><span class="full-pm-mval">Today at 2:14 PM</span></div>
            <div class="full-pm-mitem"><span class="full-pm-mkey">Created</span><span class="full-pm-mval">Apr 20, 2026</span></div>
            <div class="full-pm-mitem"><span class="full-pm-mkey">Runtime</span><span class="full-pm-mval">Tauri + Svelte 5</span></div>
            <div class="full-pm-mitem"><span class="full-pm-mkey">Token source</span><span class="full-pm-mval">common-js/src/tokens.css</span></div>
            <div class="full-pm-mitem"><span class="full-pm-mkey">Identifier</span><span class="full-pm-mval">io.librewin.gallery</span></div>
            <div class="full-pm-mitem"><span class="full-pm-mkey">Version</span><span class="full-pm-mval">0.1.0</span></div>
          </div>
          <div class="full-pm-sec-label">Recent Snapshots</div>
          <div class="full-pm-chips">
            {#each ['Buttons — dark mode', 'Token audit', 'Navigation variants', 'Form controls pass'] as s}
              <div class="full-pm-chip">{s}</div>
            {/each}
          </div>
          <div class="full-pm-sec-label">Actions</div>
          <div class="full-pm-actions">
            <button class="full-pm-act full-pm-act-primary">Open Project</button>
            <button class="full-pm-act">Duplicate</button>
            <button class="full-pm-act">Export Snapshot</button>
            <button class="full-pm-act full-pm-act-danger">Remove from List</button>
          </div>
        </div>
      </div>
    </div>
  </Card>

</div>

<style>
  .section { max-width: 1125px; }

  .group-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 40px 0 6px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  .group-desc {
    font-size: 12px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 0 0 16px;
  }

  .group-desc code {
    font-family: 'Geist Mono', monospace;
    font-size: 11px;
    background: color-mix(in srgb, var(--text-primary) 7%, transparent);
    padding: 1px 4px;
    border-radius: 3px;
    color: var(--text-primary);
  }

  .demo-block-padded {
    padding: 32px;
    display: flex;
    align-items: flex-start;
    justify-content: center;
  }

  /* ── Titlebar ── */
  .titlebar {
    height: 38px;
    background: var(--titlebar-bg);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 12px;
    position: relative;
    user-select: none;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 6px;
    z-index: 1;
  }

  .tl-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tl-close { background: #ff5f57; }
  .tl-min   { background: #ffbd2e; }
  .tl-max   { background: #28c840; }

  .titlebar-center {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .titlebar-appname {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .titlebar-right {
    margin-left: auto;
    display: flex;
    align-items: center;
    gap: 2px;
    z-index: 1;
  }

  .tb-icon-btn {
    width: 28px;
    height: 26px;
    padding: 0;
    border: none;
    background: transparent;
    border-radius: var(--radius-sm);
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 80ms, color 80ms;
  }

  .tb-icon-btn:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }

  /* ── Titlebar frame (shared box for titlebar-only sections) ── */
  .titlebar-frame {
    width: 926px;
    border-radius: 10px;
    border: 1px solid var(--border);
    overflow: hidden;
    background: var(--surface);
    box-shadow: 0 4px 16px rgba(0 0 0 / 0.10), 0 1px 4px rgba(0 0 0 / 0.08);
  }

  /* ── Windows controls ── */
  .win-left {
    flex: 1;
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .win-appicon {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    background: var(--accent);
    opacity: 0.7;
    flex-shrink: 0;
  }

  .win-controls {
    display: flex;
    align-items: center;
    height: 100%;
    flex-shrink: 0;
  }

  .win-btn {
    width: 46px;
    height: 100%;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 80ms, color 80ms;
  }

  .win-btn:hover { background: var(--surface-hover); color: var(--text-primary); }

  .win-close:hover { background: #c42b1c; color: #fff; }

  /* wf-titlebar Windows variant — remove right padding so controls flush to edge */
  .wf-titlebar-win {
    padding-right: 0;
    gap: 0;
  }

  /* ── Titlebar — Full ── */
  .ftb-bar {
    width: 926px;
    height: 44px;
    display: flex;
    align-items: center;
    padding: 0 12px;
    border-bottom: 1px solid var(--border);
    background: var(--titlebar-bg, var(--surface-panel));
    position: relative;
    user-select: none;
  }

  .ftb-left {
    display: flex;
    align-items: center;
    gap: 8px;
    z-index: 2;
    min-width: 0;
  }

  .ftb-glyph { color: var(--accent); font-size: 14px; flex-shrink: 0; }

  .ftb-app {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    flex-shrink: 0;
  }

  .ftb-divider {
    width: 1px;
    height: 14px;
    background: var(--border);
    margin: 0 2px;
    flex-shrink: 0;
  }

  .ftb-doc-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 8px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }
  .ftb-doc-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }
  .ftb-doc-ico  { opacity: 0.7; flex-shrink: 0; }
  .ftb-doc-name { white-space: nowrap; }
  .ftb-doc-chev { opacity: 0.5; flex-shrink: 0; }

  .ftb-save-as {
    display: flex;
    align-items: center;
    padding: 4px 8px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .ftb-save-as:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }

  .ftb-save {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px 6px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .ftb-save:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }

  .ftb-save-inner {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .ftb-unsaved-dot {
    position: absolute;
    top: -3px;
    right: -3px;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #ef4444;
    border: 1.5px solid var(--titlebar-bg, var(--surface-panel));
  }

  .ftb-center {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }
  .ftb-center :global(.gt-group) { pointer-events: auto; }

  .ftb-tab-groups {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .ftb-sep {
    width: 1px;
    height: 16px;
    background: rgb(255 255 255 / 0.15);
    flex-shrink: 0;
  }

  .ftb-right {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-left: auto;
    z-index: 2;
  }

  .ftb-icon-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .ftb-icon-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 7%, transparent);
    color: var(--text-primary);
  }
  .ftb-theme-light { color: #d97706; }
  .ftb-theme-dark  { color: #818cf8; }
  .ftb-theme-light:hover { color: #b45309; }
  .ftb-theme-dark:hover  { color: #6366f1; }

  /* ── WindowFrame ── */
  .wf-mock {
    width: 680px;
    border-radius: 10px;
    border: 1px solid var(--border);
    overflow: hidden;
    background: var(--surface);
    display: flex;
    flex-direction: column;
    box-shadow: 0 8px 32px rgba(0 0 0 / 0.18), 0 2px 8px rgba(0 0 0 / 0.10);
  }

  .wf-titlebar {
    height: 38px;
    background: var(--titlebar-bg);
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 12px;
    flex-shrink: 0;
  }

  .wf-appname {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    letter-spacing: -0.01em;
    margin-left: 6px;
  }

  .wf-body {
    flex: 1;
    min-height: 80px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--surface);
  }

  .wf-label {
    font-size: 11px;
    color: var(--text-muted);
  }

  .wf-statusbar {
    height: 24px;
    background: var(--surface-raised);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 10px;
    flex-shrink: 0;
  }

  .wf-status-text {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .wf-status-sep {
    width: 1px;
    height: 10px;
    background: var(--border);
    margin: 0 2px;
    flex-shrink: 0;
  }

  /* ── Status bar ── */
  .statusbar {
    width: 680px;
    height: 24px;
    background: var(--surface-raised);
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    padding: 0 2px;
  }

  .statusbar-section {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 0 10px;
    height: 100%;
  }

  .statusbar-section-fill { flex: 1; min-width: 0; }

  .statusbar-sep {
    width: 1px;
    height: 14px;
    background: var(--border);
    flex-shrink: 0;
  }

  .statusbar-text {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .statusbar-path {
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .status-dot-ok { background: var(--color-success); }

  /* ── About ── */
  .about-panel {
    width: 300px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 6px;
    padding: 28px 24px 20px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
  }

  .about-icon { margin-bottom: 4px; }

  .about-name {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
  }

  .about-version {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .about-by {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .about-by strong {
    color: var(--text-primary);
    font-weight: 600;
  }

  .about-divider {
    width: 100%;
    height: 1px;
    background: var(--border);
    margin: 10px 0 4px;
  }

  .about-links {
    display: flex;
    gap: 8px;
  }

  .about-family {
    font-size: 11px;
    color: var(--text-muted);
  }

  .about-behavior {
    font-size: 11px;
    color: var(--text-muted);
    line-height: 1.6;
    margin: 10px 0 0;
    font-style: italic;
  }

  .about-link-btn {
    padding: 4px 10px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-secondary);
    font-size: 11px;
    font-family: inherit;
    cursor: pointer;
    display: inline-flex;
    align-items: center;
    gap: 5px;
    transition: background 80ms, border-color 80ms, color 80ms;
  }

  .about-gh-btn { color: var(--text-primary); }

  .about-link-btn:hover {
    background: var(--surface-hover);
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
    color: var(--text-primary);
  }

  /* ── Settings ── */
  .settings-panel {
    width: 500px;
    display: flex;
    flex-direction: column;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    height: 696px;
  }

  .settings-hd {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 34px;
    padding: 0 10px 0 14px;
    border-bottom: 1px solid var(--border);
    background: var(--titlebar-bg, var(--surface-panel));
    flex-shrink: 0;
  }

  .settings-hd-title {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .settings-hd-actions {
    display: flex;
    align-items: center;
    gap: 1px;
  }

  .settings-hd-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: var(--radius-sm);
    cursor: pointer;
    transition: background 80ms, color 80ms;
  }

  .settings-hd-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 7%, transparent);
    color: var(--text-primary);
  }

  .settings-body {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  .settings-sidebar {
    width: 140px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface);
    padding: 8px 0;
    display: flex;
    flex-direction: column;
    gap: 1px;
  }

  .settings-tab {
    width: 100%;
    padding: 7px 14px;
    text-align: left;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 13px;
    font-family: inherit;
    cursor: pointer;
    border-radius: 0;
    transition: background 80ms, color 80ms;
  }

  .settings-tab:hover { background: var(--surface-hover); color: var(--text-primary); }

  .settings-tab-active {
    background: color-mix(in srgb, var(--accent) 10%, var(--surface));
    color: var(--accent);
    font-weight: 500;
  }

  .settings-tab-active:hover {
    background: color-mix(in srgb, var(--accent) 16%, var(--surface));
    color: var(--accent);
  }

  .settings-content {
    flex: 1;
    padding: 20px;
    min-width: 0;
  }

  .settings-group {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .settings-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
  }

  .settings-label {
    font-size: 13px;
    color: var(--text-primary);
    flex-shrink: 0;
  }

  .settings-select {
    height: 26px;
    padding: 0 8px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-primary);
    font-size: 12px;
    font-family: inherit;
    appearance: none;
    cursor: pointer;
    min-width: 140px;
  }

  .settings-toggle {
    width: 36px;
    height: 20px;
    border-radius: 10px;
    background: var(--border);
    position: relative;
    cursor: pointer;
    transition: background 120ms;
    flex-shrink: 0;
  }

  .settings-toggle::after {
    content: '';
    position: absolute;
    width: 14px;
    height: 14px;
    border-radius: 50%;
    background: #fff;
    top: 3px;
    left: 3px;
    transition: left 120ms;
    box-shadow: 0 1px 3px rgba(0 0 0 / 0.25);
  }

  .settings-toggle-on {
    background: var(--accent);
  }

  .settings-toggle-on::after {
    left: 19px;
  }

  .settings-stepper {
    display: flex;
    align-items: center;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
  }

  .step-btn {
    width: 26px;
    height: 26px;
    border: none;
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 14px;
    cursor: pointer;
    transition: background 80ms;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: inherit;
  }

  .step-btn:hover { background: var(--surface-hover); color: var(--text-primary); }

  .step-val {
    padding: 0 10px;
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-primary);
    border-left: 1px solid var(--border);
    border-right: 1px solid var(--border);
    height: 26px;
    display: flex;
    align-items: center;
    background: var(--surface-raised);
  }

  .settings-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 120px;
    color: var(--text-muted);
    font-size: 12px;
  }


  /* ── Project Manager — Picker ── */
  .dp-stage {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0;
  }

  .dp-trigger {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 0 8px;
    height: 28px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background 80ms, color 80ms;
  }

  .dp-trigger:hover { color: var(--text-primary); background: var(--surface-hover); }

  .dp-trigger-open {
    color: var(--text-primary);
    border-color: color-mix(in srgb, var(--accent) 50%, var(--border));
  }

  .dp-name { font-weight: 500; }

  .dp-chev-open { transform: rotate(180deg); }

  .dp-menu {
    margin-top: 4px;
    min-width: 240px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    padding: 4px 0;
    box-shadow: 0 4px 16px rgba(0 0 0 / 0.12), 0 1px 4px rgba(0 0 0 / 0.08);
  }

  .dp-menu-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 6px 12px 3px;
  }

  .dp-current {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 5px 8px 5px 12px;
    gap: 8px;
  }

  .dp-current-name {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    flex: 1;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dp-rename-btn {
    width: 22px;
    height: 22px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
    transition: background 80ms, color 80ms;
  }

  .dp-rename-btn:hover { background: var(--surface-hover); color: var(--text-primary); }

  .dp-sep {
    height: 1px;
    background: var(--border);
    margin: 4px 0;
  }

  .dp-action {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 6px 12px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    text-align: left;
    cursor: pointer;
    transition: background 80ms, color 80ms;
  }

  .dp-action:hover { background: var(--surface-hover); color: var(--text-primary); }

  .dp-recent {
    display: block;
    width: 100%;
    padding: 5px 12px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    text-align: left;
    cursor: pointer;
    transition: background 80ms, color 80ms;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .dp-recent:hover { background: var(--surface-hover); color: var(--text-primary); }

  /* ── Project Manager — Full ── */
  .full-pm {
    width: 800px;
    height: 825px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .full-pm-hd {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px 12px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .full-pm-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .full-pm-close {
    width: 24px;
    height: 24px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    transition: background 80ms, color 80ms;
  }

  .full-pm-close:hover { background: var(--surface-hover); color: var(--text-primary); }

  .full-pm-body {
    display: flex;
    flex: 1;
    min-height: 0;
  }

  .full-pm-sidebar {
    width: 180px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface);
    display: flex;
    flex-direction: column;
  }

  .full-pm-sb-hd {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px 8px;
    border-bottom: 1px solid var(--border);
  }

  .full-pm-sb-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .full-pm-new {
    width: 20px;
    height: 20px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 80ms, color 80ms;
  }

  .full-pm-new:hover { background: var(--surface-hover); color: var(--accent); }

  .full-pm-folder { display: flex; flex-direction: column; }

  .full-pm-folder-hd {
    display: flex;
    align-items: center;
    gap: 5px;
    width: 100%;
    padding: 4px 10px;
    background: transparent;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    text-align: left;
    transition: background 80ms, color 80ms;
  }
  .full-pm-folder-hd:hover { background: var(--surface-hover); color: var(--text-secondary); }

  .full-pm-chevron { transition: transform 0.15s; flex-shrink: 0; }
  .full-pm-chevron-open { transform: rotate(90deg); }

  .full-pm-folder-name {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    flex: 1;
  }

  .full-pm-folder-count {
    font-size: 10px;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    border-radius: 8px;
    padding: 1px 5px;
  }

  .full-pm-row-indented { padding-left: 24px; }

  .full-pm-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 10px;
    border: none;
    background: transparent;
    cursor: pointer;
    width: 100%;
    text-align: left;
    transition: background 80ms;
  }

  .full-pm-row:hover { background: var(--surface-hover); }

  .full-pm-row-active {
    background: color-mix(in srgb, var(--accent) 10%, var(--surface));
  }

  .full-pm-row-active:hover {
    background: color-mix(in srgb, var(--accent) 14%, var(--surface));
  }

  .full-pm-ico {
    width: 22px;
    height: 22px;
    border-radius: var(--radius-sm);
    background: color-mix(in srgb, var(--accent) 10%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--accent) 25%, var(--border));
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .full-pm-meta { display: flex; flex-direction: column; gap: 1px; flex: 1; min-width: 0; }

  .full-pm-pname {
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .full-pm-pmod {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .full-pm-detail {
    flex: 1;
    padding: 20px;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .full-pm-hero {
    display: flex;
    align-items: center;
    gap: 14px;
    margin-bottom: 16px;
  }

  .full-pm-glyph {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    background: color-mix(in srgb, var(--accent) 12%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--accent) 30%, var(--border));
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 18px;
    color: var(--accent);
    flex-shrink: 0;
  }

  .full-pm-dname {
    font-size: 16px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    margin: 0 0 2px;
  }

  .full-pm-dpath {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }

  .full-pm-meta-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    overflow: hidden;
    margin-bottom: 16px;
  }

  .full-pm-mitem {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 8px 10px;
    background: var(--surface);
  }

  .full-pm-mkey {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .full-pm-mval {
    font-size: 12px;
    color: var(--text-primary);
    font-family: 'Geist Mono', monospace;
  }

  .full-pm-sec-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 8px;
    margin-top: 4px;
  }

  .full-pm-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    margin-bottom: 16px;
  }

  .full-pm-chip {
    padding: 3px 10px;
    border: 1px solid var(--border);
    border-radius: 100px;
    font-size: 11px;
    color: var(--text-secondary);
    background: var(--surface);
  }

  .full-pm-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .full-pm-act {
    padding: 5px 14px;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background 80ms, color 80ms, border-color 80ms;
  }

  .full-pm-act:hover { background: var(--surface-hover); color: var(--text-primary); }

  .full-pm-act-primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }

  .full-pm-act-primary:hover { background: var(--accent-hover); border-color: var(--accent-hover); color: #fff; }

  .full-pm-act-danger { color: var(--color-error, #dc2626); }

  .full-pm-act-danger:hover {
    background: color-mix(in srgb, var(--color-error, #dc2626) 8%, var(--surface));
    border-color: color-mix(in srgb, var(--color-error, #dc2626) 40%, var(--border));
  }

</style>
