<template>
    <SettingPresentationLanguage v-model="setting.presentationLanguage" />
    <SettingPresentationTheme v-model="setting.presentationTheme" />
</template>

<script setup lang="ts">
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);
</script>
