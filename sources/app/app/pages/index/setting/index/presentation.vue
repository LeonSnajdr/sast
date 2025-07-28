<template>
    <VAppBar>
        <VAppBarTitle>
            {{ $t("setting.presentation") }}
        </VAppBarTitle>
    </VAppBar>
    <VContainer>
        <SettingPresentationFieldLanguage v-model="setting.presentationLanguage" />
        <SettingPresentationFieldTheme v-model="setting.presentationTheme" />
    </VContainer>
</template>

<script setup lang="ts">
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);
</script>
