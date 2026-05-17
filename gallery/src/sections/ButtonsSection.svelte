<script>
  import { Button, IconButton } from '@libre/ui';
  import Card from '../lib/Card.svelte';
</script>

<div class="section">
  <h2 class="group-title">Button</h2>
  <div class="grid">
    <Card id="BTN-1" label="Primary">
      <Button variant="primary">Save changes</Button>
    </Card>
    <Card id="BTN-2" label="Secondary">
      <Button variant="secondary">Cancel</Button>
    </Card>
    <Card id="BTN-3" label="Ghost">
      <Button variant="ghost">Learn more</Button>
    </Card>
    <Card id="BTN-4" label="Primary / Disabled">
      <Button variant="primary" disabled>Save changes</Button>
    </Card>
    <Card id="BTN-5" label="Secondary / Disabled">
      <Button variant="secondary" disabled>Cancel</Button>
    </Card>
    <Card id="BTN-6" label="Ghost / Disabled">
      <Button variant="ghost" disabled>Learn more</Button>
    </Card>
  </div>

  <h2 class="group-title">IconButton</h2>
  <div class="grid">
    <Card id="IBTN-1" label="Default">
      <IconButton label="Settings">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06
                   a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09
                   A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83
                   l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09
                   A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83
                   l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09
                   a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83
                   l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09
                   a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </IconButton>
    </Card>
    <Card id="IBTN-2" label="Danger">
      <IconButton label="Delete" variant="danger">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"/>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2"/>
        </svg>
      </IconButton>
    </Card>
    <Card id="IBTN-3" label="Default / Disabled">
      <IconButton label="Settings" disabled>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
        </svg>
      </IconButton>
    </Card>
    <Card id="IBTN-4" label="Danger / Disabled">
      <IconButton label="Delete" variant="danger" disabled>
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 6 5 6 21 6"/>
          <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6"/>
        </svg>
      </IconButton>
    </Card>
  </div>
</div>

<style>
  .section { max-width: 1125px; }

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

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 16px;
  }
</style>
