<script>
  import { Button, Tooltip } from '@libre/ui';
</script>

<div class="section">

  <!-- Default Resets -->
  <div class="pattern">
    <div class="pattern-meta">
      <h2 class="pattern-title">Default Resets</h2>
      <p class="pattern-desc">
        Every interactive element must reset browser-native styling before applying design tokens.
        Buttons, inputs, and selects ship with UA stylesheets that override font, spacing, and
        border behaviour unpredictably across platforms. The design system assumes these resets
        are in place — missing them produces visual drift that tokens alone cannot fix.
      </p>
      <ul class="pattern-rules">
        <li><code>font-family: inherit</code> — buttons and inputs do not inherit by default</li>
        <li><code>-webkit-appearance: none; appearance: none</code> — removes native chrome on inputs and selects</li>
        <li><code>box-sizing: border-box</code> — must be set globally via <code>*, *::before, *::after</code></li>
        <li><code>outline: none</code> on inputs — replace with explicit focus ring using <code>--accent</code></li>
        <li><code>margin: 0</code> on headings and paragraphs — UA margins are inconsistent</li>
      </ul>
    </div>
    <div class="pattern-demo">
      <div class="demo-pair">
        <div class="demo-col">
          <span class="demo-label">Without reset</span>
          <button class="bare-button">Click me</button>
        </div>
        <div class="demo-col">
          <span class="demo-label">With reset</span>
          <Button>Click me</Button>
        </div>
      </div>
    </div>
  </div>

  <div class="pattern-divider"></div>

  <!-- Tooltip Overlays -->
  <div class="pattern">
    <div class="pattern-meta">
      <h2 class="pattern-title">Tooltip Overlays</h2>
      <p class="pattern-desc">
        Tooltips are non-blocking overlays that surface supplemental information — never critical
        content. They appear on hover after a delay and must never obscure the trigger or adjacent
        interactive elements. Placement follows a strict priority so the tooltip always stays in
        the viewport.
      </p>
      <ul class="pattern-rules">
        <li><strong>Delay:</strong> 500 ms default — instant tooltips feel twitchy during normal navigation</li>
        <li><strong>Placement priority:</strong> top → bottom → left → right, depending on available space</li>
        <li><strong>Content:</strong> one short phrase, no punctuation, no wrapping — if it wraps, the content belongs in a popover</li>
        <li><strong>Pointer events:</strong> <code>pointer-events: none</code> — tooltips must never intercept clicks</li>
        <li><strong>z-index:</strong> tooltips sit above all panel chrome but below modal backdrops</li>
        <li><strong>Never:</strong> use a tooltip on disabled controls — screen readers won't announce it</li>
      </ul>
    </div>
    <div class="pattern-demo">
      <div class="demo-pair demo-pair-col">
        <div class="demo-row">
          <span class="demo-label-inline">Top (default)</span>
          <Tooltip content="Save file (⌘S)" placement="top" delay={0}>
            <Button>Save</Button>
          </Tooltip>
        </div>
        <div class="demo-row">
          <span class="demo-label-inline">Bottom</span>
          <Tooltip content="Discard all changes" placement="bottom" delay={0}>
            <Button>Discard</Button>
          </Tooltip>
        </div>
        <div class="demo-row">
          <span class="demo-label-inline">Left</span>
          <Tooltip content="Previous item" placement="left" delay={0}>
            <Button>← Prev</Button>
          </Tooltip>
        </div>
        <div class="demo-row">
          <span class="demo-label-inline">Right</span>
          <Tooltip content="Next item" placement="right" delay={0}>
            <Button>Next →</Button>
          </Tooltip>
        </div>
      </div>
    </div>
  </div>

</div>

<style>
  .section { max-width: 1125px; }

  .pattern {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: 48px;
    padding: 40px 0;
  }

  .pattern-divider {
    height: 1px;
    background: var(--border);
  }

  .pattern-title {
    font-size: 16px;
    font-weight: 600;
    letter-spacing: -0.01em;
    color: var(--text-primary);
    margin: 0 0 12px;
  }

  .pattern-desc {
    font-size: 13px;
    line-height: 1.65;
    color: var(--text-secondary);
    margin: 0 0 16px;
  }

  .pattern-rules {
    margin: 0;
    padding-left: 16px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .pattern-rules li {
    font-size: 12px;
    line-height: 1.55;
    color: var(--text-secondary);
  }

  .pattern-rules code {
    font-family: 'Geist Mono', monospace;
    font-size: 11px;
    background: color-mix(in srgb, var(--text-primary) 8%, transparent);
    padding: 1px 4px;
    border-radius: 3px;
    color: var(--text-primary);
  }

  .pattern-rules strong {
    color: var(--text-primary);
    font-weight: 600;
  }

  .pattern-demo {
    display: flex;
    align-items: flex-start;
    padding-top: 4px;
  }

  .demo-pair {
    display: flex;
    gap: 24px;
    align-items: flex-end;
  }

  .demo-pair-col {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }

  .demo-col {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }

  .demo-row {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .demo-label {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.06em;
    text-transform: uppercase;
    color: var(--text-muted);
  }

  .demo-label-inline {
    font-size: 10px;
    color: var(--text-muted);
    width: 56px;
    flex-shrink: 0;
  }

  .bare-button {
    /* intentionally not reset — shows raw UA styling */
    font-size: 13px;
  }
</style>
