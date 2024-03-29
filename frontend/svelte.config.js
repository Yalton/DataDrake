import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    // adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
    // If your environment is not supported or you settled on a specific environment, switch out the adapter.
    // See https://kit.svelte.dev/docs/adapters for more information about adapters.
    adapter: adapter({
      // default options are shown. On some platforms (like Vercel) you might want to change outDir.
      pages: 'build',
      assets: 'build',
      fallback: null
    }),
  },
  vite: {
    server: {
      host: '0.0.0.0',
      port: 4173,
    },
  },
  preprocess: vitePreprocess()
};

export default config;