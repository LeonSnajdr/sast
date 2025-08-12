import type { Locale } from "vue-i18n";

export default function usePresentation() {
    const i18n = useI18n();
    const theme = useTheme();

    const settingStore = useSettingStore();

    const { setting } = storeToRefs(settingStore);

    const applySetting = () => {
        console.debug("Applying presentation using setting", setting.value.presentationLanguage, setting.value.presentationTheme);

        applyLangauge(setting.value.presentationLanguage);
        applyTheme(setting.value.presentationTheme);
    };

    const applyLangauge = (language: string) => {
        i18n.setLocale(language as Locale);
    };

    const applyTheme = (themeName: string) => {
        theme.change(themeName);
    };

    return { applySetting, applyLangauge, applyTheme };
}
