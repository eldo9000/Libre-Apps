<script>
  import { Transport, Timecode, GlobalTabs, PanelTabs, SegmentedControl, Button, ColorWheel } from '@libre/ui';
  import Card from '../lib/Card.svelte';
  import { onDestroy } from 'svelte';
  import testColor1 from '../../images/test color 1.png';
  import testColor2 from '../../images/test color 2.png';

  // ── Test patterns for waveform alignment (SVG data URIs) ───────────────────
  const _svg = (body) => 'data:image/svg+xml;utf8,' + encodeURIComponent(
    `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 1600 900' preserveAspectRatio='none'>${body}</svg>`
  );
  const smpteBars = _svg(
    ['#bfbfbf', '#bfbf00', '#00bfbf', '#00bf00', '#bf00bf', '#bf0000', '#0000bf']
      .map((c, i) => `<rect x='${i * 1600 / 7}' width='${1600 / 7 + 0.5}' height='900' fill='${c}'/>`)
      .join('')
  );
  const STEP_COUNT = 11;
  const steppedGradient = _svg(
    Array.from({ length: STEP_COUNT }, (_, i) => {
      const v = Math.round(i * 255 / (STEP_COUNT - 1));
      return `<rect x='${i * 1600 / STEP_COUNT}' width='${1600 / STEP_COUNT + 0.5}' height='900' fill='rgb(${v},${v},${v})'/>`;
    }).join('')
  );
  const smoothGradient = _svg(
    `<defs><linearGradient id='g' x1='0' x2='1' y1='0' y2='0'><stop offset='0' stop-color='black'/><stop offset='1' stop-color='white'/></linearGradient></defs><rect width='1600' height='900' fill='url(#g)'/>`
  );

  // ── Playback state (shared across FLCK-2, FLCK-3) ──────────────────────────
  let playing = $state(false);
  let playbackRate = $state(1);
  let time = $state(0);
  const duration = 312.8;
  const fps = 24;

  let raf = 0;
  let lastTs = 0;

  function tick(ts) {
    if (lastTs) {
      const dt = (ts - lastTs) / 1000;
      time = Math.max(0, Math.min(duration, time + dt * playbackRate));
      if (time >= duration && playbackRate > 0) { playing = false; playbackRate = 1; }
      if (time <= 0 && playbackRate < 0)        { playing = false; playbackRate = 1; }
    }
    lastTs = ts;
    if (playing) raf = requestAnimationFrame(tick);
  }

  function startLoop() { cancelAnimationFrame(raf); lastTs = 0; raf = requestAnimationFrame(tick); }

  function togglePlay() {
    if (playing) { playing = false; playbackRate = 1; cancelAnimationFrame(raf); }
    else         { playing = true;  playbackRate = 1; startLoop(); }
  }
  function rewind()    { if (playing && playbackRate < 0) playbackRate = Math.max(playbackRate * 2, -8); else { playing = true; playbackRate = -1; startLoop(); } }
  function forward()   { if (playing && playbackRate > 0) playbackRate = Math.min(playbackRate * 2,  8); else { playing = true; playbackRate =  1; startLoop(); } }
  function skipStart() { time = 0;        playing = false; cancelAnimationFrame(raf); }
  function skipEnd()   { time = duration; playing = false; cancelAnimationFrame(raf); }

  onDestroy(() => cancelAnimationFrame(raf));

  // ── FLCK-1: Node strip ──────────────────────────────────────────────────────
  let activeClip = $state(1);
  const clips = [
    { id: 0, name: 'V1 — Intro',   kind: 'video' },
    { id: 1, name: 'V1 — City',    kind: 'video' },
    { id: 2, name: 'A1 — Narr',    kind: 'audio' },
    { id: 3, name: 'V1 — Bridge',  kind: 'video' },
    { id: 4, name: 'A2 — Music',   kind: 'audio' },
    { id: 5, name: 'V1 — Outro',   kind: 'video' },
  ];

  // ── FLCK-3: Transport bar track zoom ───────────────────────────────────────
  let trackZoom = $state('100%');
  const trackZoomOptions = [
    { value: '50%',  label: '50%'  },
    { value: '100%', label: '100%' },
    { value: '200%', label: '200%' },
  ];

  // ── FLCK-4: Compositor ─────────────────────────────────────────────────────
  const compositorImages = [
    { id: 'img1', label: 'Test Color 1', src: testColor1 },
    { id: 'img2', label: 'Test Color 2', src: testColor2 },
    { id: 'smpte',    label: 'SMPTE Bars',       src: smpteBars },
    { id: 'stepped',  label: 'Stepped Gradient', src: steppedGradient },
    { id: 'smooth',   label: 'Smooth Gradient',  src: smoothGradient },
  ];
  let compositorImage = $state('img1');
  let activeCompositorImage = $derived(compositorImages.find(i => i.id === compositorImage));
  let compositorTab = $state('color');
  const compositorTabs = [
    { id: 'color',   label: 'Color'   },
    { id: 'effects', label: 'Effects' },
    { id: 'audio',   label: 'Audio'   },
  ];

  // ── FLCK-5: Color Wheel ────────────────────────────────────────────────────
  let cwH = $state(210);
  let cwS = $state(0.15);
  let cwV = $state(0.325);
  let cwDragging    = $state(false);
  let cwPopDragging = $state(false); // true while compact popup wheel is being dragged
  let cwCompactOpen = $state(false);
  let cwWheelEl      = $state(null);
  let cwDotEl        = $state(null);
  let cwCompactPopEl = $state(null);
  let cwPopX  = $state(0);
  let cwPopY  = $state(0);
  let cwPopCx = $state(0); // viewport coords of wheel center
  let cwPopCy = $state(0);

  function cwHsvToHex(h, s, v) {
    const c = v * s, x = c * (1 - Math.abs(((h / 60) % 2) - 1)), m = v - c;
    let r = 0, g = 0, b = 0;
    if      (h < 60)  { r = c; g = x; b = 0; }
    else if (h < 120) { r = x; g = c; b = 0; }
    else if (h < 180) { r = 0; g = c; b = x; }
    else if (h < 240) { r = 0; g = x; b = c; }
    else if (h < 300) { r = x; g = 0; b = c; }
    else              { r = c; g = 0; b = x; }
    return '#' + [r, g, b].map(n => Math.round((n + m) * 255).toString(16).padStart(2, '0')).join('');
  }

  const cwCurrentColor = $derived(cwHsvToHex(cwH, cwS, cwV));
  const cwFullColor    = $derived(cwHsvToHex(cwH, cwS, 1));
  const cwIndAngleRad  = $derived((cwH + 270) % 360 * Math.PI / 180);
  const cwIndX         = $derived(80 + 72 * cwS * Math.cos(cwIndAngleRad));
  const cwIndY         = $derived(80 + 72 * cwS * Math.sin(cwIndAngleRad));
  // Light panel inner wheel is 180px (160 × 1.125)
  const lwIndX         = $derived(cwIndX * 1.125);
  const lwIndY         = $derived(cwIndY * 1.125);

  function cwPick(e, el) {
    const rect  = el.getBoundingClientRect();
    const dx    = e.clientX - rect.left - 80;
    const dy    = e.clientY - rect.top  - 80;
    const angle = (Math.atan2(dy, dx) * 180 / Math.PI + 360) % 360;
    cwH = (angle + 90) % 360;
    cwS = Math.min(Math.hypot(dx, dy) / 72, 1);
  }

  function cwOnDown(e) { e.preventDefault(); cwDragging = true; cwWheelEl.setPointerCapture(e.pointerId); cwPick(e, cwWheelEl); }
  function cwOnMove(e) { if (cwDragging) cwPick(e, cwWheelEl); }
  function cwOnUp()    { cwDragging = false; }

  // Gallery applies CSS zoom on <html>; clientX/Y are zoomed pixels, layout uses logical CSS px
  function getZoom() { return parseFloat(document.documentElement.style.zoom) || 1; }

  // Compact dot — pointerdown positions popover so wheel center lands under cursor, begins drag
  function openCwCompact(e) {
    e.preventDefault();
    e.stopPropagation();
    const z = getZoom();
    const cx = e.clientX / z;
    const cy = e.clientY / z;
    const offset = 92; // 12px padding + 80px (half of 160px wheel)
    let x = Math.max(8, Math.min(cx - offset, window.innerWidth  / z - 192));
    let y = Math.max(8, Math.min(cy - offset, window.innerHeight / z - 290));
    cwPopX  = x;
    cwPopY  = y;
    cwPopCx = x + offset; // logical wheel-center coords after clamping
    cwPopCy = y + offset;
    cwCompactOpen = true;
    cwPopDragging = true;
  }

  // Global drag handler for compact popup: tracks pointermove against stored wheel center coords
  $effect(() => {
    if (!cwCompactOpen || !cwPopDragging) return;
    function onMove(e) {
      const z = getZoom();
      const dx    = e.clientX / z - cwPopCx;
      const dy    = e.clientY / z - cwPopCy;
      const angle = (Math.atan2(dy, dx) * 180 / Math.PI + 360) % 360;
      cwH = (angle + 90) % 360;
      cwS = Math.min(Math.hypot(dx, dy) / 72, 1);
    }
    function onUp() { cwPopDragging = false; }
    document.addEventListener('pointermove', onMove);
    document.addEventListener('pointerup',   onUp);
    return () => {
      document.removeEventListener('pointermove', onMove);
      document.removeEventListener('pointerup',   onUp);
    };
  });

  // Popup wheel — re-press to drag again after initial drag ends
  function cwPopDown(e) { e.preventDefault(); cwPopDragging = true; }

  $effect(() => {
    if (!cwCompactOpen) return;
    function onDoc(e) {
      if (cwCompactPopEl && !cwCompactPopEl.contains(e.target) && !cwDotEl?.contains(e.target)) cwCompactOpen = false;
    }
    function onKey(e) { if (e.key === 'Escape') cwCompactOpen = false; }
    document.addEventListener('click',   onDoc);
    document.addEventListener('keydown', onKey);
    return () => {
      document.removeEventListener('click',   onDoc);
      document.removeEventListener('keydown', onKey);
    };
  });

  // ── FLCK-6: Curves channel ─────────────────────────────────────────────────
  let curvesChannel = $state('rgb');
  let curvesMode = $state('expanded');
  const curvesModeOptions = [
    { value: 'compact',  label: 'Compact'  },
    { value: 'standard', label: 'Standard' },
    { value: 'expanded', label: 'Expanded' },
  ];

  // ── FLCK-6: Bezier curve editor ───────────────────────────────────────────
  let channelPoints = $state({
    rgb: [{x:0,y:0},{x:1,y:1}],
    r:   [{x:0,y:0},{x:1,y:1}],
    g:   [{x:0,y:0},{x:1,y:1}],
    b:   [{x:0,y:0},{x:1,y:1}],
  });
  let curveDragIdx = $state(-1);

  function curvePath(pts, w, h) {
    if (pts.length < 2) return '';
    const sp = pts.map(p => [+(p.x * w).toFixed(2), +((1 - p.y) * h).toFixed(2)]);
    if (sp.length === 2) return `M${sp[0][0]},${sp[0][1]} L${sp[1][0]},${sp[1][1]}`;
    let d = `M${sp[0][0]},${sp[0][1]}`;
    for (let i = 0; i < sp.length - 1; i++) {
      const a = sp[Math.max(i-1,0)], b = sp[i], c = sp[i+1], e2 = sp[Math.min(i+2,sp.length-1)];
      const c1x = +(b[0]+(c[0]-a[0])/6).toFixed(2), c1y = +(b[1]+(c[1]-a[1])/6).toFixed(2);
      const c2x = +(c[0]-(e2[0]-b[0])/6).toFixed(2), c2y = +(c[1]-(e2[1]-b[1])/6).toFixed(2);
      d += ` C${c1x},${c1y} ${c2x},${c2y} ${c[0]},${c[1]}`;
    }
    return d;
  }

  function svgPt(e) {
    const el = e.currentTarget, r = el.getBoundingClientRect(), vb = el.viewBox.baseVal;
    return { x: (e.clientX-r.left)/r.width*vb.width, y: (e.clientY-r.top)/r.height*vb.height };
  }

  function curveDown(e, w, h) {
    e.preventDefault();
    const {x, y} = svgPt(e);
    const ch = curvesChannel;
    const pts = channelPoints[ch];
    const hitR = w * 0.024;
    for (let i = 0; i < pts.length; i++) {
      if (Math.hypot(pts[i].x*w - x, (1-pts[i].y)*h - y) < hitR) {
        if (e.detail === 2 && i !== 0 && i !== pts.length - 1) {
          channelPoints = {...channelPoints, [ch]: pts.filter((_,j) => j !== i)};
          return;
        }
        curveDragIdx = i;
        e.currentTarget.setPointerCapture(e.pointerId);
        return;
      }
    }
    if (e.detail > 1) return;
    const nx = +Math.max(0.005, Math.min(0.995, x/w)).toFixed(4);
    const ny = +Math.max(0, Math.min(1, 1-y/h)).toFixed(4);
    const next = [...pts, {x:nx,y:ny}].sort((a,b2) => a.x-b2.x);
    curveDragIdx = next.findIndex(p => p.x === nx && p.y === ny);
    channelPoints = {...channelPoints, [ch]: next};
    e.currentTarget.setPointerCapture(e.pointerId);
  }

  function curveMove(e, w, h) {
    if (curveDragIdx < 0) return;
    e.preventDefault();
    const {x, y} = svgPt(e);
    const ch = curvesChannel;
    const pts = channelPoints[ch];
    const first = curveDragIdx === 0, last = curveDragIdx === pts.length-1;
    const minX = first ? 0 : pts[curveDragIdx-1].x + 0.005;
    const maxX = last  ? 1 : pts[curveDragIdx+1].x - 0.005;
    channelPoints = {...channelPoints, [ch]: pts.map((p,i) => i !== curveDragIdx ? p : {
      x: first||last ? p.x : Math.max(minX, Math.min(maxX, x/w)),
      y: Math.max(0, Math.min(1, 1-y/h)),
    })};
  }

  function curveUp() { curveDragIdx = -1; }

  // ── FLCK-7: Scopes (BT.709 Y'CbCr density from compositorImage) ───────────
  let waveformHist = $state(null);      // Float32Array(256*256), density
  let waveformColorHist = $state(null); // Float32Array(256*256*3), accumulated R/G/B sums
  let waveformCanvasEl = $state(null);
  let wfColorize = $state(false);
  let wfDebug = $state(false);
  let wfHQ = $state(false);
  let paradeHQ = $state(false);
  let vectorscopeCanvasEl = $state(null);
  let vsZoom = $state(1);
  let vsGain = $state(1.0);
  let paradeCanvasEl = $state(null);
  let histogramCanvasEl = $state(null);
  let chromaticityCanvasEl = $state(null);
  let chromaZoom = $state(1);
  let chromaGain = $state(1.0);
  let chromaShowExtends = $state(false);
  let chromaExtendsColorize = $state(false);
  let chromaColorizeFill = $state(false);

  // 75% target box positions on viewBox 0..200 (center 100,100, outer r=95)
  // Scale = 95 / |R_100|_BT709 where |R_100| = sqrt(0.1146² + 0.5²) ≈ 0.5129
  const VS_SCALE = 95 / Math.hypot(0.1146, 0.5);
  const vsTargets = [
    { label: 'R',  Cb: -0.0859, Cr:  0.3750 },
    { label: 'M',  Cb:  0.2891, Cr:  0.1559 },
    { label: 'B',  Cb:  0.3750, Cr: -0.2191 },
    { label: 'C',  Cb:  0.0859, Cr: -0.3750 },
    { label: 'G',  Cb: -0.2891, Cr: -0.1559 },
    { label: 'Y',  Cb: -0.3750, Cr:  0.2191 },
  ].map(t => ({ ...t, x: 100 + t.Cb * VS_SCALE, y: 100 - t.Cr * VS_SCALE }));

  // I-line / skin-tone (123° CCW from +Cb, upper-left quadrant)
  const VS_I_RAD = 123 * Math.PI / 180;
  const vsILine = {
    x: 100 + Math.cos(VS_I_RAD) * 95,
    y: 100 - Math.sin(VS_I_RAD) * 95,
  };

  // Cb/Cr + RGB cache — rebuilt only when the source image changes
  let vsCbCache = $state(null);
  let vsCrCache = $state(null);
  let vsRCache  = $state(null);
  let vsGCache  = $state(null);
  let vsBCache  = $state(null);
  let vsShowExtends    = $state(false);
  let vsExtendsColorize = $state(false);
  let vsColorizeFill   = $state(false);

  // Stage 1: ingest image → cache Cb/Cr + RGB per pixel. Runs once per image change.
  $effect(() => {
    const src = activeCompositorImage?.src;
    if (!src) return;
    const img = new Image();
    img.crossOrigin = 'anonymous';
    let cancelled = false;
    img.onload = () => {
      if (cancelled) return;
      const W = 768, H = 768;
      const src_canvas = document.createElement('canvas');
      src_canvas.width = W; src_canvas.height = H;
      const sctx = src_canvas.getContext('2d');
      sctx.drawImage(img, 0, 0, W, H);
      const data = sctx.getImageData(0, 0, W, H).data;
      const N = W * H;
      const cb = new Float32Array(N);
      const cr = new Float32Array(N);
      const rArr = new Float32Array(N);
      const gArr = new Float32Array(N);
      const bArr = new Float32Array(N);
      for (let y = 0; y < H; y++) {
        for (let x = 0; x < W; x++) {
          const i = (y * W + x) * 4;
          const r = data[i] / 255, g = data[i + 1] / 255, b = data[i + 2] / 255;
          const Cb = -0.1146 * r - 0.3854 * g + 0.5000 * b;
          const Cr =  0.5000 * r - 0.4542 * g - 0.0458 * b;
          const idx = y * W + x;
          cb[idx] = Cb; cr[idx] = Cr;
          rArr[idx] = r; gArr[idx] = g; bArr[idx] = b;
        }
      }
      vsCbCache = cb; vsCrCache = cr;
      vsRCache = rArr; vsGCache = gArr; vsBCache = bArr;
    };
    img.src = src;
    return () => { cancelled = true; };
  });

  // Stage 1b: build waveform histogram. Re-runs on image change or HQ toggle.
  $effect(() => {
    const rC = vsRCache, gC = vsGCache, bC = vsBCache;
    const hq = wfHQ;
    if (!rC) return;
    const W = 768;
    const WF_COLS = hq ? 512 : 256;
    const WF_BINS = hq ? 256 : 192;
    const wfHist  = new Float32Array(WF_COLS * WF_BINS);
    const wfColor = new Float32Array(WF_COLS * WF_BINS * 3);
    const N = rC.length;
    if (hq) {
      for (let idx = 0; idx < N; idx++) {
        const r = rC[idx], g = gC[idx], b = bC[idx];
        const Y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        const x = idx % W;
        const fc = x / W * WF_COLS;
        const fb = Y * WF_BINS;
        const c0 = fc | 0, b0 = fb | 0;
        if (fc < 0 || fb < 0 || fc >= WF_COLS - 1 || fb >= WF_BINS - 1) continue;
        const dc = fc - c0, db = fb - b0;
        const w00 = (1 - dc) * (1 - db), w10 = dc * (1 - db);
        const w01 = (1 - dc) * db,        w11 = dc * db;
        const i00 = b0 * WF_COLS + c0, i10 = b0 * WF_COLS + c0 + 1;
        const i01 = (b0 + 1) * WF_COLS + c0, i11 = (b0 + 1) * WF_COLS + c0 + 1;
        wfHist[i00] += w00; wfHist[i10] += w10;
        wfHist[i01] += w01; wfHist[i11] += w11;
        wfColor[i00*3]   += r*w00; wfColor[i00*3+1] += g*w00; wfColor[i00*3+2] += b*w00;
        wfColor[i10*3]   += r*w10; wfColor[i10*3+1] += g*w10; wfColor[i10*3+2] += b*w10;
        wfColor[i01*3]   += r*w01; wfColor[i01*3+1] += g*w01; wfColor[i01*3+2] += b*w01;
        wfColor[i11*3]   += r*w11; wfColor[i11*3+1] += g*w11; wfColor[i11*3+2] += b*w11;
      }
    } else {
      for (let idx = 0; idx < N; idx++) {
        const r = rC[idx], g = gC[idx], b = bC[idx];
        const Y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        const x = idx % W;
        const col = Math.min(WF_COLS - 1, (x / W * WF_COLS) | 0);
        const bin = Math.min(WF_BINS - 1, (Y * WF_BINS) | 0);
        const cell = bin * WF_COLS + col;
        wfHist[cell]++;
        wfColor[cell * 3]     += r;
        wfColor[cell * 3 + 1] += g;
        wfColor[cell * 3 + 2] += b;
      }
    }
    waveformHist = wfHist;
    waveformColorHist = wfColor;
  });

  // Stage 3: render waveform density histogram to canvas.
  $effect(() => {
    const hist      = waveformHist;
    const colorHist = waveformColorHist;
    const colorize  = wfColorize;
    const hq        = wfHQ;
    const el = waveformCanvasEl;
    if (!hist || !el) return;

    const WF_COLS = hq ? 512 : 256;
    const WF_BINS = hq ? 256 : 192;
    const CW = el.width, CH = el.height;

    // Per-column normalization: each column normalizes to its own peak so sparse
    // columns (e.g. a single candle) are as visible as dense columns (the face).
    const colMax = new Float32Array(WF_COLS);
    for (let b = 0; b < WF_BINS; b++) {
      const row = b * WF_COLS;
      for (let c = 0; c < WF_COLS; c++) {
        const v = hist[row + c];
        if (v > colMax[c]) colMax[c] = v;
      }
    }

    const pixels = new Uint8ClampedArray(CW * CH * 4);
    for (let py = 0; py < CH; py++) {
      const bin = Math.min(WF_BINS - 1, Math.floor((1 - (py + 0.5) / CH) * WF_BINS));
      for (let px = 0; px < CW; px++) {
        const col  = Math.min(WF_COLS - 1, Math.floor((px + 0.5) / CW * WF_COLS));
        const cell = bin * WF_COLS + col;
        const density = colMax[col] > 0 ? hist[cell] / colMax[col] : 0;
        // gamma 0.55 matches vectorscope; linear alpha keeps sparse areas transparent
        const t = Math.pow(density, 0.55);

        let r = 0, g = 0, b = 0, a = 0;
        if (t > 0.01) {
          if (colorize && colorHist) {
            const w = hist[cell];
            const pr = w > 0 ? colorHist[cell * 3]     / w : 0;
            const pg = w > 0 ? colorHist[cell * 3 + 1] / w : 0;
            const pb = w > 0 ? colorHist[cell * 3 + 2] / w : 0;
            r = Math.min(255, Math.round(pr * 255));
            g = Math.min(255, Math.round(pg * 255));
            b = Math.min(255, Math.round(pb * 255));
            a = Math.min(255, Math.round(t * 255));
          } else {
            // heat: dark red → orange → warm white
            r = Math.min(255, Math.round(t < 0.5 ? t * 2 * 195 + 60 : 255));
            g = Math.min(255, Math.round(Math.pow(t, 0.55) * 250));
            b = Math.min(255, Math.round(Math.pow(t, 0.7) * 220));
            a = Math.min(255, Math.round(t * 255));
          }
        }
        const j = (py * CW + px) * 4;
        pixels[j] = r; pixels[j + 1] = g; pixels[j + 2] = b; pixels[j + 3] = a;
      }
    }
    el.getContext('2d').putImageData(new ImageData(pixels, CW, CH), 0, 0);
  });

  // Stage 2: bilinear-splat Cb/Cr cache into a 256×256 histogram, render.
  // Runs whenever gain changes (or cache rebuilds). Skips the matrix step.
  $effect(() => {
    const cb = vsCbCache, cr = vsCrCache;
    const gain = vsZoom;
    const displayGain = vsGain;
    const showExtends = vsShowExtends;
    const extendsColorize = vsExtendsColorize;
    const colorizeFill = vsColorizeFill;
    const rCache = vsRCache, gCache = vsGCache, bCache = vsBCache;
    if (!cb || !cr || !vectorscopeCanvasEl) return;

    const BIN = 256;
    const hist = new Float32Array(BIN * BIN);
    const N = cb.length;
    const histR = colorizeFill && rCache ? new Float32Array(BIN * BIN) : null;
    const histG = colorizeFill && gCache ? new Float32Array(BIN * BIN) : null;
    const histB = colorizeFill && bCache ? new Float32Array(BIN * BIN) : null;

    // Bilinear splat: each pixel distributes 1.0 across its 4 neighbor bins
    for (let i = 0; i < N; i++) {
      const fx = (cb[i] * gain + 0.5) * BIN;
      const fy = (0.5 - cr[i] * gain) * BIN;
      if (fx < 0 || fy < 0 || fx >= BIN - 1 || fy >= BIN - 1) continue;
      const x0 = fx | 0;
      const y0 = fy | 0;
      const dx = fx - x0;
      const dy = fy - y0;
      const idx00 = y0 * BIN + x0;
      const w00 = (1 - dx) * (1 - dy);
      const w10 = dx       * (1 - dy);
      const w01 = (1 - dx) * dy;
      const w11 = dx       * dy;
      hist[idx00]           += w00;
      hist[idx00 + 1]       += w10;
      hist[idx00 + BIN]     += w01;
      hist[idx00 + BIN + 1] += w11;
      if (histR) {
        const r = rCache[i], g = gCache[i], b = bCache[i];
        histR[idx00]           += r * w00; histR[idx00 + 1]       += r * w10;
        histR[idx00 + BIN]     += r * w01; histR[idx00 + BIN + 1] += r * w11;
        histG[idx00]           += g * w00; histG[idx00 + 1]       += g * w10;
        histG[idx00 + BIN]     += g * w01; histG[idx00 + BIN + 1] += g * w11;
        histB[idx00]           += b * w00; histB[idx00 + 1]       += b * w10;
        histB[idx00 + BIN]     += b * w01; histB[idx00 + BIN + 1] += b * w11;
      }
    }

    // Separable 3×3 Gaussian blur ([1,2,1]/4 horizontal then vertical).
    // Passes scale with gain. Color channels get the same treatment.
    const blurPasses = Math.floor(1 + gain / 3);
    const tmp = new Float32Array(BIN * BIN);

    const blurCh = (ch) => {
      for (let pass = 0; pass < blurPasses; pass++) {
        for (let y = 0; y < BIN; y++) {
          const row = y * BIN;
          for (let x = 0; x < BIN; x++) {
            const i = row + x;
            const l = x > 0       ? ch[i - 1] : ch[i];
            const r = x < BIN - 1 ? ch[i + 1] : ch[i];
            tmp[i] = (l + 2 * ch[i] + r) * 0.25;
          }
        }
        for (let y = 0; y < BIN; y++) {
          const row = y * BIN;
          for (let x = 0; x < BIN; x++) {
            const i = row + x;
            const u = y > 0       ? tmp[i - BIN] : tmp[i];
            const d = y < BIN - 1 ? tmp[i + BIN] : tmp[i];
            ch[i] = (u + 2 * tmp[i] + d) * 0.25;
          }
        }
      }
    };

    blurCh(hist);
    if (histR) { blurCh(histR); blurCh(histG); blurCh(histB); }

    // Map to brightness via gamma curve
    let max = 0;
    for (let i = 0; i < hist.length; i++) if (hist[i] > max) max = hist[i];
    const out = new Uint8ClampedArray(BIN * BIN * 4);
    const invMax = max > 0 ? 1 / max : 0;
    const invGamma = 1 / (2.2 * displayGain);
    for (let i = 0; i < hist.length; i++) {
      const v = Math.pow(hist[i] * invMax, invGamma);
      const j = i * 4;
      if (histR && hist[i] > 0) {
        // Normalize color to its brightest channel so gain fully controls luminance.
        // Raw avgR*v would suppress gain since dark pixels cap the output.
        const avgR = histR[i] / hist[i];
        const avgG = histG[i] / hist[i];
        const avgB = histB[i] / hist[i];
        const maxCh = Math.max(avgR, avgG, avgB, 0.001);
        const scale = v / maxCh;
        out[j]   = Math.min(255, Math.round(avgR * scale * 255));
        out[j+1] = Math.min(255, Math.round(avgG * scale * 255));
        out[j+2] = Math.min(255, Math.round(avgB * scale * 255));
        out[j+3] = Math.min(255, Math.round(v * 255));
      } else {
        const c = Math.min(255, Math.round(v * 255));
        out[j] = c; out[j+1] = c; out[j+2] = c; out[j+3] = c;
      }
    }
    const backing = document.createElement('canvas');
    backing.width = BIN; backing.height = BIN;
    backing.getContext('2d').putImageData(new ImageData(out, BIN, BIN), 0, 0);

    const ctx = vectorscopeCanvasEl.getContext('2d');
    ctx.imageSmoothingEnabled = true;
    ctx.clearRect(0, 0, vectorscopeCanvasEl.width, vectorscopeCanvasEl.height);
    ctx.drawImage(backing, 0, 0, vectorscopeCanvasEl.width, vectorscopeCanvasEl.height);

    // Extends: outermost boundary where any pixel has non-zero chroma.
    if (showExtends) {
      const NUM_BINS = 360;
      const extendDist = new Float32Array(NUM_BINS);
      const cx = BIN / 2, cy = BIN / 2;
      for (let i = 0; i < N; i++) {
        const cbv = cb[i], crv = cr[i];
        if (Math.abs(cbv) < 0.001 && Math.abs(crv) < 0.001) continue;
        const angle = Math.atan2(-crv, cbv);
        const deg = Math.floor(((angle * 180 / Math.PI) + 360) % 360);
        const dist = Math.sqrt(cbv * cbv + crv * crv) * gain * BIN;
        if (dist > extendDist[deg]) extendDist[deg] = dist;
      }
      // 8 passes of [1,2,1]/4 — approximates a wide Gaussian over the circular distance array
      const smoothed = new Float32Array(NUM_BINS);
      extendDist.forEach((v, i) => { smoothed[i] = v; });
      const tmp2 = new Float32Array(NUM_BINS);
      for (let pass = 0; pass < 8; pass++) {
        for (let deg = 0; deg < NUM_BINS; deg++) {
          const prev = smoothed[(deg + NUM_BINS - 1) % NUM_BINS];
          const next = smoothed[(deg + 1) % NUM_BINS];
          tmp2[deg] = (prev + 2 * smoothed[deg] + next) * 0.25;
        }
        smoothed.set(tmp2);
      }
      ctx.lineWidth = 1;
      if (!extendsColorize) {
        // Solid cyan — single path
        ctx.strokeStyle = 'rgba(80,220,255,0.65)';
        ctx.beginPath();
        let started = false;
        for (let deg = 0; deg < NUM_BINS; deg++) {
          if (smoothed[deg] <= 0) continue;
          const angle = deg * Math.PI / 180;
          const ex = cx + Math.cos(angle) * smoothed[deg];
          const ey = cy + Math.sin(angle) * smoothed[deg];
          if (!started) { ctx.moveTo(ex, ey); started = true; }
          else ctx.lineTo(ex, ey);
        }
        if (started) { ctx.closePath(); ctx.stroke(); }
      } else {
        // Per-degree color via BT.709 inverse. lineWidth=2 + round caps make
        // adjacent sub-pixel segments overlap into a continuous solid line.
        const S = 0.5, Y = 0.55;
        ctx.lineWidth = 2;
        ctx.lineCap = 'round';
        for (let deg = 0; deg < NUM_BINS; deg++) {
          if (smoothed[deg] <= 0) continue;
          const nextDeg = (deg + 1) % NUM_BINS;
          const angle     = deg      * Math.PI / 180;
          const nextAngle = nextDeg  * Math.PI / 180;
          const ex  = cx + Math.cos(angle)     * smoothed[deg];
          const ey  = cy + Math.sin(angle)     * smoothed[deg];
          const nex = cx + Math.cos(nextAngle) * smoothed[nextDeg];
          const ney = cy + Math.sin(nextAngle) * smoothed[nextDeg];
          const cbv = Math.cos(angle) * S;
          const crv = -Math.sin(angle) * S;
          const r = Math.min(255, Math.max(0, Math.round((Y + 1.5748 * crv) * 255)));
          const g = Math.min(255, Math.max(0, Math.round((Y - 0.1873 * cbv - 0.4681 * crv) * 255)));
          const b = Math.min(255, Math.max(0, Math.round((Y + 1.8556 * cbv) * 255)));
          ctx.strokeStyle = `rgba(${r},${g},${b},0.9)`;
          ctx.beginPath();
          ctx.moveTo(ex, ey);
          ctx.lineTo(nex, ney);
          ctx.stroke();
        }
        ctx.lineCap = 'butt';
      }
    }
  });

  // ── FLCK-15: Parade (RGB column-histograms, side-by-side) ──────────────────
  $effect(() => {
    const rCache = vsRCache, gCache = vsGCache, bCache = vsBCache;
    const hq = paradeHQ;
    const el = paradeCanvasEl;
    if (!rCache || !gCache || !bCache || !el) return;

    const W = 768;
    const PCOLS = hq ? 512 : 256;
    const PBINS = hq ? 256 : 192;
    const histR = new Float32Array(PCOLS * PBINS);
    const histG = new Float32Array(PCOLS * PBINS);
    const histB = new Float32Array(PCOLS * PBINS);
    const N = rCache.length;
    if (hq) {
      for (let idx = 0; idx < N; idx++) {
        const x = idx % W;
        const r = rCache[idx], g = gCache[idx], b = bCache[idx];
        const fc = x / W * PCOLS;
        const frB = r * PBINS, fgB = g * PBINS, fbB = b * PBINS;
        const c0 = fc | 0;
        if (fc < 0 || fc >= PCOLS - 1) continue;
        const dc = fc - c0;
        const wL = 1 - dc, wR = dc;
        const splat = (hist, fbin) => {
          const b0 = fbin | 0;
          if (fbin < 0 || fbin >= PBINS - 1) return;
          const db = fbin - b0;
          hist[b0 * PCOLS + c0]         += wL * (1 - db);
          hist[b0 * PCOLS + c0 + 1]     += wR * (1 - db);
          hist[(b0 + 1) * PCOLS + c0]   += wL * db;
          hist[(b0 + 1) * PCOLS + c0 + 1] += wR * db;
        };
        splat(histR, frB);
        splat(histG, fgB);
        splat(histB, fbB);
      }
    } else {
      for (let idx = 0; idx < N; idx++) {
        const x = idx % W;
        const col = Math.min(PCOLS - 1, (x / W * PCOLS) | 0);
        const r = rCache[idx], g = gCache[idx], b = bCache[idx];
        histR[Math.min(PBINS - 1, (r * PBINS) | 0) * PCOLS + col]++;
        histG[Math.min(PBINS - 1, (g * PBINS) | 0) * PCOLS + col]++;
        histB[Math.min(PBINS - 1, (b * PBINS) | 0) * PCOLS + col]++;
      }
    }
    const CW = el.width, CH = el.height;
    const stripW = Math.floor(CW / 3);
    const pixels = new Uint8ClampedArray(CW * CH * 4);

    const renderStrip = (hist, color, offsetX) => {
      const colMax = new Float32Array(PCOLS);
      for (let bn = 0; bn < PBINS; bn++) {
        const row = bn * PCOLS;
        for (let c = 0; c < PCOLS; c++) {
          const v = hist[row + c];
          if (v > colMax[c]) colMax[c] = v;
        }
      }
      for (let py = 0; py < CH; py++) {
        const bin = Math.min(PBINS - 1, ((1 - (py + 0.5) / CH) * PBINS) | 0);
        for (let px = 0; px < stripW; px++) {
          const col = Math.min(PCOLS - 1, ((px + 0.5) / stripW * PCOLS) | 0);
          const v = colMax[col] > 0 ? hist[bin * PCOLS + col] / colMax[col] : 0;
          const t = Math.pow(v, 0.55);
          const j = (py * CW + (px + offsetX)) * 4;
          pixels[j]     = Math.round(color[0] * t);
          pixels[j + 1] = Math.round(color[1] * t);
          pixels[j + 2] = Math.round(color[2] * t);
          pixels[j + 3] = Math.round(t * 255);
        }
      }
    };

    renderStrip(histR, [255, 80, 80],  0);
    renderStrip(histG, [80, 255, 100], stripW);
    renderStrip(histB, [100, 130, 255], stripW * 2);
    el.getContext('2d').putImageData(new ImageData(pixels, CW, CH), 0, 0);
  });

  // ── FLCK-16: Histogram (R, G, B, Y overlaid 256-bin curves) ───────────────
  $effect(() => {
    const rCache = vsRCache, gCache = vsGCache, bCache = vsBCache;
    const el = histogramCanvasEl;
    if (!rCache || !gCache || !bCache || !el) return;

    const BINS = 256;
    const hR = new Float32Array(BINS);
    const hG = new Float32Array(BINS);
    const hB = new Float32Array(BINS);
    const hY = new Float32Array(BINS);
    const N = rCache.length;
    for (let i = 0; i < N; i++) {
      const r = rCache[i], g = gCache[i], b = bCache[i];
      const y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
      hR[Math.min(BINS - 1, (r * BINS) | 0)]++;
      hG[Math.min(BINS - 1, (g * BINS) | 0)]++;
      hB[Math.min(BINS - 1, (b * BINS) | 0)]++;
      hY[Math.min(BINS - 1, (y * BINS) | 0)]++;
    }
    const smooth = (arr) => {
      const out = new Float32Array(BINS);
      for (let pass = 0; pass < 2; pass++) {
        const src = pass === 0 ? arr : out.slice();
        for (let i = 0; i < BINS; i++) {
          const l = i > 0          ? src[i - 1] : src[i];
          const r = i < BINS - 1   ? src[i + 1] : src[i];
          out[i] = (l + 2 * src[i] + r) * 0.25;
        }
      }
      return out;
    };
    const sR = smooth(hR), sG = smooth(hG), sB = smooth(hB), sY = smooth(hY);
    let max = 0;
    for (let i = 0; i < BINS; i++) {
      if (sR[i] > max) max = sR[i];
      if (sG[i] > max) max = sG[i];
      if (sB[i] > max) max = sB[i];
      if (sY[i] > max) max = sY[i];
    }
    const ctx = el.getContext('2d');
    const CW = el.width, CH = el.height;
    ctx.clearRect(0, 0, CW, CH);

    const drawCurve = (h, fill, stroke) => {
      ctx.beginPath();
      ctx.moveTo(0, CH);
      for (let i = 0; i < BINS; i++) {
        const x = (i / (BINS - 1)) * CW;
        const y = CH - (h[i] / max) * CH * 0.94;
        if (i === 0) ctx.lineTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.lineTo(CW, CH);
      ctx.closePath();
      ctx.fillStyle = fill;
      ctx.fill();
      ctx.beginPath();
      for (let i = 0; i < BINS; i++) {
        const x = (i / (BINS - 1)) * CW;
        const y = CH - (h[i] / max) * CH * 0.94;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.strokeStyle = stroke;
      ctx.lineWidth = 1;
      ctx.stroke();
    };

    ctx.globalCompositeOperation = 'lighter';
    drawCurve(sR, 'rgba(255, 60, 60, 0.38)',  'rgba(255, 110, 110, 0.95)');
    drawCurve(sG, 'rgba(60, 230, 90, 0.38)',  'rgba(120, 255, 140, 0.95)');
    drawCurve(sB, 'rgba(90, 120, 255, 0.38)', 'rgba(150, 170, 255, 0.95)');
    drawCurve(sY, 'rgba(220, 220, 220, 0.22)', 'rgba(255, 255, 255, 0.75)');
    ctx.globalCompositeOperation = 'source-over';
  });

  // ── FLCK-17: Chromaticity (CIE 1931 xy from compositorImage) ──────────────
  // Spectral locus (CIE 1931 2° observer, ~400–700nm sampled)
  const CIE_SPECTRAL_LOCUS = [
    [0.173,0.005],[0.171,0.005],[0.169,0.007],[0.164,0.011],[0.157,0.018],
    [0.144,0.030],[0.124,0.058],[0.091,0.133],[0.045,0.295],[0.008,0.538],
    [0.014,0.750],[0.074,0.834],[0.154,0.806],[0.230,0.754],[0.302,0.692],
    [0.373,0.625],[0.444,0.555],[0.512,0.487],[0.575,0.424],[0.627,0.373],
    [0.666,0.334],[0.691,0.308],[0.708,0.292],[0.719,0.281],[0.726,0.274],
    [0.730,0.270],[0.732,0.268],[0.734,0.266],
  ];
  const REC709_PRIMARIES = { r: [0.640, 0.330], g: [0.300, 0.600], b: [0.150, 0.060] };
  const D65 = [0.3127, 0.3290];
  const srgbToLinear = (c) => (c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4));

  // xy chromaticity → encoded sRGB at a reference saturation (used for boundary colorize).
  const xyToSrgb = (x, y) => {
    if (y < 1e-4) return [0, 0, 0];
    const X = x / y, Z = (1 - x - y) / y;
    let r =  3.2404542 * X - 1.5371385 - 0.4985314 * Z;
    let g = -0.9692660 * X + 1.8760108 + 0.0415560 * Z;
    let b =  0.0556434 * X - 0.2040259 + 1.0572252 * Z;
    const maxC = Math.max(r, g, b, 0.001);
    r = Math.max(0, r) / maxC;
    g = Math.max(0, g) / maxC;
    b = Math.max(0, b) / maxC;
    const enc = (v) => (v <= 0.0031308 ? 12.92 * v : 1.055 * Math.pow(v, 1 / 2.4) - 0.055);
    return [
      Math.min(255, Math.max(0, Math.round(enc(r) * 255))),
      Math.min(255, Math.max(0, Math.round(enc(g) * 255))),
      Math.min(255, Math.max(0, Math.round(enc(b) * 255))),
    ];
  };

  $effect(() => {
    const rCache = vsRCache, gCache = vsGCache, bCache = vsBCache;
    const zoom = chromaZoom;
    const displayGain = chromaGain;
    const showExtends = chromaShowExtends;
    const extendsColorize = chromaExtendsColorize;
    const colorizeFill = chromaColorizeFill;
    const el = chromaticityCanvasEl;
    if (!rCache || !gCache || !bCache || !el) return;

    const BIN = 256;
    const hist  = new Float32Array(BIN * BIN);
    const histR = colorizeFill ? new Float32Array(BIN * BIN) : null;
    const histG = colorizeFill ? new Float32Array(BIN * BIN) : null;
    const histB = colorizeFill ? new Float32Array(BIN * BIN) : null;
    const N = rCache.length;

    // Per-pixel xy storage for extends pass (keeps a single conversion path).
    const xys = showExtends ? new Float32Array(N * 2) : null;

    // Per-pixel: sRGB → linear → XYZ (D65) → xy, zoomed around D65, bilinear-splat into hist.
    for (let i = 0; i < N; i++) {
      const r = rCache[i], g = gCache[i], b = bCache[i];
      const rl = srgbToLinear(r), gl = srgbToLinear(g), bl = srgbToLinear(b);
      const X = 0.4124564 * rl + 0.3575761 * gl + 0.1804375 * bl;
      const Y = 0.2126729 * rl + 0.7151522 * gl + 0.0721750 * bl;
      const Z = 0.0193339 * rl + 0.1191920 * gl + 0.9503041 * bl;
      const sum = X + Y + Z;
      if (sum < 1e-6) {
        if (xys) { xys[i * 2] = NaN; xys[i * 2 + 1] = NaN; }
        continue;
      }
      const cx = X / sum, cy = Y / sum;
      if (xys) { xys[i * 2] = cx; xys[i * 2 + 1] = cy; }
      // Zoom around D65 (data center), then map to canvas.
      const zx = (cx - D65[0]) * zoom + D65[0];
      const zy = (cy - D65[1]) * zoom + D65[1];
      const fx = zx * BIN;
      const fy = (1 - zy) * BIN;
      if (fx < 0 || fy < 0 || fx >= BIN - 1 || fy >= BIN - 1) continue;
      const x0 = fx | 0, y0 = fy | 0;
      const dx = fx - x0, dy = fy - y0;
      const i00 = y0 * BIN + x0;
      const w00 = (1 - dx) * (1 - dy), w10 = dx * (1 - dy);
      const w01 = (1 - dx) * dy,       w11 = dx * dy;
      hist[i00]           += w00; hist[i00 + 1]       += w10;
      hist[i00 + BIN]     += w01; hist[i00 + BIN + 1] += w11;
      if (histR) {
        histR[i00]           += r * w00; histR[i00 + 1]       += r * w10;
        histR[i00 + BIN]     += r * w01; histR[i00 + BIN + 1] += r * w11;
        histG[i00]           += g * w00; histG[i00 + 1]       += g * w10;
        histG[i00 + BIN]     += g * w01; histG[i00 + BIN + 1] += g * w11;
        histB[i00]           += b * w00; histB[i00 + 1]       += b * w10;
        histB[i00 + BIN]     += b * w01; histB[i00 + BIN + 1] += b * w11;
      }
    }

    // Blur passes scale with zoom so dots stay continuous at higher zoom (matches vectorscope).
    const blurPasses = Math.floor(1 + zoom / 3);
    const tmp = new Float32Array(BIN * BIN);
    const blur = (ch) => {
      for (let pass = 0; pass < blurPasses; pass++) {
        for (let y = 0; y < BIN; y++) {
          const row = y * BIN;
          for (let x = 0; x < BIN; x++) {
            const i = row + x;
            const l = x > 0       ? ch[i - 1] : ch[i];
            const r = x < BIN - 1 ? ch[i + 1] : ch[i];
            tmp[i] = (l + 2 * ch[i] + r) * 0.25;
          }
        }
        for (let y = 0; y < BIN; y++) {
          const row = y * BIN;
          for (let x = 0; x < BIN; x++) {
            const i = row + x;
            const u = y > 0       ? tmp[i - BIN] : tmp[i];
            const d = y < BIN - 1 ? tmp[i + BIN] : tmp[i];
            ch[i] = (u + 2 * tmp[i] + d) * 0.25;
          }
        }
      }
    };
    blur(hist);
    if (histR) { blur(histR); blur(histG); blur(histB); }

    let max = 0;
    for (let i = 0; i < hist.length; i++) if (hist[i] > max) max = hist[i];
    const invMax = max > 0 ? 1 / max : 0;
    const invGamma = 1 / (2.2 * displayGain);

    const out = new Uint8ClampedArray(BIN * BIN * 4);
    for (let i = 0; i < hist.length; i++) {
      const v = Math.pow(hist[i] * invMax, invGamma);
      const j = i * 4;
      if (hist[i] > 0 && v > 0.01) {
        if (histR) {
          const avgR = histR[i] / hist[i];
          const avgG = histG[i] / hist[i];
          const avgB = histB[i] / hist[i];
          const maxCh = Math.max(avgR, avgG, avgB, 0.001);
          const scale = v / maxCh;
          out[j]   = Math.min(255, Math.round(avgR * scale * 255));
          out[j+1] = Math.min(255, Math.round(avgG * scale * 255));
          out[j+2] = Math.min(255, Math.round(avgB * scale * 255));
          out[j+3] = Math.min(255, Math.round(v * 255));
        } else {
          const c = Math.min(255, Math.round(v * 255));
          out[j] = c; out[j+1] = c; out[j+2] = c; out[j+3] = c;
        }
      }
    }
    const backing = document.createElement('canvas');
    backing.width = BIN; backing.height = BIN;
    backing.getContext('2d').putImageData(new ImageData(out, BIN, BIN), 0, 0);

    const CW = el.width, CH = el.height;
    const ctx = el.getContext('2d');
    ctx.imageSmoothingEnabled = true;
    ctx.clearRect(0, 0, CW, CH);
    ctx.drawImage(backing, 0, 0, CW, CH);

    // Extends: outermost boundary of the cluster, measured radially from D65.
    if (showExtends && xys) {
      const NUM_BINS = 360;
      const extendDist = new Float32Array(NUM_BINS);
      const cxPx = D65[0] * CW;
      const cyPx = (1 - D65[1]) * CH;
      for (let i = 0; i < N; i++) {
        const cx = xys[i * 2], cy = xys[i * 2 + 1];
        if (!Number.isFinite(cx)) continue;
        const dxC = cx - D65[0], dyC = cy - D65[1];
        if (Math.abs(dxC) < 1e-4 && Math.abs(dyC) < 1e-4) continue;
        // Canvas-space angle: y is flipped relative to CIE.
        const angle = Math.atan2(-dyC, dxC);
        const deg = Math.floor(((angle * 180 / Math.PI) + 360) % 360);
        const dist = Math.sqrt(dxC * dxC + dyC * dyC) * zoom * CW;
        if (dist > extendDist[deg]) extendDist[deg] = dist;
      }
      const smoothed = new Float32Array(NUM_BINS);
      extendDist.forEach((v, i) => { smoothed[i] = v; });
      const tmp2 = new Float32Array(NUM_BINS);
      for (let pass = 0; pass < 8; pass++) {
        for (let deg = 0; deg < NUM_BINS; deg++) {
          const prev = smoothed[(deg + NUM_BINS - 1) % NUM_BINS];
          const next = smoothed[(deg + 1) % NUM_BINS];
          tmp2[deg] = (prev + 2 * smoothed[deg] + next) * 0.25;
        }
        smoothed.set(tmp2);
      }
      ctx.lineWidth = 1;
      if (!extendsColorize) {
        ctx.strokeStyle = 'rgba(80,220,255,0.65)';
        ctx.beginPath();
        let started = false;
        for (let deg = 0; deg < NUM_BINS; deg++) {
          if (smoothed[deg] <= 0) continue;
          const a = deg * Math.PI / 180;
          const ex = cxPx + Math.cos(a) * smoothed[deg];
          const ey = cyPx + Math.sin(a) * smoothed[deg];
          if (!started) { ctx.moveTo(ex, ey); started = true; }
          else ctx.lineTo(ex, ey);
        }
        if (started) { ctx.closePath(); ctx.stroke(); }
      } else {
        // Per-degree color sampled at a fixed chromaticity radius from D65.
        const refSat = 0.18;
        ctx.lineWidth = 2;
        ctx.lineCap = 'round';
        for (let deg = 0; deg < NUM_BINS; deg++) {
          if (smoothed[deg] <= 0) continue;
          const nextDeg = (deg + 1) % NUM_BINS;
          const a  = deg     * Math.PI / 180;
          const an = nextDeg * Math.PI / 180;
          const ex  = cxPx + Math.cos(a)  * smoothed[deg];
          const ey  = cyPx + Math.sin(a)  * smoothed[deg];
          const nex = cxPx + Math.cos(an) * smoothed[nextDeg];
          const ney = cyPx + Math.sin(an) * smoothed[nextDeg];
          // CIE-space reference point (canvas y flipped → CIE y = -sin)
          const refX = D65[0] + Math.cos(a) * refSat;
          const refY = D65[1] - Math.sin(a) * refSat;
          const [R, G, B] = xyToSrgb(refX, refY);
          ctx.strokeStyle = `rgba(${R},${G},${B},0.9)`;
          ctx.beginPath();
          ctx.moveTo(ex, ey);
          ctx.lineTo(nex, ney);
          ctx.stroke();
        }
        ctx.lineCap = 'butt';
      }
    }
  });

  // ── FLCK-8: Color grading tabs ─────────────────────────────────────────────
  let colorTab = $state('primary');
  const colorTabs = [
    { id: 'primary',   label: 'Primary'   },
    { id: 'secondary', label: 'Secondary' },
    { id: 'luts',      label: 'LUTs'      },
    { id: 'hsl',       label: 'HSL'       },
  ];

  // ── FLCK-9: Mixer ──────────────────────────────────────────────────────────
  let mixerFaders = $state([80, 75, 60, 90, 100]);
  let mixerMutes  = $state([false, false, true, false, false]);
  const mixerStrips = ['A1', 'A2', 'A3', 'Music', 'Master'];

  // ── FLCK-10: Media pool ────────────────────────────────────────────────────
  let mediaQuery    = $state('');
  let selectedMedia = $state(0);
  const mediaItems = [
    { id: 0, name: 'city_walk.mp4',   kind: 'video' },
    { id: 1, name: 'narration.wav',   kind: 'audio' },
    { id: 2, name: 'bridge_shot.mp4', kind: 'video' },
    { id: 3, name: 'music_loop.aif',  kind: 'audio' },
    { id: 4, name: 'outro_clip.mp4',  kind: 'video' },
    { id: 5, name: 'fx_whoosh.wav',   kind: 'audio' },
  ];

  // ── FLCK-11: Folder tree ───────────────────────────────────────────────────
  let treeExpanded = $state({ footage: true, music: false, exports: false });

  // ── FLCK-12: Clip inspector (inline) ──────────────────────────────────────
  let inspClip = $state({ name: 'city_walk.mp4', posX: 0, posY: 0, scaleX: 1.0, scaleY: 1.0, rotation: 0, opacity: 100, blendMode: 'Normal', speed: 100, reverse: false });
  let inspCollapsed = $state({ transform: false, composite: false, speed: false });
  const blendModes = ['Normal', 'Multiply', 'Screen', 'Overlay', 'Add', 'Darken', 'Lighten'];

  function chevron(c) { return `transform:${c ? 'rotate(0deg)' : 'rotate(90deg)'}`; }

  // ── FLCK-13: Export / Delivery ─────────────────────────────────────────────
  let selectedFormat = $state(0);
  const formats = ['H.264 MP4', 'ProRes 422', 'DNxHD', 'WebM VP9', 'GIF'];
  let exportRes     = $state('1080p');
  let exportFps     = $state('23.976');
  let exportQuality = $state(85);
  const resOptions  = [
    { value: '720p',  label: '720p'  },
    { value: '1080p', label: '1080p' },
    { value: '4K',    label: '4K'    },
  ];
</script>

<div class="section">

  <!-- ── FLCK-1 Node Strip ──────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-1" label="Node Strip">
      <div class="node-strip">
        {#each clips as clip}
          <button class="node-clip" class:node-clip-active={activeClip === clip.id} onclick={() => (activeClip = clip.id)}>
            <div class="node-thumb" class:node-thumb-audio={clip.kind === 'audio'}></div>
            <span class="node-name">{clip.name}</span>
          </button>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── FLCK-2 Timeline ────────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-2" label="Timeline">
      <div class="timeline">
        <!-- Ruler -->
        <div class="tl-ruler">
          <div class="tl-track-label"></div>
          <div class="tl-ruler-ticks">
            {#each [0, 24, 48, 72, 96, 120] as f}
              <span class="tl-tick" style="left:{(f / 120) * 100}%">{f}</span>
            {/each}
          </div>
        </div>
        <!-- Tracks -->
        {#each [
          { label: 'V1', kind: 'video', clips: [{ left: 0, width: 35, title: 'Intro' }, { left: 37, width: 45, title: 'City Walk' }] },
          { label: 'A1', kind: 'audio', clips: [{ left: 0, width: 55, title: '' }] },
          { label: 'A2', kind: 'audio', clips: [{ left: 10, width: 70, title: '' }] },
        ] as track}
          <div class="tl-track">
            <div class="tl-track-label">{track.label}</div>
            <div class="tl-track-body">
              {#each track.clips as c}
                <div class="tl-clip" class:tl-clip-audio={track.kind === 'audio'} style="left:{c.left}%;width:{c.width}%">
                  {#if c.title}<span class="tl-clip-title">{c.title}</span>{/if}
                </div>
              {/each}
              <!-- Playhead -->
              <div class="tl-playhead" style="left:{(time / duration) * 100}%"></div>
            </div>
          </div>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── FLCK-3 Transport Bar ───────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-3" label="Transport Bar">
      <div class="transport-bar">
        <Timecode {time} {duration} {fps} />
        <Transport
          {playing}
          {playbackRate}
          onTogglePlay={togglePlay}
          onRewind={rewind}
          onForward={forward}
          onSkipStart={skipStart}
          onSkipEnd={skipEnd}
          showFullscreen
        />
        <div class="transport-right">
          <SegmentedControl options={trackZoomOptions} bind:value={trackZoom} variant="sliding" size="sm" />
        </div>
      </div>
    </Card>
  </div>

  <h2 class="group-title">Color</h2>

  <!-- ── Color tools row ────────────────────────────────────────────────────── -->
  <div class="cols">

    <!-- FLCK-5 Color Wheel -->
    <Card id="FLCK-5" label="Color Wheel">
      <div class="color-wheel-wrap">

        <!-- Expanded wheel -->
        <div class="cw-main-col">
          <span class="cw-compact-label">expanded</span>

          <div class="lw-panel">

            <!-- Header -->
            <div class="lw-header">
              <button class="lw-exp-btn" aria-label="Exposure">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round">
                  <circle cx="12" cy="12" r="4"/>
                  <line x1="12" y1="2"  x2="12" y2="4"/>
                  <line x1="12" y1="20" x2="12" y2="22"/>
                  <line x1="2"  y1="12" x2="4"  y2="12"/>
                  <line x1="20" y1="12" x2="22" y2="12"/>
                  <line x1="4.93" y1="4.93" x2="6.34" y2="6.34"/>
                  <line x1="17.66" y1="17.66" x2="19.07" y2="19.07"/>
                  <line x1="4.93" y1="19.07" x2="6.34" y2="17.66"/>
                  <line x1="17.66" y1="6.34" x2="19.07" y2="4.93"/>
                </svg>
                <span class="lw-exp-val">-1.00</span>
              </button>
              <span class="lw-title">Light</span>
              <button class="lw-reset-btn" aria-label="Reset">
                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M21 12a9 9 0 1 1-3-6.7"/>
                  <polyline points="21,3 21,9 15,9"/>
                </svg>
              </button>
            </div>

            <!-- Wheel area: arc brackets + handles + ticks + inner wheel -->
            <div class="lw-wheel-area">
              <!-- Arc brackets: thin curved lines on left + right (10→8 o'clock, 2→4) -->
              <svg class="lw-arcs" viewBox="0 0 240 240" aria-hidden="true">
                <path d="M 91.5 13.7 A 110 110 0 0 0 91.5 226.3" />
                <path d="M 148.5 13.7 A 110 110 0 0 1 148.5 226.3" />
              </svg>
              <!-- Arc slider handles -->
              <div class="lw-arc-handle lw-arc-handle--left"  tabindex="0" role="slider" aria-label="Arc left"></div>
              <div class="lw-arc-handle lw-arc-handle--right" tabindex="0" role="slider" aria-label="Arc right"></div>
              <!-- Color tick markers -->
              <div class="lw-tick lw-tick--white"></div>
              <div class="lw-tick lw-tick--red"></div>

              <div class="lw-inner-wheel">
                <div class="color-wheel lw-color-wheel"></div>
                <!-- Crosshair — joysticks live with compact popup H/S -->
                <div class="lw-center-cross" style="left:{lwIndX}px;top:{lwIndY}px">
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round">
                    <line x1="7" y1="0"  x2="7" y2="4.5"/>
                    <line x1="7" y1="9.5" x2="7" y2="14"/>
                    <line x1="0" y1="7"  x2="4.5" y2="7"/>
                    <line x1="9.5" y1="7" x2="14" y2="7"/>
                    <circle cx="7" cy="7" r="2" />
                  </svg>
                </div>
              </div>
            </div>

            <!-- Exp + Sat sliders -->
            <div class="lw-sliders">
              <div class="lw-slider">
                <div class="lw-slider-hd">
                  <span class="lw-slider-lbl">Exp</span>
                  <span class="lw-slider-val">{(cwV * 2 - 1).toFixed(2)}</span>
                </div>
                <div class="lw-slider-track">
                  <div class="lw-slider-thumb" style="left:{cwV * 100}%"></div>
                </div>
              </div>
              <div class="lw-slider">
                <div class="lw-slider-hd">
                  <span class="lw-slider-lbl">Sat</span>
                  <span class="lw-slider-val">{cwS.toFixed(2)}</span>
                </div>
                <div class="lw-slider-track">
                  <div class="lw-slider-thumb" style="left:{cwS * 100}%"></div>
                </div>
              </div>
            </div>

            <!-- Mini row -->
            <div class="lw-mini-row">
              <button class="lw-mini-item" aria-label="Tint">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linejoin="round">
                  <polygon points="12,3 22,21 2,21"/>
                </svg>
                <span class="lw-mini-val">0.00</span>
              </button>
              <button class="lw-mini-item" aria-label="Bracket">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M9 4 L4 12 L9 20"/>
                  <path d="M15 4 L20 12 L15 20"/>
                </svg>
                <span class="lw-mini-val">0.00</span>
              </button>
              <button class="lw-mini-item lw-mini-item--active" aria-label="Bars">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
                  <line x1="5"  y1="20" x2="5"  y2="14"/>
                  <line x1="12" y1="20" x2="12" y2="9"/>
                  <line x1="19" y1="20" x2="19" y2="4"/>
                </svg>
                <span class="lw-mini-val">0.22</span>
              </button>
            </div>

          </div>
        </div>

        <!-- Compact dot -->
        <div class="cw-compact-col">
          <span class="cw-compact-label">compact</span>
          <button
            class="cw-compact-dot"
            bind:this={cwDotEl}
            style="background:{cwCurrentColor}"
            onpointerdown={openCwCompact}
            aria-label="Pick color"
          ></button>
        </div>

      </div>
    </Card>

    <!-- Compact popover (fixed overlay, shared state) -->
    {#if cwCompactOpen}
      <div
        class="cw-compact-pop"
        bind:this={cwCompactPopEl}
        style="left:{cwPopX}px;top:{cwPopY}px"
        onclick={(e) => e.stopPropagation()}
      >
        <div
          class="color-wheel cw-pop-wheel"
          onpointerdown={cwPopDown}
        >
          <div class="cw-ind" style="left:{cwIndX}px;top:{cwIndY}px;background:{cwCurrentColor}"></div>
        </div>
        <div class="cw-v-row">
          <div class="cw-v-track" style="--hue-color:{cwFullColor}">
            <input
              type="range" min="0" max="100"
              value={Math.round(cwV * 100)}
              oninput={(e) => cwV = +e.target.value / 100}
              class="cw-v-range"
            />
          </div>
          <span class="cw-v-label">{Math.round(cwV * 100)}</span>
        </div>
        <div class="cw-footer-row">
          <div class="color-swatch" style="background:{cwCurrentColor}"></div>
          <span class="color-label">H {Math.round(cwH)}°  S {Math.round(cwS * 100)}%  V {Math.round(cwV * 100)}%</span>
        </div>
      </div>
    {/if}

    <!-- FLCK-4 Test Plate -->
    <div class="test-plate-grow">
    <Card id="FLCK-4" label="Test Plate">
      <div class="compositor">
        <div class="compositor-toolbar">
          {#each compositorImages as img}
            <button
              class="comp-img-btn"
              class:comp-img-btn--active={compositorImage === img.id}
              onclick={() => (compositorImage = img.id)}
            >{img.label}</button>
          {/each}
        </div>
        <div class="compositor-viewer">
          <div class="comp-frame">
            <img
              class="comp-canvas"
              src={activeCompositorImage?.src}
              alt={activeCompositorImage?.label}
            />
          </div>
        </div>
      </div>
    </Card>
    </div>

  </div>

  <!-- ── FLCK-6 + FLCK-8 row ──────────────────────────────────────────────── -->
  <div class="row curves-grade-row">
    <!-- FLCK-6 Curves Editor -->
    <div class="curves-grade-half">
      <div class="curves-mode-bar">
        <SegmentedControl options={curvesModeOptions} bind:value={curvesMode} size="sm" />
      </div>
      <div class="curves-editor-card">
      <Card id="FLCK-6" label="Curves Editor" flush>
        <div class="curves-wrap" class:curves-compact={curvesMode === 'compact'} class:curves-expanded={curvesMode === 'expanded'}>
          {#if curvesMode === 'expanded'}
            <svg class="curves-svg curves-svg-wide curves-interactive" viewBox="0 0 500 160" xmlns="http://www.w3.org/2000/svg"
              onpointerdown={(e) => curveDown(e, 500, 160)}
              onpointermove={(e) => curveMove(e, 500, 160)}
              onpointerup={curveUp}
              onpointercancel={curveUp}>
              {#each [40, 80, 120, 160, 200, 240, 280, 320, 360, 400, 440, 480] as gx}
                <line x1={gx} y1="0" x2={gx} y2="160" class="curves-grid-line"/>
              {/each}
              {#each [32, 64, 96, 128] as gy}
                <line x1="0" y1={gy} x2="500" y2={gy} class="curves-grid-line"/>
              {/each}
              <rect x="0" y="0" width="500" height="160" fill="none" class="curves-border"/>
              <path class="hist-luma" d="M0,160 L5,158 L10,152 L14,138 L17,116 L20,88 L23,58 L26,30 L28,14 L30,22 L32,36 L34,46 L36,52 L38,48 L40,56 L42,50 L44,60 L46,54 L48,64 L50,58 L53,68 L56,62 L59,70 L62,65 L65,72 L68,66 L71,74 L74,68 L77,76 L80,70 L83,78 L86,72 L89,80 L92,74 L95,82 L98,76 L101,83 L106,77 L111,84 L116,78 L121,85 L126,79 L131,86 L136,80 L141,87 L146,81 L151,88 L158,82 L165,89 L172,83 L180,90 L188,84 L196,91 L204,85 L212,92 L220,86 L230,93 L240,87 L250,94 L260,88 L270,95 L280,89 L290,96 L300,90 L306,86 L310,80 L314,72 L318,64 L320,58 L322,56 L324,60 L326,68 L330,78 L336,88 L344,96 L354,106 L366,116 L380,126 L393,134 L407,141 L421,147 L434,152 L447,155 L460,157 L474,159 L488,160 L500,160Z"/>
              <line x1="0" y1="160" x2="500" y2="0" class="curves-diagonal"/>
              <path d={curvePath(channelPoints[curvesChannel], 500, 160)} fill="none"
                class="curves-line {curvesChannel === 'r' ? 'curves-line-r' : curvesChannel === 'g' ? 'curves-line-g' : curvesChannel === 'b' ? 'curves-line-b' : ''}"/>
              {#each channelPoints[curvesChannel] as pt, i}
                <circle cx={pt.x * 500} cy={(1 - pt.y) * 160} r="4" class="curves-anchor" style="cursor: grab;"/>
              {/each}
            </svg>
          {:else}
            <svg class="curves-svg curves-interactive" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"
              style={curvesMode === 'compact' ? 'height:160px;max-width:176px;' : ''}
              onpointerdown={(e) => curveDown(e, 200, 200)}
              onpointermove={(e) => curveMove(e, 200, 200)}
              onpointerup={curveUp}
              onpointercancel={curveUp}>
              {#each [50, 100, 150] as g}
                <line x1={g} y1="0" x2={g} y2="200" class="curves-grid-line"/>
                <line x1="0" y1={g} x2="200" y2={g} class="curves-grid-line"/>
              {/each}
              <rect x="0" y="0" width="200" height="200" fill="none" class="curves-border"/>
              <path class="hist-luma" d="M0,200 L0,165 L3,148 L6,120 L8,18 L10,28 L12,55 L15,72 L18,82 L22,90 L28,100 L35,108 L42,113 L50,118 L60,124 L70,129 L80,133 L90,137 L100,140 L115,144 L130,148 L145,153 L160,160 L175,168 L185,175 L193,183 L198,190 L200,197 L200,200Z"/>
              <line x1="0" y1="200" x2="200" y2="0" class="curves-diagonal"/>
              <path d={curvePath(channelPoints[curvesMode === 'compact' ? 'rgb' : curvesChannel], 200, 200)} fill="none"
                class="curves-line {curvesMode !== 'compact' && curvesChannel === 'r' ? 'curves-line-r' : curvesMode !== 'compact' && curvesChannel === 'g' ? 'curves-line-g' : curvesMode !== 'compact' && curvesChannel === 'b' ? 'curves-line-b' : ''}"/>
              {#each channelPoints[curvesMode === 'compact' ? 'rgb' : curvesChannel] as pt, i}
                <circle cx={pt.x * 200} cy={(1 - pt.y) * 200} r="4" class="curves-anchor" style="cursor: grab;"/>
              {/each}
            </svg>
          {/if}
        </div>
      </Card>
      </div>
    </div>

    <!-- FLCK-8 Color Grading Tab -->
    <div class="curves-grade-half">
      <Card id="FLCK-8" label="Color Grading Tab">
      <div class="color-grade-wrap">
        <div class="cg-tabs-row">
          <GlobalTabs tabs={colorTabs} bind:active={colorTab} ariaLabel="Color grading" />
        </div>
        {#if colorTab === 'primary'}
          <div class="cg-wheels">
            {#each ['Lift', 'Gamma', 'Gain'] as wheel}
              <div class="cg-wheel-group">
                <div class="cg-wheel"></div>
                <span class="cg-wheel-label">{wheel}</span>
              </div>
            {/each}
          </div>
        {:else}
          <div class="cg-placeholder">
            {#if colorTab === 'secondary'}
              Secondary grading — select a qualifier
            {:else if colorTab === 'luts'}
              LUTs — no LUT applied
            {:else}
              HSL — hue / saturation / luminance ranges
            {/if}
          </div>
        {/if}
      </div>
    </Card>
    </div>
  </div>

  <h2 class="group-title">Scopes</h2>

  <!-- ── FLCK-7 / FLCK-14 / FLCK-15 / FLCK-16: Individual scope cards ──────── -->
  <div class="row scopes-row">
    <!-- FLCK-7 Waveform -->
    <div class="scope-card scope-card--wide">
      <Card id="FLCK-7" label="Waveform">
        <div class="scope-card-body">
          <div class="wf-wrap">
            <div class="wf-labels" aria-hidden="true">
              {#each [1023, 896, 768, 640, 512, 384, 256, 128, 0] as v}
                <span class="wf-label">{v}</span>
              {/each}
            </div>
            <div class="wf-scope">
              <canvas class="wf-canvas" bind:this={waveformCanvasEl} width="512" height="256"></canvas>
              <svg class="wf-grid" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" aria-hidden="true">
                {#each [0, 128, 256, 384, 512, 640, 768, 896, 1023] as v}
                  {@const gy = (1 - v / 1023) * 100}
                  <line x1="0" y1={gy} x2="100" y2={gy} stroke="#c8a020" stroke-width="0.4" vector-effect="non-scaling-stroke" opacity="0.7"/>
                {/each}
              </svg>
              {#if wfDebug}
                <img class="wf-debug-img" src={activeCompositorImage?.src} alt="" aria-hidden="true" />
              {/if}
            </div>
          </div>
          <div class="wf-controls">
            <label class="wf-toggle">
              <input type="checkbox" bind:checked={wfColorize} />
              Colorize
            </label>
            <label class="wf-toggle">
              <input type="checkbox" bind:checked={wfDebug} />
              Debug
            </label>
            <label class="wf-toggle">
              <input type="checkbox" bind:checked={wfHQ} />
              HQ
            </label>
          </div>
        </div>
      </Card>
    </div>

    <!-- FLCK-15 Parade -->
    <div class="scope-card scope-card--wide">
      <Card id="FLCK-15" label="Parade">
        <div class="scope-card-body">
          <div class="parade-wrap">
            <div class="wf-labels" aria-hidden="true">
              {#each [1023, 896, 768, 640, 512, 384, 256, 128, 0] as v}
                <span class="wf-label parade-label">{v}</span>
              {/each}
            </div>
            <div class="parade-scope">
              <canvas class="parade-canvas" bind:this={paradeCanvasEl} width="384" height="192"></canvas>
              <svg class="wf-grid" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" aria-hidden="true">
                {#each [0, 128, 256, 384, 512, 640, 768, 896, 1023] as v}
                  {@const gy = (1 - v / 1023) * 100}
                  <line x1="0" y1={gy} x2="100" y2={gy} stroke="#888" stroke-width="0.3" vector-effect="non-scaling-stroke" opacity="0.25"/>
                {/each}
                <line x1="33.33" y1="0" x2="33.33" y2="100" stroke="#666" stroke-width="0.5" vector-effect="non-scaling-stroke" opacity="0.6"/>
                <line x1="66.66" y1="0" x2="66.66" y2="100" stroke="#666" stroke-width="0.5" vector-effect="non-scaling-stroke" opacity="0.6"/>
              </svg>
              <div class="parade-channel-labels" aria-hidden="true">
                <span style="color:#ff5050">R</span>
                <span style="color:#50e060">G</span>
                <span style="color:#7090ff">B</span>
              </div>
            </div>
          </div>
          <div class="wf-controls">
            <label class="wf-toggle">
              <input type="checkbox" bind:checked={paradeHQ} />
              HQ
            </label>
          </div>
        </div>
      </Card>
    </div>

    <!-- FLCK-17 Chromaticity -->
    <div class="scope-card scope-card--wide">
      <Card id="FLCK-17" label="Chromaticity">
        <div class="scope-card-body">
          <div class="chroma-wrap">
            <div class="chroma-scope">
              <canvas class="chroma-canvas" bind:this={chromaticityCanvasEl} width="320" height="320"></canvas>
              <svg class="chroma-grid" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" aria-hidden="true">
                {#each [10, 20, 30, 40, 50, 60, 70, 80, 90] as gv}
                  <line x1={gv} y1="0" x2={gv} y2="100" stroke="#666" stroke-width="0.3" vector-effect="non-scaling-stroke" opacity="0.22"/>
                  <line x1="0" y1={gv} x2="100" y2={gv} stroke="#666" stroke-width="0.3" vector-effect="non-scaling-stroke" opacity="0.22"/>
                {/each}
                <polygon
                  points={CIE_SPECTRAL_LOCUS.map(([x, y]) => `${((x - D65[0]) * chromaZoom + D65[0]) * 100},${100 - ((y - D65[1]) * chromaZoom + D65[1]) * 100}`).join(' ')}
                  fill="none" stroke="#cccccc" stroke-width="0.7" vector-effect="non-scaling-stroke" opacity="0.85"
                />
                <polygon
                  points={[REC709_PRIMARIES.r, REC709_PRIMARIES.g, REC709_PRIMARIES.b].map(([x, y]) => `${((x - D65[0]) * chromaZoom + D65[0]) * 100},${100 - ((y - D65[1]) * chromaZoom + D65[1]) * 100}`).join(' ')}
                  fill="none" stroke="#c9a86e" stroke-width="0.6" vector-effect="non-scaling-stroke" opacity="0.9"
                />
                <circle cx={D65[0]*100} cy={100 - D65[1]*100} r="1.2" fill="none" stroke="#ffffff" stroke-width="0.5" vector-effect="non-scaling-stroke"/>
              </svg>
              <span class="chroma-rec709-label">Rec.709</span>
              {#each [0.0, 0.2, 0.4, 0.6, 0.8, 1.0] as v}
                <span class="chroma-tick chroma-tick-x" style="left:{v * 100}%">{v.toFixed(1)}</span>
              {/each}
              {#each [0.0, 0.2, 0.4, 0.6, 0.8, 1.0] as v}
                <span class="chroma-tick chroma-tick-y" style="bottom:{v * 100}%">{v.toFixed(1)}</span>
              {/each}
              <span class="chroma-axis-name chroma-axis-x">X</span>
              <span class="chroma-axis-name chroma-axis-y">Y</span>
            </div>
          </div>
          <div class="vs-controls">
            <div class="sld-wrap">
              <div class="sld-hd">
                <span class="sld-lbl">Zoom</span>
                <span class="sld-val">{chromaZoom.toFixed(1)}×</span>
              </div>
              <input
                type="range"
                class="form-range"
                min="1"
                max="8"
                step="0.1"
                bind:value={chromaZoom}
                style="--pct:{((chromaZoom - 1) / 7) * 100}%"
              />
            </div>
            <div class="sld-wrap" style="margin-top:6px">
              <div class="sld-hd">
                <span class="sld-lbl">Gain</span>
                <span class="sld-val">{chromaGain.toFixed(2)}</span>
              </div>
              <input
                type="range"
                class="form-range"
                min="0.25"
                max="4"
                step="0.05"
                bind:value={chromaGain}
                style="--pct:{((chromaGain - 0.25) / 3.75) * 100}%"
              />
            </div>
            <div class="vs-extends-row">
              <label class="vs-extends-toggle">
                <input type="checkbox" bind:checked={chromaShowExtends} />
                Extends
              </label>
              {#if chromaShowExtends}
                <label class="vs-extends-toggle">
                  <input type="checkbox" bind:checked={chromaExtendsColorize} />
                  Color
                </label>
              {/if}
            </div>
            <label class="vs-extends-toggle" style="margin-top:4px">
              <input type="checkbox" bind:checked={chromaColorizeFill} />
              Colorize Fill
            </label>
          </div>
        </div>
      </Card>
    </div>

    <!-- FLCK-14 Vectorscope -->
    <div class="scope-card">
      <Card id="FLCK-14" label="Vectorscope">
        <div class="scope-card-body">
          <div class="vectorscope">
            <canvas
              class="vectorscope-canvas"
              bind:this={vectorscopeCanvasEl}
              width="256"
              height="256"
            ></canvas>
            <svg class="vs-graticule" viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" aria-hidden="true">
              <circle cx="100" cy="100" r="95" fill="none" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke"/>
              <line x1="5"   y1="100" x2="195" y2="100" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke" opacity="0.6"/>
              <line x1="100" y1="5"   x2="100" y2="195" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke" opacity="0.6"/>
              <line x1="100" y1="100" x2={vsILine.x} y2={vsILine.y} stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke" opacity="0.5"/>
              {#each Array(60) as _, i}
                {@const a   = (i * 6) * Math.PI / 180}
                {@const maj = i % 5 === 0}
                {@const r1  = maj ? 88 : 92}
                <line
                  x1={100 + Math.cos(a) * r1}
                  y1={100 + Math.sin(a) * r1}
                  x2={100 + Math.cos(a) * 95}
                  y2={100 + Math.sin(a) * 95}
                  stroke="currentColor"
                  stroke-width="1"
                  vector-effect="non-scaling-stroke"
                  opacity={maj ? 1 : 0.55}
                />
              {/each}
              {#each vsTargets as t}
                <g transform="translate({t.x} {t.y})">
                  <path d="M-5,-2 L-5,-5 L-2,-5" fill="none" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke"/>
                  <path d="M2,-5 L5,-5 L5,-2"   fill="none" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke"/>
                  <path d="M-5,2 L-5,5 L-2,5"   fill="none" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke"/>
                  <path d="M2,5 L5,5 L5,2"     fill="none" stroke="currentColor" stroke-width="1" vector-effect="non-scaling-stroke"/>
                  <text x="7" y="6" font-size="7" font-family="Geist Mono, monospace" fill="currentColor">{t.label}</text>
                </g>
              {/each}
            </svg>
          </div>
          <div class="vs-controls">
            <div class="sld-wrap">
              <div class="sld-hd">
                <span class="sld-lbl">Zoom</span>
                <span class="sld-val">{vsZoom.toFixed(1)}×</span>
              </div>
              <input
                type="range"
                class="form-range"
                min="1"
                max="8"
                step="0.1"
                bind:value={vsZoom}
                style="--pct:{((vsZoom - 1) / 7) * 100}%"
              />
            </div>
            <div class="sld-wrap" style="margin-top:6px">
              <div class="sld-hd">
                <span class="sld-lbl">Gain</span>
                <span class="sld-val">{vsGain.toFixed(2)}</span>
              </div>
              <input
                type="range"
                class="form-range"
                min="0.25"
                max="4"
                step="0.05"
                bind:value={vsGain}
                style="--pct:{((vsGain - 0.25) / 3.75) * 100}%"
              />
            </div>
            <div class="vs-extends-row">
              <label class="vs-extends-toggle">
                <input type="checkbox" bind:checked={vsShowExtends} />
                Extends
              </label>
              {#if vsShowExtends}
                <label class="vs-extends-toggle">
                  <input type="checkbox" bind:checked={vsExtendsColorize} />
                  Color
                </label>
              {/if}
            </div>
            <label class="vs-extends-toggle" style="margin-top:4px">
              <input type="checkbox" bind:checked={vsColorizeFill} />
              Colorize Fill
            </label>
          </div>
        </div>
      </Card>
    </div>

    <!-- FLCK-16 Histogram -->
    <div class="scope-card">
      <Card id="FLCK-16" label="Histogram">
        <div class="scope-card-body">
          <div class="hist-wrap">
            <canvas class="hist-canvas" bind:this={histogramCanvasEl} width="256" height="160"></canvas>
            <svg class="hist-grid" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="none" aria-hidden="true">
              {#each [25, 50, 75] as gx}
                <line x1={gx} y1="0" x2={gx} y2="100" stroke="#666" stroke-width="0.4" vector-effect="non-scaling-stroke" opacity="0.35"/>
              {/each}
              {#each [25, 50, 75] as gy}
                <line x1="0" y1={gy} x2="100" y2={gy} stroke="#666" stroke-width="0.4" vector-effect="non-scaling-stroke" opacity="0.25"/>
              {/each}
            </svg>
          </div>
          <div class="hist-legend">
            <span class="hist-chip hist-chip--r">R</span>
            <span class="hist-chip hist-chip--g">G</span>
            <span class="hist-chip hist-chip--b">B</span>
            <span class="hist-chip hist-chip--y">Y</span>
          </div>
        </div>
      </Card>
    </div>
  </div>

  <h2 class="group-title">Audio</h2>

  <!-- ── FLCK-9 Audio Mixer ─────────────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-9" label="Audio Mixer">
      <div class="mixer">
        {#each mixerStrips as strip, i}
          <div class="mixer-strip" class:mixer-strip-master={strip === 'Master'}>
            <div class="mixer-vu">
              {#each [0, 1, 2, 3, 4] as seg}
                <div class="vu-seg" class:vu-green={seg < 3} class:vu-yellow={seg === 3} class:vu-red={seg === 4}
                  class:vu-active={mixerFaders[i] > (4 - seg) * 20}></div>
              {/each}
            </div>
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <input
              type="range"
              min="0"
              max="100"
              bind:value={mixerFaders[i]}
              class="mixer-fader"
              style="writing-mode: vertical-lr; direction: rtl;"
            />
            <div class="mixer-controls">
              <button class="mixer-btn" class:mixer-btn-muted={mixerMutes[i]} onclick={() => (mixerMutes[i] = !mixerMutes[i])} title="Mute">M</button>
              <button class="mixer-btn" title="Solo">S</button>
            </div>
            <span class="mixer-label">{strip}</span>
          </div>
        {/each}
      </div>
    </Card>
  </div>

  <!-- ── Side panels row ────────────────────────────────────────────────────── -->
  <div class="cols">

    <!-- FLCK-10 Media Pool -->
    <Card id="FLCK-10" label="Media Pool">
      <div class="media-pool">
        <div class="search-compact">
          <svg class="search-icon" width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
            <circle cx="11" cy="11" r="7"/><line x1="21" y1="21" x2="16.65" y2="16.65"/>
          </svg>
          <input type="search" class="search-input" placeholder="Search media…" bind:value={mediaQuery} />
        </div>
        <div class="media-grid">
          {#each mediaItems as item}
            <button class="media-item" class:media-item-active={selectedMedia === item.id} onclick={() => (selectedMedia = item.id)}>
              <div class="media-thumb" class:media-thumb-audio={item.kind === 'audio'}>
                {#if item.kind === 'video'}
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="currentColor" opacity="0.4"><path d="M8 5v14l11-7z"/></svg>
                {:else}
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" opacity="0.4"><path d="M9 18V5l12-2v13"/><circle cx="6" cy="18" r="3"/><circle cx="18" cy="16" r="3"/></svg>
                {/if}
              </div>
              <span class="media-name">{item.name}</span>
            </button>
          {/each}
        </div>
      </div>
    </Card>

    <!-- FLCK-11 Folder Tree -->
    <Card id="FLCK-11" label="Folder Tree">
      <div class="folder-tree">
        <!-- Root -->
        <div class="ft-row ft-root">
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name ft-name-root">Project Root</span>
        </div>
        <!-- Footage -->
        <button class="ft-row" onclick={() => (treeExpanded.footage = !treeExpanded.footage)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.footage)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Footage</span>
          <span class="ft-badge">3</span>
        </button>
        {#if treeExpanded.footage}
          {#each ['city_walk.mp4', 'bridge_shot.mp4', 'outro_clip.mp4'] as file}
            <div class="ft-row ft-row-child">
              <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-file"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zm-1 1.5L18.5 8H13V3.5z"/></svg>
              <span class="ft-name ft-name-file">{file}</span>
            </div>
          {/each}
        {/if}
        <!-- Music -->
        <button class="ft-row" onclick={() => (treeExpanded.music = !treeExpanded.music)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.music)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Music</span>
        </button>
        {#if treeExpanded.music}
          <div class="ft-row ft-row-child">
            <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-file"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6zm-1 1.5L18.5 8H13V3.5z"/></svg>
            <span class="ft-name ft-name-file">music_loop.aif</span>
          </div>
        {/if}
        <!-- Exports -->
        <button class="ft-row" onclick={() => (treeExpanded.exports = !treeExpanded.exports)}>
          <svg width="7" height="7" viewBox="0 0 24 24" fill="currentColor" class="ft-chev" style={chevron(!treeExpanded.exports)}><path d="M9 18l6-6-6-6"/></svg>
          <svg width="8" height="8" viewBox="0 0 24 24" fill="currentColor" class="ft-icon-folder"><path d="M20 6h-8l-2-2H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V8c0-1.1-.9-2-2-2z"/></svg>
          <span class="ft-name">Exports</span>
        </button>
      </div>
    </Card>

    <!-- FLCK-12 Clip Inspector (inline ir-* classes) -->
    <Card id="FLCK-12" label="Clip Inspector">
      <div class="ir">
        <div class="ir-clip-name">{inspClip.name}</div>

        <!-- Transform -->
        <div class="ir-section">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.transform = !inspCollapsed.transform)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Transform</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.transform)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.transform}
            {#each [['Position X', 'posX', 1], ['Position Y', 'posY', 1], ['Scale X', 'scaleX', 0.01], ['Scale Y', 'scaleY', 0.01], ['Rotation', 'rotation', 0.1], ['Opacity', 'opacity', 1]] as [label, key, step]}
              <div class="ir-row">
                <span class="ir-lbl">{label}</span>
                <input type="number" bind:value={inspClip[key]} step={step} class="ir-num" />
              </div>
            {/each}
          {/if}
        </div>

        <!-- Composite -->
        <div class="ir-section">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.composite = !inspCollapsed.composite)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Composite</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.composite)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.composite}
            <div class="ir-row">
              <span class="ir-lbl">Blend Mode</span>
              <select bind:value={inspClip.blendMode} class="ir-sel">
                {#each blendModes as m}<option>{m}</option>{/each}
              </select>
            </div>
          {/if}
        </div>

        <!-- Speed -->
        <div class="ir-section" style="border-bottom:none">
          <button class="ir-sec-hd" onclick={() => (inspCollapsed.speed = !inspCollapsed.speed)}>
            <span class="ir-dot" style="background:#555"></span>
            <span class="ir-sec-title">Speed</span>
            <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" class="ir-chev" style={chevron(inspCollapsed.speed)}><path d="M9 18l6-6-6-6"/></svg>
          </button>
          {#if !inspCollapsed.speed}
            <div class="ir-row">
              <span class="ir-lbl">Speed %</span>
              <input type="number" bind:value={inspClip.speed} min="1" max="400" step="1" class="ir-num" />
            </div>
            <div class="ir-row">
              <span class="ir-lbl">Reverse</span>
              <input type="checkbox" bind:checked={inspClip.reverse} class="ir-check" />
            </div>
          {/if}
        </div>
      </div>
    </Card>

  </div>

  <!-- ── FLCK-13 Export / Delivery ──────────────────────────────────────────── -->
  <div class="row">
    <Card id="FLCK-13" label="Export / Delivery">
      <div class="export-wrap">
        <!-- Format list -->
        <div class="export-formats">
          <div class="export-formats-hd">Presets</div>
          {#each formats as fmt, i}
            <button class="export-fmt-row" class:export-fmt-active={selectedFormat === i} onclick={() => (selectedFormat = i)}>
              {fmt}
            </button>
          {/each}
        </div>
        <!-- Settings -->
        <div class="export-settings">
          <div class="export-settings-hd">{formats[selectedFormat]}</div>

          <div class="export-field">
            <span class="export-label">Resolution</span>
            <SegmentedControl options={resOptions} bind:value={exportRes} variant="filled" size="sm" />
          </div>

          <div class="export-field">
            <label class="export-label" for="export-fps">Frame Rate</label>
            <select id="export-fps" bind:value={exportFps} class="export-select">
              {#each ['23.976', '24', '25', '29.97', '60'] as r}<option value={r}>{r} fps</option>{/each}
            </select>
          </div>

          <div class="export-field">
            <label class="export-label" for="export-quality">Quality — {exportQuality}</label>
            <input id="export-quality" type="range" min="0" max="100" bind:value={exportQuality} class="export-slider" />
          </div>

          <div class="export-field export-path-row">
            <span class="export-path">/Users/eldo/Movies/export_{formats[selectedFormat].replace(/ /g, '_').toLowerCase()}.mov</span>
            <button class="export-browse-btn">Browse</button>
          </div>

          <div class="export-action">
            <Button variant="primary" onclick={() => {}}>Export</Button>
          </div>
        </div>
      </div>
    </Card>
  </div>

</div>

<style>
  .section { max-width: 1375px; display: flex; flex-direction: column; gap: 20px; }

  .group-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 12px 0 -4px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }
  .row  { width: 100%; }
  .cols { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── FLCK-1: Node Strip ──────────────────────────────────────────────────── */
  .node-strip {
    display: flex;
    gap: 8px;
    overflow-x: auto;
    padding: 6px 4px;
  }
  .node-clip {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
    flex-shrink: 0;
  }
  .node-thumb {
    width: 80px;
    height: 44px;
    border-radius: 4px;
    background: var(--surface-raised);
    border: 1px solid var(--border);
  }
  .node-thumb-audio {
    border-color: color-mix(in srgb, var(--text-muted) 40%, var(--border));
    background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-muted));
  }
  .node-clip-active .node-thumb {
    border-color: color-mix(in srgb, var(--accent) 60%, var(--border));
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent) 30%, transparent);
  }
  .node-name {
    font-size: 9px;
    color: var(--text-muted);
    white-space: nowrap;
    font-family: 'Geist Mono', monospace;
  }

  /* ── FLCK-2: Timeline ────────────────────────────────────────────────────── */
  .timeline {
    display: flex;
    flex-direction: column;
    width: 100%;
    user-select: none;
  }
  .tl-ruler {
    display: flex;
    height: 18px;
    border-bottom: 1px solid var(--border);
  }
  .tl-ruler-ticks {
    position: relative;
    flex: 1;
    background: color-mix(in srgb, var(--surface-raised) 70%, var(--surface));
  }
  .tl-tick {
    position: absolute;
    transform: translateX(-50%);
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    top: 2px;
  }
  .tl-track {
    display: flex;
    height: 28px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .tl-track-label {
    width: 40px;
    flex-shrink: 0;
    font-size: 9px;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.06em;
    display: flex;
    align-items: center;
    padding-left: 8px;
    background: var(--surface-raised);
    border-right: 1px solid var(--border);
  }
  .tl-track-body {
    flex: 1;
    position: relative;
    background: color-mix(in srgb, var(--surface) 80%, var(--surface-raised));
    overflow: hidden;
  }
  .tl-clip {
    position: absolute;
    top: 3px;
    height: 22px;
    border-radius: 3px;
    background: color-mix(in srgb, var(--accent) 22%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--accent) 40%, var(--border));
    overflow: hidden;
  }
  .tl-clip-audio {
    background: repeating-linear-gradient(
      90deg,
      color-mix(in srgb, var(--text-secondary) 12%, transparent) 0px,
      color-mix(in srgb, var(--text-secondary) 12%, transparent) 2px,
      transparent 2px,
      transparent 6px
    );
    border-color: color-mix(in srgb, var(--text-muted) 30%, var(--border));
  }
  .tl-clip-title {
    font-size: 8px;
    color: var(--text-secondary);
    padding: 0 4px;
    line-height: 22px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  .tl-playhead {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 1px;
    background: var(--accent);
    pointer-events: none;
    z-index: 10;
  }
  .tl-playhead::before {
    content: '';
    position: absolute;
    top: 0;
    left: -3px;
    width: 7px;
    height: 7px;
    background: var(--accent);
    clip-path: polygon(50% 100%, 0 0, 100% 0);
  }

  /* ── FLCK-3: Transport Bar ───────────────────────────────────────────────── */
  .transport-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 6px 8px;
    background: var(--surface-raised);
    border-radius: 6px;
  }
  .transport-right {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  /* ── FLCK-4: Compositor ──────────────────────────────────────────────────── */
  .test-plate-grow {
    flex: 1;
    min-width: 0;
    align-self: stretch;
    display: flex;
    flex-direction: column;
  }
  .test-plate-grow :global(.card) {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .test-plate-grow :global(.card-body) {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .compositor {
    flex: 1;
    display: flex;
    flex-direction: column;
    width: 100%;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }
  .compositor-viewer {
    flex: 1;
    background: color-mix(in srgb, var(--surface) 40%, #000);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
  }
  .comp-frame {
    width: 100%;
    aspect-ratio: 16 / 9;
    overflow: hidden;
    position: relative;
  }
  .comp-canvas {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }
  .compositor-toolbar {
    display: flex;
    gap: 4px;
    padding: 6px 8px;
    background: var(--surface-raised);
    border-bottom: 1px solid var(--border);
  }
  .comp-img-btn {
    padding: 3px 10px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 12px;
    cursor: pointer;
    transition: background 120ms, color 120ms;
  }
  .comp-img-btn:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
    color: var(--text-primary);
  }
  .comp-img-btn--active {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }
  /* ── FLCK-5: Color Wheel ─────────────────────────────────────────────────── */
  .color-wheel-wrap {
    display: flex;
    flex-direction: row;
    align-items: flex-start;
    gap: 20px;
    padding: 12px;
  }
  .cw-main-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 6px;
    gap: 10px;
  }
  .cw-compact-col {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding-top: 6px;
  }
  .cw-compact-label {
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .cw-compact-dot {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    border: 2.5px solid color-mix(in srgb, var(--text-primary) 20%, transparent);
    box-shadow: 0 0 0 1px rgba(0,0,0,0.18);
    padding: 0;
    cursor: pointer;
    transition: transform 120ms, box-shadow 120ms;
    flex-shrink: 0;
  }
  .cw-compact-dot:hover {
    transform: scale(1.18);
    box-shadow: 0 0 0 2.5px var(--accent);
  }
  .cw-compact-pop {
    position: fixed;
    z-index: 9000;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 12px;
    box-shadow:
      0 12px 32px rgba(0,0,0,0.22),
      0 2px 8px rgba(0,0,0,0.14),
      0 0 0 0.5px rgba(0,0,0,0.08);
    animation: cw-pop-in 110ms cubic-bezier(0.15, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
  }
  @keyframes cw-pop-in {
    from { opacity: 0; transform: scale(0.88); }
    to   { opacity: 1; transform: scale(1); }
  }
  .cw-pop-wheel {
    width: 160px;
    height: 160px;
  }

  /* ── Light panel — structural redesign (hover-only interactivity) ────────── */
  .lw-panel {
    width: 320px;
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 0;
  }

  /* Header */
  .lw-header {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 0 4px;
  }
  .lw-exp-btn,
  .lw-reset-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 3px 4px;
    border-radius: 4px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0;
    font-family: inherit;
    transition: color 120ms;
  }
  .lw-exp-btn:hover,
  .lw-reset-btn:hover { color: var(--text-primary); }
  .lw-reset-btn { align-self: flex-start; }
  .lw-exp-val {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    line-height: 1.2;
  }
  .lw-exp-btn:hover .lw-exp-val { color: var(--text-primary); }
  .lw-title {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 13px;
    color: var(--text-secondary);
    pointer-events: none;
  }

  /* Wheel area */
  .lw-wheel-area {
    position: relative;
    width: 240px;
    height: 240px;
    margin: 0 auto;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Arc brackets (SVG) */
  .lw-arcs {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    overflow: visible;
  }
  .lw-arcs path {
    fill: none;
    stroke: color-mix(in srgb, var(--text-primary) 28%, transparent);
    stroke-width: 1.5;
    stroke-linecap: round;
  }

  /* Arc handles — positioned at 9 o'clock + 3 o'clock on radius-110 arc */
  .lw-arc-handle {
    position: absolute;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: color-mix(in srgb, var(--text-primary) 88%, transparent);
    box-shadow: 0 0 0 0.5px rgba(0,0,0,0.45);
    top: 50%;
    cursor: grab;
    transition: background 120ms, transform 120ms;
    outline: none;
  }
  .lw-arc-handle--left  { left: 10px;              transform: translate(-50%, -50%); }
  .lw-arc-handle--right { left: calc(100% - 10px); transform: translate(-50%, -50%); }
  .lw-arc-handle:hover,
  .lw-arc-handle:focus-visible { background: var(--text-primary); }
  .lw-arc-handle--left:hover,
  .lw-arc-handle--left:focus-visible  { transform: translate(-50%, -50%) scale(1.35); }
  .lw-arc-handle--right:hover,
  .lw-arc-handle--right:focus-visible { transform: translate(-50%, -50%) scale(1.35); }
  .lw-arc-handle:active { cursor: grabbing; }

  /* Color tick markers — sit in the gap between wheel edge (r=90) and arc (r=110) */
  .lw-tick {
    position: absolute;
    pointer-events: none;
  }
  .lw-tick--white {
    /* Small round dot on the left arc at ~10:30 position (upper-left) */
    width: 4px;
    height: 4px;
    border-radius: 50%;
    background: #fff;
    top: 56px;
    left: 28px;
    transform: translate(-50%, -50%);
    box-shadow: 0 0 0 0.5px rgba(0,0,0,0.55);
  }
  .lw-tick--red {
    /* Short red arc segment along right arc at ~4:30 position */
    width: 16px;
    height: 3px;
    border-radius: 2px;
    background: #ff3b30;
    bottom: 48px;
    right: 18px;
    transform: rotate(58deg);
  }

  /* Inner wheel container */
  .lw-inner-wheel {
    position: relative;
    width: 180px;
    height: 180px;
    z-index: 1;
  }
  .lw-inner-wheel .lw-color-wheel {
    width: 180px;
    height: 180px;
    box-shadow: none;
    cursor: crosshair;
    /* Resolve-style orientation: red top, blue right, green bottom, yellow left */
    background: conic-gradient(from -30deg, red, magenta, blue, cyan, lime, yellow, red);
  }
  .lw-inner-wheel .lw-color-wheel::after {
    /* Soft vignette — keeps wheel vibrant (overrides base dark overlay) */
    background: radial-gradient(circle at center,
      rgba(0,0,0,0.22) 0%,
      rgba(0,0,0,0.08) 40%,
      rgba(0,0,0,0)    62%);
  }
  .lw-inner-wheel .lw-color-wheel:hover {
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--text-primary) 22%, transparent);
  }
  .lw-center-cross {
    position: absolute;
    transform: translate(-50%, -50%);
    color: rgba(255,255,255,0.95);
    pointer-events: none;
    filter: drop-shadow(0 0 1.5px rgba(0,0,0,0.85));
    transition: left 60ms linear, top 60ms linear;
  }

  /* Sliders (Exp + Sat) */
  .lw-sliders {
    display: flex;
    gap: 18px;
    padding: 10px 6px 4px;
  }
  .lw-slider {
    flex: 1;
    cursor: pointer;
  }
  .lw-slider-hd {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 4px;
  }
  .lw-slider-lbl {
    font-size: 11px;
    color: var(--text-muted);
    transition: color 120ms;
  }
  .lw-slider-val {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-secondary);
    transition: color 120ms;
  }
  .lw-slider:hover .lw-slider-lbl,
  .lw-slider:hover .lw-slider-val { color: var(--text-primary); }
  .lw-slider-track {
    position: relative;
    height: 1px;
    background: var(--border);
  }
  .lw-slider-thumb {
    position: absolute;
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--text-muted);
    top: 50%;
    transform: translate(-50%, -50%);
    transition: background 120ms, transform 120ms;
  }
  .lw-slider:hover .lw-slider-thumb {
    background: var(--text-primary);
    transform: translate(-50%, -50%) scale(1.25);
  }

  /* Mini row */
  .lw-mini-row {
    display: flex;
    justify-content: flex-end;
    gap: 2px;
    padding: 6px 6px 0;
  }
  .lw-mini-item {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 3px 6px;
    border-radius: 3px;
    border: 1px solid transparent;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    font-size: 11px;
    transition: background 120ms, color 120ms, border-color 120ms;
  }
  .lw-mini-item:hover {
    background: color-mix(in srgb, var(--surface) 86%, var(--text-primary));
    color: var(--text-primary);
  }
  .lw-mini-item--active {
    background: color-mix(in srgb, var(--surface) 78%, var(--text-primary));
    border-color: color-mix(in srgb, var(--text-primary) 18%, transparent);
    color: var(--text-primary);
  }
  .lw-mini-val { line-height: 1; }
  .color-wheel {
    width: 160px;
    height: 160px;
    border-radius: 50%;
    background: conic-gradient(red, yellow, lime, cyan, blue, magenta, red);
    box-shadow: 0 0 0 2px var(--border);
    position: relative;
    cursor: crosshair;
    user-select: none;
    touch-action: none;
    flex-shrink: 0;
  }
  .color-wheel::after {
    content: '';
    position: absolute;
    inset: 8px;
    border-radius: 50%;
    background: radial-gradient(circle at center, rgba(10,10,12,0.94) 0%, rgba(10,10,12,0.88) 100%);
    pointer-events: none;
  }
  .cw-ind {
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 2.5px solid #fff;
    box-shadow: 0 0 0 1.5px rgba(0,0,0,0.35), 0 1px 4px rgba(0,0,0,0.3);
    transform: translate(-50%, -50%);
    pointer-events: none;
    z-index: 1;
  }
  .cw-v-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 160px;
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
    left: 0; right: 0;
    top: 50%; transform: translateY(-50%);
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
    border: 2px solid rgba(0,0,0,0.18);
    box-shadow: 0 1px 4px rgba(0,0,0,0.3);
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
  .cw-footer-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 160px;
  }
  .color-swatch {
    width: 28px;
    height: 16px;
    border-radius: 3px;
    border: 1px solid rgba(0,0,0,0.12);
    flex-shrink: 0;
  }
  .color-label {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    letter-spacing: 0.05em;
  }

  /* ── FLCK-6: Curves Editor ───────────────────────────────────────────────── */
  .curves-wrap {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .curves-svg {
    width: 100%;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 3px;
    display: block;
  }

  /* ── FLCK-7 / FLCK-14 / FLCK-15 / FLCK-16: Scopes row ────────────────────── */
  .scopes-row {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    align-items: stretch;
  }
  .scope-card {
    flex: 1 1 280px;
    min-width: 260px;
    display: flex;
    flex-direction: column;
  }
  .scope-card--wide {
    flex: 2 1 460px;
    min-width: 360px;
  }
  .scope-card :global(.card) {
    height: 100%;
  }
  .scope-card-body {
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    height: 100%;
  }
  .scope-card .scope-card-body .wf-wrap,
  .scope-card .scope-card-body .vectorscope,
  .scope-card .scope-card-body .parade-wrap,
  .scope-card .scope-card-body .hist-wrap,
  .scope-card .scope-card-body .chroma-wrap {
    flex: 1;
    min-height: 0;
  }
  /* SLD-2 pattern (mirrors FormSection) — vectorscope gain control */
  .vs-controls {
    margin-top: 6px;
    padding: 8px 10px;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: color-mix(in srgb, var(--surface) 96%, var(--text-primary));
  }
  .vs-controls .sld-wrap {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }
  .vs-controls .sld-hd {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .vs-controls .sld-lbl {
    font-size: 11px;
    color: var(--text-secondary);
  }
  .vs-controls .sld-val {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
  }
  .vs-controls .form-range {
    width: 100%;
    height: 4px;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(to right, var(--accent) var(--pct), var(--border) var(--pct));
    border-radius: 2px;
    cursor: pointer;
    outline: none;
  }
  .vs-controls .form-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 3px;
    background: var(--text-muted);
    border: 2px solid var(--surface);
    cursor: pointer;
    transition: background 80ms;
  }
  .vs-controls .form-range:hover::-webkit-slider-thumb {
    background: #fff;
  }
  .vs-extends-row {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 8px;
  }
  .vs-extends-toggle {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }
  .vs-extends-toggle input[type='checkbox'] {
    accent-color: var(--accent);
    width: 11px;
    height: 11px;
    cursor: pointer;
  }
  .wf-wrap {
    display: flex;
    flex: 1;
    gap: 4px;
    align-items: stretch;
  }
  .wf-labels {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 1px 0;
    width: 32px;
    flex-shrink: 0;
  }
  .wf-label {
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: #c8a020;
    text-align: right;
    line-height: 1;
    font-variant-numeric: tabular-nums;
  }
  .wf-scope {
    flex: 1;
    position: relative;
    min-width: 0;
    border-radius: 2px;
    border: 1px solid rgba(200, 160, 0, 0.15);
    overflow: hidden;
  }
  .wf-canvas {
    display: block;
    width: 100%;
    height: 100%;
    background: #080808;
  }
  .wf-grid {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
  .wf-debug-img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: fill;
    opacity: 0.5;
    pointer-events: none;
    border-radius: 2px;
  }
  .wf-controls {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-top: 6px;
  }
  .wf-toggle {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 11px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }
  .wf-toggle input[type='checkbox'] {
    accent-color: var(--accent);
    width: 11px;
    height: 11px;
    cursor: pointer;
  }
  .vectorscope {
    width: 100%;
    aspect-ratio: 1 / 1;
    max-height: 100%;
    border-radius: 50%;
    border: 1px solid var(--border);
    background: #000;
    position: relative;
    overflow: hidden;
    align-self: center;
  }
  .vectorscope-canvas {
    width: 100%;
    height: 100%;
    display: block;
  }
  .vs-graticule {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
    color: #c9a86e;
    opacity: 0.85;
  }

  /* ── FLCK-15: Parade ─────────────────────────────────────────────────────── */
  .parade-wrap {
    display: flex;
    gap: 4px;
    align-items: stretch;
  }
  .parade-label {
    color: var(--text-muted);
    opacity: 0.7;
  }
  .parade-scope {
    flex: 1;
    position: relative;
    min-width: 0;
    border-radius: 2px;
    border: 1px solid var(--border);
    background: #060606;
    overflow: hidden;
  }
  .parade-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .parade-channel-labels {
    position: absolute;
    inset: 0;
    display: flex;
    pointer-events: none;
  }
  .parade-channel-labels span {
    flex: 1;
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    font-weight: 700;
    letter-spacing: 0.1em;
    padding: 4px 0 0 6px;
    opacity: 0.85;
  }

  /* ── FLCK-16: Histogram ──────────────────────────────────────────────────── */
  .hist-wrap {
    position: relative;
    border-radius: 2px;
    border: 1px solid var(--border);
    background: #060606;
    overflow: hidden;
  }
  .hist-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .hist-grid {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
  .hist-legend {
    display: flex;
    gap: 8px;
    margin-top: 6px;
  }
  .hist-chip {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 3px;
    border: 1px solid currentColor;
    line-height: 1;
  }
  .hist-chip--r { color: #ff6e6e; }
  .hist-chip--g { color: #6efc8c; }
  .hist-chip--b { color: #92aaff; }
  .hist-chip--y { color: #e8e8e8; }

  /* ── FLCK-17: Chromaticity ───────────────────────────────────────────────── */
  .chroma-wrap {
    display: flex;
    align-items: stretch;
  }
  .chroma-scope {
    flex: 1;
    position: relative;
    min-width: 0;
    border-radius: 2px;
    border: 1px solid var(--border);
    background: #050505;
    overflow: hidden;
    aspect-ratio: 1 / 1;
    align-self: center;
    max-height: 100%;
  }
  .chroma-canvas {
    display: block;
    width: 100%;
    height: 100%;
  }
  .chroma-grid {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }
  .chroma-tick {
    position: absolute;
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    opacity: 0.7;
    pointer-events: none;
    font-variant-numeric: tabular-nums;
    line-height: 1;
  }
  .chroma-tick-x {
    bottom: 3px;
    transform: translateX(-50%);
  }
  .chroma-tick-y {
    left: 3px;
    transform: translateY(50%);
  }
  .chroma-axis-name {
    position: absolute;
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-weight: 600;
    color: var(--text-secondary);
    opacity: 0.85;
    pointer-events: none;
    letter-spacing: 0.05em;
  }
  .chroma-axis-x {
    right: 6px;
    bottom: 16px;
  }
  .chroma-axis-y {
    top: 6px;
    left: 14px;
  }
  .chroma-rec709-label {
    position: absolute;
    left: 20%;
    bottom: 52%;
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: #c9a86e;
    opacity: 0.9;
    pointer-events: none;
    letter-spacing: 0.04em;
  }

  /* ── FLCK-6 + FLCK-8 row ────────────────────────────────────────────────── */
  .curves-grade-row {
    display: flex;
    flex-direction: row;
    gap: 12px;
    align-items: stretch;
  }
  .curves-grade-half {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }
  .curves-grade-half :global(.card) {
    flex: 1;
  }
  .curves-editor-card {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .curves-editor-card :global(.card) {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
  .curves-editor-card :global(.card-body) {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }
  .curves-editor-card :global(.card-content) {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 !important;
    align-items: stretch;
  }
  .curves-editor-card :global(.card-frame) {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    justify-content: stretch;
  }
  .curves-mode-bar {
    display: flex;
    justify-content: flex-end;
    margin-bottom: 6px;
  }
  /* shared SVG element classes — used in all curve SVGs */
  .curves-grid-line {
    stroke: var(--border-subtle);
    stroke-width: 0.5;
    opacity: 0.7;
  }
  .curves-border {
    stroke: var(--border);
    stroke-width: 1;
  }
  .hist-luma {
    fill: var(--text-primary);
    fill-opacity: 0.12;
    stroke: var(--text-primary);
    stroke-width: 0.5;
    stroke-opacity: 0.18;
  }
  .curves-diagonal {
    stroke: var(--text-primary);
    stroke-width: 1;
    opacity: 0.55;
  }
  .curves-line {
    stroke: var(--text-primary);
    stroke-width: 1.5;
    opacity: 0.9;
  }
  .curves-line-r { stroke: color-mix(in srgb, red 65%, var(--text-primary)); }
  .curves-line-g { stroke: color-mix(in srgb, #00e060 55%, var(--text-primary)); }
  .curves-line-b { stroke: color-mix(in srgb, dodgerblue 65%, var(--text-primary)); }
  .curves-anchor {
    fill: var(--text-primary);
    stroke: var(--surface);
    stroke-width: 1.5;
    opacity: 0.9;
  }
  .curves-svg-wide {
    flex: 1;
    width: 100%;
    height: 100%;
    min-height: 0;
    border-radius: 0 0 10px 10px;
    border: none;
    border-top: 1px solid var(--border);
  }
  .curves-interactive {
    cursor: crosshair;
    user-select: none;
    touch-action: none;
  }

  /* ── FLCK-8: Color Grading ───────────────────────────────────────────────── */
  .color-grade-wrap {
    display: flex;
    flex-direction: column;
    gap: 16px;
    width: 100%;
    padding: 4px 0;
  }
  .cg-tabs-row {
    padding: 0 8px;
    display: flex;
    align-items: center;
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }
  .cg-wheels {
    display: flex;
    gap: 24px;
    padding: 8px 16px 16px;
  }
  .cg-wheel-group {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
  }
  .cg-wheel {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: conic-gradient(red, yellow, lime, cyan, blue, magenta, red);
    box-shadow: 0 0 0 1px var(--border);
    position: relative;
  }
  .cg-wheel::after {
    content: '';
    position: absolute;
    inset: 10px;
    border-radius: 50%;
    background: radial-gradient(circle at 65% 35%, rgba(255,255,255,0.5) 0%, transparent 60%),
                radial-gradient(circle at center, transparent 25%, rgba(0,0,0,0.6) 100%);
  }
  .cg-wheel-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }
  .cg-placeholder {
    padding: 40px 16px;
    text-align: center;
    font-size: 11px;
    color: var(--text-muted);
    font-style: italic;
  }

  /* ── FLCK-9: Audio Mixer ─────────────────────────────────────────────────── */
  .mixer {
    display: flex;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--surface-raised);
  }
  .mixer-strip {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 8px 4px 6px;
    gap: 4px;
    border-right: 1px solid var(--border-subtle);
  }
  .mixer-strip:last-child { border-right: none; }
  .mixer-strip-master {
    background: color-mix(in srgb, var(--surface-raised) 60%, var(--surface));
  }
  .mixer-vu {
    display: flex;
    flex-direction: column-reverse;
    gap: 1px;
    height: 60px;
    width: 14px;
  }
  .vu-seg {
    flex: 1;
    border-radius: 1px;
    opacity: 0.25;
  }
  .vu-seg.vu-active { opacity: 1; }
  .vu-green  { background: color-mix(in srgb, lime 60%, var(--accent)); }
  .vu-yellow { background: color-mix(in srgb, yellow 80%, var(--accent)); }
  .vu-red    { background: color-mix(in srgb, red 80%, var(--accent)); }
  .mixer-fader {
    height: 60px;
    width: 10px;
    accent-color: var(--accent);
    cursor: pointer;
    appearance: slider-vertical;
  }
  .mixer-controls {
    display: flex;
    gap: 2px;
  }
  .mixer-btn {
    width: 18px;
    height: 14px;
    font-size: 8px;
    font-weight: 700;
    font-family: inherit;
    border-radius: 2px;
    border: 1px solid var(--border);
    background: var(--surface);
    color: var(--text-muted);
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.1s, color 0.1s;
    padding: 0;
  }
  .mixer-btn:hover { color: var(--text-primary); }
  .mixer-btn-muted { background: color-mix(in srgb, var(--accent) 15%, var(--surface)); color: var(--accent); border-color: color-mix(in srgb, var(--accent) 40%, var(--border)); }
  .mixer-label {
    font-size: 8px;
    font-weight: 700;
    letter-spacing: 0.07em;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-top: 2px;
  }

  /* ── FLCK-10: Media Pool ─────────────────────────────────────────────────── */
  .media-pool {
    display: flex;
    flex-direction: column;
    gap: 8px;
    padding: 6px;
    width: 340px;
  }
  .search-compact {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    padding: 4px 8px;
    height: 28px;
    box-sizing: border-box;
    transition: box-shadow 0.1s;
  }
  .search-compact:focus-within { box-shadow: inset 0 -2px 0 0 var(--accent); }
  .search-icon { color: var(--text-muted); flex-shrink: 0; }
  .search-input {
    flex: 1;
    min-width: 0;
    background: transparent;
    border: none;
    outline: none;
    color: var(--text-primary);
    font-family: inherit;
    font-size: 12px;
  }
  .search-input::placeholder { color: var(--text-muted); }
  .search-input::-webkit-search-cancel-button { display: none; }
  .media-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 6px;
  }
  .media-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 2px;
  }
  .media-thumb {
    width: 90px;
    height: 60px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--surface-raised) 60%, #000);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.1s;
  }
  .media-thumb-audio {
    background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-muted));
  }
  .media-item-active .media-thumb {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px color-mix(in srgb, var(--accent) 30%, transparent);
  }
  .media-name {
    font-size: 8px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
    text-align: center;
    width: 90px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* ── FLCK-11: Folder Tree ────────────────────────────────────────────────── */
  .folder-tree {
    display: flex;
    flex-direction: column;
    width: 180px;
    font-size: 11px;
  }
  .ft-row {
    display: flex;
    align-items: center;
    gap: 5px;
    height: 28px;
    padding: 0 8px;
    width: 100%;
    background: none;
    border: none;
    cursor: default;
    font-family: inherit;
    font-size: 11px;
    color: var(--text-secondary);
    transition: background 0.1s;
    text-align: left;
  }
  button.ft-row { cursor: pointer; }
  button.ft-row:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }
  .ft-root { padding-left: 8px; }
  .ft-row-child { padding-left: 28px; cursor: default; }
  .ft-chev {
    color: var(--text-muted);
    flex-shrink: 0;
    transition: transform 0.12s;
  }
  .ft-icon-folder { color: color-mix(in srgb, var(--accent) 60%, var(--text-muted)); flex-shrink: 0; }
  .ft-icon-file   { color: var(--text-muted); flex-shrink: 0; }
  .ft-name { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .ft-name-root { font-weight: 600; color: var(--text-primary); }
  .ft-name-file { color: var(--text-muted); font-size: 10px; }
  .ft-badge {
    font-size: 8px;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0 5px;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  /* ── FLCK-12: Clip Inspector (ir-* classes) ──────────────────────────────── */
  .ir {
    background: var(--surface);
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    width: 200px;
  }
  .ir-clip-name {
    padding: 6px 12px;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-secondary);
    border-bottom: 1px solid var(--border);
    font-family: 'Geist Mono', monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .ir-section { border-bottom: 1px solid var(--border); }
  .ir-sec-hd {
    width: 100%;
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 6px 10px;
    background: var(--surface-raised);
    border: none;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.1s;
  }
  .ir-sec-hd:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }
  .ir-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }
  .ir-sec-title {
    flex: 1;
    text-align: left;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-secondary);
  }
  .ir-chev {
    color: var(--text-muted);
    transition: transform 0.15s;
    flex-shrink: 0;
  }
  .ir-row {
    display: flex;
    align-items: center;
    padding: 3px 12px;
    gap: 4px;
    transition: background 0.1s;
  }
  .ir-row:hover { background: color-mix(in srgb, var(--surface) 94%, var(--text-primary)); }
  .ir-lbl {
    flex: 1;
    font-size: 10px;
    color: var(--text-muted);
  }
  .ir-num {
    width: 56px;
    text-align: right;
    background: transparent;
    border: none;
    border-bottom: 1px solid transparent;
    color: var(--accent);
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    font-variant-numeric: tabular-nums;
    outline: none;
    padding: 0 2px;
    -moz-appearance: textfield;
  }
  .ir-num::-webkit-inner-spin-button,
  .ir-num::-webkit-outer-spin-button { -webkit-appearance: none; }
  .ir-num:focus { border-bottom-color: var(--accent); }
  .ir-sel {
    font-size: 10px;
    font-family: inherit;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    padding: 2px 4px;
    outline: none;
    cursor: pointer;
  }
  .ir-check {
    accent-color: var(--accent);
    width: 13px;
    height: 13px;
    cursor: pointer;
  }

  /* ── FLCK-13: Export / Delivery ──────────────────────────────────────────── */
  .export-wrap {
    display: flex;
    gap: 0;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    min-height: 320px;
  }
  .export-formats {
    width: 200px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface-raised);
    display: flex;
    flex-direction: column;
  }
  .export-formats-hd {
    padding: 8px 12px;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border);
  }
  .export-fmt-row {
    display: flex;
    align-items: center;
    height: 32px;
    padding: 0 12px;
    font-size: 11px;
    font-family: inherit;
    background: none;
    border: none;
    border-bottom: 1px solid var(--border-subtle);
    color: var(--text-secondary);
    cursor: pointer;
    text-align: left;
    transition: background 0.1s, color 0.1s;
  }
  .export-fmt-row:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); color: var(--text-primary); }
  .export-fmt-active {
    color: var(--accent) !important;
    border-left: 2px solid var(--accent);
    padding-left: 10px;
    background: color-mix(in srgb, var(--accent) 6%, var(--surface-raised)) !important;
  }
  .export-settings {
    flex: 1;
    padding: 12px 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
  }
  .export-settings-hd {
    font-size: 12px;
    font-weight: 600;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
  }
  .export-field {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .export-label {
    font-size: 10px;
    color: var(--text-muted);
    width: 80px;
    flex-shrink: 0;
  }
  .export-select {
    font-size: 11px;
    font-family: inherit;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-secondary);
    padding: 3px 6px;
    outline: none;
    cursor: pointer;
  }
  .export-slider {
    flex: 1;
    accent-color: var(--accent);
    cursor: pointer;
  }
  .export-path-row {
    flex-wrap: wrap;
    gap: 4px;
  }
  .export-path {
    flex: 1;
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    background: var(--surface-raised);
    border: 1px solid var(--border-subtle);
    border-radius: 3px;
    padding: 3px 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .export-browse-btn {
    font-size: 10px;
    font-family: inherit;
    padding: 3px 8px;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    color: var(--text-secondary);
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.1s;
    flex-shrink: 0;
  }
  .export-browse-btn:hover { background: color-mix(in srgb, var(--surface-raised) 85%, var(--text-primary)); }
  .export-action {
    margin-top: auto;
    padding-top: 8px;
  }
  .export-action :global(button) { width: 100%; }
</style>
