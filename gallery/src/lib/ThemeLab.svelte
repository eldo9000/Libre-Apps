<script>
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';

  const STORAGE_KEY = 'libre-theme-lab';

  const FONTS = [
    { label: 'Geist',  value: "Geist, -apple-system, BlinkMacSystemFont, 'Segoe UI', Cantarell, sans-serif" },
    { label: 'System', value: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Cantarell, sans-serif" },
    { label: 'Mono',   value: "'Geist Mono', 'SF Mono', Consolas, monospace" },
    { label: 'Serif',  value: "Georgia, 'Times New Roman', serif" },
  ];

  const SIZES = [12, 13, 14, 15, 16];

  const DEFAULTS = {
    accent:     '#003f7d',
    fontFamily: "Geist, -apple-system, BlinkMacSystemFont, 'Segoe UI', Cantarell, sans-serif",
    fontSize:   14,
  };

  let open_state = $state(false);
  let saving     = $state(false);
  let loading    = $state(false);
  let saved      = $state(false);

  let draft = $state(loadDraft());

  const isDirty = $derived(
    draft.accent     !== DEFAULTS.accent ||
    draft.fontFamily !== DEFAULTS.fontFamily ||
    draft.fontSize   !== DEFAULTS.fontSize
  );

  function loadDraft() {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) return { ...DEFAULTS, ...JSON.parse(stored) };
    } catch {}
    return { ...DEFAULTS };
  }

  function darken(hex, amount = 0.13) {
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    return '#' + [r, g, b]
      .map(c => Math.max(0, Math.round(c * (1 - amount))).toString(16).padStart(2, '0'))
      .join('');
  }

  $effect(() => {
    const { accent, fontFamily, fontSize } = draft;
    const root = document.documentElement;
    root.style.setProperty('--accent', accent);
    root.style.setProperty('--accent-hover', darken(accent));
    root.style.fontFamily = fontFamily;
    root.style.fontSize   = fontSize + 'px';
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ accent, fontFamily, fontSize }));
  });

  function presetMarkdown() {
    const data = JSON.stringify({ accent: draft.accent, fontFamily: draft.fontFamily, fontSize: draft.fontSize }, null, 2);
    return `# Libre Theme Preset\n\n\`\`\`json\n${data}\n\`\`\`\n`;
  }

  function parsePreset(text) {
    const match = text.match(/```json\s*([\s\S]*?)```/);
    if (!match) throw new Error('No JSON block found in preset file');
    return JSON.parse(match[1]);
  }

  async function handleSave() {
    saving = true;
    try {
      const path = await save({
        title: 'Save Theme Preset',
        defaultPath: 'libre-theme.md',
        filters: [{ name: 'Markdown', extensions: ['md'] }],
      });
      if (!path) return;

      await invoke('write_preset', { path, content: presetMarkdown() });
      await invoke('save_theme', {
        accent:      draft.accent,
        accentHover: darken(draft.accent),
        fontFamily:  draft.fontFamily,
        fontSize:    draft.fontSize,
      });

      saved = true;
      setTimeout(() => { saved = false; }, 2000);
    } catch (e) {
      console.error('save failed:', e);
    } finally {
      saving = false;
    }
  }

  async function handleLoad() {
    loading = true;
    try {
      const path = await open({
        title: 'Load Theme Preset',
        filters: [{ name: 'Markdown', extensions: ['md'] }],
        multiple: false,
      });
      if (!path) return;

      const content = await invoke('read_preset', { path });
      const parsed  = parsePreset(content);

      if (parsed.accent)     draft.accent     = parsed.accent;
      if (parsed.fontFamily) draft.fontFamily = parsed.fontFamily;
      if (parsed.fontSize)   draft.fontSize   = parsed.fontSize;
    } catch (e) {
      console.error('load failed:', e);
    } finally {
      loading = false;
    }
  }
</script>

