export default function usePresentation() {
    const { setLocale } = useI18n();
    const { global: uiTheme } = useTheme();

    const settingStore = useSettingStore();

    const { setting } = storeToRefs(settingStore);

    const applyUsingSetting = () => {
        console.debug("Applying presentation using setting", setting.value.presentationLanguage, setting.value.presentationTheme);

        applyLangauge(setting.value.presentationLanguage);
        applyTheme(setting.value.presentationTheme);
    };

    const applyLangauge = (language: string) => {
        setLocale(language);
    };

    const applyTheme = (theme: string) => {
        uiTheme.name.value = theme;
    };

    return { applyUsingSetting, applyLangauge, applyTheme };
}
