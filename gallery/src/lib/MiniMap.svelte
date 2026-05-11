<script>
  let { contentEl, visible } = $props();

  const W = 80;
  let canvas = $state(null);
  let vpTop  = $state(0);
  let vpH    = $state(0);

  let isDragging        = false;
  let dragYAtStart      = 0;
  let dragScrollAtStart = 0;

  function cssZoom() {
    return parseFloat(document.documentElement.style.zoom) || 1;
  }

  // Walk offsetParent chain up to contentEl, accumulating offsetTop.
  // Works because contentEl has overflow-y:auto, which WebKit treats as an
  // offsetParent boundary.
  function elOffsetTop(el) {
    let top = 0, cur = el;
    while (cur && cur !== contentEl) { top += cur.offsetTop; cur = cur.offsetParent; }
    return top;
  }

  function elOffsetLeft(el) {
    let left = 0, cur = el;
    while (cur && cur !== contentEl) { left += cur.offsetLeft; cur = cur.offsetParent; }
    return left;
  }

  // Parse --accent from the root; fall back to the canonical #2884c9.
  function accentRgb() {
    const raw = getComputedStyle(document.documentElement).getPropertyValue('--accent').trim();
    const m   = raw.match(/#([0-9a-f]{2})([0-9a-f]{2})([0-9a-f]{2})/i);
    return m ? [parseInt(m[1],16), parseInt(m[2],16), parseInt(m[3],16)] : [40, 132, 201];
  }

  // Draw a single element as a filled rounded rect, clamped to minimap bounds.
  function fillEl(ctx, el, yR, xS, color, radius = 1) {
    if (!el.offsetWidth || !el.offsetHeight) return;
    const x = elOffsetLeft(el) * xS;
    const y = elOffsetTop(el)  * yR;
    const w = Math.min(el.offsetWidth  * xS, W - x - 4);
    const h = Math.max(1.5, el.offsetHeight * yR);
    if (w < 1) return;
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.roundRect(Math.max(4, x), y, w, h, radius);
    ctx.fill();
  }

  function draw() {
    if (!canvas || !contentEl) return;

    const dpr    = window.devicePixelRatio || 1;
    const viewH  = contentEl.clientHeight;
    const totalH = contentEl.scrollHeight;
    const cW     = contentEl.clientWidth;
    if (!viewH || !totalH || !cW) return;

    const yR     = viewH / totalH;
    const xS     = (W - 8) / cW;
    const isDark = document.documentElement.classList.contains('dark');
    const [ar, ag, ab] = accentRgb();

    canvas.width        = W * dpr;
    canvas.height       = viewH * dpr;
    canvas.style.width  = W + 'px';
    canvas.style.height = viewH + 'px';

    const ctx = canvas.getContext('2d');
    ctx.scale(dpr, dpr);
    ctx.clearRect(0, 0, W, viewH);

    for (const sec of contentEl.querySelectorAll('.gallery-section')) {
      const sTop = elOffsetTop(sec) * yR;
      const sH   = sec.offsetHeight * yR;

      // ── Section h1 ────────────────────────────────────────────────────────
      const h1 = sec.querySelector('h1.section-h1');
      if (h1) {
        const hTop = elOffsetTop(h1) * yR;
        const hH   = Math.max(3, h1.offsetHeight * yR);
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.28)' : 'rgba(0,0,0,0.20)';
        ctx.fillRect(4, hTop, W - 8, hH);
        // Underline rule below h1
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.07)';
        ctx.fillRect(4, hTop + hH, W - 8, 0.5);
      }

      // ── Cards (sections that use the Card component) ──────────────────────
      const cards = sec.querySelectorAll('.card');

      if (cards.length === 0) {
        // Fallback for card-free sections (e.g. Typography): walk two levels
        // of children and draw them as nested block rects.
        const root = sec.querySelector('.section') ?? sec;
        for (const l1 of root.children) {
          if (!l1.offsetHeight || l1.offsetHeight < 5) continue;
          const l1x = Math.max(4, elOffsetLeft(l1) * xS);
          const l1y = elOffsetTop(l1) * yR;
          const l1w = Math.min(l1.offsetWidth * xS, W - l1x - 4);
          const l1h = Math.max(2, l1.offsetHeight * yR);
          if (l1w >= 2) {
            ctx.fillStyle = isDark ? 'rgba(255,255,255,0.04)' : 'rgba(0,0,0,0.035)';
            ctx.beginPath();
            ctx.roundRect(l1x, l1y, l1w, l1h, 1.5);
            ctx.fill();
          }
          for (const l2 of l1.children) {
            if (!l2.offsetHeight || l2.offsetHeight < 5) continue;
            const l2x = Math.max(4, elOffsetLeft(l2) * xS);
            const l2y = elOffsetTop(l2) * yR;
            const l2w = Math.min(l2.offsetWidth * xS, W - l2x - 4);
            const l2h = Math.max(1.5, l2.offsetHeight * yR);
            if (l2w < 2) continue;
            ctx.fillStyle = isDark ? 'rgba(255,255,255,0.075)' : 'rgba(0,0,0,0.065)';
            ctx.beginPath();
            ctx.roundRect(l2x, l2y, l2w, l2h, 1);
            ctx.fill();
          }
        }
      }

      for (const card of cards) {
        const cTop = elOffsetTop(card) * yR;
        const cH   = Math.max(6, card.offsetHeight * yR);
        const cX   = Math.max(4, elOffsetLeft(card) * xS);
        const cWmm = Math.min(card.offsetWidth * xS, W - cX - 4);
        if (cWmm < 2) continue;

        // Card body background
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.075)' : 'rgba(0,0,0,0.06)';
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, cH, 2);
        ctx.fill();

        // Card border
        ctx.strokeStyle = isDark ? 'rgba(255,255,255,0.09)' : 'rgba(0,0,0,0.09)';
        ctx.lineWidth = 0.5;
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, cH, 2);
        ctx.stroke();

        // Card header strip
        const headerEl = card.querySelector('.card-header');
        const stripH   = headerEl
          ? Math.max(2.5, headerEl.offsetHeight * yR)
          : Math.max(2.5, cH * 0.12);
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.09)' : 'rgba(0,0,0,0.07)';
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, stripH, [2, 2, 0, 0]);
        ctx.fill();

        // ── UI elements inside .card-body ──────────────────────────────────
        const body = card.querySelector('.card-body') ?? card;

        // Buttons
        const btnColor = `rgba(${ar},${ag},${ab},${isDark ? 0.5 : 0.42})`;
        for (const el of body.querySelectorAll('button')) {
          fillEl(ctx, el, yR, xS, btnColor, 1.5);
        }

        // Text-like inputs + selects + textareas
        const inputColor = isDark ? 'rgba(255,255,255,0.20)' : 'rgba(0,0,0,0.13)';
        for (const el of body.querySelectorAll(
          'input:not([type=range]):not([type=checkbox]):not([type=radio]):not([type=color]), select, textarea'
        )) {
          fillEl(ctx, el, yR, xS, inputColor, 1.5);
        }

        // Range sliders — draw as a centered thin bar so they read as tracks
        const sliderColor = `rgba(${ar},${ag},${ab},${isDark ? 0.38 : 0.30})`;
        for (const el of body.querySelectorAll('input[type=range]')) {
          if (!el.offsetWidth) continue;
          const sx  = Math.max(4, elOffsetLeft(el) * xS);
          const sw  = Math.min(el.offsetWidth * xS, W - sx - 4);
          const sy  = elOffsetTop(el) * yR + Math.max(1, el.offsetHeight * yR) * 0.5 - 1;
          ctx.fillStyle = sliderColor;
          ctx.beginPath();
          ctx.roundRect(sx, sy, Math.max(4, sw), 2, 1);
          ctx.fill();
        }

        // Checkboxes + radios — tiny squares/circles
        const checkColor = isDark ? 'rgba(255,255,255,0.25)' : 'rgba(0,0,0,0.18)';
        for (const el of body.querySelectorAll('input[type=checkbox], input[type=radio]')) {
          if (!el.offsetWidth) continue;
          const cx = Math.max(4, elOffsetLeft(el) * xS);
          const cy = elOffsetTop(el) * yR;
          const sz = Math.max(2, Math.min(el.offsetWidth * xS, el.offsetHeight * yR, 4));
          ctx.fillStyle = checkColor;
          ctx.beginPath();
          ctx.roundRect(cx, cy, sz, sz, 0.5);
          ctx.fill();
        }
      }

      // Section divider
      ctx.fillStyle = isDark ? 'rgba(255,255,255,0.07)' : 'rgba(0,0,0,0.07)';
      ctx.fillRect(0, sTop + sH - 0.5, W, 0.5);
    }

    vpTop = contentEl.scrollTop * yR;
    vpH   = Math.max(16, viewH * yR);
  }

  $effect(() => {
    if (!visible || !contentEl) return;

    let raf;
    const schedule = () => { cancelAnimationFrame(raf); raf = requestAnimationFrame(draw); };

    const ro       = new ResizeObserver(schedule);
    const darkMo   = new MutationObserver(schedule);
    // Re-draw when the page swaps in/out a section (childList changes inside
    // contentEl don't trigger ResizeObserver because contentEl itself is the
    // same fixed-size scroll viewport).
    const contentMo = new MutationObserver(schedule);

    contentEl.addEventListener('scroll', schedule, { passive: true });
    ro.observe(contentEl);
    darkMo.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] });
    contentMo.observe(contentEl, { childList: true, subtree: true });

    schedule();
    const t = setTimeout(schedule, 120);

    return () => {
      contentEl.removeEventListener('scroll', schedule);
      ro.disconnect();
      darkMo.disconnect();
      contentMo.disconnect();
      cancelAnimationFrame(raf);
      clearTimeout(t);
    };
  });

  // ── Click canvas to navigate ─────────────────────────────────────────────
  function onCanvasClick(e) {
    if (!contentEl || !canvas || isDragging) return;
    const zoom  = cssZoom();
    const rect  = canvas.getBoundingClientRect();
    const y     = (e.clientY - rect.top) / zoom;
    const ratio = contentEl.clientHeight / contentEl.scrollHeight;
    contentEl.scrollTo({ top: y / ratio - contentEl.clientHeight / 2, behavior: 'smooth' });
  }

  // ── Viewport indicator drag — pointer capture ────────────────────────────
  function onVpPointerDown(e) {
    isDragging        = true;
    dragYAtStart      = e.clientY;
    dragScrollAtStart = contentEl.scrollTop;
    e.currentTarget.setPointerCapture(e.pointerId);
    e.preventDefault();
    e.stopPropagation();
  }

  function onVpPointerMove(e) {
    if (!isDragging || !contentEl) return;
    const zoom  = cssZoom();
    const ratio = contentEl.clientHeight / contentEl.scrollHeight;
    contentEl.scrollTop = dragScrollAtStart + (e.clientY - dragYAtStart) / (ratio * zoom);
  }

  function onVpPointerUp(e) {
    isDragging = false;
    e.currentTarget.releasePointerCapture(e.pointerId);
    e.stopPropagation();
  }
