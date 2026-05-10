<script>
  import { Transport, Timecode } from '@libre/ui';
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

  onDestroy(() => cancelAnimationFrame(raf));
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
          showFullscreen
        />
        <p class="hint">
          Click play. Hit forward/back repeatedly to ramp shuttle speed
          (1×→8×). State is local to this card.
        </p>
      </div>
    </Card>

    <Card id="MED-2" label="Idle / stopped" sourceFile="common-js/src/components/Transport.svelte">
      <Transport playing={false} />
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
  .stack { display: flex; flex-direction: column; gap: 12px; }
  .hint {
    margin: 0;
    font-size: 12px;
    color: var(--text-muted);
    line-height: 1.5;
  }
</style>
