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
      apiBase: 'https://localhost:8443/', // fallback if /env.js has no API_BASE set at container runtime
    },
  },
  app: {
    head: {
      script: [
        { src: '/env.js' }
      ]
    }
  },
  colorMode: {
    preference: 'light',
    fallback: 'light',
    componentName: 'ColorScheme',
    classPrefix: '',
    classSuffix: ''
  },
  css: [
    'assets/main.css'
  ]
})
