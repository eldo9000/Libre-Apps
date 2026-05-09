<script>
  import FlickerInspector from './panels/FlickerInspector.svelte';
  import FadePanel from './panels/FadePanel.svelte';

  const PANELS = [
    { id: 'flicker-inspector', app: 'Flicker', label: 'Inspector', component: FlickerInspector },
    { id: 'fade-mp3',          app: 'Fade',    label: 'MP3',       component: FadePanel        },
  ];

  let open        = $state(true);
  let activePanel = $state('flicker-inspector');

  const ActivePanel = $derived(PANELS.find(p => p.id === activePanel)?.component);
</script>

<aside class="panels" class:open>
  <!-- Fold strip — always visible -->
  <div class="fold-strip">
    <button class="fold-btn" onclick={() => open = !open} title={open ? 'Collapse panels' : 'Expand panels'}>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="fold-icon" class:flipped={open}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !open}
      <span class="strip-label">App Panels</span>
    {/if}
  </div>

  <!-- Panel body — clipped when closed -->
  <div class="panel-body">
    <div class="panel-tabs">
      {#each PANELS as p}
        <button
          class="panel-tab"
          class:active={activePanel === p.id}
          onclick={() => activePanel = p.id}
        >
          <span class="tab-app">{p.app}</span>
          <span class="tab-label">{p.label}</span>
        </button>
      {/each}
    </div>
    <div class="panel-content">
      {#if ActivePanel}
        <svelte:component this={ActivePanel} />
      {/if}
    </div>
  </div>
</aside>

<style>
  .panels {
    display: flex;
    flex-direction: row;
    flex-shrink: 0;
    overflow: hidden;
    width: 28px;
    transition: width 0.18s ease;
    border-left: 1px solid #252525;
    background: #111;
  }
  .panels.open { width: 253px; }

  .fold-strip {
    width: 28px;
    flex-shrink: 0;
    border-right: 1px solid #1c1c1c;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 8px;
    gap: 10px;
  }

  .fold-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: 1px solid #2a2a2a;
    border-radius: 5px;
    cursor: pointer;
    color: #555;
    flex-shrink: 0;
    transition: color 0.1s, border-color 0.1s;
  }
  .fold-btn:hover { color: #aaa; border-color: #444; }

  .fold-icon {
    transition: transform 0.18s;
    transform: rotate(180deg);
  }
  .fold-icon.flipped { transform: rotate(0deg); }

  .strip-label {
    writing-mode: vertical-rl;
    transform: rotate(180deg);
    font-size: 9px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: #2a2a2a;
    user-select: none;
  }

  .panel-body {
    width: 224px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .panel-tabs {
    display: flex;
    border-bottom: 1px solid #1c1c1c;
    background: #161616;
    flex-shrink: 0;
  }

  .panel-tab {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    padding: 6px 10px 4px;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    gap: 1px;
    font-family: inherit;
    transition: border-color 0.1s;
  }
  .panel-tab.active { border-bottom-color: #2a8de0; }

  .tab-app {
    font-size: 8px;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: #333;
  }
  .tab-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.07em;
    text-transform: uppercase;
    color: #666;
  }
  .panel-tab.active .tab-label { color: #bbb; }
  .panel-tab:hover .tab-label { color: #999; }

  .panel-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  :global(html:not(.dark)) .panels {
    background: #f7f7f7;
    border-left-color: #e0e0e0;
  }
  :global(html:not(.dark)) .fold-strip {
    border-right-color: #e0e0e0;
  }
  :global(html:not(.dark)) .fold-btn {
    border-color: #d0d0d0;
    color: #bbb;
  }
  :global(html:not(.dark)) .fold-btn:hover {
    color: #333;
    border-color: #999;
  }
  :global(html:not(.dark)) .strip-label { color: #ccc; }
  :global(html:not(.dark)) .panel-tabs {
    background: #efefef;
    border-bottom-color: #e0e0e0;
  }
  :global(html:not(.dark)) .tab-app { color: #ccc; }
  :global(html:not(.dark)) .tab-label { color: #999; }
  :global(html:not(.dark)) .panel-tab.active .tab-label { color: #333; }
  :global(html:not(.dark)) .panel-tab:hover .tab-label { color: #555; }
</style>
