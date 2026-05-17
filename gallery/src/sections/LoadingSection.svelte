<script>
  import Card from '../lib/Card.svelte';

  // Skeleton list
  let listState = $state('idle'); // idle | loading | loaded
  function loadList() {
    if (listState !== 'idle') return;
    listState = 'loading';
    setTimeout(() => { listState = 'loaded'; }, 1800);
  }
  function resetList() { listState = 'idle'; }

  // Skeleton thumbnails
  let thumbState = $state('idle');
  function loadThumbs() {
    if (thumbState !== 'idle') return;
    thumbState = 'loading';
    setTimeout(() => { thumbState = 'loaded'; }, 2200);
  }
  function resetThumbs() { thumbState = 'idle'; }

  // Inline loading toast
  let toastState = $state('idle'); // idle | loading | success | error
  let toastTimer = null;
  function triggerToast(outcome) {
    if (toastState === 'loading') return;
    if (toastTimer) clearTimeout(toastTimer);
    toastState = 'loading';
    toastTimer = setTimeout(() => {
      toastState = outcome;
      toastTimer = setTimeout(() => { toastState = 'idle'; }, 2500);
    }, 1800);
  }

  // Button shimmer
  let btnLoad = $state(false);
  function triggerBtn() {
    if (btnLoad) return;
    btnLoad = true;
    setTimeout(() => { btnLoad = false; }, 2500);
  }

  const listItems = [
    { name: 'Introduction.md',  meta: '4.2 KB · doc'  },
    { name: 'Architecture.png', meta: '82 KB · image'  },
    { name: 'Notes.txt',        meta: '1.1 KB · text'  },
    { name: 'Dataset.csv',      meta: '14 KB · data'   },
    { name: 'Preview.mp4',      meta: '2.4 MB · video' },
  ];

  const thumbItems = [
    'beach.jpg', 'city.png', 'logo.svg',
    'portrait.jpg', 'banner.png', 'icon.svg',
  ];
</script>

