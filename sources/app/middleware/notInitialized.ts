export default defineNuxtRouteMiddleware(async () => {
    const isSettingInitalizedResult = await commands.getIsSettingInitialized();

    if (isSettingInitalizedResult.status == "error") {
        return navigateTo({ name: "error" });
    }

    if (isSettingInitalizedResult.data) {
        return abortNavigation();
    }
});
