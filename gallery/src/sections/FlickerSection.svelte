<script>
  import { Transport, Timecode, GlobalTabs, PanelTabs, SegmentedControl, Button } from '@libre/ui';
  import Card from '../lib/Card.svelte';
  import { onDestroy } from 'svelte';

  // ── Playback state (shared across FLCK-2, FLCK-3) ──────────────────────────
  let playing = $state(false);
  let playbackRate = $state(1);
  let time = $state(0);
  const duration = 312.8;
  const fps = 24;

  let raf = 0;
  let lastTs = 0;

  function tick(ts) {
    if (lastTs) {
      const dt = (ts - lastTs) / 1000;
      time = Math.max(0, Math.min(duration, time + dt * playbackRate));
      if (time >= duration && playbackRate > 0) { playing = false; playbackRate = 1; }
      if (time <= 0 && playbackRate < 0)        { playing = false; playbackRate = 1; }
    }
    lastTs = ts;
    if (playing) raf = requestAnimationFrame(tick);
  }

  function startLoop() { cancelAnimationFrame(raf); lastTs = 0; raf = requestAnimationFrame(tick); }

  function togglePlay() {
    if (playing) { playing = false; playbackRate = 1; cancelAnimationFrame(raf); }
    else         { playing = true;  playbackRate = 1; startLoop(); }
  }
  function rewind()    { if (playing && playbackRate < 0) playbackRate = Math.max(playbackRate * 2, -8); else { playing = true; playbackRate = -1; startLoop(); } }
  function forward()   { if (playing && playbackRate > 0) playbackRate = Math.min(playbackRate * 2,  8); else { playing = true; playbackRate =  1; startLoop(); } }
  function skipStart() { time = 0;        playing = false; cancelAnimationFrame(raf); }
  function skipEnd()   { time = duration; playing = false; cancelAnimationFrame(raf); }

  onDestroy(() => cancelAnimationFrame(raf));

  // ── FLCK-1: Node strip ──────────────────────────────────────────────────────
  let activeClip = $state(1);
  const clips = [
    { id: 0, name: 'V1 — Intro',   kind: 'video' },
    { id: 1, name: 'V1 — City',    kind: 'video' },
    { id: 2, name: 'A1 — Narr',    kind: 'audio' },
    { id: 3, name: 'V1 — Bridge',  kind: 'video' },
    { id: 4, name: 'A2 — Music',   kind: 'audio' },
    { id: 5, name: 'V1 — Outro',   kind: 'video' },
  ];

  // ── FLCK-3: Transport bar track zoom ───────────────────────────────────────
  let trackZoom = $state('100%');
  const trackZoomOptions = [
    { value: '50%',  label: '50%'  },
    { value: '100%', label: '100%' },
    { value: '200%', label: '200%' },
  ];

  // ── FLCK-4: Compositor ─────────────────────────────────────────────────────
  let compositorTab = $state('color');
  const compositorTabs = [
    { id: 'color',   label: 'Color'   },
    { id: 'effects', label: 'Effects' },
    { id: 'audio',   label: 'Audio'   },
  ];

  // ── FLCK-6: Curves channel ─────────────────────────────────────────────────
  let curvesChannel = $state('rgb');

  // ── FLCK-8: Color grading tabs ─────────────────────────────────────────────
  let colorTab = $state('primary');
  const colorTabs = [
    { id: 'primary',   label: 'Primary'   },
    { id: 'secondary', label: 'Secondary' },
    { id: 'luts',      label: 'LUTs'      },
    { id: 'hsl',       label: 'HSL'       },
  ];

  // ── FLCK-9: Mixer ──────────────────────────────────────────────────────────
  let mixerFaders = $state([80, 75, 60, 90, 100]);
  let mixerMutes  = $state([false, false, true, false, false]);
  const mixerStrips = ['A1', 'A2', 'A3', 'Music', 'Master'];

  // ── FLCK-10: Media pool ────────────────────────────────────────────────────
  let mediaQuery    = $state('');
  let selectedMedia = $state(0);
  const mediaItems = [
    { id: 0, name: 'city_walk.mp4',   kind: 'video' },
    { id: 1, name: 'narration.wav',   kind: 'audio' },
    { id: 2, name: 'bridge_shot.mp4', kind: 'video' },
    { id: 3, name: 'music_loop.aif',  kind: 'audio' },
    { id: 4, name: 'outro_clip.mp4',  kind: 'video' },
    { id: 5, name: 'fx_whoosh.wav',   kind: 'audio' },
  ];

  // ── FLCK-11: Folder tree ───────────────────────────────────────────────────
  let treeExpanded = $state({ footage: true, music: false, exports: false });

  // ── FLCK-12: Clip inspector (inline) ──────────────────────────────────────
  let inspClip = $state({ name: 'city_walk.mp4', posX: 0, posY: 0, scaleX: 1.0, scaleY: 1.0, rotation: 0, opacity: 100, blendMode: 'Normal', speed: 100, reverse: false });
  let inspCollapsed = $state({ transform: false, composite: false, speed: false });
  const blendModes = ['Normal', 'Multiply', 'Screen', 'Overlay', 'Add', 'Darken', 'Lighten'];

  function chevron(c) { return `transform:${c ? 'rotate(0deg)' : 'rotate(90deg)'}`; }

  // ── FLCK-13: Export / Delivery ─────────────────────────────────────────────
  let selectedFormat = $state(0);
  const formats = ['H.264 MP4', 'ProRes 422', 'DNxHD', 'WebM VP9', 'GIF'];
  let exportRes     = $state('1080p');
  let exportFps     = $state('23.976');
  let exportQuality = $state(85);
  const resOptions  = [
    { value: '720p',  label: '720p'  },
    { value: '1080p', label: '1080p' },
    { value: '4K',    label: '4K'    },
  ];
