<script>
  let clip = $state({
    name: 'tiny_av.mp4',
    posX: 0, posY: 0,
    scaleX: 1, scaleY: 1,
    rotation: 0, opacity: 100,
    blendMode: 'Normal',
    speed: 100,
    reverse: false,
    originalDuration: 6.05,
    transIn:  { type: 'none' },
    transOut: { type: 'none' },
  });

  const duration = $derived((clip.originalDuration / (clip.speed / 100)).toFixed(2));

  let collapsed = $state({
    transform:   false,
    composite:   false,
    speed:       false,
    transitions: false,
    effects:     false,
  });

  const transformFields = [
    { label: 'Position X', key: 'posX',     step: 1    },
    { label: 'Position Y', key: 'posY',     step: 1    },
    { label: 'Scale X',    key: 'scaleX',   step: 0.01 },
    { label: 'Scale Y',    key: 'scaleY',   step: 0.01 },
    { label: 'Rotation',   key: 'rotation', step: 0.1  },
    { label: 'Opacity',    key: 'opacity',  step: 1    },
  ];

  const blendModes = ['Normal', 'Multiply', 'Screen', 'Overlay', 'Add', 'Darken', 'Lighten'];

  const transTypes = [
    { value: 'none',           label: 'Cut'      },
    { value: 'cross_dissolve', label: 'Dissolve' },
    { value: 'dip_black',      label: 'Dip Black'},
    { value: 'dip_white',      label: 'Dip White'},
  ];

  function chevron(collapsed) {
    return `transform:${collapsed ? 'rotate(0deg)' : 'rotate(90deg)'}`;
  }
</script>

