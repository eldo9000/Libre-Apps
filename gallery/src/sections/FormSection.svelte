<script>
  import { Input, Select, Checkbox, SegmentedControl, Toggle, Stepper, Slider } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  // Input
  let inputVal  = $state('');
  let inputState = $state('default');
  const inpDisabled = $derived(inputState === 'disabled');
  const inpError    = $derived(inputState === 'error');

  // Select
  let selectVal1 = $state('');
  let selectVal2 = $state('mp4');
  let selectState = $state('default');
  const selDisabled = $derived(selectState === 'disabled');

  // Checkbox
  let chk1 = $state(false);
  let chk2 = $state(true);
  let chk5 = $state(false);
  let chk6 = $state(true);
  let checkboxState = $state('default');
  const chkDisabled = $derived(checkboxState === 'disabled');

  // Slider
  let sld1 = $state(40);
  let sld2 = $state(72);
  let sld4 = $state(40);
  let sld5 = $state(72);
  let sld6 = $state(58);
  let sliderState = $state('default');
  const sldDisabled = $derived(sliderState === 'disabled');
  const sldHover    = $derived(sliderState === 'hover');

  // Segmented Control
  let seg1 = $state('b');
  let seg2 = $state('a');
  let seg3 = $state('b');
  let seg4 = $state('a');
  let seg5 = $state('b');
  let seg6 = $state('a');
  let segState = $state('default');
  const segDisabled = $derived(segState === 'disabled');

  // Toggle
  let tgl1 = $state(false);
  let tgl2 = $state(true);
  let tgl3 = $state(false);
  let tgl4 = $state(true);
  let toggleState = $state('default');
  const tglDisabled = $derived(toggleState === 'disabled');

  // Stepper
  let stp1 = $state(30);
  let stp2 = $state(30);
  let stp3 = $state(0);
  let stepperState = $state('default');
  const stpDisabled = $derived(stepperState === 'disabled');

  const selectItems = [
    { value: 'mp4', label: 'MP4' },
    { value: 'mov', label: 'MOV' },
    { value: 'mkv', label: 'MKV' },
    { value: 'webm', label: 'WebM' },
  ];

  const segOptions = [
    { value: 'a', label: 'Day' },
    { value: 'b', label: 'Week' },
    { value: 'c', label: 'Month' },
  ];
</script>

