module.exports = {
  root: false,
  plugins: ["import"],
  rules: {
    "@typescript-eslint/consistent-type-imports": "error",
    "vue/multi-word-component-names": "off",
    "vue/attribute-hyphenation": ["error", "never"],
    "vue/v-on-event-hyphenation": ["error", "never", { autofix: true }],
    "vue/this-in-template": ["error", "never"],
    "vue/component-api-style": ["error", ["script-setup"]],
    "vue/define-macros-order": [
      "error",
      {
        order: ["defineProps", "defineEmits"]
      }
    ],
    "import/first": "error",
    "import/newline-after-import": "error",
    "import/no-duplicates": "error",
    "import/no-self-import": "error",
    "no-console": "off",
    "no-debugger": "off"
  }
};