</script>

<div class="minimap">
  {#if visible}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <canvas bind:this={canvas} class="mm-canvas" onclick={onCanvasClick}></canvas>

    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
      class="mm-vp"
      style="top:{vpTop}px;height:{vpH}px"
      onpointerdown={onVpPointerDown}
      onpointermove={onVpPointerMove}
      onpointerup={onVpPointerUp}
      role="scrollbar"
      aria-valuenow={0}
      aria-valuemin={0}
      aria-valuemax={100}
      aria-orientation="vertical"
    ></div>
  {/if}
</div>

<style>
  .minimap {
    position: relative;
    width: 80px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary) 6%);
    cursor: pointer;
    overflow: hidden;
    user-select: none;
    -webkit-user-select: none;
  }

  .mm-canvas {
    display: block;
    width: 80px;
  }

  .mm-vp {
    position: absolute;
    inset-inline: 0;
    min-height: 20px;
    background: color-mix(in srgb, var(--accent) 13%, transparent);
    border-top: 1px solid color-mix(in srgb, var(--accent) 36%, transparent);
    border-bottom: 1px solid color-mix(in srgb, var(--accent) 36%, transparent);
    cursor: grab;
    pointer-events: all;
    transition: background 120ms;
  }

  .mm-vp:hover  { background: color-mix(in srgb, var(--accent) 20%, transparent); }
  .mm-vp:active { cursor: grabbing; background: color-mix(in srgb, var(--accent) 26%, transparent); }
</style>
