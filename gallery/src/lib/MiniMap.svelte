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

      // ── Section h1 — render as legible text label, sized to fit width ────
      const h1 = sec.querySelector('h1.section-h1');
      if (h1) {
        const hTop  = elOffsetTop(h1) * yR;
        const hH    = Math.max(8, h1.offsetHeight * yR);
        const label = h1.textContent.trim();
        const maxW  = W - 8;
        ctx.save();
        // Measure at reference size, scale to fill ~90% of available width
        const refSize = 12;
        ctx.font = `600 ${refSize}px -apple-system, system-ui, sans-serif`;
        const refW    = ctx.measureText(label).width;
        const fontSize = Math.min(11, Math.max(5, (maxW / refW) * refSize * 0.9));
        ctx.font = `600 ${fontSize}px -apple-system, system-ui, sans-serif`;
        ctx.fillStyle = 'rgba(255,255,255,0.85)';
        ctx.textBaseline = 'middle';
        ctx.fillText(label, 4, hTop + hH / 2);
        ctx.restore();
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.08)' : 'rgba(0,0,0,0.07)';
        ctx.fillRect(4, hTop + hH, W - 8, 0.5);
      }

      // ── Group titles (h2.group-title) ─────────────────────────────────────
      for (const h2 of sec.querySelectorAll('h2.group-title')) {
        const hTop = elOffsetTop(h2) * yR;
        const hH   = Math.max(2, h2.offsetHeight * yR);
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.16)' : 'rgba(0,0,0,0.12)';
        ctx.beginPath();
        ctx.roundRect(4, hTop, (W - 8) * 0.55, hH, 0.5);
        ctx.fill();
      }

      // ── Card-free sections (e.g. Typography): render direct children as blocks
      const cards = sec.querySelectorAll('.card');
      if (cards.length === 0) {
        const root = sec.querySelector('.section') ?? sec;
        for (const child of root.children) {
          if (child.classList.contains('section-h1') || child.classList.contains('group-title')) continue;
          if (child.classList.contains('group-header')) continue;
          if (child.classList.contains('group-desc') || child.classList.contains('section-desc')) continue;
          if (child.tagName === 'P') continue;
          if (!child.offsetHeight || child.offsetHeight < 24) continue;
          const bx = Math.max(4, elOffsetLeft(child) * xS);
          const by = elOffsetTop(child) * yR;
          const bw = Math.min(child.offsetWidth * xS, W - bx - 4);
          const bh = Math.max(2, child.offsetHeight * yR);
          if (bw < 2) continue;
          ctx.fillStyle = isDark ? 'rgba(255,255,255,0.07)' : 'rgba(0,0,0,0.06)';
          ctx.beginPath();
          ctx.roundRect(bx, by, bw, bh, 1.5);
          ctx.fill();
        }
      }

      // ── Cards ──────────────────────────────────────────────────────────────
      for (const card of cards) {
        const cTop = elOffsetTop(card) * yR;
        const cH   = Math.max(6, card.offsetHeight * yR);
        const cX   = Math.max(4, elOffsetLeft(card) * xS);
        const cWmm = Math.min(card.offsetWidth * xS, W - cX - 4);
        if (cWmm < 2) continue;

        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.075)' : 'rgba(0,0,0,0.06)';
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, cH, 2);
        ctx.fill();

        ctx.strokeStyle = isDark ? 'rgba(255,255,255,0.09)' : 'rgba(0,0,0,0.09)';
        ctx.lineWidth = 0.5;
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, cH, 2);
        ctx.stroke();

        const headerEl = card.querySelector('.card-header');
        const stripH   = headerEl
          ? Math.max(2.5, headerEl.offsetHeight * yR)
          : Math.max(2.5, cH * 0.12);
        ctx.fillStyle = isDark ? 'rgba(255,255,255,0.09)' : 'rgba(0,0,0,0.07)';
        ctx.beginPath();
        ctx.roundRect(cX, cTop, cWmm, stripH, [2, 2, 0, 0]);
        ctx.fill();

        // ── Card-frame: component block inside the card body ──────────────
        const frameEl = card.querySelector('.card-frame');
        if (frameEl && frameEl.offsetWidth && frameEl.offsetHeight) {
          const fx = Math.max(cX + 1, elOffsetLeft(frameEl) * xS);
          const fy = elOffsetTop(frameEl) * yR;
          const fw = Math.min(frameEl.offsetWidth * xS, W - fx - 4);
          const fh = Math.max(2, frameEl.offsetHeight * yR);
          if (fw >= 2) {
            ctx.fillStyle = isDark ? 'rgba(255,255,255,0.06)' : 'rgba(0,0,0,0.05)';
            ctx.beginPath();
            ctx.roundRect(fx, fy, fw, fh, 1.5);
            ctx.fill();
          }
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
    background: color-mix(in srgb, var(--text-primary) 10%, transparent);
    border-top: 1px solid color-mix(in srgb, var(--text-primary) 20%, transparent);
    border-bottom: 1px solid color-mix(in srgb, var(--text-primary) 20%, transparent);
    cursor: grab;
    pointer-events: all;
    transition: background 120ms;
  }

  .mm-vp:hover  { background: color-mix(in srgb, var(--text-primary) 16%, transparent); }
  .mm-vp:active { cursor: grabbing; background: color-mix(in srgb, var(--text-primary) 22%, transparent); }
</style>