<div class="lab">
  <button class="lab-toggle" onclick={() => (open_state = !open_state)}>
    <span class="arrow">{open_state ? '▾' : '▸'}</span>
    <span>Theme Lab</span>
    {#if isDirty}
      <span class="unsaved" title="Unsaved changes"></span>
    {/if}
  </button>

  {#if open_state}
    <div class="lab-body">
      <!-- Accent color -->
      <div class="row">
        <span class="lbl">Accent</span>
        <div class="color-row">
          <input type="color" class="swatch" bind:value={draft.accent} />
          <span class="hex">{draft.accent}</span>
          <span class="hex derived" title="Derived hover">{darken(draft.accent)}</span>
        </div>
      </div>

      <!-- Font family -->
      <div class="row">
        <span class="lbl">Font</span>
        <select class="sel" bind:value={draft.fontFamily}>
          {#each FONTS as f}
            <option value={f.value}>{f.label}</option>
          {/each}
        </select>
      </div>

      <!-- Font size -->
      <div class="row">
        <span class="lbl">Size</span>
        <div class="size-group">
          {#each SIZES as s}
            <button
              class="sz"
              class:sz-active={draft.fontSize === s}
              onclick={() => (draft.fontSize = s)}
            >{s}</button>
          {/each}
        </div>
      </div>

      <!-- Actions -->
      <div class="actions">
        <button class="btn-load" onclick={handleLoad} disabled={loading}>
          {loading ? '···' : 'Load'}
        </button>
        <button class="btn-save" onclick={handleSave} disabled={saving}>
          {saving ? '···' : saved ? '✓' : 'Save'}
        </button>
      </div>
    </div>
  {/if}
</div>

<style>
  .lab {
    border-top: 1px solid var(--border);
  }

  .lab-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 8px 12px;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 12px;
    font-family: inherit;
    color: var(--text-secondary);
    text-align: left;
    transition: color 0.1s;
  }
  .lab-toggle:hover { color: var(--text-primary); }

  .arrow { font-size: 10px; width: 10px; flex-shrink: 0; }

  .unsaved {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: var(--accent);
    margin-left: auto;
    flex-shrink: 0;
  }

  .lab-body {
    padding: 2px 12px 10px;
    display: flex;
    flex-direction: column;
    gap: 7px;
  }

  .row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .lbl {
    font-size: 11px;
    color: var(--text-muted);
    width: 36px;
    flex-shrink: 0;
  }

  .color-row {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
  }

  .swatch {
    appearance: none;
    -webkit-appearance: none;
    width: 22px;
    height: 22px;
    border-radius: 5px;
    border: 1px solid var(--border);
    cursor: pointer;
    padding: 0;
    background: none;
    flex-shrink: 0;
  }
  .swatch::-webkit-color-swatch-wrapper { padding: 0; }
  .swatch::-webkit-color-swatch { border: none; border-radius: 4px; }

  .hex {
    font-size: 10px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
  }
  .derived { opacity: 0.6; }

  .sel {
    flex: 1;
    font-size: 11px;
    font-family: inherit;
    color: var(--text-primary);
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 5px;
    padding: 3px 6px;
    cursor: pointer;
  }

  .size-group { display: flex; gap: 3px; flex: 1; }

  .sz {
    flex: 1;
    padding: 3px 0;
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 4px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: background 0.1s, color 0.1s, border-color 0.1s;
  }
  .sz:hover { color: var(--text-primary); }
  .sz-active { background: var(--accent); border-color: var(--accent); color: #fff; }

  .actions { display: flex; gap: 6px; margin-top: 2px; }

  .btn-load {
    flex: 1;
    padding: 5px 0;
    font-size: 11px;
    font-family: inherit;
    background: var(--surface-raised);
    border: 1px solid var(--border);
    border-radius: 5px;
    cursor: pointer;
    color: var(--text-secondary);
    transition: color 0.1s, border-color 0.1s;
  }
  .btn-load:hover:not(:disabled) { color: var(--text-primary); border-color: var(--text-muted); }
  .btn-load:disabled { opacity: 0.4; cursor: default; }

  .btn-save {
    flex: 1;
    padding: 5px 0;
    font-size: 11px;
    font-family: inherit;
    background: var(--accent);
    border: 1px solid var(--accent);
    border-radius: 5px;
    cursor: pointer;
    color: #fff;
    font-weight: 500;
    transition: opacity 0.1s;
  }
  .btn-save:hover:not(:disabled) { opacity: 0.88; }
  .btn-save:disabled { opacity: 0.5; cursor: default; }
</style>
