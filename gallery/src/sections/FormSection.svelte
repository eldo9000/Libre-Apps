<script>
  import { Input, Select, Checkbox, SegmentedControl } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let inputVal = $state('');
  let inputError = $state('not-an-email');
  let selectVal1 = $state('');
  let selectVal2 = $state('mp4');
  let chk1 = $state(false);
  let chk2 = $state(true);
  let chk5 = $state(false);
  let chk6 = $state(true);
  let sld1 = $state(40);
  let sld2 = $state(72);

  let seg1 = $state('b');
  let seg2 = $state('a');
  let seg3 = $state('b');
  let seg4 = $state('a');
  let seg5 = $state('b');
  let seg6 = $state('a');

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
  <h1 class="page-title">Form Controls</h1>

  <h2 class="group-title">Input</h2>
  <div class="grid">
    <Card id="INP-1" label="Default">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Email" placeholder="you@example.com" bind:value={inputVal} />
      </div>
    </Card>
    <Card id="INP-2" label="With Error">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Email" bind:value={inputError} error="Invalid email address" />
      </div>
    </Card>
    <Card id="INP-3" label="With Icon">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Search" placeholder="Search files…" bind:value={inputVal}>
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
    <Card id="INP-4" label="Disabled">
      <div style="width: 100%; max-width: 240px;">
        <Input label="Username" value="eldo" disabled />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Select</h2>
  <div class="grid">
    <Card id="SEL-1" label="Empty / Placeholder">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal1} placeholder="Choose format…" />
      </div>
    </Card>
    <Card id="SEL-2" label="With Value">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal2} />
      </div>
    </Card>
    <Card id="SEL-3" label="Disabled">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} value="mp4" disabled />
      </div>
    </Card>
    <Card id="SEL-4" label="Flat / No bevel">
      <div style="width: 100%; max-width: 200px;">
        <Select items={selectItems} bind:value={selectVal2} variant="flat" size="sm" />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Checkbox</h2>
  <div class="grid">
    <Card id="CHK-1" label="Unchecked">
      <Checkbox bind:checked={chk1}>Notifications</Checkbox>
    </Card>
    <Card id="CHK-2" label="Checked">
      <Checkbox bind:checked={chk2}>Auto-save</Checkbox>
    </Card>
    <Card id="CHK-3" label="Unchecked / Disabled">
      <Checkbox checked={false} disabled>Sync to cloud</Checkbox>
    </Card>
    <Card id="CHK-4" label="Checked / Disabled">
      <Checkbox checked={true} disabled>Hardware acceleration</Checkbox>
    </Card>
    <Card id="CHK-5" label="Native / Off">
      <input type="checkbox" class="cb-native" bind:checked={chk5} />
    </Card>
    <Card id="CHK-6" label="Native / On">
      <input type="checkbox" class="cb-native" bind:checked={chk6} />
    </Card>
    <Card id="CHK-7" label="Native / Disabled">
      <input type="checkbox" class="cb-native" disabled />
    </Card>
  </div>

  <h2 class="group-title">Slider</h2>
  <div class="grid">
    <Card id="SLD-1" label="Default">
      <div class="sld-wrap">
        <input type="range" min="0" max="100" bind:value={sld1}
               class="form-range" style="--pct:{sld1}%" />
      </div>
    </Card>
    <Card id="SLD-2" label="With Value">
      <div class="sld-wrap">
        <div class="sld-hd">
          <span class="sld-lbl">Volume</span>
          <span class="sld-val">{sld2}%</span>
        </div>
        <input type="range" min="0" max="100" bind:value={sld2}
               class="form-range" style="--pct:{sld2}%" />
      </div>
    </Card>
    <Card id="SLD-3" label="Disabled">
      <div class="sld-wrap">
        <input type="range" min="0" max="100" value="30"
               class="form-range" style="--pct:30%" disabled />
      </div>
    </Card>
  </div>

  <h2 class="group-title">Segmented Control</h2>
  <div class="grid">
    <Card id="SEG-1" label="Filled / md">
      <SegmentedControl options={segOptions} bind:value={seg1} variant="filled" size="md" />
    </Card>
    <Card id="SEG-2" label="Filled / sm">
      <SegmentedControl options={segOptions} bind:value={seg2} variant="filled" size="sm" />
    </Card>
    <Card id="SEG-3" label="Underline / md">
      <SegmentedControl options={segOptions} bind:value={seg3} variant="underline" size="md" />
    </Card>
    <Card id="SEG-4" label="Underline / sm">
      <SegmentedControl options={segOptions} bind:value={seg4} variant="underline" size="sm" />
    </Card>
    <Card id="SEG-5" label="Tinted / md">
      <SegmentedControl options={segOptions} bind:value={seg5} variant="tinted" size="md" />
    </Card>
    <Card id="SEG-6" label="Tinted / sm">
      <SegmentedControl options={segOptions} bind:value={seg6} variant="tinted" size="sm" />
    </Card>
  </div>
</div>

<style>
  /* Slider */
  .sld-wrap { width: 100%; max-width: 240px; display: flex; flex-direction: column; gap: 6px; }
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
    border-radius: 50%;
    background: var(--accent);
    border: 2px solid var(--surface);
    box-shadow: 0 0 0 1px var(--accent);
    cursor: pointer;
  }
  .form-range:disabled::-webkit-slider-thumb {
    background: var(--text-muted);
    box-shadow: 0 0 0 1px var(--text-muted);
  }

  .section { max-width: 900px; }

  .page-title {
    font-size: 22px;
    font-weight: 600;
    letter-spacing: -0.02em;
    margin: 0 0 32px;
  }

  .group-title {
    font-size: 11px;
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
