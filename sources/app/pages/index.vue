<template>
    <NuxtPage v-if="isInitialized" />
</template>

<script setup lang="ts">
const presentation = usePresentation();
const settingStore = useSettingStore();

const { setting } = storeToRefs(settingStore);

const isInitialized = ref(false);

onBeforeMount(async () => {
    initialize();
});

const initialize = async () => {
    const settingResult = await commands.getSetting();

    if (settingResult.status == "error") {
        return navigateTo({ name: "error" });
    }

    if (settingResult.data == null) {
        return navigateTo({ name: "initialize" });
    }

    setting.value = settingResult.data!;

    presentation.applySetting();

    isInitialized.value = true;
};
</script>
