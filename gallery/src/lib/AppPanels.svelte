<script>
  import FlickerInspector from './panels/FlickerInspector.svelte';
  import FadePanel from './panels/FadePanel.svelte';
  import TurboTalkPanel from './panels/TurboTalkPanel.svelte';
  import { focus, clearFocus } from './focus.svelte.js';

  let { collapsed = false } = $props();

  const PANELS = [
    { id: 'flicker-inspector', app: 'Flicker',    label: 'Inspector', component: FlickerInspector },
    { id: 'fade-mp3',          app: 'Fade',        label: 'MP3',       component: FadePanel        },
    { id: 'turbotalk-settings', app: 'TurboTalk',  label: 'Settings',  component: TurboTalkPanel   },
  ];

  let activePanel = $state('flicker-inspector');
  const ActivePanel = $derived(PANELS.find(p => p.id === activePanel)?.component);

  // Token inspector tooltip
  let panelAsideEl = $state(null);
  let panelContentEl = $state(null);
  let tt = $state({ visible: false, y: 0, right: 0, token: null, isCard: false });
  let lastTtEl = null;

  const TOKEN_PRIORITY = ['color', 'background', 'background-color', 'border-color', 'fill', 'accent-color'];

  const NATIVE_CONTROLS = ['checkbox', 'radio', 'range', 'color', 'file'];

  function scanToken(el) {
    if (el instanceof HTMLInputElement && NATIVE_CONTROLS.includes(el.type)) return null;

    const found = new Map();
    try {
      for (const sheet of document.styleSheets) {
        try {
          for (const rule of sheet.cssRules) {
            if (!(rule instanceof CSSStyleRule)) continue;
            try { if (!el.matches(rule.selectorText)) continue; } catch { continue; }
            const re = /([\w-]+)\s*:[^;]*var\((--[\w-]+)\)/g;
            let m;
            while ((m = re.exec(rule.cssText)) !== null) {
              if (!found.has(m[1])) found.set(m[1], m[2]);
            }
          }
        } catch { /* cross-origin */ }
      }
    } catch {}
    for (const prop of TOKEN_PRIORITY) {
      if (found.has(prop)) return found.get(prop);
    }
    return found.size > 0 ? [...found.values()][0] : null;
  }

  function onPanelMove(e) {
    const el = document.elementFromPoint(e.clientX, e.clientY);
    if (!el || !panelContentEl?.contains(el)) { tt.visible = false; lastTtEl = null; return; }
    if (el === lastTtEl) return;
    lastTtEl = el;
    const zoom = parseFloat(document.documentElement.style.zoom) || 1;
    const rect = el.getBoundingClientRect();
    const cardEl = el.closest('[data-card]');
    tt.token = cardEl ? cardEl.getAttribute('data-card') : scanToken(el);
    tt.isCard = !!cardEl;
    tt.y = (rect.top + rect.height / 2) / zoom;
    tt.right = panelAsideEl.offsetWidth;
    tt.visible = true;
  }

  function onPanelLeave() { tt.visible = false; lastTtEl = null; }
</script>

