import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vuetify, { transformAssetUrls } from "vite-plugin-vuetify";
import vueMacros from "unplugin-vue-macros/vite";
import autoImport from "unplugin-auto-import/vite";

// https://vitejs.dev/config/
export default defineConfig({
    base: "./",
    plugins: [
        vueMacros({
            plugins: {
                vue: vue({ template: { transformAssetUrls } })
            }
        }),
        // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vite-plugin
        vuetify({
            autoImport: true,
            styles: { configFile: "src/styles/settings.scss" }
        }),
        autoImport({
            dts: "typings/auto-imports.d.ts",
            imports: ["vue", "pinia", "vue-router"],
            dirs: ["src/stores"]
        })
    ],
    server: {
        port: 8080
    },
    build: {
        target: "esnext",
        outDir: "dist/"
    },
    resolve: {
        alias: {
            "@": fileURLToPath(new URL("./src", import.meta.url)),
            "vue-i18n": "vue-i18n/dist/vue-i18n.cjs.js"
        }
    }
});
