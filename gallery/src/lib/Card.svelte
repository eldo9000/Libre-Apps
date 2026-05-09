<script>
  let { id, label, children } = $props();
</script>

<div class="card">
  <div class="card-header">
    <code class="card-id">{id}</code>
    <span class="card-name">{label}</span>
  </div>
  <div class="card-body">
    {@render children?.()}
  </div>
</div>

<style>
  .card {
    border: 1px solid var(--border);
    border-radius: 10px;
    overflow: hidden;
    background: var(--surface-raised);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 14px;
    border-bottom: 1px solid var(--border);
    background: var(--surface-panel);
  }

  .card-id {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--accent);
    font-family: 'Geist Mono', 'Fira Code', monospace;
  }

  .card-name {
    font-size: 12px;
    color: var(--text-secondary);
  }

  .card-body {
    padding: 28px 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 88px;
    gap: 12px;
    flex-wrap: wrap;
  }
</style>
