// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2025-07-21",
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
    experimental: {
        typedPages: true
    },
    modules: ["@nuxt/eslint", "@nuxtjs/i18n", "vuetify-nuxt-module", "@pinia/nuxt", "@vueuse/nuxt"],
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
        strategy: "no_prefix"
    },
    css: ["./assets/styles/global.scss"],
    vuetify: {
        moduleOptions: {
            styles: {
                configFile: "./assets/styles/settings.scss"
            }
        },
        vuetifyOptions: "./vuetify.config.ts"
    },
    vueuse: {
        autoImports: true
    },
    imports: {
        presets: [
            {
                from: "lodash",
                imports: [
                    { name: "cloneDeep", as: "lodCloneDeep" },
                    { name: "maxBy", as: "lodMaxBy" },
                    { name: "orderBy", as: "lodOrderBy" },
                    { name: "debounce", as: "lodDebounce" },
                    { name: "trim", as: "lodTrim" },
                    { name: "some", as: "lodSome" },
                    { name: "filter", as: "lodFilter" },
                    { name: "union", as: "lodUnion" },
                    { name: "after", as: "lodAfter" },
                    { name: "assign", as: "lodAssign" },
                    { name: "findLast", as: "lodFindLast" },
                    { name: "remove", as: "lodRemove" }
                ]
            }
        ]
    }
});
