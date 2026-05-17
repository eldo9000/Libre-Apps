<script>
  import { SectionLabel, ScrollArea, TrafficLight } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  const LAYOUT_KEY = 'libre-layout';
  function _loadLayout() {
    try { return JSON.parse(localStorage.getItem(LAYOUT_KEY) ?? 'null'); } catch { return null; }
  }
  const _layout = _loadLayout();

  let sub1Open = $state(_layout?.sub1Open ?? true);
  let sub2Open = $state(_layout?.sub2Open ?? false);

  $effect(() => {
    localStorage.setItem(LAYOUT_KEY, JSON.stringify({ sub1Open, sub2Open }));
  });
</script>

<div class="section">
  <h2 class="group-title">SectionLabel</h2>
  <div class="grid">
    <Card id="SL-1" label="sm size">
      <SectionLabel size="sm">Format</SectionLabel>
    </Card>
    <Card id="SL-2" label="xs size">
      <SectionLabel size="xs">Format</SectionLabel>
    </Card>
    <Card id="SL-3" label="sm with suffix">
      <SectionLabel size="sm" suffix="KBPS">Bitrate</SectionLabel>
    </Card>
    <Card id="SL-4" label="xs with suffix">
      <SectionLabel size="xs" suffix="PX">Width</SectionLabel>
    </Card>
  </div>

  <h2 class="group-title">Settings Sub-Section</h2>
  <div class="grid">
    <Card id="SUB-1" label="Expanded">
      <div class="sub-mock section-ruled">
        <button class="sub-hd subsection-hd" onclick={() => sub1Open = !sub1Open}>
          <TrafficLight state="gray" />
          <span class="sub-title subsection-hd-title">Hotkey</span>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="none"
               stroke="currentColor" stroke-width="2.5"
               class="sub-chev"
               style="transform:{sub1Open ? 'rotate(0deg)' : 'rotate(90deg)'}">
            <path d="M9 18l6-6-6-6"/>
          </svg>
        </button>
        {#if sub1Open}
          <div class="sub-row">
            <span class="sub-lbl">Side</span>
            <span class="sub-val">Right</span>
          </div>
          <div class="sub-row">
            <span class="sub-lbl">Key</span>
            <span class="sub-val">Option ⌥</span>
          </div>
        {/if}
      </div>
    </Card>
    <Card id="SUB-2" label="Collapsed">
      <div class="sub-mock section-ruled">
        <button class="sub-hd subsection-hd" onclick={() => sub2Open = !sub2Open}>
          <TrafficLight state="gray" />
          <span class="sub-title subsection-hd-title">Hotkey</span>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="none"
               stroke="currentColor" stroke-width="2.5"
               class="sub-chev"
               style="transform:{sub2Open ? 'rotate(0deg)' : 'rotate(90deg)'}">
            <path d="M9 18l6-6-6-6"/>
          </svg>
        </button>
        {#if sub2Open}
          <div class="sub-row">
            <span class="sub-lbl">Side</span>
            <span class="sub-val">Right</span>
          </div>
        {/if}
      </div>
    </Card>
    <Card id="SUB-3" label="Static (non-collapsible)">
      <div class="sub-mock section-ruled">
        <div class="sub-hd sub-hd-static subsection-hd">
          <TrafficLight state="gray" />
          <span class="sub-title subsection-hd-title">System</span>
        </div>
        <div class="sub-row">
          <span class="sub-lbl">Launch at login</span>
          <span class="sub-val">Off</span>
        </div>
        <div class="sub-row">
          <span class="sub-lbl">Overlay</span>
          <span class="sub-val">On</span>
        </div>
        <div class="sub-row">
          <span class="sub-lbl">Indicator</span>
          <span class="sub-val">On</span>
        </div>
      </div>
    </Card>
  </div>

  <h2 class="group-title">List Backgrounds</h2>
  <div class="grid">
    <Card id="LIST-1" label="zebra-fill">
      <div class="list-mock zebra-fill">
        {#each ['Start', 'Finish', 'Cancel', 'Reset', 'Pause'] as item}
          <div class="list-row"><span class="list-lbl">{item}</span><span class="list-val">On</span></div>
        {/each}
      </div>
    </Card>
    <Card id="LIST-2" label="zebra-fill-line">
      <div class="list-mock zebra-fill-line">
        {#each ['Start', 'Finish', 'Cancel', 'Reset', 'Pause'] as item}
          <div class="list-row"><span class="list-lbl">{item}</span><span class="list-val">On</span></div>
        {/each}
      </div>
    </Card>
    <Card id="LIST-3" label="Single Separator">
      <div class="list-mock row-separator">
        {#each ['Start', 'Finish', 'Cancel', 'Reset', 'Pause'] as item}
          <div class="list-row"><span class="list-lbl">{item}</span><span class="list-val">On</span></div>
        {/each}
      </div>
    </Card>
  </div>

  <h2 class="group-title">ScrollArea</h2>
  <div class="grid">
    <Card id="SCROLL-1" label="Vertical (y)">
      <ScrollArea direction="y" class="h-32 w-48">
        {#each { length: 12 } as _, i}
          <div class="scroll-item">Item {i + 1}</div>
        {/each}
      </ScrollArea>
    </Card>
    <Card id="SCROLL-2" label="Horizontal (x)">
      <ScrollArea direction="x" class="w-48">
        <div style="display: flex; gap: 8px; padding: 4px; white-space: nowrap;">
          {#each { length: 12 } as _, i}
            <div class="scroll-chip">Item {i + 1}</div>
          {/each}
        </div>
      </ScrollArea>
    </Card>
    <Card id="SCROLL-3" label="Both axes">
      <ScrollArea direction="both" class="h-28 w-48">
        <div style="width: 420px;">
          {#each { length: 8 } as _, i}
            <div class="scroll-item" style="white-space: nowrap;">
              Long scrollable item number {i + 1} — extends past viewport width
            </div>
          {/each}
        </div>
      </ScrollArea>
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
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 16px;
  }

  .scroll-item {
    padding: 5px 10px;
    font-size: 13px;
    color: var(--text-secondary);
    border-bottom: 1px solid var(--border-subtle);
  }

  .scroll-chip {
    padding: 4px 12px;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    font-size: 13px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  /* List Backgrounds */
  .list-mock {
    width: 100%;
    background: var(--surface);
    border-top: 1px solid var(--border);
    overflow: hidden;
  }
  .list-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 5px 10px;
    font-size: 11px;
  }
  .list-lbl { color: var(--text-secondary); }
  .list-val  { color: var(--text-muted); font-variant-numeric: tabular-nums; }

  /* Settings Sub-Section ─ collapsible disclosure used in app side panels.
     Pattern source: TurboTalk settings panel (gallery/src/lib/panels/TurboTalkPanel.svelte). */
  .sub-mock {
    width: 100%;
    background: var(--surface);
    border-top: 1px solid var(--border);
    overflow: hidden;
  }
  /* Layout/bg/title styles live in tokens.css as .subsection-hd / .subsection-hd-title */
  .sub-hd { width: 100%; border: none; cursor: pointer; font-family: inherit; transition: background 0.1s; }
  .sub-hd:hover { background: color-mix(in srgb, var(--surface-raised) 35%, #000); }
  .sub-hd-static { cursor: default; pointer-events: none; }
  .sub-title { text-align: left; }
  .sub-chev {
    color: var(--text-muted);
    transition: transform 0.15s;
    flex-shrink: 0;
  }
  .sub-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 4px 12px;
    font-size: 10px;
  }
  .sub-lbl { color: var(--text-secondary); }
  .sub-val { color: var(--text-primary); font-variant-numeric: tabular-nums; }
</style>
