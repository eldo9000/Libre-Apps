<script>
  import Card from '../lib/Card.svelte';
  import { Toggle, Slider, ColorWheel } from '@libre/ui';

  let toggled = $state(false);
  let sliderVal = $state(60);
  let color1 = $state('#2884c9');
  let color2 = $state('#c94028');
</script>

<div class="section">

  <div class="group-hd">
    <h2 class="group-title">Color Wheel</h2>
  </div>
  <div class="row">
    <Card id="CW-1" label="Compact" sourceFile="common-js/src/components/ColorWheel.svelte">
      <div class="cw-demo-compact">
        <ColorWheel bind:value={color1} />
        <span class="cw-demo-hex">{color1}</span>
      </div>
    </Card>
    <Card id="CW-2" label="Expanded" sourceFile="common-js/src/components/ColorWheel.svelte">
      <ColorWheel bind:value={color2} embedded />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Toggle Large</h2>
  </div>
  <div class="row">
    <Card id="TMP-1" label="Toggle Large" sourceFile="common-js/src/components/Toggle.svelte">
      <Toggle size="lg" bind:checked={toggled} />
    </Card>
  </div>

  <div class="group-hd">
    <h2 class="group-title">Slider</h2>
  </div>
  <div class="row">
    <Card id="TMP-3" label="Slider Value" sourceFile="common-js/src/components/Slider.svelte">
      <Slider label="Value" size="lg" min={0} max={100} bind:value={sliderVal} unit="%" />
    </Card>
  </div>

</div>

<style>
  .section {
    display: flex;
    flex-direction: column;
    gap: 24px;
    padding: 24px;
  }

  .group-hd {
    margin-bottom: -16px;
  }

  .group-title {
    font-size: 12px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
    margin: 0;
  }

  .row {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
    gap: 12px;
  }

  .cw-demo-compact {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .cw-demo-hex {
    font-size: 11px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-muted);
    letter-spacing: 0.04em;
  }
</style>
