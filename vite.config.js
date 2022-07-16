import { defineConfig } from 'vite'
import html from '@rollup/plugin-html'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  console.log('Command:', command);
  console.log('Mode:', mode);
  if (mode === 'browser-build') {
    return {
      base: '/tauri-notepad/',
      build: {
        rollupOptions: {
          input: {
            main: resolve(__dirname, 'src/main/index.html'),
          },
          output: [
            {
              dir: 'browser-build',
              format: 'es',
              name: 'main'
            }
          ],
          plugins: [html()]
        }
      },
      plugins: [react()],
    }
  } else {
    return {
      build: {
        rollupOptions: {
          input: {
            main: resolve(__dirname, 'src/main/index.html'),
            preferences: resolve(__dirname, 'src/preferences/index.html')
          },
          output: [
            {
              dir: 'build',
              format: 'es',
              name: 'main'
            },
            {
              dir: 'build',
              format: 'es',
              name: 'preferences'
            },
          ],
          plugins: [html()]
        },
      },
      plugins: [react()],
    }

  }
})