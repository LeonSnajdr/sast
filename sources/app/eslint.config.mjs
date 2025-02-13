import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt()
    .override("nuxt/vue/rules", {
        rules: {
            "vue/multi-word-component-names": "off",
            "vue/v-on-event-hyphenation": ["error", "never"],
            "vue/attribute-hyphenation": ["error", "never"],
            "vue/component-name-in-template-casing": ["error", "PascalCase", { registeredComponentsOnly: false }],
            "vue/component-api-style": ["error", ["script-setup"]],
            "vue/html-self-closing": ["error", "always"],
            "vue/define-macros-order": [
                "error",
                {
                    order: ["defineEmits", "defineProps", "defineModel"]
                }
            ],
            "vue/attributes-order": [
                "error",
                {
                    order: [
                        "DEFINITION",
                        "LIST_RENDERING",
                        "CONDITIONALS",
                        "RENDER_MODIFIERS",
                        "GLOBAL",
                        ["UNIQUE", "SLOT"],
                        "TWO_WAY_BINDING",
                        "EVENTS",
                        "ATTR_DYNAMIC",
                        "ATTR_STATIC",
                        "ATTR_SHORTHAND_BOOL",
                        "OTHER_DIRECTIVES",
                        "CONTENT"
                    ],
                    alphabetical: true
                }
            ]
        }
    })
    .override("nuxt/vue/single-root", {
        rules: {
            "vue/no-multiple-template-root": "off"
        }
    });
