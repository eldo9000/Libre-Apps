<script>
  import { onDestroy } from 'svelte';

  /**
   * Transport — canonical media transport bar.
   *
   * Layout: [skip-start ⏪] — [play] — [⏩ skip-end | loop fullscreen]
   *
   * Play button: click → toggle play/pause. Drag left/right → jog (velocity-based,
   * returns to zero the moment the mouse stops moving).
   *
   * @typedef {{
   *   playing?: boolean,
   *   playbackRate?: number,
   *   onTogglePlay?: () => void,
   *   onSkipStart?: () => void,
   *   onSkipEnd?: () => void,
   *   onRewind?: () => void,
   *   onForward?: () => void,
   *   onJog?: (rate: number) => void,
   *   showLoop?: boolean,
   *   loop?: boolean,
   *   onToggleLoop?: () => void,
   *   showFullscreen?: boolean,
   *   fullscreen?: boolean,
   *   onToggleFullscreen?: () => void,
   * }} Props
   *
   * @type {Props}
   */
  let {
    playing = false,
    playbackRate = 1,
    onTogglePlay,
    onSkipStart,
    onSkipEnd,
    onRewind,
    onForward,
    onJog,
    showLoop = false,
    loop = false,
    onToggleLoop,
    showFullscreen = false,
    fullscreen = false,
    onToggleFullscreen,
    showRange = false,
    range = false,
    onToggleRange,
  } = $props();

  const bwdActive = $derived(playing && playbackRate < 0);
  const fwdActive = $derived(playing && playbackRate > 0);

  // Jog wheel state
  let jogging = $state(false);
  let jogRate = $state(0); // -8..8
  let _jogTrackVisible = $state(false);
  let _jogEnabled = false;
  // Single delay that gates ALL scrub behavior: cursor change, fade-on, scrub enable.
  const JOG_TRIGGER_DELAY_MS = 150;
  let _jogTriggerTimer = 0;
  // After this hold duration, releasing no longer toggles play/pause — separates hold-to-scrub from tap-to-play.
  const HOLD_LOCK_MS = 1000;
  let _holdLockTimer = 0;
  let _holdLocked = false;

  let _startX = 0;
  let _currentX = 0;
  let _lastX = 0;
  let _lastTime = 0;
  let _dragDelta = 0;
  let _rafId = 0;

  // Velocity-based jog: dx/dt * sensitivity → rate. Naturally returns to 0 when mouse stops.
  function _jogTick(now) {
    if (!jogging) return;
    const dx = _currentX - _lastX;
    const dt = now - _lastTime;
    if (dt > 0) {
      const rate = Math.max(-8, Math.min(8, (dx / dt) * 10));
      jogRate = rate;
      if (_jogEnabled) onJog?.(rate);
    }
    _lastX = _currentX;
    _lastTime = now;
    _rafId = requestAnimationFrame(_jogTick);
  }

  function _onJogMove(e) {
    _currentX = e.clientX;
    _dragDelta = Math.abs(e.clientX - _startX);
  }

  function _stopJog() {
    cancelAnimationFrame(_rafId);
    window.removeEventListener('mousemove', _onJogMove);
    window.removeEventListener('mouseup', _stopJog);
    clearTimeout(_jogTriggerTimer);
    clearTimeout(_holdLockTimer);
    _jogTriggerTimer = 0;
    _holdLockTimer = 0;
    _jogEnabled = false;
    document.body.classList.remove('transport-jogging');
    const wasDrag = _dragDelta >= 4;
    const wasHeld = _holdLocked;
    _holdLocked = false;
    jogging = false;
    jogRate = 0;
    _jogTrackVisible = false;
    if (!wasDrag && !wasHeld) {
      onTogglePlay?.();
    } else {
      onJog?.(0);
    }
  }

  function handlePlayMouseDown(e) {
    if (e.button !== 0) return;
    if (!onJog) {
      onTogglePlay?.();
      return;
    }
    e.preventDefault();
    _startX = e.clientX;
    _currentX = e.clientX;
    _lastX = e.clientX;
    _lastTime = performance.now();
    _dragDelta = 0;
    jogging = true;
    _jogTriggerTimer = setTimeout(() => {
      _jogEnabled = true;
      _jogTrackVisible = true;
      document.body.classList.add('transport-jogging');
    }, JOG_TRIGGER_DELAY_MS);
    _holdLockTimer = setTimeout(() => { _holdLocked = true; }, HOLD_LOCK_MS);
    _rafId = requestAnimationFrame(_jogTick);
    window.addEventListener('mousemove', _onJogMove);
    window.addEventListener('mouseup', _stopJog);
  }


  onDestroy(() => {
    cancelAnimationFrame(_rafId);
    clearTimeout(_jogTriggerTimer);
    clearTimeout(_holdLockTimer);
    document.body.classList.remove('transport-jogging');
    window.removeEventListener('mousemove', _onJogMove);
    window.removeEventListener('mouseup', _stopJog);
  });
</script>

