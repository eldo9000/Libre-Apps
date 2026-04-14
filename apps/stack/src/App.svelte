<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { WindowFrame, Titlebar } from '@libre/ui';
  import { EditorView, keymap, lineNumbers, drawSelection, highlightActiveLine, highlightActiveLineGutter } from '@codemirror/view';
  import { EditorState, Compartment } from '@codemirror/state';
  import { defaultKeymap, history, historyKeymap, indentWithTab } from '@codemirror/commands';
  import { markdown } from '@codemirror/lang-markdown';
  import { oneDark } from '@codemirror/theme-one-dark';
  import { searchKeymap, search } from '@codemirror/search';

  const appWindow = getCurrentWindow();

  // ── Tab model ──────────────────────────────────────────────────────────────
  let nextTabId = 1;

  function makeTab(path = null, title = 'Untitled', content = '') {
    return {
      id: nextTabId++,
      path,
      title,
      content,
      saved: true,
      wordCount: 0,
      charCount: 0,
      lineCount: 1,
      encoding: 'UTF-8',
    };
  }

  let tabs = $state([makeTab()]);
  let activeTabId = $state(tabs[0].id);
  let activeTab = $derived(tabs.find(t => t.id === activeTabId) ?? tabs[0]);

  // ── Panel state ─────────────────────────────────────────────────────────────
  let leftOpen = $state(true);
  let rightOpen = $state(true);
  let leftTab = $state('outline'); // 'outline' | 'minimap'
  let rightTab = $state('stats');  // 'stats' | 'format' | 'search'

  // ── Dark mode — follows OS theme via shared ~/.config/librewin/theme ─────────
  let isDark = $state(false);

  $effect(() => {
    document.documentElement.classList.toggle('dark', isDark);
  });

  // ── CodeMirror ──────────────────────────────────────────────────────────────
  // Non-reactive — CM instance lives outside Svelte reactivity
  let editorEl = null;
  let view = null;
  const themeCompartment = new Compartment();

  function buildState(content, dark) {
    return EditorState.create({
      doc: content,
      extensions: [
        lineNumbers(),
        highlightActiveLine(),
        highlightActiveLineGutter(),
        drawSelection(),
        history(),
        search({ top: false }),
        markdown(),
        themeCompartment.of(dark ? oneDark : EditorView.theme({}, { dark: false })),
        keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap, indentWithTab]),
        EditorView.updateListener.of((update) => {
          if (update.docChanged) {
            const tab = tabs.find(t => t.id === activeTabId);
            if (!tab) return;
            const doc = update.state.doc.toString();
            tab.content = doc;
            tab.saved = false;
            updateStats(tab, doc);
            updateOutline(doc);
          }
        }),
        EditorView.lineWrapping,
      ],
    });
  }

  function updateStats(tab, doc) {
    const lines = doc.split('\n');
    tab.lineCount = lines.length;
    tab.charCount = doc.length;
    const words = doc.trim() ? doc.trim().split(/\s+/).length : 0;
    tab.wordCount = words;
    tabs = tabs; // trigger reactivity
  }

  // ── Outline ─────────────────────────────────────────────────────────────────
  let outline = $state([]);

  function updateOutline(doc) {
    const lines = doc.split('\n');
    const items = [];
    for (const line of lines) {
      const m = line.match(/^(#{1,6})\s+(.+)/);
      if (m) {
        items.push({ level: m[1].length, text: m[2] });
      }
    }
    outline = items;
  }

  function jumpToHeading(text) {
    if (!view) return;
    const doc = view.state.doc.toString();
    const target = `# ${text}`;
    // Find any heading line matching this text
    const lines = doc.split('\n');
    let pos = 0;
    for (const line of lines) {
      const m = line.match(/^#{1,6}\s+(.+)/);
      if (m && m[1] === text) {
        view.dispatch({
          selection: { anchor: pos },
          scrollIntoView: true,
        });
        view.focus();
        break;
      }
      pos += line.length + 1;
    }
  }

  // ── Search panel (right panel ↔ CM search) ──────────────────────────────────
  let searchQuery = $state('');
  let replaceQuery = $state('');
  let searchCaseSensitive = $state(false);

  function doSearch() {
    if (!view || !searchQuery) return;
    // Open CM search panel programmatically
    import('@codemirror/search').then(({ openSearchPanel }) => {
      openSearchPanel(view);
    });
  }

  function doReplace() {
    if (!view || !searchQuery) return;
    import('@codemirror/search').then(({ replaceNext }) => {
      replaceNext(view);
    });
  }

  function doReplaceAll() {
    if (!view || !searchQuery) return;
    import('@codemirror/search').then(({ replaceAll }) => {
      replaceAll(view);
    });
  }

  // ── File ops ─────────────────────────────────────────────────────────────────
  async function newFile() {
    const tab = makeTab();
    tabs = [...tabs, tab];
    switchTab(tab.id);
  }

  async function openFile() {
    try {
      const result = await invoke('open_file_dialog');
      if (!result) return;
      const { path, content } = result;
      const filename = path.split('/').pop();
      // Check if already open
      const existing = tabs.find(t => t.path === path);
      if (existing) {
        switchTab(existing.id);
        return;
      }
      const tab = makeTab(path, filename, content);
      tab.saved = true;
      updateStats(tab, content);
      tabs = [...tabs, tab];
      switchTab(tab.id);
    } catch (e) {
      console.error('Open failed:', e);
    }
  }

  async function saveFile() {
    const tab = activeTab;
    if (!tab) return;
    const content = view ? view.state.doc.toString() : tab.content;
    try {
      if (tab.path) {
        await invoke('write_file', { path: tab.path, content });
        tab.saved = true;
        tabs = tabs;
      } else {
        await saveFileAs();
      }
    } catch (e) {
      console.error('Save failed:', e);
    }
  }

  async function saveFileAs() {
    const tab = activeTab;
    if (!tab) return;
    const content = view ? view.state.doc.toString() : tab.content;
    try {
      const path = await invoke('save_file_dialog', {
        defaultName: tab.title.endsWith('.md') ? tab.title : `${tab.title}.md`,
      });
      if (!path) return;
      await invoke('write_file', { path, content });
      tab.path = path;
      tab.title = path.split('/').pop();
      tab.saved = true;
      tabs = tabs;
    } catch (e) {
      console.error('Save as failed:', e);
    }
  }

  async function exportHtml() {
    const tab = activeTab;
    if (!tab) return;
    const content = view ? view.state.doc.toString() : tab.content;
    try {
      const html = await invoke('export_html', { content });
      const defaultName = (tab.title.replace(/\.md$/, '') || 'document') + '.html';
      const path = await invoke('save_file_dialog', { defaultName });
      if (!path) return;
      await invoke('write_file', { path, content: html });
    } catch (e) {
      console.error('Export failed:', e);
    }
  }

  function exportTxt() {
    const tab = activeTab;
    if (!tab) return;
    const content = view ? view.state.doc.toString() : tab.content;
    // Strip markdown syntax roughly
    const plain = content
      .replace(/^#{1,6}\s+/gm, '')
      .replace(/\*\*(.+?)\*\*/g, '$1')
      .replace(/\*(.+?)\*/g, '$1')
      .replace(/`(.+?)`/g, '$1')
      .replace(/\[(.+?)\]\(.+?\)/g, '$1');
    invoke('save_file_dialog', { defaultName: (tab.title.replace(/\.md$/, '') || 'document') + '.txt' })
      .then(path => path && invoke('write_file', { path, content: plain }))
      .catch(console.error);
  }

  function printToPdf() {
    window.print();
  }

  // ── Tab switching ─────────────────────────────────────────────────────────
  function switchTab(id) {
    if (activeTabId === id) return;
    // Save current CM content to outgoing tab
    if (view && activeTab) {
      activeTab.content = view.state.doc.toString();
    }
    activeTabId = id;
    // Restore content to CM on next tick
    const tab = tabs.find(t => t.id === id);
    if (view && tab) {
      view.setState(buildState(tab.content, isDark));
      updateOutline(tab.content);
    }
  }

  function closeTab(id) {
    const idx = tabs.findIndex(t => t.id === id);
    if (idx < 0) return;
    const tab = tabs[idx];
    if (!tab.saved) {
      const ok = confirm(`"${tab.title}" has unsaved changes. Close anyway?`);
      if (!ok) return;
    }
    const newTabs = tabs.filter(t => t.id !== id);
    if (newTabs.length === 0) {
      tabs = [makeTab()];
      activeTabId = tabs[0].id;
      if (view) view.setState(buildState('', isDark));
      outline = [];
      return;
    }
    tabs = newTabs;
    if (activeTabId === id) {
      const newActive = newTabs[Math.min(idx, newTabs.length - 1)];
      activeTabId = newActive.id;
      if (view) {
        view.setState(buildState(newActive.content, isDark));
        updateOutline(newActive.content);
      }
    }
  }

  // ── Keyboard shortcuts ────────────────────────────────────────────────────
  function handleKeydown(e) {
    const mod = e.ctrlKey || e.metaKey;
    if (!mod) return;
    if (e.key === 'n') { e.preventDefault(); newFile(); }
    else if (e.key === 'o') { e.preventDefault(); openFile(); }
    else if (e.key === 's' && !e.shiftKey) { e.preventDefault(); saveFile(); }
    else if (e.key === 's' && e.shiftKey) { e.preventDefault(); saveFileAs(); }
    else if (e.key === 'w') { e.preventDefault(); closeTab(activeTabId); }
    else if (e.key === '\\') { e.preventDefault(); leftOpen = !leftOpen; }
    else if (e.key === '/') { e.preventDefault(); rightOpen = !rightOpen; }
  }

  // ── Window chrome ──────────────────────────────────────────────────────────
  // minimize / toggleMax handled by <Titlebar> — only custom close is needed here.
  async function closeWindow() {
    const unsaved = tabs.filter(t => !t.saved);
    if (unsaved.length > 0) {
      const ok = confirm(`${unsaved.length} file(s) have unsaved changes. Close anyway?`);
      if (!ok) return;
    }
    await appWindow.close();
  }

  // ── Lifecycle ──────────────────────────────────────────────────────────────
  onMount(async () => {
    // theme + accent CSS vars are handled by WindowFrame via initTheme.
    // We still need isDark state here to drive CodeMirror's theme compartment.
    const theme = await invoke('get_theme');
    const mq = window.matchMedia('(prefers-color-scheme: dark)');
    isDark = theme === 'dark' || (theme === 'system' && mq.matches);

    // Live update: keep isDark in sync for CodeMirror when OS scheme changes.
    const mqHandler = async (e) => {
      const t = await invoke('get_theme');
      if (t === 'system') isDark = e.matches;
    };
    mq.addEventListener('change', mqHandler);

    const tab = activeTab;
    view = new EditorView({
      state: buildState(tab.content, isDark),
      parent: editorEl,
    });
    updateOutline(tab.content);

    return () => mq.removeEventListener('change', mqHandler);
  });

  onDestroy(() => {
    view?.destroy();
  });

  // Sync dark mode to CM theme
  $effect(() => {
    if (!view) return;
    view.dispatch({
      effects: themeCompartment.reconfigure(isDark ? oneDark : EditorView.theme({}, { dark: false })),
    });
  });

  // ── Drag resize for panels ─────────────────────────────────────────────────
  let leftWidth = $state(220);
  let rightWidth = $state(260);

  function startResizeLeft(e) {
    e.preventDefault();
    const startX = e.clientX;
    const startW = leftWidth;
    const move = (ev) => {
      leftWidth = Math.max(160, Math.min(400, startW + ev.clientX - startX));
    };
    const up = () => {
      window.removeEventListener('mousemove', move);
      window.removeEventListener('mouseup', up);
    };
    window.addEventListener('mousemove', move);
    window.addEventListener('mouseup', up);
  }

  function startResizeRight(e) {
    e.preventDefault();
    const startX = e.clientX;
    const startW = rightWidth;
    const move = (ev) => {
      rightWidth = Math.max(200, Math.min(480, startW - (ev.clientX - startX)));
    };
    const up = () => {
      window.removeEventListener('mousemove', move);
      window.removeEventListener('mouseup', up);
    };
    window.addEventListener('mousemove', move);
    window.addEventListener('mouseup', up);
  }

  // ── Derived stats ──────────────────────────────────────────────────────────
  let stats = $derived({
    words: activeTab?.wordCount ?? 0,
    chars: activeTab?.charCount ?? 0,
    lines: activeTab?.lineCount ?? 1,
    encoding: activeTab?.encoding ?? 'UTF-8',
    path: activeTab?.path ?? null,
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<!-- Root layout -->
<WindowFrame style="color: var(--text);">

  <!-- ── Titlebar ──────────────────────────────────────────────────────────── -->
  <Titlebar height="h-11" onclose={closeWindow}>
    <!-- Brand -->
    <div class="flex items-center gap-2 px-3 shrink-0">
      <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="var(--accent)" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="shrink-0">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
        <line x1="10" y1="9" x2="8" y2="9"/>
      </svg>
      <span class="text-sm font-medium" style="color: var(--text);">Stack</span>
    </div>

    <!-- Tab bar -->
    <div class="flex items-end gap-px flex-1 overflow-x-auto overflow-y-hidden px-1 min-w-0" style="height: 100%; padding-top: 6px;">
      {#each tabs as tab (tab.id)}
        <button
          onclick={() => switchTab(tab.id)}
          class="flex items-center gap-1.5 px-3 text-xs rounded-t-md shrink-0 border-l border-r border-t transition-colors"
          style="
            height: 32px;
            max-width: 180px;
            background: {tab.id === activeTabId ? 'var(--tab-active)' : 'var(--tab-inactive)'};
            border-color: {tab.id === activeTabId ? 'var(--border)' : 'transparent'};
            color: {tab.id === activeTabId ? 'var(--text)' : 'var(--text-2)'};
            font-weight: {tab.id === activeTabId ? '500' : '400'};
          "
        >
          <span class="truncate max-w-[120px]">{tab.title}</span>
          {#if !tab.saved}
            <span class="w-1.5 h-1.5 rounded-full shrink-0 unsaved-dot" style="background: var(--accent);"></span>
          {/if}
          <span
            class="ml-auto shrink-0 rounded px-0.5 hover:bg-red-100 hover:text-red-600 text-xs leading-none"
            style="color: var(--text-3);"
            role="button"
            tabindex="0"
            aria-label="Close tab"
            onclick={(e) => { e.stopPropagation(); closeTab(tab.id); }}
            onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); closeTab(tab.id); } }}
          >×</span>
        </button>
      {/each}

      <!-- New tab button -->
      <button
        onclick={newFile}
        class="flex items-center justify-center w-7 h-7 rounded text-lg shrink-0 transition-colors"
        style="color: var(--text-3);"
        title="New file (Ctrl+N)"
        aria-label="New file"
      >+</button>
    </div>

    <!-- Menu actions -->
    <div class="flex items-center gap-0.5 px-2 shrink-0">
      <button onclick={openFile} class="titlebar-btn" title="Open (Ctrl+O)" aria-label="Open file">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
        </svg>
      </button>
      <button onclick={saveFile} class="titlebar-btn" title="Save (Ctrl+S)" aria-label="Save file">
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <path d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"/>
          <polyline points="17 21 17 13 7 13 7 21"/>
          <polyline points="7 3 7 8 15 8"/>
        </svg>
      </button>

      <!-- Export dropdown -->
      <div class="relative group">
        <button class="titlebar-btn" title="Export" aria-label="Export">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="7 10 12 15 17 10"/>
            <line x1="12" y1="15" x2="12" y2="3"/>
          </svg>
        </button>
        <div class="absolute right-0 top-full mt-1 rounded-lg shadow-lg border overflow-hidden z-50 hidden group-focus-within:block group-hover:block"
          style="background: var(--surface); border-color: var(--border); min-width: 140px;">
          <button onclick={saveFileAs} class="export-item w-full text-left">Save As (.md)</button>
          <button onclick={exportHtml} class="export-item w-full text-left">Export HTML</button>
          <button onclick={exportTxt} class="export-item w-full text-left">Export TXT</button>
          <button onclick={printToPdf} class="export-item w-full text-left">Print / PDF</button>
        </div>
      </div>

      <!-- Toggle left panel -->
      <button
        onclick={() => leftOpen = !leftOpen}
        class="titlebar-btn"
        title="Toggle outline (Ctrl+\)"
        aria-label="Toggle outline panel"
        style="color: {leftOpen ? 'var(--accent)' : 'var(--text-2)'};"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <line x1="9" y1="3" x2="9" y2="21"/>
        </svg>
      </button>

      <!-- Toggle right panel -->
      <button
        onclick={() => rightOpen = !rightOpen}
        class="titlebar-btn"
        title="Toggle info panel (Ctrl+/)"
        aria-label="Toggle info panel"
        style="color: {rightOpen ? 'var(--accent)' : 'var(--text-2)'};"
      >
        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round">
          <rect x="3" y="3" width="18" height="18" rx="2"/>
          <line x1="15" y1="3" x2="15" y2="21"/>
        </svg>
      </button>
    </div>

  </Titlebar>

  <!-- ── Main area: panels + editor ────────────────────────────────────────── -->
  <div class="flex flex-1 overflow-hidden" role="main">

    <!-- Left panel: Outline / Minimap -->
    {#if leftOpen}
      <div
        class="flex flex-col shrink-0 overflow-hidden"
        style="width: {leftWidth}px; background: var(--panel-bg); border-right: 1px solid var(--panel-border);"
      >
        <!-- Panel tabs -->
        <div class="flex items-center gap-0 px-2 pt-1.5 shrink-0" style="border-bottom: 1px solid var(--panel-border);">
          <button
            onclick={() => leftTab = 'outline'}
            class="panel-tab"
            style="color: {leftTab === 'outline' ? 'var(--accent)' : 'var(--text-2)'}; border-bottom: 2px solid {leftTab === 'outline' ? 'var(--accent)' : 'transparent'};"
          >Outline</button>
          <button
            onclick={() => leftTab = 'minimap'}
            class="panel-tab"
            style="color: {leftTab === 'minimap' ? 'var(--accent)' : 'var(--text-2)'}; border-bottom: 2px solid {leftTab === 'minimap' ? 'var(--accent)' : 'transparent'};"
          >Minimap</button>
        </div>

        {#if leftTab === 'outline'}
          <!-- Outline -->
          <div class="flex-1 overflow-y-auto py-1">
            {#if outline.length === 0}
              <p class="text-xs px-3 py-2" style="color: var(--text-3);">No headings yet</p>
            {/if}
            {#each outline as item}
              <button
                onclick={() => jumpToHeading(item.text)}
                class="w-full text-left px-3 py-1 text-xs truncate transition-colors hover:opacity-80"
                style="
                  padding-left: {8 + (item.level - 1) * 12}px;
                  color: {item.level === 1 ? 'var(--accent)' : item.level === 2 ? 'var(--text)' : 'var(--text-2)'};
                  font-weight: {item.level <= 2 ? '500' : '400'};
                "
                title={item.text}
              >
                <span class="mr-1 opacity-40" style="font-size: 9px;">{'#'.repeat(item.level)}</span>{item.text}
              </button>
            {/each}
          </div>
        {:else}
          <!-- Minimap placeholder -->
          <div class="flex-1 overflow-hidden relative">
            <div class="absolute inset-0 px-2 py-2 overflow-hidden pointer-events-none" aria-hidden="true">
              <!-- Render a tiny uneditable copy of the doc as a visual minimap -->
              <pre class="text-[3px] leading-[4px] whitespace-pre-wrap break-all select-none opacity-60"
                style="color: var(--text); font-family: monospace;">
                {activeTab?.content ?? ''}
              </pre>
            </div>
          </div>
        {/if}
      </div>

      <!-- Left resize handle -->
      <div class="resize-handle" onmousedown={startResizeLeft} role="separator" aria-orientation="vertical" tabindex="0"></div>
    {/if}

    <!-- Editor -->
    <div class="flex-1 overflow-hidden" bind:this={editorEl}></div>

    <!-- Right resize handle -->
    {#if rightOpen}
      <div class="resize-handle" onmousedown={startResizeRight} role="separator" aria-orientation="vertical" tabindex="0"></div>

      <!-- Right panel: Stats / Format / Search -->
      <div
        class="flex flex-col shrink-0 overflow-hidden"
        style="width: {rightWidth}px; background: var(--panel-bg); border-left: 1px solid var(--panel-border);"
      >
        <!-- Panel tabs -->
        <div class="flex items-center gap-0 px-2 pt-1.5 shrink-0" style="border-bottom: 1px solid var(--panel-border);">
          <button onclick={() => rightTab = 'stats'} class="panel-tab"
            style="color: {rightTab === 'stats' ? 'var(--accent)' : 'var(--text-2)'}; border-bottom: 2px solid {rightTab === 'stats' ? 'var(--accent)' : 'transparent'};">Stats</button>
          <button onclick={() => rightTab = 'format'} class="panel-tab"
            style="color: {rightTab === 'format' ? 'var(--accent)' : 'var(--text-2)'}; border-bottom: 2px solid {rightTab === 'format' ? 'var(--accent)' : 'transparent'};">Format</button>
          <button onclick={() => rightTab = 'search'} class="panel-tab"
            style="color: {rightTab === 'search' ? 'var(--accent)' : 'var(--text-2)'}; border-bottom: 2px solid {rightTab === 'search' ? 'var(--accent)' : 'transparent'};">Search</button>
        </div>

        {#if rightTab === 'stats'}
          <!-- Stats -->
          <div class="flex-1 overflow-y-auto p-4 space-y-3">
            <div class="stat-row">
              <span class="stat-label">Words</span>
              <span class="stat-value">{stats.words.toLocaleString()}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">Characters</span>
              <span class="stat-value">{stats.chars.toLocaleString()}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">Lines</span>
              <span class="stat-value">{stats.lines.toLocaleString()}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">Encoding</span>
              <span class="stat-value">{stats.encoding}</span>
            </div>
            <div class="stat-row">
              <span class="stat-label">Format</span>
              <span class="stat-value">Markdown</span>
            </div>
            {#if stats.path}
              <div class="pt-2" style="border-top: 1px solid var(--panel-border);">
                <p class="text-xs" style="color: var(--text-3);">File</p>
                <p class="text-xs mt-0.5 break-all" style="color: var(--text-2);">{stats.path}</p>
              </div>
            {/if}
          </div>

        {:else if rightTab === 'format'}
          <!-- Format / Encoding -->
          <div class="flex-1 overflow-y-auto p-4 space-y-4">
            <div>
              <p class="text-xs font-medium mb-2" style="color: var(--text-2);">Line endings</p>
              <div class="space-y-1">
                {#each ['LF (Unix)', 'CRLF (Windows)', 'CR (Classic Mac)'] as le}
                  <label class="flex items-center gap-2 text-xs cursor-pointer">
                    <input type="radio" name="le" value={le} checked={le === 'LF (Unix)'} class="accent-[var(--accent)]" />
                    <span style="color: var(--text);">{le}</span>
                  </label>
                {/each}
              </div>
            </div>
            <div>
              <p class="text-xs font-medium mb-2" style="color: var(--text-2);">Encoding</p>
              <select class="w-full text-xs rounded px-2 py-1.5 border" style="background: var(--surface); color: var(--text); border-color: var(--border);">
                <option>UTF-8</option>
                <option>UTF-16</option>
                <option>ISO-8859-1</option>
              </select>
            </div>
            <div>
              <p class="text-xs font-medium mb-2" style="color: var(--text-2);">Tab size</p>
              <select class="w-full text-xs rounded px-2 py-1.5 border" style="background: var(--surface); color: var(--text); border-color: var(--border);">
                <option>2 spaces</option>
                <option>4 spaces</option>
                <option>Tab character</option>
              </select>
            </div>
          </div>

        {:else if rightTab === 'search'}
          <!-- Search & Replace -->
          <div class="flex-1 overflow-y-auto p-3 space-y-3">
            <div class="space-y-1">
              <label class="text-xs font-medium" style="color: var(--text-2);">Find</label>
              <input
                bind:value={searchQuery}
                onkeydown={(e) => e.key === 'Enter' && doSearch()}
                placeholder="Search…"
                class="w-full text-xs rounded px-2 py-1.5 border"
                style="background: var(--surface); color: var(--text); border-color: var(--border);"
              />
            </div>
            <div class="space-y-1">
              <label class="text-xs font-medium" style="color: var(--text-2);">Replace</label>
              <input
                bind:value={replaceQuery}
                placeholder="Replace with…"
                class="w-full text-xs rounded px-2 py-1.5 border"
                style="background: var(--surface); color: var(--text); border-color: var(--border);"
              />
            </div>
            <label class="flex items-center gap-2 text-xs cursor-pointer">
              <input type="checkbox" bind:checked={searchCaseSensitive} class="accent-[var(--accent)]" />
              <span style="color: var(--text);">Case sensitive</span>
            </label>
            <div class="flex gap-2 pt-1">
              <button onclick={doSearch} class="action-btn flex-1">Find</button>
              <button onclick={doReplace} class="action-btn flex-1">Replace</button>
            </div>
            <button onclick={doReplaceAll} class="action-btn w-full">Replace All</button>
            <p class="text-xs" style="color: var(--text-3);">Tip: use Ctrl+H in the editor for the built-in find/replace.</p>
          </div>
        {/if}
      </div>
    {/if}
  </div>

  <!-- ── Status bar ──────────────────────────────────────────────────────────── -->
  <div class="flex items-center justify-between px-3 shrink-0 text-xs" style="height: 24px; background: var(--titlebar-bg); border-top: 1px solid var(--titlebar-border); color: var(--text-3);">
    <span>{activeTab?.path ? activeTab.path.split('/').pop() : 'Untitled'}{activeTab?.saved === false ? ' •' : ''}</span>
    <span>{stats.words} words · {stats.lines} lines · {stats.encoding}</span>
  </div>
</WindowFrame>

<style>
  :global(.titlebar-btn) {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: none;
    cursor: pointer;
    border-radius: 6px;
    color: var(--text-2);
    transition: background 120ms, color 120ms;
  }
  :global(.titlebar-btn:hover) {
    background: var(--surface-3);
    color: var(--text);
  }
  :global(.panel-tab) {
    padding: 4px 10px 6px;
    font-size: 11px;
    font-weight: 500;
    background: none;
    border: none;
    border-bottom: 2px solid transparent;
    cursor: pointer;
    transition: color 120ms;
  }
  :global(.panel-tab:hover) { opacity: 0.8; }

  :global(.stat-row) {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  :global(.stat-label) {
    font-size: 11px;
    color: var(--text-2);
  }
  :global(.stat-value) {
    font-size: 12px;
    font-weight: 500;
    color: var(--text);
    font-variant-numeric: tabular-nums;
  }
  :global(.action-btn) {
    padding: 5px 10px;
    font-size: 11px;
    font-weight: 500;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    transition: opacity 120ms;
  }
  :global(.action-btn:hover) { opacity: 0.88; }
  :global(.export-item) {
    display: block;
    padding: 7px 12px;
    font-size: 12px;
    color: var(--text);
    background: none;
    border: none;
    cursor: pointer;
    transition: background 120ms;
  }
  :global(.export-item:hover) { background: var(--surface-3); }
</style>
