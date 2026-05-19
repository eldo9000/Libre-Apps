<script>
  /**
   * Toggle — on/off switch.
   *
   * Props:
   *   checked  — Boolean (use bind:checked). Default: false.
   *   disabled — Boolean. Default: false.
   *   onchange — Called with the new boolean after toggling.
   *   class    — Extra classes on the root element.
   *
   * Usage:
   *   <Toggle bind:checked={autoSave} />
   *   <Toggle bind:checked={val} disabled={true} />
   */

  let {
    checked  = $bindable(false),
    disabled = false,
    size     = 'md',
    onchange,
    class: extraClass = '',
  } = $props();

  function toggle() {
    if (disabled) return;
    checked = !checked;
    onchange?.(checked);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div
  class="toggle toggle-{size} {checked ? 'toggle-on' : ''} {disabled ? 'toggle-disabled' : ''} {extraClass}"
  role="switch"
  aria-checked={checked}
  aria-disabled={disabled}
  tabindex={disabled ? -1 : 0}
  onclick={toggle}
  onkeydown={e => (e.key === ' ' || e.key === 'Enter') && toggle()}
></div>

<style>
  .toggle {
    width: 36px;
    height: 16px;
    border-radius: 6px;
    background: var(--border);
    position: relative;
    cursor: pointer;
    transition: background 120ms;
    flex-shrink: 0;
  }

  .toggle::after {
    content: '';
    position: absolute;
    width: 10px;
    height: 10px;
    border-radius: 50%;
    background: #fff;
    top: 3px;
    left: 3px;
    transition: left 120ms;
    box-shadow: 0 1px 3px rgba(0 0 0 / 0.25);
  }

  .toggle-on {
    background: var(--accent);
  }

  .toggle-on::after {
    left: 23px;
  }

  .toggle-disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .toggle-lg {
    width: 44px;
    height: 24px;
    border-radius: 7px;
  }

  .toggle-lg::after {
    width: 17px;
    height: 17px;
    border-radius: 4px;
    top: 3.5px;
    left: 3.5px;
  }

  .toggle-lg.toggle-on::after {
    left: 23.5px;
  }
</style>
