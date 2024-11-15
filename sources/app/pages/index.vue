<template>
    <div />
</template>

<script setup lang="ts">
const isSettingInitalized = ref(false);

onBeforeMount(async () => {
    await loadIsSettingInitalized();
    redirect();
});

const loadIsSettingInitalized = async () => {
    const isSettingInitalizedResult = await commands.getIsSettingInitialized();

    if (isSettingInitalizedResult.status == "error") {
        return;
    }

    isSettingInitalized.value = isSettingInitalizedResult.data;
};

const redirect = () => {
    if (isSettingInitalized.value) {
        navigateTo({ name: "welcome" });
        return;
    }

    navigateTo({ name: "initialize" });
};
</script>
