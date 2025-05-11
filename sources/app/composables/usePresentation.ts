import type { Locale } from "vue-i18n";

export default function usePresentation() {
    const { setLocale } = useI18n();
    const { global: uiTheme } = useTheme();

    const settingStore = useSettingStore();

    const { setting } = storeToRefs(settingStore);

    const applySetting = () => {
        console.debug("Applying presentation using setting", setting.value.presentationLanguage, setting.value.presentationTheme);

        applyLangauge(setting.value.presentationLanguage);
        applyTheme(setting.value.presentationTheme);
    };

    const applyLangauge = (language: string) => {
        setLocale(language as Locale);
    };

    const applyTheme = (theme: string) => {
        uiTheme.name.value = theme;
    };

    return { applySetting, applyLangauge, applyTheme };
}
