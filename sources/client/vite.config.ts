import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueMacros from "unplugin-vue-macros/vite";

// https://vitejs.dev/config/
export default defineConfig({
    base: "./",
    plugins: [
        vueMacros({
            plugins: {
                vue: vue()
            }
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
            "@": fileURLToPath(new URL("./src", import.meta.url))
        }
    }
});
