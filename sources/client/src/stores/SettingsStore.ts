import type { Settings } from "@/bindings";
import * as commands from "@/bindings";
import { useI18n } from "vue-i18n";
import { useTheme } from "vuetify/dist/vuetify-labs.js";

export const useSettingsStore = defineStore("settingsStore", () => {
    const notify = useNotificationStore();
    const { locale } = useI18n();
    const theme = useTheme();

    const settings = ref<Settings>();

    const loadSettings = async () => {
        try {
            settings.value = await commands.getSettings();

            console.log(settings.value);
        } catch (error) {
            console.error("Loading settings failed", error);
            notify.error("settingsStore.load.settings.failed");
        }
    };

    const initializeAppWithSettings = async () => {
        await loadSettings();

        locale.value = settings.value.language;
        theme.global.name.value = settings.value.theme;
    };

    return {
        settings,
        initializeAppWithSettings
    };
});
