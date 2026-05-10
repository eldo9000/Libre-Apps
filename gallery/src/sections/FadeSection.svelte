<script>
  import Card from '../lib/Card.svelte';

  // Queue items
  let queueItems = $state([
    { name: 'interview_raw.mp4',  ext: 'mp4', size: '1.2 GB', status: 'done',       progress: 100, type: 'video' },
    { name: 'podcast_ep12.m4a',   ext: 'm4a', size: '84 MB',  status: 'converting', progress: 60,  type: 'audio' },
    { name: 'logo_animation.mov', ext: 'mov', size: '340 MB', status: 'queued',     progress: 0,   type: 'video' },
    { name: 'cover_art.png',      ext: 'png', size: '2.4 MB', status: 'error',      progress: 0,   type: 'image' },
  ]);

  let selectedFormat  = $state('mp4');
  let outputPath      = $state('~/Downloads/Converted');
  let videoCodec      = $state('h264');
  let videoBitrate    = $state(4);
  let videoRes        = $state('1080p');
  let audioCodec      = $state('aac');
  let audioBitrate    = $state(192);
  let audioSampleRate = $state('44100');
  let audioChannels   = $state('stereo');
  let audioMode       = $state('cbr');
  let audioVbrQuality = $state(2);
  let videoFrameRate  = $state('29.97');
  let videoTwoPass    = $state(false);
  let videoCollapsed  = $state(false);
  let audioCollapsed  = $state(false);
  let dragOver        = $state(false);

  const formats = [
    { id: 'mp4',  label: 'MP4',  badge: 'VIDEO' },
    { id: 'mp3',  label: 'MP3',  badge: 'AUDIO' },
    { id: 'webp', label: 'WebP', badge: 'IMAGE' },
    { id: 'gif',  label: 'GIF',  badge: 'IMAGE' },
    { id: 'mov',  label: 'MOV',  badge: 'VIDEO' },
    { id: 'wav',  label: 'WAV',  badge: 'AUDIO' },
  ];

  const frameRates = ['23.976', '24', '25', '29.97', '60'];

  function seg(active, i, total) {
    const base  = 'fd-seg-btn';
    const first = i === 0 ? ' fd-seg-first' : '';
    const last  = i === total - 1 ? ' fd-seg-last' : '';
    const on    = active ? ' fd-seg-on' : '';
    return base + first + last + on;
  }

  function chevron(c) {
    return `transform:${c ? 'rotate(0deg)' : 'rotate(90deg)'}`;
  }
</script>

