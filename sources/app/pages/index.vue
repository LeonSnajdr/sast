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
const projectStore = useProjectStore();

const { setting } = storeToRefs(settingStore);
const { lastOpenedProject } = storeToRefs(projectStore);

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

    await projectStore.loadLastOpenedProject();

    if (!setting.value.behaviorOpenWelcome && lastOpenedProject.value) {
        await navigateTo({ name: "index-project-id-home", params: { id: lastOpenedProject.value.id } });
    }

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
