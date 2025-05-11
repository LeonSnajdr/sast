import { defineVuetifyConfiguration } from "vuetify-nuxt-module/custom-configuration";

export default defineVuetifyConfiguration({
    theme: {
        defaultTheme: "dark",
        // https://github.com/vuetifyjs/vuetify/blob/master/packages/vuetify/src/composables/theme.ts#L128
        themes: {
            light: {
                colors: {
                    primary: "#303a4e",
                    "primary-lighten-1": "#636b7a",
                    "primary-lighten-2": "#969ba5",
                    "primary-lighten-3": "#e4e5e8",
                    secondary: "#8098a7",
                    "secondary-lighten-1": "#9fb1bd",
                    "secondary-lighten-2": "#bfcbd2",
                    "secondary-lighten-3": "#dee4e8",
                    "secondary-lighten-4": "#f2f5f6",
                    tertiary: "#599b8b",
                    "tertiary-lighten-1": "#82b4a8",
                    "tertiary-lighten-2": "#abccc4",
                    "tertiary-lighten-3": "#d4e5e1",
                    background: "#f9fafc",
                    surface: "#ffffff",
                    "on-surface": "#1c1c1c",
                    error: "#e52727",
                    "error-lighten-1": "#dcbaba",
                    warning: "#dc8b22",
                    "warning-lighten-1": "#e2d4c1",
                    success: "#7d9e39",
                    "success-lighten-1": "#d7e1c2",
                    info: "#2196f3",
                    "terminal-background": "#f9fafc",
                    "terminal-foreground": "#383a42",
                    "terminal-cursor": "#4f525d",
                    "terminal-selectionForeground": "#383a42",
                    "terminal-selectionBackground": "#999a9e",
                    "terminal-selectionInactiveBackground": "#999a9e",
                    "terminal-white": "#d3d7cf",
                    "terminal-brightWhite": "#eeeeec",
                    "terminal-black": "#383a42",
                    "terminal-brightBlack": "#4f525d",
                    "terminal-blue": "#0184bc",
                    "terminal-brightBlue": "#61afef",
                    "terminal-cyan": "#0997b3",
                    "terminal-brightCyan": "#56b5c1",
                    "terminal-green": "#50a14f",
                    "terminal-brightGreen": "#98c379",
                    "terminal-magenta": "#a626a4",
                    "terminal-brightMagenta": "#c577dd",
                    "terminal-red": "#e45649",
                    "terminal-brightRed": "#df6c75",
                    "terminal-yellow": "#c18301",
                    "terminal-brightYellow": "#e4c07a"
                },
                variables: {
                    "border-color": "#000000",
                    "high-emphasis-opacity": "1",
                    "medium-emphasis-opacity": 0.73,
                    "disabled-opacity": 0.51
                }
            },
            dark: {
                colors: {
                    primary: "#4271c4",
                    "primary-lighten-1": "#345898",
                    "primary-lighten-2": "#26406c",
                    "primary-lighten-3": "#182840",
                    secondary: "#8098a7",
                    "secondary-lighten-1": "#627682",
                    "secondary-lighten-2": "#45545e",
                    "secondary-lighten-3": "#273239",
                    "secondary-lighten-4": "#22282b",
                    tertiary: "#599b8b",
                    "tertiary-lighten-1": "#45786d",
                    "tertiary-lighten-2": "#31564f",
                    "tertiary-lighten-3": "#1e3332",
                    background: "#191919",
                    surface: "#2c2c2c",
                    "on-surface": "#ffffff",
                    error: "#e52727",
                    "error-lighten-1": "#dcbaba",
                    warning: "#dc8b22",
                    "warning-lighten-1": "#e2d4c1",
                    success: "#c9f56f",
                    "success-lighten-1": "#d7e1c2",
                    info: "#2196f3",
                    "terminal-background": "#191919",
                    "terminal-foreground": "#e5e5e5",
                    "terminal-cursor": "#4f525d",
                    "terminal-selectionForeground": "#d4d4d4",
                    "terminal-selectionBackground": "#8f8f8f",
                    "terminal-selectionInactiveBackground": "#808080",
                    "terminal-white": "#e5e5e5",
                    "terminal-brightWhite": "#e5e5e5",
                    "terminal-black": "#000000",
                    "terminal-brightBlack": "#666666",
                    "terminal-blue": "#2472c8",
                    "terminal-brightBlue": "#3b8eea",
                    "terminal-cyan": "#11a8cd",
                    "terminal-brightCyan": "#29b8db",
                    "terminal-green": "#0dbc79",
                    "terminal-brightGreen": "#23d18b",
                    "terminal-magenta": "#bc3fbc",
                    "terminal-brightMagenta": "#d670d6",
                    "terminal-red": "#cd3131",
                    "terminal-brightRed": "#f14c4c",
                    "terminal-yellow": "#e5e510",
                    "terminal-brightYellow": "#f5f543"
                }
            }
        }
    },
    defaults: {
        VSnackbar: {
            timeout: 5000,
            color: "info",
            location: "bottom right",
            class: "text-break"
        },
        VBanner: {
            style: "flex: unset",
            rounded: true,
            border: true,
            elevation: 1
        },
        VSwitch: {
            color: "primary",
            hideDetails: "auto"
        },
        VCheckbox: {
            color: "primary",
            hideDetails: "auto"
        },
        VTextField: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true,
            VIcon: { size: "small" }
        },
        VSelect: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true,
            VIcon: { size: "small" }
        },
        VCombobox: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true,
            VIcon: { size: "small" }
        },
        VAutocomplete: {
            variant: "outlined",
            autoSelectFirst: true,
            hideDetails: "auto",
            persistentPlaceholder: true,
            VIcon: { size: "small" }
        },
        VTextarea: {
            variant: "outlined",
            hideDetails: "auto",
            persistentPlaceholder: true
        },
        VField: {
            variant: "outlined"
        },
        VTable: {
            VPagination: {
                density: "comfortable",
                showFirstLastPage: true,
                totalVisible: 6
            }
        },
        VDataTable: {
            density: "comfortable",
            hover: true
        },
        VDataTableServer: {
            density: "comfortable",
            hover: true
        },
        VDataTableVirtual: {
            density: "comfortable",
            hover: true
        },
        VChip: {
            color: "primary",
            variant: "flat"
        },
        VCard: {
            class: "d-flex flex-column",

            VCardTitle: {
                class: "text-h6 font-weight-light d-flex ga-2 align-center",

                VIcon: {
                    size: "small"
                }
            }
        },
        VNavigationDrawer: {
            railWidth: "72",
            width: "210",
            VList: {
                class: "mt-n2"
            },
            VListItem: {
                VListItemTitle: {
                    class: "text-caption d-flex align-center"
                }
            }
        },
        VSystemBar: {
            class: "border-b",
            color: "surface"
        },
        VProgressCircular: {
            size: 20
        },
        VProgressLinear: {
            height: 2,
            color: "secondary"
        },
        VTabs: {
            density: "compact",
            color: "primary",

            VTab: {
                class: "font-weight-bold px-2"
            }
        },
        VExpansionPanel: {
            VExpansionPanelText: {
                class: "pt-2"
            }
        },
        VDialog: {
            transition: false,
            VCardTitle: {
                VIcon: {
                    class: "ml-1"
                }
            },
            VCardActions: {
                class: "mx-4"
            }
        },
        VTooltip: {
            openDelay: 400
        },
        VAppBar: {
            density: "compact",
            border: "b",
            class: "pr-3 ",
            elevation: "0",

            VAppBarTitle: {
                class: "text-body-1"
            }
        },
        VBtn: {
            class: "text-none"
        }
    }
});
