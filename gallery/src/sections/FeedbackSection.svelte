<script>
  import { ProgressBar, Dialog, Tooltip, Button, TrafficLight } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let { toaster } = $props();

  // TrafficLight
  let tlState = $state('filled');

  // ProgressBar
  let pbState = $state('default');
  const pbVariant     = $derived(pbState);
  const heroValue     = $derived(pbState === 'success' ? 100 : pbState === 'error' ? 45 : 65);
  const heroLabel     = $derived(
    pbState === 'success' ? 'Installation complete' :
    pbState === 'error'   ? 'Installation failed'   :
    'Installing components…'
  );

  // Dialog
  let dialogSm = $state(false);
  let dialogMd = $state(false);
  let dialogLg = $state(false);
</script>

<div class="section">

  <div class="group-hd">
    <h2 class="group-title">TrafficLight</h2>
    <div class="state-toggle">
      <button class:active={tlState === 'filled'}  onclick={() => tlState = 'filled'}>Filled</button>
      <button class:active={tlState === 'outline'} onclick={() => tlState = 'outline'}>Outline</button>
    </div>
  </div>
  <div class="grid">
    <Card id="TL-1" label="Traffic Light" sourceFile="common-js/src/components/TrafficLight.svelte">
      <div class="tl-row">
        {#if tlState === 'filled'}
          <TrafficLight state="gray" size={10} />
        {/if}
        <TrafficLight state="green"  size={10} variant={tlState === 'outline' ? 'outline' : undefined} />
        <TrafficLight state="yellow" size={10} variant={tlState === 'outline' ? 'outline' : undefined} />
        <TrafficLight state="red"    size={10} variant={tlState === 'outline' ? 'outline' : undefined} />
      </div>
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">ProgressBar</h2>
    <div class="state-toggle">
      <button class:active={pbState === 'default'} onclick={() => pbState = 'default'}>Default</button>
      <button class:active={pbState === 'success'} onclick={() => pbState = 'success'}>Success</button>
      <button class:active={pbState === 'error'}   onclick={() => pbState = 'error'}>Error</button>
    </div>
  </div>
  <div class="grid">
    <Card id="PB-1" label="Standard" sourceFile="common-js/src/components/ProgressBar.svelte">
      <div class="pb-wrap"><ProgressBar value={65} variant={pbVariant} /></div>
    </Card>
    <Card id="PB-5" label="Thick" sourceFile="common-js/src/components/ProgressBar.svelte">
      <div class="pb-wrap"><ProgressBar value={55} height="h-3" variant={pbVariant} /></div>
    </Card>
  </div>
  <div class="prog-hero-row">
    <Card id="PB-7" label="Hero">
      <div class="prog-hero" class:prog-success={pbState === 'success'} class:prog-error={pbState === 'error'}>
        <div class="prog-hero-track">
          <div class="prog-hero-fill" style="--pct:{heroValue}%"></div>
        </div>
        <div class="prog-hero-footer">
          <span class="prog-hero-lbl">{heroLabel}</span>
          <span class="prog-hero-pct">{heroValue}%</span>
        </div>
      </div>
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
  .section { max-width: 1125px; }

  /* Group header with state toggle */
  .group-hd {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin: 32px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }
  .group-hd .group-title { margin: 0; padding: 0; border: none; }

  .state-toggle { display: flex; gap: 2px; }
  .state-toggle button {
    font-size: 11px;
    padding: 3px 10px;
    border-radius: 5px;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 80ms, color 80ms;
  }
  .state-toggle button:hover { color: var(--text-primary); }
  .state-toggle button.active {
    background: color-mix(in srgb, var(--surface) 75%, var(--text-primary));
    color: var(--text-primary);
  }

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
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 16px;
  }

  .tl-row { display: flex; align-items: center; gap: 8px; }

  /* ProgressBar standard */
  .pb-wrap { width: 100%; padding: 0 4px; }

  /* ProgressBar hero */
  .prog-hero-row { margin-top: 16px; }
  .prog-hero { width: 100%; }

  .prog-hero-track {
    height: 20px;
    background: color-mix(in srgb, var(--surface) 60%, var(--border));
    border: 1px solid var(--border);
    border-radius: 7px;
    overflow: hidden;
    box-sizing: border-box;
    transition: border-color 200ms;
  }
  .prog-success .prog-hero-track { border-color: color-mix(in srgb, #22c55e 35%, var(--border)); }
  .prog-error   .prog-hero-track { border-color: color-mix(in srgb, #ef4444 35%, var(--border)); }

  .prog-hero-fill {
    position: relative;
    height: 100%;
    width: var(--pct);
    background: var(--accent);
    border-radius: 6px;
    overflow: hidden;
    transition: width 500ms cubic-bezier(0.4, 0, 0.2, 1), background 200ms;
  }
  .prog-success .prog-hero-fill { background: #22c55e; }
  .prog-error   .prog-hero-fill { background: #ef4444; }

  /* Scan line */
  .prog-hero-fill::after {
    content: '';
    position: absolute;
    top: 2px;
    bottom: 2px;
    left: 0;
    width: 2px;
    background: rgba(255, 255, 255, 0.6);
    border-radius: 1px;
    animation: prog-scan 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }
  .prog-success .prog-hero-fill::after,
  .prog-error   .prog-hero-fill::after { display: none; }

  @keyframes prog-scan {
    0%   { left: 0;    opacity: 0; }
    6%   { opacity: 0.8; }
    88%  { opacity: 0.5; }
    100% { left: 100%; opacity: 0; }
  }

  .prog-hero-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 7px;
  }
  .prog-hero-lbl { font-size: 11px; color: var(--text-secondary); transition: color 200ms; }
  .prog-hero-pct { font-size: 11px; font-variant-numeric: tabular-nums; color: var(--text-muted); transition: color 200ms; }
  .prog-success .prog-hero-lbl,
  .prog-success .prog-hero-pct { color: #22c55e; }
  .prog-error   .prog-hero-lbl,
  .prog-error   .prog-hero-pct { color: #ef4444; }

  .dialog-body {
    margin: 0;
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1.6;
  }
</style>
