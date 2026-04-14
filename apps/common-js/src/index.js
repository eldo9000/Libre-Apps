// @libre/ui — public API

// Theme
export { initTheme } from './theme.js';

// Typed command wrappers (TypeScript — import directly for full types)
// export * from './api/commands.ts';   // uncomment if bundler handles .ts
// export * from './api/dialogs.ts';

// Layout
export { default as WindowFrame } from './components/WindowFrame.svelte';
export { default as Titlebar }    from './components/Titlebar.svelte';

// Components
export { default as Button }      from './components/Button.svelte';
export { default as IconButton }  from './components/IconButton.svelte';
export { default as ProgressBar } from './components/ProgressBar.svelte';
export { default as ScrollArea }  from './components/ScrollArea.svelte';
export { default as TabBar }      from './components/TabBar.svelte';
export { default as Toast }       from './components/Toast.svelte';
export { default as Toaster }     from './components/Toaster.svelte';
