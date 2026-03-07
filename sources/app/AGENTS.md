# AGENTS.md

## Commands

`yarn dev` - Start the Tauri desktop app with the Nuxt frontend [NOTE: Don't use this unless otherwise told to]
`yarn build` - Build/package the Tauri desktop app
`yarn lint` - Run ESLint
`yarn cspell` - Run spellcheck

**Do not run:** `yarn dev` (assume already running), `yarn build` (only when explicitly asked)

## Structure

`app/` - Nuxt app code: pages, components, stores, composables, assets
`src-tauri/` - Rust/Tauri desktop backend, commands, database, migrations
`i18n/` - Localization config and translation files
`public/` - Static public assets
`nuxt.config.ts` - Nuxt configuration
