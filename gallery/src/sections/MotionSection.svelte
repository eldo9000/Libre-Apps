<script>
  import BezierEditor from '@libre/ui/src/components/BezierEditor.svelte';

  const motionTokens = [
    { name: '--ease-linear', label: 'Linear'      },
    { name: '--ease-hard',   label: 'Hard Ease'   },
    { name: '--ease-out',    label: 'Gentle Ease' },
  ];

  const durationTokens = [
    { name: '--duration-instant', label: 'Instant', ms: 80  },
    { name: '--duration-fast',    label: 'Fast',    ms: 120 },
    { name: '--duration-base',    label: 'Base',    ms: 200 },
    { name: '--duration-slow',    label: 'Slow',    ms: 400 },
  ];

  const MOTION_STORAGE_KEY = 'libre-motion-tokens';

  const BEZIER_DEFAULTS = {
    '--ease-linear': [1/3,  1/3,  2/3,  2/3 ],
    '--ease-hard':   [0.9,  0,    0.1,  1   ],
    '--ease-out':    [0.25, 0.46, 0.45, 0.94],
  };
  const DURATION_DEFAULTS = {
    '--ease-linear': 600,
    '--ease-hard':   600,
    '--ease-out':    600,
  };

  function loadMotionStorage() {
    try {
      const raw = localStorage.getItem(MOTION_STORAGE_KEY);
      if (!raw) return null;
      return JSON.parse(raw);
    } catch { return null; }
  }

  function saveMotionStorage() {
    localStorage.setItem(MOTION_STORAGE_KEY, JSON.stringify({
      beziers:   Object.fromEntries(Object.entries(savedBeziers).map(([k, v]) => [k, [...v]])),
      durations: { ...durations },
    }));
  }

  const _stored = loadMotionStorage();

  let durations = $state(_stored?.durations ?? { ...DURATION_DEFAULTS });

  let beziers = $state(
    _stored?.beziers
      ? Object.fromEntries(Object.entries(_stored.beziers).map(([k, v]) => [k, [...v]]))
      : Object.fromEntries(Object.entries(BEZIER_DEFAULTS).map(([k, v]) => [k, [...v]]))
  );

  let savedBeziers = $state(
    _stored?.beziers
      ? Object.fromEntries(Object.entries(_stored.beziers).map(([k, v]) => [k, [...v]]))
      : Object.fromEntries(Object.entries(BEZIER_DEFAULTS).map(([k, v]) => [k, [...v]]))
  );

  // Window minimize/restore demo — each variant is fully independent
  let variants = $state(
    _stored?.variants?.length
      ? _stored.variants.map(v => ({ ...v, state: 'restored' }))
      : [
          { label: 'Sharp', dur: 200,  state: 'restored' },
          { label: 'Slow',  dur: 2000, state: 'restored' },
        ]
  );

  $effect(() => {
    try {
      const cur = loadMotionStorage() ?? {};
      localStorage.setItem(MOTION_STORAGE_KEY, JSON.stringify({
        ...cur,
        variants: variants.map(v => ({ label: v.label, dur: v.dur })),
      }));
    } catch {}
  });

  // Easing curve direction — applied to BOTH minimize and restore identically.
  // 'forward'  — cubic-bezier(0.38, 0, 0.13, 1) — sharp departure, gentle arrival
  // 'reversed' — cubic-bezier(0.87, 0, 0.62, 1) — gentle departure, sharp arrival
  let curveDir = $state('forward');
  const CURVES = {
    forward:  'cubic-bezier(0.38, 0.00, 0.13, 1.00)',
    reversed: 'cubic-bezier(0.87, 0.00, 0.62, 1.00)',
  };

  $effect(() => {
    const timers = variants.map((v) => setInterval(() => {
      if (v.state === 'restored')  v.state = 'minimizing';
      else if (v.state === 'minimized') v.state = 'restoring';
    }, v.dur + 1000));
    return () => timers.forEach(clearInterval);
  });

  // Animation playground state
  let playing = $state(false);
  let activeEase = $state('--ease-out');
  let animKey = $state(0);

  function play(tokenName) {
    activeEase = tokenName;
    playing = false;
    // force reflow so CSS animation restarts
    animKey += 1;
    requestAnimationFrame(() => { playing = true; });
  }
