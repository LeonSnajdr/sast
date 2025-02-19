<template>
    <VAppBar>
        <VAppBarTitle>
            {{ t("setting.presentation") }}
        </VAppBarTitle>
    </VAppBar>
    <SettingPresentationFieldLanguage v-model="setting.presentationLanguage" />
    <SettingPresentationFieldTheme v-model="setting.presentationTheme" />
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

watch(
    setting,
    async () => {
        const updateResult = await commands.settingUpdateOne({
            id: setting.value.id,
            presentationLanguage: setting.value.presentationLanguage,
            presentationTheme: setting.value.presentationTheme
        });

        if (updateResult.status === "error") {
            notify.error(t("error.default"), { error: updateResult.error });
            return;
        }
    },
    { deep: true }
);
</script>
