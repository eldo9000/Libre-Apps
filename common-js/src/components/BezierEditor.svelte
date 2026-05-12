<script>
  /**
   * BezierEditor — interactive cubic-bezier curve editor with animated playback preview.
   *
   * @typedef {{
   *   name?: string,
   *   value?: number[],
   *   savedValue?: number[],
   *   duration?: number,
   *   onSave?: () => void,
   * }} Props
   * @type {Props}
   */
  let {
    name       = '',
    value      = $bindable([0.25, 0.46, 0.45, 0.94]),
    savedValue = $bindable([0.25, 0.46, 0.45, 0.94]),
    duration   = $bindable(600),
    onSave,
  } = $props();

  const SVG_SIZE        = 120;
  const LINEAR_EDITABLE = [1/3, 1/3, 2/3, 2/3];

  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const fmt   = (n) => (Math.round(n * 100) / 100).toFixed(2);

  function bezierPath([x1, y1, x2, y2], size = SVG_SIZE) {
    return `M 0,${size} C ${x1*size},${size - y1*size} ${x2*size},${size - y2*size} ${size},0`;
  }
  function bezierCss([x1, y1, x2, y2]) {
    return `cubic-bezier(${x1}, ${y1}, ${x2}, ${y2})`;
  }
  function bezierCssRounded([x1, y1, x2, y2]) {
    return `cubic-bezier(${fmt(x1)}, ${fmt(y1)}, ${fmt(x2)}, ${fmt(y2)})`;
  }
  function bezierCssReflected([x1, y1, x2, y2]) {
    return `cubic-bezier(${fmt(1 - x2)}, ${fmt(1 - y2)}, ${fmt(1 - x1)}, ${fmt(1 - y1)})`;
  }
  function bezierHandles([x1, y1, x2, y2], size = SVG_SIZE) {
    return { p1: { x: x1*size, y: size - y1*size }, p2: { x: x2*size, y: size - y2*size } };
  }

  const isDirty = $derived(
    value[0] !== savedValue[0] || value[1] !== savedValue[1] ||
    value[2] !== savedValue[2] || value[3] !== savedValue[3]
  );
  const h = $derived(bezierHandles(value));

  function reset()  { value = [...LINEAR_EDITABLE]; }
  function cancel() { if (isDirty) value = [...savedValue]; }
  function save() {
    if (!isDirty) return;
    savedValue = [...value];
    if (name) document.documentElement.style.setProperty(name, bezierCss(value));
    onSave?.();
  }

  // ── Cubic bezier easing solver ──────────────────────────────────────────────
  function cubicBezierEasing([p1x, p1y, p2x, p2y]) {
    const cx = 3 * p1x, bx = 3 * (p2x - p1x) - cx, ax = 1 - cx - bx;
    const cy = 3 * p1y, by = 3 * (p2y - p1y) - cy, ay = 1 - cy - by;
    const sampleX = (t) => ((ax * t + bx) * t + cx) * t;
    const sampleY = (t) => ((ay * t + by) * t + cy) * t;
    const slopeX  = (t) => (3 * ax * t + 2 * bx) * t + cx;
    function solveT(x) {
      let t = x;
      for (let i = 0; i < 8; i++) {
        const xt = sampleX(t) - x;
        if (Math.abs(xt) < 1e-6) return t;
        const d = slopeX(t); if (Math.abs(d) < 1e-6) break;
        t -= xt / d;
      }
      let lo = 0, hi = 1; t = x;
      for (let i = 0; i < 32; i++) {
        const xt = sampleX(t);
        if (Math.abs(xt - x) < 1e-6) return t;
        if (x > xt) lo = t; else hi = t;
        t = (lo + hi) / 2;
      }
      return t;
    }
    return (x) => sampleY(solveT(Math.max(0, Math.min(1, x))));
  }

  let pillDir     = $state(0);
  let pingPong    = $state(false);
  let resetting   = $state(false);
  let tracePos    = $state(null);
  let drag        = $state(null);
  let svgRef      = $state(null);
  let traceAnimId = 0;

  function startTrace(reverse = false) {
    if (traceAnimId) cancelAnimationFrame(traceAnimId);
    const start = performance.now();
    function step(now) {
      const tNorm = Math.min(1, (now - start) / duration);
      const x = reverse ? (1 - tNorm) : tNorm;
      const y = cubicBezierEasing(value)(x);
      tracePos = { x: x * 120, y: (1 - y) * 120 };
      if (tNorm < 1) {
        traceAnimId = requestAnimationFrame(step);
      } else {
        traceAnimId = 0;
        tracePos = null;
      }
    }
    traceAnimId = requestAnimationFrame(step);
  }

  function triggerAnimation() {
    if (pingPong) {
      pillDir = pillDir ? 0 : 1;
      startTrace(pillDir === 0);
      return;
    }
    if (pillDir) {
      resetting = true;
      pillDir   = 0;
      requestAnimationFrame(() => requestAnimationFrame(() => {
        resetting = false;
        pillDir   = 1;
        startTrace(false);
      }));
    } else {
      pillDir = 1;
      startTrace(false);
    }
  }

  function onHandleDown(e, idx) {
    e.stopPropagation();
    e.preventDefault();
    e.currentTarget.setPointerCapture(e.pointerId);
    const r = svgRef.getBoundingClientRect();
    const cursorX = (e.clientX - r.left) / r.width;
    const cursorY = 1 - (e.clientY - r.top) / r.height;
    drag = { idx, pointerId: e.pointerId, offX: cursorX - value[idx], offY: cursorY - value[idx + 1] };
  }

  function onHandleMove(e) {
    if (!drag || !svgRef) return;
    const r = svgRef.getBoundingClientRect();
    const x = clamp((e.clientX - r.left) / r.width - drag.offX, 0, 1);
    const y = clamp(1 - (e.clientY - r.top) / r.height - drag.offY, 0, 1);
    const b = [...value];
    b[drag.idx]     = x;
    b[drag.idx + 1] = y;
    value = b;
  }

  function onHandleUp(e) {
    if (!drag) return;
    if (e.currentTarget.hasPointerCapture(drag.pointerId)) e.currentTarget.releasePointerCapture(drag.pointerId);
    drag = null;
  }