</script>

<div class="section">

  <!-- ── Principles ─────────────────────────────────────────────── -->
  <h2 class="group-title">Principles</h2>
  <div class="principles-grid">
    <div class="principle-card">
      <div class="principle-icon">⚡</div>
      <div class="principle-label">Functional</div>
      <div class="principle-desc">Motion communicates state changes, not decoration. Every transition must justify its cost.</div>
    </div>
    <div class="principle-card">
      <div class="principle-icon">⏱</div>
      <div class="principle-label">Fast</div>
      <div class="principle-desc">UI transitions stay under 200 ms. Longer durations are reserved for content loads and intentional emphasis.</div>
    </div>
    <div class="principle-card">
      <div class="principle-icon">↗</div>
      <div class="principle-label">Directional</div>
      <div class="principle-desc">Elements enter and exit with spatial purpose — they come from somewhere and go somewhere.</div>
    </div>
  </div>

  <!-- ── Durations ───────────────────────────────────────────────── -->
  <h2 class="group-title">Durations</h2>
  <div class="duration-row">
    {#each durationTokens as t}
      <div class="duration-item">
        <div class="duration-bar-wrap">
          <div class="duration-bar" style="width: {Math.round(t.ms / 4)}px;"></div>
        </div>
        <code class="dur-token">{t.name}</code>
        <span class="dur-ms">{t.ms}ms</span>
        <span class="dur-label">{t.label}</span>
      </div>
    {/each}
  </div>

  <!-- ── Easing Curves ───────────────────────────────────────────── -->
  <h2 class="group-title">Easing Curves</h2>
  <div class="motion-row">
    {#each motionTokens as t}
      <BezierEditor
        name={t.name}
        bind:value={beziers[t.name]}
        bind:savedValue={savedBeziers[t.name]}
        bind:duration={durations[t.name]}
        onSave={saveMotionStorage}
      />
    {/each}
  </div>

  <!-- ── Playground ─────────────────────────────────────────────── -->
  <h2 class="group-title">Playground</h2>
  <div class="playground">
    <div class="playground-controls">
      {#each motionTokens as t}
        {@const [x1, y1, x2, y2] = beziers[t.name]}
        <button
          class="ease-btn"
          class:active={activeEase === t.name}
          onclick={() => play(t.name)}
        >
          <svg class="ease-thumb" viewBox="0 0 40 40" width="40" height="40">
            <path
              d="M 4 36 C {4 + x1*32} {36 - y1*32} {4 + x2*32} {36 - y2*32} 36 4"
              fill="none" stroke="currentColor" stroke-width="1.5"
            />
          </svg>
          <span>{t.label}</span>
        </button>
      {/each}
    </div>
    <div class="playground-track">
      {#key animKey}
        {#if playing}
          {@const [x1, y1, x2, y2] = beziers[activeEase]}
          {@const dur = durations[activeEase]}
          <div
            class="playground-dot"
            style="
              animation: slide-dot {dur}ms cubic-bezier({x1},{y1},{x2},{y2}) forwards;
            "
          ></div>
        {:else}
          <div class="playground-dot"></div>
        {/if}
      {/key}
    </div>
    <button class="play-btn" onclick={() => play(activeEase)}>Play</button>
  </div>

  <!-- ── Window — Minimize / Restore ──────────────────────────── -->
  <h2 class="group-title">Window — Minimize / Restore</h2>

  <div class="min-toolbar">
    <span class="min-toolbar-label">Easing</span>
    <div class="min-mode-btns" role="group">
      <button
        class="min-mode-btn"
        class:active={curveDir === 'forward'}
        onclick={() => curveDir = 'forward'}
      >Sharp ⇒ Gentle</button>
      <button
        class="min-mode-btn"
        class:active={curveDir === 'reversed'}
        onclick={() => curveDir = 'reversed'}
      >Gentle ⇒ Sharp</button>
    </div>
  </div>

  <div class="min-row">
    {#each variants as v}
      <div class="min-stage">
        <label class="min-dur-label">
          <input type="number" min="50" max="5000" step="50" bind:value={v.dur} class="min-dur-input" />
          <span>ms</span>
        </label>
        <div
          class="min-window"
          class:is-minimized={v.state === 'minimized'}
          class:do-minimize={v.state === 'minimizing'}
          class:do-restore={v.state === 'restoring'}
          style="--win-dur: {v.dur}ms; --win-ease: {CURVES[curveDir]};"
          onanimationend={() => {
            if (v.state === 'minimizing') v.state = 'minimized';
            else if (v.state === 'restoring') v.state = 'restored';
          }}
        >
          <div class="min-tb">
            <div class="min-dot min-close"></div>
            <div class="min-dot min-yellow"></div>
            <div class="min-dot min-green"></div>
            <span class="min-name">Document</span>
          </div>
          <div class="min-body">
            <div class="min-line" style="width:80%"></div>
            <div class="min-line" style="width:55%"></div>
            <div class="min-line" style="width:70%"></div>
            <div class="min-line" style="width:40%"></div>
          </div>
        </div>
        <div class="min-taskbar">
          <div class="min-task-icon" class:min-task-active={v.state === 'minimized' || v.state === 'minimizing'}>
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect x="2" y="3" width="20" height="14" rx="2"/>
              <line x1="8" y1="21" x2="16" y2="21"/>
              <line x1="12" y1="17" x2="12" y2="21"/>
            </svg>
          </div>
        </div>
        <span class="min-label">{v.label}</span>
      </div>
    {/each}
  </div>

</div>

<style>
  .section { max-width: 1125px; }

  .group-title {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 36px 0 14px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  /* ── Principles ── */
  .principles-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  .principle-card {
    padding: 16px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .principle-icon {
    font-size: 18px;
    line-height: 1;
  }

  .principle-label {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .principle-desc {
    font-size: 11px;
    line-height: 1.55;
    color: var(--text-secondary);
  }

  /* ── Durations ── */
  .duration-row {
    display: flex;
    gap: 32px;
    align-items: flex-end;
    flex-wrap: wrap;
  }

  .duration-item {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 5px;
  }

  .duration-bar-wrap {
    height: 6px;
    display: flex;
    align-items: center;
  }

  .duration-bar {
    height: 6px;
    background: var(--accent);
    border-radius: 3px;
    opacity: 0.7;
    min-width: 4px;
  }

  .dur-token { font-size: 9px; font-family: 'Geist Mono', monospace; color: var(--text-muted); }
  .dur-ms { font-size: 13px; font-family: 'Geist Mono', monospace; color: var(--text-primary); font-weight: 600; }
  .dur-label { font-size: 10px; color: var(--text-secondary); }

  /* ── Easing curves ── */
  .motion-row {
    display: flex;
    gap: 32px;
    align-items: flex-start;
    width: max-content;
    max-width: none;
    flex-wrap: nowrap;
  }

  /* ── Playground ── */
  .playground {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 24px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
  }

  .playground-controls {
    display: flex;
    gap: 12px;
  }

  .ease-btn {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px 6px 8px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    font-size: 11px;
    color: var(--text-secondary);
    cursor: pointer;
    transition: border-color 120ms, color 120ms;
    font-family: inherit;
  }

  .ease-btn:hover {
    border-color: var(--accent);
    color: var(--text-primary);
  }

  .ease-btn.active {
    border-color: var(--accent);
    color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }

  .ease-thumb {
    color: currentColor;
    opacity: 0.7;
    flex-shrink: 0;
  }

  .playground-track {
    height: 20px;
    position: relative;
    border-radius: 10px;
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    overflow: visible;
  }

  .playground-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: var(--accent);
    position: absolute;
    top: 0;
    left: 0;
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent) 25%, transparent);
  }

  @keyframes slide-dot {
    from { left: 0; }
    to   { left: calc(100% - 20px); }
  }

  .play-btn {
    align-self: flex-start;
    padding: 5px 16px;
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: var(--radius-sm);
    font-size: 12px;
    font-weight: 600;
    font-family: inherit;
    cursor: pointer;
    transition: background 120ms;
  }

  .play-btn:hover { background: var(--accent-hover); }

  /* ── Window Minimize / Restore ── */
  .min-toolbar {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 12px;
  }

  .min-toolbar-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .min-mode-btns {
    display: inline-flex;
    background: rgb(0 0 0 / 18%);
    border-radius: 5px;
    padding: 2px;
    gap: 2px;
  }

  .min-mode-btn {
    padding: 3px 12px;
    border: none;
    border-radius: 3px;
    background: transparent;
    font-family: inherit;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 100ms, color 100ms;
  }

  .min-mode-btn:hover { color: var(--text-secondary); }

  .min-mode-btn.active {
    background: color-mix(in srgb, white 18%, var(--surface-raised));
    box-shadow: 0 1px 3px rgb(0 0 0 / 30%);
    color: var(--text-primary);
  }

  .min-dur-label {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 10px;
    color: var(--text-muted);
    align-self: flex-end;
  }

  .min-dur-input {
    width: 64px;
    padding: 3px 7px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: var(--radius-sm);
    color: var(--text-primary);
    font-family: 'Geist Mono', monospace;
    font-size: 12px;
  }

  .min-dur-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .min-row {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 16px;
  }

  .min-stage {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 24px 16px 16px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
  }

  .min-window {
    width: 100%;
    border-radius: 8px;
    border: 1px solid var(--border);
    overflow: hidden;
    background: var(--surface);
    box-shadow: 0 4px 16px rgb(0 0 0 / 0.12), 0 1px 3px rgb(0 0 0 / 0.08);
    /* Anchor scale-down to the taskbar icon position:
       19px right of window left (5px taskbar padding + 14px half-icon),
       30px below window bottom (12px stage gap + 18px to icon center). */
    transform-origin: 19px calc(100% + 30px);
    transform: scale(1);
    opacity: 1;
    will-change: transform, opacity;
  }

  .min-window.is-minimized {
    transform: scale(0.05);
    opacity: 0;
    pointer-events: none;
  }

  .min-window.do-minimize {
    animation: win-minimize var(--win-dur, 360ms) var(--win-ease, cubic-bezier(0.38, 0.00, 0.13, 1.00)) both;
  }
  .min-window.do-restore {
    animation: win-restore var(--win-dur, 360ms) var(--win-ease, cubic-bezier(0.38, 0.00, 0.13, 1.00)) both;
  }

  @keyframes win-minimize {
    0%   { transform: scale(1);    opacity: 1; }
    100% { transform: scale(0.05); opacity: 0; }
  }

  @keyframes win-restore {
    0%   { transform: scale(0.05); opacity: 0; }
    100% { transform: scale(1);    opacity: 1; }
  }

  .min-tb {
    height: 30px;
    background: var(--titlebar-bg, var(--surface-panel));
    border-bottom: 1px solid var(--border);
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 0 10px;
  }

  .min-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .min-close  { background: #ff5f57; }
  .min-yellow { background: #ffbd2e; }
  .min-green  { background: #28c840; }

  .min-name {
    margin-left: 4px;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-secondary);
  }

  .min-body {
    padding: 14px 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .min-line {
    height: 8px;
    border-radius: 4px;
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
  }

  .min-label {
    font-size: 10px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-muted);
    margin-top: auto;
  }

  .min-taskbar {
    width: 100%;
    height: 36px;
    background: var(--titlebar-bg, var(--surface-panel));
    border: 1px solid var(--border);
    border-radius: 7px;
    display: flex;
    align-items: center;
    padding: 0 5px;
  }

  .min-task-icon {
    width: 28px;
    height: 28px;
    border-radius: 5px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    opacity: 0.4;
    transition: opacity 200ms ease, background 150ms ease, color 150ms ease;
  }

  .min-task-active {
    opacity: 1;
    background: color-mix(in srgb, var(--accent) 12%, transparent);
    color: var(--accent);
  }
</style>
