export default defineNuxtRouteMiddleware(async () => {
    const settingStore = useSettingStore();
    const { setting } = storeToRefs(settingStore);

    const settingResult = await commands.getSetting();

    if (settingResult.status == "error") {
        return navigateTo({ name: "error" });
    }

    if (settingResult.data == null) {
        return navigateTo({ name: "initialize" });
    }

    setting.value = settingResult.data!;
});
