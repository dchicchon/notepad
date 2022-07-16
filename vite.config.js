import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  console.log('Command:', command);
  console.log('Mode:', mode);
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
          output: {
            dir: "browser-build"
          }
        },
      },
      plugins: [react()],
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
    plugins: [react()],
  }
})