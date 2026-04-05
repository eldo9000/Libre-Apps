<script>
  import { onMount } from 'svelte';

  let { path, ext, streamUrl } = $props();

  let videoEl = $state(null);
  let loaded = $state(false);
  let playing = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1.0);
  let muted = $state(false);
  let showControls = $state(true);
  let playbackRate = $state(1);
  let isFullscreen = $state(false);
  let showVolumeSlider = $state(false);
  let showSpeedMenu = $state(false);
  let controlsTimer = null;

  const SPEEDS = [0.5, 1, 1.5, 2];

  function formatTime(secs) {
    if (!isFinite(secs)) return '0:00';
    const s = Math.floor(secs);
    const h = Math.floor(s / 3600);
    const m = Math.floor((s % 3600) / 60);
    const sec = s % 60;
    if (h > 0) {
      return `${h}:${String(m).padStart(2, '0')}:${String(sec).padStart(2, '0')}`;
    }
    return `${m}:${String(sec).padStart(2, '0')}`;
  }

  function togglePlay() {
    if (!videoEl) return;
    if (videoEl.paused) {
      videoEl.play();
    } else {
      videoEl.pause();
    }
  }

  function seek(e) {
    if (!videoEl || !duration) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const pct = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    videoEl.currentTime = pct * duration;
  }

  function setVolume(e) {
    if (!videoEl) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const pct = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    volume = pct;
    videoEl.volume = pct;
    muted = pct === 0;
    videoEl.muted = muted;
  }

  function toggleMute() {
    if (!videoEl) return;
    muted = !muted;
    videoEl.muted = muted;
  }

  function setSpeed(s) {
    playbackRate = s;
    if (videoEl) videoEl.playbackRate = s;
    showSpeedMenu = false;
  }

  function stepFrame(dir) {
    if (!videoEl) return;
    videoEl.currentTime = Math.max(0, Math.min(duration, videoEl.currentTime + dir * (1 / 30)));
  }

  function toggleFullscreen() {
    if (!document.fullscreenElement) {
      document.documentElement.requestFullscreen();
    } else {
      document.exitFullscreen();
    }
  }

  function resetControlsTimer() {
    showControls = true;
    clearTimeout(controlsTimer);
    controlsTimer = setTimeout(() => {
      if (playing) showControls = false;
    }, 2500);
  }

  function handleKeydown(e) {
    switch (e.key) {
      case ' ':
        e.preventDefault();
        togglePlay();
        break;
      case 'ArrowLeft':
        e.preventDefault();
        if (videoEl) videoEl.currentTime = Math.max(0, videoEl.currentTime - 5);
        break;
      case 'ArrowRight':
        e.preventDefault();
        if (videoEl) videoEl.currentTime = Math.min(duration, videoEl.currentTime + 5);
        break;
      case ',':
        e.preventDefault();
        stepFrame(-1);
        break;
      case '.':
        e.preventDefault();
        stepFrame(1);
        break;
      case 'm':
      case 'M':
        e.preventDefault();
        toggleMute();
        break;
      case 'f':
      case 'F':
        e.preventDefault();
        toggleFullscreen();
        break;
      case 'ArrowUp':
        e.preventDefault();
        if (videoEl) {
          volume = Math.min(1, volume + 0.1);
          videoEl.volume = volume;
        }
        break;
      case 'ArrowDown':
        e.preventDefault();
        if (videoEl) {
          volume = Math.max(0, volume - 0.1);
          videoEl.volume = volume;
        }
        break;
    }
  }

  let progressPct = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);
  let volumePct = $derived(muted ? 0 : volume * 100);
</script>

<svelte:window onkeydown={handleKeydown} />

<div
  class="relative flex flex-col h-full bg-black"
  onmousemove={resetControlsTimer}
  onmouseenter={resetControlsTimer}
  role="application"
