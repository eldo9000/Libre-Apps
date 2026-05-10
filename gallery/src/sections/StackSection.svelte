<script>
  import { SegmentedControl } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let activeDocTab = $state('notes');
  let editorMode = $state('edit');
  let activeHeading = $state('overview');
  let searchQuery = $state('');
  let replaceQuery = $state('');
  let boldActive = $state(false);
  let italicActive = $state(false);
  let treeExpanded = $state({ notes: true, docs: false, archive: false });

  const editorModeOptions = [
    { value: 'edit', label: 'Edit' },
    { value: 'split', label: 'Split' },
    { value: 'preview', label: 'Preview' },
  ];

  const editorLines = [
    { type: 'h1',   text: '# Libre Apps — Architecture Notes' },
    { type: 'empty', text: '' },
    { type: 'h2',   text: '## Overview' },
    { type: 'empty', text: '' },
    { type: 'body', text: 'The shared foundation provides a consistent design system across all five apps.' },
    { type: 'empty', text: '' },
    { type: 'h2',   text: '## Components' },
    { type: 'empty', text: '' },
    { type: 'body', text: 'Every component uses CSS custom properties for theming.' },
    { type: 'empty', text: '' },
    { type: 'h3',   text: '### Button' },
    { type: 'empty', text: '' },
    { type: 'body', text: 'Primary, secondary, and ghost variants.' },
    { type: 'empty', text: '' },
    { type: 'h2',   text: '## Token System' },
    { type: 'empty', text: '' },
    { type: 'body', text: 'Design tokens are the single source of truth for all visual values.' },
  ];

  // Find the index of the last non-empty line for cursor placement
  const lastContentLineIndex = editorLines.reduce((acc, line, i) => {
    return line.type !== 'empty' ? i : acc;
  }, 0);

  const outlineItems = [
    { id: 'arch-notes', level: 1, badge: 'H1', label: 'Libre Apps — Architecture Notes', indent: 0 },
    { id: 'overview', level: 2, badge: 'H2', label: 'Overview', indent: 1 },
    { id: 'components', level: 2, badge: 'H2', label: 'Components', indent: 1 },
    { id: 'button', level: 3, badge: 'H3', label: 'Button', indent: 2 },
    { id: 'token-system', level: 2, badge: 'H2', label: 'Token System', indent: 1 },
  ];

  const docTabs = [
    { id: 'notes', label: 'notes.md' },
    { id: 'roadmap', label: 'roadmap.md' },
    { id: 'ideas', label: 'ideas.md' },
  ];

  const treeFiles = [
    { id: 'notes', label: 'notes.md', active: true, unsaved: false },
    { id: 'roadmap', label: 'roadmap.md', active: false, unsaved: false },
    { id: 'ideas', label: 'ideas.md', active: false, unsaved: false },
    { id: 'scratch', label: 'scratch.md', active: false, unsaved: true },
  ];

  // Line numbers — count non-empty lines
  let lineNum = 0;
  const numberedLines = editorLines.map((line) => {
    if (line.type !== 'empty') {
      lineNum += 1;
      return { ...line, num: lineNum };
    }
    return { ...line, num: null };
  });
</script>

