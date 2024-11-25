export const useSettingStore = defineStore("setting", () => {
    const setting = ref<SettingContract>({} as SettingContract);

    return { setting };
});
