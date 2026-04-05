<script>
  import { onMount } from 'svelte';

  let { path, ext, name, size, streamUrl } = $props();

  let audioEl = $state(null);
  let loaded = $state(false);
  let playing = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1.0);

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

  function formatSize(bytes) {
    if (!bytes) return '';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }

  function togglePlay() {
    if (!audioEl) return;
    if (audioEl.paused) {
      audioEl.play();
    } else {
      audioEl.pause();
    }
  }

  function seek(e) {
    if (!audioEl || !duration) return;
    const rect = e.currentTarget.getBoundingClientRect();
    const pct = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
    audioEl.currentTime = pct * duration;
  }

  function handleVolumeInput(e) {
    volume = parseFloat(e.currentTarget.value);
    if (audioEl) audioEl.volume = volume;
  }

  function handleKeydown(e) {
    switch (e.key) {
      case ' ':
        e.preventDefault();
        togglePlay();
        break;
      case 'ArrowLeft':
        e.preventDefault();
        if (audioEl) audioEl.currentTime = Math.max(0, audioEl.currentTime - 5);
        break;
      case 'ArrowRight':
        e.preventDefault();
        if (audioEl) audioEl.currentTime = Math.min(duration, audioEl.currentTime + 5);
        break;
      case 'ArrowUp':
        e.preventDefault();
        volume = Math.min(1, volume + 0.1);
        if (audioEl) audioEl.volume = volume;
        break;
      case 'ArrowDown':
        e.preventDefault();
        volume = Math.max(0, volume - 0.1);
        if (audioEl) audioEl.volume = volume;
        break;
    }
  }

  let progressPct = $derived(duration > 0 ? (currentTime / duration) * 100 : 0);
</script>

<svelte:window onkeydown={handleKeydown} />

<!-- Hidden audio element -->
<audio
  bind:this={audioEl}
  src={streamUrl(path)}
  onloadedmetadata={() => { loaded = true; duration = audioEl.duration; }}
  ontimeupdate={() => { currentTime = audioEl.currentTime; }}
  onplay={() => { playing = true; }}
  onpause={() => { playing = false; }}
></audio>

<div class="flex flex-col h-full bg-gray-900 items-center justify-center px-8">
  <div class="flex flex-col items-center gap-6 w-full max-w-sm">

    <!-- Audio icon -->
    <div class="w-24 h-24 rounded-2xl bg-gray-800 flex items-center justify-center border border-gray-700">
      <svg width="52" height="52" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="text-[#0066cc]">
        <path d="M9 18V5l12-2v13"/>
        <circle cx="6" cy="18" r="3"/>
        <circle cx="18" cy="16" r="3"/>
      </svg>
    </div>

    <!-- File name + ext badge -->
    <div class="flex flex-col items-center gap-2 text-center">
      <span class="text-gray-100 text-xl font-semibold truncate max-w-full" title={name}>{name}</span>
      <div class="flex items-center gap-2">
        {#if ext}
          <span class="px-2 py-0.5 rounded bg-gray-700 text-gray-400 text-xs uppercase tracking-wide">{ext}</span>
        {/if}
        {#if size}
          <span class="text-gray-500 text-xs">{formatSize(size)}</span>
        {/if}
      </div>
    </div>

    <!-- Progress bar -->
    <div class="w-full flex flex-col gap-1.5">
      <button
        class="w-full h-1.5 bg-gray-700 rounded-full relative cursor-pointer group"
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
      <div class="flex justify-between text-gray-500 text-xs tabular-nums">
        <span>{formatTime(currentTime)}</span>
        <span>{formatTime(duration)}</span>
      </div>
    </div>

    <!-- Play/Pause button -->
    <button
      onclick={togglePlay}
      class="w-16 h-16 rounded-full bg-[#0066cc] hover:bg-blue-500 flex items-center justify-center transition-colors shadow-lg shadow-blue-900/30"
      title="Play/Pause (Space)"
    >
      {#if playing}
        <svg width="26" height="26" viewBox="0 0 24 24" fill="white">
          <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
        </svg>
      {:else}
        <svg width="26" height="26" viewBox="0 0 24 24" fill="white" style="margin-left: 3px;">
          <path d="M8 5v14l11-7z"/>
        </svg>
      {/if}
    </button>

    <!-- Volume slider -->
    <div class="w-full flex items-center gap-2">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="text-gray-500 shrink-0">
        <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 7.97v8.05c1.48-.73 2.5-2.25 2.5-4.02z"/>
      </svg>
      <input
        type="range"
        min="0"
        max="1"
        step="0.01"
        value={volume}
        oninput={handleVolumeInput}
        class="flex-1 h-1.5 accent-[#0066cc] cursor-pointer"
        aria-label="Volume"
      />
      <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" class="text-gray-500 shrink-0">
        <path d="M3 9v6h4l5 5V4L7 9H3zm13.5 3A4.5 4.5 0 0 0 14 7.97v8.05c1.48-.73 2.5-2.25 2.5-4.02zM14 3.23v2.06c2.89.86 5 3.54 5 6.71s-2.11 5.85-5 6.71v2.06c4.01-.91 7-4.49 7-8.77s-2.99-7.86-7-8.77z"/>
      </svg>
    </div>

  </div>
</div>
