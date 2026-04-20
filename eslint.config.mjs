import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import svelteParser from 'svelte-eslint-parser';
import globals from 'globals';

export default [
  // ── Global ignores ──────────────────────────────────────────────────────
  {
    ignores: [
      '**/node_modules/**',
      '**/dist/**',
      '**/.svelte-kit/**',
      '**/src-tauri/**',
      '**/target/**',
    ],
  },

  // ── Base JS rules ───────────────────────────────────────────────────────
  js.configs.recommended,

  // ── Svelte plugin (flat/recommended includes svelte-eslint-parser) ──────
  ...svelte.configs['flat/recommended'],

  // ── Plain JS files ──────────────────────────────────────────────────────
  {
    files: ['apps/*/src/**/*.js', 'common-js/src/**/*.js'],
    languageOptions: {
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    rules: {
      'no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }],
      // Ban Svelte 4 legacy: createEventDispatcher
      'no-restricted-imports': [
        'error',
        {
          paths: [
            {
              name: 'svelte',
              importNames: ['createEventDispatcher'],
              message: 'Use callback props instead ($props() pattern, Svelte 5).',
            },
          ],
        },
      ],
    },
  },

  // ── Svelte component files ───────────────────────────────────────────────
  {
    files: ['apps/*/src/**/*.svelte', 'common-js/src/**/*.svelte'],
    languageOptions: {
      parser: svelteParser,
      ecmaVersion: 2022,
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
    rules: {
      'no-unused-vars': ['warn', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }],

      // Svelte 5 hygiene
      'svelte/no-at-debug-tags': 'error',
      'svelte/no-reactive-reassign': 'error',

      // Ban Svelte 4 legacy: export let (use $props() rune instead)
      'no-restricted-syntax': [
        'error',
        {
          selector: 'ExportNamedDeclaration[declaration.kind="let"]',
          message: 'Use $props() rune instead of "export let" (Svelte 5 pattern).',
        },
      ],

      // Ban Svelte 4 legacy: createEventDispatcher
      'no-restricted-imports': [
        'error',
        {
          paths: [
            {
              name: 'svelte',
              importNames: ['createEventDispatcher'],
              message: 'Use callback props instead ($props() pattern, Svelte 5).',
            },
          ],
        },
      ],
    },
  },
];
