<script>
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  // ── Tab factory ────────────────────────────────────────────────────────────
  function makeTab(path = '') {
    return {
      id: Math.random().toString(36).slice(2),
      path,
      history: [path],
      historyIdx: 0,
      items: [],
      selected: null,
      selectedItem: null,
      selectedIndex: -1,
      search: '',
      tagFilter: null,
      tagInput: '',
      loading: false,
      error: null,
    };
  }

  // ── Pane factory ───────────────────────────────────────────────────────────
  function makePaneState() {
    return { tabs: [makeTab()], activeTabIdx: 0 };
  }

  // ── Global state ───────────────────────────────────────────────────────────
  let homeDir = '';
  let volumes = [];
  let sidebarTab = 'quick';
  export let dualPane = false;
  let panes = [makePaneState(), makePaneState()];
  let activePaneIdx = 0;

  // Folder tree (shared across panes)
  let treeRoots = [];
  let treeExpanded = new Set();
  let treeChildren = {};

  // Tag save state
  let tagSaving = false;
  let tagError = null;

  // File list element refs per pane
  let fileListEls = [];

  // Context menu
  let contextMenu = null;
  let winepathFound = false;
  let wineprefixReady = false;

  // Custom Splice presets
  let customPresets = [];

  // Toast
  let toast = null;
  let toastTimer = null;

  // ── Active pane/tab shorthands ─────────────────────────────────────────────
  $: ap = panes[activePaneIdx];
  $: t = ap.tabs[ap.activeTabIdx];

  function updateTab(updates) {
    panes[activePaneIdx].tabs[panes[activePaneIdx].activeTabIdx] = {
      ...panes[activePaneIdx].tabs[panes[activePaneIdx].activeTabIdx],
      ...updates,
    };
    panes = panes;
  }

  // ── Tab operations ─────────────────────────────────────────────────────────
  function newTab() {
    const tab = makeTab(homeDir);
    panes[activePaneIdx].tabs = [...panes[activePaneIdx].tabs, tab];
    panes[activePaneIdx].activeTabIdx = panes[activePaneIdx].tabs.length - 1;
    panes = panes;
    if (homeDir) navigateTo(homeDir);
  }

  function closeTab(i, e) {
    e?.stopPropagation();
    const p = panes[activePaneIdx];
    if (p.tabs.length === 1) return;
    p.tabs = p.tabs.filter((_, idx) => idx !== i);
    if (p.activeTabIdx >= p.tabs.length) p.activeTabIdx = p.tabs.length - 1;
    panes = panes;
  }

  function switchPaneTab(pi, i) {
    panes[pi].activeTabIdx = i;
    panes = panes;
    activePaneIdx = pi;
    setTimeout(() => fileListEls[pi]?.focus(), 0);
  }

  function tabLabel(tab) {
    if (!tab.path) return 'New Tab';
    return tab.path.split('/').filter(Boolean).pop() || '/';
  }

  // ── Dual-pane toggle ───────────────────────────────────────────────────────
  export async function toggleDualPane() {
    dualPane = !dualPane;
    if (dualPane && !panes[1].tabs[panes[1].activeTabIdx].path && homeDir) {
      const prev = activePaneIdx;
      activePaneIdx = 1;
      await navigateTo(homeDir);
      activePaneIdx = prev;
    }
  }

  // ── Dir cache + IPC ────────────────────────────────────────────────────────
  const dirCache = new Map();

  async function loadDir(path) {
    if (!path) return [];
    if (dirCache.has(path)) return dirCache.get(path);
    try {
      const entries = await invoke('list_dir', { path });
      dirCache.set(path, entries);
      return entries;
    } catch (e) {
      console.error('list_dir error:', path, e);
      return [];
    }
  }

  onMount(async () => {
    try {
      homeDir = await invoke('get_home_dir');
      volumes = await invoke('get_volumes');
      const wineStatus = await invoke('get_wine_status');
      winepathFound = wineStatus.winepath_available;
      wineprefixReady = wineStatus.wineprefix_initialized;
      treeRoots = (await loadDir(homeDir)).filter(e => e.is_dir);
      await navigateTo(homeDir);
    } catch (e) {
      updateTab({ error: String(e) });
    }

    // Load custom Splice presets (best-effort — Splice may not be installed yet)
    try { customPresets = await invoke('list_splice_presets'); } catch (_) {}

    // Quick Convert result notifications
    quickConvertUnlisten = await listen('quick-convert-done', ({ payload }) => {
      const name = payload.output.split('/').pop();
      showToast(`Converted → ${name}`);
      refreshDir();
    });
    await listen('quick-convert-error', ({ payload }) => {
      showToast(`Convert failed: ${payload.message}`);
    });
  });

  onDestroy(() => { quickConvertUnlisten?.(); });

  // ── Navigation ────────────────────────────────────────────────────────────
  async function navigateTo(path) {
    if (!path) return;
    const pi = activePaneIdx;
    const p = panes[pi];
    const tab = p.tabs[p.activeTabIdx];
    const newHistory = [...tab.history.slice(0, tab.historyIdx + 1), path];
    panes[pi].tabs[p.activeTabIdx] = {
      ...tab,
      path,
      history: newHistory,
      historyIdx: newHistory.length - 1,
      selected: null, selectedItem: null, selectedIndex: -1,
      search: '', tagFilter: null, tagInput: '',
      loading: true, error: null,
    };
    panes = panes;
    const items = await loadDir(path);
    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, loading: false };
    panes = panes;
    expandAncestors(path);
  }

  async function goBack() {
    const pi = activePaneIdx;
    const p = panes[pi];
    const tab = p.tabs[p.activeTabIdx];
    if (tab.historyIdx <= 0) return;
    const newIdx = tab.historyIdx - 1;
    const path = tab.history[newIdx];
    panes[pi].tabs[p.activeTabIdx] = { ...tab, historyIdx: newIdx, path,
      selected: null, selectedItem: null, selectedIndex: -1,
      search: '', tagFilter: null, loading: true };
    panes = panes;
    const items = await loadDir(path);
    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, loading: false };
    panes = panes;
  }

  async function goForward() {
    const pi = activePaneIdx;
    const p = panes[pi];
    const tab = p.tabs[p.activeTabIdx];
    if (tab.historyIdx >= tab.history.length - 1) return;
    const newIdx = tab.historyIdx + 1;
    const path = tab.history[newIdx];
    panes[pi].tabs[p.activeTabIdx] = { ...tab, historyIdx: newIdx, path,
      selected: null, selectedItem: null, selectedIndex: -1,
      search: '', tagFilter: null, loading: true };
    panes = panes;
    const items = await loadDir(path);
    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, loading: false };
    panes = panes;
  }

  async function refreshDir() {
    const pi = activePaneIdx;
    const p = panes[pi];
    const tab = p.tabs[p.activeTabIdx];
    dirCache.delete(tab.path);
    panes[pi].tabs[p.activeTabIdx] = { ...tab, loading: true };
    panes = panes;
    const items = await loadDir(tab.path);
    const selectedItem = items.find(i => i.name === tab.selected) ?? null;
    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, selectedItem, loading: false };
    panes = panes;
  }

  // ── Quick Access ──────────────────────────────────────────────────────────
  $: quickAccessGroups = [
    { group: 'LOCAL', items: [
      { name: 'Home',      path: homeDir,                icon: 'home' },
      { name: 'Desktop',   path: `${homeDir}/Desktop`,   icon: 'desktop' },
      { name: 'Documents', path: `${homeDir}/Documents`, icon: 'folder' },
      { name: 'Downloads', path: `${homeDir}/Downloads`, icon: 'download' },
      { name: 'Pictures',  path: `${homeDir}/Pictures`,  icon: 'image' },
      { name: 'Music',     path: `${homeDir}/Music`,     icon: 'music' },
      { name: 'Videos',    path: `${homeDir}/Videos`,    icon: 'video' },
    ]},
    { group: 'DEVICES', items: volumes.map(v => ({ name: v.name, path: v.path, icon: 'drive' })) },
  ];

  // ── Folder tree ───────────────────────────────────────────────────────────
  function expandAncestors(path) {
    if (!homeDir || !path.startsWith(homeDir)) return;
    const rel = path.slice(homeDir.length);
    const next = new Set(treeExpanded);
    let acc = homeDir;
    for (const seg of rel.split('/').filter(Boolean)) { acc += '/' + seg; next.add(acc); }
    treeExpanded = next;
  }

  async function toggleTreeExpand(node) {
    const next = new Set(treeExpanded);
    if (next.has(node.path)) {
      next.delete(node.path);
    } else {
      next.add(node.path);
      if (!treeChildren[node.path]) {
        const children = await loadDir(node.path);
        treeChildren[node.path] = children.filter(e => e.is_dir);
        treeChildren = { ...treeChildren };
      }
    }
    treeExpanded = next;
  }

  function flattenTree(roots, depth = 0) {
    const rows = [];
    for (const node of roots) {
      rows.push({ ...node, depth });
      if (treeExpanded.has(node.path)) rows.push(...flattenTree(treeChildren[node.path] ?? [], depth + 1));
    }
    return rows;
  }

  $: treeRows = flattenTree(treeRoots);

  // ── Tags in current dir (active pane) ─────────────────────────────────────
  $: defaultTagCounts = Object.fromEntries(
    DEFAULT_TAGS.map(dt => [dt.name, t.items.filter(i => (i.tags || []).includes(dt.name)).length])
  );

  $: allDirTags = (() => {
    const counts = {};
    for (const item of t.items) {
      for (const tag of (item.tags || [])) {
        counts[tag] = (counts[tag] || 0) + 1;
      }
    }
    return Object.entries(counts).sort((a, b) => b[1] - a[1]).map(([tag, count]) => ({ tag, count }));
  })();

  const TAG_COLORS = [
    'bg-blue-500','bg-purple-500','bg-green-500','bg-yellow-500',
    'bg-red-500','bg-pink-500','bg-indigo-500','bg-teal-500',
    'bg-orange-500','bg-cyan-500',
  ];

  const DEFAULT_TAGS = [
    { name: 'red',    color: 'bg-red-400'     },
    { name: 'yellow', color: 'bg-amber-300'   },
    { name: 'green',  color: 'bg-emerald-400' },
    { name: 'blue',   color: 'bg-sky-400'     },
    { name: 'purple', color: 'bg-violet-400'  },
    { name: 'gray',   color: 'bg-gray-400'    },
  ];
  const TAG_COLOR_OVERRIDES = Object.fromEntries(DEFAULT_TAGS.map(dt => [dt.name, dt.color]));

  function tagColor(tag) {
    if (TAG_COLOR_OVERRIDES[tag]) return TAG_COLOR_OVERRIDES[tag];
    let h = 0;
    for (let i = 0; i < tag.length; i++) h = (h * 31 + tag.charCodeAt(i)) >>> 0;
    return TAG_COLORS[h % TAG_COLORS.length];
  }

  // ── Tag operations ────────────────────────────────────────────────────────
  async function addTag() {
    if (!t.selectedItem || !t.tagInput.trim()) return;
    const newTag = t.tagInput.trim().toLowerCase().replace(/\s+/g, '-');
    const existing = t.selectedItem.tags || [];
    if (existing.includes(newTag)) { updateTab({ tagInput: '' }); return; }
    tagSaving = true; tagError = null;
    try {
      await invoke('set_tags', { path: t.selectedItem.path, tags: [...existing, newTag] });
      updateTab({ tagInput: '' });
      await refreshDir();
    } catch (e) { tagError = String(e); }
    tagSaving = false;
  }

  async function removeTag(tag) {
    if (!t.selectedItem) return;
    tagSaving = true; tagError = null;
    try {
      await invoke('set_tags', { path: t.selectedItem.path, tags: (t.selectedItem.tags || []).filter(x => x !== tag) });
      await refreshDir();
    } catch (e) { tagError = String(e); }
    tagSaving = false;
  }

  // ── Context menu ──────────────────────────────────────────────────────────
  function showToast(msg) {
    clearTimeout(toastTimer);
    toast = msg;
    toastTimer = setTimeout(() => { toast = null; }, 3000);
  }

  // Determine which Quick Convert presets apply to a file extension
  function quickConvertPresets(ext) {
    const e = (ext ?? '').toLowerCase();
    const images = ['jpg','jpeg','png','webp','tiff','tif','bmp','gif','avif','heic','heif','psd','raw','cr2','nef','arw','dng'];
    const videos = ['mp4','mkv','webm','avi','mov','m4v','flv','wmv','ts','mpg','mpeg'];
    const audios = ['mp3','wav','flac','ogg','aac','opus','m4a','wma','aiff'];
    if (images.includes(e)) return [
      { preset: 'image_jpeg', label: '→ JPEG' },
      { preset: 'image_png',  label: '→ PNG' },
      { preset: 'image_webp', label: '→ WebP' },
    ];
    if (videos.includes(e)) return [
      { preset: 'video_mp4',  label: '→ MP4 (H.264)' },
      { preset: 'audio_mp3',  label: '→ MP3 (extract audio)' },
    ];
    if (audios.includes(e)) return [
      { preset: 'audio_mp3',  label: '→ MP3' },
      { preset: 'audio_flac', label: '→ FLAC (lossless)' },
    ];
    return [];
  }

  let quickConvertUnlisten = null;

  function onContextMenu(e, item) {
    e.preventDefault();
    if (item.is_dir) return; // no context menu for folders
    contextMenu = { x: e.clientX, y: e.clientY, item };
  }

  function hideContextMenu() { contextMenu = null; }

  async function copyWindowsPath() {
    if (!contextMenu) return;
    const item = contextMenu.item;
    hideContextMenu();
    if (!wineprefixReady) { showToast('Wine is still setting up, try again in a moment'); return; }
    try {
      const winPath = await invoke('get_windows_path', { path: item.path });
      await navigator.clipboard.writeText(winPath);
      showToast('Windows path copied');
    } catch (e) {
      showToast(e === 'wine_not_initialized' ? 'Wine is still setting up, try again in a moment' : 'Could not get Windows path');
    }
  }

  async function runQuickConvert(item, preset) {
    hideContextMenu();
    showToast('Converting…');
    try {
      await invoke('quick_convert', { path: item.path, preset });
    } catch (e) {
      showToast(`Convert failed: ${e}`);
    }
  }

  async function runSplicePreset(item, presetId) {
    hideContextMenu();
    showToast('Converting…');
    try {
      await invoke('run_splice_preset', { path: item.path, presetId });
    } catch (e) {
      showToast(`Convert failed: ${e}`);
    }
  }

  // Custom presets that match a given file extension
  function customPresetsFor(ext) {
    const e = (ext ?? '').toLowerCase();
    const images = ['jpg','jpeg','png','webp','tiff','tif','bmp','gif','avif','heic','heif','psd','raw','cr2','nef','arw','dng'];
    const videos = ['mp4','mkv','webm','avi','mov','m4v','flv','wmv','ts','mpg','mpeg'];
    const audios = ['mp3','wav','flac','ogg','aac','opus','m4a','wma','aiff'];
    let mediaType = null;
    if (images.includes(e)) mediaType = 'image';
    else if (videos.includes(e)) mediaType = 'video';
    else if (audios.includes(e)) mediaType = 'audio';
    if (!mediaType) return [];
    return customPresets.filter(p => p.media_type === mediaType);
  }

  // ── Helpers ───────────────────────────────────────────────────────────────
  function formatSize(bytes) {
    if (bytes == null) return '—';
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1073741824) return `${(bytes / 1048576).toFixed(1)} MB`;
    return `${(bytes / 1073741824).toFixed(1)} GB`;
  }

  function formatDate(ts) {
    if (!ts) return '—';
    return new Date(ts * 1000).toLocaleDateString('en-US', { month: 'short', day: '2-digit', year: 'numeric' });
  }

  function kindLabel(item) {
    if (item.is_dir) return 'Folder';
    return item.extension?.toUpperCase() ?? 'File';
  }

  async function openItem(item) {
    if (item.is_dir) { await navigateTo(item.path); }
    else { try { await invoke('open_file', { path: item.path }); } catch (e) { console.error(e); } }
  }

  function getDisplayItems(tab) {
    let items = tab.items;
    if (tab.search.trim()) items = items.filter(e => e.name.toLowerCase().includes(tab.search.trim().toLowerCase()));
    if (tab.tagFilter) items = items.filter(e => (e.tags || []).includes(tab.tagFilter));
    return items;
  }

  function selectItem(item, pi) {
    const dispItems = getDisplayItems(panes[pi].tabs[panes[pi].activeTabIdx]);
    panes[pi].tabs[panes[pi].activeTabIdx] = {
      ...panes[pi].tabs[panes[pi].activeTabIdx],
      selected: item.name,
      selectedItem: item,
      selectedIndex: dispItems.indexOf(item),
      tagInput: '',
    };
    panes = panes;
    tagError = null;
  }

  function selectByIndex(pi, dispItems, idx) {
    if (idx < 0 || idx >= dispItems.length) return;
    const item = dispItems[idx];
    panes[pi].tabs[panes[pi].activeTabIdx] = {
      ...panes[pi].tabs[panes[pi].activeTabIdx],
      selected: item.name, selectedItem: item, selectedIndex: idx, tagInput: '',
    };
    panes = panes;
    tagError = null;
    fileListEls[pi]?.querySelector(`[data-idx="${idx}"]`)?.scrollIntoView({ block: 'nearest' });
  }

  // ── Keyboard navigation ────────────────────────────────────────────────────
  function handleKeydown(e, pi, dispItems) {
    if (e.target.tagName === 'INPUT') return;
    const tab = panes[pi].tabs[panes[pi].activeTabIdx];
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault();
        if (dispItems.length === 0) break;
        selectByIndex(pi, dispItems, tab.selectedIndex < dispItems.length - 1 ? tab.selectedIndex + 1 : 0);
        break;
      case 'ArrowUp':
        e.preventDefault();
        if (dispItems.length === 0) break;
        selectByIndex(pi, dispItems, tab.selectedIndex <= 0 ? dispItems.length - 1 : tab.selectedIndex - 1);
        break;
      case 'Enter':
        e.preventDefault();
        if (tab.selectedIndex >= 0 && tab.selectedIndex < dispItems.length) {
          activePaneIdx = pi;
          openItem(dispItems[tab.selectedIndex]);
        }
        break;
      case 'Backspace':
        e.preventDefault();
        if (tab.path && tab.path !== '/') {
          activePaneIdx = pi;
          const parent = tab.path.split('/').slice(0, -1).join('/') || '/';
          navigateTo(parent);
        }
        break;
    }
  }
