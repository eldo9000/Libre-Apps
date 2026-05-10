import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { fileURLToPath } from 'url';

// common-js lives outside gallery/, so Node module resolution can't walk up
// to gallery/node_modules/ from there. These aliases anchor @tauri-apps/api
// subpath imports to the copy installed inside gallery/node_modules/.
const tauriApi = (sub) =>
  fileURLToPath(new URL(`./node_modules/@tauri-apps/api/${sub}.js`, import.meta.url));

export default defineConfig({
  plugins: [tailwindcss(), svelte()],
  resolve: {
    alias: [
      { find: '@tauri-apps/api/window',       replacement: tauriApi('window') },
      { find: '@tauri-apps/api/core',         replacement: tauriApi('core') },
      { find: '@tauri-apps/api/event',        replacement: tauriApi('event') },
      { find: '@tauri-apps/api/path',         replacement: tauriApi('path') },
      { find: '@tauri-apps/api/webviewWindow', replacement: tauriApi('webviewWindow') },
    ],
  },
  clearScreen: false,
  server: {
    port: 1422,
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**', '**/.focus.json'],
    },
  },
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: {
    target: process.env.TAURI_ENV_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
    minify: process.env.TAURI_ENV_DEBUG ? false : 'oxc',
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});
