export const useSettingStore = defineStore("setting", () => {
    const isInitialized = ref(false);
    const setting = ref<SettingContract>({} as SettingContract);

    const loadIsInitialized = async () => {
        const isInitalizedResult = await commands.getIsSettingInitialized();

        if (isInitalizedResult.status == "error") {
            navigateTo({ name: "error" });
            return;
        }

        isInitialized.value = isInitalizedResult.data;
    };

    const loadSetting = async () => {
        const settingResult = await commands.getSetting();

        if (settingResult.status == "error") {
            navigateTo({ name: "error" });
            return;
        }

        setting.value = settingResult.data;
    };

    return { loadIsInitialized, loadSetting, isInitialized, setting };
});
