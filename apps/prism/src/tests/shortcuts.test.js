import { describe, it, expect, vi } from 'vitest';
import { mount, unmount } from 'svelte';
import { SHORTCUT_GROUPS } from '../lib/shortcuts.js';
import ShortcutsOverlay from '../lib/ShortcutsOverlay.svelte';

describe('SHORTCUT_GROUPS data', () => {
  it('has at least 4 groups', () => {
    expect(SHORTCUT_GROUPS.length).toBeGreaterThanOrEqual(4);
  });

  it('every group has a name and at least one item', () => {
    for (const g of SHORTCUT_GROUPS) {
      expect(typeof g.group).toBe('string');
      expect(g.group.length).toBeGreaterThan(0);
      expect(Array.isArray(g.items)).toBe(true);
      expect(g.items.length).toBeGreaterThan(0);
    }
  });

  it('every shortcut item has key and description', () => {
    for (const g of SHORTCUT_GROUPS) {
      for (const item of g.items) {
        expect(typeof item.key).toBe('string');
        expect(item.key.length).toBeGreaterThan(0);
        expect(typeof item.description).toBe('string');
        expect(item.description.length).toBeGreaterThan(0);
      }
    }
  });

  it('Global group exists and contains ? shortcut', () => {
    const global = SHORTCUT_GROUPS.find(g => g.group === 'Global');
    expect(global).toBeDefined();
    const qShortcut = global.items.find(i => i.key === '?');
    expect(qShortcut).toBeDefined();
  });

  it('Video group exists and contains Space shortcut', () => {
    const video = SHORTCUT_GROUPS.find(g => g.group === 'Video');
    expect(video).toBeDefined();
    const space = video.items.find(i => i.key === 'Space');
    expect(space).toBeDefined();
  });
});

describe('ShortcutsOverlay component', () => {
  it('renders all groups', () => {
    const target = document.createElement('div');
    document.body.appendChild(target);
    let app;
    expect(() => {
      app = mount(ShortcutsOverlay, { target, props: { onClose: () => {} } });
    }).not.toThrow();

    for (const g of SHORTCUT_GROUPS) {
      expect(target.textContent).toContain(g.group);
    }

    if (app) unmount(app);
    document.body.removeChild(target);
  });

  it('renders at least one shortcut per group', () => {
    const target = document.createElement('div');
    document.body.appendChild(target);
    let app;
    app = mount(ShortcutsOverlay, { target, props: { onClose: () => {} } });

    for (const g of SHORTCUT_GROUPS) {
      for (const item of g.items) {
        expect(target.textContent).toContain(item.key);
      }
    }

    unmount(app);
    document.body.removeChild(target);
  });

  it('calls onClose when Escape is pressed', () => {
    const target = document.createElement('div');
    document.body.appendChild(target);
    const onClose = vi.fn();
    const app = mount(ShortcutsOverlay, { target, props: { onClose } });

    const event = new KeyboardEvent('keydown', { key: 'Escape', bubbles: true, cancelable: true });
    window.dispatchEvent(event);
    expect(onClose).toHaveBeenCalledOnce();

    unmount(app);
    document.body.removeChild(target);
  });
});