<div class="section">

  <!-- ── Bars ──────────────────────────────────────────────────── -->
  <h2 class="group-title">Bars</h2>
  <div class="grid">
    <Card id="LOAD-1" label="Indeterminate — Sweep">
      <div class="bar-demo">
        <div class="bar-track"><div class="bar-sweep"></div></div>
      </div>
    </Card>
    <Card id="LOAD-2" label="Indeterminate — Bounce">
      <div class="bar-demo">
        <div class="bar-track"><div class="bar-bounce"></div></div>
      </div>
    </Card>
    <Card id="LOAD-3" label="Indeterminate — Pulse">
      <div class="bar-demo">
        <div class="bar-track bar-track-pulse"><div class="bar-pulse-fill"></div></div>
      </div>
    </Card>
  </div>

  <!-- ── Spinners ───────────────────────────────────────────────── -->
  <h2 class="group-title">Spinners</h2>
  <div class="grid">
    <Card id="LOAD-4" label="Border spin — sm / md / lg">
      <div class="spinner-row">
        <div class="spin spin-sm"></div>
        <div class="spin spin-md"></div>
        <div class="spin spin-lg"></div>
      </div>
    </Card>
    <Card id="LOAD-5" label="Dot pulse">
      <div class="dots-row">
        <div class="dot"></div>
        <div class="dot"></div>
        <div class="dot"></div>
      </div>
    </Card>
    <Card id="LOAD-6" label="Arc ring">
      <div class="arc-wrap">
        <svg class="arc-ring" width="32" height="32" viewBox="0 0 32 32">
          <circle cx="16" cy="16" r="12" fill="none" stroke="var(--border)" stroke-width="2.5"/>
          <circle
            class="arc-indicator"
            cx="16" cy="16" r="12"
            fill="none"
            stroke="var(--accent)"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-dasharray="38 38"
          />
        </svg>
      </div>
    </Card>
  </div>

  <!-- ── Skeleton Panels ────────────────────────────────────────── -->
  <h2 class="group-title">Skeleton Panels</h2>
  <div class="grid-wide">

    <!-- List skeleton -->
    <Card id="LOAD-7" label="Live List">
      <div class="sk-panel">
        {#if listState === 'idle'}
          <div class="sk-trigger">
            <button class="load-btn" onclick={loadList}>Load list</button>
          </div>
        {:else if listState === 'loading'}
          <div class="sk-list">
            {#each Array.from({length: 5}) as _, i}
              <div class="sk-row">
                <div class="sk-avatar shimmer"></div>
                <div class="sk-lines">
                  <div class="sk-line shimmer" style="width:{58 + (i * 7) % 30}%"></div>
                  <div class="sk-line sk-line-sm shimmer" style="width:{32 + (i * 11) % 28}%"></div>
                </div>
                <div class="sk-badge shimmer"></div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="real-list">
            {#each listItems as item, i}
              <div class="real-row" style="animation-delay:{i * 55}ms">
                <div class="real-icon">
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
                    <polyline points="14 2 14 8 20 8"/>
                  </svg>
                </div>
                <div class="real-text">
                  <span class="real-name">{item.name}</span>
                  <span class="real-meta">{item.meta}</span>
                </div>
              </div>
            {/each}
          </div>
        {/if}
        <button class="sk-reset-btn" onclick={resetList} title="Reset" aria-label="Reset">↺</button>
      </div>
    </Card>

    <!-- Thumbnail skeleton -->
    <Card id="LOAD-8" label="Live Thumbnails">
      <div class="sk-panel">
        {#if thumbState === 'idle'}
          <div class="sk-trigger">
            <button class="load-btn" onclick={loadThumbs}>Load thumbnails</button>
          </div>
        {:else if thumbState === 'loading'}
          <div class="sk-thumb-grid">
            {#each Array.from({length: 6}) as _, i}
              <div class="sk-thumb shimmer" style="animation-delay:{i * 80}ms"></div>
            {/each}
          </div>
        {:else}
          <div class="sk-thumb-grid">
            {#each thumbItems as label, i}
              <div class="real-thumb" style="animation-delay:{i * 70}ms">
                <div class="real-thumb-img">
                  <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" opacity="0.35">
                    <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                    <circle cx="8.5" cy="8.5" r="1.5"/>
                    <polyline points="21 15 16 10 5 21"/>
                  </svg>
                </div>
                <span class="real-thumb-label">{label}</span>
              </div>
            {/each}
          </div>
        {/if}
        <button class="sk-reset-btn" onclick={resetThumbs} title="Reset" aria-label="Reset">↺</button>
      </div>
    </Card>

  </div>

  <!-- ── Loading Toasts ─────────────────────────────────────────── -->
  <h2 class="group-title">Toasts</h2>
  <div class="grid">
    <Card id="LOAD-9" label="Loading → Resolved">
      <div class="toast-demo">
        <div class="toast-triggers">
          <button class="load-btn" onclick={() => triggerToast('success')}>→ Success</button>
          <button class="load-btn load-btn-danger" onclick={() => triggerToast('error')}>→ Error</button>
        </div>
        <div class="toast-stage">
          {#if toastState !== 'idle'}
            <div class="mock-toast" class:toast-loading={toastState === 'loading'} class:toast-success={toastState === 'success'} class:toast-error={toastState === 'error'}>
              {#if toastState === 'loading'}
                <div class="spin spin-sm toast-spin"></div>
                <span class="toast-msg">Exporting files…</span>
              {:else if toastState === 'success'}
                <div class="toast-icon toast-icon-success">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="20 6 9 17 4 12"/>
                  </svg>
                </div>
                <div class="toast-body">
                  <span class="toast-msg">Export complete</span>
                  <span class="toast-detail">3 files processed</span>
                </div>
              {:else}
                <div class="toast-icon toast-icon-error">
                  <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                    <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
                  </svg>
                </div>
                <div class="toast-body">
                  <span class="toast-msg">Export failed</span>
                  <span class="toast-detail">Permission denied</span>
                </div>
              {/if}
            </div>
          {:else}
            <div class="toast-placeholder">trigger above to preview</div>
          {/if}
        </div>
      </div>
    </Card>
  </div>

  <!-- ── Sheen & Outline ────────────────────────────────────────── -->
  <h2 class="group-title">Sheen &amp; Outline</h2>
  <div class="grid">

    <Card id="LOAD-10" label="Sheen Sweep — Surface">
      <div class="sheen-surface shimmer">
        <div class="sheen-line shimmer-line-wide"></div>
        <div class="sheen-line shimmer-line-mid"></div>
        <div class="sheen-gap"></div>
        <div class="sheen-line shimmer-line-wide"></div>
        <div class="sheen-line shimmer-line-short"></div>
      </div>
    </Card>

    <Card id="LOAD-11" label="Sheen — Button Loading">
      <div class="btn-sheen-wrap">
        <button
          class="sheen-btn"
          class:sheen-btn-loading={btnLoad}
          onclick={triggerBtn}
        >
          {#if btnLoad}
            <div class="spin spin-xs"></div>
            Processing…
          {:else}
            Save Changes
          {/if}
        </button>
        <span class="sheen-hint">{btnLoad ? 'loading state' : 'click to trigger'}</span>
      </div>
    </Card>

    <Card id="LOAD-12" label="Outline Shine — Spinning Border">
      <div class="outline-demo">
        <div class="outline-wrap">
          <div class="outline-inner">
            <div class="spin spin-sm"></div>
            <span class="outline-label">Indexing…</span>
          </div>
        </div>
      </div>
    </Card>

    <Card id="LOAD-13" label="Pulse Glow — Border">
      <div class="pulse-demo">
        <div class="pulse-card">
          <div class="spin spin-sm"></div>
          <span class="pulse-label">Syncing…</span>
        </div>
      </div>
    </Card>

  </div>
</div>

<style>
  .section { max-width: 1125px; }

  .group-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 32px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 16px;
  }

  .grid-wide {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }

  /* ── Bars ── */
  .bar-demo {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 4px 0;
  }

  .bar-track {
    position: relative;
    width: 100%;
    height: 4px;
    background: var(--border);
    border-radius: 2px;
    overflow: hidden;
  }

  /* Sweep: single segment glides left → right and loops */
  .bar-sweep {
    position: absolute;
    height: 100%;
    width: 35%;
    background: var(--accent);
    border-radius: 2px;
    animation: load-bar-sweep 1.8s cubic-bezier(0.4, 0, 0.2, 1) infinite;
  }

  @keyframes load-bar-sweep {
    0%   { left: -35%; }
    60%  { left: 100%; }
    100% { left: 100%; }
  }

  /* Bounce: segment alternates direction */
  .bar-bounce {
    position: absolute;
    height: 100%;
    width: 35%;
    background: var(--accent);
    border-radius: 2px;
    animation: load-bar-bounce 1.1s cubic-bezier(0.45, 0, 0.55, 1) infinite alternate;
  }

  @keyframes load-bar-bounce {
    from { left: 0; }
    to   { left: 65%; }
  }

  /* Pulse: full bar opacity cycles */
  .bar-track-pulse { overflow: visible; }
  .bar-pulse-fill {
    position: absolute;
    inset: 0;
    background: var(--accent);
    border-radius: 2px;
    animation: load-bar-pulse 1.4s ease-in-out infinite;
  }

  @keyframes load-bar-pulse {
    0%, 100% { opacity: 0.25; }
    50%       { opacity: 1;    }
  }

  /* ── Spinners ── */
  .spinner-row {
    display: flex;
    align-items: center;
    gap: 14px;
  }

  .spin {
    border-radius: 50%;
    border-style: solid;
    border-color: var(--border);
    border-top-color: var(--accent);
    animation: load-spin 0.72s linear infinite;
    flex-shrink: 0;
  }

  .spin-xs { width: 12px; height: 12px; border-width: 1.5px; }
  .spin-sm { width: 16px; height: 16px; border-width: 2px;   }
  .spin-md { width: 22px; height: 22px; border-width: 2.5px; }
  .spin-lg { width: 30px; height: 30px; border-width: 3px;   }

  @keyframes load-spin {
    to { transform: rotate(360deg); }
  }

  /* Dot pulse */
  .dots-row {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent);
    animation: load-dot-pulse 1.2s ease-in-out infinite;
  }
  .dot:nth-child(2) { animation-delay: 0.18s; }
  .dot:nth-child(3) { animation-delay: 0.36s; }

  @keyframes load-dot-pulse {
    0%, 80%, 100% { transform: scale(0.65); opacity: 0.35; }
    40%            { transform: scale(1);    opacity: 1;    }
  }

  /* Arc ring */
  .arc-wrap { display: flex; align-items: center; justify-content: center; }

  .arc-indicator {
    transform-box: fill-box;
    transform-origin: center;
    animation: load-spin 1s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  /* ── Skeleton panels ── */
  .sk-panel {
    position: relative;
    width: 100%;
    min-height: 140px;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .sk-trigger {
    display: flex;
    align-items: center;
    justify-content: center;
    flex: 1;
    padding: 24px 0;
  }

  .load-btn {
    padding: 5px 14px;
    border-radius: var(--radius-sm);
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background 80ms, border-color 80ms, color 80ms;
  }

  .load-btn:hover {
    background: var(--surface-hover);
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .load-btn-danger:hover {
    border-color: var(--color-danger);
    color: var(--color-danger);
  }

  .sk-reset-btn {
    position: absolute;
    top: 0;
    right: 0;
    width: 22px;
    height: 22px;
    padding: 0;
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    color: var(--text-muted);
    font-size: 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 80ms, background 80ms;
    opacity: 0.6;
  }

  .sk-reset-btn:hover { opacity: 1; color: var(--text-primary); background: var(--surface-hover); }

  /* Shimmer effect: translated highlight overlay */
  .shimmer {
    position: relative;
    overflow: hidden;
  }

  .shimmer::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      color-mix(in srgb, var(--text-primary) 7%, transparent) 50%,
      transparent 100%
    );
    transform: translateX(-100%);
    animation: load-shimmer 1.6s ease-in-out infinite;
  }

  @keyframes load-shimmer {
    to { transform: translateX(200%); }
  }

  /* Skeleton list */
  .sk-list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 2px 0;
  }

  .sk-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .sk-avatar {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: var(--surface-raised);
    flex-shrink: 0;
  }

  .sk-lines {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .sk-line {
    height: 9px;
    border-radius: 4px;
    background: var(--surface-raised);
  }

  .sk-line-sm { height: 7px; }

  .sk-badge {
    width: 42px;
    height: 18px;
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    flex-shrink: 0;
  }

  /* Real list content */
  .real-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .real-row {
    display: flex;
    align-items: center;
    gap: 10px;
    animation: load-row-in 180ms ease-out backwards;
  }

  @keyframes load-row-in {
    from { opacity: 0; transform: translateY(5px); }
    to   { opacity: 1; transform: translateY(0);   }
  }

  .real-icon {
    width: 28px;
    height: 28px;
    border-radius: 50%;
    background: color-mix(in srgb, var(--accent) 12%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--accent) 25%, var(--border));
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    flex-shrink: 0;
  }

  .real-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .real-name {
    font-size: 12px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .real-meta {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  /* Thumbnail skeleton */
  .sk-thumb-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }

  .sk-thumb {
    aspect-ratio: 1;
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
  }

  .real-thumb {
    aspect-ratio: 1;
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    border: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 5px;
    animation: load-row-in 200ms ease-out backwards;
    overflow: hidden;
  }

  .real-thumb-img {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
  }

  .real-thumb-label {
    font-size: 8px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    text-align: center;
    padding: 0 4px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }

  /* ── Loading toast ── */
  .toast-demo {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 100%;
  }

  .toast-triggers {
    display: flex;
    gap: 8px;
  }

  .toast-stage {
    min-height: 48px;
    display: flex;
    align-items: center;
  }

  .toast-placeholder {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  .mock-toast {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 14px;
    border-radius: var(--radius-md);
    border: 1px solid var(--border);
    background: var(--surface-raised);
    width: 100%;
    animation: load-toast-in 150ms ease-out;
  }

  @keyframes load-toast-in {
    from { opacity: 0; transform: translateY(-6px); }
    to   { opacity: 1; transform: translateY(0);    }
  }

  .toast-spin { flex-shrink: 0; }

  .toast-loading { border-color: color-mix(in srgb, var(--accent) 40%, var(--border)); }

  .toast-success {
    border-color: color-mix(in srgb, var(--color-success) 40%, var(--border));
    background: color-mix(in srgb, var(--color-success-bg) 60%, var(--surface-raised));
  }

  .toast-error {
    border-color: color-mix(in srgb, var(--color-danger) 40%, var(--border));
    background: color-mix(in srgb, var(--color-danger-bg) 60%, var(--surface-raised));
  }

  .toast-icon {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .toast-icon-success { background: var(--color-success); color: #fff; }
  .toast-icon-error   { background: var(--color-danger);  color: #fff; }

  .toast-body { display: flex; flex-direction: column; gap: 1px; }

  .toast-msg {
    font-size: 12px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .toast-detail {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  /* ── Sheen surface ── */
  .sheen-surface {
    width: 100%;
    padding: 14px;
    border-radius: var(--radius-sm);
    background: var(--surface-raised);
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .sheen-line {
    height: 9px;
    border-radius: 4px;
    background: color-mix(in srgb, var(--text-primary) 10%, var(--surface-raised));
  }
  .shimmer-line-wide  { width: 85%; }
  .shimmer-line-mid   { width: 62%; }
  .shimmer-line-short { width: 44%; }

  .sheen-gap { height: 6px; }

  /* Sheen button */
  .btn-sheen-wrap {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .sheen-btn {
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 16px;
    border-radius: var(--radius-sm);
    background: var(--accent);
    border: none;
    color: #fff;
    font-size: 13px;
    font-family: inherit;
    font-weight: 500;
    cursor: pointer;
    transition: background 80ms;
    white-space: nowrap;
  }

  .sheen-btn:hover:not(.sheen-btn-loading) { background: var(--accent-hover); }

  .sheen-btn-loading { cursor: default; opacity: 0.85; }

  .sheen-btn-loading::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(
      105deg,
      transparent 30%,
      rgba(255 255 255 / 0.4) 50%,
      transparent 70%
    );
    transform: translateX(-100%);
    animation: load-btn-sheen 1.3s ease-in-out infinite;
  }

  @keyframes load-btn-sheen {
    to { transform: translateX(200%); }
  }

  .sheen-hint {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  /* Spinning border outline */
  .outline-demo {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px 0;
  }

  .outline-wrap {
    position: relative;
    padding: 1.5px;
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .outline-wrap::before {
    content: '';
    position: absolute;
    width: 200%;
    height: 200%;
    top: -50%;
    left: -50%;
    background: conic-gradient(
      from 0deg,
      transparent 55%,
      var(--accent) 70%,
      color-mix(in srgb, var(--accent) 30%, white) 78%,
      var(--accent) 85%,
      transparent 100%
    );
    animation: load-outline-spin 1.5s linear infinite;
    transform-origin: center;
  }

  @keyframes load-outline-spin {
    to { transform: rotate(360deg); }
  }

  .outline-inner {
    position: relative;
    background: var(--surface-raised);
    border-radius: calc(var(--radius-md) - 1.5px);
    padding: 12px 20px;
    display: flex;
    align-items: center;
    gap: 10px;
    z-index: 1;
  }

  .outline-label {
    font-size: 12px;
    color: var(--text-secondary);
  }

  /* Pulse glow */
  .pulse-demo {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 8px 0;
  }

  .pulse-card {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 20px;
    border-radius: var(--radius-md);
    border: 1px solid color-mix(in srgb, var(--accent) 35%, var(--border));
    background: var(--surface-raised);
    animation: load-pulse-glow 1.6s ease-in-out infinite;
  }

  @keyframes load-pulse-glow {
    0%, 100% { box-shadow: 0 0 0 0   color-mix(in srgb, var(--accent) 0%,   transparent); }
    50%       { box-shadow: 0 0 0 5px color-mix(in srgb, var(--accent) 20%,  transparent); }
  }

  .pulse-label {
    font-size: 12px;
    color: var(--text-secondary);
  }
</style>
