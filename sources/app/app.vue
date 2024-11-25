<template>
    <VApp>
        <WindowBar />
        <NuxtLayout>
            <VMain>
                <NotificationView />
                <NuxtPage />
            </VMain>
        </NuxtLayout>
    </VApp>
</template>

<script setup lang="ts">
const presentation = usePresentation();

const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

onBeforeMount(async () => {
    await ensureInitialization();
});

const ensureInitialization = async () => {
    const settingResult = await commands.getSetting();

    if (settingResult.status == "error") {
        navigateTo({ name: "error" });
        return null;
    }

    if (settingResult.data == null) {
        navigateTo({ name: "initialize" });
    }

    setting.value = settingResult.data!;

    presentation.applySetting();
};
</script>
