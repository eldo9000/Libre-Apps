<script>
  import { Tabs, Menu, GlobalTabs, PanelTabs, SegmentedControl } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let activeTab2 = $state('');
  let activeTab3 = $state('');
  let menuOpen = $state(false);
  let menuAnchor = $state(null);

  let compactQuery   = $state('');
  let prominentQuery = $state('');

  let globalActive = $state('overview');
  const globalTabs = [
    { id: 'overview',   label: 'Overview' },
    { id: 'components', label: 'Components' },
    { id: 'patterns',   label: 'Patterns' },
    { id: 'tokens',     label: 'Tokens' },
  ];

  let panelActive = $state('media');
  const panelTabs = [
    { id: 'media',   label: 'Media' },
    { id: 'effects', label: 'Effects' },
    { id: 'source',  label: 'Source' },
  ];

  let trackZoom = $state('expanded');
  const trackZoomOptions = [
    {
      value: 'compact',
      label: 'Compact',
      icon: `<svg width="11" height="11" viewBox="0 0 11 11" fill="currentColor">
        <rect x="0.5" y="2"    width="10" height="1.5" rx="0.75"/>
        <rect x="0.5" y="4.75" width="10" height="1.5" rx="0.75"/>
        <rect x="0.5" y="7.5"  width="10" height="1.5" rx="0.75"/>
      </svg>`,
    },
    {
      value: 'expanded',
      label: 'Expanded',
      icon: `<svg width="11" height="11" viewBox="0 0 11 11" fill="currentColor">
        <rect x="0.5" y="1"    width="10" height="2.5" rx="1"/>
        <rect x="0.5" y="4.25" width="10" height="2.5" rx="1"/>
        <rect x="0.5" y="7.5"  width="10" height="2.5" rx="1"/>
      </svg>`,
    },
    {
      value: 'stretch',
      label: 'Stretch',
      icon: `<svg width="11" height="11" viewBox="0 0 11 11">
        <line x1="0.5" y1="1"  x2="10.5" y2="1"  stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
        <rect x="0.5" y="2.5"  width="10" height="6" rx="1" fill="currentColor" opacity="0.85"/>
        <line x1="0.5" y1="10" x2="10.5" y2="10" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
      </svg>`,
    },
  ];

  const tabs = [
    { id: 'overview', label: 'Overview' },
    { id: 'details',  label: 'Details' },
    { id: 'history',  label: 'History' },
  ];

  const menuItems = [
    { label: 'Open',      onclick: () => { menuOpen = false; } },
    { label: 'Rename',    onclick: () => { menuOpen = false; } },
    { label: 'Duplicate', onclick: () => { menuOpen = false; } },
    { separator: true, label: '', onclick: () => {} },
    { label: 'Move to Trash', onclick: () => { menuOpen = false; } },
  ];

  function openMenu(e) {
    menuAnchor = e.currentTarget;
    menuOpen = true;
  }
</script>

