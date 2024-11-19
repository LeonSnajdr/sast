export default defineNuxtRouteMiddleware(async () => {
    const settingStore = useSettingStore();

    const { isInitialized } = storeToRefs(settingStore);

    await settingStore.loadIsInitialized();

    if (isInitialized.value) {
        return abortNavigation();
    }
});
