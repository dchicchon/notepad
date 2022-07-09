import { defineConfig } from 'vite'
import { fileURLToPath } from 'url'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  build: {
    rollupOptions: {
      // makeAbsoluteExternalsRelative: false,
      input: {
        main: fileURLToPath(new URL('./src/main/index.html', import.meta.url)),
        preferences: fileURLToPath(new URL('./src/preferences/index.html', import.meta.url))
      },
      output: [
        { 
          dir: "dist",
          format: 'es',
          name: "main"
        },
        {
          dir: "dist",
          format: 'es',
          name: "preferences"
        },
      ]
    },
  }
})
