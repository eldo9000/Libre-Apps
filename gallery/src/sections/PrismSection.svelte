<script>
  import { Transport, Timecode, PanelTabs } from '@libre/ui';
  import Card from '../lib/Card.svelte';
  import { onDestroy } from 'svelte';

  // Playback state — rAF loop copied from MediaSection pattern
  let playing = $state(false);
  let playbackRate = $state(1);
  let time = $state(0);
  const duration = 245.0;
  const fps = 24;

  let raf = 0;
  let lastTs = 0;

  function tick(ts) {
    if (lastTs) {
      const dt = (ts - lastTs) / 1000;
      time = Math.max(0, Math.min(duration, time + dt * playbackRate));
      if (time >= duration && playbackRate > 0) {
        playing = false;
        playbackRate = 1;
      }
      if (time <= 0 && playbackRate < 0) {
        playing = false;
        playbackRate = 1;
      }
    }
    lastTs = ts;
    if (playing) raf = requestAnimationFrame(tick);
  }

  function startLoop() {
    cancelAnimationFrame(raf);
    lastTs = 0;
    raf = requestAnimationFrame(tick);
  }

  function togglePlay() {
    if (playing) { playing = false; playbackRate = 1; cancelAnimationFrame(raf); }
    else { playing = true; playbackRate = 1; startLoop(); }
  }
  function rewind() {
    if (playing && playbackRate < 0) playbackRate = Math.max(playbackRate * 2, -8);
    else { playing = true; playbackRate = -1; startLoop(); }
  }
  function forward() {
    if (playing && playbackRate > 0) playbackRate = Math.min(playbackRate * 2, 8);
    else { playing = true; playbackRate = 1; startLoop(); }
  }
  function skipStart() { time = 0; playing = false; cancelAnimationFrame(raf); }
  function skipEnd()   { time = duration; playing = false; cancelAnimationFrame(raf); }

  onDestroy(() => cancelAnimationFrame(raf));

  // PRS-1 state
  let zoom = $state(100);
  let rotation = $state(0);
  let imageTab = $state('info');
  const imageTabs = [
    { id: 'info', label: 'Info' },
    { id: 'edit', label: 'Edit' },
    { id: 'share', label: 'Share' },
  ];

  function zoomIn()  { zoom = Math.min(400, zoom + 10); }
  function zoomOut() { zoom = Math.max(10,  zoom - 10); }
  function rotate()  { rotation = (rotation + 90) % 360; }
  function fitView() { zoom = 100; rotation = 0; }

  // PRS-3 state
  let pdfPage = $state(1);
  const pdfPageCount = 12;
  function prevPage() { pdfPage = Math.max(1, pdfPage - 1); }
  function nextPage() { pdfPage = Math.min(pdfPageCount, pdfPage + 1); }

  // PRS-4 state
  let audioVolume = $state(80);

  // Simulated text-line widths for PDF viewer
  const lineWidths = [
    100, 85, 92, 70, 88, 75, 60, 95, 80, 65,
    90, 78, 50, 88, 72, 84, 58, 76, 92, 68,
  ];
</script>