{#if tt.visible}
  <div class="panel-tt" style="top:{tt.y}px;right:{tt.right}px">
    <span class="panel-tt-bub" class:panel-tt-null={!tt.token && !tt.isCard} class:panel-tt-card={tt.isCard}>{tt.token ?? 'null'}</span>
    <span class="panel-tt-arrow"></span>
  </div>
{/if}

<aside class="panels" class:panels-collapsed={collapsed} bind:this={panelAsideEl}>
  <div class="panel-body" class:panel-body-hidden={collapsed}>
    <!-- Focus indicator — only renders when something is focused -->
    {#if focus.cards.length > 0}
      <div class="focus-bar has-focus">
        {#if focus.cards.length === 1}
          <div class="focus-info">
            <code class="focus-id">{focus.cards[0].id}</code>
            <span class="focus-label">{focus.cards[0].label}</span>
          </div>
          {#if focus.cards[0].sourceFile}
            <span class="focus-file">{focus.cards[0].sourceFile}</span>
          {/if}
          <button class="focus-clear" onclick={clearFocus} title="Clear focus">✕</button>
        {:else}
          <div class="focus-info">
            <code class="focus-id">{focus.cards.length} selected</code>
            <span class="focus-label">{focus.cards.map(c => c.id).join(', ')}</span>
          </div>
          <button class="focus-clear" onclick={clearFocus} title="Clear all">✕</button>
        {/if}
      </div>
    {/if}
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
    <div class="panel-content" bind:this={panelContentEl} onmousemove={onPanelMove} onmouseleave={onPanelLeave}>
      {#if ActivePanel}
        <svelte:component this={ActivePanel} />
      {/if}
    </div>
  </div>
</aside>

<style>
  .panel-tt {
    position: fixed;
    z-index: 9999;
    pointer-events: none;
    display: flex;
    align-items: center;
    transform: translateY(-50%);
  }
  .panel-tt-bub {
    background: var(--text-primary);
    color: var(--surface);
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    padding: 4px 8px;
    border-radius: 4px;
    white-space: nowrap;
    line-height: 1;
  }
  .panel-tt-null { color: #f5a623; }
  .panel-tt-card { color: var(--accent); }
  .panel-tt-arrow {
    width: 0;
    height: 0;
    border-top: 5px solid transparent;
    border-bottom: 5px solid transparent;
    border-left: 5px solid var(--text-primary);
  }

  .panels {
    display: flex;
    flex-direction: row;
    flex-shrink: 0;
    width: 304px;
    border-left: 1px solid rgba(255 255 255 / 0.06);
    background: #111;
    transition: width 0.18s ease;
    overflow: hidden;
  }

  .panels-collapsed {
    width: 0;
  }

  .panel-body-hidden {
    display: none;
  }

  .panel-body {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .focus-bar {
    padding: 6px 10px;
    border-bottom: 1px solid #1c1c1c;
    background: #141414;
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
    min-height: 32px;
  }

  .focus-hint {
    font-size: 9px;
    letter-spacing: 0.06em;
    color: #2a2a2a;
    text-transform: uppercase;
    flex: 1;
  }

  .focus-info {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
    min-width: 0;
  }

  .focus-id {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--text-primary);
    flex-shrink: 0;
  }

  .focus-label {
    font-size: 10px;
    color: #888;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .focus-file {
    font-size: 8px;
    font-family: 'Geist Mono', monospace;
    color: #444;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
    display: block;
  }

  .focus-clear {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 10px;
    color: #444;
    padding: 0;
    flex-shrink: 0;
    transition: color 0.1s;
  }
  .focus-clear:hover { color: #aaa; }

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
  .panel-tab.active { border-bottom-color: var(--accent); }

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
    border-left-color: rgba(0 0 0 / 0.07);
  }
  :global(html:not(.dark)) .focus-bar { background: #f0f0f0; border-bottom-color: #e0e0e0; }
  :global(html:not(.dark)) .focus-hint { color: #ccc; }
  :global(html:not(.dark)) .focus-label { color: #888; }
  :global(html:not(.dark)) .focus-file { color: #bbb; }
  :global(html:not(.dark)) .focus-clear { color: #ccc; }
  :global(html:not(.dark)) .focus-clear:hover { color: #555; }

  :global(html:not(.dark)) .panel-tabs {
    background: #efefef;
    border-bottom-color: #e0e0e0;
  }
  :global(html:not(.dark)) .tab-app { color: #ccc; }
  :global(html:not(.dark)) .tab-label { color: #999; }
  :global(html:not(.dark)) .panel-tab.active .tab-label { color: #333; }
  :global(html:not(.dark)) .panel-tab:hover .tab-label { color: #555; }
</style>