</script>

<div class="section">

  <!-- ── FLCK-1 Node Strip ──────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-1" label="Node Strip">
      <div class="node-strip">
        {#each clips as clip}
          <button class="node-clip" class:node-clip-active={activeClip === clip.id} onclick={() => (activeClip = clip.id)}>
            <div class="node-thumb" class:node-thumb-audio={clip.kind === 'audio'}></div>
            <span class="node-name">{clip.name}</span>
          </button>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── FLCK-2 Timeline ────────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-2" label="Timeline">
      <div class="timeline">
        <!-- Ruler -->
        <div class="tl-ruler">
          <div class="tl-track-label"></div>
          <div class="tl-ruler-ticks">
            {#each [0, 24, 48, 72, 96, 120] as f}
              <span class="tl-tick" style="left:{(f / 120) * 100}%">{f}</span>
            {/each}
          </div>
        </div>
        <!-- Tracks -->
        {#each [
          { label: 'V1', kind: 'video', clips: [{ left: 0, width: 35, title: 'Intro' }, { left: 37, width: 45, title: 'City Walk' }] },
          { label: 'A1', kind: 'audio', clips: [{ left: 0, width: 55, title: '' }] },
          { label: 'A2', kind: 'audio', clips: [{ left: 10, width: 70, title: '' }] },
        ] as track}
          <div class="tl-track">
            <div class="tl-track-label">{track.label}</div>
            <div class="tl-track-body">
              {#each track.clips as c}
                <div class="tl-clip" class:tl-clip-audio={track.kind === 'audio'} style="left:{c.left}%;width:{c.width}%">
                  {#if c.title}<span class="tl-clip-title">{c.title}</span>{/if}
                </div>
              {/each}
              <!-- Playhead -->
              <div class="tl-playhead" style="left:{(time / duration) * 100}%"></div>
            </div>
          </div>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── FLCK-3 Transport Bar ───────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-3" label="Transport Bar">
      <div class="transport-bar">
        <Timecode {time} {duration} {fps} />
        <Transport
          {playing}
          {playbackRate}
          onTogglePlay={togglePlay}
          onRewind={rewind}
          onForward={forward}
          onSkipStart={skipStart}
          onSkipEnd={skipEnd}
          showFullscreen
        />
        <div class="transport-right">
          <SegmentedControl options={trackZoomOptions} bind:value={trackZoom} variant="sliding" size="sm" />
        </div>
      </div>
    </Card>
  </div>

  <!-- ── FLCK-4 Compositor ──────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-4" label="Compositor">
      <div class="compositor">
        <div class="compositor-viewer">
          <div class="comp-frame">
            <div class="comp-canvas"></div>
          </div>
        </div>
        <div class="compositor-tabs">
          <PanelTabs tabs={compositorTabs} bind:active={compositorTab} ariaLabel="Compositor panel" />
        </div>
      </div>
    </Card>
  </div>

  <!-- ── Color tools row ────────────────────────────────────────────────────── -->
  <div class="cols">

    <!-- FLCK-5 Color Wheel -->
    <Card id="FLCK-5" label="Color Wheel">
      <div class="color-wheel-wrap">
        <div class="color-wheel"></div>
        <div class="color-swatch"></div>
        <span class="color-label">Hue 210°  Sat 80%  Val 90%</span>
      </div>
    </Card>

    <!-- FLCK-6 Curves Editor -->
    <Card id="FLCK-6" label="Curves Editor">
      <div class="curves-wrap">
        <div class="curves-channels">
          {#each ['rgb', 'r', 'g', 'b'] as ch}
            <button class="curves-ch-btn" class:curves-ch-active={curvesChannel === ch} onclick={() => (curvesChannel = ch)}>{ch.toUpperCase()}</button>
          {/each}
        </div>
        <svg class="curves-svg" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
          <!-- Grid -->
          {#each [50, 100, 150] as g}
            <line x1={g} y1="0" x2={g} y2="200" stroke="var(--border-subtle)" stroke-width="1"/>
            <line x1="0" y1={g} x2="200" y2={g} stroke="var(--border-subtle)" stroke-width="1"/>
          {/each}
          <!-- Border -->
          <rect x="0" y="0" width="200" height="200" fill="none" stroke="var(--border)" stroke-width="1"/>
          <!-- Baseline diagonal -->
          <line x1="0" y1="200" x2="200" y2="0" stroke="var(--text-muted)" stroke-width="1" stroke-dasharray="4 4" opacity="0.5"/>
          <!-- RGB S-curve -->
          {#if curvesChannel === 'rgb'}
            <path d="M0,200 C50,180 80,140 100,100 C120,60 150,20 200,0" fill="none" stroke="var(--text-secondary)" stroke-width="1.5" opacity="0.8"/>
          {/if}
          {#if curvesChannel === 'r'}
            <path d="M0,200 C40,170 90,130 110,90 C130,50 160,15 200,0" fill="none" stroke="color-mix(in srgb, red 70%, var(--text-primary))" stroke-width="1.5" opacity="0.8"/>
          {/if}
          {#if curvesChannel === 'g'}
            <path d="M0,200 C60,175 85,145 100,100 C115,55 140,25 200,0" fill="none" stroke="color-mix(in srgb, lime 70%, var(--text-primary))" stroke-width="1.5" opacity="0.8"/>
          {/if}
          {#if curvesChannel === 'b'}
            <path d="M0,200 C55,185 75,150 95,108 C115,65 155,22 200,0" fill="none" stroke="color-mix(in srgb, dodgerblue 70%, var(--text-primary))" stroke-width="1.5" opacity="0.8"/>
          {/if}
          <!-- Control points -->
          <circle cx="100" cy="100" r="4" fill="var(--accent)" stroke="var(--surface)" stroke-width="1.5"/>
        </svg>
      </div>
    </Card>

    <!-- FLCK-7 Scopes -->
    <Card id="FLCK-7" label="Scopes">
      <div class="scopes-wrap">
        <div class="scope-block">
          <span class="scope-label">WAVEFORM</span>
          <svg class="waveform-svg" viewBox="0 0 180 120" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none">
            <rect width="180" height="120" fill="var(--surface)"/>
            {#each Array(8) as _, i}
              <rect x={i * 22 + 2} y={20 + Math.sin(i * 1.3) * 15} width="18" height={60 + Math.cos(i * 0.9) * 20} fill="var(--accent)" opacity="0.18"/>
            {/each}
            {#each Array(6) as _, i}
              <path d="M{i * 30},120 Q{i * 30 + 15},{60 + Math.sin(i) * 30} {i * 30 + 30},120" fill="none" stroke="var(--accent)" stroke-width="1" opacity="0.4"/>
            {/each}
          </svg>
        </div>
        <div class="scope-block">
          <span class="scope-label">VECTORSCOPE</span>
          <div class="vectorscope">
            <div class="vectorscope-wheel"></div>
            <div class="vectorscope-dots">
              {#each Array(12) as _, i}
                <div class="vs-dot" style="left:{42 + Math.cos(i * 0.8) * 18}%;top:{42 + Math.sin(i * 1.1) * 18}%"></div>
              {/each}
            </div>
          </div>
        </div>
      </div>
    </Card>

  </div>

  <!-- ── FLCK-8 Color Grading Tab ───────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-8" label="Color Grading Tab">
      <div class="color-grade-wrap">
        <div class="cg-tabs-row">
          <GlobalTabs tabs={colorTabs} bind:active={colorTab} ariaLabel="Color grading" />
        </div>
        {#if colorTab === 'primary'}
          <div class="cg-wheels">
            {#each ['Lift', 'Gamma', 'Gain'] as wheel}
              <div class="cg-wheel-group">
                <div class="cg-wheel"></div>
                <span class="cg-wheel-label">{wheel}</span>
              </div>
            {/each}
          </div>
        {:else}
          <div class="cg-placeholder">
            {#if colorTab === 'secondary'}
              Secondary grading — select a qualifier
            {:else if colorTab === 'luts'}
              LUTs — no LUT applied
            {:else}
              HSL — hue / saturation / luminance ranges
            {/if}
          </div>
        {/if}
      </div>
    </Card>
  </div>

  <!-- ── FLCK-9 Audio Mixer ─────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-9" label="Audio Mixer">
      <div class="mixer">
        {#each mixerStrips as strip, i}
          <div class="mixer-strip" class:mixer-strip-master={strip === 'Master'}>
            <div class="mixer-vu">
              {#each [0, 1, 2, 3, 4] as seg}
                <div class="vu-seg" class:vu-green={seg < 3} class:vu-yellow={seg === 3} class:vu-red={seg === 4}
                  class:vu-active={mixerFaders[i] > (4 - seg) * 20}></div>
              {/each}
            </div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <input
              type="range"
              min="0"
              max="100"
              bind:value={mixerFaders[i]}
              class="mixer-fader"
              style="writing-mode: vertical-lr; direction: rtl;"
            />
            <div class="mixer-controls">
              <button class="mixer-btn" class:mixer-btn-muted={mixerMutes[i]} onclick={() => (mixerMutes[i] = !mixerMutes[i])} title="Mute">M</button>
              <button class="mixer-btn" title="Solo">S</button>
            </div>
            <span class="mixer-label">{strip}</span>
          </div>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── Side panels row ────────────────────────────────────────────────────── -->
  <div class="cols">

    <!-- FLCK-10 Media Pool -->
    <Card id="FLCK-10" label="Media Pool">
      <div class="media-pool">
        <div class="search-compact">
          <svg class="search-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <circle cx="11" cy="11" r="7"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input type="search" class="search-input" placeholder="Search media…" bind:value={mediaQuery} />
        </div>
        <div class="media-grid">
          {#each mediaItems as item}
            <button class="media-item" class:media-item-active={selectedMedia === item.id} onclick={() => (selectedMedia = item.id)}>
              <div class="media-thumb" class:media-thumb-audio={item.kind === 'audio'}>
                {#if item.kind === 'video'}
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" opacity="0.4"><path d="M8 5v14l11-7z"/></svg>
                {:else}
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" opacity="0.4"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
                {/if}
              </div>
              <span class="media-name">{item.name}</span>
            </button>
          {/each}
        </div>
      </div>
    </Card>

    <!-- FLCK-11 Folder Tree -->
    <Card id="FLCK-11" label="Folder Tree">
      <div class="folder-tree">
        <!-- Root -->
        <div class="ft-row ft-root">
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name ft-name-root">Project Root</span>
        </div>
        <!-- Footage -->
        <button class="ft-row" onclick={() => (treeExpanded.footage = !treeExpanded.footage)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.footage)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Footage</span>
          <span class="ft-badge">3</span>
        </button>
        {#if treeExpanded.footage}
          {#each ['city_walk.mp4', 'bridge_shot.mp4', 'outro_clip.mp4'] as file}
            <div class="ft-row ft-row-child">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-file"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zm-1 1.5L18.5 8H13V3.5z"/></svg>
              <span class="ft-name ft-name-file">{file}</span>
            </div>
          {/each}
        {/if}
        <!-- Music -->
        <button class="ft-row" onclick={() => (treeExpanded.music = !treeExpanded.music)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.music)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Music</span>
        </button>
        {#if treeExpanded.music}
          <div class="ft-row ft-row-child">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-file"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zm-1 1.5L18.5 8H13V3.5z"/></svg>
            <span class="ft-name ft-name-file">music_loop.aif</span>
          </div>
        {/if}
        <!-- Exports -->
        <button class="ft-row" onclick={() => (treeExpanded.exports = !treeExpanded.exports)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.exports)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Exports</span>
        </button>
      </div>
    </Card>

    <!-- FLCK-12 Clip Inspector (inline ir-* classes) -->
    <Card id="FLCK-12" label="Clip Inspector">
      <div class="ir">
        <div class="ir-clip-name">{inspClip.name}</div>

        <!-- Transform -->
        <div class="ir-section">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.transform = !inspCollapsed.transform)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Transform</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.transform)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.transform}
            {#each [['Position X', 'posX', 1], ['Position Y', 'posY', 1], ['Scale X', 'scaleX', 0.01], ['Scale Y', 'scaleY', 0.01], ['Rotation', 'rotation', 0.1], ['Opacity', 'opacity', 1]] as [label, key, step]}
              <div class="ir-row">
                <span class="ir-lbl">{label}</span>
                <input type="number" bind:value={inspClip[key]} step={step} class="ir-num" />
              </div>
            {/each}
          {/if}
        </div>

        <!-- Composite -->
        <div class="ir-section">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.composite = !inspCollapsed.composite)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Composite</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.composite)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.composite}
            <div class="ir-row">
              <span class="ir-lbl">Blend Mode</span>
              <select bind:value={inspClip.blendMode} class="ir-sel">
                {#each blendModes as m}<option>{m}</option>{/each}
              </select>
            </div>
          {/if}
        </div>

        <!-- Speed -->
        <div class="ir-section" style="border-bottom:none">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.speed = !inspCollapsed.speed)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Speed</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.speed)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.speed}
            <div class="ir-row">
              <span class="ir-lbl">Speed %</span>
              <input type="number" bind:value={inspClip.speed} min="1" max="400" step="1" class="ir-num" />
            </div>
            <div class="ir-row">
              <span class="ir-lbl">Reverse</span>
              <input type="checkbox" bind:checked={inspClip.reverse} class="ir-check" />
            </div>
          {/if}
        </div>
      </div>
    </Card>

  </div>

  <!-- ── FLCK-13 Export / Delivery ──────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-13" label="Export / Delivery">
      <div class="export-wrap">
        <!-- Format list -->
        <div class="export-formats">
          <div class="export-formats-hd">Presets</div>
          {#each formats as fmt, i}
            <button class="export-fmt-row" class:export-fmt-active={selectedFormat === i} onclick={() => (selectedFormat = i)}>
              {fmt}
            </button>
          {/each}
        </div>
        <!-- Settings -->
        <div class="export-settings">
          <div class="export-settings-hd">{formats[selectedFormat]}</div>

          <div class="export-field">
            <span class="export-label">Resolution</span>
            <SegmentedControl options={resOptions} bind:value={exportRes} variant="filled" size="sm" />
          </div>

          <div class="export-field">
            <label class="export-label" for="export-fps">Frame Rate</label>
            <select id="export-fps" bind:value={exportFps} class="export-select">
              {#each ['23.976', '24', '25', '29.97', '60'] as r}<option value={r}>{r} fps</option>{/each}
            </select>
          </div>

          <div class="export-field">
            <label class="export-label" for="export-quality">Quality — {exportQuality}</label>
            <input id="export-quality" type="range" min="0" max="100" bind:value={exportQuality} class="export-slider" />
          </div>

          <div class="export-field export-path-row">
            <span class="export-path">/Users/eldo/Movies/export_{formats[selectedFormat].replace(/ /g, '_').toLowerCase()}.mov</span>
            <button class="export-browse-btn">Browse</button>
          </div>

          <div class="export-action">
            <Button variant="primary" onclick={() => {}}>Export</Button>
          </div>
        </div>
      </div>
    </Card>
  </div>

</div>

<style>
  .section { max-width: 1375px; display: flex; flex-direction: column; gap: 20px; }
  .row  { width: 100%; }
  .cols { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── FLCK-1: Node Strip ──────────────────────────────────────────────────── */
  .node-strip {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding: 6px 4px;
  }
  .node-clip {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
  }
  .node-thumb {
    width: 80px;
    height: 44px;
    border-radius: 4px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
  }
  .node-thumb-audio {
    border-color: color-mix(in srgb, var(--text-muted) 40%, var(--border));
    background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-muted));
  }
  .node-clip-active .node-thumb {
    border-color: color-mix(in srgb, var(--accent) 60%, var(--border));
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent) 30%, transparent);
  }
  .node-name {
    font-size: 9px;
    color: var(--text-muted);
    white-space: nowrap;
    font-family: 'Geist Mono', monospace;
  }

  /* ── FLCK-2: Timeline ────────────────────────────────────────────────────── */
  .timeline {
    display: flex;
    flex-direction: column;
    width: 100%;
    user-select: none;
  }
  .tl-ruler {
    display: flex;
    height: 18px;
    border-bottom: 1px solid var(--border);
  }
  .tl-ruler-ticks {
    position: relative;
    flex: 1;
    background: color-mix(in srgb, var(--surface-raised) 70%, var(--surface));
  }
  .tl-tick {
    position: absolute;
    transform: translateX(-50%);
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    top: 2px;
  }
  .tl-track {
    display: flex;
    height: 28px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .tl-track-label {
    width: 40px;
    flex-shrink: 0;
    font-size: 9px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.06em;
    display: flex;
    align-items: center;
    padding-left: 8px;
    background: var(--surface-raised);
    border-right: 1px solid var(--border);
  }
  .tl-track-body {
    flex: 1;
    position: relative;
    background: color-mix(in srgb, var(--surface) 80%, var(--surface-raised));
    overflow: hidden;
  }
  .tl-clip {
    position: absolute;
    top: 3px;
    height: 22px;
    border-radius: 3px;
    background: color-mix(in srgb, var(--accent) 22%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--accent) 40%, var(--border));
    overflow: hidden;
  }
  .tl-clip-audio {
    background: repeating-linear-gradient(
      90deg,
      color-mix(in srgb, var(--text-secondary) 12%, transparent) 0px,
      color-mix(in srgb, var(--text-secondary) 12%, transparent) 2px,
      transparent 2px,
      transparent 6px
    );
    border-color: color-mix(in srgb, var(--text-muted) 30%, var(--border));
  }
  .tl-clip-title {
    font-size: 8px;
    color: var(--text-secondary);
    padding: 0 4px;
    line-height: 22px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .tl-playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--accent);
    pointer-events: none;
    z-index: 10;
  }
  .tl-playhead::before {
    content: '';
    position: absolute;
    top: 0;
    left: -3px;
    width: 7px;
    height: 7px;
    background: var(--accent);
    clip-path: polygon(50% 100%, 0 0, 100% 0);
  }

  /* ── FLCK-3: Transport Bar ───────────────────────────────────────────────── */
  .transport-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 6px 8px;
    background: var(--surface-raised);
    border-radius: 6px;
  }
  .transport-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  /* ── FLCK-4: Compositor ──────────────────────────────────────────────────── */
  .compositor {
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }
  .compositor-viewer {
    background: color-mix(in srgb, var(--surface) 40%, #000);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
  }
  .comp-frame {
    width: 100%;
    max-width: 540px;
    aspect-ratio: 16 / 9;
    border: 1px solid color-mix(in srgb, var(--border) 60%, #000);
    border-radius: 2px;
    overflow: hidden;
    position: relative;
  }
  .comp-canvas {
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg,
      color-mix(in srgb, var(--accent) 14%, #000) 0%,
      #0a0a0a 40%,
      color-mix(in srgb, var(--surface) 15%, #000) 100%
    );
  }
  .compositor-tabs {
    background: var(--surface-raised);
    border-top: 1px solid var(--border);
    padding: 0 8px;
    display: flex;
    align-items: stretch;
  }

  /* ── FLCK-5: Color Wheel ─────────────────────────────────────────────────── */
  .color-wheel-wrap {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    padding: 12px;
  }
  .color-wheel {
    width: 160px;
    height: 160px;
    border-radius: 50%;
    background: conic-gradient(red, yellow, lime, cyan, blue, magenta, red);
    box-shadow: 0 0 0 2px var(--border);
    position: relative;
  }
  .color-wheel::after {
    content: '';
    position: absolute;
    inset: 20px;
    border-radius: 50%;
    background: radial-gradient(circle at 70% 35%, white 0%, transparent 65%),
                radial-gradient(circle at center, transparent 30%, rgba(0,0,0,0.7) 100%);
  }
  .color-swatch {
    width: 48px;
    height: 16px;
    border-radius: 3px;
    background: var(--accent);
    border: 1px solid var(--border);
  }
  .color-label {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    letter-spacing: 0.05em;
  }

  /* ── FLCK-6: Curves Editor ───────────────────────────────────────────────── */
  .curves-wrap {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 8px;
  }
  .curves-channels {
    display: flex;
    gap: 4px;
  }
  .curves-ch-btn {
    font-size: 9px;
    font-weight: 700;
    padding: 2px 7px;
    border-radius: 3px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    font-family: 'Geist Mono', monospace;
    letter-spacing: 0.05em;
  }
  .curves-ch-btn:hover { color: var(--text-primary); }
  .curves-ch-active {
    background: var(--surface-panel);
    color: var(--accent) !important;
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
  }
  .curves-svg {
    width: 100%;
    max-width: 220px;
    height: 200px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 3px;
    display: block;
  }

  /* ── FLCK-7: Scopes ──────────────────────────────────────────────────────── */
  .scopes-wrap {
    display: flex;
    gap: 12px;
    padding: 8px;
  }
  .scope-block {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .scope-label {
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 0.12em;
    color: var(--text-muted);
    text-transform: uppercase;
  }
  .waveform-svg {
    width: 180px;
    height: 120px;
    border: 1px solid var(--border);
    border-radius: 3px;
    display: block;
  }
  .vectorscope {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: var(--surface);
    position: relative;
    overflow: hidden;
  }
  .vectorscope-wheel {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    background: conic-gradient(
      color-mix(in srgb, red 20%, transparent),
      color-mix(in srgb, yellow 20%, transparent),
      color-mix(in srgb, lime 20%, transparent),
      color-mix(in srgb, cyan 20%, transparent),
      color-mix(in srgb, blue 20%, transparent),
      color-mix(in srgb, magenta 20%, transparent),
      color-mix(in srgb, red 20%, transparent)
    );
    opacity: 0.3;
  }
  .vectorscope-dots { position: absolute; inset: 0; }
  .vs-dot {
    position: absolute;
    width: 3px;
    height: 3px;
    border-radius: 50%;
    background: var(--accent);
    opacity: 0.6;
    transform: translate(-50%, -50%);
  }

  /* ── FLCK-8: Color Grading ───────────────────────────────────────────────── */
  .color-grade-wrap {
    display: flex;
    flex-direction: column;
    gap: 16px;
    width: 100%;
    padding: 4px 0;
  }
  .cg-tabs-row {
    padding: 0 8px;
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }
  .cg-wheels {
    display: flex;
    gap: 24px;
    padding: 8px 16px 16px;
  }
  .cg-wheel-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }
  .cg-wheel {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: conic-gradient(red, yellow, lime, cyan, blue, magenta, red);
    box-shadow: 0 0 0 1px var(--border);
    position: relative;
  }
  .cg-wheel::after {
    content: '';
    position: absolute;
    inset: 10px;
    border-radius: 50%;
    background: radial-gradient(circle at 65% 35%, rgba(255,255,255,0.5) 0%, transparent 60%),
                radial-gradient(circle at center, transparent 25%, rgba(0,0,0,0.6) 100%);
  }
  .cg-wheel-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .cg-placeholder {
    padding: 40px 16px;
    text-align: center;
    font-size: 11px;
    color: var(--text-muted);
    font-style: italic;
  }

  /* ── FLCK-9: Audio Mixer ─────────────────────────────────────────────────── */
  .mixer {
    display: flex;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--surface-raised);
  }
  .mixer-strip {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px 4px 6px;
    gap: 4px;
    border-right: 1px solid var(--border-subtle);
  }
  .mixer-strip:last-child { border-right: none; }
  .mixer-strip-master {
    background: color-mix(in srgb, var(--surface-raised) 60%, var(--surface));
  }
  .mixer-vu {
    display: flex;
    flex-direction: column-reverse;
    gap: 1px;
    height: 60px;
    width: 14px;
  }
  .vu-seg {
    flex: 1;
    border-radius: 1px;
    opacity: 0.25;
  }
  .vu-seg.vu-active { opacity: 1; }
  .vu-green  { background: color-mix(in srgb, lime 60%, var(--accent)); }
  .vu-yellow { background: color-mix(in srgb, yellow 80%, var(--accent)); }
  .vu-red    { background: color-mix(in srgb, red 80%, var(--accent)); }
  .mixer-fader {
    height: 60px;
    width: 10px;
    accent-color: var(--accent);
    cursor: pointer;
    appearance: slider-vertical;
  }
  .mixer-controls {
    display: flex;
    gap: 2px;
  }
  .mixer-btn {
    width: 18px;
    height: 14px;
    font-size: 8px;
    font-weight: 700;
    font-family: inherit;
    border-radius: 2px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.1s, color 0.1s;
    padding: 0;
  }
  .mixer-btn:hover { color: var(--text-primary); }
  .mixer-btn-muted { background: color-mix(in srgb, var(--accent) 15%, var(--surface)); color: var(--accent); border-color: color-mix(in srgb, var(--accent) 40%, var(--border)); }
  .mixer-label {
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 0.07em;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-top: 2px;
  }

  /* ── FLCK-10: Media Pool ─────────────────────────────────────────────────── */
  .media-pool {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 6px;
    width: 340px;
  }
  .search-compact {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    padding: 4px 8px;
    height: 28px;
    box-sizing: border-box;
    transition: box-shadow 0.1s;
  }
  .search-compact:focus-within { box-shadow: inset 0 -2px 0 0 var(--accent); }
  .search-icon { color: var(--text-muted); flex-shrink: 0; }
  .search-input {
    flex: 1;
    min-width: 0;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 12px;
  }
  .search-input::placeholder { color: var(--text-muted); }
  .search-input::-webkit-search-cancel-button { display: none; }
  .media-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
  }
  .media-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px;
  }
  .media-thumb {
    width: 90px;
    height: 60px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface-raised) 60%, #000);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.1s;
  }
  .media-thumb-audio {
    background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-muted));
  }
  .media-item-active .media-thumb {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent) 30%, transparent);
  }
  .media-name {
    font-size: 8px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
    text-align: center;
    width: 90px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* ── FLCK-11: Folder Tree ────────────────────────────────────────────────── */
  .folder-tree {
    display: flex;
    flex-direction: column;
    width: 180px;
    font-size: 11px;
  }
  .ft-row {
    display: flex;
    align-items: center;
    gap: 5px;
    height: 28px;
    padding: 0 8px;
    width: 100%;
    background: none;
    border: none;
    cursor: default;
    font-family: inherit;
    font-size: 11px;
    color: var(--text-secondary);
    transition: background 0.1s;
    text-align: left;
  }
  button.ft-row { cursor: pointer; }
  button.ft-row:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }
  .ft-root { padding-left: 8px; }
  .ft-row-child { padding-left: 28px; cursor: default; }
  .ft-chev {
    color: var(--text-muted);
    flex-shrink: 0;
    transition: transform 0.12s;
  }
  .ft-icon-folder { color: color-mix(in srgb, var(--accent) 60%, var(--text-muted)); flex-shrink: 0; }
  .ft-icon-file   { color: var(--text-muted); flex-shrink: 0; }
  .ft-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .ft-name-root { font-weight: 600; color: var(--text-primary); }
  .ft-name-file { color: var(--text-muted); font-size: 10px; }
  .ft-badge {
    font-size: 8px;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0 5px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  /* ── FLCK-12: Clip Inspector (ir-* classes) ──────────────────────────────── */
  .ir {
    background: var(--surface);
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    width: 200px;
  }
  .ir-clip-name {
    padding: 6px 12px;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary);
    border-bottom: 1px solid var(--border);
    font-family: 'Geist Mono', monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
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
    padding: 3px 12px;
    gap: 4px;
    transition: background 0.1s;
  }
  .ir-row:hover { background: color-mix(in srgb, var(--surface) 94%, var(--text-primary)); }
  .ir-lbl {
    flex: 1;
    font-size: 10px;
    color: var(--text-muted);
  }
  .ir-num {
    width: 56px;
    text-align: right;
    background: transparent;
    border: none;
    border-bottom: 1px solid transparent;
    color: var(--accent);
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    outline: none;
    padding: 0 2px;
    -moz-appearance: textfield;
  }
  .ir-num::-webkit-inner-spin-button,
  .ir-num::-webkit-outer-spin-button { -webkit-appearance: none; }
  .ir-num:focus { border-bottom-color: var(--accent); }
  .ir-sel {
    font-size: 10px;
    font-family: inherit;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    padding: 2px 4px;
    outline: none;
    cursor: pointer;
  }
  .ir-check {
    accent-color: var(--accent);
    width: 13px;
    height: 13px;
    cursor: pointer;
  }

  /* ── FLCK-13: Export / Delivery ──────────────────────────────────────────── */
  .export-wrap {
    display: flex;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    min-height: 320px;
  }
  .export-formats {
    width: 200px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface-raised);
    display: flex;
    flex-direction: column;
  }
  .export-formats-hd {
    padding: 8px 12px;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
  }
  .export-fmt-row {
    display: flex;
    align-items: center;
    height: 32px;
    padding: 0 12px;
    font-size: 11px;
    font-family: inherit;
    background: none;
    border: none;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    cursor: pointer;
    text-align: left;
    transition: background 0.1s, color 0.1s;
  }
  .export-fmt-row:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); color: var(--text-primary); }
  .export-fmt-active {
    color: var(--accent) !important;
    border-left: 2px solid var(--accent);
    padding-left: 10px;
    background: color-mix(in srgb, var(--accent) 6%, var(--surface-raised)) !important;
  }
  .export-settings {
    flex: 1;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .export-settings-hd {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }
  .export-field {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .export-label {
    font-size: 10px;
    color: var(--text-muted);
    width: 80px;
    flex-shrink: 0;
  }
  .export-select {
    font-size: 11px;
    font-family: inherit;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    padding: 3px 6px;
    outline: none;
    cursor: pointer;
  }
  .export-slider {
    flex: 1;
    accent-color: var(--accent);
    cursor: pointer;
  }
  .export-path-row {
    flex-wrap: wrap;
    gap: 4px;
  }
  .export-path {
    flex: 1;
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    background: var(--surface-raised);
    border: 1px solid var(--border-subtle);
    border-radius: 3px;
    padding: 3px 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .export-browse-btn {
    font-size: 10px;
    font-family: inherit;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text-secondary);
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.1s;
    flex-shrink: 0;
  }
  .export-browse-btn:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }
  .export-action {
    margin-top: auto;
    padding-top: 8px;
  }
  .export-action :global(button) { width: 100%; }
</style>
