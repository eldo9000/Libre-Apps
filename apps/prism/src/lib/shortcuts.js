// Keyboard shortcut definitions for Prism.
// Imported by both ShortcutsOverlay.svelte and shortcuts.test.js
// so the overlay always renders exactly what's bound.

export const SHORTCUT_GROUPS = [
  {
    group: 'Global',
    items: [
      { key: '?',       description: 'Show keyboard shortcuts' },
      { key: 'I',       description: 'Toggle file info panel' },
      { key: 'Ctrl+O', description: 'Open file dialog' },
      { key: 'Escape', description: 'Close overlay / back' },
      { key: '← / →',  description: 'Prev / next file in folder' },
    ],
  },
  {
    group: 'Image',
    items: [
      { key: '+  /  −',  description: 'Zoom in / out' },
      { key: 'Scroll',   description: 'Zoom in / out' },
      { key: '0',        description: 'Reset zoom to 100%' },
      { key: 'R',        description: 'Rotate right 90°' },
      { key: 'L',        description: 'Rotate left 90°' },
    ],
  },
  {
    group: 'Video',
    items: [
      { key: 'Space',   description: 'Play / pause' },
      { key: '← / →',  description: 'Seek ±5 seconds' },
      { key: ', / .',   description: 'Step one frame back / forward' },
      { key: '↑ / ↓',  description: 'Volume up / down' },
      { key: 'M',       description: 'Toggle mute' },
      { key: 'F',       description: 'Toggle fullscreen' },
    ],
  },
  {
    group: 'Audio',
    items: [
      { key: 'Space',  description: 'Play / pause' },
      { key: '← / →', description: 'Seek ±5 seconds' },
      { key: '↑ / ↓', description: 'Volume up / down' },
    ],
  },
  {
    group: 'PDF',
    items: [
      { key: '← / →',   description: 'Prev / next page' },
      { key: '+  /  −', description: 'Zoom in / out' },
    ],
  },
  {
    group: '3D Model',
    items: [
      { key: 'Drag',   description: 'Orbit / rotate camera' },
      { key: 'Scroll', description: 'Zoom' },
    ],
  },
];
