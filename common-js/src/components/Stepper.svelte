<script>
  /**
   * Stepper — numeric increment/decrement control.
   *
   * Props:
   *   value    — Number (use bind:value). Default: 0.
   *   min      — Minimum value. Default: 0.
   *   max      — Maximum value. Default: 100.
   *   step     — Increment amount. Default: 1.
   *   suffix   — Unit string appended to the display value (e.g. 's'). Default: ''.
   *   disabled — Boolean. Default: false.
   *   onchange — Called with the new number after a step.
   *   class    — Extra classes on the root element.
   *
   * Usage:
   *   <Stepper bind:value={interval} min={5} max={300} step={5} suffix="s" />
   */

  let {
    value    = $bindable(0),
    min      = 0,
    max      = 100,
    step     = 1,
    suffix   = '',
    disabled = false,
    onchange,
    class: extraClass = '',
  } = $props();

  function decrement() {
    const next = Math.max(min, value - step);
    value = next;
    onchange?.(next);
  }

  function increment() {
    const next = Math.min(max, value + step);
    value = next;
    onchange?.(next);
  }

  const atMin = $derived(value <= min);
  const atMax = $derived(value >= max);
</script>

<div class="stepper {disabled ? 'stepper-disabled' : ''} {extraClass}">
  <button
    class="step-btn"
    onclick={decrement}
    {disabled}
    aria-disabled={atMin || disabled}
    aria-label="Decrease"
    tabindex={disabled ? -1 : 0}
  >−</button>
  <span class="step-val">{value}{suffix}</span>
  <button
    class="step-btn"
    onclick={increment}
    {disabled}
    aria-disabled={atMax || disabled}
    aria-label="Increase"
    tabindex={disabled ? -1 : 0}
  >+</button>
</div>

<style>
  .stepper {
    display: inline-flex;
    align-items: center;
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    overflow: hidden;
  }

  .stepper-disabled {
    opacity: 0.4;
    pointer-events: none;
  }

  .step-btn {
    width: 26px;
    height: 22px;
    border: none;
    background: var(--surface);
    color: var(--text-secondary);
    font-size: 14px;
    cursor: pointer;
    transition: background 80ms, color 80ms;
    display: flex;
    align-items: center;
    justify-content: center;
    font-family: inherit;
    flex-shrink: 0;
  }

  .step-btn:hover {
    background: color-mix(in srgb, var(--surface) 85%, black);
    color: var(--text-primary);
  }

  .step-val {
    padding: 0 10px;
    font-size: 12px;
    font-family: 'Geist Mono', monospace;
    color: var(--text-primary);
    border-left: 1px solid var(--border);
    border-right: 1px solid var(--border);
    height: 22px;
    display: flex;
    align-items: center;
    background: var(--surface-raised);
    white-space: nowrap;
    min-width: 36px;
    justify-content: center;
  }
</style>
