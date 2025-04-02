<template>
    <VAppBar>
        <TerminalTabs />
        <VSpacer />
        <TerminalActionSpawn />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

let unlistenPtySpawnedEvent: UnlistenFn;

onMounted(async () => {
    unlistenPtySpawnedEvent = await events.terminalCreatedEvent.listen((event) => {
        navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: event.payload } });
    });
});

onBeforeUnmount(() => {
    unlistenPtySpawnedEvent();
});
</script>
