<script>
  import { ProgressBar, Dialog, Tooltip, Button } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let { toaster } = $props();

  let dialogSm = $state(false);
  let dialogMd = $state(false);
  let dialogLg = $state(false);
</script>

<div class="section">
  <h1 class="page-title">Feedback</h1>

  <h2 class="group-title">ProgressBar</h2>
  <div class="grid">
    <Card id="PB-1" label="Default / 65%">
      <div class="pb-wrap"><ProgressBar value={65} /></div>
    </Card>
    <Card id="PB-2" label="Success / 100%">
      <div class="pb-wrap"><ProgressBar value={100} variant="success" /></div>
    </Card>
    <Card id="PB-3" label="Error / 40%">
      <div class="pb-wrap"><ProgressBar value={40} variant="error" /></div>
    </Card>
    <Card id="PB-4" label="Default / 0%">
      <div class="pb-wrap"><ProgressBar value={0} /></div>
    </Card>
    <Card id="PB-5" label="Default / thick h-3">
      <div class="pb-wrap"><ProgressBar value={55} height="h-3" /></div>
    </Card>
    <Card id="PB-6" label="Success / thick h-3">
      <div class="pb-wrap"><ProgressBar value={80} variant="success" height="h-3" /></div>
    </Card>
  </div>

  <h2 class="group-title">Dialog</h2>
  <div class="grid">
    <Card id="DLG-1" label="Small (w-80)">
      <Button variant="secondary" onclick={() => (dialogSm = true)}>Open sm</Button>
      <Dialog bind:open={dialogSm} title="Confirm action" size="sm">
        <p class="dialog-body">This will permanently delete the file. This action cannot be undone.</p>
        {#snippet footer()}
          <Button variant="ghost" onclick={() => (dialogSm = false)}>Cancel</Button>
          <Button variant="primary" onclick={() => (dialogSm = false)}>Delete</Button>
        {/snippet}
      </Dialog>
    </Card>
    <Card id="DLG-2" label="Medium (w-480)">
      <Button variant="secondary" onclick={() => (dialogMd = true)}>Open md</Button>
      <Dialog bind:open={dialogMd} title="Export settings" size="md">
        <p class="dialog-body">Configure your export options before proceeding. Changes apply immediately on confirm.</p>
        {#snippet footer()}
          <Button variant="ghost" onclick={() => (dialogMd = false)}>Cancel</Button>
          <Button variant="primary" onclick={() => (dialogMd = false)}>Export</Button>
        {/snippet}
      </Dialog>
    </Card>
    <Card id="DLG-3" label="Large (w-640)">
      <Button variant="secondary" onclick={() => (dialogLg = true)}>Open lg</Button>
      <Dialog bind:open={dialogLg} title="Advanced preferences" size="lg">
        <p class="dialog-body">Adjust advanced settings for this operation. These changes will take effect immediately after you confirm.</p>
        {#snippet footer()}
          <Button variant="ghost" onclick={() => (dialogLg = false)}>Cancel</Button>
          <Button variant="primary" onclick={() => (dialogLg = false)}>Apply</Button>
        {/snippet}
      </Dialog>
    </Card>
  </div>

  <h2 class="group-title">Tooltip</h2>
  <div class="grid">
    <Card id="TT-1" label="Top (default)">
      <Tooltip content="Opens in a new window" placement="top">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </Card>
    <Card id="TT-2" label="Bottom">
      <Tooltip content="Opens in a new window" placement="bottom">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </Card>
    <Card id="TT-3" label="Left">
      <Tooltip content="Opens in a new window" placement="left">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </Card>
    <Card id="TT-4" label="Right">
      <Tooltip content="Opens in a new window" placement="right">
        <Button variant="secondary">Hover me</Button>
      </Tooltip>
    </Card>
  </div>

  <h2 class="group-title">Toaster</h2>
  <div class="grid">
    <Card id="TOAST-1" label="Default / info">
      <Button
        variant="secondary"
        onclick={() => toaster?.show({ message: 'File saved', detail: 'document.md' })}
      >
        Show Default
      </Button>
    </Card>
    <Card id="TOAST-2" label="Success">
      <Button
        variant="secondary"
        onclick={() => toaster?.show({ message: 'Export complete', detail: '3 files processed', variant: 'success' })}
      >
        Show Success
      </Button>
    </Card>
    <Card id="TOAST-3" label="Error">
      <Button
        variant="secondary"
        onclick={() => toaster?.show({ message: 'Export failed', detail: 'Permission denied', variant: 'error' })}
      >
        Show Error
      </Button>
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
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 16px;
  }

  .pb-wrap {
    width: 100%;
    padding: 0 4px;
  }

  .dialog-body {
    margin: 0;
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.6;
  }
</style>
