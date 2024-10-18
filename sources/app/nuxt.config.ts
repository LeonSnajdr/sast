// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2024-04-03",
    devtools: { enabled: true },
    ssr: false,
    devServer: { host: process.env.TAURI_DEV_HOST || "localhost" },
    vite: {
        clearScreen: false,
        envPrefix: ["VITE_", "TAURI_"],
        server: {
            strictPort: true
        }
    },
    modules: ["@nuxt/eslint", "@nuxtjs/i18n", "vuetify-nuxt-module", "nuxt-typed-router"],
    i18n: {
        locales: [
            {
                code: "en",
                name: "English"
            },
            {
                code: "de",
                name: "Deutsch"
            }
        ],
        defaultLocale: "en",
        strategy: "no_prefix",
        vueI18n: "./i18n.config.ts"
    },
    vuetify: {
        vuetifyOptions: "./vuetify.config.ts"
    }
});
