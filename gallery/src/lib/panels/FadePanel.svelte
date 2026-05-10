<script>
  let opts = $state({
    bitrate_mode: 'cbr',
    bitrate: 192,
    vbr_quality: 2,
    sample_rate: '44100',
    channels: 'stereo',
  });

  let collapsed = $state({
    encoding: false,
  });

  const bitrates = [64, 128, 192, 256, 320];

  const sampleRates = [
    { value: '44100', label: '44.1k' },
    { value: '48000', label: '48k'   },
  ];

  const channels = [
    { value: 'source', label: 'Source' },
    { value: 'mono',   label: 'Mono'   },
    { value: 'stereo', label: 'Stereo' },
    { value: 'joint',  label: 'Joint'  },
  ];

  function seg(active, i, total) {
    const base = 'fd-seg-btn';
    const first = i === 0 ? ' fd-seg-first' : '';
    const last  = i === total - 1 ? ' fd-seg-last' : '';
    const on    = active ? ' fd-seg-on' : '';
    return base + first + last + on;
  }

  function chevron(c) {
    return `transform:${c ? 'rotate(0deg)' : 'rotate(90deg)'}`;
  }
</script>

<div class="ir">
  <!-- Encoding -->
  <div class="ir-section" style="border-bottom:none">
    <button class="ir-sec-hd" onclick={() => collapsed.encoding = !collapsed.encoding}>
      <span class="ir-dot" style="background:#555"></span>
      <span class="ir-sec-title">MP3 Encoding</span>
      <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(collapsed.encoding)}><path d="M9 18l6-6-6-6"/></svg>
    </button>
    {#if !collapsed.encoding}
      <!-- Bitrate Mode -->
      <div class="ir-row ir-row-field">
        <span class="ir-lbl">Bitrate Mode</span>
        <div class="fd-seg">
          {#each [['cbr','CBR'],['vbr','VBR']] as [v, lbl], i}
            <button onclick={() => opts.bitrate_mode = v} class={seg(opts.bitrate_mode === v, i, 2)}>{lbl}</button>
          {/each}
        </div>
      </div>

      {#if opts.bitrate_mode === 'cbr'}
        <!-- CBR Bitrate -->
        <div class="ir-row ir-row-field ir-row-col">
          <span class="ir-lbl">Bitrate — kbps</span>
          <div class="fd-seg fd-seg-wide">
            {#each bitrates as br, i}
              <button onclick={() => opts.bitrate = br} class={seg(opts.bitrate === br, i, bitrates.length)}>{br}</button>
            {/each}
          </div>
        </div>
      {:else}
        <!-- VBR Quality -->
        <div class="ir-row ir-row-field ir-row-col">
          <span class="ir-lbl">VBR Quality — V{opts.vbr_quality}</span>
          <input
            type="range" min="0" max="9" step="1"
            bind:value={opts.vbr_quality}
            class="fd-range"
            style="--pct:{(opts.vbr_quality / 9) * 100}%"
          />
          <div class="fd-range-labels">
            <span>V0 best</span><span>V9 smallest</span>
          </div>
        </div>
      {/if}

      <!-- Sample Rate -->
      <div class="ir-row ir-row-field">
        <span class="ir-lbl">Sample Rate</span>
        <div class="fd-seg">
          {#each sampleRates as sr, i}
            <button onclick={() => opts.sample_rate = sr.value} class={seg(opts.sample_rate === sr.value, i, sampleRates.length)}>{sr.label}</button>
          {/each}
        </div>
      </div>

      <!-- Channels -->
      <div class="ir-row ir-row-field ir-row-col">
        <span class="ir-lbl">Channels</span>
        <div class="fd-seg fd-seg-wide">
          {#each channels as ch, i}
            <button onclick={() => opts.channels = ch.value} class={seg(opts.channels === ch.value, i, channels.length)}>{ch.label}</button>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  .ir {
    background: var(--surface);
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    min-height: 100%;
  }


  .ir-section { border-bottom: 1px solid var(--border); }

  .ir-sec-hd {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 6px 10px;
    background: var(--surface-raised);
    border: none;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.1s;
  }
  .ir-sec-hd:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }

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
    color: var(--text-secondary);
  }

  .ir-chev {
    color: var(--text-muted);
    transition: transform 0.15s;
    flex-shrink: 0;
  }

  .ir-row {
    display: flex;
    align-items: center;
    padding: 4px 12px;
    gap: 6px;
    transition: background 0.1s;
  }
  .ir-row:hover { background: color-mix(in srgb, var(--surface) 94%, var(--text-primary)); }

  .ir-row-field {
    padding-top: 5px;
    padding-bottom: 5px;
  }

  .ir-row-col {
    flex-direction: column;
    align-items: flex-start;
    gap: 5px;
  }

  .ir-lbl {
    flex: 1;
    font-size: 10px;
    color: var(--text-muted);
  }

  /* Segmented buttons */
  .fd-seg {
    display: flex;
    flex-shrink: 0;
  }
  .fd-seg-wide { width: 100%; }

  .fd-seg-btn {
    flex: 1;
    padding: 2px 6px;
    font-size: 9px;
    font-family: inherit;
    font-weight: 600;
    letter-spacing: 0.04em;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-left: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    white-space: nowrap;
  }
  .fd-seg-first { border-left: 1px solid var(--border); border-radius: 4px 0 0 4px; }
  .fd-seg-last  { border-radius: 0 4px 4px 0; }
  .fd-seg-btn:hover { color: var(--text-primary); background: color-mix(in srgb, var(--surface-panel) 80%, var(--text-primary)); }
  .fd-seg-on {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-panel));
    color: #fff;
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
  }
  .fd-seg-on + .fd-seg-btn { border-left-color: color-mix(in srgb, var(--accent) 40%, var(--border)); }
  :global(html:not(.dark)) .ir .fd-seg-on { color: var(--text-primary); }

  /* Range slider */
  .fd-range {
    width: 100%;
    accent-color: var(--accent);
    cursor: pointer;
  }
  .fd-range-labels {
    width: 100%;
    display: flex;
    justify-content: space-between;
    font-size: 9px;
    color: var(--text-muted);
    margin-top: -2px;
  }
</style>
