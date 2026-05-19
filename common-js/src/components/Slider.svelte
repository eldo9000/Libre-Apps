<script>
  let {
    label     = '',
    min       = 0,
    max       = 100,
    value     = $bindable(0),
    step      = 1,
    unit      = '',
    size      = 'md',
    disabled  = false,
    showValue = true,
    class: extraClass = '',
  } = $props();

  const pct = $derived(((value - min) / (max - min) * 100).toFixed(1));
</script>

<div class="slider slider-{size} {extraClass}">
  {#if label}
    <span class="slider-label">{label}</span>
  {/if}
  <input
    type="range"
    class="slider-track"
    {min} {max} {step} {disabled}
    bind:value
    style="--pct:{pct}%"
  />
  {#if showValue}<span class="slider-val">{value}{unit}</span>{/if}
</div>

<style>
  .slider {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
  }

  /* ── md (default) ────────────────────────────────────────────────────── */
  .slider-label {
    font-size: 12px;
    color: var(--text-secondary);
    width: 72px;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .slider-track {
    flex: 1;
    height: 2px;
    appearance: none;
    -webkit-appearance: none;
    background: linear-gradient(to right, var(--accent) var(--pct, 0%), var(--border) var(--pct, 0%));
    border-radius: 1px;
    cursor: pointer;
    outline: none;
  }

  .slider-track:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .slider-track::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 10px;
    height: 10px;
    border-radius: 3px;
    background: var(--text-muted);
    border: 2px solid var(--surface);
    cursor: pointer;
    transition: background 80ms;
  }

  .slider-track:not(:disabled):hover::-webkit-slider-thumb {
    background: #fff;
  }

  .slider-track:disabled::-webkit-slider-thumb {
    background: var(--text-muted);
  }

  .slider-val {
    font-size: 11px;
    font-variant-numeric: tabular-nums;
    color: var(--text-muted);
    width: 36px;
    text-align: right;
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* ── lg ──────────────────────────────────────────────────────────────── */
  .slider-lg {
    gap: 16px;
  }

  .slider-lg .slider-label {
    font-size: 14px;
    width: 88px;
  }

  .slider-lg .slider-track {
    height: 8px;
    border-radius: 4px;
  }

  .slider-lg .slider-track::-webkit-slider-thumb {
    width: 14px;
    height: 14px;
    border-radius: 3px;
    background: var(--text-muted);
    border: 1px solid rgba(0, 0, 0, 0.5);
    cursor: pointer;
    transition: background 80ms;
  }

  .slider-lg .slider-track:not(:disabled):hover::-webkit-slider-thumb {
    background: #fff;
  }

  .slider-lg .slider-val {
    font-size: 13px;
    width: 44px;
  }
</style>
