<script>
  let { value = $bindable('#2884c9'), embedded = false } = $props();

  // ── Color math ────────────────────────────────────────────────
  function hsvToRgb(hh, ss, vv) {
    const c = vv * ss;
    const x = c * (1 - Math.abs(((hh / 60) % 2) - 1));
    const m = vv - c;
    let r = 0, g = 0, b = 0;
    if      (hh < 60)  { r = c; g = x; b = 0; }
    else if (hh < 120) { r = x; g = c; b = 0; }
    else if (hh < 180) { r = 0; g = c; b = x; }
    else if (hh < 240) { r = 0; g = x; b = c; }
    else if (hh < 300) { r = x; g = 0; b = c; }
    else               { r = c; g = 0; b = x; }
    return [Math.round((r + m) * 255), Math.round((g + m) * 255), Math.round((b + m) * 255)];
  }

  function hsvToHex(hh, ss, vv) {
    return '#' + hsvToRgb(hh, ss, vv).map(n => n.toString(16).padStart(2, '0')).join('');
  }

  function hexToHsv(hex) {
    if (!hex || !/^#[0-9a-fA-F]{6}$/.test(hex)) return null;
    const r = parseInt(hex.slice(1, 3), 16) / 255;
    const g = parseInt(hex.slice(3, 5), 16) / 255;
    const b = parseInt(hex.slice(5, 7), 16) / 255;
    const max = Math.max(r, g, b), min = Math.min(r, g, b), d = max - min;
    let hh = 0, ss = 0, vv = max;
    if (d > 0) {
      ss = d / max;
      if      (max === r) hh = ((g - b) / d + 6) % 6 * 60;
      else if (max === g) hh = ((b - r) / d + 2) * 60;
      else                hh = ((r - g) / d + 4) * 60;
    }
    return [hh, ss, vv];
  }

  // ── Initial state ─────────────────────────────────────────────
  const init = hexToHsv(value) ?? [206, 0.80, 0.79];
  let h = $state(init[0]);
  let s = $state(init[1]);
  let v = $state(init[2]);

  // ── Derived colors ────────────────────────────────────────────
  const currentColor = $derived(hsvToHex(h, s, v));
  const fullSatColor = $derived(hsvToHex(h, s, 1));

  $effect(() => { value = currentColor; });

  // ── Wheel constants ───────────────────────────────────────────
  const W = 152;
  const R = W / 2 - 1; // usable radius

  // ── Indicator position from H + S ────────────────────────────
  const indX = $derived(W / 2 + R * s * Math.cos(h * Math.PI / 180));
  const indY = $derived(W / 2 + R * s * Math.sin(h * Math.PI / 180));

  // ── DOM refs & popover state ──────────────────────────────────
  let dotEl     = $state(null);
  let popoverEl = $state(null);
  let canvasEl  = $state(null);
  let dragging  = $state(false);
  let expanded  = $state(embedded);
  let px = $state(0);
  let py = $state(0);

  // ── Draw hue+saturation wheel once canvas mounts ─────────────
  $effect(() => {
    if (!canvasEl || !expanded) return;
    const ctx = canvasEl.getContext('2d');
    const cx = W / 2, cy = W / 2;
    const img = ctx.createImageData(W, W);
    for (let y = 0; y < W; y++) {
      for (let x = 0; x < W; x++) {
        const dx = x - cx, dy = y - cy;
        const dist = Math.hypot(dx, dy);
        if (dist <= R) {
          const angle = Math.atan2(dy, dx);
          const hue   = ((angle * 180 / Math.PI) + 360) % 360;
          const sat   = dist / R;
          const [rr, gg, bb] = hsvToRgb(hue, sat, 1);
          const i = (y * W + x) * 4;
          img.data[i] = rr; img.data[i + 1] = gg; img.data[i + 2] = bb; img.data[i + 3] = 255;
        }
      }
    }
    ctx.putImageData(img, 0, 0);
  });

  // ── Open / close ──────────────────────────────────────────────
  function openPopover(e) {
    if (embedded) return;
    e.stopPropagation();
    const rect = dotEl.getBoundingClientRect();
    px = Math.max(8, Math.min(
      rect.left + rect.width / 2 - 96,        // 96 = half of 192px popover
      window.innerWidth - 200
    ));
    py = Math.max(8, Math.min(
      rect.bottom + 8,
      window.innerHeight - 248
    ));
    expanded = true;
  }

  function close() {
    if (!embedded) expanded = false;
  }

  // ── Outside-click + Escape dismiss ───────────────────────────
  $effect(() => {
    if (!expanded || embedded) return;
    function onDoc(e) {
      if (popoverEl && !popoverEl.contains(e.target) && !dotEl?.contains(e.target)) close();
    }
    function onKey(e) { if (e.key === 'Escape') close(); }
    document.addEventListener('click', onDoc);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('click', onDoc);
      document.removeEventListener('keydown', onKey);
    };
  });

  // ── Wheel pointer interaction ─────────────────────────────────
  function onWheelDown(e) {
    e.preventDefault();
    dragging = true;
    canvasEl.setPointerCapture(e.pointerId);
    pickHS(e);
  }
  function onWheelMove(e) { if (dragging) pickHS(e); }
  function onWheelUp()    { dragging = false; }

  function pickHS(e) {
    const rect = canvasEl.getBoundingClientRect();
    const dx = e.clientX - rect.left  - W / 2;
    const dy = e.clientY - rect.top   - W / 2;
    h = ((Math.atan2(dy, dx) * 180 / Math.PI) + 360) % 360;
    s = Math.min(Math.hypot(dx, dy) / R, 1);
  }