>
  <!-- Video element -->
  <!-- svelte-ignore a11y_media_has_caption -->
  <video
    bind:this={videoEl}
    src={streamUrl(path)}
    class="w-full h-full object-contain"
    onloadedmetadata={() => {
      loaded = true;
      duration = videoEl.duration;
      volume = videoEl.volume;
    }}
    ontimeupdate={() => { currentTime = videoEl.currentTime; }}
    onplay={() => { playing = true; }}
    onpause={() => { playing = false; showControls = true; clearTimeout(controlsTimer); }}
    onvolumechange={() => {
      volume = videoEl.volume;
      muted = videoEl.muted;
    }}
    onfullscreenchange={() => { isFullscreen = !!document.fullscreenElement; }}
  ></video>

  <!-- Loading spinner -->
  {#if !loaded}
    <div class="absolute inset-0 flex items-center justify-center bg-black">
      <div class="flex flex-col items-center gap-3">
        <div class="w-8 h-8 rounded-full border-2 border-gray-700 border-t-[#0066cc] animate-spin"></div>
        <span class="text-gray-400 text-sm">Loading video…</span>
      </div>
    </div>
  {/if}

  <!-- Controls overlay -->
  {#if loaded}
    <!-- Click-to-play overlay on video -->
    <button
      onclick={togglePlay}
      class="absolute inset-0 w-full h-full bg-transparent cursor-pointer"
      aria-label="Toggle play"
      style="padding-bottom: 72px;"
    ></button>

    <!-- Bottom controls bar -->
    <div
      class="absolute bottom-0 left-0 right-0 px-4 pb-3 pt-8 bg-gradient-to-t from-black/80 to-transparent transition-opacity duration-200"
      style="opacity: {showControls ? 1 : 0};"
    >
      <!-- Progress bar -->
      <div class="mb-3">
        <button
          class="w-full h-1.5 bg-white/20 rounded-full relative cursor-pointer group"
          onclick={seek}
          aria-label="Seek"
        >
          <div
            class="h-full bg-[#0066cc] rounded-full pointer-events-none"
            style="width: {progressPct}%"
          ></div>
          <div
            class="absolute top-1/2 -translate-y-1/2 w-3 h-3 bg-white rounded-full shadow opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none"
            style="left: calc({progressPct}% - 6px)"
          ></div>
        </button>
      </div>

      <!-- Controls row -->
      <div class="flex items-center gap-2">
        <!-- Frame step back -->
        <button
          onclick={() => stepFrame(-1)}
          class="w-7 h-7 flex items-center justify-center text-white/70 hover:text-white transition-colors"
          title="Step back 1 frame (,)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 6h2v12H6zm3.5 6 8.5 6V6z"/>
          </svg>
        </button>

        <!-- Play/Pause -->
        <button
          onclick={togglePlay}
          class="w-9 h-9 flex items-center justify-center text-white hover:text-white/80 transition-colors"
          title="Play/Pause (Space)"
        >
          {#if playing}
            <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
              <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
            </svg>
          {:else}
            <svg width="22" height="22" viewBox="0 0 24 24" fill="currentColor">
              <path d="M8 5v14l11-7z"/>
            </svg>
          {/if}
        </button>

        <!-- Frame step forward -->
        <button
          onclick={() => stepFrame(1)}
          class="w-7 h-7 flex items-center justify-center text-white/70 hover:text-white transition-colors"
          title="Step forward 1 frame (.)"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/>
          </svg>
        </button>

        <!-- Time display -->
        <span class="text-white/80 text-xs tabular-nums ml-1">
          {formatTime(currentTime)} / {formatTime(duration)}
        </span>

        <!-- Spacer -->
        <div class="flex-1"></div>

        <!-- Volume -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          role="group"
          aria-label="Volume"
          class="flex items-center gap-1 relative"
          onmouseenter={() => { showVolumeSlider = true; }}
          onmouseleave={() => { showVolumeSlider = false; }}
        >
          <button
            onclick={toggleMute}
            class="w-7 h-7 flex items-center justify-center text-white/70 hover:text-white transition-colors"
            title="Mute (M)"
          >
            {#if muted || volume === 0}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M16.5 12A4.5 4.5 0 0 0 14 7.97v2.21l2.45 2.45c.03-.2.05-.41.05-.63zm2.5 0c0 .94-.2 1.82-.54 2.64l1.51 1.51A8.82 8.82 0 0 0 21 12c0-4.28-2.99-7.86-7-8.77v2.06c2.89.86 5 3.54 5 6.71zM4.27 3L3 4.27 7.73 9H3v6h4l5 5v-6.73l4.25 4.25c-.67.52-1.42.93-2.25 1.18v2.06A8.99 8.99 0 0 0 17.73 18l1.99 2L21 18.73 4.27 3zM12 4L9.91 6.09 12 8.18V4z"/>
              </svg>
            {:else if volume < 0.5}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M18.5 12A4.5 4.5 0 0 0 16 7.97v8.05c1.48-.73 2.5-2.25 2.5-4.02zM5 9v6h4l5 5V4L9 9H5z"/>
              </svg>
            {:else}
              <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 7.97v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/>
              </svg>
            {/if}
          </button>
          {#if showVolumeSlider}
            <button
              class="w-20 h-1.5 bg-white/20 rounded-full cursor-pointer group relative"
              onclick={setVolume}
              aria-label="Volume"
            >
              <div
                class="h-full bg-white rounded-full pointer-events-none"
                style="width: {volumePct}%"
              ></div>
            </button>
          {/if}
        </div>

        <!-- Speed selector -->
        <div class="relative">
          <button
            onclick={() => { showSpeedMenu = !showSpeedMenu; }}
            class="px-2 h-6 flex items-center text-white/70 hover:text-white text-xs rounded border border-white/20 hover:border-white/40 transition-colors"
            title="Playback speed"
          >{playbackRate}×</button>
          {#if showSpeedMenu}
            <div class="absolute bottom-8 right-0 bg-gray-900 border border-gray-700 rounded-lg py-1 shadow-xl z-10">
              {#each SPEEDS as s}
                <button
                  onclick={() => setSpeed(s)}
                  class="w-full px-4 py-1.5 text-left text-sm hover:bg-gray-700 transition-colors {s === playbackRate ? 'text-[#0066cc]' : 'text-gray-200'}"
                >{s}×</button>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Fullscreen -->
        <button
          onclick={toggleFullscreen}
          class="w-7 h-7 flex items-center justify-center text-white/70 hover:text-white transition-colors"
          title="Fullscreen (F)"
        >
          {#if isFullscreen}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M5 16h3v3h2v-5H5v2zm3-8H5v2h5V5H8v3zm6 11h2v-3h3v-2h-5v5zm2-11V5h-2v5h5V8h-3z"/>
            </svg>
          {:else}
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z"/>
            </svg>
          {/if}
        </button>
      </div>
    </div>
  {/if}
</div>
