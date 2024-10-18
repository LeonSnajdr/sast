// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: "2024-04-03",
    devtools: { enabled: true },
    modules: ["@nuxt/eslint", "vuetify-nuxt-module", "@nuxtjs/i18n"],
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
    }
});
