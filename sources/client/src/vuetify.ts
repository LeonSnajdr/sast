import "@mdi/font/css/materialdesignicons.css";
import "vuetify/styles";

import { createVuetify } from "vuetify";
import dateAdapter from "@/plugins/DateAdapter";
import i18n from "@/i18n";
import { de, en } from "vuetify/locale";

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
    date: {
        adapter: dateAdapter
    },
    locale: {
        messages: { de: de, en: en },
        locale: i18n.global.locale.value
    },
    theme: {
        defaultTheme: "light",
        // https://github.com/vuetifyjs/vuetify/blob/master/packages/vuetify/src/composables/theme.ts#L128
        themes: {
            light: {
                colors: {
                    primary: "#303A4E",
                    secondary: "#8098A7",
                    tertiary: "#599B8B",
                    background: "#F9FAFC",
                    surface: "#FFFFFF",
                    error: "#E52727",
                    warning: "#DC8B22",
                    success: "#7D9E39",
                    info: "#2196F3"
                }
            },
            dark: {
                colors: {
                    primary: "#2F81F7",
                    secondary: "#30363D",
                    tertiary: "#345F4F",
                    background: "#010409",
                    surface: "#161B22",
                    error: "#E52727",
                    warning: "#DC8B22",
                    success: "#7D9E39",
                    info: "#2196F3"
                }
            }
        }
    },
    defaults: {
        VIcon: {
            size: 18
        },
        VBtn: {
            variant: "flat"
        },
        VSnackbar: {
            timeout: 5000,
            color: "info",
            location: "bottom right"
        },
        VSwitch: {
            color: "primary",
            hideDetails: "auto"
        },
        VTextField: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VSelect: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VCombobox: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VAutocomplete: {
            variant: "outlined",
            autoSelectFirst: true,
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VTextarea: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VChip: {
            color: "primary",
            size: "small",
            variant: "elevated"
        },
        VCard: {
            class: "d-flex flex-column",

            VCardTitle: {
                class: "text-medium-emphasis font-weight-light d-flex align-center"
            }
        },
        VNavigationDrawer: {
            class: "elevation-1 pa-3",
            railWidth: "80",
            rounded: true,

            VDivider: {
                class: "my-2"
            },

            VListItem: {
                rounded: true,
                class: "px-3 my-1 pt-2"
            }
        },
        VProgressCircular: {
            size: 20
        },
        VProgressLinear: {
            height: 2,
            color: "secondary"
        }
    }
});