<div class="transport">
  <!-- Left of play: skip-to-start, rewind -->
  <button
    class="t-btn"
    onclick={onSkipStart}
    title="Go to start"
    aria-label="Go to start"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M6 6h2v12H6zm3.5 6 8.5 6V6z"/>
    </svg>
  </button>

  <button
    class="t-btn"
    class:t-btn-active={bwdActive}
    onclick={onRewind}
    title="Rewind (J)"
    aria-label="Rewind"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M11 18V6l-8.5 6 8.5 6zm.5-6 8.5 6V6l-8.5 6z"/>
    </svg>
  </button>

  <!-- Play -->
  <div class="play-wrap">
    <div class="jog-track" class:jog-track-active={_jogTrackVisible}>
      <div class="jog-thumb" style="--pos: {jogRate / 8}"></div>
    </div>
    <button
      class="t-play"
      class:t-play-jogging={jogging}
      onmousedown={handlePlayMouseDown}
      title="Play / Pause (Space) — drag left/right to jog"
      aria-label={playing ? 'Pause' : 'Play'}
    >
      {#if playing}
        <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
        </svg>
      {:else}
        <svg width="12" height="12" viewBox="0 0 24 24" fill="currentColor">
          <path d="M8 5v14l11-7z"/>
        </svg>
      {/if}
    </button>
  </div>

  <!-- Right of play: fast-forward, skip-to-end, extras -->
  <button
    class="t-btn"
    class:t-btn-active={fwdActive}
    onclick={onForward}
    title="Fast forward (L)"
    aria-label="Fast forward"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M4 18l8.5-6L4 6v12zm9 0 8.5-6L13 6v12z"/>
    </svg>
  </button>

  <button
    class="t-btn"
    onclick={onSkipEnd}
    title="Go to end"
    aria-label="Go to end"
  >
    <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
      <path d="M6 18l8.5-6L6 6v12zm2.5-6 8.5 6V6zM16 6h2v12h-2z"/>
    </svg>
  </button>

  {#if showLoop || showFullscreen || showRange}
    <div class="t-divider"></div>

    {#if showLoop}
      <button
        class="t-btn"
        class:t-btn-active={loop}
        onclick={onToggleLoop}
        title="Loop"
        aria-label="Toggle loop"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="currentColor">
          <path d="M7 7h10v3l4-4-4-4v3H5v6h2V7zm10 10H7v-3l-4 4 4 4v-3h12v-6h-2v5z"/>
        </svg>
      </button>
    {/if}

    {#if showRange}
      <button
        class="t-btn"
        class:t-btn-active={range}
        onclick={onToggleRange}
        title="Playback range"
        aria-label="Toggle playback range"
      >
        <!-- in/out range markers: two vertical bars with a line between -->
        <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
          <rect x="3" y="7" width="2" height="10" rx="1"/>
          <rect x="19" y="7" width="2" height="10" rx="1"/>
          <rect x="5" y="11" width="14" height="2" rx="1"/>
        </svg>
      </button>
    {/if}

    {#if showFullscreen}
      <button
        class="t-btn"
        class:t-btn-active={fullscreen}
        onclick={onToggleFullscreen}
        title="Fullscreen (F)"
        aria-label="Toggle fullscreen"
      >
        <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"/>
        </svg>
      </button>
    {/if}
  {/if}
</div>

<style>
  :global(body.transport-jogging),
  :global(body.transport-jogging *) {
    cursor: ew-resize !important;
    user-select: none !important;
  }

  .transport {
    display: inline-flex;
    align-items: center;
    align-self: center;
    gap: 2px;
    padding: 0 8px;
    height: 40px;
    background: color-mix(in srgb, var(--surface) 70%, var(--surface-deep, black));
    border-top: 1px solid var(--border);
    border-bottom: 1px solid var(--border);
    box-sizing: border-box;
  }

  .t-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0;
    transition: background 0.1s, color 0.1s;
    flex-shrink: 0;
  }

  .t-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    color: var(--text-secondary);
  }

  .t-btn-active {
    color: var(--accent) !important;
  }

  /* Play button: centered via grid's auto column */
  .play-wrap {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .jog-track {
    position: absolute;
    bottom: calc(100% + 6px);
    left: 50%;
    transform: translateX(-50%);
    width: 80px;
    height: 3px;
    background: color-mix(in srgb, var(--text-primary) 15%, transparent);
    border-radius: 999px;
    opacity: 0;
    transition: opacity 150ms;  /* fade out */
    pointer-events: none;
  }

  .jog-track-active {
    opacity: 1;
    transition: opacity 500ms;  /* fade in */
  }

  .jog-thumb {
    position: absolute;
    width: 7px;
    height: 7px;
    background: var(--accent);
    border-radius: 999px;
    top: 50%;
    left: 50%;
    /* no transition — immediate response to mouse velocity */
    transform: translate(-50%, -50%) translateX(calc(var(--pos, 0) * 34px));
  }

  .t-play {
    width: 34px;
    height: 34px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 999px;
    cursor: pointer;
    padding: 0;
    transition: background 0.1s, box-shadow 0.12s;
  }

  .t-play:hover { background: var(--accent-hover); }

  .t-play-jogging {
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 40%, transparent);
  }

  .t-divider {
    width: 1px;
    height: 16px;
    background: var(--border);
    margin: 0 4px;
    flex-shrink: 0;
  }
</style>
