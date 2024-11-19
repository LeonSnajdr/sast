export default defineNuxtRouteMiddleware(async () => {
    const settingStore = useSettingStore();

    const { isInitialized } = storeToRefs(settingStore);

    await settingStore.loadIsInitialized();

    if (!isInitialized.value) {
        return navigateTo({ name: "initialize" });
    }

    await settingStore.loadSetting();
});
