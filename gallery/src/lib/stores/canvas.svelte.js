const CANVAS_KEY = 'libre-gallery-canvas';

let _saved = null;
try { _saved = JSON.parse(localStorage.getItem(CANVAS_KEY) ?? 'null'); } catch {}

export const canvas = $state({
  osMode:       _saved?.osMode       ?? 'macos',
  bgPattern:    _saved?.bgPattern    ?? 'none',
  bgBrightness: _saved?.bgBrightness ?? 0,
  activeTab:    _saved?.activeTab    ?? 'overview',
});

$effect.root(() => {
  $effect(() => {
    try {
      localStorage.setItem(CANVAS_KEY, JSON.stringify({
        osMode:       canvas.osMode,
        bgPattern:    canvas.bgPattern,
        bgBrightness: canvas.bgBrightness,
        activeTab:    canvas.activeTab,
      }));
    } catch {}
  });
});

export const PATTERNS = {
  dots: {
    image:    'radial-gradient(circle, color-mix(in srgb, var(--text-primary) 14%, transparent) 1px, transparent 1px)',
    size:     '16px 16px',
    position: '0 0',
  },
  grid: {
    image:    'linear-gradient(color-mix(in srgb, var(--text-primary) 9%, transparent) 1px, transparent 1px), linear-gradient(90deg, color-mix(in srgb, var(--text-primary) 9%, transparent) 1px, transparent 1px)',
    size:     '16px 16px, 16px 16px',
    position: '0 0, 0 0',
  },
  checker: {
    image:    'linear-gradient(45deg, color-mix(in srgb, var(--text-primary) 6%, transparent) 25%, transparent 25%, transparent 75%, color-mix(in srgb, var(--text-primary) 6%, transparent) 75%), linear-gradient(45deg, color-mix(in srgb, var(--text-primary) 6%, transparent) 25%, transparent 25%, transparent 75%, color-mix(in srgb, var(--text-primary) 6%, transparent) 75%)',
    size:     '12px 12px, 12px 12px',
    position: '0 0, 6px 6px',
  },
};
