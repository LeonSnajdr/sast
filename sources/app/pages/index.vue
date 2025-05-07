<template>
    <div class="h-100 overflow-auto">
        <NuxtPage v-if="isInitialized" />
    </div>
</template>

<script setup lang="ts">
const route = useRoute();
const presentation = usePresentation();

const settingStore = useSettingStore();
const updateStore = useUpdateStore();

const { setting } = storeToRefs(settingStore);

const isInitialized = ref(false);

onBeforeMount(async () => {
    initialize();
});

const initialize = async () => {
    const settingResult = await commands.settingGetDefault();

    if (settingResult.status == "error") {
        return await navigateTo({ name: "error" });
    }

    if (settingResult.data == null) {
        return await navigateTo({ name: "initialize" });
    }

    setting.value = settingResult.data!;

    presentation.applySetting();

    checkForUpdate();

    isInitialized.value = true;
};

const checkForUpdate = async () => {
    await updateStore.loadUpdate();
    updateStore.notifyIfUpdateIsAvailable();
};

useKeybind(["control", ","], () => {
    if (route.name === "initialize") return;

    return navigateTo({ name: "index-setting-index-presentation" });
});
</script>
