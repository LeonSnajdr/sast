<template>
    <NuxtPage />
</template>

<script setup lang="ts">
const { switchProjectDebounced } = useProject();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);

const hotkeyCleanups: (() => void)[] = [];

const cleanupHotkeys = () => {
    for (const cleanup of hotkeyCleanups) {
        cleanup();
    }
};

const setupHotkeys = () => {
    cleanupHotkeys();

    for (const project of allProjects.value) {
        if (!project.quickSwitchKeybind) continue;

        const cleanup = useHotkey(project.quickSwitchKeybind, async () => await switchProjectDebounced(project), { inputs: true });

        hotkeyCleanups.push(cleanup);
    }
};

onBeforeMount(() => {
    projectStore.loadAllProjects();

    setupHotkeys();
});

onBeforeUnmount(cleanupHotkeys);

watch(allProjects, setupHotkeys);
</script>