<div class="section">

  <div class="group-hd">
    <h2 class="group-title">Input</h2>
    <div class="state-toggle">
      <button class:active={inputState === 'default'}  onclick={() => inputState = 'default'}>Default</button>
      <button class:active={inputState === 'error'}    onclick={() => inputState = 'error'}>Error</button>
      <button class:active={inputState === 'disabled'} onclick={() => inputState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="INP-1" label="Default">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Email" placeholder="you@example.com" bind:value={inputVal}
               error={inpError ? 'Invalid email address' : undefined}
               disabled={inpDisabled} />
      </div>
    </Card>
    <Card id="INP-3" label="With Icon">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Search" placeholder="Search files…" bind:value={inputVal}
               error={inpError ? 'No results found' : undefined}
               disabled={inpDisabled}>
          {#snippet icon()}
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="11" cy="11" r="8"/>
              <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            </svg>
          {/snippet}
        </Input>
      </div>
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Select</h2>
    <div class="state-toggle">
      <button class:active={selectState === 'default'}  onclick={() => selectState = 'default'}>Default</button>
      <button class:active={selectState === 'disabled'} onclick={() => selectState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="SEL-1" label="Empty / Placeholder">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal1} placeholder="Choose format…"
                disabled={selDisabled} />
      </div>
    </Card>
    <Card id="SEL-2" label="With Value">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal2}
                disabled={selDisabled} />
      </div>
    </Card>
    <Card id="SEL-4" label="Flat / No bevel">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal2} variant="flat" size="sm"
                disabled={selDisabled} />
      </div>
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Checkbox</h2>
    <div class="state-toggle">
      <button class:active={checkboxState === 'default'}  onclick={() => checkboxState = 'default'}>Default</button>
      <button class:active={checkboxState === 'disabled'} onclick={() => checkboxState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="CHK-1" label="Off">
      <Checkbox bind:checked={chk1} disabled={chkDisabled}>Notifications</Checkbox>
    </Card>
    <Card id="CHK-2" label="On">
      <Checkbox bind:checked={chk2} disabled={chkDisabled}>Auto-save</Checkbox>
    </Card>
    <Card id="CHK-5" label="Native / Off">
      <input type="checkbox" class="cb-native" bind:checked={chk5} disabled={chkDisabled} />
    </Card>
    <Card id="CHK-6" label="Native / On">
      <input type="checkbox" class="cb-native" bind:checked={chk6} disabled={chkDisabled} />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Slider</h2>
    <div class="state-toggle">
      <button class:active={sliderState === 'default'}  onclick={() => sliderState = 'default'}>Default</button>
      <button class:active={sliderState === 'hover'}    onclick={() => sliderState = 'hover'}>Hover</button>
      <button class:active={sliderState === 'disabled'} onclick={() => sliderState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="SLD-1" label="Slider">
      <div class="sld-wrap" class:force-hover={sldHover} class:sld-disabled={sldDisabled}>
        <input type="range" min="0" max="100" bind:value={sld1}
               class="form-range" class:force-hover={sldHover}
               style="--pct:{sld1}%" disabled={sldDisabled} />
      </div>
    </Card>
    <Card id="SLD-2" label="With Value">
      <div class="sld-wrap" class:force-hover={sldHover} class:sld-disabled={sldDisabled}>
        <div class="sld-hd">
          <span class="sld-lbl">Volume</span>
          <span class="sld-val">{sld2}%</span>
        </div>
        <input type="range" min="0" max="100" bind:value={sld2}
               class="form-range" class:force-hover={sldHover}
               style="--pct:{sld2}%" disabled={sldDisabled} />
      </div>
    </Card>
    <Card id="SLD-4" label="Compact">
      <div class="sld-wrap" class:force-hover={sldHover} class:sld-disabled={sldDisabled}>
        <input type="range" min="0" max="100" bind:value={sld4}
               class="form-range form-range--compact" class:force-hover={sldHover}
               style="--pct:{sld4}%" disabled={sldDisabled} />
      </div>
    </Card>
    <Card id="SLD-5" label="Compact / With Value">
      <div class="sld-wrap sld-wrap--inline" class:force-hover={sldHover} class:sld-disabled={sldDisabled}>
        <span class="sld-lbl">Volume</span>
        <input type="range" min="0" max="100" bind:value={sld5}
               class="form-range form-range--compact" class:force-hover={sldHover}
               style="--pct:{sld5}%" disabled={sldDisabled} />
        <span class="sld-val">{sld5}%</span>
      </div>
    </Card>
    <Card id="SLD-6" label="Component / Large" sourceFile="common-js/src/components/Slider.svelte">
      <Slider size="lg" min={0} max={100} bind:value={sld6} showValue={false} disabled={sldDisabled} />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Segmented Control</h2>
    <div class="state-toggle">
      <button class:active={segState === 'default'}  onclick={() => segState = 'default'}>Default</button>
      <button class:active={segState === 'disabled'} onclick={() => segState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="SEG-1" label="Filled / md">
      <SegmentedControl options={segOptions} bind:value={seg1} variant="filled" size="md" disabled={segDisabled} />
    </Card>
    <Card id="SEG-2" label="Filled / sm">
      <SegmentedControl options={segOptions} bind:value={seg2} variant="filled" size="sm" disabled={segDisabled} />
    </Card>
    <Card id="SEG-3" label="Underline / md">
      <SegmentedControl options={segOptions} bind:value={seg3} variant="underline" size="md" disabled={segDisabled} />
    </Card>
    <Card id="SEG-4" label="Underline / sm">
      <SegmentedControl options={segOptions} bind:value={seg4} variant="underline" size="sm" disabled={segDisabled} />
    </Card>
    <Card id="SEG-5" label="Tinted / md">
      <SegmentedControl options={segOptions} bind:value={seg5} variant="tinted" size="md" disabled={segDisabled} />
    </Card>
    <Card id="SEG-6" label="Tinted / sm">
      <SegmentedControl options={segOptions} bind:value={seg6} variant="tinted" size="sm" disabled={segDisabled} />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Toggle</h2>
    <div class="state-toggle">
      <button class:active={toggleState === 'default'}  onclick={() => toggleState = 'default'}>Default</button>
      <button class:active={toggleState === 'disabled'} onclick={() => toggleState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="TGL-1" label="Off" sourceFile="common-js/src/components/Toggle.svelte">
      <Toggle bind:checked={tgl1} disabled={tglDisabled} />
    </Card>
    <Card id="TGL-2" label="On" sourceFile="common-js/src/components/Toggle.svelte">
      <Toggle bind:checked={tgl2} disabled={tglDisabled} />
    </Card>
    <Card id="TGL-3" label="Large / Off" sourceFile="common-js/src/components/Toggle.svelte">
      <Toggle size="lg" bind:checked={tgl3} disabled={tglDisabled} />
    </Card>
    <Card id="TGL-4" label="Large / On" sourceFile="common-js/src/components/Toggle.svelte">
      <Toggle size="lg" bind:checked={tgl4} disabled={tglDisabled} />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Stepper</h2>
    <div class="state-toggle">
      <button class:active={stepperState === 'default'}  onclick={() => stepperState = 'default'}>Default</button>
      <button class:active={stepperState === 'disabled'} onclick={() => stepperState = 'disabled'}>Disabled</button>
    </div>
  </div>
  <div class="grid">
    <Card id="STP-1" label="Default" sourceFile="common-js/src/components/Stepper.svelte">
      <Stepper bind:value={stp1} min={0} max={100} step={1} disabled={stpDisabled} />
    </Card>
    <Card id="STP-2" label="With Suffix" sourceFile="common-js/src/components/Stepper.svelte">
      <Stepper bind:value={stp2} min={5} max={300} step={5} suffix="s" disabled={stpDisabled} />
    </Card>
    <Card id="STP-3" label="At Minimum" sourceFile="common-js/src/components/Stepper.svelte">
      <Stepper bind:value={stp3} min={0} max={10} step={1} disabled={stpDisabled} />
    </Card>
  </div>


</div>

<style>
  /* Slider */
  .sld-wrap { width: 100%; max-width: 240px; display: flex; flex-direction: column; gap: 6px; }
  .sld-wrap:not(.sld-disabled):hover .sld-lbl,
  .sld-wrap:not(.sld-disabled):hover .sld-val,
  .sld-wrap.force-hover:not(.sld-disabled) .sld-lbl,
  .sld-wrap.force-hover:not(.sld-disabled) .sld-val { color: #fff; transition: color 80ms; }
  .sld-wrap--inline { flex-direction: row; align-items: center; gap: 8px; }
  .sld-wrap--inline .form-range--compact { flex: 1; }
  .sld-wrap--inline .sld-val { white-space: nowrap; }
  .sld-hd   { display: flex; justify-content: space-between; align-items: center; }
  .sld-lbl  { font-size: 11px; color: var(--text-secondary); }
  .sld-val  { font-size: 11px; font-variant-numeric: tabular-nums; color: var(--text-muted); }

  .form-range {
    width: 100%;
    height: 4px;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(to right, var(--accent) var(--pct), var(--border) var(--pct));
    border-radius: 2px;
    cursor: pointer;
    outline: none;
  }
  .form-range:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }
  .form-range::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 14px;
    height: 14px;
    border-radius: 3px;
    background: var(--text-muted);
    border: 2px solid var(--surface);
    cursor: pointer;
    transition: background 80ms;
  }
  .form-range:hover::-webkit-slider-thumb,
  .form-range.force-hover::-webkit-slider-thumb {
    background: #fff;
  }
  .form-range:disabled::-webkit-slider-thumb {
    background: var(--text-muted);
  }
  .form-range--compact { height: 2px; }
  .form-range--compact::-webkit-slider-thumb {
    width: 10px;
    height: 10px;
    border-radius: 3px;
  }

  /* Group header with state toggle */
  .group-hd {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    margin: 32px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }
  .group-hd .group-title {
    margin: 0;
    padding: 0;
    border: none;
  }

  .state-toggle {
    display: flex;
    gap: 2px;
  }
  .state-toggle button {
    font-size: 11px;
    padding: 3px 10px;
    border-radius: 5px;
    border: none;
    background: none;
    color: var(--text-muted);
    cursor: pointer;
    transition: background 80ms, color 80ms;
  }
  .state-toggle button:hover {
    color: var(--text-primary);
  }
  .state-toggle button.active {
    background: color-mix(in srgb, var(--surface) 75%, var(--text-primary));
    color: var(--text-primary);
  }

  .section { max-width: 1125px; }

  .group-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: 0.1em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 32px 0 16px;
    padding-bottom: 8px;
    border-bottom: 1px solid var(--border);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 16px;
  }

</style>
