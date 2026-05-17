<script>
  import { Menu } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  // ── State ──────────────────────────────────────────────────────────────────
  let selectedFile   = $state('gallery');
  let contextOpen    = $state(false);
  let contextAnchor  = $state(null);
  let searchQuery    = $state('');
  let viewMode       = $state('column');

  // ── Static data ────────────────────────────────────────────────────────────
  const col1Items = [
    { name: 'src-tauri',     type: 'folder' },
    { name: 'src',           type: 'folder' },
    { name: 'node_modules',  type: 'folder' },
    { name: 'package.json',  type: 'file'   },
    { name: 'README.md',     type: 'file'   },
    { name: 'vite.config.js',type: 'file'   },
  ];

  const col2Items = [
    { name: 'App.svelte', type: 'file'   },
    { name: 'app.css',    type: 'file'   },
    { name: 'lib',        type: 'folder' },
    { name: 'sections',   type: 'folder' },
  ];

  const quickFiles = [
    { name: 'App.svelte',        type: 'file',   date: 'Today',     pinned: true  },
    { name: 'tokens.css',        type: 'file',   date: 'Yesterday', pinned: true  },
    { name: 'ShelfSection.svelte', type: 'file', date: 'May 10',    pinned: false },
    { name: 'Card.svelte',       type: 'file',   date: 'May 9',     pinned: false },
    { name: 'package.json',      type: 'file',   date: 'May 8',     pinned: false },
  ];

  const tagColors = {
    Work:     '#0066cc',
    Active:   '#22c55e',
    UI:       '#a855f7',
    Personal: '#f59e0b',
    Archive:  '#6b7280',
    Urgent:   '#ef4444',
    Media:    '#06b6d4',
    Design:   '#ec4899',
  };

  const contextMenuItems = [
    { label: 'Open',            onclick: () => { contextOpen = false; } },
    { label: 'Open With ▸',     onclick: () => { contextOpen = false; } },
    { label: 'Get Info',        onclick: () => { contextOpen = false; } },
    { separator: true, label: '', onclick: () => {} },
    { label: 'Copy',            onclick: () => { contextOpen = false; } },
    { label: 'Cut',             onclick: () => { contextOpen = false; } },
    { label: 'Paste',           onclick: () => { contextOpen = false; } },
    { separator: true, label: '', onclick: () => {} },
    { label: 'Move to Trash',   onclick: () => { contextOpen = false; } },
  ];

  function handleRowContextMenu(e, name) {
    e.preventDefault();
    selectedFile = name;
    contextAnchor = e.currentTarget;
    contextOpen = true;
  }

  function handleRowClick(name) {
    selectedFile = name;
  }
</script>

