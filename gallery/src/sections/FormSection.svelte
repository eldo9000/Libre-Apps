<script>
  import { Input, Select, Checkbox, SegmentedControl } from '@libre/ui';
  import Card from '../lib/Card.svelte';

  let inputVal = $state('');
  let inputError = $state('not-an-email');
  let selectVal1 = $state('');
  let selectVal2 = $state('mp4');
  let chk1 = $state(false);
  let chk2 = $state(true);
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
    <Card id="SEG-5" label="Sliding / md">
      <SegmentedControl options={segOptions} bind:value={seg5} variant="sliding" size="md" />
    </Card>
    <Card id="SEG-6" label="Sliding / sm">
      <SegmentedControl options={segOptions} bind:value={seg6} variant="sliding" size="sm" />
    </Card>
  </div>
</div>

<style>
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