<!-- Snippets for Tabs panels — must be defined in template scope -->
{#snippet panelOverview()}
  <p class="panel-content">Overview content area</p>
{/snippet}
{#snippet panelDetails()}
  <p class="panel-content">Details content area</p>
{/snippet}
{#snippet panelHistory()}
  <p class="panel-content">History content area</p>
{/snippet}
{#snippet vpanel1()}
  <p class="panel-content">General settings</p>
{/snippet}
{#snippet vpanel2()}
  <p class="panel-content">Network settings</p>
{/snippet}
{#snippet vpanel3()}
  <p class="panel-content">Advanced settings</p>
{/snippet}

<div class="section">
  <h1 class="page-title">Navigation</h1>

  <h2 class="group-title">Global Tabs</h2>
  <div class="grid-wide">
    <Card id="GTAB-1" label="Top-bar workspace switcher" sourceFile="common-js/src/components/GlobalTabs.svelte">
      <div class="centered">
        <GlobalTabs tabs={globalTabs} bind:active={globalActive} />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Panel Tabs</h2>
  <div class="grid">
    <Card id="PTAB-1" label="Secondary panel header" sourceFile="common-js/src/components/PanelTabs.svelte">
      <div class="panel-tabs-frame">
        <PanelTabs tabs={panelTabs} bind:active={panelActive} ariaLabel="Panel section" />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Sliding Tabs</h2>
  <div class="grid">
    <Card id="STAB-1" label="Track zoom — sliding pill (sm)" sourceFile="common-js/src/components/SegmentedControl.svelte">
      <div class="centered">
        <SegmentedControl
          options={trackZoomOptions}
          bind:value={trackZoom}
          variant="sliding"
          size="sm"
        />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Tabs (with panels)</h2>
  <div class="grid-wide">
    <Card id="TABS-1" label="Horizontal">
      <div style="width: 100%;">
        <Tabs
          bind:activeTab={activeTab2}
          tabs={[
            { id: 'overview', label: 'Overview', panel: panelOverview },
            { id: 'details',  label: 'Details',  panel: panelDetails },
            { id: 'history',  label: 'History',  panel: panelHistory },
          ]}
        />
      </div>
    </Card>
    <Card id="TABS-2" label="Vertical">
      <div style="width: 100%; min-height: 120px;">
        <Tabs
          bind:activeTab={activeTab3}
          orientation="vertical"
          tabs={[
            { id: 'general',  label: 'General',  panel: vpanel1 },
            { id: 'network',  label: 'Network',  panel: vpanel2 },
            { id: 'advanced', label: 'Advanced', panel: vpanel3 },
          ]}
        />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Search</h2>
  <div class="grid">
    <Card id="SRCH-1" label="Compact / toolbar">
      <div class="search-compact">
        <svg class="search-icon" width="13" height="13" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="11" cy="11" r="7"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          type="search"
          class="search-input"
          placeholder="Search…"
          bind:value={compactQuery}
        />
        {#if compactQuery}
          <button class="search-clear" onclick={() => (compactQuery = '')} aria-label="Clear">✕</button>
        {/if}
      </div>
    </Card>

    <Card id="SRCH-2" label="Prominent / with shortcut">
      <div class="search-prominent">
        <svg class="search-icon" width="15" height="15" viewBox="0 0 24 24" fill="none"
             stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
          <circle cx="11" cy="11" r="7"/>
          <line x1="21" y1="21" x2="16.65" y2="16.65"/>
        </svg>
        <input
          type="search"
          class="search-input"
          placeholder="Search components, tokens, docs…"
          bind:value={prominentQuery}
        />
        {#if prominentQuery}
          <button class="search-clear" onclick={() => (prominentQuery = '')} aria-label="Clear">✕</button>
        {:else}
          <kbd class="search-kbd">⌘K</kbd>
        {/if}
      </div>
    </Card>
  </div>

  <h2 class="group-title">Menu</h2>
  <div class="grid">
    <Card id="MENU-1" label="Context / Dropdown">
      <div style="position: relative;">
        <button
          bind:this={menuAnchor}
          class="menu-trigger"
          onclick={openMenu}
        >
          Actions ▾
        </button>
        <Menu bind:open={menuOpen} anchor={menuAnchor} items={menuItems} />
      </div>
    </Card>
  </div>
</div>

<style>
  .section { max-width: 900px; }

  .page-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: -0.02em;
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

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 16px;
  }

  .grid-wide {
    display: grid;
    grid-template-columns: 1fr;
    gap: 16px;
  }

  :global(.panel-content) {
    margin: 12px 16px;
    font-size: 13px;
    color: var(--text-secondary);
  }

  .menu-trigger {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 6px 14px;
    font-size: 13px;
    font-family: inherit;
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.1s;
  }

  .menu-trigger:hover {
    background: var(--surface-panel);
  }

  .centered {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px 0;
  }

  /* Mock panel-header chrome so PanelTabs reads in context (the underline
     pattern relies on the parent panel having its own bottom border). */
  .panel-tabs-frame {
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0 4px;
    display: flex;
    align-items: stretch;
    border-bottom: 1px solid var(--border);
  }

  /* ── Search ──────────────────────────────────────────────────────────── */
  .search-compact,
  .search-prominent {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    transition: border-color 0.1s, box-shadow 0.1s;
  }
  .search-compact   { padding: 4px 8px; height: 28px; }
  .search-prominent { padding: 6px 10px; height: 36px; border-radius: 8px; }

  .search-prominent:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 20%, transparent);
  }
  /* Compact search: no ring, no border-color change — just a 2px accent
     underline drawn inside the bottom edge via inset box-shadow. */
  .search-compact:focus-within {
    box-shadow: inset 0 -2px 0 0 var(--accent);
  }

  .search-icon {
    color: var(--text-muted);
    flex-shrink: 0;
  }
  .search-compact:focus-within .search-icon,
  .search-prominent:focus-within .search-icon {
    color: var(--text-primary);
  }

  .search-input {
    flex: 1;
    min-width: 0;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
  }
  .search-compact   .search-input { font-size: 12px; }
  .search-prominent .search-input { font-size: 13px; }
  .search-input::placeholder { color: var(--text-muted); }
  .search-input::-webkit-search-cancel-button { display: none; }

  .search-clear {
    background: transparent;
    border: none;
    padding: 0;
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 11px;
    cursor: pointer;
    border-radius: 3px;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .search-clear:hover {
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    color: var(--text-primary);
  }

  .search-kbd {
    font-family: 'Geist Mono', monospace;
    font-size: 10px;
    color: var(--text-muted);
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 6px;
    line-height: 1;
    flex-shrink: 0;
  }
</style>
