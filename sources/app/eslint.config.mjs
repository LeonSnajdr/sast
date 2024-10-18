import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt().override("nuxt/vue/rules", {
    rules: {
        "vue/multi-word-component-names": "off",
        "vue/v-on-event-hyphenation": ["error", "never"],
        "vue/attribute-hyphenation": ["error", "never"],
        "vue/component-name-in-template-casing": ["error", "PascalCase", { registeredComponentsOnly: false }],
        "vue/component-api-style": ["error", ["script-setup"]],
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
                    "OTHER_ATTR",
                    "OTHER_DIRECTIVES",
                    "CONTENT"
                ],
                alphabetical: true
            }
        ]
    }
});
