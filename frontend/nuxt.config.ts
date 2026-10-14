// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: {
    enabled: true
  },
  modules: [
      '@nuxt/ui'
  ],
  runtimeConfig: {
    public: {
      apiBase: '/api/',
      turnstileSiteKey: '0x4AAAAAAEjMzgiesFBNdWUD',
    },
  },
  app: {
    head: {
      script: [
        { src: '/env.js' }
      ]
    }
  },
  ui: {
    colorMode: false
  },
  css: [
    'assets/main.css'
  ],
  ssr: false,
  vite: {
    server: {
      proxy: {
        '/api': {
          target: 'https://127.0.0.1:8080',
          changeOrigin: true,
          secure: false,
          rewrite: (path) => path.replace(/^\/api/, '')
        }
      }
    }
  }
})
