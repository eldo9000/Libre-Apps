<script>
  import { Toaster } from '@libre/ui';
  import ThemeLab from './lib/ThemeLab.svelte';
  import AppPanels from './lib/AppPanels.svelte';
  import TokensSection from './sections/TokensSection.svelte';
  import ButtonsSection from './sections/ButtonsSection.svelte';
  import FormSection from './sections/FormSection.svelte';
  import FeedbackSection from './sections/FeedbackSection.svelte';
  import NavigationSection from './sections/NavigationSection.svelte';
  import LayoutSection from './sections/LayoutSection.svelte';

  const sections = [
    { id: 'tokens',     label: 'Tokens',           component: TokensSection },
    { id: 'buttons',    label: 'Buttons & Actions', component: ButtonsSection },
    { id: 'form',       label: 'Form Controls',     component: FormSection },
    { id: 'feedback',   label: 'Feedback',          component: FeedbackSection },
    { id: 'navigation', label: 'Navigation',        component: NavigationSection },
    { id: 'layout',     label: 'Layout',            component: LayoutSection },
  ];

  let dark      = $state(false);
  let toaster   = $state(null);
  let zoom      = $state(100);
  let active    = $state('tokens');
  let contentEl = $state(null);

  $effect(() => {
    document.documentElement.classList.toggle('dark', dark);
  });

  $effect(() => {
    document.documentElement.style.zoom = String(zoom / 100);
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

  function zoomIn()  { if (zoom < 200) zoom += 10; }
  function zoomOut() { if (zoom > 100) zoom -= 10; }

  $effect(() => {
    function onWheel(e) {
      if (!e.ctrlKey) return;
      e.preventDefault();
      if (e.deltaY < 0) zoomIn(); else zoomOut();
    }
    function onKeydown(e) {
      if (!e.ctrlKey && !e.metaKey) return;
      if (e.key === '=' || e.key === '+') { e.preventDefault(); zoomIn(); }
      else if (e.key === '-')             { e.preventDefault(); zoomOut(); }
      else if (e.key === '0')             { e.preventDefault(); zoom = 100; }
      zoom = Math.max(100, Math.min(200, zoom));
    }
    window.addEventListener('wheel', onWheel, { passive: false });
    window.addEventListener('keydown', onKeydown);
    return () => {
      window.removeEventListener('wheel', onWheel);
      window.removeEventListener('keydown', onKeydown);
    };
  });
</script>

<div class="shell">
  <aside class="sidebar">
    <div class="sidebar-header">
      <div class="logo">
        <span class="logo-mark">◈</span>
        Libre UI
      </div>
      <button class="theme-btn" onclick={() => (dark = !dark)} title="Toggle theme">
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
  </aside>

  <main class="content" bind:this={contentEl}>
    <div class="zoom-wrap">
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
    </div>
  </main>

  <AppPanels />
  <Toaster bind:this={toaster} />
</div>

<!-- Pinned dock — position:fixed anchors it to viewport bottom-left, scales with html zoom but never gets pushed off-screen by overflow. -->
<aside class="dock">
  <ThemeLab />
  <div class="dock-footer">
    <button class="zoom-btn" onclick={zoomOut} disabled={zoom <= 100} aria-label="Zoom out">−</button>
    <button class="zoom-label" onclick={() => zoom = 100} title="Reset zoom">{zoom}%</button>
    <button class="zoom-btn" onclick={zoomIn} disabled={zoom >= 200} aria-label="Zoom in">+</button>
  </div>
</aside>

<style>
  .shell {
    display: flex;
    height: 100vh;
    overflow: hidden;
    background: var(--surface);
    color: var(--text-primary);
  }

  .sidebar {
    width: 200px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    background: var(--surface-panel);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

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

  .logo-mark {
    color: var(--accent);
    font-size: 15px;
  }

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

  nav {
    padding: 8px 8px 280px;     /* extra bottom padding so items can scroll past the fixed dock */
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }

  .dock {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 200px;                /* matches sidebar width — both scale identically under html-zoom */
    background: var(--surface-panel);
    border-right: 1px solid var(--border);
    border-top: 1px solid var(--border);
    z-index: 10;
    display: flex;
    flex-direction: column;
  }

  .dock-footer {
    padding: 8px 12px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 4px;
  }

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
  }

  .zoom-wrap {
    padding: 48px 56px 120px;
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

  .sidebar-footer {
    padding: 8px 12px;
    border-top: 1px solid var(--border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 4px;
  }

  .zoom-btn {
    background: none;
    border: 1px solid var(--border);
    border-radius: 5px;
    width: 26px;
    height: 26px;
    cursor: pointer;
    font-size: 15px;
    line-height: 1;
    color: var(--text-secondary);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: border-color 0.1s, color 0.1s;
    flex-shrink: 0;
  }
  .zoom-btn:hover:not(:disabled) { border-color: var(--accent); color: var(--text-primary); }
  .zoom-btn:disabled { opacity: 0.35; cursor: default; }

  .zoom-label {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    min-width: 36px;
    text-align: center;
    flex: 1;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0;
  }
  .zoom-label:hover { color: var(--text-primary); }
</style>
