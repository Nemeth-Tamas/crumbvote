import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import tailwindcss from '@tailwindcss/vite'

const backendTarget =
  process.env.CRUMBVOTE_DEV_API ??
  'http://127.0.0.1:3000'

export default defineConfig({
  plugins: [
    svelte(),
    tailwindcss(),
  ],

  server: {
    proxy: {
      '/api': backendTarget,
      '/media': backendTarget,
    },
  },
})