<div class="section">

  <!-- Row 1: Image viewer + 3D model viewer -->
  <div class="cols">
    <!-- PRS-1: Media Viewer (Image) -->
    <Card id="PRS-1" label="Media Viewer" sourceFile="common-js/src/components/PanelTabs.svelte">
      <div class="viewer-shell" style="width:480px">
        <!-- Top toolbar -->
        <div class="viewer-toolbar">
          <span class="filename">mountain_vista.jpg</span>
          <div class="toolbar-spacer"></div>
          <PanelTabs
            tabs={imageTabs}
            bind:active={imageTab}
            ariaLabel="Image panels"
          />
        </div>

        <!-- Image area -->
        <div class="viewer-area viewer-image-bg" style="height:260px">
          <svg
            width="320"
            height="180"
            viewBox="0 0 320 180"
            style="transform: rotate({rotation}deg); transition: transform 0.2s ease;"
          >
            <!-- Sky gradient substitute -->
            <rect width="320" height="130" fill="color-mix(in srgb, var(--surface) 15%, #1a2a4a)" rx="2"/>
            <!-- Ground -->
            <rect y="130" width="320" height="50" fill="color-mix(in srgb, var(--surface) 20%, #2a3a2a)"/>
            <!-- Mountain silhouettes — back range -->
            <polygon
              points="0,130 60,60 120,100 180,45 240,90 300,55 320,75 320,130"
              fill="color-mix(in srgb, var(--text-muted) 30%, #3a4a5a)"
            />
            <!-- Mountain silhouettes — front range -->
            <polygon
              points="0,130 40,90 90,120 140,70 190,105 250,65 290,95 320,80 320,130"
              fill="color-mix(in srgb, var(--text-muted) 20%, #2a3530)"
            />
            <!-- Snow caps -->
            <polygon points="140,70 155,52 170,70" fill="color-mix(in srgb, #fff 70%, var(--surface))"/>
            <polygon points="250,65 262,48 274,65" fill="color-mix(in srgb, #fff 60%, var(--surface))"/>
            <!-- Sun -->
            <circle cx="270" cy="35" r="14" fill="color-mix(in srgb, #ffcc44 80%, var(--surface))"/>
          </svg>
        </div>

        <!-- Bottom toolbar -->
        <div class="viewer-toolbar">
          <button class="icon-btn" onclick={zoomOut} title="Zoom out">−</button>
          <span class="zoom-label">{zoom}%</span>
          <button class="icon-btn" onclick={zoomIn} title="Zoom in">+</button>
          <div class="tb-sep"></div>
          <button class="icon-btn" onclick={rotate} title="Rotate 90°">↺</button>
          <button class="icon-btn" onclick={fitView} title="Fit to window">⊡</button>
          <button class="icon-btn" title="Fullscreen">⛶</button>
        </div>
      </div>
    </Card>

    <!-- PRS-2: 3D Model Viewer -->
    <Card id="PRS-2" label="3D Model Viewer">
      <div class="model-shell" style="width:360px">
        <span class="filename" style="padding: 6px 10px 4px; display:block;">model_v3.glb</span>
        <!-- Dark viewport -->
        <div class="model-viewport" style="height:260px">
          <!-- Wireframe cube — isometric projection -->
          <svg width="180" height="160" viewBox="0 0 180 160" style="overflow:visible">
            <!-- Back edges -->
            <line x1="50" y1="40" x2="110" y2="40" stroke="color-mix(in srgb, var(--accent) 40%, transparent)" stroke-width="1"/>
            <line x1="50" y1="40" x2="50" y2="100" stroke="color-mix(in srgb, var(--accent) 40%, transparent)" stroke-width="1"/>
            <line x1="110" y1="40" x2="130" y2="60" stroke="color-mix(in srgb, var(--accent) 40%, transparent)" stroke-width="1"/>
            <line x1="50" y1="40" x2="70" y2="60" stroke="color-mix(in srgb, var(--accent) 40%, transparent)" stroke-width="1"/>
            <!-- Front face -->
            <line x1="70" y1="60" x2="130" y2="60" stroke="color-mix(in srgb, var(--accent) 75%, transparent)" stroke-width="1.5"/>
            <line x1="70" y1="60" x2="70" y2="120" stroke="color-mix(in srgb, var(--accent) 75%, transparent)" stroke-width="1.5"/>
            <line x1="130" y1="60" x2="130" y2="120" stroke="color-mix(in srgb, var(--accent) 75%, transparent)" stroke-width="1.5"/>
            <line x1="70" y1="120" x2="130" y2="120" stroke="color-mix(in srgb, var(--accent) 75%, transparent)" stroke-width="1.5"/>
            <!-- Bottom edges -->
            <line x1="50" y1="100" x2="70" y2="120" stroke="color-mix(in srgb, var(--accent) 55%, transparent)" stroke-width="1.2"/>
            <line x1="110" y1="100" x2="130" y2="120" stroke="color-mix(in srgb, var(--accent) 55%, transparent)" stroke-width="1.2"/>
            <line x1="50" y1="100" x2="110" y2="100" stroke="color-mix(in srgb, var(--accent) 40%, transparent)" stroke-width="1"/>
            <!-- Vertices -->
            {#each [
              [50,40],[110,40],[50,100],[110,100],
              [70,60],[130,60],[70,120],[130,120],
            ] as [vx, vy]}
              <circle cx={vx} cy={vy} r="2.5" fill="var(--accent)" opacity="0.7"/>
            {/each}
          </svg>
          <!-- Axis indicator -->
          <div class="axis-indicator">
            <svg width="28" height="28" viewBox="0 0 28 28">
              <line x1="14" y1="14" x2="24" y2="8"  stroke="#e55" stroke-width="1.5"/>
              <line x1="14" y1="14" x2="4"  y2="8"  stroke="#5b5" stroke-width="1.5"/>
              <line x1="14" y1="14" x2="14" y2="24" stroke="#55e" stroke-width="1.5"/>
              <text x="25" y="9"  font-size="6" fill="#e55" font-family="monospace">X</text>
              <text x="1"  y="9"  font-size="6" fill="#5b5" font-family="monospace">Y</text>
              <text x="12" y="27" font-size="6" fill="#55e" font-family="monospace">Z</text>
            </svg>
          </div>
        </div>
        <!-- Mode toolbar -->
        <div class="viewer-toolbar">
          <button class="icon-btn" title="Orbit mode">⟳</button>
          <button class="icon-btn" title="Pan mode">✥</button>
          <button class="icon-btn" title="Zoom mode">⊕</button>
          <div class="toolbar-spacer"></div>
          <button class="text-btn" title="Reset camera position">Reset View</button>
        </div>
      </div>
    </Card>
  </div>

  <!-- Row 2: PDF viewer + Audio player -->
  <div class="cols">
    <!-- PRS-3: PDF Viewer -->
    <Card id="PRS-3" label="PDF Viewer">
      <div class="pdf-shell" style="width:340px">
        <!-- Top toolbar -->
        <div class="viewer-toolbar">
          <button class="icon-btn" onclick={prevPage} title="Previous page" disabled={pdfPage <= 1}>‹</button>
          <span class="zoom-label">Page {pdfPage} of {pdfPageCount}</span>
          <button class="icon-btn" onclick={nextPage} title="Next page" disabled={pdfPage >= pdfPageCount}>›</button>
          <div class="tb-sep"></div>
          <button class="icon-btn" title="Zoom out">−</button>
          <span class="zoom-label">100%</span>
          <button class="icon-btn" title="Zoom in">+</button>
        </div>

        <!-- Page area -->
        <div class="pdf-page-area">
          <div class="pdf-page">
            <!-- Heading -->
            <div class="pdf-heading"></div>
            <!-- Body lines -->
            {#each lineWidths as w, i}
              <div
                class="text-line"
                style="width: {w}%; opacity: {i % 5 === 4 ? 0.3 : 0.55};"
              ></div>
            {/each}
          </div>
        </div>
      </div>
    </Card>

    <!-- PRS-4: Audio Player -->
    <Card id="PRS-4" label="Audio Player" sourceFile="common-js/src/components/Transport.svelte">
      <div class="audio-shell" style="width:400px">
        <div class="audio-row">
          <!-- Audio icon -->
          <div class="audio-icon">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
              <rect width="32" height="32" rx="6" fill="color-mix(in srgb, var(--accent) 15%, transparent)"/>
              <!-- Waveform bars -->
              <rect x="5"  y="14" width="2.5" height="4"  rx="1.2" fill="var(--accent)" opacity="0.6"/>
              <rect x="9"  y="10" width="2.5" height="12" rx="1.2" fill="var(--accent)" opacity="0.8"/>
              <rect x="13" y="7"  width="2.5" height="18" rx="1.2" fill="var(--accent)"/>
              <rect x="17" y="11" width="2.5" height="10" rx="1.2" fill="var(--accent)" opacity="0.8"/>
              <rect x="21" y="13" width="2.5" height="6"  rx="1.2" fill="var(--accent)" opacity="0.6"/>
              <rect x="25" y="15" width="2.5" height="2"  rx="1.2" fill="var(--accent)" opacity="0.4"/>
            </svg>
          </div>

          <!-- File info -->
          <div class="audio-meta">
            <span class="audio-title">ambient_loop.flac</span>
            <span class="audio-sub">Unknown · FLAC 24-bit 96kHz</span>
          </div>

          <!-- Volume control -->
          <div class="vol-group">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" style="color: var(--text-muted); flex-shrink:0;">
              <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3c0-1.77-1.02-3.29-2.5-4.03v8.05c1.48-.73 2.5-2.25 2.5-4.02z"/>
            </svg>
            <input type="range" min="0" max="100" bind:value={audioVolume} class="vol-slider" title="Volume"/>
          </div>
        </div>

        <!-- Transport + Timecode -->
        <div class="audio-controls">
          <Transport
            {playing}
            {playbackRate}
            onTogglePlay={togglePlay}
            onRewind={rewind}
            onForward={forward}
            onSkipStart={skipStart}
            onSkipEnd={skipEnd}
          />
          <Timecode {time} {duration} fps={1} />
        </div>
      </div>
    </Card>
  </div>

  <!-- Row 3: Shortcuts overlay -->
  <div class="row">
    <Card id="PRS-5" label="Shortcuts Overlay">
      <div class="shortcuts-shell" style="width:560px">
        <!-- Header -->
        <div class="shortcuts-header">
          <span class="shortcuts-title">Keyboard Shortcuts</span>
          <button class="icon-btn" title="Close">×</button>
        </div>

        <!-- 3-column grid -->
        <div class="shortcuts-grid">
          <!-- Navigation group -->
          <div class="shortcut-group">
            <div class="group-header">Navigation</div>
            {#each [
              ['Space', 'Play / Pause'],
              ['←', 'Prev frame'],
              ['→', 'Next frame'],
              ['J', 'Rewind'],
              ['L', 'Forward'],
              ['Home', 'First frame'],
              ['End', 'Last frame'],
            ] as [key, desc]}
              <div class="shortcut-row">
                <kbd class="kbd">{key}</kbd>
                <span class="shortcut-desc">{desc}</span>
              </div>
            {/each}
          </div>

          <!-- View group -->
          <div class="shortcut-group">
            <div class="group-header">View</div>
            {#each [
              ['+', 'Zoom in'],
              ['−', 'Zoom out'],
              ['0', 'Fit to window'],
              ['R', 'Rotate 90°'],
              ['F', 'Fullscreen'],
              ['I', 'Toggle info'],
            ] as [key, desc]}
              <div class="shortcut-row">
                <kbd class="kbd">{key}</kbd>
                <span class="shortcut-desc">{desc}</span>
              </div>
            {/each}
          </div>

          <!-- Files group -->
          <div class="shortcut-group">
            <div class="group-header">Files</div>
            {#each [
              ['O', 'Open file'],
              ['⌘←', 'Prev file'],
              ['⌘→', 'Next file'],
              ['Del', 'Move to trash'],
            ] as [key, desc]}
              <div class="shortcut-row">
                <kbd class="kbd">{key}</kbd>
                <span class="shortcut-desc">{desc}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    </Card>
  </div>