<div class="section">

  <!-- FD-1: Drop Zone -->
  <div class="row">
    <Card id="FD-1" label="Drop Zone">
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="drop-zone"
        class:drag-over={dragOver}
        onmouseenter={() => dragOver = true}
        onmouseleave={() => dragOver = false}
        role="button"
        tabindex="0"
      >
        <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="drop-icon">
          <polyline points="16 16 12 12 8 16"/>
          <line x1="12" y1="12" x2="12" y2="21"/>
          <path d="M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3"/>
        </svg>
        <span class="drop-primary">Drop files to convert</span>
        <span class="drop-secondary">or click to browse</span>
      </div>
    </Card>
  </div>

  <!-- FD-2 + FD-3 -->
  <div class="cols">

    <!-- FD-2: Queue List -->
    <Card id="FD-2" label="Queue List">
      <div class="queue-wrap">
        <div class="queue-header">
          <span class="queue-count">4 files</span>
          <button class="queue-clear">Clear done</button>
        </div>
        {#each queueItems as item}
          <div class="queue-row">
            <div
              class="queue-icon"
              class:queue-icon-video={item.type === 'video'}
              class:queue-icon-audio={item.type === 'audio'}
              class:queue-icon-image={item.type === 'image'}
            >
              <span class="queue-ext">{item.ext.toUpperCase()}</span>
            </div>
            <div class="queue-info">
              <span class="queue-name">{item.name}</span>
              <span class="queue-size">{item.size}</span>
            </div>
            {#if item.status === 'done'}
              <span class="badge badge-done">Done</span>
            {:else if item.status === 'converting'}
              <span class="badge badge-converting">Converting…</span>
            {:else if item.status === 'queued'}
              <span class="badge badge-queued">Queued</span>
            {:else if item.status === 'error'}
              <span class="badge badge-error">Error</span>
            {/if}
            <button class="queue-remove" aria-label="Remove">×</button>
          </div>
          {#if item.status === 'converting'}
            <div class="queue-progress-track">
              <div class="queue-progress-fill" style="width:{item.progress}%"></div>
            </div>
          {/if}
        {/each}
      </div>
    </Card>

    <!-- FD-3: Format Preset Cards -->
    <Card id="FD-3" label="Format Preset Cards">
      <div class="fmt-wrap">
        <span class="fmt-title">Output Format</span>
        <div class="fmt-grid">
          {#each formats as fmt}
            <button
              class="fmt-btn"
              class:fmt-btn-selected={selectedFormat === fmt.id}
              onclick={() => selectedFormat = fmt.id}
            >
              <span class="fmt-label">{fmt.label}</span>
              <span class="fmt-badge">{fmt.badge}</span>
            </button>
          {/each}
        </div>
      </div>
    </Card>

  </div>

  <!-- FD-4: Output Path Picker -->
  <div class="row">
    <Card id="FD-4" label="Output Path Picker">
      <div class="path-row">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" class="path-icon">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
        <span class="path-text">{outputPath}</span>
        <button class="path-browse">Browse…</button>
      </div>
    </Card>
  </div>

  <!-- FD-5: Conversion Progress Row -->
  <div class="row">
    <Card id="FD-5" label="Conversion Progress Row">
      <div class="prog-wrap">
        <div class="prog-top">
          <span class="prog-name">podcast_ep12.m4a → MP3</span>
          <span class="prog-pct">60%</span>
        </div>
        <div class="prog-track">
          <div class="prog-fill"></div>
        </div>
        <span class="prog-speed">2.4× realtime — 00:01:23 remaining</span>
      </div>
    </Card>
  </div>

  <!-- FD-6 + FD-7 -->
  <div class="cols">

    <!-- FD-6: Video Codec Options -->
    <Card id="FD-6" label="Video Codec Options">
      <div class="ir">
        <div class="ir-section" style="border-bottom:none">
          <button class="ir-sec-hd" onclick={() => videoCollapsed = !videoCollapsed}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Video</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(videoCollapsed)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !videoCollapsed}
            <!-- Codec -->
            <div class="ir-row ir-row-field">
              <span class="ir-lbl">Codec</span>
              <div class="fd-seg">
                {#each [['h264','H.264'],['h265','H.265'],['vp9','VP9']] as [v, lbl], i}
                  <button onclick={() => videoCodec = v} class={seg(videoCodec === v, i, 3)}>{lbl}</button>
                {/each}
              </div>
            </div>

            <!-- Bitrate -->
            <div class="ir-row ir-row-field ir-row-col">
              <span class="ir-lbl">Bitrate — {videoBitrate} Mbps</span>
              <input
                type="range" min="1" max="20" step="1"
                bind:value={videoBitrate}
                class="fd-range"
              />
              <div class="fd-range-labels">
                <span>1 Mbps</span><span>20 Mbps</span>
              </div>
            </div>

            <!-- Resolution -->
            <div class="ir-row ir-row-field">
              <span class="ir-lbl">Resolution</span>
              <div class="fd-seg">
                {#each [['720p','720p'],['1080p','1080p'],['4k','4K']] as [v, lbl], i}
                  <button onclick={() => videoRes = v} class={seg(videoRes === v, i, 3)}>{lbl}</button>
                {/each}
              </div>
            </div>

            <!-- Frame Rate -->
            <div class="ir-row ir-row-field">
              <span class="ir-lbl">Frame Rate</span>
              <select class="fd-select" bind:value={videoFrameRate}>
                {#each frameRates as fr}
                  <option value={fr}>{fr} fps</option>
                {/each}
              </select>
            </div>

            <!-- Two-pass -->
            <div class="ir-row ir-row-field">
              <span class="ir-lbl">Two-pass encoding</span>
              <input type="checkbox" bind:checked={videoTwoPass} class="fd-check" />
            </div>
          {/if}
        </div>
      </div>
    </Card>

    <!-- FD-7: Audio Codec Options -->
    <Card id="FD-7" label="Audio Codec Options">
      <div class="ir">
        <div class="ir-section" style="border-bottom:none">
          <button class="ir-sec-hd" onclick={() => audioCollapsed = !audioCollapsed}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Audio</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(audioCollapsed)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !audioCollapsed}
            <!-- Codec -->
            <div class="ir-row ir-row-field ir-row-col">
              <span class="ir-lbl">Codec</span>
              <div class="fd-seg fd-seg-wide">
                {#each [['aac','AAC'],['mp3','MP3'],['flac','FLAC'],['opus','Opus']] as [v, lbl], i}
                  <button onclick={() => audioCodec = v} class={seg(audioCodec === v, i, 4)}>{lbl}</button>
                {/each}
              </div>
            </div>

            <!-- Bitrate Mode -->
            <div class="ir-row ir-row-field">
              <span class="ir-lbl">Bitrate Mode</span>
              <div class="fd-seg">
                {#each [['cbr','CBR'],['vbr','VBR']] as [v, lbl], i}
                  <button onclick={() => audioMode = v} class={seg(audioMode === v, i, 2)}>{lbl}</button>
                {/each}
              </div>
            </div>

            {#if audioMode === 'cbr'}
              <!-- CBR Bitrate -->
              <div class="ir-row ir-row-field ir-row-col">
                <span class="ir-lbl">Bitrate — kbps</span>
                <div class="fd-seg fd-seg-wide">
                  {#each [64, 128, 192, 256, 320] as br, i}
                    <button onclick={() => audioBitrate = br} class={seg(audioBitrate === br, i, 5)}>{br}</button>
                  {/each}
                </div>
              </div>
            {:else}
              <!-- VBR Quality -->
              <div class="ir-row ir-row-field ir-row-col">
                <span class="ir-lbl">VBR Quality — V{audioVbrQuality}</span>
                <input
                  type="range" min="0" max="9" step="1"
                  bind:value={audioVbrQuality}
                  class="fd-range"
                  style="--pct:{(audioVbrQuality / 9) * 100}%"
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
                {#each [['44100','44.1k'],['48000','48k']] as [v, lbl], i}
                  <button onclick={() => audioSampleRate = v} class={seg(audioSampleRate === v, i, 2)}>{lbl}</button>
                {/each}
              </div>
            </div>

            <!-- Channels -->
            <div class="ir-row ir-row-field ir-row-col">
              <span class="ir-lbl">Channels</span>
              <div class="fd-seg fd-seg-wide">
                {#each [['mono','Mono'],['stereo','Stereo'],['source','Source']] as [v, lbl], i}
                  <button onclick={() => audioChannels = v} class={seg(audioChannels === v, i, 3)}>{lbl}</button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>
    </Card>

  </div>

</div>

<style>
  .section { max-width: 1100px; display: flex; flex-direction: column; gap: 20px; }
  .row  { width: 100%; }
  .cols { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── Drop Zone ─────────────────────────────────────────────────── */
  .drop-zone {
    width: 100%;
    min-height: 128px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 6px;
    border: 2px dashed var(--border);
    border-radius: 12px;
    background: color-mix(in srgb, var(--surface-raised) 60%, transparent);
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    box-sizing: border-box;
    padding: 20px;
  }
  .drop-zone:hover,
  .drop-zone.drag-over {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 6%, var(--surface-raised));
  }
  .drop-zone:hover .drop-icon,
  .drop-zone.drag-over .drop-icon {
    color: var(--accent);
  }
  .drop-icon { color: var(--text-muted); transition: color 0.15s; }
  .drop-primary  { font-size: 14px; color: var(--text-secondary); }
  .drop-secondary { font-size: 12px; color: var(--text-muted); }

  /* ── Queue List ─────────────────────────────────────────────────── */
  .queue-wrap {
    width: 380px;
    display: flex;
    flex-direction: column;
  }
  .queue-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 6px 12px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .queue-count {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
  }
  .queue-clear {
    font-size: 10px;
    color: var(--text-muted);
    background: none;
    border: none;
    cursor: pointer;
    font-family: inherit;
    padding: 2px 6px;
    border-radius: 4px;
  }
  .queue-clear:hover { color: var(--text-secondary); background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-primary)); }

  .queue-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    min-height: 60px;
    border-bottom: 1px solid var(--border-subtle);
    position: relative;
    transition: background 0.1s;
  }
  .queue-row:hover { background: color-mix(in srgb, var(--surface-raised) 90%, var(--text-primary)); }
  .queue-row:last-of-type { border-bottom: none; }
  .queue-row:hover .queue-remove { opacity: 1; }

  .queue-icon {
    width: 28px;
    height: 28px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }
  .queue-icon-video { background: color-mix(in srgb, var(--accent) 18%, var(--surface-raised)); }
  .queue-icon-audio { background: color-mix(in srgb, #4ade80 30%, var(--surface-raised)); }
  .queue-icon-image { background: color-mix(in srgb, var(--surface-panel) 80%, var(--text-primary)); }

  .queue-ext {
    font-size: 7px;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--text-secondary);
  }

  .queue-info {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .queue-name {
    font-size: 12px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .queue-size {
    font-size: 10px;
    color: var(--text-muted);
  }

  .badge {
    font-size: 10px;
    font-weight: 600;
    flex-shrink: 0;
  }
  .badge-done       { color: #4ade80; }
  .badge-converting { color: var(--accent); }
  .badge-queued     { color: var(--text-muted); }
  .badge-error      { color: color-mix(in srgb, #f87171 80%, var(--text-primary)); }

  .queue-remove {
    font-size: 14px;
    color: var(--text-muted);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0 2px;
    line-height: 1;
    opacity: 0;
    transition: opacity 0.1s, color 0.1s;
    flex-shrink: 0;
  }
  .queue-remove:hover { color: var(--text-primary); }

  .queue-progress-track {
    width: 100%;
    height: 3px;
    background: var(--border);
    margin: 0;
  }
  .queue-progress-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    transition: width 0.3s;
  }

  /* ── Format Preset Cards ────────────────────────────────────────── */
  .fmt-wrap {
    width: 280px;
    display: flex;
    flex-direction: column;
    gap: 10px;
  }
  .fmt-title {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .fmt-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }
  .fmt-btn {
    width: 100%;
    height: 52px;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 3px;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--surface-raised);
    cursor: pointer;
    font-family: inherit;
    transition: border-color 0.12s, background 0.12s;
  }
  .fmt-btn:hover {
    border-color: color-mix(in srgb, var(--accent) 50%, var(--border));
    background: color-mix(in srgb, var(--accent) 4%, var(--surface-raised));
  }
  .fmt-btn-selected {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, var(--surface-raised));
  }
  .fmt-label {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }
  .fmt-badge {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .fmt-btn-selected .fmt-label { color: var(--accent); }
  .fmt-btn-selected .fmt-badge { color: color-mix(in srgb, var(--accent) 70%, var(--text-muted)); }

  /* ── Output Path ────────────────────────────────────────────────── */
  .path-row {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    box-sizing: border-box;
  }
  .path-icon { color: var(--text-muted); flex-shrink: 0; }
  .path-text {
    flex: 1;
    font-size: 12px;
    font-family: 'Geist Mono', 'Fira Code', monospace;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .path-browse {
    font-size: 11px;
    color: var(--text-secondary);
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 3px 8px;
    cursor: pointer;
    font-family: inherit;
    flex-shrink: 0;
    transition: background 0.1s;
  }
  .path-browse:hover { background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-primary)); }

  /* ── Conversion Progress ────────────────────────────────────────── */
  .prog-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 5px;
    box-sizing: border-box;
  }
  .prog-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .prog-name {
    font-size: 12px;
    color: var(--text-primary);
  }
  .prog-pct {
    font-size: 12px;
    font-weight: 600;
    color: var(--accent);
  }
  .prog-track {
    width: 100%;
    height: 4px;
    background: var(--border);
    border-radius: 2px;
    overflow: hidden;
  }
  .prog-fill {
    height: 100%;
    background: var(--accent);
    border-radius: 2px;
    animation: progress-pulse 2s ease-in-out infinite;
  }
  .prog-speed {
    font-size: 11px;
    color: var(--text-muted);
  }

  @keyframes progress-pulse {
    0%   { width: 58%; }
    50%  { width: 65%; }
    100% { width: 58%; }
  }

  /* ── Codec Panels (shared ir-* + fd-seg styles) ─────────────────── */
  .ir {
    width: 300px;
    background: var(--surface);
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    flex-direction: column;
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
  :global(html:not(.dark)) .section .fd-seg-on { color: var(--text-primary); }

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

  /* Select / Checkbox */
  .fd-select {
    font-size: 10px;
    font-family: inherit;
    color: var(--text-secondary);
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 4px;
    cursor: pointer;
  }
  .fd-check {
    accent-color: var(--accent);
    cursor: pointer;
  }
</style>