</script>

<div class="be-root">
  <svg
    class="be-svg"
    class:be-svg-dragging={drag !== null}
    viewBox="0 0 120 120"
    xmlns="http://www.w3.org/2000/svg"
    bind:this={svgRef}
    style="touch-action: none;"
  >
    {#each [30, 60, 90] as g}
      <line x1={g} y1="0" x2={g} y2="120" stroke="var(--border-subtle)" stroke-width="0.5"/>
      <line x1="0" y1={g} x2="120" y2={g} stroke="var(--border-subtle)" stroke-width="0.5"/>
    {/each}
    <rect x="0" y="0" width="120" height="120" fill="none" stroke="var(--border)" stroke-width="1"/>
    <line x1="0" y1="120" x2="120" y2="0" stroke="var(--text-muted)" stroke-width="0.8" stroke-dasharray="3 3" opacity="0.4"/>
    <path d={bezierPath(value)} fill="none"
      stroke={isDirty ? 'var(--accent)' : '#fff'}
      stroke-width={isDirty ? 1.5 : 0.5}
    />
    <line x1="0"   y1="120" x2={h.p1.x} y2={h.p1.y} stroke="var(--text-muted)" stroke-width="1" opacity="0.55"/>
    <line x1="120" y1="0"   x2={h.p2.x} y2={h.p2.y} stroke="var(--text-muted)" stroke-width="1" opacity="0.55"/>
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <circle
      cx={h.p1.x} cy={h.p1.y}
      r={isDirty ? 3 : 2}
      fill={isDirty ? '#fff' : 'var(--accent)'}
      stroke="var(--accent)" stroke-width="1.5"
      class="be-handle"
      class:be-handle-dragging={drag?.idx === 0}
      onpointerdown={(e) => onHandleDown(e, 0)}
      onpointermove={onHandleMove}
      onpointerup={onHandleUp}
      onpointercancel={onHandleUp}
    />
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <circle
      cx={h.p2.x} cy={h.p2.y}
      r={isDirty ? 3 : 2}
      fill={isDirty ? '#fff' : 'var(--accent)'}
      stroke="var(--accent)" stroke-width="1.5"
      class="be-handle"
      class:be-handle-dragging={drag?.idx === 2}
      onpointerdown={(e) => onHandleDown(e, 2)}
      onpointermove={onHandleMove}
      onpointerup={onHandleUp}
      onpointercancel={onHandleUp}
    />
    <circle cx="0"   cy="120" r="2.5" fill="var(--text-muted)"/>
    <circle cx="120" cy="0"   r="2.5" fill="var(--text-muted)"/>
    {#if tracePos}
      <circle
        cx={tracePos.x} cy={tracePos.y} r="3"
        fill="#ff8a3d" stroke="var(--surface)" stroke-width="1"
        pointer-events="none"
      />
    {/if}
  </svg>

  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div class="be-track" onclick={triggerAnimation}>
    <div
      class="be-pill"
      class:be-pill-run={pillDir}
      style="transition-duration: {resetting ? '0ms' : duration + 'ms'}; transition-timing-function: {pillDir ? bezierCss(value) : bezierCssReflected(value)};"
    ></div>
  </div>

  <div class="be-actions">
    <div class="be-action-group">
      <button class="be-btn" onclick={reset} aria-label="Reset to linear" title="Reset to linear">
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="3 12 3 6 9 6"/>
          <path d="M3 12a9 9 0 1 0 3-6.7L3 6"/>
        </svg>
      </button>
      <button
        class="be-btn"
        class:be-btn-active={pingPong}
        onclick={() => (pingPong = !pingPong)}
        aria-label="Toggle ping-pong"
        title={pingPong ? 'Ping-pong: on' : 'Ping-pong: off'}
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="17 1 21 5 17 9"/>
          <path d="M3 11V9a4 4 0 0 1 4-4h14"/>
          <polyline points="7 23 3 19 7 15"/>
          <path d="M21 13v2a4 4 0 0 1-4 4H3"/>
        </svg>
      </button>
    </div>

    <div class="be-action-group">
      <button
        class="be-btn be-btn-save"
        disabled={!isDirty}
        onclick={save}
        aria-label="Save changes"
        title={isDirty ? 'Save changes' : 'No changes'}
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"/>
        </svg>
      </button>
      <button
        class="be-btn be-btn-cancel"
        disabled={!isDirty}
        onclick={cancel}
        aria-label="Cancel changes"
        title={isDirty ? 'Cancel changes' : 'No changes'}
      >
        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">
          <line x1="18" y1="6" x2="6" y2="18"/>
          <line x1="6"  y1="6" x2="18" y2="18"/>
        </svg>
      </button>
    </div>

    <div class="be-action-group">
      <div class="be-dur-wrap">
        <input
          type="number"
          class="be-dur-input"
          min="0.1"
          max="10"
          step="0.1"
          value={duration / 1000}
          oninput={(e) => {
            const v = parseFloat(e.currentTarget.value);
            if (!isNaN(v) && v >= 0.1) {
              duration = Math.round(v * 1000);
              onSave?.();
            }
          }}
        />
        <span class="be-dur-unit">s</span>
      </div>
    </div>
  </div>

  <div class="be-meta">
    {#if name}<code class="be-token">{name}</code>{/if}
    <span class="be-value">{bezierCssRounded(value)}</span>
  </div>
</div>

<style>
  .be-root {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 320px;
    flex-shrink: 0;
  }

  .be-svg {
    width: 240px;
    height: 240px;
    background: var(--surface);
    border-radius: 8px;
    display: block;
  }
  .be-svg-dragging { user-select: none; }

  .be-handle {
    cursor: grab;
    transition: fill 80ms ease-out, stroke-width 80ms ease-out;
  }

  .be-handle:hover {
    fill: color-mix(in srgb, var(--accent) 25%, white);
  }

  .be-handle-dragging {
    cursor: grabbing;
    fill: var(--accent) !important;
    stroke-width: 2.5 !important;
  }

  .be-track {
    width: 240px;
    height: 18px;
    background: var(--surface-raised);
    border-radius: 9px;
    border: 1px solid var(--border);
    position: relative;
    cursor: pointer;
    flex-shrink: 0;
    overflow: hidden;
    transition: background 100ms, border-color 100ms;
  }

  .be-track:hover {
    background: color-mix(in srgb, var(--accent) 10%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
  }

  .be-pill {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 12px;
    height: 12px;
    border-radius: 6px;
    background: var(--accent);
    transition-property: left;
  }
  .be-pill-run { left: calc(240px - 15px); }

  .be-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 240px;
  }

  .be-action-group {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .be-btn {
    width: 26px;
    height: 22px;
    padding: 0;
    border-radius: 4px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 80ms, color 80ms, border-color 80ms;
  }

  .be-btn:hover {
    background: var(--surface-hover);
    color: var(--text-primary);
  }
  .be-btn:active { background: var(--surface-active); }

  .be-btn-active {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--accent) 50%, var(--border));
    color: var(--accent);
  }

  .be-btn-active:hover {
    background: color-mix(in srgb, var(--accent) 28%, var(--surface-raised));
    color: var(--accent);
  }

  .be-btn:disabled {
    opacity: 0.35;
    cursor: default;
    pointer-events: none;
  }

  .be-btn-save:not(:disabled) {
    background: color-mix(in srgb, var(--color-success) 18%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--color-success) 50%, var(--border));
    color: var(--color-success);
  }

  .be-btn-save:not(:disabled):hover {
    background: color-mix(in srgb, var(--color-success) 30%, var(--surface-raised));
  }

  .be-btn-cancel:not(:disabled) {
    background: color-mix(in srgb, var(--color-danger) 16%, var(--surface-raised));
    border-color: color-mix(in srgb, var(--color-danger) 50%, var(--border));
    color: var(--color-danger);
  }

  .be-btn-cancel:not(:disabled):hover {
    background: color-mix(in srgb, var(--color-danger) 28%, var(--surface-raised));
  }

  .be-dur-wrap {
    display: flex;
    align-items: center;
    gap: 3px;
  }

  .be-dur-input {
    width: 42px;
    height: 22px;
    padding: 0 5px;
    border-radius: 4px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    color: var(--text-primary);
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    text-align: right;
    appearance: textfield;
  }

  .be-dur-input::-webkit-inner-spin-button,
  .be-dur-input::-webkit-outer-spin-button {
    appearance: none;
  }

  .be-dur-input:focus {
    outline: none;
    border-color: var(--accent);
  }

  .be-dur-unit {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  .be-meta {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .be-token {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-secondary);
  }

  .be-value {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    word-break: break-all;
  }
</style>
