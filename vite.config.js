import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig(({ command, mode }) => {
  console.log('Command:', command);
  console.log('Mode:', mode);
  return {
    build: {
      rollupOptions: {
        input: {
          index: 'index.html',
          notepad: 'notepad.html',
          preferences: 'preferences.html'
        },
        output: [
          {
            dir: 'dist',
            format: 'es',
            name: 'notepad'
          },
          {
            dir: 'dist',
            format: 'es',
            name: 'preferences'
          },
        ],
      },
    },
    plugins: [react()],
  }
})