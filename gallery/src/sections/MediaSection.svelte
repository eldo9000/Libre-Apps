<script>
  import { Transport, Timecode } from '@libre/ui';
  import BezierEditor from '@libre/ui/src/components/BezierEditor.svelte';
  import Card from '../lib/Card.svelte';
  import { onDestroy } from 'svelte';

  // Mock playhead state — apps wire these to a real timeline store.
  let playing = $state(false);
  let playbackRate = $state(1);
  let time = $state(0);
  const duration = 187.42;
  const fps = 30;

  let raf = 0;
  let lastTs = 0;

  function tick(ts) {
    if (lastTs) {
      const dt = (ts - lastTs) / 1000;
      time = Math.max(0, Math.min(duration, time + dt * playbackRate));
      if (time >= duration && playbackRate > 0) {
        if (loop) { time = 0; }
        else { playing = false; playbackRate = 1; }
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
  function rewind()  {
    if (playing && playbackRate < 0) playbackRate = Math.max(playbackRate * 2, -8);
    else { playing = true; playbackRate = -1; startLoop(); }
  }
  function forward() {
    if (playing && playbackRate > 0) playbackRate = Math.min(playbackRate * 2, 8);
    else { playing = true; playbackRate = 1; startLoop(); }
  }
  function skipStart() { time = 0; playing = false; cancelAnimationFrame(raf); }
  function skipEnd()   { time = duration; playing = false; cancelAnimationFrame(raf); }

  let loop = $state(false);
  function toggleLoop() { loop = !loop; }

  let fullscreen = $state(false);
  function toggleFullscreen() { fullscreen = !fullscreen; }

  let range = $state(false);
  function toggleRange() { range = !range; }

  // Jog — Transport drives this via its own rAF; we just advance time on each call.
  let _jogActive = false;
  let _lastJogTime = 0;

  function onJog(rate) {
    const now = performance.now();
    if (rate !== 0) {
      if (!_jogActive) {
        _jogActive = true;
        playing = false;
        cancelAnimationFrame(raf);
        lastTs = 0;
      }
      if (_lastJogTime > 0) {
        const dt = (now - _lastJogTime) / 1000;
        time = Math.max(0, Math.min(duration, time + dt * rate));
      }
      _lastJogTime = now;
    } else {
      _jogActive = false;
      _lastJogTime = 0;
    }
  }

  onDestroy(() => {
    cancelAnimationFrame(raf);
    document.body.classList.remove('tl-seeking');
  });

  // ── Timeline ─────────────────────────────────────────────
  let tlEl = $state(null);

  function fmtTick(s) {
    const m = Math.floor(s / 60);
    const ss = Math.floor(s % 60);
    return `${m}:${ss.toString().padStart(2, '0')}`;
  }

  const tlTicks = (() => {
    const ticks = [];
    for (let t = 0; t <= duration; t += 5) {
      ticks.push({
        t,
        pct: (t / duration) * 100,
        major: t % 30 === 0,
        mid: t % 10 === 0 && t % 30 !== 0,
        label: fmtTick(t),
      });
    }
    return ticks;
  })();

  function tlSeek(e) {
    if (e.button !== 0) return;
    applySeek(e);
    document.body.classList.add('tl-seeking');
    const onMove = (me) => applySeek(me);
    const onUp   = () => {
      document.body.classList.remove('tl-seeking');
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
    };
    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
  }

  function applySeek(e) {
    if (!tlEl) return;
    const rect = tlEl.getBoundingClientRect();
    const frac = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    time = frac * duration;
  }

  // ── Bezier / localStorage ─────────────────────────────────
  const MEDIA_KEY = 'libre-media';
  function _loadMedia() {
    try { return JSON.parse(localStorage.getItem(MEDIA_KEY) ?? 'null'); } catch { return null; }
  }
  const _media = _loadMedia();

  let bezierValue = $state(_media?.bezierValue ?? [0.25, 0.46, 0.45, 0.94]);
  let bezierSaved = $state(_media?.bezierValue ?? [0.25, 0.46, 0.45, 0.94]);

  $effect(() => {
    localStorage.setItem(MEDIA_KEY, JSON.stringify({ bezierValue: [...bezierValue] }));
  });
</script>

<div class="section">
  <h2 class="group-title">Transport</h2>
  <div class="grid">
    <Card id="MED-1" label="Live transport (mock playhead)" sourceFile="common-js/src/components/Transport.svelte">
      <div class="stack">
        <Transport
          {playing}
          {playbackRate}
          onTogglePlay={togglePlay}
          onRewind={rewind}
          onForward={forward}
          onSkipStart={skipStart}
          onSkipEnd={skipEnd}
          {onJog}
          showLoop
          {loop}
          onToggleLoop={toggleLoop}
          showRange
          {range}
          onToggleRange={toggleRange}
          showFullscreen
          {fullscreen}
          onToggleFullscreen={toggleFullscreen}
        />
        <ul class="hint-list">
          <li>Click play — toggle play / pause</li>
          <li>⏪ / ⏩ — shuttle ramp (1×→8×)</li>
          <li>Loop — repeat from start when playback ends</li>
          <li>Drag play button left/right — jog; speed follows drag velocity, stops instantly when mouse stops</li>
        </ul>
      </div>
    </Card>

  </div>

  <Card id="MED-7" label="Timeline" sourceFile="common-js/src/components/Transport.svelte" flush>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="timeline" bind:this={tlEl} onmousedown={tlSeek}>
      <!-- ruler -->
      <div class="tl-ruler">
        {#each tlTicks as tk}
          <div
            class="tl-tick"
            class:tl-major={tk.major}
            class:tl-mid={tk.mid}
            style="left: {tk.pct}%"
          >
            {#if tk.major && tk.pct < 98}
              <span class="tl-label">{tk.label}</span>
            {/if}
          </div>
        {/each}
      </div>
      <!-- track + progress fill -->
      <div class="tl-track">
        <div class="tl-fill" style="width: {(time / duration) * 100}%"></div>
      </div>
      <!-- playhead -->
      <div class="tl-playhead" style="left: {(time / duration) * 100}%">
        <div class="tl-ph-handle"></div>
        <div class="tl-ph-line"></div>
      </div>
    </div>
  </Card>

  <h2 class="group-title">Bezier Editor</h2>
  <div class="grid bezier-grid">
    <Card id="MED-6" label="Bezier Editor" sourceFile="common-js/src/components/BezierEditor.svelte">
      <BezierEditor
        bind:value={bezierValue}
        bind:savedValue={bezierSaved}
      />
    </Card>
  </div>

  <h2 class="group-title">Timecode</h2>
  <div class="grid">
    <Card id="MED-3" label="Live (advances with transport above)" sourceFile="common-js/src/components/Timecode.svelte">
      <Timecode {time} {duration} {fps} />
    </Card>
    <Card id="MED-4" label="Static — 00:01:23:14 / 03:07:25" sourceFile="common-js/src/components/Timecode.svelte">
      <Timecode time={83.467} duration={187.42} {fps} />
    </Card>
    <Card id="MED-5" label="Zeroed — all digits dim" sourceFile="common-js/src/components/Timecode.svelte">
      <Timecode time={0} duration={0} {fps} />
    </Card>
  </div>
</div>

<style>
  .section { display: flex; flex-direction: column; gap: 24px; }
  .group-title {
    font-size: 14px;
    font-weight: 600;
    letter-spacing: -0.01em;
    color: var(--text-primary);
    margin: 8px 0 0;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 16px;
  }
  .bezier-grid {
    grid-template-columns: minmax(460px, 1fr);
  }
  .stack { display: flex; flex-direction: column; gap: 12px; }
  .hint-list {
    margin: 0;
    padding-left: 16px;
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.7;
  }

  /* ── Timeline ─────────────────────────────────────── */
  :global(body.tl-seeking) {
    cursor: col-resize !important;
    user-select: none !important;
  }

  .timeline {
    position: relative;
    width: 100%;
    height: 64px;
    background: color-mix(in srgb, var(--surface) 70%, var(--surface-deep, black));
    cursor: col-resize;
    user-select: none;
    overflow: hidden;
    border-radius: 0 0 10px 10px;
  }

  /* Ruler: top 30px, contains ticks + labels */
  .tl-ruler {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    height: 30px;
    border-bottom: 1px solid var(--border);
  }

  .tl-tick {
    position: absolute;
    top: 0;
    width: 1px;
    height: 5px;
    background: color-mix(in srgb, var(--text-muted) 25%, transparent);
    pointer-events: none;
  }

  .tl-mid {
    height: 9px;
    background: color-mix(in srgb, var(--text-muted) 45%, transparent);
  }

  .tl-major {
    height: 15px;
    background: color-mix(in srgb, var(--text-muted) 70%, transparent);
  }

  .tl-label {
    position: absolute;
    top: 17px;
    left: 4px;
    font-size: 9px;
    font-family: 'Geist Mono', 'Fira Code', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    white-space: nowrap;
    pointer-events: none;
    letter-spacing: 0.02em;
  }

  /* Track: bottom 34px with progress fill */
  .tl-track {
    position: absolute;
    top: 30px;
    left: 0;
    right: 0;
    bottom: 0;
    background: color-mix(in srgb, var(--surface) 55%, var(--surface-deep, black));
    overflow: hidden;
  }

  .tl-fill {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    background: color-mix(in srgb, var(--accent) 20%, transparent);
    pointer-events: none;
    /* no transition — follows mouse/playhead in real time */
  }

  /* Playhead: accent line + downward-triangle handle */
  .tl-playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    pointer-events: none;
    z-index: 4;
    transform: translateX(-50%);
  }

  .tl-ph-handle {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 0;
    height: 0;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 8px solid var(--accent);
  }

  .tl-ph-line {
    position: absolute;
    top: 0;
    bottom: 0;
    left: 50%;
    width: 1px;
    transform: translateX(-50%);
    background: var(--accent);
    opacity: 0.85;
  }
</style>
