import { describe, it, expect, vi } from 'vitest';
import { mount, unmount } from 'svelte';
import App from '../App.svelte';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn((cmd) => {
    const responses = {
      get_theme: 'system',
      get_accent: '#0066cc',
    };
    return Promise.resolve(responses[cmd] ?? null);
  }),
}));

vi.mock('@tauri-apps/api/window', () => ({
  getCurrentWindow: vi.fn(() => ({
    listen: vi.fn().mockResolvedValue(() => {}),
    onCloseRequested: vi.fn().mockResolvedValue(() => {}),
    isMaximized: vi.fn().mockResolvedValue(false),
    minimize: vi.fn().mockResolvedValue(undefined),
    maximize: vi.fn().mockResolvedValue(undefined),
    unmaximize: vi.fn().mockResolvedValue(undefined),
    close: vi.fn().mockResolvedValue(undefined),
  })),
}));

vi.mock('@tauri-apps/api/event', () => ({
  listen: vi.fn().mockResolvedValue(() => {}),
  once: vi.fn().mockResolvedValue(() => {}),
  emit: vi.fn().mockResolvedValue(undefined),
}));

describe('Ghost App', () => {
  it('mounts without throwing', () => {
    const target = document.createElement('div');
    document.body.appendChild(target);
    let app;
    expect(() => {
      app = mount(App, { target });
    }).not.toThrow();
    if (app) unmount(app);
    document.body.removeChild(target);
  });
});
