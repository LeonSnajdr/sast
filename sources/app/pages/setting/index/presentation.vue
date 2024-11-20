<template>
    <SettingPresentationLanguage v-model="editSetting.presentationLanguage" />
    <SettingPresentationTheme v-model="editSetting.presentationTheme" />
</template>

<script setup lang="ts">
const editSetting = defineModel<SettingContract>({ required: true });
</script>
