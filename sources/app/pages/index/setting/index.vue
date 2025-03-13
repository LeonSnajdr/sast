<template>
    <SettingDrawer />
    <VContainer class="h-100">
        <NuxtPage />
    </VContainer>
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

watch(
    setting,
    async () => {
        const updateContract: SettingUpdateContract = {
            id: setting.value.id,
            presentationLanguage: setting.value.presentationLanguage,
            presentationTheme: setting.value.presentationTheme,
            behaviorOpenWelcome: setting.value.behaviorOpenWelcome
        };

        const updateResult = await commands.settingUpdateOne(updateContract);

        if (updateResult.status === "error") {
            notify.error(t("error.default"), { error: updateResult.error });
            return;
        }
    },
    { deep: true }
);
</script>
