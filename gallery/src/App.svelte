<script>
  import { onMount, onDestroy } from 'svelte';
  import { Toaster } from '@libre/ui';
  import ThemeLab from './lib/ThemeLab.svelte';
  import AppPanels from './lib/AppPanels.svelte';
  import TokensSection from './sections/TokensSection.svelte';
  import ButtonsSection from './sections/ButtonsSection.svelte';
  import FormSection from './sections/FormSection.svelte';
  import FeedbackSection from './sections/FeedbackSection.svelte';
  import NavigationSection from './sections/NavigationSection.svelte';
  import LayoutSection from './sections/LayoutSection.svelte';
  import TypographySection from './sections/TypographySection.svelte';
  import DemoTilesSection from './sections/DemoTilesSection.svelte';
  import { createZoom, ZOOM_STEPS } from './lib/stores/zoom.svelte.js';
  import { tooltip, setHint } from './lib/stores/tooltip.svelte.js';

  const sections = [
    { id: 'demo',       label: 'Demo Tiles',        component: DemoTilesSection },
    { id: 'tokens',     label: 'Tokens',           component: TokensSection },
    { id: 'typography', label: 'Typography',        component: TypographySection },
    { id: 'buttons',    label: 'Buttons & Actions', component: ButtonsSection },
    { id: 'form',       label: 'Form Controls',     component: FormSection },
    { id: 'feedback',   label: 'Feedback',          component: FeedbackSection },
    { id: 'navigation', label: 'Navigation',        component: NavigationSection },
    { id: 'layout',     label: 'Layout',            component: LayoutSection },
  ];

  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

  let dark      = $state(localStorage.getItem('libre-gallery-dark') === 'true');
  let toaster   = $state(null);
  let active    = $state('tokens');
  let contentEl = $state(null);

  const zoom = createZoom();

  // ── Footer chrome ──────────────────────────────────────────────────────
  let appVersion = $state('');
  let aboutOpen = $state(false);
  let aboutClosing = $state(false);

  // ── Updater state machine ──────────────────────────────────────────────
  // The shape is the canonical contract every Libre app implements. When the
  // real `tauri-plugin-updater` wiring lands, replace `runUpdateCheck` with
  // the plugin calls — the footer UI consumes the same shape verbatim.
  /** @type {{status:'idle'}|{status:'checking'}|{status:'up-to-date'}|{status:'available',version:string}|{status:'downloading',progress:number}|{status:'ready'}|{status:'error',message:string}} */
  let updater = $state({ status: 'idle' });

  function closeAbout() {
    if (aboutClosing) return;
    aboutClosing = true;
    setTimeout(() => { aboutOpen = false; aboutClosing = false; }, 500);
  }

  async function fetchVersion() {
    if (!isTauri) {
      appVersion = '0.1.0-dev';
      return;
    }
    try {
      const { getVersion } = await import('@tauri-apps/api/app');
      appVersion = await getVersion();
    } catch {
      appVersion = '0.1.0';
    }
  }

  async function runUpdateCheck() {
    // Mocked for the gallery (bundle.active=false → not distributed). Real
    // apps replace this body with `await check()` from
    // `@tauri-apps/plugin-updater` and dispatch the resulting state.
    updater = { status: 'checking' };
    await new Promise(r => setTimeout(r, 1200));
    updater = { status: 'up-to-date' };
    setTimeout(() => { if (updater.status === 'up-to-date') updater = { status: 'idle' }; }, 4000);
  }

  // Tooltip delegation — single root-level mouseover resolves data-tooltip.
  function onPointerOver(e) {
    const el = /** @type {HTMLElement|null} */ (e.target)?.closest?.('[data-tooltip]');
    if (el) setHint(el.getAttribute('data-tooltip') || '');
  }
  function onPointerOut(e) {
    const from = /** @type {HTMLElement|null} */ (e.target)?.closest?.('[data-tooltip]');
    if (!from) return;
    const to = /** @type {HTMLElement|null} */ (e.relatedTarget)?.closest?.('[data-tooltip]');
    if (to !== from) setHint('');
  }

  $effect(() => {
    document.documentElement.classList.toggle('dark', dark);
    localStorage.setItem('libre-gallery-dark', String(dark));
  });

  $effect(() => {
    if (!contentEl) return;
    const els = contentEl.querySelectorAll('.gallery-section');
    const observer = new IntersectionObserver(
      (entries) => {
        const visible = entries
          .filter(e => e.isIntersecting)
          .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);
        if (visible.length > 0) active = visible[0].target.id;
      },
      { root: contentEl, threshold: 0, rootMargin: '0px 0px -72% 0px' }
    );
    els.forEach(el => observer.observe(el));
    return () => observer.disconnect();
  });

  function scrollTo(id) {
    contentEl?.querySelector(`#${id}`)?.scrollIntoView({ behavior: 'smooth', block: 'start' });
  }

  function onKeydown(e) {
    if (e.key === 'l' || e.key === 'L') {
      const tag = document.activeElement?.tagName;
      if (tag === 'INPUT' || tag === 'TEXTAREA') return;
      e.preventDefault();
      dark = !dark;
      return;
    }
    zoom.handleKey(e);
  }

  onMount(() => {
    window.addEventListener('keydown', onKeydown);
    fetchVersion();
    runUpdateCheck();
  });

  onDestroy(() => {
    window.removeEventListener('keydown', onKeydown);
  });