<div class="section">

  <!-- STK-1 Editor Toolbar -->
  <div class="row">
    <Card id="STK-1" label="Editor Toolbar">
      <div class="toolbar-wrap">
        <!-- Doc tabs -->
        <div class="doc-tabs">
          {#each docTabs as tab}
            <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
            <div
              class="doc-tab"
              class:active={activeDocTab === tab.id}
              onclick={() => (activeDocTab = tab.id)}
              role="tab"
              tabindex="0"
            >
              <svg width="11" height="11" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true" style="opacity:0.6;flex-shrink:0">
                <rect x="1" y="0.5" width="10" height="11" rx="1.5" fill="none" stroke="currentColor" stroke-width="1"/>
                <line x1="3" y1="4" x2="9" y2="4" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
                <line x1="3" y1="6" x2="9" y2="6" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
                <line x1="3" y1="8" x2="7" y2="8" stroke="currentColor" stroke-width="1" stroke-linecap="round"/>
              </svg>
              {tab.label}
              <button class="tab-close" type="button" aria-label="Close {tab.label}">×</button>
            </div>
          {/each}
        </div>

        <!-- Formatting toolbar -->
        <div class="fmt-toolbar">
          <button
            class="toolbar-btn"
            class:active={boldActive}
            type="button"
            onclick={() => (boldActive = !boldActive)}
            aria-label="Bold"
            title="Bold"
          >
            <strong style="font-size:13px;font-family:serif;">B</strong>
          </button>
          <button
            class="toolbar-btn"
            class:active={italicActive}
            type="button"
            onclick={() => (italicActive = !italicActive)}
            aria-label="Italic"
            title="Italic"
          >
            <em style="font-size:13px;font-family:serif;">I</em>
          </button>
          <button class="toolbar-btn" type="button" aria-label="Heading" title="Heading">
            <svg width="13" height="13" viewBox="0 0 13 13" fill="currentColor" aria-hidden="true">
              <text x="0" y="11" font-size="11" font-weight="700" font-family="serif">H</text>
            </svg>
          </button>
          <div class="toolbar-sep"></div>
          <button class="toolbar-btn" type="button" aria-label="Unordered list" title="List">
            <svg width="13" height="13" viewBox="0 0 13 13" fill="currentColor" aria-hidden="true">
              <circle cx="2" cy="4" r="1.2"/>
              <rect x="5" y="3.2" width="7" height="1.5" rx="0.75"/>
              <circle cx="2" cy="9" r="1.2"/>
              <rect x="5" y="8.2" width="7" height="1.5" rx="0.75"/>
            </svg>
          </button>
          <button class="toolbar-btn" type="button" aria-label="Ordered list" title="Ordered list">
            <svg width="13" height="13" viewBox="0 0 13 13" fill="currentColor" aria-hidden="true">
              <text x="0" y="5.5" font-size="5" font-family="monospace">1.</text>
              <rect x="5" y="3.2" width="7" height="1.5" rx="0.75"/>
              <text x="0" y="10.5" font-size="5" font-family="monospace">2.</text>
              <rect x="5" y="8.2" width="7" height="1.5" rx="0.75"/>
            </svg>
          </button>
          <button class="toolbar-btn" type="button" aria-label="Code" title="Inline code">
            <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <polyline points="4,3 1,6.5 4,10"/>
              <polyline points="9,3 12,6.5 9,10"/>
            </svg>
          </button>
          <button class="toolbar-btn" type="button" aria-label="Link" title="Link">
            <svg width="13" height="13" viewBox="0 0 13 13" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <path d="M5 8l3-3"/>
              <path d="M3.5 9.5a2.5 2.5 0 0 1 0-3.5l1-1a2.5 2.5 0 0 1 3.5 3.5l-.5.5"/>
              <path d="M6.5 7.5a2.5 2.5 0 0 1 3.5-3.5l1 1a2.5 2.5 0 0 1-3.5 3.5"/>
            </svg>
          </button>
        </div>

        <!-- Mode switcher -->
        <div class="mode-switcher">
          <SegmentedControl
            options={editorModeOptions}
            bind:value={editorMode}
            variant="sliding"
            size="sm"
          />
        </div>
      </div>
    </Card>
  </div>

  <!-- Main layout -->
  <div class="cols">

    <!-- STK-2 Document Tree -->
    <Card id="STK-2" label="Document Tree">
      <div class="tree-panel">
        <div class="tree-header">
          <span class="panel-label">Documents</span>
          <button class="icon-btn" type="button" aria-label="New file" title="New file">
            <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true">
              <rect x="5.25" y="1" width="1.5" height="10" rx="0.75"/>
              <rect x="1" y="5.25" width="10" height="1.5" rx="0.75"/>
            </svg>
          </button>
        </div>

        <!-- Notes section -->
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div
          class="tree-section-header"
          onclick={() => (treeExpanded.notes = !treeExpanded.notes)}
          role="button"
          tabindex="0"
        >
          <svg
            class="chevron"
            class:expanded={treeExpanded.notes}
            width="10" height="10" viewBox="0 0 10 10" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
          >
            <polyline points="3,3 6,6 9,3"/>
          </svg>
          <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor" aria-hidden="true" style="color:var(--accent);opacity:0.8">
            <path d="M1 2.5A1.5 1.5 0 0 1 2.5 1h4l3 3v6.5A1.5 1.5 0 0 1 8 12H2.5A1.5 1.5 0 0 1 1 10.5z" fill="none" stroke="currentColor" stroke-width="1"/>
            <polyline points="6.5,1 6.5,4 9.5,4" fill="none" stroke="currentColor" stroke-width="1"/>
          </svg>
          <span class="tree-section-label">Notes</span>
        </div>
        {#if treeExpanded.notes}
          {#each treeFiles as file}
            <div
              class="tree-row"
              class:active={file.active}
              role="button"
              tabindex="0"
            >
              <span style="width:16px;flex-shrink:0"></span>
              <svg width="11" height="11" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1" aria-hidden="true" style="opacity:0.5;flex-shrink:0">
                <rect x="2" y="0.5" width="8" height="11" rx="1.5"/>
                <line x1="4" y1="4" x2="8" y2="4"/>
                <line x1="4" y1="6" x2="8" y2="6"/>
                <line x1="4" y1="8" x2="7" y2="8"/>
              </svg>
              <span class="tree-filename">{file.label}</span>
              {#if file.unsaved}
                <span class="unsaved-dot" aria-label="Unsaved changes"></span>
              {/if}
            </div>
          {/each}
        {/if}

        <!-- Docs section -->
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div
          class="tree-section-header"
          onclick={() => (treeExpanded.docs = !treeExpanded.docs)}
          role="button"
          tabindex="0"
        >
          <svg
            class="chevron"
            class:expanded={treeExpanded.docs}
            width="10" height="10" viewBox="0 0 10 10" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
          >
            <polyline points="3,3 6,6 9,3"/>
          </svg>
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1" aria-hidden="true" style="color:var(--text-muted)">
            <path d="M1 2.5A1.5 1.5 0 0 1 2.5 1h4l3 3v6.5A1.5 1.5 0 0 1 8 12H2.5A1.5 1.5 0 0 1 1 10.5z"/>
            <polyline points="6.5,1 6.5,4 9.5,4"/>
          </svg>
          <span class="tree-section-label">Docs</span>
        </div>

        <!-- Archive section -->
        <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
        <div
          class="tree-section-header"
          onclick={() => (treeExpanded.archive = !treeExpanded.archive)}
          role="button"
          tabindex="0"
        >
          <svg
            class="chevron"
            class:expanded={treeExpanded.archive}
            width="10" height="10" viewBox="0 0 10 10" fill="none"
            stroke="currentColor" stroke-width="1.5" stroke-linecap="round" aria-hidden="true"
          >
            <polyline points="3,3 6,6 9,3"/>
          </svg>
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none" stroke="currentColor" stroke-width="1" aria-hidden="true" style="color:var(--text-muted)">
            <rect x="0.5" y="2.5" width="11" height="9" rx="1"/>
            <path d="M0.5 5h11"/>
            <rect x="4" y="6.5" width="4" height="2" rx="0.5"/>
          </svg>
          <span class="tree-section-label">Archive</span>
        </div>

        <div class="tree-footer">4 files · 12 KB</div>
      </div>
    </Card>

    <!-- STK-3 Markdown Editor -->
    <Card id="STK-3" label="Markdown Editor">
      <div class="editor-panel">
        <div class="editor-inner">
          <div class="gutter">
            {#each numberedLines as line}
              {#if line.num !== null}
                <div class="line-num">{line.num}</div>
              {:else}
                <div class="line-num-empty"></div>
              {/if}
            {/each}
          </div>
          <div class="editor-content">
            {#each editorLines as line, i}
              {#if line.type === 'empty'}
                <div class="editor-spacer"></div>
              {:else if line.type === 'h1'}
                <div class="editor-line el-h1">{line.text}{#if i === lastContentLineIndex}<span class="cursor">|</span>{/if}</div>
              {:else if line.type === 'h2'}
                <div class="editor-line el-h2">{line.text}{#if i === lastContentLineIndex}<span class="cursor">|</span>{/if}</div>
              {:else if line.type === 'h3'}
                <div class="editor-line el-h3">{line.text}{#if i === lastContentLineIndex}<span class="cursor">|</span>{/if}</div>
              {:else}
                <div class="editor-line el-body">{line.text}{#if i === lastContentLineIndex}<span class="cursor">|</span>{/if}</div>
              {/if}
            {/each}
          </div>
        </div>
      </div>
    </Card>

    <!-- STK-4 Preview Pane -->
    <Card id="STK-4" label="Preview Pane">
      <div class="preview-panel">
        <div class="preview-label">Rendered Preview</div>
        <div class="preview-content">
          {#each editorLines as line}
            {#if line.type === 'h1'}
              <h1 class="preview-h1">{line.text.replace(/^#\s*/, '')}</h1>
            {:else if line.type === 'h2'}
              <h2 class="preview-h2">{line.text.replace(/^##\s*/, '')}</h2>
            {:else if line.type === 'h3'}
              <h3 class="preview-h3">{line.text.replace(/^###\s*/, '')}</h3>
            {:else if line.type === 'body'}
              <p class="preview-p">{line.text}</p>
            {/if}
          {/each}
        </div>
      </div>
    </Card>

  </div>

  <!-- Outline + search -->
  <div class="cols">

    <!-- STK-5 Outline Panel -->
    <Card id="STK-5" label="Outline Panel">
      <div class="outline-panel">
        <div class="panel-label" style="margin-bottom:8px;">Outline</div>
        {#each outlineItems as item}
          <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
          <div
            class="outline-row"
            class:active={activeHeading === item.id}
            onclick={() => (activeHeading = item.id)}
            role="button"
            tabindex="0"
            style="padding-left:{8 + item.indent * 14}px"
          >
            <span class="h-badge">{item.badge}</span>
            <span class="outline-label">{item.label}</span>
          </div>
        {/each}
      </div>
    </Card>

    <!-- STK-6 Search Bar -->
    <Card id="STK-6" label="Search Bar">
      <div class="find-replace">
        <div class="panel-label" style="margin-bottom:6px;">Find &amp; Replace</div>
        <div class="find-replace-box">
          <!-- Row 1: Find -->
          <div class="fr-row">
            <svg class="fr-icon" width="13" height="13" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <circle cx="11" cy="11" r="7"/>
              <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            </svg>
            <input
              type="search"
              class="fr-input"
              placeholder="Find…"
              bind:value={searchQuery}
            />
            {#if searchQuery}
              <span class="match-badge">3 of 12</span>
            {/if}
            <button class="fr-nav-btn" type="button" aria-label="Previous match">↑</button>
            <button class="fr-nav-btn" type="button" aria-label="Next match">↓</button>
            {#if searchQuery}
              <button class="fr-nav-btn" type="button" onclick={() => (searchQuery = '')} aria-label="Clear search">×</button>
            {/if}
          </div>
          <!-- Row 2: Replace -->
          <div class="fr-row">
            <svg class="fr-icon" width="13" height="13" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
              <polyline points="17 1 21 5 17 9"/>
              <path d="M3 11V9a4 4 0 0 1 4-4h14"/>
              <polyline points="7 23 3 19 7 15"/>
              <path d="M21 13v2a4 4 0 0 1-4 4H3"/>
            </svg>
            <input
              type="text"
              class="fr-input"
              placeholder="Replace…"
              bind:value={replaceQuery}
            />
            <button class="fr-replace-btn" type="button">Replace</button>
            <button class="fr-replace-btn" type="button">All</button>
          </div>
        </div>

        <!-- Options row -->
        <div class="fr-options">
          <label class="fr-option-label">
            <input type="checkbox" class="fade-check" />
            Match case
          </label>
          <label class="fr-option-label">
            <input type="checkbox" class="fade-check" />
            Whole word
          </label>
          <label class="fr-option-label">
            <input type="checkbox" class="fade-check" />
            Regex
          </label>
        </div>
      </div>
    </Card>

  </div>

</div>

<style>
  .section { max-width: 1100px; display: flex; flex-direction: column; gap: 20px; }
  .row { width: 100%; }
  .cols { display: flex; flex-wrap: wrap; gap: 20px; align-items: flex-start; }

  /* ── STK-1 Toolbar ───────────────────────────────────────────────── */
  .toolbar-wrap {
    display: flex;
    align-items: center;
    gap: 0;
    width: 100%;
    min-height: 44px;
    background: var(--surface-panel);
    border-radius: 6px;
    padding: 0 8px;
    box-sizing: border-box;
    overflow: hidden;
  }

  .doc-tabs {
    display: flex;
    align-items: flex-end;
    gap: 2px;
    flex-shrink: 0;
    padding-bottom: 0;
    margin-right: 12px;
  }

  .doc-tab {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 4px 10px;
    font-size: 12px;
    border-radius: 6px 6px 0 0;
    cursor: pointer;
    user-select: none;
    border: 1px solid transparent;
    color: var(--text-muted);
    white-space: nowrap;
  }
  .doc-tab.active {
    background: var(--surface);
    color: var(--text-primary);
    border-color: var(--border);
    border-bottom-color: transparent;
  }
  .doc-tab:not(.active):hover {
    color: var(--text-secondary);
    background: color-mix(in srgb, var(--surface-raised) 60%, transparent);
  }

  .tab-close {
    background: none;
    border: none;
    padding: 0 0 0 2px;
    font-size: 13px;
    line-height: 1;
    cursor: pointer;
    color: var(--text-muted);
    opacity: 0.6;
    flex-shrink: 0;
  }
  .tab-close:hover { opacity: 1; color: var(--text-primary); }

  .fmt-toolbar {
    display: flex;
    align-items: center;
    gap: 1px;
    flex: 1;
    justify-content: center;
  }

  .toolbar-sep {
    width: 1px;
    height: 16px;
    background: var(--border);
    margin: 0 4px;
    flex-shrink: 0;
  }

  .toolbar-btn {
    width: 26px;
    height: 26px;
    background: none;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .toolbar-btn:hover { background: var(--surface-raised); }
  .toolbar-btn.active { color: var(--accent); }

  .mode-switcher {
    flex-shrink: 0;
    margin-left: 8px;
  }

  /* ── STK-2 Document Tree ─────────────────────────────────────────── */
  .tree-panel {
    width: 200px;
    min-height: 400px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .tree-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 10px 6px;
    border-bottom: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  .panel-label {
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
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border-radius: 4px;
    transition: background 0.1s, color 0.1s;
  }
  .icon-btn:hover { background: var(--surface-raised); color: var(--text-primary); }

  .tree-section-header {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 5px 8px;
    cursor: pointer;
    user-select: none;
    font-size: 11px;
    font-weight: 600;
    color: var(--text-secondary);
  }
  .tree-section-header:hover {
    background: color-mix(in srgb, var(--surface-raised) 70%, transparent);
  }

  .chevron {
    transition: transform 0.15s ease;
    color: var(--text-muted);
    flex-shrink: 0;
    transform: rotate(-90deg);
  }
  .chevron.expanded { transform: rotate(0deg); }

  .tree-section-label { flex: 1; }

  .tree-row {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    height: 28px;
    cursor: pointer;
    font-size: 12px;
    user-select: none;
    color: var(--text-secondary);
    box-sizing: border-box;
    border-left: 2px solid transparent;
  }
  .tree-row:hover {
    background: color-mix(in srgb, var(--surface-raised) 70%, transparent);
  }
  .tree-row.active {
    border-left-color: var(--accent);
    padding-left: 6px;
    color: var(--text-primary);
  }

  .tree-filename { flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }

  .unsaved-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent);
    flex-shrink: 0;
    opacity: 0.7;
  }

  .tree-footer {
    margin-top: auto;
    padding: 6px 10px;
    font-size: 10px;
    color: var(--text-muted);
    border-top: 1px solid var(--border-subtle);
    flex-shrink: 0;
  }

  /* ── STK-3 Markdown Editor ───────────────────────────────────────── */
  .editor-panel {
    width: 400px;
    height: 400px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .editor-inner {
    display: flex;
    flex: 1;
    overflow-y: auto;
    font-family: 'Geist Mono', 'Fira Code', monospace;
    font-size: 13px;
    padding: 12px 0;
  }

  .gutter {
    width: 32px;
    flex-shrink: 0;
    padding-top: 0;
  }

  .line-num {
    font-family: 'Geist Mono', 'Fira Code', monospace;
    font-size: 10px;
    color: var(--text-muted);
    text-align: right;
    padding-right: 8px;
    user-select: none;
    min-width: 28px;
    line-height: 1.7;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: flex-end;
    box-sizing: border-box;
  }

  .line-num-empty {
    height: 8px;
  }

  .editor-content {
    flex: 1;
    padding-right: 12px;
    min-width: 0;
  }

  .editor-line {
    line-height: 1.7;
    white-space: pre-wrap;
    word-break: break-all;
  }

  .editor-spacer { height: 8px; }

  .el-h1 { font-size: 15px; font-weight: 700; color: var(--text-primary); }
  .el-h2 { font-size: 14px; font-weight: 600; color: var(--text-primary); }
  .el-h3 { font-size: 13px; font-weight: 600; color: var(--text-secondary); }
  .el-body { font-size: 13px; color: var(--text-secondary); }

  .cursor {
    color: var(--accent);
    animation: blink 1s step-end infinite;
    font-weight: 300;
  }

  @keyframes blink {
    from { opacity: 1; }
    50%  { opacity: 0; }
    to   { opacity: 1; }
  }

  /* ── STK-4 Preview Pane ──────────────────────────────────────────── */
  .preview-panel {
    width: 360px;
    height: 400px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .preview-label {
    font-size: 9px;
    font-weight: 700;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    padding: 8px 20px 0;
    flex-shrink: 0;
  }

  .preview-content {
    flex: 1;
    padding: 12px 20px 20px;
    overflow-y: auto;
  }

  .preview-h1 {
    font-size: 20px;
    font-weight: 700;
    color: var(--text-primary);
    border-bottom: 1px solid var(--border);
    padding-bottom: 8px;
    margin: 0 0 12px;
  }

  .preview-h2 {
    font-size: 16px;
    font-weight: 700;
    color: var(--text-primary);
    margin: 20px 0 6px;
  }

  .preview-h3 {
    font-size: 14px;
    font-weight: 700;
    color: var(--text-secondary);
    margin: 12px 0 4px;
  }

  .preview-p {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
    margin: 0 0 8px;
  }

  /* ── STK-5 Outline Panel ─────────────────────────────────────────── */
  .outline-panel {
    width: 220px;
    min-height: 280px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 8px 0;
    display: flex;
    flex-direction: column;
  }

  .outline-panel > .panel-label {
    padding: 0 10px;
  }

  .outline-row {
    display: flex;
    align-items: center;
    gap: 6px;
    height: 26px;
    cursor: pointer;
    font-size: 12px;
    color: var(--text-secondary);
    user-select: none;
    border-left: 2px solid transparent;
    padding-right: 8px;
    box-sizing: border-box;
  }
  .outline-row:hover {
    background: color-mix(in srgb, var(--surface-raised) 70%, transparent);
  }
  .outline-row.active {
    border-left-color: var(--accent);
    color: var(--text-primary);
  }

  .outline-label {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-size: 12px;
  }

  .h-badge {
    font-family: 'Geist Mono', 'Fira Code', monospace;
    font-size: 8px;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 2px;
    padding: 1px 3px;
    color: var(--text-muted);
    flex-shrink: 0;
    line-height: 1.4;
  }

  /* ── STK-6 Find & Replace ────────────────────────────────────────── */
  .find-replace {
    width: 320px;
    padding: 4px 0;
  }

  .find-replace-box {
    border: 1px solid var(--border);
    border-radius: 8px;
    background: var(--surface-raised);
    padding: 8px 10px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 8px;
  }

  .fr-row {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .fr-icon {
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .fr-input {
    flex: 1;
    min-width: 0;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 3px 6px;
    font-size: 12px;
    font-family: inherit;
    color: var(--text-primary);
    outline: none;
    transition: border-color 0.1s;
  }
  .fr-input:focus { border-color: var(--accent); }
  .fr-input::placeholder { color: var(--text-muted); }
  .fr-input::-webkit-search-cancel-button { display: none; }

  .match-badge {
    font-size: 10px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .fr-nav-btn {
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-muted);
    font-size: 12px;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 3px;
    flex-shrink: 0;
    transition: background 0.1s, color 0.1s;
  }
  .fr-nav-btn:hover { background: var(--surface-raised); color: var(--text-primary); }

  .fr-replace-btn {
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 2px 8px;
    font-size: 11px;
    font-family: inherit;
    color: var(--text-secondary);
    cursor: pointer;
    white-space: nowrap;
    transition: background 0.1s, border-color 0.1s;
    flex-shrink: 0;
  }
  .fr-replace-btn:hover { background: var(--surface-raised); border-color: var(--accent); color: var(--text-primary); }

  .fr-options {
    display: flex;
    align-items: center;
    gap: 12px;
    flex-wrap: wrap;
  }

  .fr-option-label {
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 11px;
    color: var(--text-secondary);
    cursor: pointer;
    user-select: none;
  }
</style>
