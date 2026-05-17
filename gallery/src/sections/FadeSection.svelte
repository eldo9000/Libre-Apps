<script>
  import Card from '../lib/Card.svelte';
  import { onDestroy } from 'svelte';

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

  // ── Timeline Editor (FD-8) ─────────────────────────────────────────
  const TL_DURATION   = 45.3;
  const TL_TRIM_START = 0.08;
  const TL_TRIM_END   = 0.92;
  const TL_FADE_IN    = 0.20;
  const TL_FADE_OUT   = 0.80;

  let tlPlaying = $state(false);
  let tlTime    = $state(TL_DURATION * TL_TRIM_START);
  let tlLoop    = $state(false);
  let tlRaf     = 0;
  let tlLast    = 0;

  function tlTick(ts) {
    if (tlLast) {
      const dt = (ts - tlLast) / 1000;
      tlTime = Math.min(TL_DURATION, tlTime + dt);
      if (tlTime >= TL_DURATION * TL_TRIM_END) {
        if (tlLoop) tlTime = TL_DURATION * TL_TRIM_START;
        else { tlPlaying = false; tlTime = TL_DURATION * TL_TRIM_START; }
      }
    }
    tlLast = ts;
    if (tlPlaying) tlRaf = requestAnimationFrame(tlTick);
  }

  function tlTogglePlay() {
    if (tlPlaying) { tlPlaying = false; cancelAnimationFrame(tlRaf); }
    else { tlPlaying = true; tlLast = 0; tlRaf = requestAnimationFrame(tlTick); }
  }

  function tlStop() {
    tlPlaying = false;
    cancelAnimationFrame(tlRaf);
    tlTime = TL_DURATION * TL_TRIM_START;
  }

  function fmtTC(s) {
    const m   = Math.floor(s / 60);
    const sec = Math.floor(s % 60);
    const cs  = Math.floor((s % 1) * 100);
    return `${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}.${String(cs).padStart(2, '0')}`;
  }

  // Deterministic waveform — looks natural, stable across renders
  const waveAmps = Array.from({ length: 200 }, (_, i) => {
    const t = i / 200;
    const v = Math.abs(
      Math.sin(t * Math.PI * 7.3)  * 0.60 +
      Math.sin(t * Math.PI * 19.1) * 0.22 +
      Math.sin(t * Math.PI * 43.7) * 0.10 +
      Math.sin(t * 997)            * 0.08
    );
    return Math.min(1, Math.max(0.04, v));
  });

  onDestroy(() => cancelAnimationFrame(tlRaf));
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

  <!-- FD-8: Timeline Editor (film strip + waveform) -->
  <div class="row">
    <Card id="FD-8" label="Timeline Editor — Film Strip + Waveform" sourceFile="Fade-App/src/lib/Timeline.svelte">
      <div class="tl-wrap">

        <!-- Film strip (video frames) -->
        <div class="tl-filmstrip">
          {#each Array(20) as _, i}
            <div class="tl-frame" style="background:linear-gradient(to bottom,hsl({(i*17+185)%360},{9+(i%4)*3}%,{8+(i%5)*2}%),hsl({(i*17+205)%360},{7+(i%3)*2}%,{5+(i%4)*2}%))"></div>
          {/each}
          <!-- Playhead overlay on filmstrip -->
          <div class="tl-fs-head" style="left:{(tlTime/TL_DURATION)*100}%"></div>
        </div>

        <!-- Waveform viewport -->
        <div class="tl-viewport">
          <div class="tl-track-bg">
            <!-- Waveform SVG -->
            <svg class="tl-wave-svg" preserveAspectRatio="none" viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg">
              {#each waveAmps as amp, i}
                {@const h = Math.max(1, amp * 92)}
                {@const y = (100 - h) / 2}
                <rect x={i} y={y} width={0.85} height={h} fill="hsl({180 + i * 0.9},100%,55%)" />
              {/each}
            </svg>

            <!-- Pre-trim dim -->
            <div class="tl-dim" style="left:0;width:{TL_TRIM_START*100}%;border-radius:4px 0 0 4px"></div>
            <!-- Post-trim dim -->
            <div class="tl-dim" style="left:{TL_TRIM_END*100}%;right:0;border-radius:0 4px 4px 0"></div>

            <!-- Fade-in gradient overlay -->
            <div class="tl-fade-grad" style="left:{TL_TRIM_START*100}%;width:{(TL_FADE_IN-TL_TRIM_START)*100}%;background:linear-gradient(to right,rgba(0,0,0,0.72),transparent)"></div>
            <!-- Fade-out gradient overlay -->
            <div class="tl-fade-grad" style="left:{TL_FADE_OUT*100}%;width:{(TL_TRIM_END-TL_FADE_OUT)*100}%;background:linear-gradient(to right,transparent,rgba(0,0,0,0.72))"></div>

            <!-- Fade-in triangle handle -->
            <div class="tl-fade-handle" style="left:{TL_FADE_IN*100}%">
              <svg width="14" height="10" viewBox="0 0 14 10" xmlns="http://www.w3.org/2000/svg">
                <polygon points="7,10 0,0 14,0" fill="rgba(74,222,128,0.75)"/>
              </svg>
            </div>
            <!-- Fade-out triangle handle -->
            <div class="tl-fade-handle" style="left:{TL_FADE_OUT*100}%">
              <svg width="14" height="10" viewBox="0 0 14 10" xmlns="http://www.w3.org/2000/svg">
                <polygon points="7,10 0,0 14,0" fill="rgba(74,222,128,0.75)"/>
              </svg>
            </div>

            <!-- Playhead -->
            <div class="tl-playhead" style="left:{(tlTime/TL_DURATION)*100}%"></div>
          </div>

          <!-- Left trim handle (outside overflow-hidden so nugget can poke out) -->
          <div class="tl-trim-handle" style="left:{TL_TRIM_START*100}%">
            <div class="tl-trim-line"></div>
            <div class="tl-trim-nugget">
              <svg width="3" height="7" viewBox="0 0 3 7"><path d="M3 0 L0 3.5 L3 7 Z" fill="rgba(255,255,255,0.55)"/></svg>
              <svg width="3" height="7" viewBox="0 0 3 7"><path d="M0 0 L3 3.5 L0 7 Z" fill="rgba(255,255,255,0.55)"/></svg>
            </div>
          </div>
          <!-- Right trim handle -->
          <div class="tl-trim-handle" style="left:{TL_TRIM_END*100}%">
            <div class="tl-trim-line"></div>
            <div class="tl-trim-nugget">
              <svg width="3" height="7" viewBox="0 0 3 7"><path d="M3 0 L0 3.5 L3 7 Z" fill="rgba(255,255,255,0.55)"/></svg>
              <svg width="3" height="7" viewBox="0 0 3 7"><path d="M0 0 L3 3.5 L0 7 Z" fill="rgba(255,255,255,0.55)"/></svg>
            </div>
          </div>
        </div>

        <!-- Controls row -->
        <div class="tl-controls">
          <div class="tl-btns">
            <!-- Skip to start -->
            <button class="tl-btn" onclick={() => { tlTime = TL_DURATION * TL_TRIM_START; tlPlaying = false; cancelAnimationFrame(tlRaf); }}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><path d="M6 6h2v12H6zm3.5 6 8.5 6V6z"/></svg>
            </button>
            <!-- Play / Pause -->
            <button class="tl-btn tl-btn-play" class:tl-btn-active={tlPlaying} onclick={tlTogglePlay}>
              {#if tlPlaying}
                <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="4" width="4" height="16"/><rect x="14" y="4" width="4" height="16"/></svg>
              {:else}
                <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              {/if}
            </button>
            <!-- Stop -->
            <button class="tl-btn" onclick={tlStop}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><rect x="6" y="6" width="12" height="12"/></svg>
            </button>
            <!-- Skip to end -->
            <button class="tl-btn" onclick={() => { tlTime = TL_DURATION * TL_TRIM_END; tlPlaying = false; cancelAnimationFrame(tlRaf); }}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor"><path d="M6 18l8.5-6L6 6v12z"/><rect x="16" y="6" width="2" height="12"/></svg>
            </button>
            <!-- Loop -->
            <button class="tl-btn" class:tl-btn-active={tlLoop} onclick={() => tlLoop = !tlLoop}>
              <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M17 1l4 4-4 4"/><path d="M3 11V9a4 4 0 0 1 4-4h14"/>
                <path d="M7 23l-4-4 4-4"/><path d="M21 13v2a4 4 0 0 1-4 4H3"/>
              </svg>
            </button>
          </div>

          <div class="tl-timecodes">
            <span><span class="tl-tc-lbl">FROM START:</span> <span class="tl-tc-val">{fmtTC(tlTime)}</span></span>
            <span><span class="tl-tc-lbl">TO END:</span> <span class="tl-tc-val">{fmtTC(Math.max(0, TL_DURATION - tlTime))}</span></span>
            <span><span class="tl-tc-lbl">TOTAL TIME:</span> <span class="tl-tc-val">{fmtTC(TL_DURATION)}</span></span>
          </div>
        </div>

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
  .section { max-width: 1375px; display: flex; flex-direction: column; gap: 20px; }
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

  /* ── Timeline Editor (FD-8) ────────────────────────────────────────── */
  .tl-wrap {
    width: 100%;
    background: #0d0d0d;
    border-radius: 6px;
    overflow: visible;
    display: flex;
    flex-direction: column;
    padding: 6px 0;
  }

  .tl-filmstrip {
    position: relative;
    height: 68px;
    margin: 0 12px 4px;
    border-radius: 4px;
    overflow: hidden;
    display: flex;
    gap: 1px;
    background: #111;
    cursor: crosshair;
    flex-shrink: 0;
  }

  .tl-frame {
    flex: 1 1 0%;
    height: 100%;
    min-width: 0;
  }

  .tl-fs-head {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: #60a5fa;
    opacity: 0.9;
    transform: translateX(-50%);
    pointer-events: none;
    z-index: 10;
  }

  .tl-viewport {
    position: relative;
    height: 142px;
    margin: 0 12px;
    flex-shrink: 0;
  }

  .tl-track-bg {
    position: absolute;
    inset: 0;
    background: #161616;
    border-radius: 4px;
    overflow: hidden;
  }

  .tl-wave-svg {
    width: 100%;
    height: 100%;
    display: block;
  }

  .tl-dim {
    position: absolute;
    top: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.85);
    pointer-events: none;
  }

  .tl-fade-grad {
    position: absolute;
    top: 0;
    bottom: 0;
    pointer-events: none;
  }

  .tl-fade-handle {
    position: absolute;
    top: 0;
    transform: translateX(-50%);
    z-index: 25;
    cursor: move;
  }

  .tl-playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 2px;
    background: #60a5fa;
    opacity: 0.9;
    transform: translateX(-50%);
    z-index: 30;
    pointer-events: none;
  }

  .tl-trim-handle {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 14px;
    transform: translateX(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 50;
    cursor: ew-resize;
  }

  .tl-trim-line {
    position: absolute;
    inset: 0;
    width: 2px;
    left: 50%;
    transform: translateX(-50%);
    background: rgba(255, 255, 255, 0.38);
    border-radius: 9999px;
  }

  .tl-trim-nugget {
    position: relative;
    z-index: 10;
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 5px 3px;
    border-radius: 3px;
    background: rgba(255, 255, 255, 0.12);
    border: 1px solid rgba(255, 255, 255, 0.22);
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.55), 0 1px 3px rgba(0, 0, 0, 0.5);
  }

  .tl-controls {
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    margin-top: 4px;
    flex-shrink: 0;
  }

  .tl-btns {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .tl-btn {
    width: 30px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    background: rgba(255, 255, 255, 0.07);
    color: rgba(255, 255, 255, 0.6);
    cursor: pointer;
    transition: filter 0.1s;
    flex-shrink: 0;
  }
  .tl-btn:hover { filter: brightness(1.3); }

  .tl-btn-play {
    border-color: #3b82f6;
    color: white;
  }
  .tl-btn-play.tl-btn-active {
    background: #2563eb;
  }
  .tl-btn:not(.tl-btn-play).tl-btn-active {
    background: #2563eb;
    border-color: #3b82f6;
    color: white;
  }

  .tl-timecodes {
    display: flex;
    align-items: center;
    gap: 16px;
    font-family: 'Geist Mono', monospace;
    font-size: 11px;
    font-variant-numeric: tabular-nums;
  }
  .tl-tc-lbl { color: rgba(255, 255, 255, 0.45); }
  .tl-tc-val { color: rgba(255, 255, 255, 0.9); }
</style>
