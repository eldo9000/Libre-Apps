<script>
  import Card from '../lib/Card.svelte';
  import { Select, SegmentedControl, Input, Toggle, Slider } from '@libre/ui';

  // ── Resize state ───────────────────────────────────────────────────────────
  let panelHeight = $state(820);

  // ── Install Manager (LW-2) state ──────────────────────────────────────────
  let imTab         = $state('discover');
  let imSearch      = $state('');
  let imPanelHeight = $state(560);
  let _imDragStartY = 0;
  let _imDragStartH = 0;

  function onImResizeStart(e) {
    _imDragStartY = e.clientY;
    _imDragStartH = imPanelHeight;
    window.addEventListener('mousemove', onImResizeMove);
    window.addEventListener('mouseup', onImResizeEnd);
  }

  function onImResizeMove(e) {
    imPanelHeight = Math.max(320, _imDragStartH + e.clientY - _imDragStartY);
  }

  function onImResizeEnd() {
    window.removeEventListener('mousemove', onImResizeMove);
    window.removeEventListener('mouseup', onImResizeEnd);
  }
  let _dragStartY = 0;
  let _dragStartH = 0;

  function onResizeStart(e) {
    _dragStartY = e.clientY;
    _dragStartH = panelHeight;
    window.addEventListener('mousemove', onResizeMove);
    window.addEventListener('mouseup', onResizeEnd);
  }

  function onResizeMove(e) {
    panelHeight = Math.max(320, _dragStartH + e.clientY - _dragStartY);
  }

  function onResizeEnd() {
    window.removeEventListener('mousemove', onResizeMove);
    window.removeEventListener('mouseup', onResizeEnd);
  }

  // ── Settings panel state ───────────────────────────────────────────────────
  let activeCategory = $state('personalization');
  let searchQuery    = $state('');
  let themeMode      = $state('light');
  let fontSize       = $state('medium');
  let density        = $state('compact');
  let wallpaper      = $state('gradient-blue');

  // ── Default Apps panel state ───────────────────────────────────────────────
  let defaultBrowser    = $state('avalanche');
  let defaultFileManager = $state('shelf');
  let defaultMediaViewer = $state('prism');
  let defaultTextEditor  = $state('stack');
  let defaultEmail       = $state('thunderbird');
  let defaultTerminal    = $state('system');
  let defaultImageEditor = $state('none');
  let defaultPdfViewer   = $state('system');

  // ── Display panel state ────────────────────────────────────────────────────
  let displayRes   = $state('1920×1080');
  let displayScale = $state('100%');
  let displayHz    = $state('60 Hz');
  let nightLight   = $state(false);

  // ── Sound panel state ──────────────────────────────────────────────────────
  let outputVolume = $state(72);
  let muteOutput   = $state(false);
  let inputGain    = $state(80);
  let muteInput    = $state(false);

  // ── Network panel state ────────────────────────────────────────────────────
  let wifiEnabled     = $state(true);
  let selectedNetwork = $state('LibreWin-5G');

  const networks = [
    { id: 'LibreWin-5G',   label: 'LibreWin-5G',   strength: 4, secured: true  },
    { id: 'LibreWin-2G',   label: 'LibreWin-2G',   strength: 3, secured: true  },
    { id: 'Neighbors-Net', label: 'Neighbors-Net',  strength: 2, secured: true  },
    { id: 'OpenNet',       label: 'OpenNet',        strength: 1, secured: false },
  ];

  // ── Privacy panel state ────────────────────────────────────────────────────
  let locationEnabled = $state(false);
  let cameraEnabled   = $state(true);
  let micEnabled      = $state(true);

  const privacyApps = [
    { name: 'Shelf',  camera: false, mic: false, location: false },
    { name: 'Prism',  camera: true,  mic: false, location: false },
    { name: 'Splice', camera: true,  mic: true,  location: false },
    { name: 'Pilot',  camera: false, mic: true,  location: true  },
  ];
  let privacyAppsState = $state(privacyApps.map(a => ({ ...a })));

  // ── Updates panel state ────────────────────────────────────────────────────
  let autoCheck    = $state(true);
  let autoDownload = $state(false);

  // ── Power & Battery panel state ────────────────────────────────────────────
  let sleepAfter     = $state('10');
  let showBatteryPct = $state(true);
  let powerBtnAction = $state('sleep');
  const sleepItems = [
    { value: '2',     label: '2 minutes'  },
    { value: '5',     label: '5 minutes'  },
    { value: '10',    label: '10 minutes' },
    { value: '15',    label: '15 minutes' },
    { value: '30',    label: '30 minutes' },
    { value: 'never', label: 'Never'      },
  ];

  // ── Start Menu & Taskbar panel state ──────────────────────────────────────
  let taskbarPos  = $state('bottom');
  let taskbarSize = $state('default');
  let showClock   = $state(true);
  let showBattery = $state(true);
  let showNotifs  = $state(true);

  // ── Bluetooth panel state ──────────────────────────────────────────────────
  let bluetoothEnabled = $state(true);
  let discoverable     = $state(false);
  const btDevices = [
    { id: 'airpods',  name: 'AirPods Pro',   type: 'Audio',    connected: true  },
    { id: 'keyboard', name: 'Magic Keyboard', type: 'Keyboard', connected: true  },
    { id: 'trackpad', name: 'Trackpad',       type: 'Pointer',  connected: false },
  ];

  // ── Accounts panel state ───────────────────────────────────────────────────
  let autoLogin = $state(false);

  // ── Date & Time panel state ────────────────────────────────────────────────
  let autoTime = $state(true);
  let timezone = $state('America/Los_Angeles');
  let use24h   = $state(false);
  const timezones = [
    'America/New_York', 'America/Chicago', 'America/Denver', 'America/Los_Angeles',
    'Europe/London', 'Europe/Paris', 'Europe/Berlin',
    'Asia/Tokyo', 'Asia/Shanghai', 'Australia/Sydney',
  ];
  const tzItems = timezones.map(tz => ({ value: tz, label: tz.replace(/_/g, ' ') }));

  // ── Notifications panel state ──────────────────────────────────────────────
  let doNotDisturb = $state(false);
  let notifsOnLock = $state(false);
  let criticalOnly = $state(false);
  const notifApps = [
    { name: 'Shelf',     enabled: true  },
    { name: 'Updates',   enabled: true  },
    { name: 'Pilot',     enabled: true  },
    { name: 'Prism',     enabled: false },
    { name: 'Avalanche', enabled: false },
  ];
  let notifAppsState = $state(notifApps.map(a => ({ ...a })));

  // ── Devices panel state ────────────────────────────────────────────────────
  let mouseSpeed      = $state(50);
  let naturalScroll   = $state(true);
  let tapToClick      = $state(true);
  let twoFingerScroll = $state(true);
  let keyLayout       = $state('us');
  let keyRepeatRate   = $state('normal');
  const keyLayoutItems = [
    { value: 'us', label: 'US' },
    { value: 'gb', label: 'UK' },
    { value: 'de', label: 'DE' },
    { value: 'fr', label: 'FR' },
    { value: 'es', label: 'ES' },
  ];

  // ── Printers panel state ───────────────────────────────────────────────────
  const printers = [
    { id: 'hp-laserjet', name: 'HP LaserJet Pro', status: 'Ready',   isDefault: true  },
    { id: 'epson-photo', name: 'Epson EcoTank',   status: 'Offline', isDefault: false },
  ];
  let selectedPrinter = $state('hp-laserjet');

  // ── Sharing & Remote panel state ───────────────────────────────────────────
  let sshEnabled = $state(false);
  let sshPort    = $state('22');
  let rdpEnabled = $state(false);
  let smbEnabled = $state(false);

  // ── Backup panel state ─────────────────────────────────────────────────────
  let backupEnabled  = $state(false);
  let backupSchedule = $state('daily');
  const backupScheduleItems = [
    { value: 'hourly', label: 'Hourly' },
    { value: 'daily',  label: 'Daily'  },
    { value: 'weekly', label: 'Weekly' },
    { value: 'manual', label: 'Manual' },
  ];

  // ── Boot panel state ───────────────────────────────────────────────────────
  let grubTimeout = $state(5);
  let quietBoot   = $state(true);
  let grubDefault = $state('fedora');
  const bootEntryItems = [
    { value: 'fedora',        label: 'LibreWin OS (default)' },
    { value: 'fedora-rescue', label: 'LibreWin OS (rescue)'  },
    { value: 'windows',       label: 'Windows 11'            },
  ];

  // ── Kernel panel state ─────────────────────────────────────────────────────
  let swappiness = $state(60);
  let thp        = $state('madvise');
  const thpItems = [
    { value: 'always',  label: 'Always'  },
    { value: 'madvise', label: 'madvise' },
    { value: 'never',   label: 'Never'   },
  ];

  // ── Logs panel state ───────────────────────────────────────────────────────
  let logBoot     = $state('current');
  let logSeverity = $state('all');
  const logBootItems = [
    { value: 'current', label: 'This boot' },
    { value: 'last',    label: 'Last boot' },
    { value: 'all',     label: 'All boots' },
  ];
  const logSevItems = [
    { value: 'all',  label: 'All'   },
    { value: 'warn', label: 'Warn'  },
    { value: 'err',  label: 'Error' },
  ];
  const logEntries = [
    { time: '00:00:01.342', sev: 'info', msg: 'Reached target Basic System'                    },
    { time: '00:00:01.891', sev: 'info', msg: 'Started udev Kernel Device Manager'             },
    { time: '00:00:02.104', sev: 'info', msg: 'Started NetworkManager'                         },
    { time: '00:00:02.667', sev: 'warn', msg: 'bluetooth: firmware not found, using fallback'  },
    { time: '00:00:03.201', sev: 'info', msg: 'Reached target Network'                         },
    { time: '00:00:04.019', sev: 'info', msg: 'Started SDDM Display Manager'                   },
    { time: '00:00:05.334', sev: 'err',  msg: 'sshd: no hostkeys available — exiting'          },
    { time: '00:00:06.102', sev: 'info', msg: 'Reached target Graphical Interface'             },
  ];

  // ── Localization panel state ───────────────────────────────────────────────
  let sysLanguage = $state('en_US');
  let regionFmt   = $state('en_US');
  let inputMethod = $state('none');
  const languageItems = [
    { value: 'en_US', label: 'English (US)'   },
    { value: 'en_GB', label: 'English (UK)'   },
    { value: 'de_DE', label: 'German'          },
    { value: 'fr_FR', label: 'French'          },
    { value: 'es_ES', label: 'Spanish'         },
    { value: 'ja_JP', label: 'Japanese'        },
    { value: 'zh_CN', label: 'Chinese (Simp.)' },
  ];
  const inputMethodItems = [
    { value: 'none',   label: 'None'   },
    { value: 'ibus',   label: 'IBus'   },
    { value: 'fcitx5', label: 'Fcitx5' },
  ];

  // ── Window Management panel state ──────────────────────────────────────────
  let virtualDesktops = $state('4');
  let animSpeed       = $state('normal');
  let focusPolicy     = $state('click');
  const desktopItems  = ['1','2','3','4','6','8'].map(n => ({ value: n, label: n }));
  const animSpeedItems = [
    { value: 'none',   label: 'None'   },
    { value: 'fast',   label: 'Fast'   },
    { value: 'normal', label: 'Normal' },
    { value: 'slow',   label: 'Slow'   },
  ];
  const focusItems = [
    { value: 'click', label: 'Click' },
    { value: 'hover', label: 'Hover' },
  ];

  // ── Sidebar structure ──────────────────────────────────────────────────────
  const categoryGroups = [
    {
      group: 'Personal',
      items: [
        { id: 'personalization', label: 'Appearance',    icon: '◑' },
        { id: 'notifications',   label: 'Notifications', icon: '◎' },
      ],
    },
    {
      group: 'System',
      items: [
        { id: 'display-sound',   label: 'Display & Sound',  icon: '▣' },
        { id: 'connectivity',    label: 'Connectivity',     icon: '⊕' },
        { id: 'power',           label: 'Power',            icon: '◉' },
        { id: 'region-language', label: 'Region & Language', icon: '◷' },
      ],
    },
    {
      group: 'Accounts',
      items: [
        { id: 'accounts', label: 'Users',    icon: '○' },
        { id: 'privacy',  label: 'Security', icon: '⊗' },
      ],
    },
    {
      group: 'Hardware',
      items: [
        { id: 'devices',      label: 'Devices',  icon: '⊟' },
        { id: 'disk-manager', label: 'Storage',  icon: '◫' },
        { id: 'printers',     label: 'Printers', icon: '⊡' },
      ],
    },
    {
      group: 'Sharing',
      items: [
        { id: 'sharing', label: 'Remote & Sharing', icon: '⊘' },
        { id: 'backup',  label: 'Backup',           icon: '↻' },
      ],
    },
    {
      group: 'Software',
      items: [
        { id: 'about-updates', label: 'About & Updates', icon: 'ⓘ' },
        { id: 'apps',          label: 'Default Apps',    icon: '⊠' },
        { id: 'startup',       label: 'Services',        icon: '⊙' },
      ],
    },
  ];

  const advancedCategories = [
    { id: 'boot',          label: 'Boot',           icon: '⊞' },
    { id: 'kernel',        label: 'Kernel',         icon: '⊚' },
    { id: 'window-mgmt',   label: 'Window Mgmt',    icon: '◰' },
    { id: 'config-editor', label: 'dconf',          icon: '≡' },
    { id: 'logs',          label: 'Logs',           icon: '≋' },
    { id: 'sys-monitor',   label: 'System Monitor', icon: '◎' },
    { id: 'hw-info',       label: 'Hardware Info',  icon: '⊚' },
    { id: 'firewall',      label: 'Firewall',       icon: '⊗' },
  ];

  const filteredGroups = $derived(
    searchQuery.trim()
      ? categoryGroups
          .map(g => ({ ...g, items: g.items.filter(c => c.label.toLowerCase().includes(searchQuery.toLowerCase())) }))
          .filter(g => g.items.length > 0)
      : categoryGroups
  );

  const filteredAdvanced = $derived(
    searchQuery.trim()
      ? advancedCategories.filter(c => c.label.toLowerCase().includes(searchQuery.toLowerCase()))
      : advancedCategories
  );

  let advancedExpanded = $state(true);

  const wallpapers = [
    { id: 'gradient-blue',   label: 'Ocean',  color: 'linear-gradient(135deg,#1a3a5c,#2884c9)' },
    { id: 'gradient-purple', label: 'Dusk',   color: 'linear-gradient(135deg,#2d1b4e,#7c3aed)' },
    { id: 'gradient-slate',  label: 'Slate',  color: 'linear-gradient(135deg,#1e293b,#334155)' },
    { id: 'gradient-forest', label: 'Forest', color: 'linear-gradient(135deg,#14532d,#16a34a)' },
    { id: 'solid-dark',      label: 'Noir',   color: '#0a0a0a' },
    { id: 'solid-sand',      label: 'Sand',   color: 'linear-gradient(135deg,#78350f,#d97706)' },
  ];