</div>

<style>
  .section {
    max-width: 1375px;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .row { width: 100%; }
  .cols { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── Shared viewer chrome ── */
  .viewer-shell,
  .model-shell,
  .pdf-shell,
  .audio-shell,
  .shortcuts-shell {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--surface);
  }

  .viewer-toolbar {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 4px 8px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-raised);
    min-height: 32px;
    flex-shrink: 0;
  }

  .viewer-toolbar:last-child {
    border-bottom: none;
    border-top: 1px solid var(--border);
  }

  .toolbar-spacer { flex: 1; }

  .filename {
    font-size: 12px;
    color: var(--text-secondary);
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .icon-btn {
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 14px;
    padding: 0;
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .icon-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
    color: var(--text-primary);
  }

  .icon-btn:disabled {
    opacity: 0.35;
    cursor: default;
  }

  .text-btn {
    height: 24px;
    padding: 0 10px;
    background: transparent;
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 11px;
    font-family: inherit;
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .text-btn:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
    color: var(--text-primary);
  }

  .zoom-label {
    font-size: 11px;
    color: var(--text-muted);
    min-width: 36px;
    text-align: center;
    white-space: nowrap;
  }

  .tb-sep {
    width: 1px;
    height: 14px;
    background: var(--border);
    margin: 0 4px;
    flex-shrink: 0;
  }

  /* ── PRS-1 Image viewer ── */
  .viewer-area {
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    flex: 1;
  }

  .viewer-image-bg {
    background: color-mix(in srgb, var(--surface) 40%, #000);
  }

  /* ── PRS-2 3D viewer ── */
  .model-viewport {
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--surface) 30%, #000);
    border-radius: 0;
    position: relative;
    overflow: hidden;
    flex: 1;
  }

  .axis-indicator {
    position: absolute;
    bottom: 10px;
    left: 10px;
    opacity: 0.8;
  }

  /* ── PRS-3 PDF viewer ── */
  .pdf-page-area {
    flex: 1;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: 16px 12px;
    background: color-mix(in srgb, var(--surface) 60%, var(--border));
    overflow-y: auto;
  }

  .pdf-page {
    background: var(--surface);
    border-radius: 2px;
    box-shadow: 0 1px 6px rgb(0 0 0 / 15%);
    padding: 24px 28px;
    width: 260px;
    min-height: 340px;
    display: flex;
    flex-direction: column;
    gap: 0;
  }

  .pdf-heading {
    height: 10px;
    background: var(--text-muted);
    border-radius: 2px;
    opacity: 0.5;
    margin-bottom: 16px;
    width: 70%;
  }

  .text-line {
    height: 6px;
    background: var(--border);
    border-radius: 2px;
    margin-bottom: 10px;
    flex-shrink: 0;
  }

  /* ── PRS-4 Audio player ── */
  .audio-row {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 14px 8px;
  }

  .audio-icon { flex-shrink: 0; }

  .audio-meta {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    overflow: hidden;
  }

  .audio-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .audio-sub {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  .vol-group {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .vol-slider {
    width: 80px;
    accent-color: var(--accent);
  }

  .audio-controls {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 0 14px 12px;
    flex-wrap: wrap;
  }

  /* ── PRS-5 Shortcuts overlay ── */
  .shortcuts-header {
    display: flex;
    align-items: center;
    padding: 12px 16px 10px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-raised);
  }

  .shortcuts-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    flex: 1;
  }

  .shortcuts-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0;
    padding: 16px;
  }

  .shortcut-group {
    display: flex;
    flex-direction: column;
    gap: 0;
    padding: 0 12px;
    border-right: 1px solid var(--border-subtle);
  }

  .shortcut-group:last-child {
    border-right: none;
  }

  .shortcut-group:first-child {
    padding-left: 0;
  }

  .group-header {
    font-size: 9px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-muted);
    padding-bottom: 8px;
    margin-bottom: 4px;
    border-bottom: 1px solid var(--border);
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 5px 0;
  }

  .kbd {
    display: inline-block;
    padding: 2px 6px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    font-family: ui-monospace, monospace;
    font-size: 11px;
    color: var(--text-secondary);
    white-space: nowrap;
    min-width: 28px;
    text-align: center;
    flex-shrink: 0;
  }

  .shortcut-desc {
    font-size: 12px;
    color: var(--text-secondary);
    white-space: nowrap;
  }
</style>