<div class="ir">
  <!-- Clip -->
  <div class="ir-clip">
    <p class="ir-clip-label">Clip</p>
    <p class="ir-clip-name">{clip.name}</p>
  </div>

  <!-- Transform -->
  <div class="ir-section">
    <button class="ir-sec-hd" onclick={() => collapsed.transform = !collapsed.transform}>
      <span class="ir-dot" style="background:#2a8de0"></span>
      <span class="ir-sec-title">Transform</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.transform)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.transform}
      {#each transformFields as f}
        <div class="ir-row">
          <span class="ir-lbl">{f.label}</span>
          <button class="ir-diamond">
            <svg width="8" height="8" viewBox="0 0 10 10"><polygon points="5,0 10,5 5,10 0,5" fill="none" stroke="currentColor" stroke-width="1.5"/></svg>
          </button>
          <input type="number" bind:value={clip[f.key]} step={f.step} class="ir-num" />
        </div>
      {/each}
    {/if}
  </div>

  <!-- Composite -->
  <div class="ir-section">
    <button class="ir-sec-hd" onclick={() => collapsed.composite = !collapsed.composite}>
      <span class="ir-dot" style="background:#9a6fc8"></span>
      <span class="ir-sec-title">Composite</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.composite)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.composite}
      <div class="ir-row">
        <span class="ir-lbl">Blend Mode</span>
        <select bind:value={clip.blendMode} class="ir-sel w-20">
          {#each blendModes as m}<option>{m}</option>{/each}
        </select>
      </div>
    {/if}
  </div>

  <!-- Speed -->
  <div class="ir-section">
    <button class="ir-sec-hd" onclick={() => collapsed.speed = !collapsed.speed}>
      <span class="ir-dot" style="background:#e8a020"></span>
      <span class="ir-sec-title">Speed</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.speed)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.speed}
      <div class="ir-row">
        <span class="ir-lbl">Speed %</span>
        <input type="number" bind:value={clip.speed} min="1" max="400" step="1" class="ir-num" />
      </div>
      <div class="ir-row">
        <span class="ir-lbl">Reverse</span>
        <input type="checkbox" bind:checked={clip.reverse} class="ir-check" />
      </div>
      <div class="ir-row">
        <span class="ir-lbl">Duration</span>
        <span class="ir-val">{duration}s</span>
      </div>
    {/if}
  </div>

  <!-- Transitions -->
  <div class="ir-section">
    <button class="ir-sec-hd" onclick={() => collapsed.transitions = !collapsed.transitions}>
      <span class="ir-dot" style="background:#44cc88"></span>
      <span class="ir-sec-title">Transitions</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.transitions)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.transitions}
      <div class="ir-row">
        <span class="ir-lbl">In</span>
        <select value={clip.transIn.type} onchange={(e) => clip.transIn = { ...clip.transIn, type: e.target.value }} class="ir-sel w-24">
          {#each transTypes as t}<option value={t.value}>{t.label}</option>{/each}
        </select>
      </div>
      {#if clip.transIn.type !== 'none'}
        <div class="ir-row ir-row-sub">
          <span class="ir-lbl">Duration</span>
          <input type="number" value={clip.transIn.duration ?? 0.5} min="0.1" max="5" step="0.1" oninput={(e) => clip.transIn = { ...clip.transIn, duration: +e.target.value }} class="ir-num" />
        </div>
      {/if}
      <div class="ir-row">
        <span class="ir-lbl">Out</span>
        <select value={clip.transOut.type} onchange={(e) => clip.transOut = { ...clip.transOut, type: e.target.value }} class="ir-sel w-24">
          {#each transTypes as t}<option value={t.value}>{t.label}</option>{/each}
        </select>
      </div>
      {#if clip.transOut.type !== 'none'}
        <div class="ir-row ir-row-sub">
          <span class="ir-lbl">Duration</span>
          <input type="number" value={clip.transOut.duration ?? 0.5} min="0.1" max="5" step="0.1" oninput={(e) => clip.transOut = { ...clip.transOut, duration: +e.target.value }} class="ir-num" />
        </div>
      {/if}
    {/if}
  </div>

  <!-- Effects -->
  <div class="ir-section" style="border-bottom:none">
    <button class="ir-sec-hd" onclick={() => collapsed.effects = !collapsed.effects}>
      <span class="ir-dot" style="background:#cc88ff"></span>
      <span class="ir-sec-title">Effects</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.effects)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.effects}
      <div class="ir-effects-empty">
        <p class="ir-effects-msg">No effects applied</p>
        <button class="ir-effects-add">+ Add Effect</button>
      </div>
    {/if}
  </div>
</div>

<style>
  .ir {
    --ir-bg:       #111111;
    --ir-surface:  #181818;
    --ir-raised:   #1e1e1e;
    --ir-border:   #252525;
    --ir-txt:      #d0d0d0;
    --ir-sec:      #888;
    --ir-muted:    #555;
    --ir-val:      #7aaacf;
    --ir-accent:   #2a8de0;
    background: var(--ir-bg);
    color: var(--ir-txt);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    min-height: 100%;
  }

  .ir-clip {
    padding: 8px 12px;
    border-bottom: 1px solid var(--ir-border);
    background: var(--ir-raised);
    flex-shrink: 0;
  }
  .ir-clip-label {
    font-size: 9px;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--ir-muted);
    margin: 0 0 2px;
  }
  .ir-clip-name {
    font-size: 11px;
    font-weight: 500;
    color: var(--ir-txt);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .ir-section {
    border-bottom: 1px solid var(--ir-border);
  }

  .ir-sec-hd {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 6px 10px;
    background: var(--ir-surface);
    border: none;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.1s;
  }
  .ir-sec-hd:hover { background: color-mix(in srgb, var(--ir-surface) 85%, #fff); }

  .ir-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .ir-sec-title {
    flex: 1;
    text-align: left;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--ir-sec);
  }

  .ir-chev {
    color: var(--ir-muted);
    transition: transform 0.15s;
    flex-shrink: 0;
  }

  .ir-row {
    display: flex;
    align-items: center;
    padding: 3px 12px;
    gap: 4px;
    transition: background 0.1s;
  }
  .ir-row:hover { background: rgba(255,255,255,0.03); }
  .ir-row-sub { padding-left: 20px; }

  .ir-lbl {
    flex: 1;
    font-size: 10px;
    color: var(--ir-muted);
  }

  .ir-diamond {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--ir-muted);
    opacity: 0.5;
    flex-shrink: 0;
    padding: 0;
  }
  .ir-diamond:hover { opacity: 1; color: var(--ir-val); }

  .ir-num {
    width: 56px;
    text-align: right;
    background: transparent;
    border: none;
    border-bottom: 1px solid transparent;
    color: var(--ir-val);
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    outline: none;
    padding: 0 2px;
    -moz-appearance: textfield;
  }
  .ir-num::-webkit-inner-spin-button,
  .ir-num::-webkit-outer-spin-button { -webkit-appearance: none; }
  .ir-num:focus { border-bottom-color: var(--ir-val); }

  .ir-val {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--ir-val);
    width: 56px;
    text-align: right;
  }

  .ir-sel {
    font-size: 10px;
    font-family: inherit;
    background: var(--ir-raised);
    border: 1px solid var(--ir-border);
    border-radius: 4px;
    color: var(--ir-sec);
    padding: 2px 4px;
    outline: none;
    cursor: pointer;
  }

  .ir-check {
    accent-color: var(--ir-accent);
    width: 13px;
    height: 13px;
    cursor: pointer;
  }

  .ir-effects-empty {
    padding: 14px 12px;
    text-align: center;
  }
  .ir-effects-msg {
    font-size: 10px;
    color: var(--ir-muted);
    margin: 0 0 8px;
  }
  .ir-effects-add {
    font-size: 9px;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--ir-border);
    background: none;
    color: var(--ir-accent);
    cursor: pointer;
    font-family: inherit;
    transition: background 0.1s;
  }
  .ir-effects-add:hover { background: rgba(42,141,224,0.08); }

  :global(html:not(.dark)) .ir {
    --ir-bg:      #f7f7f7;
    --ir-surface: #eeeeee;
    --ir-raised:  #f3f3f3;
    --ir-border:  #e0e0e0;
    --ir-txt:     #1a1a1a;
    --ir-sec:     #555;
    --ir-muted:   #aaa;
    --ir-val:     #1a60aa;
  }
</style>
