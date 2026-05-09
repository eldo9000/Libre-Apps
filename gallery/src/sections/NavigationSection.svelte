<script>
  import { TabBar, Tabs, Menu } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let activeTab1 = $state('overview');
  let activeTab2 = $state('');
  let activeTab3 = $state('');
  let menuOpen = $state(false);
  let menuAnchor = $state(null);

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

  <h2 class="group-title">TabBar</h2>
  <div class="grid">
    <Card id="TBAR-1" label="Horizontal (3 tabs)">
      <div style="width: 100%;">
        <TabBar
          tabs={tabs}
          active={activeTab1}
          onSelect={(id) => (activeTab1 = id)}
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
</style>
