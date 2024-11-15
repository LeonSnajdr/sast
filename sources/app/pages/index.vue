<template>
    <Welcome v-if="!isLoading && isSettingInitalized" />
    <SettingInitialize v-if="!isLoading && !isSettingInitalized" />
</template>

<script setup lang="ts">
const isLoading = ref(true);
const isSettingInitalized = ref(false);

onBeforeMount(async () => {
    isLoading.value = true;
    await loadIsSettingInitalized();
    isLoading.value = false;
});

const loadIsSettingInitalized = async () => {
    const isSettingInitalizedResult = await commands.getIsSettingInitialized();

    if (isSettingInitalizedResult.status == "error") {
        return;
    }

    isSettingInitalized.value = isSettingInitalizedResult.data;
};
</script>
