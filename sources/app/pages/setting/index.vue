<template>
    <SettingDrawer />
    <VContainer>
        <NuxtPage v-model="editSetting" />
    </VContainer>
</template>

<script setup lang="ts">
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

const editSetting = ref<SettingContract>(lodCloneDeep(setting.value));
</script>
