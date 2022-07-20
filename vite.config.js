import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'
import { terser } from 'rollup-plugin-terser'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  if (mode === 'browser-build') {
    return {
      base: '/notepad/',
      build: {
        rollupOptions: {
          input: {
            index: resolve(__dirname, 'index.html'),
            notepad: resolve(__dirname, 'notepad.html'),
            preferences: resolve(__dirname, 'preferences.html')
          },
        },
        outDir: "browser-build"
      },
      plugins: [react(), terser()],
    }
  }
  return {
    server: {
      port: 3000
    },
    build: {
      rollupOptions: {
        input: {
          index: resolve(__dirname, 'index.html'),
          notepad: resolve(__dirname, 'notepad.html'),
          preferences: resolve(__dirname, 'preferences.html')
        },
      },
    },
    plugins: [react(), terser()],
  }
})