</script>

<svelte:window onpointerover={onPointerOver} onpointerout={onPointerOut} />

<div class="shell">
  <div class="main-row">
    <aside class="sidebar">
      <div class="sidebar-header">
        <div class="logo">
          <span class="logo-mark">◈</span>
          Libre UI
        </div>
        <button
          class="theme-btn"
          onclick={() => (dark = !dark)}
          data-tooltip="Toggle theme — L"
          aria-label="Toggle theme"
        >
          {dark ? '☀' : '☾'}
        </button>
      </div>
      <nav>
        {#each sections as s}
          <button
            class="nav-item"
            class:nav-active={active === s.id}
            onclick={() => scrollTo(s.id)}
          >
            {s.label}
          </button>
        {/each}
      </nav>
      <div class="sidebar-bottom">
        <ThemeLab />
      </div>
    </aside>

    <main class="content" bind:this={contentEl}>
      {#each sections as s}
        <section class="gallery-section" id={s.id}>
          <h1 class="section-h1">{s.label}</h1>
          {#if s.id === 'feedback'}
            <FeedbackSection {toaster} />
          {:else}
            <svelte:component this={s.component} />
          {/if}
        </section>
      {/each}
    </main>

    <AppPanels />
  </div>

  <!-- Footer bar — canonical Libre footer pattern (zoom · hint · version+about). -->
  <footer class="libre-footer">
    <!-- LEFT: zoom controls -->
    <div class="footer-zoom">
      <button
        class="zoom-btn"
        onclick={zoom.stepOut}
        disabled={zoom.level === ZOOM_STEPS[0]}
        data-tooltip="Zoom out the UI — ⌘−"
        aria-label="Zoom out"
      >−</button>
      <button
        class="zoom-pct"
        class:zoom-pct-active={zoom.level !== 1.0}
        onclick={zoom.reset}
        data-tooltip="Reset zoom to 100% — ⌘0"
        aria-label="Reset zoom"
      >{Math.round(zoom.level * 100)}%</button>
      <button
        class="zoom-btn"
        onclick={zoom.stepIn}
        disabled={zoom.level === ZOOM_STEPS[ZOOM_STEPS.length - 1]}
        data-tooltip="Zoom in the UI — ⌘+"
        aria-label="Zoom in"
      >+</button>
    </div>

    <!-- CENTER: rolling hint text, with optional updater override -->
    <span
      class="footer-hint"
      style="opacity:{tooltip.opacity}; transition:opacity {tooltip.duration}ms linear {tooltip.delay}ms"
    >
      {tooltip.text}
    </span>

    <!-- RIGHT: updater chip + version/about -->
    <div class="footer-right">
      {#if updater.status === 'checking'}
        <span class="upd-chip upd-checking" data-tooltip="Checking for updates…">
          <span class="upd-dot"></span> Checking
        </span>
      {:else if updater.status === 'up-to-date'}
        <span class="upd-chip upd-ok" data-tooltip="You're on the latest version">
          <span class="upd-dot"></span> Up to date
        </span>
      {:else if updater.status === 'available'}
        <button class="upd-chip upd-available" data-tooltip="Click to install update">
          <span class="upd-dot"></span> Update {updater.version}
        </button>
      {:else if updater.status === 'downloading'}
        <span class="upd-chip upd-progress" data-tooltip="Downloading update…">
          <span class="upd-dot"></span> {Math.round(updater.progress * 100)}%
        </span>
      {:else if updater.status === 'ready'}
        <button class="upd-chip upd-ready" data-tooltip="Restart to apply update">
          <span class="upd-dot"></span> Restart to update
        </button>
      {:else if updater.status === 'error'}
        <span class="upd-chip upd-error" data-tooltip={updater.message}>
          <span class="upd-dot"></span> Update failed
        </span>
      {/if}

      <button
        class="version-btn libre-pulse"
        onclick={() => (aboutOpen = true)}
        data-tooltip="About Libre UI"
      >Libre UI{appVersion ? ` v${appVersion}` : ''}</button>
    </div>
  </footer>

  <Toaster bind:this={toaster} />
</div>

<!-- About modal — backdrop blurs in over 2s, card crossfades + blur-lifts over 1s. -->
{#if aboutOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div
    class="{aboutClosing ? 'about-backdrop-out' : 'about-backdrop'} about-shell"
    onpointerdown={closeAbout}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
    <div
      class="{aboutClosing ? 'about-card-out' : 'about-card'} about-frame"
      onpointerdown={(e) => e.stopPropagation()}
    >
      <div class="about-header">
        <div class="about-glyph">
          <span>◈</span>
        </div>
        <p class="about-name">Libre UI</p>
        {#if appVersion}<p class="about-version">v{appVersion}</p>{/if}
      </div>

      <div class="about-body">
        <p>The shared component foundation for the <strong>Libre</strong> family of desktop apps.</p>
        <p class="about-by">By <strong>Iron Tree Software</strong></p>
        <div class="about-meta">
          <div><span>Runtime</span><span>Tauri + Svelte 5 + Rust</span></div>
          <div><span>Tokens</span><span>common-js/src/tokens.css</span></div>
          <div><span>License</span><span>Proprietary</span></div>
        </div>
      </div>
    </div>
  </div>
{/if}


<style>
  /* ── Shell ─────────────────────────────────────────────────────────── */
  .shell {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    background: var(--surface);
    color: var(--text-primary);
  }

  .main-row {
    flex: 1;
    min-height: 0;
    display: flex;
    flex-direction: row;
  }

  .sidebar {
    width: 200px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface-panel);
    display: flex;
    flex-direction: column;
  }

  .sidebar-bottom { flex-shrink: 0; }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px 12px;
    border-bottom: 1px solid var(--border);
  }

  .logo {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 13px;
    font-weight: 600;
    letter-spacing: -0.01em;
    color: var(--text-primary);
  }
  .logo-mark { color: var(--accent); font-size: 15px; }

  .theme-btn {
    background: none;
    border: 1px solid var(--border);
    border-radius: 6px;
    width: 28px;
    height: 28px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.1s;
  }
  .theme-btn:hover { border-color: var(--accent); color: var(--text-primary); }

  nav { padding: 8px; flex: 1; min-height: 0; overflow-y: auto; }

  .nav-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 7px 10px;
    background: none;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
    font-family: inherit;
    color: var(--text-secondary);
    transition: color 0.1s, background 0.1s;
  }
  .nav-item:hover { color: var(--text-primary); background: var(--surface-raised); }
  .nav-active { color: var(--text-primary) !important; background: var(--surface-raised) !important; font-weight: 500; }

  .content {
    flex: 1;
    overflow-y: auto;
    background: var(--surface);
    padding: 48px 56px 72px;
  }

  .gallery-section {
    padding-bottom: 72px;
    margin-bottom: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .gallery-section:last-child { border-bottom: none; padding-bottom: 0; }

  .section-h1 {
    font-size: 30px;
    font-weight: 700;
    letter-spacing: -0.03em;
    color: var(--text-primary);
    margin: 0 0 32px 0;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
    line-height: 1.1;
  }

  /* ── Footer bar ────────────────────────────────────────────────────── */
  .libre-footer {
    height: 24px;
    flex-shrink: 0;
    border-top: 1px solid var(--border);
    background: var(--surface-panel);
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 0 8px;
    user-select: none;
    -webkit-user-select: none;
    font-size: 11px;
  }

  .footer-zoom {
    display: flex;
    align-items: center;
    gap: 2px;
    flex-shrink: 0;
  }

  .zoom-btn {
    width: 17px;
    height: 17px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    border: none;
    border-radius: 3px;
    color: color-mix(in srgb, var(--text-primary) 45%, transparent);
    font-size: 11px;
    line-height: 1;
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    padding: 0;
  }
  .zoom-btn:hover:not(:disabled) {
    background: color-mix(in srgb, var(--text-primary) 10%, transparent);
    color: var(--text-primary);
  }
  .zoom-btn:disabled { opacity: 0.2; cursor: default; }

  .zoom-pct {
    height: 17px;
    padding: 0 5px;
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    border: none;
    border-radius: 3px;
    color: color-mix(in srgb, var(--text-primary) 35%, transparent);
    font-size: 9px;
    font-family: 'Geist Mono', monospace;
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .zoom-pct:hover { background: color-mix(in srgb, var(--text-primary) 10%, transparent); }
  .zoom-pct-active { color: var(--text-primary); }

  .footer-hint {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    color: color-mix(in srgb, var(--text-primary) 50%, transparent);
    font-size: 11px;
    line-height: 1;
  }

  .footer-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  /* ── Updater chip ──────────────────────────────────────────────────── */
  .upd-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 3px 7px;
    border: none;
    border-radius: 999px;
    background: transparent;
    color: var(--text-secondary);
    font-size: 10px;
    font-family: inherit;
    line-height: 1;
    cursor: default;
  }
  button.upd-chip { cursor: pointer; }
  button.upd-chip:hover { color: var(--text-primary); }

  .upd-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: currentColor;
    flex-shrink: 0;
  }

  .upd-checking  { color: color-mix(in srgb, var(--text-primary) 50%, transparent); }
  .upd-checking .upd-dot { animation: upd-pulse 1.2s ease-in-out infinite; }
  .upd-ok        { color: #4ade80; }
  .upd-available { color: var(--accent); }
  .upd-progress  { color: var(--accent); }
  .upd-ready     { color: var(--accent); }
  .upd-error     { color: #ef4444; }

  @keyframes upd-pulse {
    0%, 100% { opacity: 0.3; }
    50%      { opacity: 1; }
  }

  /* ── Version button (pulses subtly) ───────────────────────────────── */
  .version-btn {
    background: transparent;
    border: none;
    padding: 0;
    margin: 0;
    font-family: inherit;
    font-size: 10px;
    font-weight: 500;
    cursor: pointer;
    color: color-mix(in srgb, var(--text-primary) 20%, transparent);
    transition: opacity 0.1s;
  }
  .version-btn:hover { opacity: 0.8; color: var(--text-primary); }

  .libre-pulse { animation: libre-pulse 20s ease-in-out infinite; }
  @keyframes libre-pulse {
    0%, 50%   { color: color-mix(in srgb, var(--text-primary) 20%, transparent); }
    75%       { color: var(--accent); }
    100%      { color: color-mix(in srgb, var(--text-primary) 20%, transparent); }
  }

  /* ── About modal ──────────────────────────────────────────────────── */
  .about-shell {
    position: fixed;
    inset: 0;
    z-index: 600;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .about-frame {
    position: relative;
    width: 420px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    overflow: hidden;
    box-shadow: 0 24px 64px rgb(0 0 0 / 0.45);
  }

  .about-header {
    padding: 32px 32px 24px;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, var(--accent) 6%, var(--surface-raised));
  }

  .about-glyph {
    width: 64px;
    height: 64px;
    border-radius: 16px;
    background: color-mix(in srgb, var(--accent) 14%, var(--surface));
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 28px;
  }

  .about-name {
    font-size: 22px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  .about-version {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 2px 0 0;
  }

  .about-body {
    padding: 24px 32px;
    display: flex;
    flex-direction: column;
    gap: 16px;
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.55;
  }
  .about-body strong { color: var(--text-primary); }
  .about-by { margin: 0; }

  .about-meta {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding-top: 12px;
    border-top: 1px solid var(--border);
    font-size: 12px;
  }
  .about-meta > div {
    display: flex;
    justify-content: space-between;
  }
  .about-meta > div > span:first-child { color: var(--text-secondary); }
  .about-meta > div > span:last-child  { color: var(--text-primary); font-family: 'Geist Mono', monospace; }

  @keyframes about-backdrop-in {
    0%   { background: rgb(0 0 0 / 0);   backdrop-filter: blur(0px); }
    100% { background: rgb(0 0 0 / 0.5); backdrop-filter: blur(7px); }
  }
  .about-backdrop {
    animation: about-backdrop-in 2s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }
  @keyframes about-card-in {
    0%   { opacity: 0; filter: blur(8px); transform: translateY(12px); }
    100% { opacity: 1; filter: blur(0px); transform: translateY(0); }
  }
  .about-card {
    opacity: 0;
    animation: about-card-in 1s cubic-bezier(0.22, 1, 0.36, 1) forwards;
  }
  @keyframes about-backdrop-out {
    0%   { background: rgb(0 0 0 / 0.5); backdrop-filter: blur(7px); }
    100% { background: rgb(0 0 0 / 0);   backdrop-filter: blur(0px); }
  }
  .about-backdrop-out {
    animation: about-backdrop-out 0.5s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }
  @keyframes about-card-out {
    0%   { opacity: 1; filter: blur(0px); transform: translateY(0); }
    100% { opacity: 0; filter: blur(8px); transform: translateY(12px); }
  }
  .about-card-out {
    animation: about-card-out 0.5s cubic-bezier(0.4, 0, 1, 1) forwards;
  }
</style>
