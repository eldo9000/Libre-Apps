<script>
  let file = $state({ name: 'audio_track.wav' });

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
  <!-- File -->
  <div class="ir-clip">
    <p class="ir-clip-label">File</p>
    <p class="ir-clip-name">{file.name}</p>
  </div>

  <!-- Encoding -->
  <div class="ir-section" style="border-bottom:none">
    <button class="ir-sec-hd" onclick={() => collapsed.encoding = !collapsed.encoding}>
      <span class="ir-dot" style="background:#e87c30"></span>
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
    --ir-bg:      #111111;
    --ir-surface: #181818;
    --ir-raised:  #1e1e1e;
    --ir-border:  #252525;
    --ir-txt:     #d0d0d0;
    --ir-sec:     #888;
    --ir-muted:   #555;
    --ir-val:     #e87c30;
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

  .ir-section { border-bottom: 1px solid var(--ir-border); }

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
    padding: 4px 12px;
    gap: 6px;
    transition: background 0.1s;
  }
  .ir-row:hover { background: rgba(255,255,255,0.03); }

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
    color: var(--ir-muted);
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
    background: var(--ir-raised);
    border: 1px solid var(--ir-border);
    border-left: none;
    color: var(--ir-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    white-space: nowrap;
  }
  .fd-seg-first { border-left: 1px solid var(--ir-border); border-radius: 4px 0 0 4px; }
  .fd-seg-last  { border-radius: 0 4px 4px 0; }
  .fd-seg-btn:hover { color: var(--ir-txt); background: color-mix(in srgb, var(--ir-raised) 80%, #fff); }
  .fd-seg-on {
    background: color-mix(in srgb, var(--ir-val) 18%, var(--ir-raised));
    color: var(--ir-val);
    border-color: color-mix(in srgb, var(--ir-val) 40%, var(--ir-border));
  }
  .fd-seg-on + .fd-seg-btn { border-left-color: color-mix(in srgb, var(--ir-val) 40%, var(--ir-border)); }

  /* Range slider */
  .fd-range {
    width: 100%;
    accent-color: var(--ir-val);
    cursor: pointer;
  }
  .fd-range-labels {
    width: 100%;
    display: flex;
    justify-content: space-between;
    font-size: 9px;
    color: var(--ir-muted);
    margin-top: -2px;
  }

  :global(html:not(.dark)) .ir {
    --ir-bg:      #f7f7f7;
    --ir-surface: #eeeeee;
    --ir-raised:  #f3f3f3;
    --ir-border:  #e0e0e0;
    --ir-txt:     #1a1a1a;
    --ir-sec:     #555;
    --ir-muted:   #aaa;
    --ir-val:     #c4621a;
  }
</style>