<div class="section">

  <!-- SH-1: Path Bar -->
  <div class="row">
    <Card id="SH-1" label="Path Bar">
      <div class="path-bar">
        <div class="path-nav">
          <button class="nav-btn" aria-label="Back">‹</button>
          <button class="nav-btn" aria-label="Forward">›</button>
          <button class="nav-btn" aria-label="Up">↑</button>
          <span class="nav-sep"></span>
          <div class="breadcrumbs">
            <button class="breadcrumb-seg">Home</button>
            <span class="bc-div">/</span>
            <button class="breadcrumb-seg">Projects</button>
            <span class="bc-div">/</span>
            <button class="breadcrumb-seg current">Libre-Apps</button>
          </div>
        </div>
        <div class="path-right">
          <div class="search-compact">
            <svg class="search-icon" width="12" height="12" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <circle cx="11" cy="11" r="7"/>
              <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            </svg>
            <input type="search" class="search-input" placeholder="Search…" bind:value={searchQuery} />
          </div>
          <div class="view-btns">
            <button class="view-btn" class:active={viewMode === 'list'}   onclick={() => viewMode = 'list'}   aria-label="List view">
              <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
                <rect x="0" y="1" width="12" height="2" rx="1"/>
                <rect x="0" y="5" width="12" height="2" rx="1"/>
                <rect x="0" y="9" width="12" height="2" rx="1"/>
              </svg>
            </button>
            <button class="view-btn" class:active={viewMode === 'column'} onclick={() => viewMode = 'column'} aria-label="Column view">
              <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
                <rect x="0" y="0" width="3.5" height="12" rx="1"/>
                <rect x="4.25" y="0" width="3.5" height="12" rx="1"/>
                <rect x="8.5"  y="0" width="3.5" height="12" rx="1"/>
              </svg>
            </button>
            <button class="view-btn" class:active={viewMode === 'grid'}   onclick={() => viewMode = 'grid'}   aria-label="Grid view">
              <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
                <rect x="0" y="0" width="5" height="5" rx="1"/>
                <rect x="7" y="0" width="5" height="5" rx="1"/>
                <rect x="0" y="7" width="5" height="5" rx="1"/>
                <rect x="7" y="7" width="5" height="5" rx="1"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </Card>
  </div>

  <!-- SH-2: File Browser — Column View -->
  <div class="row">
    <Card id="SH-2" label="File Browser — Column View">
      <div class="browser">
        <!-- Column 1 -->
        <div class="col-pane">
          <div class="section-header">Gallery</div>
          {#each col1Items as item}
            <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
            <div
              class="file-row"
              class:selected={selectedFile === item.name}
              onclick={() => handleRowClick(item.name)}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handleRowClick(item.name); }}
              oncontextmenu={(e) => handleRowContextMenu(e, item.name)}
            >
              <span class="file-icon">
                {#if item.type === 'folder'}
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                    <path d="M1 3.5C1 2.67 1.67 2 2.5 2H5l1.5 1.5H11.5C12.33 3.5 13 4.17 13 5V10.5C13 11.33 12.33 12 11.5 12H2.5C1.67 12 1 11.33 1 10.5V3.5Z" fill="currentColor" opacity="0.7"/>
                  </svg>
                {:else}
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                    <path d="M3 1.5H8.5L11 4V12.5H3V1.5Z" fill="currentColor" opacity="0.5"/>
                    <path d="M8.5 1.5V4H11" stroke="currentColor" stroke-width="1" fill="none"/>
                  </svg>
                {/if}
              </span>
              <span class="file-name">{item.name}</span>
              {#if item.type === 'folder'}
                <span class="file-arrow">›</span>
              {:else}
                <span class="file-size">—</span>
              {/if}
            </div>
          {/each}
        </div>

        <!-- Column 2 -->
        <div class="col-pane">
          <div class="section-header">src/</div>
          {#each col2Items as item}
            <!-- svelte-ignore a11y_no_static_element_interactions a11y_click_events_have_key_events -->
            <div
              class="file-row"
              class:selected={selectedFile === item.name}
              onclick={() => handleRowClick(item.name)}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') handleRowClick(item.name); }}
              oncontextmenu={(e) => handleRowContextMenu(e, item.name)}
            >
              <span class="file-icon">
                {#if item.type === 'folder'}
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                    <path d="M1 3.5C1 2.67 1.67 2 2.5 2H5l1.5 1.5H11.5C12.33 3.5 13 4.17 13 5V10.5C13 11.33 12.33 12 11.5 12H2.5C1.67 12 1 11.33 1 10.5V3.5Z" fill="currentColor" opacity="0.7"/>
                  </svg>
                {:else}
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                    <path d="M3 1.5H8.5L11 4V12.5H3V1.5Z" fill="currentColor" opacity="0.5"/>
                    <path d="M8.5 1.5V4H11" stroke="currentColor" stroke-width="1" fill="none"/>
                  </svg>
                {/if}
              </span>
              <span class="file-name">{item.name}</span>
              {#if item.type === 'folder'}
                <span class="file-arrow">›</span>
              {:else}
                <span class="file-size">—</span>
              {/if}
            </div>
          {/each}
        </div>

        <!-- Column 3: Preview -->
        <div class="col-pane col-preview">
          <div class="section-header">Preview</div>
          <div class="preview-content">
            <svg width="48" height="48" viewBox="0 0 48 48" fill="none" aria-hidden="true" class="preview-icon">
              <path d="M4 13C4 9.69 6.69 7 10 7H18L22 11H38C41.31 11 44 13.69 44 17V35C44 38.31 41.31 41 38 41H10C6.69 41 4 38.31 4 35V13Z" fill="var(--accent)" opacity="0.35"/>
            </svg>
            <span class="preview-name">{selectedFile}</span>
            <span class="preview-meta">Folder · 4 items</span>
          </div>
        </div>
      </div>

      <!-- Context menu wired to right-click -->
      <Menu bind:open={contextOpen} anchor={contextAnchor} items={contextMenuItems} />
    </Card>
  </div>

  <!-- Side panels -->
  <div class="cols">

    <!-- SH-3: Quick Files -->
    <Card id="SH-3" label="Quick Files">
      <div class="quick-files">
        <div class="qf-header">
          <span class="qf-label">Quick Files</span>
          <button class="icon-btn" aria-label="Pin">
            <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
              <path d="M7 1L11 5L9 7L7.5 5.5L4.5 8.5V10.5H2.5V8.5L5.5 5.5L4 4L2 6L1 5L7 1Z"/>
            </svg>
          </button>
        </div>

        <div class="qf-group-label">Pinned</div>
        {#each quickFiles.filter(f => f.pinned) as f}
          <div class="qf-row">
            <span class="file-icon qf-icon">
              <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                <path d="M3 1.5H8.5L11 4V12.5H3V1.5Z" fill="currentColor" opacity="0.5"/>
                <path d="M8.5 1.5V4H11" stroke="currentColor" stroke-width="1" fill="none"/>
              </svg>
            </span>
            <span class="qf-name">{f.name}</span>
            <span class="qf-date">{f.date}</span>
          </div>
        {/each}

        <div class="qf-group-label">Recent</div>
        {#each quickFiles.filter(f => !f.pinned) as f}
          <div class="qf-row">
            <span class="file-icon qf-icon">
              <svg width="14" height="14" viewBox="0 0 14 14" fill="none" aria-hidden="true">
                <path d="M3 1.5H8.5L11 4V12.5H3V1.5Z" fill="currentColor" opacity="0.5"/>
                <path d="M8.5 1.5V4H11" stroke="currentColor" stroke-width="1" fill="none"/>
              </svg>
            </span>
            <span class="qf-name">{f.name}</span>
            <span class="qf-date">{f.date}</span>
          </div>
        {/each}
      </div>
    </Card>

    <!-- SH-4: Preview Panel -->
    <Card id="SH-4" label="Preview Panel">
      <div class="preview-panel">
        <div class="section-header">Preview</div>
        <div class="preview-body">
          <div class="preview-icon-wrap">
            <svg width="64" height="64" viewBox="0 0 64 64" fill="none" aria-hidden="true">
              <path d="M6 18C6 13.58 9.58 10 14 10H24L30 16H50C54.42 16 58 19.58 58 24V48C58 52.42 54.42 56 50 56H14C9.58 56 6 52.42 6 48V18Z" fill="var(--accent)" opacity="0.35"/>
            </svg>
          </div>
          <p class="preview-filename">gallery</p>
          <p class="preview-typemeta">Folder · 24 items</p>
          <button class="open-btn">Open</button>
          <!-- svelte-ignore a11y_missing_attribute -->
          <a class="finder-link" role="button" tabindex="0">Show in Finder</a>
        </div>
      </div>
    </Card>

    <!-- SH-5: File Info -->
    <Card id="SH-5" label="File Info">
      <div class="file-info">
        <div class="section-header">Info</div>
        <div class="info-row"><span class="info-label">Name</span><span class="info-val">gallery</span></div>
        <div class="info-row"><span class="info-label">Kind</span><span class="info-val">Folder</span></div>
        <div class="info-row"><span class="info-label">Size</span><span class="info-val">2.4 MB</span></div>
        <div class="info-row"><span class="info-label">Items</span><span class="info-val">24</span></div>
        <div class="info-row"><span class="info-label">Created</span><span class="info-val">Mar 12, 2026</span></div>
        <div class="info-row"><span class="info-label">Modified</span><span class="info-val">May 10, 2026</span></div>
        <div class="info-row"><span class="info-label">Permissions</span><span class="info-val mono">drwxr-xr-x</span></div>
        <div class="info-sep"></div>
        <div class="info-tags-header">Tags</div>
        <div class="info-tags">
          {#each ['Work', 'Active', 'UI'] as tag}
            <span class="tag-chip" style="--tc: {tagColors[tag] ?? 'var(--accent)'}">
              <span class="tag-dot" style="background: var(--tc)"></span>
              {tag}
            </span>
          {/each}
        </div>
      </div>
    </Card>

  </div>

  <!-- Inline elements -->
  <div class="cols">

    <!-- SH-6: Tag Chip -->
    <Card id="SH-6" label="Tag Chip">
      <div class="tag-wrap">
        {#each ['Work', 'Personal', 'Archive', 'Urgent', 'Media', 'Design'] as tag}
          <span class="tag-chip" style="--tc: {tagColors[tag] ?? 'var(--accent)'}">
            <span class="tag-dot" style="background: var(--tc)"></span>
            {tag}
          </span>
        {/each}
        <button class="tag-new">+ New Tag</button>
      </div>
    </Card>

    <!-- SH-7: Context Menu -->
    <Card id="SH-7" label="Context Menu">
      <div class="ctx-menu-mock" role="menu" aria-label="Context menu">
        <div class="ctx-item" role="menuitem">Open</div>
        <div class="ctx-item" role="menuitem">Open With ▸</div>
        <div class="ctx-item" role="menuitem">Get Info</div>
        <div class="ctx-sep" role="separator"></div>
        <div class="ctx-item" role="menuitem">Copy</div>
        <div class="ctx-item" role="menuitem">Cut</div>
        <div class="ctx-item" role="menuitem">Paste</div>
        <div class="ctx-sep" role="separator"></div>
        <div class="ctx-item ctx-destructive" role="menuitem">Move to Trash</div>
      </div>
    </Card>

  </div>

</div>

<style>
  /* ── Layout ─────────────────────────────────────────────────────────────── */
  .section { max-width: 1375px; display: flex; flex-direction: column; gap: 20px; }
  .row     { width: 100%; }
  .cols    { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── Shared helpers ─────────────────────────────────────────────────────── */
  .section-header {
    padding: 6px 10px;
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    border-bottom: 1px solid var(--border-subtle);
  }

  .file-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    height: 28px;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-primary);
    user-select: none;
  }
  .file-row:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
  }
  .file-row.selected {
    background: color-mix(in srgb, var(--accent) 12%, var(--surface));
    color: var(--text-primary);
  }

  .file-icon {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    color: var(--text-muted);
    display: flex;
    align-items: center;
  }
  .file-name  { flex: 1; min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-size: 13px; }
  .file-arrow { color: var(--text-muted); font-size: 12px; }
  .file-size  { color: var(--text-muted); font-size: 11px; }

  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 4px 12px;
    font-size: 11px;
  }
  .info-label { color: var(--text-muted); font-size: 10px; }
  .info-val   { color: var(--text-secondary); }
  .info-val.mono { font-family: 'Geist Mono', monospace; font-size: 10px; }

  /* ── SH-1: Path Bar ─────────────────────────────────────────────────────── */
  .path-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    height: 36px;
    gap: 8px;
  }

  .path-nav {
    display: flex;
    align-items: center;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .nav-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    line-height: 1;
    color: var(--text-secondary);
    padding: 2px 5px;
    border-radius: 4px;
    transition: background 0.1s, color 0.1s;
  }
  .nav-btn:hover {
    background: color-mix(in srgb, var(--surface) 90%, var(--text-primary));
    color: var(--text-primary);
  }

  .nav-sep {
    width: 1px;
    height: 16px;
    background: var(--border);
    margin: 0 6px;
    flex-shrink: 0;
  }

  .breadcrumbs {
    display: flex;
    align-items: center;
    gap: 0;
    min-width: 0;
    overflow: hidden;
  }

  .breadcrumb-seg {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 12px;
    padding: 2px 4px;
    color: var(--text-secondary);
    font-family: inherit;
    border-radius: 3px;
    white-space: nowrap;
    transition: background 0.1s, color 0.1s;
  }
  .breadcrumb-seg:hover {
    background: color-mix(in srgb, var(--surface) 90%, var(--text-primary));
    color: var(--text-primary);
  }
  .breadcrumb-seg.current {
    color: var(--text-primary);
    font-weight: 600;
  }

  .bc-div {
    color: var(--text-muted);
    font-size: 12px;
    margin: 0 1px;
    user-select: none;
  }

  .path-right {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-shrink: 0;
  }

  .search-compact {
    display: flex;
    align-items: center;
    gap: 6px;
    border: 1px solid var(--border);
    border-radius: 6px;
    background: var(--surface);
    padding: 4px 8px;
    height: 28px;
    width: 140px;
  }
  .search-compact:focus-within {
    box-shadow: inset 0 -2px 0 0 var(--accent);
  }
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

  .view-btns {
    display: flex;
    align-items: center;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--surface-raised);
  }
  .view-btn {
    background: transparent;
    border: none;
    border-right: 1px solid var(--border);
    cursor: pointer;
    padding: 5px 7px;
    color: var(--text-muted);
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.1s, color 0.1s;
  }
  .view-btn:last-child { border-right: none; }
  .view-btn:hover { background: color-mix(in srgb, var(--surface-raised) 80%, var(--text-primary)); color: var(--text-primary); }
  .view-btn.active { background: color-mix(in srgb, var(--accent) 12%, var(--surface-raised)); color: var(--accent); }

  /* ── SH-2: File Browser ─────────────────────────────────────────────────── */
  .browser {
    display: flex;
    width: 100%;
    height: 320px;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    background: var(--surface);
  }

  .col-pane {
    display: flex;
    flex-direction: column;
    width: 220px;
    flex-shrink: 0;
    border-right: 1px solid var(--border);
    overflow-y: auto;
  }

  .col-preview {
    flex: 1;
    min-width: 0;
    border-right: none;
  }

  .preview-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    gap: 8px;
    padding: 20px;
    color: var(--text-secondary);
    text-align: center;
  }

  .preview-icon { flex-shrink: 0; }
  .preview-name { font-size: 14px; font-weight: 600; color: var(--text-primary); margin: 0; }
  .preview-meta { font-size: 12px; color: var(--text-secondary); margin: 0; }

  /* ── SH-3: Quick Files ──────────────────────────────────────────────────── */
  .quick-files {
    width: 200px;
    display: flex;
    flex-direction: column;
  }

  .qf-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px 6px;
    border-bottom: 1px solid var(--border-subtle);
  }
  .qf-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    padding: 2px;
    border-radius: 3px;
    display: flex;
    align-items: center;
    transition: color 0.1s;
  }
  .icon-btn:hover { color: var(--text-primary); }

  .qf-group-label {
    padding: 6px 10px 3px;
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .qf-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 10px;
    cursor: pointer;
    transition: background 0.1s;
  }
  .qf-row:hover {
    background: color-mix(in srgb, var(--surface) 94%, var(--text-primary));
  }
  .qf-icon { color: var(--accent); opacity: 0.7; }
  .qf-name {
    flex: 1;
    font-size: 12px;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .qf-date {
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
  }

  /* ── SH-4: Preview Panel ─────────────────────────────────────────────────── */
  .preview-panel {
    width: 280px;
    display: flex;
    flex-direction: column;
  }
  .preview-body {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 20px 16px;
  }
  .preview-icon-wrap { margin-bottom: 4px; }
  .preview-filename {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin: 0;
  }
  .preview-typemeta {
    font-size: 12px;
    color: var(--text-secondary);
    margin: 0;
  }
  .open-btn {
    width: 100%;
    padding: 6px 0;
    background: var(--accent);
    color: #fff;
    border: none;
    border-radius: 6px;
    font-family: inherit;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    margin-top: 4px;
    transition: opacity 0.1s;
  }
  .open-btn:hover { opacity: 0.88; }
  .finder-link {
    font-size: 12px;
    color: var(--accent);
    cursor: pointer;
    text-decoration: none;
  }
  .finder-link:hover { text-decoration: underline; }

  /* ── SH-5: File Info ─────────────────────────────────────────────────────── */
  .file-info {
    width: 220px;
    display: flex;
    flex-direction: column;
  }
  .info-sep {
    height: 1px;
    background: var(--border-subtle);
    margin: 6px 0;
  }
  .info-tags-header {
    padding: 4px 12px;
    font-size: 10px;
    font-weight: 600;
    color: var(--text-muted);
  }
  .info-tags {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    padding: 4px 12px 10px;
  }

  /* ── Tag chip (shared SH-5 + SH-6) ─────────────────────────────────────── */
  .tag-chip {
    display: inline-flex;
    align-items: center;
    gap: 5px;
    padding: 2px 8px 2px 6px;
    background: color-mix(in srgb, var(--tc, var(--accent)) 10%, var(--surface-raised));
    border: 1px solid color-mix(in srgb, var(--tc, var(--accent)) 30%, transparent);
    border-radius: 10px;
    font-size: 11px;
    font-weight: 500;
    color: var(--text-primary);
    cursor: default;
    user-select: none;
  }
  .tag-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  /* ── SH-6: Tag wrap ─────────────────────────────────────────────────────── */
  .tag-wrap {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
    padding: 4px 0;
  }
  .tag-new {
    display: inline-flex;
    align-items: center;
    padding: 2px 8px;
    background: transparent;
    border: 1px dashed var(--border);
    border-radius: 10px;
    font-family: inherit;
    font-size: 11px;
    color: var(--text-muted);
    cursor: pointer;
    transition: border-color 0.1s, color 0.1s;
  }
  .tag-new:hover {
    border-color: var(--accent);
    color: var(--accent);
  }

  /* ── SH-7: Context Menu mock ─────────────────────────────────────────────── */
  .ctx-menu-mock {
    width: 180px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    box-shadow: 0 4px 16px rgb(0 0 0 / 14%);
    padding: 4px 0;
    font-size: 13px;
  }
  .ctx-item {
    padding: 5px 14px;
    cursor: default;
    color: var(--text-primary);
    user-select: none;
    transition: background 0.08s;
  }
  .ctx-item:hover {
    background: color-mix(in srgb, var(--surface) 86%, var(--text-primary));
  }
  .ctx-destructive {
    color: #c0392b;
  }
  :global(html.dark) .ctx-destructive {
    color: #e74c3c;
  }
  .ctx-sep {
    height: 1px;
    background: var(--border);
    margin: 3px 0;
  }
</style>
