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
      archiveContext: null, // { archivePath, internalDir } when browsing inside an archive
    };
  }

  // ── Archive helpers ────────────────────────────────────────────────────────
  const ARCHIVE_EXTS = new Set(['zip', '7z', 'tar', 'gz', 'bz2', 'xz', 'rar', 'tgz', 'tbz2', 'txz']);

  function isArchive(item) {
    if (!item || item.is_dir) return false;
    const name = (item.name ?? '').toLowerCase();
    if (name.endsWith('.tar.gz') || name.endsWith('.tar.bz2') || name.endsWith('.tar.xz')) return true;
    const ext = (item.extension ?? '').toLowerCase();
    return ARCHIVE_EXTS.has(ext);
  }

  // Returns direct children of internalDir from the flat archive listing.
  // Synthesizes directory entries for implicit dirs (some archivers omit them).
  function archiveEntriesForDir(entries, internalDir) {
    const prefix = internalDir ? internalDir + '/' : '';
    const seen = new Set();
    const result = [];
    for (const entry of entries) {
      if (!entry.path.startsWith(prefix)) continue;
      const rest = entry.path.slice(prefix.length);
      if (!rest) continue;
      const slashIdx = rest.indexOf('/');
      if (slashIdx === -1) {
        if (!seen.has(entry.path)) { seen.add(entry.path); result.push(entry); }
      } else {
        const dirName = rest.slice(0, slashIdx);
        const dirKey = prefix + dirName;
        if (!seen.has(dirKey)) {
          seen.add(dirKey);
          result.push({ name: dirName, path: dirKey, is_dir: true, size: null, modified: null, extension: null, tags: [] });
        }
      }
    }
    // dirs first, then alpha
    result.sort((a, b) => {
      if (a.is_dir && !b.is_dir) return -1;
      if (!a.is_dir && b.is_dir) return 1;
      return a.name.localeCompare(b.name);
    });
    return result;
  }

  async function enterArchive(archivePath) {
    const parentDir = archivePath.split('/').slice(0, -1).join('/') || '/';
    // Push parent dir to history so goBack() exits the archive naturally
    const pi = activePaneIdx;
    const p = panes[pi];
    const tab = p.tabs[p.activeTabIdx];
    const newHistory = [...tab.history.slice(0, tab.historyIdx + 1), parentDir];
    panes[pi].tabs[p.activeTabIdx] = {
      ...tab,
      history: newHistory,
      historyIdx: newHistory.length - 1,
      archiveContext: { archivePath, internalDir: '' },
      loading: true, error: null,
      selected: null, selectedItem: null, selectedIndex: -1,
    };
    panes = panes;
    await loadArchiveDir();
  }

  async function loadArchiveDir() {
    const pi = activePaneIdx;
    const ctx = panes[pi].tabs[panes[pi].activeTabIdx].archiveContext;
    if (!ctx) return;
    try {
      const allEntries = await invoke('list_archive', { path: ctx.archivePath });
      const items = archiveEntriesForDir(allEntries, ctx.internalDir);
      panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, loading: false };
      panes = panes;
    } catch (e) {
      panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], error: String(e), loading: false };
      panes = panes;
    }
  }

  // ── Pane factory ───────────────────────────────────────────────────────────
  function makePaneState() {
    return { tabs: [makeTab()], activeTabIdx: 0 };
  }

  // ── Global state ───────────────────────────────────────────────────────────
  let homeDir = '';
  let volumes = [];
  let sidebarTab = $state('quick');
  let { dualPane = $bindable(false) } = $props();
  let panes = $state([makePaneState(), makePaneState()]);
  let activePaneIdx = $state(0);

  // Folder tree (shared across panes)
  let treeRoots = [];
  let treeExpanded = $state(new Set());
  let treeChildren = {};

  // Tag save state
  let tagSaving = $state(false);
  let tagError = $state(null);

  // File list element refs per pane
  let fileListEls = [];

  // Context menu
  let contextMenu = $state(null);
  let winepathFound = $state(false);
  let wineprefixReady = false;

  // Custom Fade presets
  let customPresets = [];

  // Toast
  let toast = $state(null);
  let toastTimer = null;

  // ── Active pane/tab shorthands ─────────────────────────────────────────────
  let ap = $derived(panes[activePaneIdx]);
  let t = $derived(ap.tabs[ap.activeTabIdx]);

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

    // Load custom Fade presets (best-effort — Fade may not be installed yet)
    try { customPresets = await invoke('list_fade_presets'); } catch { /* no-op */ }

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

    // Inside an archive: step up one internal dir level first
    if (tab.archiveContext && tab.archiveContext.internalDir) {
      const parts = tab.archiveContext.internalDir.split('/');
      parts.pop();
      const newInternalDir = parts.join('/');
      panes[pi].tabs[p.activeTabIdx] = {
        ...tab,
        archiveContext: { ...tab.archiveContext, internalDir: newInternalDir },
        selected: null, selectedItem: null, selectedIndex: -1,
        loading: true, error: null,
      };
      panes = panes;
      await loadArchiveDir();
      return;
    }

    // Exit archive or normal back
    if (tab.historyIdx <= 0 && !tab.archiveContext) return;
    if (tab.archiveContext) {
      // At archive root — exit to filesystem
      panes[pi].tabs[p.activeTabIdx] = {
        ...tab,
        archiveContext: null,
        selected: null, selectedItem: null, selectedIndex: -1,
        search: '', tagFilter: null, loading: true, error: null,
      };
      panes = panes;
      const items = await loadDir(tab.history[tab.historyIdx]);
      panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], items, loading: false };
      panes = panes;
      return;
    }

    const newIdx = tab.historyIdx - 1;
    const path = tab.history[newIdx];
    panes[pi].tabs[p.activeTabIdx] = { ...tab, historyIdx: newIdx, path,
      archiveContext: null,
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
  let quickAccessGroups = $derived([
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
  ]);

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

  let treeRows = $derived(flattenTree(treeRoots));

  // ── Tags in current dir (active pane) ─────────────────────────────────────
  let defaultTagCounts = $derived(Object.fromEntries(
    DEFAULT_TAGS.map(dt => [dt.name, t.items.filter(i => (i.tags || []).includes(dt.name)).length])
  ));

  let allDirTags = $derived((() => {
    const counts = {};
    for (const item of t.items) {
      for (const tag of (item.tags || [])) {
        counts[tag] = (counts[tag] || 0) + 1;
      }
    }
    return Object.entries(counts).sort((a, b) => b[1] - a[1]).map(([tag, count]) => ({ tag, count }));
  })());

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
    if (item.is_dir && !t.archiveContext) return; // no context menu for plain folders
    if (item.is_dir && t.archiveContext) return; // no context menu for dirs inside archives either
    contextMenu = { x: e.clientX, y: e.clientY, item, insideArchive: !!t.archiveContext, archiveCtx: t.archiveContext };
  }

  async function extractHere(item) {
    hideContextMenu();
    const parentDir = item.path.split('/').slice(0, -1).join('/') || '/';
    showToast('Extracting…');
    try {
      await invoke('extract_archive', { path: item.path, dest: parentDir });
      showToast('Extracted');
      dirCache.delete(parentDir);
      if (!t.archiveContext) await refreshDir();
    } catch (e) { showToast(`Extract failed: ${e}`); }
  }

  async function extractTo(item) {
    hideContextMenu();
    const dest = await invoke('pick_directory');
    if (!dest) return;
    showToast('Extracting…');
    try {
      await invoke('extract_archive', { path: item.path, dest });
      showToast('Extracted');
    } catch (e) { showToast(`Extract failed: ${e}`); }
  }

  async function extractSelectedFromArchive() {
    const ctx = t.archiveContext;
    if (!ctx || !t.selectedItem) return;
    hideContextMenu();
    const parentDir = ctx.archivePath.split('/').slice(0, -1).join('/') || '/';
    showToast('Extracting…');
    try {
      await invoke('extract_files', { path: ctx.archivePath, files: [t.selectedItem.path], dest: parentDir });
      showToast('Extracted');
    } catch (e) { showToast(`Extract failed: ${e}`); }
  }

  async function extractAllFromArchive(dest) {
    const ctx = t.archiveContext;
    if (!ctx) return;
    hideContextMenu();
    const target = dest ?? (ctx.archivePath.split('/').slice(0, -1).join('/') || '/');
    showToast('Extracting…');
    try {
      await invoke('extract_archive', { path: ctx.archivePath, dest: target });
      showToast('Extracted');
      dirCache.delete(target);
    } catch (e) {
      if (String(e).includes('PASSWORD_REQUIRED')) {
        showPasswordDialog(ctx.archivePath, target);
      } else {
        showToast(`Extract failed: ${e}`);
      }
    }
  }

  async function extractAllFromArchiveTo() {
    const ctx = t.archiveContext;
    if (!ctx) return;
    hideContextMenu();
    const dest = await invoke('pick_directory');
    if (!dest) return;
    await extractAllFromArchive(dest);
  }

  // Password dialog state
  let passwordDialog = $state(null); // { archivePath, dest }
  let passwordInput = $state('');

  function showPasswordDialog(archivePath, dest) {
    passwordDialog = { archivePath, dest };
    passwordInput = '';
  }

  async function submitPassword() {
    if (!passwordDialog) return;
    const { archivePath, dest } = passwordDialog;
    passwordDialog = null;
    showToast('Extracting…');
    try {
      await invoke('extract_archive_with_password', { path: archivePath, dest, password: passwordInput });
      showToast('Extracted');
      dirCache.delete(dest);
    } catch (e) { showToast(`Wrong password or extract failed: ${e}`); }
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

  async function runFadePreset(item, presetId) {
    hideContextMenu();
    showToast('Converting…');
    try {
      await invoke('run_fade_preset', { path: item.path, presetId });
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
    if (t.archiveContext) {
      // Inside an archive — navigate deeper into subdirs; files do nothing
      if (item.is_dir) {
        const ctx = t.archiveContext;
        const newInternalDir = ctx.internalDir ? `${ctx.internalDir}/${item.name}` : item.name;
        updateTab({
          archiveContext: { ...ctx, internalDir: newInternalDir },
          selected: null, selectedItem: null, selectedIndex: -1,
          loading: true, error: null,
        });
        await loadArchiveDir();
        // TODO: nested archive-within-archive browse not supported in M1
      }
      return;
    }
    if (item.is_dir) {
      await navigateTo(item.path);
    } else if (isArchive(item)) {
      await enterArchive(item.path);
    } else {
      try { await invoke('open_file', { path: item.path }); } catch (e) { console.error(e); }
    }
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
    <div class="sidebar-nav w-48 bg-gray-50 dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 flex flex-col shrink-0 outline-none"
      tabindex="-1">

      <!-- Sidebar tabs -->
      <div class="flex border-b border-gray-200 dark:border-gray-700 px-1 pt-1.5 gap-0.5 shrink-0">
        {#each [['quick','Quick'],['folders','Folders'],['tags','Tags']] as [id, label]}
          <button onclick={() => sidebarTab = id}
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
              <button onclick={() => item.path && navigateTo(item.path)} disabled={!item.path}
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
              <button onclick={(e) => { e.stopPropagation(); toggleTreeExpand(node); }}
                class="w-4 h-4 flex items-center justify-center shrink-0 {active ? 'text-white/60' : 'text-gray-400 dark:text-gray-600'}">
                <svg width="8" height="8" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3">
                  {#if treeExpanded.has(node.path)}<path d="M6 9l6 6 6-6"/>{:else}<path d="M9 18l6-6-6-6"/>{/if}
                </svg>
              </button>
              <div onclick={() => navigateTo(node.path)} class="flex items-center gap-1.5 flex-1 min-w-0" role="button" tabindex="0" onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') navigateTo(node.path); }}>
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
            <button onclick={() => updateTab({ tagFilter: null })}
              class="w-full flex items-center gap-1.5 mb-2 px-2 py-1 text-xs rounded bg-[var(--accent)]/10 text-[var(--accent)] hover:bg-[var(--accent)]/20 transition-colors">
              <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M18 6L6 18M6 6l12 12"/></svg>
              Clear: <strong class="ml-0.5">{t.tagFilter}</strong>
            </button>
          {/if}

          <p class="text-[10px] font-semibold uppercase tracking-widest text-gray-400 dark:text-gray-500 px-1 mb-1.5">Colors</p>
          {#each DEFAULT_TAGS as dt}
            <button onclick={() => updateTab({ tagFilter: t.tagFilter === dt.name ? null : dt.name })}
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
              <button onclick={() => updateTab({ tagFilter: t.tagFilter === tag ? null : tag })}
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
        if (pt.archiveContext) {
          // Real filesystem crumbs up to the archive file
          const archivePath = pt.archiveContext.archivePath;
          const segs = archivePath.split('/').filter(Boolean);
          const fsCrumbs = [{ label: '/', path: '/', isArchive: false }];
          let acc = '';
          for (const seg of segs) { acc += '/' + seg; fsCrumbs.push({ label: seg, path: acc, isArchive: false }); }
          // Archive root crumb
          const archiveName = segs[segs.length - 1] ?? 'archive';
          fsCrumbs[fsCrumbs.length - 1] = { label: archiveName, path: archivePath, isArchive: true };
          // Internal path segments
          if (pt.archiveContext.internalDir) {
            const parts = pt.archiveContext.internalDir.split('/');
            let iacc = '';
            for (const part of parts) {
              iacc = iacc ? `${iacc}/${part}` : part;
              fsCrumbs.push({ label: part, internalPath: iacc, isArchive: true });
            }
          }
          return fsCrumbs;
        }
        if (!pt.path) return [{ label: 'Home', path: homeDir, isArchive: false }];
        const segments = pt.path.split('/').filter(Boolean);
        const crumbs = [{ label: '/', path: '/', isArchive: false }];
        let acc = '';
        for (const seg of segments) { acc += '/' + seg; crumbs.push({ label: seg, path: acc, isArchive: false }); }
        return crumbs;
      })()}

      <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
      <div role="region" aria-label="File pane"
        class="flex-1 flex flex-col min-w-0 {dualPane && pi === 0 ? 'border-r border-gray-200 dark:border-gray-700' : ''} {activePaneIdx === pi && dualPane ? 'ring-1 ring-inset ring-[var(--accent)]/30' : ''}"
        onmousedown={() => { activePaneIdx = pi; }}>

        <!-- Pane tab bar -->
        <div class="flex items-center bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-1.5 pt-1 gap-0.5 shrink-0 overflow-x-auto">
          {#each paneTabs as tab, i}
            <div onclick={() => switchPaneTab(pi, i)}
              role="tab"
              tabindex="0"
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') switchPaneTab(pi, i); }}
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
                <button onclick={(e) => { e.stopPropagation(); activePaneIdx = pi; closeTab(i, e); }}
                  aria-label="Close tab"
                  class="ml-0.5 w-3.5 h-3.5 flex items-center justify-center rounded-full shrink-0
                    hover:bg-gray-300 dark:hover:bg-gray-600 text-gray-400 hover:text-gray-700 dark:hover:text-gray-200 transition-colors">
                  <svg width="7" height="7" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3"><path d="M18 6L6 18M6 6l12 12"/></svg>
                </button>
              {/if}
            </div>
          {/each}
          <button onclick={() => { activePaneIdx = pi; newTab(); }}
            class="flex items-center justify-center w-6 h-6 mb-0.5 rounded text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors shrink-0"
            aria-label="New tab">
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M12 5v14M5 12h14"/></svg>
          </button>
        </div>

        <!-- Toolbar -->
        <div class="flex items-center gap-1 px-3 py-2 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 shrink-0">
          <button onclick={() => { activePaneIdx = pi; goBack(); }} disabled={pt.historyIdx === 0}
            aria-label="Go back"
            class="p-1 rounded transition-colors {pt.historyIdx > 0 ? 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800' : 'text-gray-300 dark:text-gray-700 cursor-default'}">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M15 18l-6-6 6-6"/></svg>
          </button>
          <button onclick={() => { activePaneIdx = pi; goForward(); }} disabled={pt.historyIdx >= pt.history.length - 1}
            aria-label="Go forward"
            class="p-1 rounded transition-colors {pt.historyIdx < pt.history.length - 1 ? 'text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800' : 'text-gray-300 dark:text-gray-700 cursor-default'}">
            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5"><path d="M9 18l6-6-6-6"/></svg>
          </button>

          <div class="flex items-center gap-0.5 ml-1 flex-1 min-w-0 overflow-hidden">
            {#each breadcrumb as crumb, i}
              {#if i > 0}<span class="text-gray-300 dark:text-gray-600 text-xs mx-0.5">›</span>{/if}
              <button onclick={() => {
                activePaneIdx = pi;
                if (crumb.isArchive && !crumb.internalPath) {
                  // Go to archive root
                  const ctx = pt.archiveContext;
                  if (ctx) {
                    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], archiveContext: { ...ctx, internalDir: '' }, loading: true, error: null, selected: null, selectedItem: null, selectedIndex: -1 };
                    panes = panes;
                    loadArchiveDir();
                  }
                } else if (crumb.isArchive && crumb.internalPath !== undefined) {
                  // Navigate to an internal archive dir
                  const ctx = pt.archiveContext;
                  if (ctx) {
                    panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], archiveContext: { ...ctx, internalDir: crumb.internalPath }, loading: true, error: null, selected: null, selectedItem: null, selectedIndex: -1 };
                    panes = panes;
                    loadArchiveDir();
                  }
                } else {
                  // Normal filesystem navigation — clear archiveContext
                  panes[pi].tabs[panes[pi].activeTabIdx] = { ...panes[pi].tabs[panes[pi].activeTabIdx], archiveContext: null };
                  panes = panes;
                  navigateTo(crumb.path);
                }
              }}
                class="text-sm px-1 py-0.5 rounded transition-colors truncate max-w-[120px]
                  {i === breadcrumb.length - 1
                    ? crumb.isArchive ? 'text-[var(--accent)] font-medium' : 'text-gray-800 dark:text-gray-200 font-medium'
                    : crumb.isArchive ? 'text-[var(--accent)]/70 hover:text-[var(--accent)] hover:bg-[var(--accent)]/10' : 'text-gray-400 hover:text-gray-600 hover:bg-gray-100 dark:hover:bg-gray-800'}"
              >{#if crumb.isArchive && i > 0}
                <span class="mr-0.5 opacity-60">⊞</span>{/if}{crumb.label}</button>
            {/each}
          </div>

          {#if pt.archiveContext}
            <div class="flex items-center gap-1 shrink-0 ml-1">
              <button onclick={() => { activePaneIdx = pi; extractAllFromArchive(); }}
                class="px-2 py-0.5 text-[11px] rounded bg-[var(--accent)]/10 text-[var(--accent)] hover:bg-[var(--accent)]/20 transition-colors"
                title="Extract all to archive's folder">Extract All</button>
              <button onclick={() => { activePaneIdx = pi; extractAllFromArchiveTo(); }}
                class="px-2 py-0.5 text-[11px] rounded bg-gray-100 dark:bg-gray-800 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-700 transition-colors"
                title="Extract all to a chosen folder">Extract To…</button>
            </div>
          {/if}

          {#if pt.tagFilter}
            <div class="flex items-center gap-1 px-2 py-0.5 rounded-full bg-[var(--accent)]/15 text-[var(--accent)] text-[11px] shrink-0">
              <span class="w-1.5 h-1.5 rounded-full {tagColor(pt.tagFilter)}"></span>
              {pt.tagFilter}
              <button onclick={() => { activePaneIdx = pi; updateTab({ tagFilter: null }); }} class="ml-0.5 hover:opacity-70" aria-label="Clear tag filter">×</button>
            </div>
          {/if}

          <div class="flex items-center gap-1.5 bg-gray-100 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded px-2 py-1 ml-2 w-36 shrink-0 focus-within:border-[var(--accent)] transition-colors">
            <svg width="11" height="11" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-gray-400 shrink-0">
              <path d="M3 10a7 7 0 1 0 14 0a7 7 0 1 0 -14 0" /><path d="M21 21l-6 -6" />
            </svg>
            <input value={pt.search} oninput={e => { activePaneIdx = pi; updateTab({ search: e.currentTarget.value }); }}
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
        <div class="flex-1 overflow-y-auto outline-none" role="listbox" tabindex="0"
          bind:this={fileListEls[pi]}
          onkeydown={e => { activePaneIdx = pi; handleKeydown(e, pi, dispItems); }}>
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
              <div data-idx={i}
                role="row"
                tabindex="0"
                onclick={() => { activePaneIdx = pi; selectItem(item, pi); }}
                ondblclick={() => { activePaneIdx = pi; openItem(item); }}
                oncontextmenu={e => { activePaneIdx = pi; onContextMenu(e, item); }}
                onkeydown={e => { if (e.key === 'Enter') { activePaneIdx = pi; openItem(item); } }}
                class="flex items-center px-4 py-1.5 gap-3 cursor-default transition-colors
                  {pt.selected === item.name ? 'bg-[var(--accent)] text-white' : 'hover:bg-gray-50 dark:hover:bg-gray-800 text-gray-800 dark:text-gray-200'}">
                {#if item.is_dir}
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 {pt.selected === item.name ? 'text-white/75' : 'text-[var(--accent)]'}">
                    <path d="M5 4h4l3 3h7a2 2 0 0 1 2 2v8a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2" />
                  </svg>
                {:else if isArchive(item)}
                  <!-- Archive file icon -->
                  <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.75" stroke-linecap="round" stroke-linejoin="round" class="shrink-0 {pt.selected === item.name ? 'text-white/75' : 'text-amber-500 dark:text-amber-400'}">
                    <path d="M14 3v4a1 1 0 0 0 1 1h4" /><path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2" />
                    <path d="M12 17v-6" /><path d="M9.5 14.5l2.5 2.5l2.5 -2.5" />
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
                <span class="w-16 text-right text-xs {pt.selected === item.name ? 'text-white/60' : 'text-gray-400 dark:text-gray-500'}">
                  {#if item.is_dir}—{:else if pt.archiveContext && item.compressed_size != null && item.compressed_size !== item.size}
                    {formatSize(item.compressed_size)} <span class="opacity-60">→ {formatSize(item.size)}</span>
                  {:else}{formatSize(item.size)}{/if}
                </span>
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
                  <button onclick={(e) => { e.stopPropagation(); activePaneIdx = pi; removeTag(tag); }} disabled={tagSaving} class="hover:opacity-70 leading-none">×</button>
                </span>
              {/each}
              <form onsubmit={(e) => { e.preventDefault(); activePaneIdx = pi; addTag(); }} class="flex items-center gap-1">
                <input value={pt.tagInput} oninput={e => { activePaneIdx = pi; updateTab({ tagInput: e.currentTarget.value }); }}
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
    onclick={hideContextMenu}
    onkeydown={(e) => { if (e.key === 'Escape') hideContextMenu(); }}
  ></button>
  <div class="fixed z-50 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600
              rounded shadow-lg py-1 min-w-[190px]"
    style="left: {contextMenu.x}px; top: {contextMenu.y}px;"
    role="menu"
    tabindex="-1"
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => { if (e.key === 'Escape') hideContextMenu(); }}>

    <!-- Archive actions: inside an archive -->
    {#if contextMenu.insideArchive}
      <button class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
        onclick={extractSelectedFromArchive}>Extract Selected Here</button>
      <button class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
        onclick={() => extractAllFromArchive()}>Extract All Here</button>
      <button class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
        onclick={extractAllFromArchiveTo}>Extract All To…</button>
    {:else}
      <!-- Archive actions: archive file in filesystem -->
      {#if isArchive(contextMenu.item)}
        <button class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          onclick={() => extractHere(contextMenu.item)}>Extract Here</button>
        <button class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          onclick={() => extractTo(contextMenu.item)}>Extract To…</button>
        <div class="my-1 border-t border-gray-200 dark:border-gray-600"></div>
      {/if}

      <!-- Quick Convert presets (media files only) -->
      {#each quickConvertPresets(contextMenu.item.extension) as { preset, label }}
        <button
          class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
                 hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2"
          onclick={() => runQuickConvert(contextMenu.item, preset)}
        >
          <span class="text-[var(--accent)] font-medium">Quick Convert</span>
          <span>{label}</span>
        </button>
      {/each}

      <!-- Custom Fade presets -->
      {#each customPresetsFor(contextMenu.item.extension) as cp (cp.id)}
        <button
          class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
                 hover:bg-gray-100 dark:hover:bg-gray-700 flex items-center gap-2"
          onclick={() => runFadePreset(contextMenu.item, cp.id)}
        >
          <span class="text-[var(--accent)] font-medium">Fade</span>
          <span>{cp.name}</span>
        </button>
      {/each}

      <!-- Separator -->
      {#if (quickConvertPresets(contextMenu.item.extension).length > 0 || customPresetsFor(contextMenu.item.extension).length > 0) && winepathFound}
        <div class="my-1 border-t border-gray-200 dark:border-gray-600"></div>
      {/if}

      <!-- Copy Windows path (Wine) -->
      {#if winepathFound}
        <button
          class="w-full text-left px-3 py-1.5 text-xs text-gray-700 dark:text-gray-300
                 hover:bg-gray-100 dark:hover:bg-gray-700"
          onclick={copyWindowsPath}
        >
          Copy Windows path
        </button>
      {/if}

      <!-- Fallback -->
      {#if !isArchive(contextMenu.item) && quickConvertPresets(contextMenu.item.extension).length === 0 && customPresetsFor(contextMenu.item.extension).length === 0 && !winepathFound}
        <button class="w-full text-left px-3 py-1.5 text-xs text-gray-400 cursor-default" disabled>No actions available</button>
      {/if}
    {/if}
  </div>
{/if}

{#if passwordDialog}
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
    <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-600 rounded-lg shadow-xl p-5 w-72">
      <p class="text-sm font-medium text-gray-800 dark:text-gray-200 mb-3">Archive password required</p>
      <input
        bind:value={passwordInput}
        type="password"
        placeholder="Enter password"
        class="w-full text-sm border border-gray-300 dark:border-gray-600 rounded px-2.5 py-1.5 bg-white dark:bg-gray-900 text-gray-800 dark:text-gray-200 outline-none focus:border-[var(--accent)] mb-3"
        onkeydown={(e) => { if (e.key === 'Enter') submitPassword(); if (e.key === 'Escape') passwordDialog = null; }}
      />
      <div class="flex justify-end gap-2">
        <button onclick={() => passwordDialog = null}
          class="px-3 py-1.5 text-xs rounded text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">Cancel</button>
        <button onclick={submitPassword}
          class="px-3 py-1.5 text-xs rounded bg-[var(--accent)] text-white hover:opacity-90 transition-opacity">Extract</button>
      </div>
    </div>
  </div>
{/if}

{#if toast}
  <div class="fixed bottom-6 left-1/2 -translate-x-1/2 z-50 bg-gray-800 dark:bg-gray-700 text-white text-xs px-4 py-2 rounded shadow-lg pointer-events-none">
    {toast}
  </div>
{/if}
