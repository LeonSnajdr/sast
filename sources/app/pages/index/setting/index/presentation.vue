<template>
    <VAppBar>
        <VAppBarTitle>
            {{ t("setting.presentation") }}
        </VAppBarTitle>
    </VAppBar>
    <SettingPresentationLanguage v-model="setting.presentationLanguage" />
    <SettingPresentationTheme v-model="setting.presentationTheme" />
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

watch(
    setting,
    async () => {
        const updateResult = await commands.updateSetting({
            id: setting.value.id,
            presentationLanguage: setting.value.presentationLanguage,
            presentationTheme: setting.value.presentationTheme
        });

        if (updateResult.status === "error") {
            notify.error(t("error.default"));
            console.log(updateResult);

            return;
        }
    },
    { deep: true }
);
</script>