</script>

<div class="flex flex-col h-full bg-white dark:bg-gray-900 text-base select-none">

  <!-- ── Main area (sidebar + pane(s)) ─────────────────────────────────────── -->
  <div class="flex flex-1 min-h-0">

    <!-- ── Sidebar ────────────────────────────────────────────────────────── -->
    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
    <div class="sidebar-nav w-48 bg-gray-50 dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col shrink-0 outline-none"
      tabindex="-1">

      <!-- Sidebar tabs -->
      <div class="flex border-b border-gray-200 dark:border-gray-700 px-1 pt-1.5 gap-0.5 shrink-0">
        {#each [['quick','Quick'],['folders','Folders'],['tags','Tags']] as [id, label]}
          <button on:click={() => sidebarTab = id}
            class="flex-1 text-[11px] py-1 rounded-t font-medium transition-colors
              {sidebarTab === id
                ? 'bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-200 border border-b-white dark:border-gray-700 dark:border-b-gray-900 -mb-px'
                : 'text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-400'}"
          >{label}</button>
        {/each}
      </div>

      <!-- Quick Access -->
      {#if sidebarTab === 'quick'}
        <div class="flex-1 overflow-y-auto py-1">
          {#each quickAccessGroups as group}
            <div class="w-full px-3 pt-3 pb-1 text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500">{group.group}</div>
            {#each group.items as item}
              {@const active = t.path === item.path}
              <button on:click={() => item.path && navigateTo(item.path)} disabled={!item.path}
                class="w-full flex items-center gap-2 px-4 py-1.5 transition-colors text-left disabled:opacity-40 text-sm
                  {active ? 'bg-[var(--accent)] text-white' : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700'}">
                <span class="shrink-0 {active ? 'text-white/75' : 'text-gray-400 dark:text-gray-500'}">
                  {#if item.icon === 'home'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M5 12l-2 0l9 -9l9 9l-2 0" /><path d="M5 12v7a2 2 0 0 0 2 2h10a2 2 0 0 0 2 -2v-7" /><path d="M9 21v-6a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v6" /></svg>
                  {:else if item.icon === 'desktop'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10" /><path d="M7 20h10" /><path d="M9 16v4" /><path d="M15 16v4" /></svg>
                  {:else if item.icon === 'download'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 17v2a2 2 0 0 0 2 2h12a2 2 0 0 0 2 -2v-2" /><path d="M7 11l5 5l5 -5" /><path d="M12 4l0 12" /></svg>
                  {:else if item.icon === 'image'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M15 8h.01" /><path d="M3 6a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12" /><path d="M3 16l5 -5c.928 -.893 2.072 -.893 3 0l5 5" /><path d="M14 14l1 -1c.928 -.893 2.072 -.893 3 0l3 3" /></svg>
                  {:else if item.icon === 'music'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M3 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /><path d="M13 17a3 3 0 1 0 6 0a3 3 0 0 0 -6 0" /><path d="M9 17v-13h10v13" /><path d="M9 8h10" /></svg>
                  {:else if item.icon === 'video'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M15 10l4.553 -2.276a1 1 0 0 1 1.447 .894v6.764a1 1 0 0 1 -1.447 .894l-4.553 -2.276v-4" /><path d="M3 8a2 2 0 0 1 2 -2h8a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2l0 -8" /></svg>
                  {:else if item.icon === 'drive'}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M4 6a8 3 0 1 0 16 0a8 3 0 1 0 -16 0" /><path d="M4 6v6a8 3 0 0 0 16 0v-6" /><path d="M4 12v6a8 3 0 0 0 16 0v-6" /></svg>
                  {:else}
                    <svg width="13" height="13" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"><path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" /></svg>
                  {/if}
                </span>
                <span class="text-sm truncate">{item.name}</span>
              </button>
            {/each}
          {/each}
        </div>

      <!-- Folders tree -->
      {:else if sidebarTab === 'folders'}
        <div class="flex-1 overflow-y-auto py-1">
          {#each treeRows as node}
            {@const active = t.path === node.path}
            <div class="flex items-center gap-0.5 pr-2 py-1 cursor-pointer transition-colors
              {active ? 'bg-[var(--accent)]' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}"
              style="padding-left: {8 + node.depth * 14}px;">
              <button on:click|stopPropagation={() => toggleTreeExpand(node)}
                class="w-4 h-4 flex items-center justify-center shrink-0 {active ? 'text-white/60' : 'text-gray-400 dark:text-gray-600'}">
                <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                  {#if treeExpanded.has(node.path)}<path d="M6 9l6 6 6-6"/>{:else}<path d="M9 18l6-6-6-6"/>{/if}
                </svg>
              </button>
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <!-- svelte-ignore a11y-no-static-element-interactions -->
              <div on:click={() => navigateTo(node.path)} class="flex items-center gap-1.5 flex-1 min-w-0">
                <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
                  class="{active ? 'text-white/75' : 'text-gray-400 dark:text-gray-500'} shrink-0">
                  <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" />
                </svg>
                <span class="text-sm truncate {active ? 'text-white font-medium' : 'text-gray-600 dark:text-gray-400'}">{node.name}</span>
              </div>
            </div>
          {/each}
        </div>

      <!-- Tags tab -->
      {:else}
        <div class="flex-1 overflow-y-auto py-2 px-2">
          {#if t.tagFilter}
            <button on:click={() => updateTab({ tagFilter: null })}
              class="w-full flex items-center gap-1.5 mb-2 px-2 py-1 text-xs rounded bg-[var(--accent)]/10 text-[var(--accent)] hover:bg-[var(--accent)]/20 transition-colors">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12"/></svg>
              Clear: <strong class="ml-0.5">{t.tagFilter}</strong>
            </button>
          {/if}

          <p class="text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 px-1 mb-1.5">Colors</p>
          {#each DEFAULT_TAGS as dt}
            <button on:click={() => updateTab({ tagFilter: t.tagFilter === dt.name ? null : dt.name })}
              class="w-full flex items-center gap-2 px-2 py-1.5 rounded mb-0.5 text-left transition-colors
                {t.tagFilter === dt.name ? 'bg-[var(--accent)]/10' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}">
              <span class="w-2.5 h-2.5 rounded-full shrink-0 {dt.color}"></span>
              <span class="text-sm text-gray-700 dark:text-gray-300 flex-1 capitalize">{dt.name}</span>
              {#if defaultTagCounts[dt.name] > 0}
                <span class="text-[11px] text-gray-400 dark:text-gray-500">{defaultTagCounts[dt.name]}</span>
              {/if}
            </button>
          {/each}

          {#if allDirTags.filter(d => !TAG_COLOR_OVERRIDES[d.tag]).length > 0}
            <p class="text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 px-1 mt-3 mb-1.5">In this folder</p>
            {#each allDirTags.filter(d => !TAG_COLOR_OVERRIDES[d.tag]) as { tag, count }}
              <button on:click={() => updateTab({ tagFilter: t.tagFilter === tag ? null : tag })}
                class="w-full flex items-center gap-2 px-2 py-1.5 rounded mb-0.5 text-left transition-colors
                  {t.tagFilter === tag ? 'bg-[var(--accent)]/10' : 'hover:bg-gray-100 dark:hover:bg-gray-700'}">
                <span class="w-2.5 h-2.5 rounded-full shrink-0 {tagColor(tag)}"></span>
                <span class="text-sm text-gray-700 dark:text-gray-300 flex-1 truncate">{tag}</span>
                <span class="text-[11px] text-gray-400 dark:text-gray-500">{count}</span>
              </button>
            {/each}
          {/if}
        </div>
      {/if}
    </div>

    <!-- ── File pane(s) ───────────────────────────────────────────────────── -->
    {#each (dualPane ? [0, 1] : [0]) as pi}
      {@const paneTabs = panes[pi].tabs}
      {@const paneActiveTabIdx = panes[pi].activeTabIdx}
      {@const pt = paneTabs[paneActiveTabIdx]}
      {@const dispItems = getDisplayItems(pt)}
      {@const breadcrumb = (() => {
        if (!pt.path) return [{ label: 'Home', path: homeDir }];
        const segments = pt.path.split('/').filter(Boolean);
        const crumbs = [{ label: '/', path: '/' }];
        let acc = '';
        for (const seg of segments) { acc += '/' + seg; crumbs.push({ label: seg, path: acc }); }
        return crumbs;
      })()}

      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div class="flex-1 flex flex-col min-w-0 {dualPane && pi === 0 ? 'border-r border-gray-200 dark:border-gray-700' : ''} {activePaneIdx === pi && dualPane ? 'ring-1 ring-inset ring-[var(--accent)]/30' : ''}"
        on:mousedown={() => { activePaneIdx = pi; }}>

        <!-- Pane tab bar -->
        <div class="flex items-center bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-1.5 pt-1 gap-0.5 shrink-0 overflow-x-auto">
          {#each paneTabs as tab, i}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <div on:click={() => switchPaneTab(pi, i)}
              class="flex items-center gap-1.5 px-3 py-1.5 rounded-t text-sm cursor-pointer transition-colors shrink-0 max-w-[160px]
                {i === paneActiveTabIdx
                  ? 'bg-white dark:bg-gray-900 text-gray-800 dark:text-gray-200 border border-b-white dark:border-gray-700 dark:border-b-gray-900 -mb-px'
                  : 'text-gray-500 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-700 hover:text-gray-700 dark:hover:text-gray-300'}">
              <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round"
                class="{i === paneActiveTabIdx ? 'text-[var(--accent)]' : 'text-gray-400'} shrink-0">
                <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" />
              </svg>
              <span class="truncate">{tabLabel(tab)}</span>
              {#if paneTabs.length > 1}
                <button on:click={e => { e.stopPropagation(); activePaneIdx = pi; closeTab(i, e); }}
                  aria-label="Close tab"
                  class="ml-0.5 w-3.5 h-3.5 flex items-center justify-center rounded-full shrink-0
                    hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors">
                  <svg width="7" height="7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M18 6L6 18M6 6l12 12"/></svg>
                </button>
              {/if}
            </div>
          {/each}
          <button on:click={() => { activePaneIdx = pi; newTab(); }}
            class="flex items-center justify-center w-6 h-6 mb-0.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors shrink-0"
            aria-label="New tab">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 5v14M5 12h14"/></svg>
          </button>
        </div>

        <!-- Toolbar -->
        <div class="flex items-center gap-1 px-3 py-2 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 shrink-0">
          <button on:click={() => { activePaneIdx = pi; goBack(); }} disabled={pt.historyIdx === 0}
            aria-label="Go back"
            class="p-1 rounded transition-colors {pt.historyIdx > 0 ? 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800' : 'text-gray-300 dark:text-gray-700 cursor-default'}">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
          </button>
          <button on:click={() => { activePaneIdx = pi; goForward(); }} disabled={pt.historyIdx >= pt.history.length - 1}
            aria-label="Go forward"
            class="p-1 rounded transition-colors {pt.historyIdx < pt.history.length - 1 ? 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800' : 'text-gray-300 dark:text-gray-700 cursor-default'}">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
          </button>

          <div class="flex items-center gap-0.5 ml-1 flex-1 min-w-0 overflow-hidden">
            {#each breadcrumb as crumb, i}
              {#if i > 0}<span class="text-gray-300 dark:text-gray-600 text-xs mx-0.5">›</span>{/if}
              <button on:click={() => { activePaneIdx = pi; navigateTo(crumb.path); }}
                class="text-sm px-1 py-0.5 rounded transition-colors truncate max-w-[120px]
                  {i === breadcrumb.length - 1
                    ? 'text-gray-800 dark:text-gray-200 font-medium'
                    : 'text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-gray-800'}"
              >{crumb.label}</button>
            {/each}
          </div>

          {#if pt.tagFilter}
            <div class="flex items-center gap-1 px-2 py-0.5 rounded-full bg-[var(--accent)]/15 text-[var(--accent)] text-[11px] shrink-0">
              <span class="w-1.5 h-1.5 rounded-full {tagColor(pt.tagFilter)}"></span>
              {pt.tagFilter}
              <button on:click={() => { activePaneIdx = pi; updateTab({ tagFilter: null }); }} class="ml-0.5 hover:opacity-70" aria-label="Clear tag filter">×</button>
            </div>
          {/if}

          <div class="flex items-center gap-1.5 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded px-2 py-1 ml-2 w-36 shrink-0 focus-within:border-[var(--accent)] transition-colors">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gray-400 shrink-0">
              <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /><path d="M21 21l-6 -6" />
            </svg>
            <input value={pt.search} on:input={e => { activePaneIdx = pi; updateTab({ search: e.currentTarget.value }); }}
              placeholder="Search"
              class="text-sm text-gray-700 dark:text-gray-200 bg-transparent outline-none w-full placeholder-gray-400" />
          </div>
        </div>

        <!-- Column headers -->
        <div class="flex items-center px-4 py-1.5 border-b border-gray-100 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-[11px] font-semibold text-gray-400 uppercase tracking-wider shrink-0">
          <div class="flex-1">Name</div>
          <div class="w-32">Tags</div>
          <div class="w-28 text-right">Modified</div>
          <div class="w-20 text-right">Kind</div>
          <div class="w-16 text-right">Size</div>
        </div>

        <!-- File list -->
        <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
        <div class="flex-1 overflow-y-auto outline-none" tabindex="0"
          bind:this={fileListEls[pi]}
          on:keydown={e => { activePaneIdx = pi; handleKeydown(e, pi, dispItems); }}>
          {#if pt.loading}
            <div class="flex items-center justify-center h-full gap-2 text-gray-300 dark:text-gray-600">
              <svg class="animate-spin" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0"/></svg>
              <span class="text-sm">Loading…</span>
            </div>
          {:else if pt.error}
            <div class="flex flex-col items-center justify-center h-full gap-2 text-red-400">
              <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
              <p class="text-sm text-center px-4">{pt.error}</p>
            </div>
          {:else if dispItems.length === 0}
            <div class="flex flex-col items-center justify-center h-full gap-2 text-gray-300 dark:text-gray-600">
              <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" /></svg>
              <p class="text-sm">{pt.search.trim() || pt.tagFilter ? 'No results' : 'Empty folder'}</p>
            </div>
          {:else}
            {#each dispItems as item, i}
              <!-- svelte-ignore a11y-click-events-have-key-events -->
              <!-- svelte-ignore a11y-no-static-element-interactions -->
              <div data-idx={i}
                on:click={() => { activePaneIdx = pi; selectItem(item, pi); }}
                on:dblclick={() => { activePaneIdx = pi; openItem(item); }}
                on:contextmenu={e => { activePaneIdx = pi; onContextMenu(e, item); }}
                class="flex items-center px-4 py-1.5 gap-3 cursor-default transition-colors
                  {pt.selected === item.name ? 'bg-[var(--accent)] text-white' : 'hover:bg-gray-50 dark:hover:bg-gray-800 text-gray-800 dark:text-gray-200'}">
                {#if item.is_dir}
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 {pt.selected === item.name ? 'text-white/75' : 'text-[var(--accent)]'}">
                    <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" />
                  </svg>
                {:else}
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 {pt.selected === item.name ? 'text-white/60' : 'text-gray-400 dark:text-gray-500'}">
                    <path d="M14 3v4a1 1 0 0 0 1 1h4" /><path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" />
                  </svg>
                {/if}

                <span class="flex-1 text-sm truncate">{item.name}</span>

                <!-- Tag chips -->
                <div class="w-32 flex items-center gap-0.5 overflow-hidden shrink-0">
                  {#each (item.tags || []).slice(0, 3) as tag}
                    <span class="inline-flex items-center px-1.5 py-0.5 rounded-full text-[10px] font-medium text-white {tagColor(tag)} shrink-0">{tag}</span>
                  {/each}
                  {#if (item.tags || []).length > 3}
                    <span class="text-[10px] text-gray-400">+{item.tags.length - 3}</span>
                  {/if}
                </div>

                <span class="w-28 text-right text-xs {pt.selected === item.name ? 'text-white/60' : 'text-gray-400 dark:text-gray-500'}">{formatDate(item.modified)}</span>
                <span class="w-20 text-right text-xs {pt.selected === item.name ? 'text-white/60' : 'text-gray-400 dark:text-gray-500'}">{kindLabel(item)}</span>
                <span class="w-16 text-right text-xs {pt.selected === item.name ? 'text-white/60' : 'text-gray-400 dark:text-gray-500'}">{item.is_dir ? '—' : formatSize(item.size)}</span>
              </div>
            {/each}
          {/if}
        </div>

        <!-- Status bar / Tag editor -->
        <div class="border-t border-gray-100 dark:border-gray-700 px-4 py-1.5 bg-gray-50 dark:bg-gray-800 shrink-0 min-h-[34px] flex items-center">
          {#if pt.selectedItem}
            <div class="flex items-center gap-1.5 flex-wrap flex-1">
              {#each (pt.selectedItem.tags || []) as tag}
                <span class="inline-flex items-center gap-1 px-1.5 py-0.5 rounded-full text-[11px] font-medium text-white {tagColor(tag)}">
                  {tag}
                  <button on:click|stopPropagation={() => { activePaneIdx = pi; removeTag(tag); }} disabled={tagSaving} class="hover:opacity-70 leading-none">×</button>
                </span>
              {/each}
              <form on:submit|preventDefault={() => { activePaneIdx = pi; addTag(); }} class="flex items-center gap-1">
                <input value={pt.tagInput} on:input={e => { activePaneIdx = pi; updateTab({ tagInput: e.currentTarget.value }); }}
                  placeholder="+ add tag" disabled={tagSaving}
                  class="text-[12px] bg-transparent border border-gray-300 dark:border-gray-600 rounded px-1.5 py-0.5 outline-none focus:border-[var(--accent)] w-20 text-gray-700 dark:text-gray-300 placeholder-gray-400" />
              </form>
              {#if tagError}<span class="text-[11px] text-red-400">{tagError}</span>{/if}
              <span class="ml-auto text-xs text-gray-400 truncate max-w-[200px]">{pt.selectedItem.name}</span>
            </div>
          {:else}
            <span class="text-xs text-gray-400 dark:text-gray-500">
              {dispItems.length} item{dispItems.length !== 1 ? 's' : ''}
              {#if pt.search.trim()} · filtered by "{pt.search}"{/if}
              {#if pt.tagFilter} · tag: {pt.tagFilter}{/if}
            </span>
          {/if}
        </div>

      </div>
    {/each}

  </div>
</div>

{#if contextMenu}
  <button
    type="button"
    class="fixed inset-0 z-40 cursor-default bg-transparent border-none p-0"
    aria-label="Close context menu"
    on:click={hideContextMenu}
    on:keydown={(e) => { if (e.key === 'Escape') hideContextMenu(); }}
  ></button>
  <div class="fixed z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600
              rounded shadow-lg py-1 min-w-[190px]"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    on:click|stopPropagation>

    <!-- Quick Convert presets (media files only) -->
    {#each quickConvertPresets(contextMenu.item.extension) as { preset, label }}
      <button
        class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
               hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2"
        on:click={() => runQuickConvert(contextMenu.item, preset)}
      >
        <span class="text-[var(--accent)] font-medium">Quick Convert</span>
        <span>{label}</span>
      </button>
    {/each}

    <!-- Custom Splice presets -->
    {#each customPresetsFor(contextMenu.item.extension) as cp (cp.id)}
      <button
        class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
               hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2"
        on:click={() => runSplicePreset(contextMenu.item, cp.id)}
      >
        <span class="text-[var(--accent)] font-medium">Splice</span>
        <span>{cp.name}</span>
      </button>
    {/each}

    <!-- Separator if both sections present -->
    {#if (quickConvertPresets(contextMenu.item.extension).length > 0 || customPresetsFor(contextMenu.item.extension).length > 0) && winepathFound}
      <div class="my-1 border-t border-gray-200 dark:border-gray-600"></div>
    {/if}

    <!-- Copy Windows path (Wine) -->
    {#if winepathFound}
      <button
        class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
               hover:bg-gray-100 dark:hover:bg-gray-700"
        on:click={copyWindowsPath}
      >
        Copy Windows path
      </button>
    {/if}

    <!-- Fallback if nothing to show -->
    {#if quickConvertPresets(contextMenu.item.extension).length === 0 && customPresetsFor(contextMenu.item.extension).length === 0 && !winepathFound}
      <button
        class="w-full text-left px-3 py-1.5 text-xs text-gray-400 cursor-default"
        disabled
      >No actions available</button>
    {/if}
  </div>
{/if}

{#if toast}
  <div class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-gray-800 dark:bg-gray-700 text-white text-xs px-4 py-2 rounded shadow-lg pointer-events-none">
    {toast}
  </div>
{/if}
