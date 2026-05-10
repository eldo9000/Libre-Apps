<script>
  import { Select, TrafficLight } from '@libre/ui';

  const HOTKEY_KEY_ITEMS = [
    { value: 'option',          label: 'Option ⌥'  },
    { value: 'control',         label: 'Control ⌃' },
    { value: 'command',         label: 'Command ⌘' },
    { value: 'shift',           label: 'Shift ⇧'   },
    { value: 'numpad_enter',    label: 'Enter'      },
    { value: 'numpad_0',        label: '0'          },
    { value: 'numpad_decimal',  label: '.'          },
    { value: 'numpad_add',      label: '+'          },
    { value: 'numpad_subtract', label: '−'          },
    { value: 'numpad_multiply', label: '*'          },
  ];

  const HISTORY_AUTO_DELETE_ITEMS = [
    { value: 'restart', label: 'For current session' },
    { value: '1d',      label: 'For 1 day'           },
    { value: '5d',      label: 'For 5 days'          },
    { value: '10d',     label: 'For 10 days'         },
    { value: '30d',     label: 'For 30 days'         },
  ];

  let hotkeySide         = $state('right');
  let hotkeyKeyPart      = $state('option');
  let cfgHotkeyMode      = $state('hold');
  let cfgDevice          = $state('default');
  let cfgCancelOnEsc     = $state(true);
  let cfgCancelOnHold    = $state(true);
  let cfgTheme           = $state('auto');
  let cfgSaveHistory     = $state(true);
  let cfgHistoryAutoDelete = $state('10d');
  let cfgSoundOnStart    = $state(false);
  let cfgSoundOnFinish   = $state(false);
  let cfgSoundOnCancel   = $state(false);
  let cfgSoundVolume     = $state(0.7);
  let cfgLaunchLogin     = $state(false);
  let cfgShowOverlay     = $state(true);
  let cfgTranscriptIndicator = $state(true);

  function seg(active, i, total) {
    const base  = 'tt-seg-btn';
    const first = i === 0        ? ' tt-seg-first' : '';
    const last  = i === total - 1 ? ' tt-seg-last'  : '';
    const on    = active          ? ' tt-seg-on'    : '';
    return base + first + last + on;
  }


</script>