</script>

<div class="section">

  <!-- LW-1: Full settings panel -->
  <Card id="LW-1" label="Settings" sourceFile="gallery/src/sections/LibreWinSection.svelte">
    <div class="lw-settings-wrap">
    <div class="lw-settings" style="height:{panelHeight}px">

      <!-- sidebar -->
      <div class="lw-sidebar">
        <div class="lw-sidebar-search">
          <Input placeholder="Search settings…" bind:value={searchQuery}>
            {#snippet icon()}
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none"
                   stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
              </svg>
            {/snippet}
          </Input>
        </div>

        <nav class="lw-nav">
          {#each filteredGroups as grp}
            <div class="lw-nav-group-label">{grp.group}</div>
            {#each grp.items as cat}
              <button
                class="lw-nav-item {activeCategory === cat.id ? 'lw-nav-active' : ''}"
                onclick={() => activeCategory = cat.id}
              >
                <span class="lw-nav-icon">{cat.icon}</span>
                <span class="lw-nav-label">{cat.label}</span>
              </button>
            {/each}
          {/each}
        </nav>

        <!-- Advanced expandable -->
        <div class="lw-nav-advanced-wrap">
          <div class="lw-nav-divider"></div>
          <button
            class="lw-nav-item lw-nav-advanced-toggle"
            onclick={() => advancedExpanded = !advancedExpanded}
          >
            <span class="lw-nav-icon">⋯</span>
            <span class="lw-nav-label">Advanced</span>
            <span class="lw-adv-chevron {advancedExpanded ? 'lw-adv-chevron-open' : ''}">›</span>
          </button>
          {#if advancedExpanded}
            <div class="lw-adv-items">
              {#each filteredAdvanced as cat}
                <button
                  class="lw-nav-item lw-nav-sub {activeCategory === cat.id ? 'lw-nav-active' : ''}"
                  onclick={() => activeCategory = cat.id}
                >
                  <span class="lw-nav-icon">{cat.icon}</span>
                  <span class="lw-nav-label">{cat.label}</span>
                </button>
              {/each}
            </div>
          {/if}
        </div>

      </div>

      <!-- pane -->
      <div class="lw-pane">

        <!-- ──────────────────────────────────────────── PANEL: personalization ── -->
        {#if activeCategory === 'personalization'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Appearance</h2>
            <p class="lw-pane-sub">Visual style applied across all Libre apps.</p>
          </div>

          <div class="lw-section-label">Theme</div>
          <div class="lw-theme-row">
            <button class="lw-theme-btn {themeMode === 'auto' ? 'lw-theme-active' : ''}" onclick={() => themeMode = 'auto'}>
              <div class="lw-theme-preview lw-preview-auto"></div>
              <span class="lw-theme-label">Auto</span>
            </button>
            <button class="lw-theme-btn {themeMode === 'light' ? 'lw-theme-active' : ''}" onclick={() => themeMode = 'light'}>
              <div class="lw-theme-preview lw-preview-light">
                <div class="lw-preview-bar"></div>
                <div class="lw-preview-content">
                  <div class="lw-preview-line" style="width:60%"></div>
                  <div class="lw-preview-line" style="width:40%"></div>
                </div>
              </div>
              <span class="lw-theme-label">Light</span>
            </button>
            <button class="lw-theme-btn {themeMode === 'dark' ? 'lw-theme-active' : ''}" onclick={() => themeMode = 'dark'}>
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

          <div class="lw-section-label" style="margin-top:20px">Wallpaper</div>
          <div class="lw-wallpaper-grid">
            {#each wallpapers as wp}
              <button class="lw-wp-btn {wallpaper === wp.id ? 'lw-wp-active' : ''}"
                onclick={() => wallpaper = wp.id} title={wp.label}>
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
              <SegmentedControl variant="filled" size="md" bind:value={fontSize}
                options={[{value:'small',label:'Small'},{value:'medium',label:'Medium'},{value:'large',label:'Large'}]} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Density</div>
                <div class="lw-field-hint">Controls padding and row height.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={density}
                options={[{value:'compact',label:'Compact'},{value:'default',label:'Default'},{value:'relaxed',label:'Relaxed'}]} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Taskbar</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Position</div></div>
              <SegmentedControl variant="filled" size="md" bind:value={taskbarPos}
                options={[{value:'bottom',label:'Bottom'},{value:'top',label:'Top'}]} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Size</div></div>
              <SegmentedControl variant="filled" size="md" bind:value={taskbarSize}
                options={[{value:'compact',label:'Compact'},{value:'default',label:'Default'},{value:'large',label:'Large'}]} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">System Tray</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Clock & Date</div></div>
              <Toggle size="lg" bind:checked={showClock} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Battery</div></div>
              <Toggle size="lg" bind:checked={showBattery} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Notifications</div></div>
              <Toggle size="lg" bind:checked={showNotifs} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: notifications ── -->
        {:else if activeCategory === 'notifications'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Notifications</h2>
            <p class="lw-pane-sub">Do Not Disturb, per-app alerts, and notification position.</p>
          </div>

          <div class="lw-section-label">Focus</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Do Not Disturb</div>
                <div class="lw-field-hint">Silence all notifications until turned off.</div>
              </div>
              <Toggle size="lg" bind:checked={doNotDisturb} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Critical Alerts Only</div>
                <div class="lw-field-hint">Allow only system-critical notifications.</div>
              </div>
              <Toggle size="lg" bind:checked={criticalOnly} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Show on Lock Screen</div></div>
              <Toggle size="lg" bind:checked={notifsOnLock} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">App Notifications</div>
          <div class="lw-field-rows">
            {#each notifAppsState as app, i}
              <div class="lw-field-row">
                <span class="lw-field-label">{app.name}</span>
                <Toggle size="lg" bind:checked={notifAppsState[i].enabled} />
              </div>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Position</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Notification Position</div></div>
              <SegmentedControl variant="filled" size="md" value="top-right"
                options={[{value:'top-right',label:'Top Right'},{value:'top-left',label:'Top Left'},{value:'bottom-right',label:'Bottom Right'}]} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: display-sound ── -->
        {:else if activeCategory === 'display-sound'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Display & Sound</h2>
            <p class="lw-pane-sub">Screen output, visual comfort, and audio settings.</p>
          </div>

          <div class="lw-section-label">Display</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Resolution</div>
                <div class="lw-field-hint">Display output size.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={displayRes}
                options={[{value:'1280×800',label:'1280×800'},{value:'1920×1080',label:'1920×1080'},{value:'2560×1440',label:'2560×1440'}]} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Scale</div>
                <div class="lw-field-hint">UI zoom level.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={displayScale}
                options={[{value:'100%',label:'100%'},{value:'125%',label:'125%'},{value:'150%',label:'150%'},{value:'200%',label:'200%'}]} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Refresh Rate</div>
                <div class="lw-field-hint">Screen update frequency.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={displayHz}
                options={[{value:'60 Hz',label:'60 Hz'},{value:'75 Hz',label:'75 Hz'},{value:'120 Hz',label:'120 Hz'}]} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Night Light</div>
                <div class="lw-field-hint">Warms screen color in the evening.</div>
              </div>
              <Toggle size="lg" bind:checked={nightLight} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Sound Output</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <Slider label="Volume" size="lg" min={0} max={100} bind:value={outputVolume} unit="%" disabled={muteOutput} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Mute</div></div>
              <Toggle size="lg" bind:checked={muteOutput} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Output Device</div></div>
              <SegmentedControl variant="filled" size="md" value="Built-in"
                options={[{value:'Built-in',label:'Built-in'},{value:'HDMI',label:'HDMI'},{value:'USB',label:'USB'}]} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Sound Input</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <Slider label="Gain" size="lg" min={0} max={100} bind:value={inputGain} unit="%" disabled={muteInput} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Mute</div></div>
              <Toggle size="lg" bind:checked={muteInput} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: connectivity ── -->
        {:else if activeCategory === 'connectivity'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Connectivity</h2>
            <p class="lw-pane-sub">Wi-Fi, Bluetooth, and paired device settings.</p>
          </div>

          <div class="lw-section-label">Wi-Fi</div>
          <div class="lw-field-rows" style="margin-bottom:20px">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Wi-Fi</div></div>
              <Toggle size="lg" bind:checked={wifiEnabled} />
            </div>
          </div>

          {#if wifiEnabled}
            <div class="lw-section-label">Available Networks</div>
            <div class="lw-network-scroll">
            <div class="lw-network-list">
              {#each networks as net}
                <button
                  class="lw-network-row {selectedNetwork === net.id ? 'lw-network-active' : ''}"
                  onclick={() => selectedNetwork = net.id}
                >
                  <span class="lw-net-signal">
                    {#each [1,2,3,4] as bar}
                      <span class="lw-net-bar {bar <= net.strength ? 'lw-net-bar-on' : ''}"></span>
                    {/each}
                  </span>
                  <span class="lw-net-name">{net.label}</span>
                  {#if !net.secured}<span class="lw-net-open">Open</span>{/if}
                  {#if selectedNetwork === net.id}<span class="lw-net-check">✓</span>{/if}
                </button>
              {/each}
            </div>
            </div>
          {/if}

          <div class="lw-section-label" style="margin-top:20px">Bluetooth</div>
          <div class="lw-field-rows" style="margin-bottom:20px">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Bluetooth</div></div>
              <Toggle size="lg" bind:checked={bluetoothEnabled} />
            </div>
            {#if bluetoothEnabled}
              <div class="lw-field-row">
                <div>
                  <div class="lw-field-label">Discoverable</div>
                  <div class="lw-field-hint">Allow other devices to find this computer.</div>
                </div>
                <Toggle size="lg" bind:checked={discoverable} />
              </div>
            {/if}
          </div>

          {#if bluetoothEnabled}
            <div class="lw-section-label">Paired Devices</div>
            <div class="lw-field-rows">
              {#each btDevices as dev}
                <div class="lw-field-row">
                  <div>
                    <div class="lw-field-label">{dev.name}</div>
                    <div class="lw-field-hint">{dev.type}</div>
                  </div>
                  <span class="lw-bt-status {dev.connected ? 'lw-bt-connected' : ''}">
                    {dev.connected ? 'Connected' : 'Not connected'}
                  </span>
                </div>
              {/each}
            </div>
          {/if}

        <!-- ──────────────────────────────────────────── PANEL: power ── -->
        {:else if activeCategory === 'power'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Power & Battery</h2>
            <p class="lw-pane-sub">Sleep, battery display, and power button behavior.</p>
          </div>

          <div class="lw-battery-status">
            <div class="lw-battery-bar"><div class="lw-battery-fill" style="width:78%"></div></div>
            <span class="lw-battery-label">78% — Charging</span>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Sleep</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Sleep After</div>
                <div class="lw-field-hint">Inactivity before screen sleeps.</div>
              </div>
              <Select items={sleepItems} bind:value={sleepAfter} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Battery</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Show Percentage</div>
                <div class="lw-field-hint">Display battery level in taskbar.</div>
              </div>
              <Toggle size="lg" bind:checked={showBatteryPct} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Power Button</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">When Pressed</div></div>
              <SegmentedControl variant="filled" size="md" bind:value={powerBtnAction}
                options={[{value:'sleep',label:'Sleep'},{value:'hibernate',label:'Hibernate'},{value:'shutdown',label:'Shut Down'}]} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: region-language ── -->
        {:else if activeCategory === 'region-language'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Region & Language</h2>
            <p class="lw-pane-sub">Date, time, language, region formats, and input methods.</p>
          </div>

          <div class="lw-section-label">Date & Time</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Set Automatically</div>
                <div class="lw-field-hint">Sync with network time servers.</div>
              </div>
              <Toggle size="lg" bind:checked={autoTime} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Time Zone</div></div>
              <Select items={tzItems} bind:value={timezone} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">24-Hour Clock</div></div>
              <Toggle size="lg" bind:checked={use24h} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Language</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">System Language</div></div>
              <Select items={languageItems} bind:value={sysLanguage} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Region & Formats</div>
                <div class="lw-field-hint">Date, number, currency, and units format.</div>
              </div>
              <Select items={languageItems} bind:value={regionFmt} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Input</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Input Method</div>
                <div class="lw-field-hint">Framework for CJK and other complex scripts.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={inputMethod}
                options={inputMethodItems} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: accounts (users) ── -->
        {:else if activeCategory === 'accounts'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Users</h2>
            <p class="lw-pane-sub">User account and login settings.</p>
          </div>

          <div class="lw-about-identity" style="margin-bottom:20px">
            <div class="lw-about-logo" style="font-size:22px">○</div>
            <div>
              <div class="lw-about-os">liveuser</div>
              <div class="lw-about-ver">Administrator</div>
            </div>
          </div>

          <div class="lw-section-label">Login</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Auto-Login</div>
                <div class="lw-field-hint">Sign in automatically at startup.</div>
              </div>
              <Toggle size="lg" bind:checked={autoLogin} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Change Password</div></div>
              <button class="lw-btn-secondary">Change…</button>
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: privacy (security) ── -->
        {:else if activeCategory === 'privacy'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Security & Privacy</h2>
            <p class="lw-pane-sub">Control what apps can access.</p>
          </div>

          <div class="lw-section-label">System Permissions</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Location</div>
                <div class="lw-field-hint">Allow apps to access your location.</div>
              </div>
              <Toggle size="lg" bind:checked={locationEnabled} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Camera</div>
                <div class="lw-field-hint">Allow apps to use the camera.</div>
              </div>
              <Toggle size="lg" bind:checked={cameraEnabled} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Microphone</div>
                <div class="lw-field-hint">Allow apps to use the microphone.</div>
              </div>
              <Toggle size="lg" bind:checked={micEnabled} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">App Access</div>
          <div class="lw-field-rows">
            <div class="lw-field-row lw-privacy-header">
              <span class="lw-field-label">App</span>
              <div class="lw-privacy-cols">
                <span class="lw-privacy-col-label">Camera</span>
                <span class="lw-privacy-col-label">Mic</span>
                <span class="lw-privacy-col-label">Location</span>
              </div>
            </div>
            {#each privacyAppsState as app, i}
              <div class="lw-field-row">
                <span class="lw-field-label">{app.name}</span>
                <div class="lw-privacy-cols">
                  <Toggle bind:checked={privacyAppsState[i].camera} />
                  <Toggle bind:checked={privacyAppsState[i].mic} />
                  <Toggle bind:checked={privacyAppsState[i].location} />
                </div>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: devices ── -->
        {:else if activeCategory === 'devices'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Devices</h2>
            <p class="lw-pane-sub">Mouse, touchpad, and keyboard settings.</p>
          </div>

          <div class="lw-section-label">Mouse</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <Slider label="Speed" size="lg" min={0} max={100} bind:value={mouseSpeed} unit="%" />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Natural Scrolling</div>
                <div class="lw-field-hint">Scroll content in the direction of finger movement.</div>
              </div>
              <Toggle size="lg" bind:checked={naturalScroll} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Touchpad</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Tap to Click</div></div>
              <Toggle size="lg" bind:checked={tapToClick} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Two-Finger Scroll</div></div>
              <Toggle size="lg" bind:checked={twoFingerScroll} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Keyboard</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Layout</div></div>
              <Select items={keyLayoutItems} bind:value={keyLayout} />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Key Repeat Rate</div></div>
              <SegmentedControl variant="filled" size="md" bind:value={keyRepeatRate}
                options={[{value:'slow',label:'Slow'},{value:'normal',label:'Normal'},{value:'fast',label:'Fast'}]} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: disk-manager (storage) ── -->
        {:else if activeCategory === 'disk-manager'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Storage</h2>
            <p class="lw-pane-sub">View, partition, and format storage devices.</p>
          </div>

          <div class="lw-disk-list">
            {#each [
              { dev: '/dev/sda', label: 'Primary SSD', size: '256 GB', type: 'NVMe', used: 60 },
              { dev: '/dev/sdb', label: 'Data Drive',  size: '1 TB',   type: 'SATA', used: 30 },
            ] as disk}
              <div class="lw-disk-card">
                <div class="lw-disk-header">
                  <span class="lw-disk-icon">◫</span>
                  <div>
                    <div class="lw-field-label">{disk.label}</div>
                    <div class="lw-field-hint">{disk.dev} · {disk.size} {disk.type}</div>
                  </div>
                  <div class="lw-disk-actions">
                    <button class="lw-btn-secondary">Partition</button>
                    <button class="lw-btn-secondary">Format</button>
                  </div>
                </div>
                <div class="lw-disk-bar-wrap">
                  <div class="lw-disk-bar"><div class="lw-disk-bar-used" style="width:{disk.used}%"></div></div>
                  <span class="lw-field-hint">{disk.used}% used</span>
                </div>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: printers ── -->
        {:else if activeCategory === 'printers'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Printers</h2>
            <p class="lw-pane-sub">Manage printers and print queues.</p>
          </div>

          <div class="lw-field-rows" style="margin-bottom:12px">
            {#each printers as printer}
              <button
                class="lw-field-row lw-printer-row {selectedPrinter === printer.id ? 'lw-printer-active' : ''}"
                onclick={() => selectedPrinter = printer.id}
              >
                <div>
                  <div class="lw-field-label">{printer.name}</div>
                  <div class="lw-field-hint">{printer.isDefault ? 'Default · ' : ''}{printer.status}</div>
                </div>
                <span class="lw-svc-status {printer.status === 'Ready' ? 'lw-svc-active' : 'lw-svc-inactive'}">
                  {printer.status}
                </span>
              </button>
            {/each}
          </div>
          <button class="lw-btn-secondary">Add Printer…</button>

        <!-- ──────────────────────────────────────────── PANEL: sharing ── -->
        {:else if activeCategory === 'sharing'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Remote & Sharing</h2>
            <p class="lw-pane-sub">SSH, remote desktop, and file sharing.</p>
          </div>

          <div class="lw-section-label">Remote Access</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">SSH Server</div>
                <div class="lw-field-hint">Allow remote terminal access via SSH.</div>
              </div>
              <Toggle size="lg" bind:checked={sshEnabled} />
            </div>
            {#if sshEnabled}
              <div class="lw-field-row">
                <div><div class="lw-field-label">SSH Port</div></div>
                <Input bind:value={sshPort} style="width:80px;text-align:right" />
              </div>
            {/if}
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Remote Desktop (RDP)</div>
                <div class="lw-field-hint">Allow remote graphical access via KRDP.</div>
              </div>
              <Toggle size="lg" bind:checked={rdpEnabled} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">File Sharing</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">SMB / Windows Sharing</div>
                <div class="lw-field-hint">Share folders over the local network.</div>
              </div>
              <Toggle size="lg" bind:checked={smbEnabled} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: backup ── -->
        {:else if activeCategory === 'backup'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Backup</h2>
            <p class="lw-pane-sub">Automatic snapshots and off-machine backup.</p>
          </div>

          <div class="lw-section-label">Snapshots</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Enable Snapshots</div>
                <div class="lw-field-hint">Automatic BTRFS / snapper snapshots.</div>
              </div>
              <Toggle size="lg" bind:checked={backupEnabled} />
            </div>
            {#if backupEnabled}
              <div class="lw-field-row">
                <div><div class="lw-field-label">Schedule</div></div>
                <SegmentedControl variant="filled" size="md" bind:value={backupSchedule}
                  options={backupScheduleItems} />
              </div>
            {/if}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Last Snapshot</div>
          <div class="lw-field-rows">
            {#each [
              ['Time',     'Today at 03:00'],
              ['Type',     'Pre-update'],
              ['Size',     '2.3 GB'],
              ['Location', '/snapshots/2026-05-17'],
            ] as [k, v]}
              <div class="lw-field-row">
                <span class="lw-field-label">{k}</span>
                <span class="lw-detail-val">{v}</span>
              </div>
            {/each}
          </div>
          <button class="lw-btn-secondary" style="margin-top:12px">Restore Snapshot…</button>

        <!-- ──────────────────────────────────────────── PANEL: about-updates ── -->
        {:else if activeCategory === 'about-updates'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">About & Updates</h2>
            <p class="lw-pane-sub">System information, version details, and update preferences.</p>
          </div>

          <div class="lw-about-identity">
            <div class="lw-about-logo">◈</div>
            <div>
              <div class="lw-about-os">LibreWin OS</div>
              <div class="lw-about-ver">Version 0.8</div>
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">System</div>
          <div class="lw-field-rows">
            {#each [
              ['Desktop', 'KDE Plasma 6'],
              ['Kernel',  'Linux 6.x'],
              ['Base',    'Fedora 43'],
              ['Build',   '2026-05-17'],
            ] as [k, v]}
              <div class="lw-field-row">
                <span class="lw-field-label">{k}</span>
                <span class="lw-detail-val">{v}</span>
              </div>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Links</div>
          <div class="lw-field-rows">
            {#each [
              ['GitHub',       'github.com/eldo9000/LibreWin-OS'],
              ['Report a Bug', 'github.com/eldo9000/LibreWin-OS/issues'],
            ] as [k, v]}
              <div class="lw-field-row">
                <span class="lw-field-label">{k}</span>
                <span class="lw-detail-val lw-detail-mono" style="font-size:11px">{v}</span>
              </div>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Updates</div>
          <div class="lw-update-status">
            <span class="lw-update-dot"></span>
            <div>
              <div class="lw-update-title">LibreWin OS is up to date</div>
              <div class="lw-update-sub">Version 0.8 · Last checked today at 12:00</div>
            </div>
            <button class="lw-btn-secondary" style="margin-left:auto">Check Now</button>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Update Preferences</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Check Automatically</div>
                <div class="lw-field-hint">Check for updates in the background.</div>
              </div>
              <Toggle size="lg" bind:checked={autoCheck} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Download in Background</div>
                <div class="lw-field-hint">Download updates without prompting.</div>
              </div>
              <Toggle size="lg" bind:checked={autoDownload} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: apps (default apps) ── -->
        {:else if activeCategory === 'apps'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Default Apps</h2>
            <p class="lw-pane-sub">Choose which app opens each type of file or link.</p>
          </div>

          <div class="lw-section-label">Internet</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Web Browser</div>
                <div class="lw-field-hint">Opens http:// and https:// links</div>
              </div>
              <Select bind:value={defaultBrowser} size="sm">
                <option value="avalanche">Avalanche</option>
                <option value="firefox">Firefox</option>
                <option value="chromium">Chromium</option>
              </Select>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Email Client</div>
                <div class="lw-field-hint">Opens mailto: links</div>
              </div>
              <Select bind:value={defaultEmail} size="sm">
                <option value="thunderbird">Thunderbird</option>
                <option value="geary">Geary</option>
                <option value="none">None</option>
              </Select>
            </div>
          </div>

          <div class="lw-section-label">Files & Media</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">File Manager</div>
                <div class="lw-field-hint">Opens folders and directories</div>
              </div>
              <Select bind:value={defaultFileManager} size="sm">
                <option value="shelf">Shelf</option>
                <option value="nautilus">Nautilus</option>
                <option value="dolphin">Dolphin</option>
              </Select>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Media Viewer</div>
                <div class="lw-field-hint">Opens images, video, and audio</div>
              </div>
              <Select bind:value={defaultMediaViewer} size="sm">
                <option value="prism">Prism</option>
                <option value="vlc">VLC</option>
                <option value="mpv">mpv</option>
              </Select>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Image Editor</div>
                <div class="lw-field-hint">Opens image files for editing</div>
              </div>
              <Select bind:value={defaultImageEditor} size="sm">
                <option value="none">None</option>
                <option value="gimp">GIMP</option>
                <option value="inkscape">Inkscape</option>
              </Select>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">PDF Viewer</div>
                <div class="lw-field-hint">Opens .pdf documents</div>
              </div>
              <Select bind:value={defaultPdfViewer} size="sm">
                <option value="system">System Default</option>
                <option value="prism">Prism</option>
                <option value="evince">Evince</option>
              </Select>
            </div>
          </div>

          <div class="lw-section-label">Text & Code</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Text Editor</div>
                <div class="lw-field-hint">Opens plain text and markdown files</div>
              </div>
              <Select bind:value={defaultTextEditor} size="sm">
                <option value="stack">Stack</option>
                <option value="gedit">gedit</option>
                <option value="kate">Kate</option>
              </Select>
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Terminal</div>
                <div class="lw-field-hint">Default terminal emulator</div>
              </div>
              <Select bind:value={defaultTerminal} size="sm">
                <option value="system">System Default</option>
                <option value="konsole">Konsole</option>
                <option value="gnome-terminal">GNOME Terminal</option>
                <option value="alacritty">Alacritty</option>
              </Select>
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: startup (services) ── -->
        {:else if activeCategory === 'startup'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Startup & Services</h2>
            <p class="lw-pane-sub">Applications and services that run at login.</p>
          </div>

          <div class="lw-section-label">Startup Apps</div>
          <div class="lw-field-rows">
            {#each [
              { name: 'Shelf',         hint: 'File manager',        enabled: true  },
              { name: 'KDE Connect',   hint: 'Phone integration',   enabled: true  },
              { name: 'Redshift',      hint: 'Night light daemon',  enabled: false },
              { name: 'Flicker Agent', hint: 'Screen capture tray', enabled: true  },
            ] as item}
              <div class="lw-field-row">
                <div>
                  <div class="lw-field-label">{item.name}</div>
                  <div class="lw-field-hint">{item.hint}</div>
                </div>
                <Toggle size="lg" checked={item.enabled} />
              </div>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">System Services</div>
          <div class="lw-field-rows">
            {#each [
              { name: 'NetworkManager', status: 'active'   },
              { name: 'sshd',           status: 'inactive' },
              { name: 'firewalld',      status: 'active'   },
              { name: 'bluetooth',      status: 'active'   },
            ] as svc}
              <div class="lw-field-row">
                <span class="lw-field-label lw-detail-mono" style="font-size:12px">{svc.name}</span>
                <span class="lw-svc-status lw-svc-{svc.status}">{svc.status}</span>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: boot ── -->
        {:else if activeCategory === 'boot'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Boot & Bootloader</h2>
            <p class="lw-pane-sub">GRUB timeout, kernel selection, and boot flags.</p>
          </div>

          <div class="lw-section-label">GRUB</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <Slider label="Timeout" size="lg" min={0} max={30} bind:value={grubTimeout} unit="s" />
            </div>
            <div class="lw-field-row">
              <div><div class="lw-field-label">Default Entry</div></div>
              <Select items={bootEntryItems} bind:value={grubDefault} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Quiet Boot</div>
                <div class="lw-field-hint">Hide kernel messages during startup.</div>
              </div>
              <Toggle size="lg" bind:checked={quietBoot} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Boot Entries</div>
          <div class="lw-field-rows">
            {#each bootEntryItems as entry}
              <div class="lw-field-row">
                <span class="lw-field-label">{entry.label}</span>
                {#if entry.value === grubDefault}
                  <span class="lw-svc-status lw-svc-active">default</span>
                {/if}
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: kernel ── -->
        {:else if activeCategory === 'kernel'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Kernel & Low-Level</h2>
            <p class="lw-pane-sub">Runtime kernel parameters and module control.</p>
          </div>

          <div class="lw-section-label">Memory</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <Slider label="Swappiness" size="lg" min={0} max={100} bind:value={swappiness} />
            </div>
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Transparent Huge Pages</div>
                <div class="lw-field-hint">Controls THP allocation policy.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={thp} options={thpItems} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">sysctl Parameters</div>
          <div class="lw-field-rows">
            {#each [
              { key: 'vm.swappiness',         val: swappiness.toString() },
              { key: 'vm.dirty_ratio',         val: '20'       },
              { key: 'net.core.rmem_max',      val: '16777216' },
              { key: 'kernel.nmi_watchdog',    val: '0'        },
              { key: 'net.ipv4.tcp_fastopen',  val: '3'        },
            ] as entry}
              <div class="lw-field-row lw-cfg-row">
                <span class="lw-detail-mono" style="font-size:11.5px;color:var(--text-secondary)">{entry.key}</span>
                <span class="lw-detail-mono lw-cfg-val">{entry.val}</span>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: window-mgmt ── -->
        {:else if activeCategory === 'window-mgmt'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Window Management</h2>
            <p class="lw-pane-sub">Virtual desktops, animations, and focus behavior.</p>
          </div>

          <div class="lw-section-label">Desktops</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div><div class="lw-field-label">Virtual Desktops</div></div>
              <SegmentedControl variant="filled" size="md" bind:value={virtualDesktops}
                options={desktopItems} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Animations</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Animation Speed</div>
                <div class="lw-field-hint">Set to None to disable all animations.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={animSpeed}
                options={animSpeedItems} />
            </div>
          </div>

          <div class="lw-section-label" style="margin-top:20px">Focus</div>
          <div class="lw-field-rows">
            <div class="lw-field-row">
              <div>
                <div class="lw-field-label">Focus Policy</div>
                <div class="lw-field-hint">How windows gain keyboard focus.</div>
              </div>
              <SegmentedControl variant="filled" size="md" bind:value={focusPolicy}
                options={focusItems} />
            </div>
          </div>

        <!-- ──────────────────────────────────────────── PANEL: config-editor (dconf) ── -->
        {:else if activeCategory === 'config-editor'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">dconf / Config Editor</h2>
            <p class="lw-pane-sub">Browse and edit system configuration keys.</p>
          </div>

          <div class="lw-cfg-breadcrumb">
            <span class="lw-cfg-crumb lw-cfg-crumb-root">org</span>
            <span class="lw-cfg-sep">›</span>
            <span class="lw-cfg-crumb lw-cfg-crumb-root">librewin</span>
            <span class="lw-cfg-sep">›</span>
            <span class="lw-cfg-crumb">desktop</span>
          </div>

          <div class="lw-field-rows" style="margin-bottom:16px">
            {#each [
              { key: 'theme',        val: "'light'"   },
              { key: 'accent-color', val: "'#2884c9'" },
              { key: 'font-size',    val: '14'         },
              { key: 'panel-height', val: '40'         },
              { key: 'show-clock',   val: 'true'       },
              { key: 'clock-format', val: "'12h'"      },
            ] as entry}
              <div class="lw-field-row lw-cfg-row">
                <span class="lw-detail-mono" style="font-size:11.5px;color:var(--text-secondary)">{entry.key}</span>
                <span class="lw-detail-mono lw-cfg-val">{entry.val}</span>
              </div>
            {/each}
          </div>
          <p class="lw-field-hint" style="padding:0 2px">Editing keys directly may affect system stability.</p>

        <!-- ──────────────────────────────────────────── PANEL: logs ── -->
        {:else if activeCategory === 'logs'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Logs</h2>
            <p class="lw-pane-sub">System journal, boot log, and crash reports.</p>
          </div>

          <div class="lw-log-toolbar">
            <SegmentedControl variant="filled" size="md" bind:value={logBoot} options={logBootItems} />
            <SegmentedControl variant="filled" size="md" bind:value={logSeverity} options={logSevItems} />
            <button class="lw-btn-secondary" style="margin-left:auto">Export Bundle…</button>
          </div>

          <div class="lw-log-list">
            {#each logEntries.filter(e => logSeverity === 'all' || e.sev === logSeverity) as entry}
              <div class="lw-log-row lw-log-{entry.sev}">
                <span class="lw-log-time">{entry.time}</span>
                <span class="lw-log-sev lw-log-sev-{entry.sev}">{entry.sev.toUpperCase()}</span>
                <span class="lw-log-msg">{entry.msg}</span>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: sys-monitor ── -->
        {:else if activeCategory === 'sys-monitor'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">System Monitor</h2>
            <p class="lw-pane-sub">Running processes and resource usage.</p>
          </div>

          <div class="lw-resource-bars">
            {#each [
              { label: 'CPU',    pct: 18, color: '#3b82f6' },
              { label: 'Memory', pct: 44, color: '#8b5cf6' },
              { label: 'Disk',   pct: 60, color: '#f59e0b' },
            ] as r}
              <div class="lw-resource-row">
                <span class="lw-resource-label">{r.label}</span>
                <div class="lw-resource-track">
                  <div class="lw-resource-fill" style="width:{r.pct}%;background:{r.color}"></div>
                </div>
                <span class="lw-resource-pct">{r.pct}%</span>
              </div>
            {/each}
          </div>

          <div class="lw-section-label" style="margin-top:20px">Processes</div>
          <div class="lw-field-rows">
            <div class="lw-field-row lw-proc-header">
              <span class="lw-field-label" style="flex:1">Name</span>
              <span class="lw-proc-col">PID</span>
              <span class="lw-proc-col">CPU</span>
              <span class="lw-proc-col">Mem</span>
            </div>
            {#each [
              { name: 'plasmashell',  pid: 1421, cpu: '3.2', mem: '148 MB' },
              { name: 'kwin_wayland', pid: 1388, cpu: '1.1', mem: '89 MB'  },
              { name: 'shelf',        pid: 2201, cpu: '0.4', mem: '62 MB'  },
              { name: 'systemd',      pid: 1,    cpu: '0.0', mem: '12 MB'  },
              { name: 'Xwayland',     pid: 1522, cpu: '0.2', mem: '44 MB'  },
            ] as p}
              <div class="lw-field-row">
                <span class="lw-field-label lw-proc-name" style="flex:1">{p.name}</span>
                <span class="lw-proc-col lw-detail-mono">{p.pid}</span>
                <span class="lw-proc-col">{p.cpu}%</span>
                <span class="lw-proc-col">{p.mem}</span>
              </div>
            {/each}
          </div>

        <!-- ──────────────────────────────────────────── PANEL: hw-info ── -->
        {:else if activeCategory === 'hw-info'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Hardware Info</h2>
            <p class="lw-pane-sub">Detected hardware components.</p>
          </div>

          {#each [
            ['Processor', [
              ['Model',  'Apple M2 (via QEMU)'],
              ['Cores',  '8 (4P + 4E)'],
              ['Speed',  '3.49 GHz'],
            ]],
            ['Graphics', [
              ['Model',  'VirtIO GPU'],
              ['VRAM',   '256 MB (shared)'],
              ['Driver', 'virtio-gpu'],
            ]],
            ['Memory', [
              ['Installed', '8 GB'],
              ['Speed',     'LPDDR5'],
              ['Slots',     '1/1'],
            ]],
            ['Storage', [
              ['/dev/sda', '256 GB NVMe SSD'],
              ['/dev/sdb', '1 TB SATA HDD'],
            ]],
          ] as [section, rows]}
            <div class="lw-section-label" style="margin-top:20px">{section}</div>
            <div class="lw-field-rows">
              {#each rows as [k, v]}
                <div class="lw-field-row">
                  <span class="lw-field-label">{k}</span>
                  <span class="lw-detail-val">{v}</span>
                </div>
              {/each}
            </div>
          {/each}

        <!-- ──────────────────────────────────────────── PANEL: firewall ── -->
        {:else if activeCategory === 'firewall'}
          <div class="lw-pane-header">
            <h2 class="lw-pane-title">Firewall</h2>
            <p class="lw-pane-sub">Incoming and outgoing connection rules.</p>
          </div>

          <div class="lw-update-status" style="margin-bottom:20px">
            <span class="lw-update-dot"></span>
            <div>
              <div class="lw-update-title">Firewall is active</div>
              <div class="lw-update-sub">Blocking unauthorized inbound connections</div>
            </div>
            <button class="lw-btn-secondary" style="margin-left:auto">Disable</button>
          </div>

          <div class="lw-section-label">Rules</div>
          <div class="lw-field-rows">
            <div class="lw-field-row lw-proc-header">
              <span class="lw-field-label" style="flex:1">Service</span>
              <span class="lw-proc-col">Port</span>
              <span class="lw-proc-col">Direction</span>
              <span class="lw-proc-col">Action</span>
            </div>
            {#each [
              { svc: 'SSH',   port: '22',  dir: 'In',  action: 'Allow' },
              { svc: 'HTTP',  port: '80',  dir: 'In',  action: 'Deny'  },
              { svc: 'HTTPS', port: '443', dir: 'In',  action: 'Allow' },
              { svc: 'DNS',   port: '53',  dir: 'Out', action: 'Allow' },
            ] as rule}
              <div class="lw-field-row">
                <span class="lw-field-label" style="flex:1">{rule.svc}</span>
                <span class="lw-proc-col lw-detail-mono">{rule.port}</span>
                <span class="lw-proc-col">{rule.dir}</span>
                <span class="lw-proc-col lw-fw-{rule.action.toLowerCase()}">{rule.action}</span>
              </div>
            {/each}
          </div>
          <button class="lw-btn-secondary" style="margin-top:12px">Add Rule…</button>

        {/if}
      </div>
    </div>
    <div class="lw-resize-handle" onmousedown={onResizeStart}></div>
    </div>
  </Card>

  <!-- LW-2: Install Manager -->
  <Card id="LW-2" label="Install Manager" sourceFile="gallery/src/sections/LibreWinSection.svelte">
    <div class="im-wrap">
      <div class="im-shell" style="height:{imPanelHeight}px">

        <!-- Top tab bar -->
        <div class="im-topbar">
          <div class="im-tabs">
            <button class="im-tab {imTab === 'library'  ? 'im-tab-active' : ''}" onclick={() => imTab = 'library'}>Library</button>
            <button class="im-tab {imTab === 'sources'  ? 'im-tab-active' : ''}" onclick={() => imTab = 'sources'}>Sources</button>
            <button class="im-tab {imTab === 'discover' ? 'im-tab-active' : ''}" onclick={() => imTab = 'discover'}>Discover</button>
          </div>
          <div class="im-search-wrap">
            <Input bind:value={imSearch} placeholder="Search packages…" size="sm" />
          </div>
        </div>

        <!-- Content pane -->
        <div class="im-pane">
          {#if imTab === 'discover'}
            <div class="im-pane-header">
              <h2 class="im-pane-title">Discover</h2>
              <p class="im-pane-sub">Explore available packages.</p>
            </div>
          {:else if imTab === 'library'}
            <div class="im-pane-header">
              <h2 class="im-pane-title">Library</h2>
              <p class="im-pane-sub">Packages installed on this system.</p>
            </div>
          {:else if imTab === 'sources'}
            <div class="im-pane-header">
              <h2 class="im-pane-title">Sources</h2>
              <p class="im-pane-sub">Manage package repositories and mirrors.</p>
            </div>
          {/if}
        </div>

      </div>
      <div class="lw-resize-handle" onmousedown={onImResizeStart}></div>
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
  .lw-settings-wrap {
    width: 700px;
    display: flex;
    flex-direction: column;
    border-radius: 8px;
    border: 1px solid var(--border);
    overflow: hidden;
  }

  .lw-settings {
    display: flex;
    overflow: hidden;
    background: var(--surface);
  }

  .lw-resize-handle {
    height: 16px;
    cursor: ns-resize;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .lw-resize-handle::after {
    content: '';
    width: 40px;
    height: 4px;
    border-radius: 2px;
    background: color-mix(in srgb, var(--border) 80%, transparent);
    transition: background 120ms;
  }

  .lw-resize-handle:hover::after { background: var(--accent); }

  /* ── Sidebar ──────────────────────────────────────────────────────────── */
  .lw-sidebar {
    width: 160px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: color-mix(in srgb, black 10%, var(--surface-raised));
    display: flex;
    flex-direction: column;
  }

  .lw-sidebar-search {
    padding: 8px 8px 6px;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .lw-nav {
    display: flex;
    flex-direction: column;
    padding: 4px 0 4px;
    flex: 1;
    overflow-y: auto;
  }

  .lw-nav-group-label {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.07em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 10px 10px 3px;
    border-top: 1px solid var(--border);
    margin-top: 6px;
  }

  .lw-nav-group-label:first-child {
    padding-top: 4px;
    border-top: none;
    margin-top: 0;
  }

  .lw-nav-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 2px 10px;
    border: none;
    border-left: 2px solid transparent;
    background: none;
    cursor: pointer;
    text-align: left;
    color: var(--text-secondary);
    font-size: 12.5px;
    font-weight: 500;
    white-space: nowrap;
    transition: color 120ms, background 120ms, border-color 120ms;
    outline: none;
  }

  .lw-nav-item:hover { color: var(--text-primary); }

  .lw-nav-active {
    border-left-color: var(--accent) !important;
    color: var(--accent) !important;
    background: color-mix(in srgb, var(--accent) 10%, var(--surface-raised)) !important;
  }

  .lw-nav-icon {
    font-size: 12px;
    width: 14px;
    text-align: center;
    flex-shrink: 0;
    opacity: 0.6;
  }

  .lw-nav-active .lw-nav-icon { opacity: 1; color: var(--accent); }

  .lw-nav-label { font-size: 12.5px; white-space: nowrap; }

  /* ── Advanced section ─────────────────────────────────────────────────── */
  .lw-nav-advanced-wrap { padding: 0 0 4px; flex-shrink: 0; }

  .lw-nav-divider {
    height: 1px;
    background: var(--border);
    margin: 4px 2px 4px;
  }

  .lw-nav-advanced-toggle { align-self: stretch; }

  .lw-adv-chevron {
    margin-left: auto;
    font-size: 13px;
    color: var(--text-muted);
    transition: transform 120ms;
    flex-shrink: 0;
  }

  .lw-adv-chevron-open { transform: rotate(90deg); }

  .lw-adv-items {
    display: flex;
    flex-direction: column;
    padding-left: 8px;
  }

  .lw-nav-sub { opacity: 0.85; }

/* ── Pane ──────────────────────────────────────────────────────────────── */
  .lw-pane {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
    display: flex;
    flex-direction: column;
  }

  .lw-pane-header { margin-bottom: 20px; }

  .lw-pane-title {
    font-size: 17px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0 0 4px;
    letter-spacing: -0.02em;
  }

  .lw-pane-sub { font-size: 12px; color: var(--text-muted); margin: 0; }

  .lw-section-label {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin-bottom: 8px;
  }

  /* ── Theme picker ──────────────────────────────────────────────────────── */
  .lw-theme-row { display: flex; gap: 12px; justify-content: center; }

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

  .lw-theme-btn:hover { border-color: color-mix(in srgb, var(--border) 40%, var(--text-muted)); }
  .lw-theme-active { border-color: var(--accent) !important; }
  .lw-theme-active .lw-theme-label { color: #fff; }

  .lw-theme-preview {
    width: 80px;
    height: 52px;
    border-radius: 5px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .lw-preview-auto  { background: linear-gradient(135deg, #f5f5f5 50%, #1a1a1a 50%); }
  .lw-preview-light { background: #f5f5f5; }
  .lw-preview-dark  { background: #1a1a1a; }
  .lw-preview-bar { height: 10px; background: #e0e0e0; flex-shrink: 0; }
  .lw-preview-bar-dark { background: #2a2a2a; }
  .lw-preview-content { display: flex; flex-direction: column; gap: 4px; padding: 6px; }
  .lw-preview-line { height: 6px; border-radius: 3px; background: #d0d0d0; }
  .lw-preview-line-dark { background: #3a3a3a; }
  .lw-theme-label { font-size: 11.5px; color: var(--text-secondary); font-weight: 500; }

  /* ── Accent ────────────────────────────────────────────────────────────── */
  .lw-accent-preview-row { display: flex; align-items: center; gap: 8px; margin-bottom: 10px; }

  .lw-accent-dot { width: 20px; height: 20px; border-radius: 50%; flex-shrink: 0; }

  .lw-accent-hex { font-size: 12px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }

  .lw-sld-stack { display: flex; flex-direction: column; gap: 14px; }

  /* ── Wallpaper ─────────────────────────────────────────────────────────── */
  .lw-wallpaper-grid { display: flex; gap: 10px; flex-wrap: wrap; justify-content: center; }

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
  .lw-wp-active { border-color: var(--accent) !important; }
  .lw-wp-active .lw-wp-label { color: #fff; }
  .lw-wp-swatch { width: 52px; height: 36px; border-radius: 4px; }
  .lw-wp-label { font-size: 10.5px; color: var(--text-muted); }

  /* ── Field rows ────────────────────────────────────────────────────────── */
  .lw-field-rows {
    display: flex;
    flex-direction: column;
  }

  .lw-field-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    background: transparent;
    border-bottom: 1px solid var(--border);
  }

  .lw-field-row:last-child { border-bottom: none; }
  .lw-field-label { font-size: 12.5px; color: var(--text-primary); font-weight: 500; }
  .lw-field-hint { font-size: 11px; color: var(--text-muted); margin-top: 2px; }




  /* ── Buttons ───────────────────────────────────────────────────────────── */
  .lw-btn-secondary {
    padding: 5px 12px;
    font-size: 12px;
    border: 1px solid var(--border);
    border-radius: 5px;
    background: var(--surface);
    color: var(--text-primary);
    cursor: pointer;
    transition: background 120ms;
    white-space: nowrap;
    flex-shrink: 0;
  }

  .lw-btn-secondary:hover { background: color-mix(in srgb, var(--surface) 90%, var(--text-primary)); }

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

  .lw-btn-danger:hover { background: color-mix(in srgb, #ef4444 16%, transparent); }

  /* ── Network ─────────────────────────────────────────────────────────────── */
  .lw-network-scroll {
    background: color-mix(in srgb, var(--surface) 50%, #000);
    border-radius: 10px;
    max-height: 240px;
    overflow-y: auto;
    padding: 4px;
  }
  .lw-network-list { display: flex; flex-direction: column; gap: 1px; }

  .lw-network-row {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 7px;
    border: none;
    background: none;
    cursor: pointer;
    text-align: left;
    transition: background 120ms;
  }

  .lw-network-row:hover { background: color-mix(in srgb, var(--surface) 94%, var(--text-primary)); }
  .lw-network-active    { background: color-mix(in srgb, var(--accent) 10%, transparent) !important; }

  .lw-net-signal { display: flex; align-items: flex-end; gap: 2px; width: 18px; height: 14px; flex-shrink: 0; }
  .lw-net-bar { width: 3px; border-radius: 1px; background: var(--border); }
  .lw-net-bar:nth-child(1) { height: 4px;  }
  .lw-net-bar:nth-child(2) { height: 7px;  }
  .lw-net-bar:nth-child(3) { height: 10px; }
  .lw-net-bar:nth-child(4) { height: 13px; }
  .lw-net-bar-on { background: var(--accent); }
  .lw-net-name  { font-size: 12.5px; color: var(--text-primary); flex: 1; }
  .lw-net-open  { font-size: 11px; color: var(--text-muted); }
  .lw-net-check { font-size: 12px; color: var(--accent); }

  /* ── Privacy ─────────────────────────────────────────────────────────────── */
  .lw-privacy-header { background: color-mix(in srgb, var(--surface) 92%, var(--text-primary)) !important; }
  .lw-privacy-cols   { display: flex; gap: 12px; align-items: center; }
  .lw-privacy-col-label { font-size: 11px; color: var(--text-muted); width: 52px; text-align: center; }

  /* ── Updates ─────────────────────────────────────────────────────────────── */
  .lw-update-status {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
  }

  .lw-update-dot   { width: 10px; height: 10px; border-radius: 50%; background: #22c55e; flex-shrink: 0; }
  .lw-update-title { font-size: 12.5px; font-weight: 500; color: var(--text-primary); }
  .lw-update-sub   { font-size: 11px; color: var(--text-muted); margin-top: 2px; }

  /* ── About ───────────────────────────────────────────────────────────────── */
  .lw-about-identity {
    display: flex;
    align-items: center;
    gap: 16px;
    padding: 16px 18px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
  }

  .lw-about-logo {
    font-size: 28px;
    color: var(--accent);
    width: 52px;
    height: 52px;
    border-radius: 12px;
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .lw-about-os  { font-size: 15px; font-weight: 600; color: var(--text-primary); letter-spacing: -0.02em; }
  .lw-about-ver { font-size: 12px; color: var(--text-muted); margin-top: 2px; }

  /* ── Power panel ─────────────────────────────────────────────────────────── */
  .lw-battery-status {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
  }

  .lw-battery-bar {
    width: 80px;
    height: 12px;
    border: 1.5px solid var(--border);
    border-radius: 3px;
    overflow: hidden;
    flex-shrink: 0;
  }

  .lw-battery-fill { height: 100%; background: #22c55e; border-radius: 2px; }
  .lw-battery-label { font-size: 12.5px; color: var(--text-primary); font-weight: 500; }

  /* ── Bluetooth ───────────────────────────────────────────────────────────── */
  .lw-bt-status    { font-size: 11.5px; color: var(--text-muted); }
  .lw-bt-connected { color: #22c55e; }

  /* ── Disk manager ─────────────────────────────────────────────────────────── */
  .lw-disk-list { display: flex; flex-direction: column; gap: 10px; }

  .lw-disk-card {
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 12px 14px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
  }

  .lw-disk-header { display: flex; align-items: center; gap: 10px; margin-bottom: 10px; }
  .lw-disk-icon { font-size: 20px; color: var(--accent); flex-shrink: 0; }
  .lw-disk-actions { display: flex; gap: 6px; margin-left: auto; }
  .lw-disk-bar-wrap { display: flex; align-items: center; gap: 10px; }
  .lw-disk-bar { flex: 1; height: 6px; border-radius: 3px; background: var(--border); overflow: hidden; }
  .lw-disk-bar-used { height: 100%; background: var(--accent); border-radius: 3px; }

  /* ── Printers ─────────────────────────────────────────────────────────────── */
  .lw-printer-row { cursor: pointer; transition: background 120ms; }
  .lw-printer-active { background: color-mix(in srgb, var(--accent) 10%, transparent) !important; }

  /* ── Services ─────────────────────────────────────────────────────────────── */
  .lw-svc-status   { font-size: 11.5px; font-weight: 500; }
  .lw-svc-active   { color: #22c55e; }
  .lw-svc-inactive { color: var(--text-muted); }

  /* ── System monitor ───────────────────────────────────────────────────────── */
  .lw-resource-bars { display: flex; flex-direction: column; gap: 8px; }
  .lw-resource-row  { display: flex; align-items: center; gap: 10px; }
  .lw-resource-label { font-size: 12px; color: var(--text-secondary); width: 52px; flex-shrink: 0; }
  .lw-resource-track { flex: 1; height: 6px; border-radius: 3px; background: var(--border); overflow: hidden; }
  .lw-resource-fill  { height: 100%; border-radius: 3px; }
  .lw-resource-pct   { font-size: 11.5px; color: var(--text-muted); width: 32px; text-align: right; font-variant-numeric: tabular-nums; }

  .lw-proc-header { background: color-mix(in srgb, var(--surface) 92%, var(--text-primary)) !important; }
  .lw-proc-col    { font-size: 11.5px; color: var(--text-muted); width: 60px; text-align: right; flex-shrink: 0; }
  .lw-proc-name   { font-size: 12px !important; font-family: 'Geist Mono', monospace; }

  /* ── Config / dconf editor ────────────────────────────────────────────────── */
  .lw-cfg-breadcrumb {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 10px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: color-mix(in srgb, var(--surface) 96%, var(--text-primary));
    margin-bottom: 12px;
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
  }

  .lw-cfg-sep         { color: var(--text-muted); }
  .lw-cfg-crumb       { color: var(--text-secondary); }
  .lw-cfg-crumb-root  { color: var(--text-muted); }
  .lw-cfg-row         { align-items: center; }
  .lw-cfg-val         { color: var(--accent) !important; font-size: 11.5px !important; }

  /* ── Logs ─────────────────────────────────────────────────────────────────── */
  .lw-log-toolbar {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
    flex-wrap: wrap;
  }

  .lw-log-list {
    display: flex;
    flex-direction: column;
    gap: 1px;
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
    font-family: 'Geist Mono', monospace;
  }

  .lw-log-row {
    display: flex;
    align-items: baseline;
    gap: 10px;
    padding: 7px 12px;
    background: color-mix(in srgb, var(--surface) 97%, var(--text-primary));
    border-bottom: 1px solid var(--border);
  }

  .lw-log-row:last-child { border-bottom: none; }
  .lw-log-time { font-size: 10.5px; color: var(--text-muted); flex-shrink: 0; font-variant-numeric: tabular-nums; }
  .lw-log-sev  { font-size: 10px; font-weight: 700; letter-spacing: 0.04em; width: 36px; flex-shrink: 0; }
  .lw-log-sev-info { color: var(--text-muted); }
  .lw-log-sev-warn { color: #f59e0b; }
  .lw-log-sev-err  { color: #ef4444; }
  .lw-log-msg  { font-size: 11.5px; color: var(--text-primary); flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .lw-log-warn { background: color-mix(in srgb, #f59e0b 5%, var(--surface)); }
  .lw-log-err  { background: color-mix(in srgb, #ef4444 5%, var(--surface)); }

  /* ── Firewall ─────────────────────────────────────────────────────────────── */
  .lw-fw-allow { color: #22c55e; font-weight: 500; }
  .lw-fw-deny  { color: #ef4444; font-weight: 500; }

  /* ── Install Manager (LW-2) ──────────────────────────────────────────────── */
  .im-wrap {
    width: 700px;
    border-radius: 8px;
    border: 1px solid var(--border);
    overflow: hidden;
  }

  .im-shell {
    display: flex;
    flex-direction: column;
    background: var(--surface);
  }

  .im-topbar {
    display: flex;
    align-items: center;
    height: 44px;
    padding: 0 16px;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface) 60%, var(--surface-raised));
    flex-shrink: 0;
    gap: 8px;
  }

  .im-tabs {
    display: flex;
    align-items: center;
    gap: 2px;
    flex: 1;
  }

  .im-tab {
    padding: 5px 14px;
    border: none;
    border-radius: 6px;
    background: none;
    cursor: pointer;
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    font-family: inherit;
    transition: color 120ms, background 120ms;
    outline: none;
  }

  .im-tab:hover { color: var(--text-primary); background: color-mix(in srgb, var(--surface) 94%, var(--text-primary)); }

  .im-tab-active {
    color: var(--accent) !important;
    background: color-mix(in srgb, var(--accent) 10%, var(--surface-raised)) !important;
  }

  .im-search-wrap { width: 200px; flex-shrink: 0; }

  .im-pane {
    flex: 1;
    overflow-y: auto;
    padding: 24px 28px;
  }

  .im-pane-header { margin-bottom: 20px; }
  .im-pane-title {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-primary);
    letter-spacing: -0.02em;
    margin-bottom: 4px;
  }
  .im-pane-sub { font-size: 13px; color: var(--text-muted); }
</style>