</script>

<!-- Minimized dot ─────────────────────────────────────────────── -->
{#if !embedded}
  <button
    bind:this={dotEl}
    class="cw-dot"
    style="background:{currentColor}"
    onclick={openPopover}
    aria-label="Pick color"
  ></button>
{/if}

<!-- Expanded popover ──────────────────────────────────────────── -->
{#if expanded}
  <div
    bind:this={popoverEl}
    class="cw-popover"
    class:cw-embedded={embedded}
    style={embedded ? '' : `left:${px}px;top:${py}px`}
    onclick={(e) => e.stopPropagation()}
  >
    <!-- Wheel + indicator -->
    <div class="cw-wheel-wrap">
      <canvas
        bind:this={canvasEl}
        width={W}
        height={W}
        class="cw-canvas"
        onpointerdown={onWheelDown}
        onpointermove={onWheelMove}
        onpointerup={onWheelUp}
      ></canvas>
      <div
        class="cw-ind"
        style="left:{indX}px;top:{indY}px;background:{currentColor}"
      ></div>
    </div>

    <!-- Value (brightness) slider -->
    <div class="cw-v-row">
      <div class="cw-v-track" style="--hue-color:{fullSatColor}">
        <input
          type="range" min="0" max="100"
          value={Math.round(v * 100)}
          oninput={(e) => v = +e.target.value / 100}
          class="cw-v-range"
        />
      </div>
      <span class="cw-v-label">{Math.round(v * 100)}</span>
    </div>

    <!-- Swatch + hex -->
    <div class="cw-footer">
      <div class="cw-swatch" style="background:{currentColor}"></div>
      <span class="cw-hex">{currentColor}</span>
    </div>
  </div>
{/if}

<style>
  /* ── Dot ─────────────────────────────────────────────────────── */
  .cw-dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    border: 2px solid color-mix(in srgb, var(--text-primary) 20%, transparent);
    box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.14);
    padding: 0;
    cursor: pointer;
    transition: transform 120ms, box-shadow 120ms;
    flex-shrink: 0;
  }
  .cw-dot:hover {
    transform: scale(1.2);
    box-shadow: 0 0 0 2px var(--accent);
  }
  .cw-dot:focus-visible {
    outline: none;
    box-shadow: 0 0 0 2.5px var(--accent);
  }

  /* ── Popover ─────────────────────────────────────────────────── */
  .cw-popover {
    position: fixed;
    z-index: 9000;
    width: 192px;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 12px;
    box-shadow:
      0 12px 32px rgba(0, 0, 0, 0.22),
      0 2px 8px rgba(0, 0, 0, 0.14),
      0 0 0 0.5px rgba(0, 0, 0, 0.08);
    animation: cw-in 110ms cubic-bezier(0.15, 0, 0.2, 1);
  }

  @keyframes cw-in {
    from { opacity: 0; transform: scale(0.88); }
    to   { opacity: 1; transform: scale(1); }
  }

  .cw-embedded {
    position: relative;
    box-shadow: none;
    border: none;
    animation: none;
    padding: 4px;
  }

  /* ── Wheel ───────────────────────────────────────────────────── */
  .cw-wheel-wrap {
    position: relative;
    width: 152px;
    height: 152px;
    margin: 0 auto 10px;
  }

  .cw-canvas {
    display: block;
    border-radius: 50%;
    cursor: crosshair;
    user-select: none;
    touch-action: none;
  }

  /* Indicator dot */
  .cw-ind {
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 2.5px solid #fff;
    box-shadow:
      0 0 0 1.5px rgba(0, 0, 0, 0.32),
      0 1px 4px rgba(0, 0, 0, 0.3);
    transform: translate(-50%, -50%);
    pointer-events: none;
  }

  /* ── Value slider ────────────────────────────────────────────── */
  .cw-v-row {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 10px;
  }

  .cw-v-track {
    flex: 1;
    height: 8px;
    border-radius: 4px;
    background: linear-gradient(to right, #000 0%, var(--hue-color) 100%);
    position: relative;
    display: flex;
    align-items: center;
  }

  .cw-v-range {
    position: absolute;
    left: 0;
    right: 0;
    top: 50%;
    transform: translateY(-50%);
    width: 100%;
    height: 20px;
    appearance: none;
    -webkit-appearance: none;
    background: transparent;
    cursor: pointer;
    outline: none;
    margin: 0;
  }
  .cw-v-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #fff;
    border: 2px solid rgba(0, 0, 0, 0.18);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.3);
    cursor: pointer;
  }

  .cw-v-label {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    width: 22px;
    text-align: right;
    flex-shrink: 0;
  }

  /* ── Footer ──────────────────────────────────────────────────── */
  .cw-footer {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .cw-swatch {
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px solid rgba(0, 0, 0, 0.12);
    flex-shrink: 0;
  }

  .cw-hex {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-secondary);
    letter-spacing: 0.04em;
  }
</style>