<div class="ir">

  <!-- Hotkey -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">Hotkey</span>
    </div>
    <div class="ir-row ir-row-field">
      <div class="tt-seg" data-card="SEG-6">
        {#each [['left','Left'],['right','Right']] as [v, lbl], i}
          <button onclick={() => hotkeySide = v} class={seg(hotkeySide === v, i, 2)}>{lbl}</button>
        {/each}
      </div>
      <div class="ir-key-sel" data-card="SEL-2">
        <Select items={HOTKEY_KEY_ITEMS} bind:value={hotkeyKeyPart} variant="flat" size="sm" />
      </div>
    </div>
  </div>

  <!-- Recording -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">Recording</span>
    </div>
    <div class="ir-row ir-row-field">
      <div class="tt-seg" data-card="SEG-6">
        {#each [['hold','Hold'],['toggle','Toggle']] as [v, lbl], i}
          <button onclick={() => cfgHotkeyMode = v} class={seg(cfgHotkeyMode === v, i, 2)}>{lbl}</button>
        {/each}
      </div>
      <div class="ir-key-sel" data-card="SEL-2">
        <Select items={[{ value: 'default', label: 'System default' }]} bind:value={cfgDevice} variant="flat" size="sm" />
      </div>
    </div>
  </div>

  <!-- Cancel -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">Cancel shortcuts</span>
    </div>
    <div class="ir-row ir-row-field" data-card={cfgCancelOnEsc ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row">
        <input type="checkbox" class="cb-native" bind:checked={cfgCancelOnEsc} />
        <span class="ir-lbl" style="flex:unset">Press Escape</span>
      </label>
    </div>
    <div class="ir-row ir-row-field" data-card={cfgCancelOnHold ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row">
        <input type="checkbox" class="cb-native" bind:checked={cfgCancelOnHold} />
        <span class="ir-lbl" style="flex:unset">Hold trigger key</span>
      </label>
    </div>
  </div>

  <!-- Theme -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">Theme</span>
    </div>
    <div class="ir-row ir-row-field">
      <div class="tt-seg tt-seg-wide" data-card="SEG-6">
        {#each [['auto','Auto'],['light','Light'],['dark','Dark']] as [v, lbl], i}
          <button onclick={() => cfgTheme = v} class={seg(cfgTheme === v, i, 3)}>{lbl}</button>
        {/each}
      </div>
    </div>
  </div>

  <!-- History -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">History</span>
    </div>
    <div class="ir-row ir-row-field" data-card={cfgSaveHistory ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row">
        <input type="checkbox" class="cb-native" bind:checked={cfgSaveHistory} />
        <span class="ir-lbl" style="flex:unset">Save history</span>
      </label>
      <div class="ir-key-sel" data-card={cfgSaveHistory ? 'SEL-2' : 'SEL-3'}>
        <Select items={HISTORY_AUTO_DELETE_ITEMS} bind:value={cfgHistoryAutoDelete} disabled={!cfgSaveHistory} variant="flat" size="sm" />
      </div>
    </div>
  </div>

  <!-- Audio -->
  <div class="ir-section section-ruled">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">Audio indicators</span>
    </div>
    <div class="ir-row ir-row-field">
      <span class="ir-lbl">Play on</span>
      <div class="tt-multi">
        <button onclick={() => cfgSoundOnStart = !cfgSoundOnStart}
                class="tt-multi-btn" class:tt-multi-on={cfgSoundOnStart}
                data-card={cfgSoundOnStart ? 'CHK-6' : 'CHK-5'}>Start</button>
        <button onclick={() => cfgSoundOnFinish = !cfgSoundOnFinish}
                class="tt-multi-btn" class:tt-multi-on={cfgSoundOnFinish}
                data-card={cfgSoundOnFinish ? 'CHK-6' : 'CHK-5'}>Finish</button>
        <button onclick={() => cfgSoundOnCancel = !cfgSoundOnCancel}
                class="tt-multi-btn" class:tt-multi-on={cfgSoundOnCancel}
                data-card={cfgSoundOnCancel ? 'CHK-6' : 'CHK-5'}>Cancel</button>
      </div>
    </div>
    <div class="ir-row ir-row-field ir-row-col" data-card="SLD-2">
      <div class="tt-vol-hd">
        <span class="ir-lbl" style="flex:unset">Volume</span>
        <span class="tt-vol-val">{Math.round(cfgSoundVolume * 100)}%</span>
      </div>
      <input
        type="range"
        min="0" max="1" step="0.01"
        bind:value={cfgSoundVolume}
        class="tt-range"
        style="--pct:{cfgSoundVolume * 100}%"
      />
    </div>
  </div>

  <!-- System -->
  <div class="ir-section section-ruled" style="border-bottom:none">
    <div class="ir-sec-hd subsection-hd" data-card="SUB-3">
      <TrafficLight state="gray" />
      <span class="ir-sec-title subsection-hd-title">System</span>
    </div>
    <div class="ir-row ir-row-field" data-card={cfgLaunchLogin ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row">
        <input type="checkbox" class="cb-native" bind:checked={cfgLaunchLogin} />
        <span class="ir-lbl" style="flex:unset">Launch at login</span>
      </label>
    </div>
    <div class="ir-row ir-row-field" data-card={cfgShowOverlay ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row">
        <input type="checkbox" class="cb-native" bind:checked={cfgShowOverlay} />
        <span class="ir-lbl" style="flex:unset">Active recording overlay</span>
      </label>
    </div>
    <div class="ir-row ir-row-field" data-card={!cfgShowOverlay ? 'CHK-7' : cfgTranscriptIndicator ? 'CHK-6' : 'CHK-5'}>
      <label class="tt-check-row" class:tt-disabled={!cfgShowOverlay}>
        <input type="checkbox" class="cb-native" bind:checked={cfgTranscriptIndicator} disabled={!cfgShowOverlay} />
        <span class="ir-lbl" style="flex:unset">Recording length overlay</span>
      </label>
    </div>
    <div class="ir-row ir-row-field ir-update-row">
      <button class="tt-update-btn">Check for updates</button>
    </div>
  </div>

</div>

<style>
  .ir {
    background: var(--surface);
    color: var(--text-primary);
    font-size: 13px;
    display: flex;
    flex-direction: column;
    min-height: 100%;
  }

  .ir-section {
    border-bottom: 1px solid var(--border);
    padding-bottom: 16px;
  }

  /* .ir-sec-hd and .ir-sec-title styles live in tokens.css as
     .subsection-hd / .subsection-hd-title */
  .ir-sec-hd { width: 100%; }

  .ir-row {
    display: flex;
    align-items: center;
    padding: 4px 12px;
    gap: 6px;
    transition: background 0.1s;
  }
  .ir-row:hover .ir-lbl { color: var(--text-primary); }

  .ir-row-field {
    padding-top: 5px;
    padding-bottom: 5px;
  }

  .ir-key-sel { flex: 1; min-width: 0; margin-left: 6px; }
  /* Fixed-width seg slot so paired rows' dropdowns left-align. */
  .ir-row .tt-seg:not(.tt-seg-wide) { width: 88px; }

  .ir-row-col {
    flex-direction: column;
    align-items: flex-start;
    gap: 5px;
  }

  .ir-lbl {
    flex: 1;
    font-size: 10px;
    color: var(--text-secondary);
  }

  /* Segmented buttons */
  .tt-seg {
    display: flex;
    flex-shrink: 0;
  }
  .tt-seg-wide { width: 100%; }

  .tt-seg-btn {
    flex: 1;
    padding: 2px 6px;
    font-size: 9px;
    font-family: inherit;
    font-weight: 600;
    letter-spacing: 0.04em;
    background: var(--surface-panel);
    border: 1px solid var(--border);
    border-left: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
    white-space: nowrap;
  }
  .tt-seg-first { border-left: 1px solid var(--border); border-radius: 4px 0 0 4px; }
  .tt-seg-last  { border-radius: 0 4px 4px 0; }
  .tt-seg-btn:hover { color: var(--text-primary); background: color-mix(in srgb, var(--surface-panel) 80%, var(--text-primary)); }
  .tt-seg-on {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-panel));
    color: #fff;
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
  }
  .tt-seg-on + .tt-seg-btn { border-left-color: color-mix(in srgb, var(--accent) 40%, var(--border)); }
  :global(html:not(.dark)) .ir .tt-seg-on { color: var(--text-primary); }

  /* Checkbox rows */
  .tt-check-row {
    display: flex;
    align-items: center;
    gap: 6px;
    cursor: pointer;
  }
  .tt-disabled { opacity: 0.4; cursor: not-allowed; }

  /* .cb-native styles live in tokens.css — linked to CHK-5/6/7 */

  .ir-update-row { padding-top: 10px; }
  .tt-update-btn {
    width: 100%;
    padding: 5px 10px;
    font-size: 10px;
    font-family: inherit;
    font-weight: 600;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-panel);
    color: var(--text-secondary);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .tt-update-btn:hover { background: color-mix(in srgb, var(--surface-panel) 80%, var(--text-primary)); color: var(--text-primary); }

  /* Multi-select toggle buttons — independent, spaced apart */
  .tt-multi {
    display: flex;
    gap: 4px;
    flex-shrink: 0;
  }
  .tt-multi-btn {
    padding: 2px 7px;
    font-size: 9px;
    font-family: inherit;
    font-weight: 600;
    letter-spacing: 0.04em;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--surface-panel);
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.1s, color 0.1s;
  }
  .tt-multi-btn:hover { color: var(--text-primary); }
  .tt-multi-on {
    background: color-mix(in srgb, var(--accent) 18%, var(--surface-panel));
    border-color: color-mix(in srgb, var(--accent) 40%, var(--border));
    color: #fff;
  }

  /* Volume */
  .tt-vol-hd {
    width: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .tt-vol-val {
    font-size: 10px;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
  }

  .tt-range {
    width: 100%;
    height: 4px;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(to right, var(--accent) var(--pct, 70%), var(--border) var(--pct, 70%));
    border-radius: 2px;
    cursor: pointer;
    outline: none;
  }
  .tt-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid var(--surface);
    box-shadow: 0 0 0 1px var(--accent);
    cursor: pointer;
  }
</style>
