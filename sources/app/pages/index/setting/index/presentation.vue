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
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);
</script>
