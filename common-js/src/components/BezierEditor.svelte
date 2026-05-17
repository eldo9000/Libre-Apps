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
  const ZOOM_INSET      = 30;   // when zoomed out, 0→1 maps to [ZOOM_INSET, SVG_SIZE - ZOOM_INSET]
  const ZOOM_INNER      = SVG_SIZE - 2 * ZOOM_INSET; // 60 — width of the inner 0→1 box
  const LINEAR_EDITABLE = [1/3, 1/3, 2/3, 2/3];

  const clamp = (v, lo, hi) => Math.max(lo, Math.min(hi, v));
  const fmt   = (n) => (Math.round(n * 100) / 100).toFixed(2);

  let zoomedOut = $state(false);

  /** Map a bezier (x, y) — where (0,0) is bottom-left and (1,1) is top-right — to SVG coords. */
  function toSvg(bx, by) {
    if (zoomedOut) {
      return { x: ZOOM_INSET + bx * ZOOM_INNER, y: (SVG_SIZE - ZOOM_INSET) - by * ZOOM_INNER };
    }
    return { x: bx * SVG_SIZE, y: (1 - by) * SVG_SIZE };
  }

  /** Convert pointer client coords (inside the SVG element) to bezier-space (x, y). */
  function fromPointer(clientX, clientY, rect) {
    const ex = (clientX - rect.left) / rect.width;
    const ey = 1 - (clientY - rect.top) / rect.height;
    if (zoomedOut) {
      // SVG spans [-0.5, 1.5] in bezier-space when zoomed out
      return { x: ex * 2 - 0.5, y: ey * 2 - 0.5 };
    }
    return { x: ex, y: ey };
  }

  /** Clamp a handle (x, y) to the editable range. X always [0,1] for CSS bezier validity. */
  function clampHandle(x, y) {
    const cx = clamp(x, 0, 1);
    const cy = zoomedOut ? clamp(y, -0.5, 1.5) : clamp(y, 0, 1);
    return [cx, cy];
  }

  function toggleZoom() {
    zoomedOut = !zoomedOut;
    if (!zoomedOut) {
      // Pull handle Y values back into [0, 1] when leaving zoomed mode.
      value = [value[0], clamp(value[1], 0, 1), value[2], clamp(value[3], 0, 1)];
    }
  }

  function bezierPath([x1, y1, x2, y2]) {
    const a  = toSvg(0, 0);
    const b  = toSvg(1, 1);
    const c1 = toSvg(x1, y1);
    const c2 = toSvg(x2, y2);
    return `M ${a.x},${a.y} C ${c1.x},${c1.y} ${c2.x},${c2.y} ${b.x},${b.y}`;
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
  function bezierHandles([x1, y1, x2, y2]) {
    return { p1: toSvg(x1, y1), p2: toSvg(x2, y2) };
  }

  const isDirty = $derived(
    value[0] !== savedValue[0] || value[1] !== savedValue[1] ||
    value[2] !== savedValue[2] || value[3] !== savedValue[3]
  );
  const h          = $derived(bezierHandles(value));
  const anchor0    = $derived(toSvg(0, 0));
  const anchor1    = $derived(toSvg(1, 1));
  const innerBoxX  = $derived(zoomedOut ? ZOOM_INSET : 0);
  const innerBoxY  = $derived(zoomedOut ? ZOOM_INSET : 0);
  const innerBoxSz = $derived(zoomedOut ? ZOOM_INNER : SVG_SIZE);

  function reset()  { value = [...LINEAR_EDITABLE]; }
  function cancel() { if (isDirty) value = [...savedValue]; }
  function save() {
    if (!isDirty) return;
    savedValue = [...value];
    if (name) document.documentElement.style.setProperty(name, bezierCss(value));
    if (selectedPresetId !== null) {
      const p = presets.find(x => x.id === selectedPresetId);
      if (p) { p.value = [...value]; savePresetsStorage(); }
    }
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
      const pt = toSvg(x, y);
      tracePos = { x: pt.x, y: pt.y };
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
    const p = fromPointer(e.clientX, e.clientY, r);
    drag = { idx, pointerId: e.pointerId, offX: p.x - value[idx], offY: p.y - value[idx + 1] };
  }

  function onHandleMove(e) {
    if (!drag || !svgRef) return;
    const r = svgRef.getBoundingClientRect();
    const p = fromPointer(e.clientX, e.clientY, r);
    const [cx, cy] = clampHandle(p.x - drag.offX, p.y - drag.offY);
    const b = [...value];
    b[drag.idx]     = cx;
    b[drag.idx + 1] = cy;
    value = b;
  }

  function onHandleUp(e) {
    if (!drag) return;
    if (e.currentTarget.hasPointerCapture(drag.pointerId)) e.currentTarget.releasePointerCapture(drag.pointerId);
    drag = null;
  }

  // ── Presets ──────────────────────────────────────────────────────────────
  const PRESET_STORAGE_KEY = 'libre-bezier-presets';

  const DEFAULT_PRESETS = [
    { id: 1, name: 'Linear',       value: [0.33, 0.33, 0.67, 0.67], defaultValue: [0.33, 0.33, 0.67, 0.67] },
    { id: 2, name: 'Ease',         value: [0.25, 0.10, 0.25, 1.00], defaultValue: [0.25, 0.10, 0.25, 1.00] },
    { id: 3, name: 'Ease In',      value: [0.42, 0.00, 1.00, 1.00], defaultValue: [0.42, 0.00, 1.00, 1.00] },
    { id: 4, name: 'Ease Out',     value: [0.00, 0.00, 0.58, 1.00], defaultValue: [0.00, 0.00, 0.58, 1.00] },
    { id: 5, name: 'Ease In Out',  value: [0.42, 0.00, 0.58, 1.00], defaultValue: [0.42, 0.00, 0.58, 1.00] },
    { id: 6, name: 'Sharp Out',    value: [0.00, 0.00, 0.13, 1.00], defaultValue: [0.00, 0.00, 0.13, 1.00] },
    { id: 7, name: 'Spring',       value: [0.34, 1.56, 0.64, 1.00], defaultValue: [0.34, 1.56, 0.64, 1.00] },
    { id: 8, name: 'Anticipate',   value: [0.36, 0.00, 0.66,-0.56], defaultValue: [0.36, 0.00, 0.66,-0.56] },
  ];

  function loadPresets() {
    try {
      const raw = localStorage.getItem(PRESET_STORAGE_KEY);
      if (!raw) return DEFAULT_PRESETS.map(p => ({ ...p, value: [...p.value], defaultValue: [...p.defaultValue] }));
      const parsed = JSON.parse(raw);
      return DEFAULT_PRESETS.map(def => {
        const stored = parsed.find(p => p.id === def.id);
        return stored
          ? { ...def, name: stored.name, value: [...stored.value] }
          : { ...def, value: [...def.value], defaultValue: [...def.defaultValue] };
      });
    } catch {
      return DEFAULT_PRESETS.map(p => ({ ...p, value: [...p.value], defaultValue: [...p.defaultValue] }));
    }
  }

  function savePresetsStorage() {
    try {
      localStorage.setItem(PRESET_STORAGE_KEY, JSON.stringify(
        presets.map(p => ({ id: p.id, name: p.name, value: [...p.value] }))
      ));
    } catch {}
  }

  let presets        = $state(loadPresets());
  let selectedPresetId = $state(null);
  let editingPresetId  = $state(null);

  function selectPreset(p) {
    selectedPresetId = p.id;
    value      = [...p.value];
    savedValue = [...p.value];
  }

  function renamePreset(id, newName) {
    const p = presets.find(x => x.id === id);
    if (p && newName.trim()) { p.name = newName.trim(); savePresetsStorage(); }
  }

  function resetPreset(p) {
    p.value = [...p.defaultValue];
    savePresetsStorage();
    if (selectedPresetId === p.id) { value = [...p.defaultValue]; savedValue = [...p.defaultValue]; }
  }
</script>

<div class="be-root">

  <!-- ── Left column: graph + controls ── -->
  <div class="be-left">
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
      {#if zoomedOut}
        <rect
          x={innerBoxX} y={innerBoxY}
          width={innerBoxSz} height={innerBoxSz}
          fill="none"
          stroke="var(--text-muted)"
          stroke-width="0.8"
          opacity="0.7"
        />
      {/if}
      <line
        x1={anchor0.x} y1={anchor0.y}
        x2={anchor1.x} y2={anchor1.y}
        stroke="var(--text-muted)" stroke-width="0.8" stroke-dasharray="3 3" opacity="0.4"
      />
      <path d={bezierPath(value)} fill="none"
        stroke={isDirty ? 'var(--accent)' : '#fff'}
        stroke-width={isDirty ? 1.5 : 0.5}
      />
      <line x1={anchor0.x} y1={anchor0.y} x2={h.p1.x} y2={h.p1.y} stroke="var(--text-muted)" stroke-width="1" opacity="0.55"/>
      <line x1={anchor1.x} y1={anchor1.y} x2={h.p2.x} y2={h.p2.y} stroke="var(--text-muted)" stroke-width="1" opacity="0.55"/>
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
      <circle cx={anchor0.x} cy={anchor0.y} r="2.5" fill="var(--text-muted)"/>
      <circle cx={anchor1.x} cy={anchor1.y} r="2.5" fill="var(--text-muted)"/>
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
            <line x1="3" y1="12" x2="21" y2="12"/>
            <polyline points="7 7 2 12 7 17"/>
            <polyline points="17 7 22 12 17 17"/>
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
        <button
          class="be-btn"
          class:be-btn-active={zoomedOut}
          onclick={toggleZoom}
          aria-label="Toggle zoomed-out view"
          title={zoomedOut ? 'Zoomed out: handles can extend past 0–1' : 'Zoom out to edit beyond 0–1'}
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <polyline points="15 3 21 3 21 9"/>
            <polyline points="9 21 3 21 3 15"/>
            <line x1="21" y1="3"  x2="14" y2="10"/>
            <line x1="3"  y1="21" x2="10" y2="14"/>
          </svg>
        </button>
      </div>
    </div>

    <div class="be-meta">
      {#if name}<code class="be-token">{name}</code>{/if}
      <span class="be-value">{bezierCssRounded(value)}</span>
    </div>
  </div>

  <!-- ── Presets column ── -->
  <div class="be-presets">
    <div class="be-presets-list">
      {#each presets as p (p.id)}
        <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
        <div
          class="be-preset-item"
          class:active={selectedPresetId === p.id}
          onclick={() => selectPreset(p)}
        >
          <svg class="be-preset-thumb" viewBox="0 0 40 40" xmlns="http://www.w3.org/2000/svg">
            <line x1="14" y1="0"  x2="14" y2="40" stroke="currentColor" opacity="0.1"/>
            <line x1="26" y1="0"  x2="26" y2="40" stroke="currentColor" opacity="0.1"/>
            <line x1="0"  y1="14" x2="40" y2="14" stroke="currentColor" opacity="0.1"/>
            <line x1="0"  y1="26" x2="40" y2="26" stroke="currentColor" opacity="0.1"/>
            <path
              d="M 4 36 C {4 + p.value[0]*32} {36 - p.value[1]*32} {4 + p.value[2]*32} {36 - p.value[3]*32} 36 4"
              fill="none" stroke="currentColor" stroke-width="1.6"
            />
          </svg>
          <input
            class="be-preset-name"
            type="text"
            value={p.name}
            readonly={editingPresetId !== p.id}
            onclick={(e) => editingPresetId === p.id && e.stopPropagation()}
            ondblclick={(e) => { e.stopPropagation(); editingPresetId = p.id; e.currentTarget.select(); }}
            onblur={(e) => { renamePreset(p.id, e.currentTarget.value); editingPresetId = null; }}
            onkeydown={(e) => { if (e.key === 'Enter') e.currentTarget.blur(); e.stopPropagation(); }}
          />
          <button
            class="be-preset-reset-btn"
            onclick={(e) => { e.stopPropagation(); resetPreset(p); }}
            title="Reset to default"
            tabindex="-1"
          >
            <svg width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="3 12 3 6 9 6"/>
              <path d="M3 12a9 9 0 1 0 3-6.7L3 6"/>
            </svg>
          </button>
        </div>
      {/each}
    </div>
  </div>

</div>

<style>
  .be-root {
    display: flex;
    flex-direction: row;
    gap: 12px;
    align-items: stretch;
    width: 400px;
    flex-shrink: 0;
  }

  .be-left {
    display: flex;
    flex-direction: column;
    gap: 12px;
    width: 240px;
    flex-shrink: 0;
  }

  /* ── Presets panel ── */
  .be-presets {
    display: flex;
    flex-direction: column;
    flex: 1;
    min-width: 0;
    align-self: flex-start;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    overflow: hidden;
  }

  .be-presets-list {
    flex: 1;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }

  .be-preset-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 7px 3px 5px;
    cursor: pointer;
    border-bottom: 1px solid var(--border);
    transition: background 80ms;
    position: relative;
    flex-shrink: 0;
  }

  .be-preset-item:last-child { border-bottom: none; }

  .be-preset-item:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
  }

  .be-preset-item.active {
    background: color-mix(in srgb, var(--accent) 10%, transparent);
  }

  .be-preset-item:hover .be-preset-reset-btn { opacity: 1; }

  .be-preset-thumb {
    width: 24px;
    height: 24px;
    flex-shrink: 0;
    color: var(--text-secondary);
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    border-radius: 3px;
  }

  .be-preset-item.active .be-preset-thumb { color: var(--accent); }

  .be-preset-name {
    flex: 1;
    min-width: 0;
    font-size: 10px;
    font-family: inherit;
    color: var(--text-primary);
    background: transparent;
    border: 1px solid transparent;
    border-radius: 3px;
    padding: 2px 3px;
    outline: none;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .be-preset-name:not([readonly]) {
    cursor: text;
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }

  .be-preset-reset-btn {
    flex-shrink: 0;
    width: 14px;
    height: 14px;
    padding: 0;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    opacity: 0;
    transition: opacity 80ms, background 80ms, color 80ms;
  }

  .be-preset-reset-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 10%, transparent);
    color: var(--text-primary);
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
