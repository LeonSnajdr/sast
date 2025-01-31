import { defineVuetifyConfiguration } from "vuetify-nuxt-module/custom-configuration";

export default defineVuetifyConfiguration({
    labComponents: ["VSnackbarQueue"],
    theme: {
        defaultTheme: "dark",
        // https://github.com/vuetifyjs/vuetify/blob/master/packages/vuetify/src/composables/theme.ts#L128
        themes: {
            light: {
                colors: {
                    primary: "#303A4E",
                    "primary-lighten-1": "#636B7A",
                    "primary-lighten-2": "#969BA5",
                    "primary-lighten-3": "#E4E5E8",
                    secondary: "#8098A7",
                    "secondary-lighten-1": "#9FB1BD",
                    "secondary-lighten-2": "#BFCBD2",
                    "secondary-lighten-3": "#DEE4E8",
                    "secondary-lighten-4": "#F2F5F6",
                    tertiary: "#599B8B",
                    "tertiary-lighten-1": "#82B4A8",
                    "tertiary-lighten-2": "#ABCCC4",
                    "tertiary-lighten-3": "#D4E5E1",
                    background: "#F9FAFC",
                    surface: "#FFFFFF",
                    "on-surface": "#1C1C1C",
                    error: "#E52727",
                    "error-lighten-1": "#DCBABA",
                    warning: "#DC8B22",
                    "warning-lighten-1": "#E2D4C1",
                    success: "#7D9E39",
                    "success-lighten-1": "#D7E1C2",
                    info: "#2196F3",
                    "terminal-selectionBackground": "#999a9e",
                    "terminal-white": "#fafafa",
                    "terminal-brightWhite": "#ffffff",
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
                    primary: "#4271C4",
                    "primary-lighten-1": "#345898",
                    "primary-lighten-2": "#26406C",
                    "primary-lighten-3": "#182840",
                    secondary: "#8098A7",
                    "secondary-lighten-1": "#627682",
                    "secondary-lighten-2": "#45545E",
                    "secondary-lighten-3": "#273239",
                    "secondary-lighten-4": "#22282B",
                    tertiary: "#599B8B",
                    "tertiary-lighten-1": "#45786D",
                    "tertiary-lighten-2": "#31564F",
                    "tertiary-lighten-3": "#1E3332",
                    background: "#191919",
                    surface: "#2C2C2C",
                    "on-surface": "#FFFFFF",
                    error: "#E52727",
                    "error-lighten-1": "#DCBABA",
                    warning: "#DC8B22",
                    "warning-lighten-1": "#E2D4C1",
                    success: "#C9F56F",
                    "success-lighten-1": "#D7E1C2",
                    info: "#2196F3"
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

            VListItem: {
                VListItemTitle: {
                    class: "text-caption"
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
            VCardTitle: {
                VIcon: {
                    class: "ml-n1"
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
        }
    }
});
