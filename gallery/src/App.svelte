<script>
  import { onMount, onDestroy } from 'svelte';
  import { Toaster, GlobalTabs } from '@libre/ui';
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
  import MediaSection from './sections/MediaSection.svelte';
  import LoadingSection from './sections/LoadingSection.svelte';
  import PatternsSection from './sections/PatternsSection.svelte';
  import MotionSection from './sections/MotionSection.svelte';
  import UIStandardsSection from './sections/UIStandardsSection.svelte';
  import AudioSection from './sections/AudioSection.svelte';
  import FlickerSection from './sections/FlickerSection.svelte';
  import ShelfSection from './sections/ShelfSection.svelte';
  import StackSection from './sections/StackSection.svelte';
  import PrismSection from './sections/PrismSection.svelte';
  import FadeSection from './sections/FadeSection.svelte';
  import LibreWinSection from './sections/LibreWinSection.svelte';
  import MiniMap from './lib/MiniMap.svelte';
  import { createZoom, ZOOM_STEPS } from './lib/stores/zoom.svelte.js';
  import { tooltip, setHint } from './lib/stores/tooltip.svelte.js';
  import { canvas } from './lib/stores/canvas.svelte.js';

  const sections = [
    { id: 'demo',       label: 'Demo Layouts',      component: DemoTilesSection,  tab: 'overview'    },
    { id: 'buttons',    label: 'Buttons & Actions', component: ButtonsSection,    tab: 'components'  },
    { id: 'form',       label: 'Form Controls',     component: FormSection,       tab: 'components'  },
    { id: 'feedback',   label: 'Feedback',          component: FeedbackSection,   tab: 'components'  },
    { id: 'navigation', label: 'Navigation',        component: NavigationSection, tab: 'components'  },
    { id: 'layout',     label: 'Layout',            component: LayoutSection,     tab: 'components'  },
    { id: 'media',      label: 'Media',             component: MediaSection,      tab: 'components'  },
    { id: 'loading',    label: 'Loading',           component: LoadingSection,    tab: 'components'  },
    { id: 'typography', label: 'Typography',        component: TypographySection, tab: 'surface'     },
    { id: 'tokens',     label: 'Tokens',            component: TokensSection,     tab: 'surface'     },
    { id: 'patterns',   label: 'Patterns',          component: PatternsSection,   tab: 'foundation'  },
    { id: 'motion',     label: 'Motion',            component: MotionSection,     tab: 'surface'     },
    { id: 'ui-standards', label: 'UI Standards',   component: UIStandardsSection, tab: 'foundation' },
    { id: 'audio',        label: 'Audio',          component: AudioSection,        tab: 'surface'     },
    { id: 'app-flicker', label: 'Flicker',   component: FlickerSection,  tab: 'applications'  },
    { id: 'app-shelf',   label: 'Shelf',     component: ShelfSection,    tab: 'applications'  },
    { id: 'app-stack',   label: 'Stack',     component: StackSection,    tab: 'applications'  },
    { id: 'app-prism',   label: 'Prism',     component: PrismSection,    tab: 'applications'  },
    { id: 'app-fade',      label: 'Fade',      component: FadeSection,      tab: 'applications'  },
    { id: 'app-librewin', label: 'LibreWin',  component: LibreWinSection,  tab: 'applications'  },
  ];

  const TABS_FOUNDATION = [
    { id: 'foundation',   label: 'Foundation' },
    { id: 'surface',      label: 'Surface'     },
    { id: 'components',   label: 'Components'  },
  ];

  const TABS_APPS = [
    { id: 'applications', label: 'Applications' },
    { id: 'overview',     label: 'Demo Layouts' },
  ];

  const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

  let themeMode = $state(/** @type {'auto'|'light'|'dark'} */ (localStorage.getItem('libre-gallery-theme') || 'auto'));
  let dark      = $state(
    themeMode === 'auto'
      ? window.matchMedia('(prefers-color-scheme: dark)').matches
      : themeMode === 'dark'
  );

  function cycleTheme() {
    const order = /** @type {const} */ (['auto', 'light', 'dark']);
    themeMode = order[(order.indexOf(themeMode) + 1) % 3];
    localStorage.setItem('libre-gallery-theme', themeMode);
    dark = themeMode === 'auto'
      ? window.matchMedia('(prefers-color-scheme: dark)').matches
      : themeMode === 'dark';
  }
  let toaster   = $state(null);
  let active    = $state('demo');
  let contentEl = $state(null);

  const visibleSections = $derived(sections.filter(s => s.tab === activeTab));

  const zoom = createZoom();

  // ── Footer chrome ──────────────────────────────────────────────────────
  let appVersion = $state('');
  let aboutOpen = $state(false);
  let aboutClosing = $state(false);

  // ── Top bar state ──────────────────────────────────────────────────────
  let activeTab = $state('overview');
  $effect(() => { canvas.activeTab = activeTab; });
  let projectName = $state('Libre UI');
  let panelsCollapsed = $state(localStorage.getItem('libre-panels-collapsed') === 'true');

  function togglePanels() {
    panelsCollapsed = !panelsCollapsed;
    localStorage.setItem('libre-panels-collapsed', String(panelsCollapsed));
  }
  let docPickerOpen = $state(false);
  let editingName = $state(false);
  let nameInput = $state('');
  let recentFiles = $state([
    { name: 'Libre UI',      path: '/showcase/libre-ui' },
    { name: 'Component Lab', path: '/showcase/component-lab' },
    { name: 'Token Sandbox', path: '/showcase/token-sandbox' },
  ]);
  let copyLogsStatus = $state('');
  let copyLogsTimer = null;

  function newProject()  { docPickerOpen = false; }
  function openProject() { docPickerOpen = false; }
  function openRecentFile(p) {
    docPickerOpen = false;
    const r = recentFiles.find(f => f.path === p);
    if (r) projectName = r.name;
  }
  function beginEditName() {
    docPickerOpen = false;
    nameInput = projectName;
    editingName = true;
  }
  function commitName() {
    editingName = false;
    projectName = (nameInput.trim() || 'Untitled');
  }
  function onNameKey(e) {
    if (e.key === 'Enter') commitName();
    if (e.key === 'Escape') editingName = false;
  }
  function hardReload() { window.location.reload(); }
  async function copyRecentLogs() {
    // Stub — real apps invoke('read_recent_logs') and prefix with build info.
    try {
      await navigator.clipboard.writeText(`=== Libre UI gallery — ${new Date().toISOString()} ===\n(no log pipeline wired)\n`);
      copyLogsStatus = 'ok';
    } catch {
      copyLogsStatus = 'fail';
    }
    if (copyLogsTimer) clearTimeout(copyLogsTimer);
    copyLogsTimer = setTimeout(() => { copyLogsStatus = ''; copyLogsTimer = null; }, 1500);
  }
  function revealLogFolder() {
    // Stub — real apps invoke('reveal_log_folder').
  }

  // ── Updater state machine ──────────────────────────────────────────────
  // The shape is the canonical contract every Libre app implements. When the
  // real `tauri-plugin-updater` wiring lands, replace `runUpdateCheck` with
  // the plugin calls — the footer UI consumes the same shape verbatim.
  /** @type {{status:'idle'}|{status:'checking'}|{status:'up-to-date'}|{status:'available',version:string}|{status:'downloading',progress:number}|{status:'ready'}|{status:'error',message:string}} */
  let updater = $state({ status: 'idle' });

  function closeAbout() {
    if (aboutClosing) return;
    aboutClosing = true;
    setTimeout(() => { aboutOpen = false; aboutClosing = false; }, 250);
  }

  // ── Settings modal ────────────────────────────────────────────────────
  let settingsOpen    = $state(false);
  let settingsClosing = $state(false);
  let settingsTab     = $state('global');

  function closeSettings() {
    if (settingsClosing) return;
    settingsClosing = true;
    setTimeout(() => { settingsOpen = false; settingsClosing = false; }, 250);
  }

  // ── Project Manager modal ─────────────────────────────────────────────
  let projectManagerOpen    = $state(false);
  let projectManagerClosing = $state(false);
  let pmOpenFolders = $state({ active: true, apps: true, archive: false });

  function closeProjectManager() {
    if (projectManagerClosing) return;
    projectManagerClosing = true;
    setTimeout(() => { projectManagerOpen = false; projectManagerClosing = false; }, 250);
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
    updater = { status: 'available', version: 'v0.2.0' };
  }

  // ── Applications tab token scanner (same logic as AppPanels sidebar) ─────
  const APP_TOKEN_PRIORITY = ['color', 'background', 'background-color', 'border-color', 'fill', 'accent-color'];
  const APP_NATIVE_CONTROLS = ['checkbox', 'radio', 'range', 'color', 'file'];

  function scanAppToken(el) {
    if (el instanceof HTMLInputElement && APP_NATIVE_CONTROLS.includes(el.type)) return null;
    const found = new Map();
    try {
      for (const sheet of document.styleSheets) {
        try {
          for (const rule of sheet.cssRules) {
            if (!(rule instanceof CSSStyleRule)) continue;
            try { if (!el.matches(rule.selectorText)) continue; } catch { continue; }
            const re = /([\w-]+)\s*:[^;]*var\((--[\w-]+)\)/g;
            let m;
            while ((m = re.exec(rule.cssText)) !== null) {
              if (!found.has(m[1])) found.set(m[1], m[2]);
            }
          }
        } catch { /* cross-origin */ }
      }
    } catch {}
    for (const prop of APP_TOKEN_PRIORITY) {
      if (found.has(prop)) return found.get(prop);
    }
    return found.size > 0 ? [...found.values()][0] : null;
  }

  let appTt = $state({ visible: false, x: 0, y: 0, token: null, isCard: false });
  let lastAppTtEl = null;

  function onAppMove(e) {
    if (activeTab !== 'applications') { appTt.visible = false; return; }
    const el = document.elementFromPoint(e.clientX, e.clientY);
    if (!el || !contentEl?.contains(el)) { appTt.visible = false; lastAppTtEl = null; return; }
    if (el === lastAppTtEl) return;
    lastAppTtEl = el;
    const zoom = parseFloat(document.documentElement.style.zoom) || 1;
    const cardEl = el.closest('[data-card]');
    appTt.token = cardEl ? cardEl.getAttribute('data-card') : scanAppToken(el);
    appTt.isCard = !!cardEl;
    appTt.x = e.clientX / zoom;
    appTt.y = e.clientY / zoom;
    appTt.visible = true;
  }

  function onAppLeave() { appTt.visible = false; lastAppTtEl = null; }

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
    // Reset scroll + nav highlight whenever the active tab changes.
    visibleSections;
    active = visibleSections[0]?.id ?? '';
    contentEl?.scrollTo({ top: 0, behavior: 'instant' });
  });

  $effect(() => {
    if (!contentEl) return;
    visibleSections; // re-run when tab changes so observer picks up new sections
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

{#if appTt.visible}
  <div class="app-tt" style="left:{appTt.x + 14}px;top:{appTt.y - 28}px">
    <span class="app-tt-bub" class:app-tt-null={!appTt.token && !appTt.isCard} class:app-tt-card={appTt.isCard}>{appTt.token ?? 'null'}</span>
  </div>
{/if}

<div class="shell">
  <!-- Top bar — canonical Libre top chrome (logo · doc picker · workspace tabs · dev tools). -->
  <div class="top-bar" data-tauri-drag-region>
    <!-- LEFT: logo + name + doc picker -->
    <div class="top-left">
      <span class="top-glyph">◈</span>
      <span class="top-app">Libre UI</span>
      <div class="top-divider"></div>

      <div class="doc-picker">
        {#if editingName}
          <!-- svelte-ignore a11y_autofocus -->
          <input
            autofocus
            bind:value={nameInput}
            onblur={commitName}
            onkeydown={onNameKey}
            class="doc-rename"
          />
        {:else}
          <button
            class="doc-btn"
            class:doc-btn-open={docPickerOpen}
            onclick={() => (docPickerOpen = !docPickerOpen)}
            title="Switch project"
          >
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" class="doc-ico">
              <rect x="2" y="6" width="20" height="12" rx="2"/>
              <line x1="2" y1="10" x2="22" y2="10"/>
              <line x1="2" y1="14" x2="22" y2="14"/>
              <line x1="7" y1="6" x2="7" y2="18"/>
              <line x1="17" y1="6" x2="17" y2="18"/>
            </svg>
            <span class="doc-name">{projectName}</span>
            <svg width="9" height="9" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                 stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"
                 class="doc-chev" class:doc-chev-open={docPickerOpen}>
              <polyline points="6 9 12 15 18 9"/>
            </svg>
          </button>
        {/if}

        {#if docPickerOpen}
          <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
          <div class="doc-backdrop" onclick={() => (docPickerOpen = false)}></div>

          <div class="doc-menu">
            <div class="doc-menu-label">Current Project</div>
            <div class="doc-current">
              <span class="doc-current-name">{projectName}</span>
              <button class="doc-rename-btn" onclick={beginEditName} title="Rename project" aria-label="Rename">
                <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                     stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
                  <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
                </svg>
              </button>
            </div>
            <div class="doc-sep"></div>

            <button class="doc-action" onclick={newProject}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                   stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="doc-action-ico">
                <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
              New Project
            </button>
            <button class="doc-action" onclick={openProject}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                   stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="doc-action-ico">
                <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
              </svg>
              Open Project…
            </button>

            {#if recentFiles.length > 0}
              <div class="doc-sep"></div>
              <div class="doc-menu-label">Recent</div>
              {#each recentFiles as f}
                <button class="doc-recent" onclick={() => openRecentFile(f.path)} title={f.path}>
                  {f.name}
                </button>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

      <div class="top-divider"></div>

      <!-- Save As button -->
      <button class="save-as-btn" data-tooltip="Save a copy under a new name">
        Save As
      </button>

      <!-- Save button with unsaved-indicator dot -->
      <button class="save-btn" data-tooltip="Save (⌘S)" aria-label="Save">
        <div class="save-btn-inner">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
            <polyline points="17 21 17 13 7 13 7 21"/>
            <polyline points="7 3 7 8 15 8"/>
          </svg>
          <span class="unsaved-dot" aria-label="Unsaved changes"></span>
        </div>
      </button>
    </div>

    <!-- CENTER: workspace tabs (absolutely centered) -->
    <div class="top-center">
      <div class="tab-groups">
        <GlobalTabs tabs={TABS_FOUNDATION} bind:active={activeTab} />
        <div class="tab-group-sep"></div>
        <GlobalTabs tabs={TABS_APPS} bind:active={activeTab} color="#e07a2f" />
      </div>
    </div>

    <!-- RIGHT: dev tools group + project/global settings + copy status -->
    <div class="top-right">
      <!-- Theme toggle: auto → light → dark -->
      <button
        class="theme-btn"
        class:theme-btn-auto={themeMode === 'auto'}
        class:theme-btn-light={themeMode === 'light'}
        class:theme-btn-dark={themeMode === 'dark'}
        onclick={cycleTheme}
        data-tooltip="Theme: {themeMode === 'auto' ? 'Automatic' : themeMode === 'light' ? 'Light' : 'Dark'} — click to cycle"
        aria-label="Cycle theme"
      >
        {#if themeMode === 'light'}
          <!-- Sun -->
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="4"/>
            <line x1="12" y1="2"  x2="12" y2="5"/>
            <line x1="12" y1="19" x2="12" y2="22"/>
            <line x1="4.22" y1="4.22"  x2="6.34" y2="6.34"/>
            <line x1="17.66" y1="17.66" x2="19.78" y2="19.78"/>
            <line x1="2"  y1="12" x2="5"  y2="12"/>
            <line x1="19" y1="12" x2="22" y2="12"/>
            <line x1="4.22" y1="19.78" x2="6.34" y2="17.66"/>
            <line x1="17.66" y1="6.34" x2="19.78" y2="4.22"/>
          </svg>
        {:else if themeMode === 'dark'}
          <!-- Moon -->
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/>
          </svg>
        {:else}
          <!-- Auto: circle split half light (left filled) half dark (right outline) -->
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M12 5 A7 7 0 0 0 12 19 Z" fill="currentColor" stroke="none"/>
            <circle cx="12" cy="12" r="7"/>
            <line x1="12" y1="5" x2="12" y2="19"/>
          </svg>
        {/if}
      </button>

      <!-- Project Manager -->
      <button
        class="gear-btn"
        onclick={() => (projectManagerOpen = true)}
        data-tooltip="Project Manager"
        aria-label="Project Manager"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/>
          <polyline points="9 22 9 12 15 12 15 22"/>
        </svg>
      </button>

      <!-- Global Settings -->
      <button
        class="gear-btn"
        onclick={() => (settingsOpen = true)}
        data-tooltip="Global Settings"
        aria-label="Global Settings"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="3"/>
          <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/>
        </svg>
      </button>

      <!-- Right sidebar toggle -->
      <button
        class="gear-btn"
        class:sidebar-toggle-active={!panelsCollapsed}
        onclick={togglePanels}
        data-tooltip="{panelsCollapsed ? 'Show' : 'Hide'} right panel"
        aria-label="Toggle right panel"
      >
        <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
             stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <line x1="15" y1="3" x2="15" y2="21"/>
        </svg>
      </button>

      <div class="dev-group">
        <button class="dev-btn" onclick={hardReload} data-tooltip="Restart app (dev)" title="Restart app" aria-label="Restart app">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8"/>
            <path d="M3 3v5h5"/>
          </svg>
        </button>
        <button class="dev-btn" onclick={copyRecentLogs} data-tooltip="Copy recent logs to clipboard (dev)" title="Copy recent logs" aria-label="Copy recent logs">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <rect x="9" y="2" width="6" height="4" rx="1"/>
            <path d="M9 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2h-3"/>
          </svg>
        </button>
        <button class="dev-btn" onclick={revealLogFolder} data-tooltip="Reveal log folder (dev)" title="Reveal log folder" aria-label="Reveal log folder">
          <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
            <path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7z"/>
          </svg>
        </button>
      </div>
      {#if copyLogsStatus}
        <span class="copy-status" class:copy-status-fail={copyLogsStatus === 'fail'}>
          {copyLogsStatus === 'ok' ? 'Copied ✓' : 'Copy failed'}
        </span>
      {/if}
    </div>
  </div>

  <div class="main-row">
    <aside class="sidebar">
      <nav>
        {#each visibleSections as s}
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
        <ThemeLab bind:dark />
      </div>
    </aside>

    <MiniMap {contentEl} visible={activeTab === 'components' || activeTab === 'applications' || activeTab === 'foundation' || activeTab === 'surface' || activeTab === 'overview'} />

    <main class="content" bind:this={contentEl} onmousemove={onAppMove} onmouseleave={onAppLeave}>
      {#each visibleSections as s}
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

    <AppPanels collapsed={panelsCollapsed} />
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

    <!-- CENTER: hint footer — rolling hint text, with optional updater override -->
    <span
      class="hint-footer"
      style="opacity:{tooltip.opacity}; transition:opacity {tooltip.duration}ms linear {tooltip.delay}ms"
    >
      {tooltip.text}
    </span>

    <!-- RIGHT: updater chip + version/about -->
    <div class="footer-right">
      {#if updater.status === 'available' || updater.status === 'ready'}
        <button
          class="update-available-label libre-pulse"
          onclick={() => (settingsOpen = true)}
        >update available</button>
      {/if}
      <button
        class="version-btn"
        onclick={() => (aboutOpen = true)}
        data-tooltip="About Libre UI"
      >about</button>
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

<!-- Settings modal — tabbed: Global Settings / Project Settings -->
{#if settingsOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div
    class="{settingsClosing ? 'about-backdrop-out' : 'about-backdrop'} about-shell"
    onpointerdown={closeSettings}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
    <div
      class="{settingsClosing ? 'about-card-out' : 'about-card'} settings-frame"
      onpointerdown={(e) => e.stopPropagation()}
    >
      <!-- Update banner -->
      <div class="upd-banner">
        <span class="upd-banner-dot"></span>
        <span class="upd-banner-text">Update available — <strong>v0.2.0</strong></span>
        <button class="upd-banner-btn">Install Update</button>
        <button class="upd-banner-dismiss" aria-label="Dismiss">
          <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <!-- Tab bar header — matches TabBar component exactly -->
      <div class="stab-header" role="tablist">
        <button
          role="tab"
          class="stab"
          class:stab-active={settingsTab === 'global'}
          onclick={() => settingsTab = 'global'}
          aria-selected={settingsTab === 'global'}
        >Global Settings</button>
        <button
          role="tab"
          class="stab"
          class:stab-active={settingsTab === 'project'}
          onclick={() => settingsTab = 'project'}
          aria-selected={settingsTab === 'project'}
        >Project Settings</button>
        <button
          role="tab"
          class="stab"
          class:stab-active={settingsTab === 'plugins'}
          onclick={() => settingsTab = 'plugins'}
          aria-selected={settingsTab === 'plugins'}
        >Plug-ins</button>
        <div class="stab-spacer"></div>
        <button class="settings-close" onclick={closeSettings} aria-label="Close settings">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="settings-body">
        {#if settingsTab === 'global'}
          <!-- Appearance -->
          <div class="settings-section-label">Appearance</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Theme</span>
              <span class="settings-row-desc">Light or dark interface</span>
            </div>
            <select class="settings-select">
              <option>System</option>
              <option>Light</option>
              <option>Dark</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Accent color</span>
              <span class="settings-row-desc">Synced from ~/.config/librewin/accent</span>
            </div>
            <div class="settings-accent-row">
              {#each ['#0066cc','#7c3aed','#059669','#dc2626','#d97706','#db2777'] as col}
                <button class="settings-swatch" style="background:{col}" aria-label={col}></button>
              {/each}
            </div>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Interface density</span>
              <span class="settings-row-desc">Controls padding and spacing</span>
            </div>
            <select class="settings-select">
              <option>Compact</option>
              <option>Default</option>
              <option>Spacious</option>
            </select>
          </div>

          <!-- Gallery -->
          <div class="settings-section-label">Gallery</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Default zoom</span>
              <span class="settings-row-desc">Starting zoom level on launch</span>
            </div>
            <select class="settings-select">
              <option>75%</option><option>90%</option>
              <option selected>100%</option>
              <option>110%</option><option>125%</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Show card IDs</span>
              <span class="settings-row-desc">Display component IDs in card headers</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" checked />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Show source paths</span>
              <span class="settings-row-desc">Show file path in token inspector</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" checked />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

          <!-- Developer -->
          <div class="settings-section-label">Developer</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Dev tools bar</span>
              <span class="settings-row-desc">Reload, logs, and folder shortcuts</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" checked />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Log level</span>
              <span class="settings-row-desc">Verbosity for the dev log pipeline</span>
            </div>
            <select class="settings-select">
              <option>Error</option><option>Warn</option>
              <option selected>Info</option>
              <option>Debug</option><option>Trace</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Persist focus on reload</span>
              <span class="settings-row-desc">Write .focus.json before dev restart</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

        {:else}
          <!-- Identity -->
          <div class="settings-section-label">Identity</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Project name</span>
              <span class="settings-row-desc">Display name used in title bar and recents</span>
            </div>
            <input class="settings-input" type="text" value={projectName} />
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Identifier</span>
              <span class="settings-row-desc">Internal slug — used in save paths</span>
            </div>
            <input class="settings-input" type="text" value="libre-ui" />
          </div>

          <!-- Layout -->
          <div class="settings-section-label">Layout</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Default section</span>
              <span class="settings-row-desc">Section shown on launch</span>
            </div>
            <select class="settings-select">
              <option>Demo Tiles</option><option>Tokens</option>
              <option>Typography</option><option>Buttons & Actions</option>
              <option>Form Controls</option><option>Navigation</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Right panel</span>
              <span class="settings-row-desc">Panel open on launch</span>
            </div>
            <select class="settings-select">
              <option>Collapsed</option><option>Flicker Inspector</option>
              <option>Fade MP3</option><option>TurboTalk Settings</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Remember scroll position</span>
              <span class="settings-row-desc">Restore last scroll on reopen</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" checked />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

          <!-- Tokens -->
          <div class="settings-section-label">Tokens</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Token source</span>
              <span class="settings-row-desc">CSS file used as the token reference</span>
            </div>
            <select class="settings-select">
              <option>common-js/src/tokens.css</option>
              <option>Custom…</option>
            </select>
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Sync accent on save</span>
              <span class="settings-row-desc">Write accent to ~/.config/librewin/accent</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

          <!-- Export -->
          <div class="settings-section-label">Export</div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Export path</span>
              <span class="settings-row-desc">Default output folder for snapshots</span>
            </div>
            <input class="settings-input" type="text" value="~/Downloads" />
          </div>

          <div class="settings-row">
            <div class="settings-row-text">
              <span class="settings-row-label">Include dark mode variants</span>
              <span class="settings-row-desc">Export both light and dark screenshots</span>
            </div>
            <label class="settings-toggle">
              <input type="checkbox" checked />
              <span class="settings-toggle-track"></span>
            </label>
          </div>

        {:else}
          <!-- Plugins tab -->
          <div class="settings-section-label">Installed</div>

          {#each [
            { name: 'Token Exporter',   desc: 'Export tokens to CSS / JSON / Figma',  version: '1.2.0', enabled: true  },
            { name: 'Contrast Checker', desc: 'WCAG AA/AAA contrast audit on any card', version: '0.9.1', enabled: true  },
            { name: 'Screenshot Grid',  desc: 'Batch-capture all gallery cards to PNG', version: '1.0.3', enabled: false },
            { name: 'Figma Sync',       desc: 'Push component snapshots to Figma',     version: '0.4.0', enabled: false },
          ] as p}
            <div class="settings-row plugin-row">
              <div class="settings-row-text">
                <span class="settings-row-label">{p.name}</span>
                <span class="settings-row-desc">{p.desc}</span>
              </div>
              <div class="plugin-right">
                <span class="plugin-version">v{p.version}</span>
                <label class="settings-toggle">
                  <input type="checkbox" checked={p.enabled} />
                  <span class="settings-toggle-track"></span>
                </label>
              </div>
            </div>
          {/each}

          <div class="settings-section-label">Available</div>

          {#each [
            { name: 'Motion Preview',   desc: 'Animate transitions between card states' },
            { name: 'A11y Audit',       desc: 'Run accessibility checks across sections' },
            { name: 'Icon Browser',     desc: 'Browse and insert Libre icon set'         },
          ] as p}
            <div class="settings-row plugin-row">
              <div class="settings-row-text">
                <span class="settings-row-label">{p.name}</span>
                <span class="settings-row-desc">{p.desc}</span>
              </div>
              <button class="plugin-install-btn">Install</button>
            </div>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- Project Manager modal — 1200px wide mockup -->
{#if projectManagerOpen}
  <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
  <div
    class="{projectManagerClosing ? 'about-backdrop-out' : 'about-backdrop'} about-shell"
    onpointerdown={closeProjectManager}
  >
    <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
    <div
      class="{projectManagerClosing ? 'about-card-out' : 'about-card'} pm-frame"
      onpointerdown={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="pm-header">
        <span class="pm-title">Project Manager</span>
        <button class="settings-close" onclick={closeProjectManager} aria-label="Close project manager">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor"
               stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"/><line x1="6" y1="6" x2="18" y2="18"/>
          </svg>
        </button>
      </div>

      <div class="pm-body">
        <!-- Sidebar: project list -->
        <div class="pm-sidebar">
          <div class="pm-sidebar-header">
            <span class="pm-sidebar-label">Projects</span>
            <button class="pm-new-btn" title="New project">
              <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                   stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <line x1="12" y1="5" x2="12" y2="19"/><line x1="5" y1="12" x2="19" y2="12"/>
              </svg>
            </button>
          </div>

          {#each [
            { key: 'active',  label: 'Active', projects: [
              { name: 'Libre UI',       mod: 'Today',       active: true  },
              { name: 'Component Lab',  mod: 'Yesterday',   active: false },
              { name: 'Token Sandbox',  mod: '3 days ago',  active: false },
            ]},
            { key: 'apps',    label: 'Apps', projects: [
              { name: 'Fade Converter', mod: 'Last week',   active: false },
              { name: 'Prism Viewer',   mod: 'Last week',   active: false },
              { name: 'Stack Editor',   mod: '2 weeks ago', active: false },
            ]},
            { key: 'archive', label: 'Archive', projects: [
              { name: 'Motion Demos',       mod: 'Last month', active: false },
              { name: 'Accessibility Pass', mod: 'Last month', active: false },
            ]},
          ] as folder}
            <div class="pm-folder">
              <button class="pm-folder-hd" onclick={() => { pmOpenFolders = { ...pmOpenFolders, [folder.key]: !pmOpenFolders[folder.key] }; }}>
                <svg class="pm-folder-chevron" class:pm-folder-chevron-open={pmOpenFolders[folder.key]}
                     width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                     stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="9 18 15 12 9 6"/>
                </svg>
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                     stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                  <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
                </svg>
                <span class="pm-folder-name">{folder.label}</span>
                <span class="pm-folder-count">{folder.projects.length}</span>
              </button>
              {#if pmOpenFolders[folder.key]}
                {#each folder.projects as p}
                  <button class="pm-project-row pm-project-indented" class:pm-project-active={p.active}>
                    <div class="pm-project-ico">
                      <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                           stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round">
                        <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
                        <line x1="3" y1="9" x2="21" y2="9"/>
                        <line x1="9" y1="21" x2="9" y2="9"/>
                      </svg>
                    </div>
                    <div class="pm-project-meta">
                      <span class="pm-project-name">{p.name}</span>
                      <span class="pm-project-mod">{p.mod}</span>
                    </div>
                  </button>
                {/each}
              {/if}
            </div>
          {/each}
        </div>

        <!-- Main: project detail -->
        <div class="pm-detail">
          <div class="pm-detail-hero">
            <div class="pm-detail-glyph">◈</div>
            <div class="pm-detail-heading">
              <h2 class="pm-detail-name">Libre UI</h2>
              <span class="pm-detail-path">~/Downloads/Github/Libre-Apps/gallery</span>
            </div>
          </div>

          <div class="pm-detail-meta-grid">
            <div class="pm-meta-item"><span class="pm-meta-key">Last opened</span><span class="pm-meta-val">Today at 2:14 PM</span></div>
            <div class="pm-meta-item"><span class="pm-meta-key">Created</span><span class="pm-meta-val">Apr 20, 2026</span></div>
            <div class="pm-meta-item"><span class="pm-meta-key">Runtime</span><span class="pm-meta-val">Tauri + Svelte 5</span></div>
            <div class="pm-meta-item"><span class="pm-meta-key">Token source</span><span class="pm-meta-val">common-js/src/tokens.css</span></div>
            <div class="pm-meta-item"><span class="pm-meta-key">Identifier</span><span class="pm-meta-val">io.librewin.gallery</span></div>
            <div class="pm-meta-item"><span class="pm-meta-key">Version</span><span class="pm-meta-val">0.1.0</span></div>
          </div>

          <div class="pm-detail-section-label">Recent Snapshots</div>
          <div class="pm-snapshots">
            {#each ['Buttons — dark mode', 'Token audit', 'Navigation variants', 'Form controls pass'] as s}
              <div class="pm-snapshot-chip">{s}</div>
            {/each}
          </div>

          <div class="pm-detail-section-label">Actions</div>
          <div class="pm-actions">
            <button class="pm-action-btn pm-action-primary">Open Project</button>
            <button class="pm-action-btn">Duplicate</button>
            <button class="pm-action-btn">Export Snapshot</button>
            <button class="pm-action-btn pm-action-danger">Remove from List</button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}


<style>
  /* ── Applications tab token tooltip ──────────────────────────────────── */
  .app-tt {
    position: fixed;
    z-index: 9999;
    pointer-events: none;
  }
  .app-tt-bub {
    background: var(--text-primary);
    color: var(--surface);
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    padding: 4px 8px;
    border-radius: 4px;
    white-space: nowrap;
    line-height: 1;
    display: block;
  }
  .app-tt-null { color: #f5a623; }
  .app-tt-card { color: var(--accent); }

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
  .nav-item:hover { color: #fff; }
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
    font-size: 60px;
    font-weight: 700;
    letter-spacing: -0.03em;
    color: var(--text-primary);
    margin: 0 0 32px 0;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border);
    line-height: 1.1;
  }

  /* ── Top bar ───────────────────────────────────────────────────────── */
  .top-bar {
    height: 44px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
    padding: 0 12px;
    border-bottom: 1px solid var(--border);
    background: var(--titlebar-bg, var(--surface-panel));
    position: relative;
    user-select: none;
    -webkit-user-select: none;
  }

  .top-left {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
    z-index: 2;
  }

  .top-glyph {
    color: var(--accent);
    font-size: 14px;
    flex-shrink: 0;
  }
  .top-app {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
    flex-shrink: 0;
  }
  .top-divider {
    width: 1px;
    height: 14px;
    background: var(--border);
    margin: 0 2px;
    flex-shrink: 0;
  }

  /* Doc picker */
  .doc-picker { position: relative; }

  .doc-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 5px 8px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .doc-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }
  .doc-btn-open {
    background: color-mix(in srgb, var(--text-primary) 10%, transparent);
    color: var(--text-primary);
  }

  .doc-ico { opacity: 0.7; flex-shrink: 0; }
  .doc-name {
    max-width: 160px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .doc-chev {
    opacity: 0.5;
    flex-shrink: 0;
    transition: transform 0.15s;
  }
  .doc-chev-open { transform: rotate(180deg); }

  .doc-rename {
    background: var(--surface);
    border: 1px solid var(--accent);
    border-radius: 4px;
    padding: 2px 8px;
    width: 176px;
    font-size: 12px;
    font-family: inherit;
    color: var(--text-primary);
    outline: none;
  }

  .doc-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
  }

  .doc-menu {
    position: absolute;
    top: 100%;
    left: 0;
    margin-top: 6px;
    width: 256px;
    border-radius: 8px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    box-shadow: 0 16px 48px rgb(0 0 0 / 0.4);
    padding: 6px 0;
    overflow: hidden;
    z-index: 50;
  }

  .doc-menu-label {
    padding: 4px 12px 2px;
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .doc-current {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 6px 12px;
  }
  .doc-current-name {
    flex: 1;
    font-size: 12px;
    font-weight: 500;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .doc-rename-btn {
    background: transparent;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 2px;
    flex-shrink: 0;
    transition: color 0.1s;
  }
  .doc-rename-btn:hover { color: var(--text-primary); }

  .doc-sep {
    height: 1px;
    background: var(--border);
    margin: 4px 12px;
  }

  .doc-action,
  .doc-recent {
    width: 100%;
    text-align: left;
    background: transparent;
    border: none;
    padding: 6px 12px;
    font-size: 12px;
    font-family: inherit;
    color: var(--text-secondary);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .doc-action:hover,
  .doc-recent:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }
  .doc-recent {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .doc-action-ico { opacity: 0.6; flex-shrink: 0; }

  /* Save As + Save buttons */
  .save-as-btn {
    display: flex;
    align-items: center;
    padding: 4px 8px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    font-size: 12px;
    font-family: inherit;
    cursor: pointer;
    white-space: nowrap;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .save-as-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }

  .save-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px 6px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-secondary);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .save-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
  }

  .save-btn-inner {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .unsaved-dot {
    position: absolute;
    top: -3px;
    right: -3px;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #ef4444;
    border: 1.5px solid var(--titlebar-bg, var(--surface-panel));
    flex-shrink: 0;
  }

  /* Workspace tabs container — overlay-centered. The pill itself is GlobalTabs. */
  .top-center {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
  }
  .top-center :global(.gt-group) { pointer-events: auto; }

  .tab-groups {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .tab-group-sep {
    width: 1px;
    height: 16px;
    background: rgb(255 255 255 / 0.15);
    flex-shrink: 0;
  }

  /* Right zone — dev tools */
  .top-right {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-left: auto;
    z-index: 2;
  }

  .dev-group {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 2px 4px;
    border-radius: 5px;
    background: rgb(74 222 128 / 0.06);
    border: 1px solid rgb(74 222 128 / 0.22);
  }
  .dev-btn {
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: transparent;
    border: none;
    border-radius: 3px;
    color: #4ade80;
    cursor: pointer;
    padding: 0;
    transition: background 0.1s;
  }
  .dev-btn:hover { background: rgb(74 222 128 / 0.15); }

  .copy-status {
    font-size: 11px;
    color: var(--text-secondary);
  }
  .copy-status-fail { color: #ef4444; }

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

  .hint-footer {
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

  .update-available-label {
    font-size: 10px;
    font-weight: 500;
    color: color-mix(in srgb, var(--text-primary) 20%, transparent);
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
  }
  .update-available-label:hover { color: var(--text-primary); }

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
    height: 600px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    overflow: hidden;
    box-shadow: 0 24px 64px rgb(0 0 0 / 0.45);
    display: flex;
    flex-direction: column;
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
    flex: 1;
    min-height: 0;
    overflow-y: auto;
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
    animation: about-backdrop-in 1s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }
  @keyframes about-card-in {
    0%   { opacity: 0; filter: blur(8px); transform: translateY(12px); }
    100% { opacity: 1; filter: blur(0px); transform: translateY(0); }
  }
  .about-card {
    opacity: 0;
    animation: about-card-in 0.5s cubic-bezier(0.22, 1, 0.36, 1) forwards;
  }
  @keyframes about-backdrop-out {
    0%   { background: rgb(0 0 0 / 0.5); backdrop-filter: blur(7px); }
    100% { background: rgb(0 0 0 / 0);   backdrop-filter: blur(0px); }
  }
  .about-backdrop-out {
    animation: about-backdrop-out 0.25s cubic-bezier(0.4, 0, 0.2, 1) forwards;
  }
  @keyframes about-card-out {
    0%   { opacity: 1; filter: blur(0px); transform: translateY(0); }
    100% { opacity: 0; filter: blur(8px); transform: translateY(12px); }
  }
  .about-card-out {
    animation: about-card-out 0.25s cubic-bezier(0.4, 0, 1, 1) forwards;
  }

  /* ── Update banner ─────────────────────────────────────────────────── */
  .upd-banner {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    background: color-mix(in srgb, var(--accent) 10%, var(--surface-panel));
    border-bottom: 1px solid color-mix(in srgb, var(--accent) 25%, transparent);
    flex-shrink: 0;
  }

  .upd-banner-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
    animation: upd-pulse 1.8s ease-in-out infinite;
  }

  .upd-banner-text {
    flex: 1;
    font-size: 12px;
    color: var(--text-secondary);
  }
  .upd-banner-text strong { color: var(--text-primary); }

  .upd-banner-btn {
    font-size: 11px;
    font-weight: 600;
    font-family: inherit;
    padding: 4px 12px;
    border-radius: 5px;
    border: 1px solid var(--accent);
    background: var(--accent);
    color: #fff;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s;
  }
  .upd-banner-btn:hover { background: var(--accent-hover); }

  .upd-banner-dismiss {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    background: transparent;
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .upd-banner-dismiss:hover { background: color-mix(in srgb, var(--text-primary) 8%, transparent); color: var(--text-primary); }

  /* ── Plugin rows ────────────────────────────────────────────────────── */
  .plugin-row { align-items: center; }

  .plugin-right {
    display: flex;
    align-items: center;
    gap: 10px;
    flex-shrink: 0;
  }

  .plugin-version {
    font-size: 10px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  .plugin-install-btn {
    font-size: 11px;
    font-weight: 600;
    font-family: inherit;
    padding: 4px 12px;
    border-radius: 5px;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, border-color 0.1s;
  }
  .plugin-install-btn:hover { background: color-mix(in srgb, var(--text-primary) 10%, transparent); }

  /* ── Settings tab bar (matches TabBar component) ──────────────────── */
  .stab-header {
    display: flex;
    align-items: stretch;
    border-bottom: 1px solid var(--border);
    background: color-mix(in srgb, black 10%, var(--surface-raised));
    flex-shrink: 0;
  }

  .stab {
    padding: 8px 20px;
    font-size: 13px;
    font-weight: 500;
    font-family: inherit;
    background: transparent;
    border: none;
    border-bottom: 2px solid transparent;
    color: var(--text-secondary);
    cursor: pointer;
    transition: color 0.1s, border-color 0.1s;
    outline: none;
    white-space: nowrap;
  }
  .stab:hover { color: var(--text-primary); }
  .stab-active {
    border-bottom-color: var(--accent);
    color: var(--text-primary);
    background: color-mix(in srgb, white 18%, var(--surface-raised));
  }

  .stab-spacer { flex: 1; }

  /* ── Project Manager modal ─────────────────────────────────────────── */
  .pm-frame {
    position: relative;
    width: 1200px;
    height: 900px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    overflow: hidden;
    box-shadow: 0 24px 64px rgb(0 0 0 / 0.45);
    display: flex;
    flex-direction: column;
  }

  .pm-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-panel);
    flex-shrink: 0;
  }

  .pm-title {
    font-size: 19px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .pm-body {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }

  /* Sidebar */
  .pm-sidebar {
    width: 220px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface-panel);
    display: flex;
    flex-direction: column;
    overflow-y: auto;
  }

  .pm-sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 14px 6px;
    flex-shrink: 0;
  }

  .pm-sidebar-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .pm-new-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .pm-new-btn:hover { background: color-mix(in srgb, var(--text-primary) 14%, transparent); color: var(--text-primary); }

  .pm-project-row {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 5px 14px;
    background: transparent;
    border: none;
    cursor: pointer;
    text-align: left;
    transition: background 0.08s;
  }

  .pm-folder { display: flex; flex-direction: column; }

  .pm-folder-hd {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 4px 10px;
    background: transparent;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    text-align: left;
    transition: background 0.08s, color 0.08s;
  }
  .pm-folder-hd:hover { background: color-mix(in srgb, var(--text-primary) 4%, transparent); color: var(--text-secondary); }

  .pm-folder-chevron { transition: transform 0.15s; flex-shrink: 0; }
  .pm-folder-chevron-open { transform: rotate(90deg); }

  .pm-folder-name {
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    flex: 1;
  }

  .pm-folder-count {
    font-size: 10px;
    color: var(--text-muted);
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    border-radius: 8px;
    padding: 1px 5px;
  }

  .pm-project-indented { padding-left: 28px; }
  .pm-project-row:hover { background: color-mix(in srgb, var(--text-primary) 4%, transparent); }
  .pm-project-active { background: color-mix(in srgb, var(--accent) 10%, transparent) !important; }

  .pm-project-ico { color: var(--text-muted); flex-shrink: 0; }
  .pm-project-active .pm-project-ico { color: var(--accent); }

  .pm-project-meta { display: flex; flex-direction: column; gap: 1px; min-width: 0; }

  .pm-project-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .pm-project-mod {
    font-size: 10px;
    color: var(--text-muted);
  }

  /* Detail pane */
  .pm-detail {
    flex: 1;
    min-width: 0;
    overflow-y: auto;
    padding: 18px 28px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .pm-detail-hero {
    display: flex;
    align-items: center;
    gap: 16px;
  }

  .pm-detail-glyph {
    width: 52px;
    height: 52px;
    border-radius: 14px;
    background: color-mix(in srgb, var(--accent) 14%, var(--surface));
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--accent);
    font-size: 22px;
    flex-shrink: 0;
  }

  .pm-detail-heading { display: flex; flex-direction: column; gap: 4px; }

  .pm-detail-name {
    font-size: 22px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 0;
    letter-spacing: -0.02em;
  }

  .pm-detail-path {
    font-size: 11px;
    color: var(--text-muted);
    font-family: 'Geist Mono', monospace;
  }

  .pm-detail-meta-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2px;
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    overflow: hidden;
  }

  .pm-meta-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 14px;
    background: var(--surface-panel);
    gap: 12px;
  }
  .pm-meta-item:nth-child(odd) { background: color-mix(in srgb, var(--surface-panel) 96%, #000); }

  .pm-meta-key { font-size: 11px; color: var(--text-muted); flex-shrink: 0; }
  .pm-meta-val { font-size: 11px; color: var(--text-primary); font-family: 'Geist Mono', monospace; text-align: right; }

  .pm-detail-section-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .pm-snapshots {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .pm-snapshot-chip {
    font-size: 11px;
    padding: 4px 10px;
    border-radius: 999px;
    background: color-mix(in srgb, var(--text-primary) 6%, transparent);
    border: 1px solid var(--border);
    color: var(--text-secondary);
  }

  .pm-actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .pm-action-btn {
    padding: 7px 16px;
    font-size: 13px;
    font-weight: 500;
    font-family: inherit;
    border-radius: 7px;
    border: 1px solid var(--border);
    background: color-mix(in srgb, var(--text-primary) 5%, transparent);
    color: var(--text-primary);
    cursor: pointer;
    transition: background 0.1s, border-color 0.1s;
  }
  .pm-action-btn:hover { background: color-mix(in srgb, var(--text-primary) 10%, transparent); }
  .pm-action-primary {
    background: var(--accent);
    border-color: var(--accent);
    color: #fff;
  }
  .pm-action-primary:hover { background: var(--accent-hover); border-color: var(--accent-hover); }
  .pm-action-danger { color: #ef4444; border-color: color-mix(in srgb, #ef4444 30%, transparent); }
  .pm-action-danger:hover { background: color-mix(in srgb, #ef4444 8%, transparent); }

  /* ── Theme toggle button ───────────────────────────────────────────── */
  .theme-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    background: transparent;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .theme-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 7%, transparent);
  }
  .theme-btn-light { color: #d97706; }
  .theme-btn-dark  { color: #818cf8; }
  .theme-btn-auto  { color: var(--text-muted); }
  .theme-btn-light:hover { color: #b45309; }
  .theme-btn-dark:hover  { color: #6366f1; }
  .theme-btn-auto:hover  { color: var(--text-primary); }

  /* ── Gear button ───────────────────────────────────────────────────── */
  .gear-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    background: transparent;
    border: none;
    border-radius: 5px;
    color: var(--text-muted);
    cursor: pointer;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .gear-btn:hover {
    background: color-mix(in srgb, var(--text-primary) 7%, transparent);
    color: var(--text-primary);
  }
  .sidebar-toggle-active {
    color: var(--text-secondary);
  }

  /* ── Settings modal ────────────────────────────────────────────────── */
  .settings-frame {
    position: relative;
    width: 480px;
    height: 600px;
    border-radius: 12px;
    border: 1px solid var(--border);
    background: var(--surface-raised);
    overflow: hidden;
    box-shadow: 0 24px 64px rgb(0 0 0 / 0.45);
    display: flex;
    flex-direction: column;
  }

  .settings-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-panel);
    flex-shrink: 0;
  }

  .settings-title {
    font-size: 19px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .settings-close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: color-mix(in srgb, var(--text-primary) 6%, transparent);
    border: none;
    border-radius: 5px;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .settings-close:hover {
    background: color-mix(in srgb, var(--text-primary) 12%, transparent);
    color: var(--text-primary);
  }

  .settings-body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 8px 0 16px;
  }

  .settings-section-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 16px 20px 6px;
  }

  .settings-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 24px;
    padding: 9px 20px;
    transition: background 0.08s;
  }
  .settings-row:hover {
    background: color-mix(in srgb, var(--text-primary) 3%, transparent);
  }

  .settings-row-text {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .settings-row-label {
    font-size: 13px;
    color: var(--text-primary);
    font-weight: 500;
  }

  .settings-row-desc {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .settings-select {
    appearance: none;
    -webkit-appearance: none;
    background: color-mix(in srgb, var(--text-primary) 6%, transparent);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 12px;
    font-family: inherit;
    padding: 5px 28px 5px 10px;
    cursor: pointer;
    outline: none;
    flex-shrink: 0;
    min-width: 100px;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='10' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2.5' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 8px center;
    transition: border-color 0.1s;
  }
  .settings-select:hover { border-color: color-mix(in srgb, var(--text-primary) 30%, transparent); }
  .settings-select:focus { border-color: var(--accent); }

  .settings-input {
    background: color-mix(in srgb, var(--text-primary) 6%, transparent);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-primary);
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
    padding: 5px 10px;
    outline: none;
    flex-shrink: 0;
    min-width: 140px;
    transition: border-color 0.1s;
  }
  .settings-input:hover { border-color: color-mix(in srgb, var(--text-primary) 30%, transparent); }
  .settings-input:focus { border-color: var(--accent); }

  /* Accent swatches */
  .settings-accent-row {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .settings-swatch {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    padding: 0;
    transition: transform 0.1s, border-color 0.1s;
  }
  .settings-swatch:hover {
    transform: scale(1.2);
    border-color: var(--border);
  }

  /* Toggle switch */
  .settings-toggle {
    position: relative;
    display: inline-flex;
    align-items: center;
    cursor: pointer;
    flex-shrink: 0;
  }

  .settings-toggle input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .settings-toggle-track {
    display: block;
    width: 32px;
    height: 18px;
    border-radius: 9px;
    background: color-mix(in srgb, var(--text-primary) 15%, transparent);
    border: 1px solid var(--border);
    position: relative;
    transition: background 0.15s, border-color 0.15s;
  }

  .settings-toggle-track::after {
    content: '';
    position: absolute;
    top: 2px;
    left: 2px;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--text-muted);
    transition: transform 0.15s, background 0.15s;
  }

  .settings-toggle input:checked + .settings-toggle-track {
    background: var(--accent);
    border-color: var(--accent);
  }

  .settings-toggle input:checked + .settings-toggle-track::after {
    transform: translateX(14px);
    background: #fff;
  }
</style>
