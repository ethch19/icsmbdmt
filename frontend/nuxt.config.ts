export default defineNuxtConfig({
    modules: ['@nuxt/content', "@nuxt/image"],
    css: ['~/assets/css/main.css'],
    devtools: { enabled: true },
    compatibilityDate: '2024-11-01',
    future: {
        compatibilityVersion: 4
    },
    app: {
        pageTransition: { name: "page", mode: "out-in" },
        layoutTransition: { name: "layout", mode: "out-in" }
    },
    nitro: {
        experimental: {
            wasm: true
        },
        devProxy: {
            '/api': {
                target: 'http://localhost:8000/api',
                changeOrigin: true,
                prependPath: true
            }
        }
    },
    // Add runtime config for API base URL
    runtimeConfig: {
        public: {
            apiBase: process.env.API_BASE_URL || 'http://localhost:8000'
        }
    }
})