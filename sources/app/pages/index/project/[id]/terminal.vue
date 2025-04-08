<template>
    <VAppBar>
        <TerminalTabs />
        <VSpacer />
        <TerminalActionCreate />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

let unlistenTerminalCreatedEvent: UnlistenFn;

onMounted(async () => {
    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen((event) => {
        navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: selectedProject.value.id, terminalId: event.payload } });
    });
});

onBeforeUnmount(() => {
    unlistenTerminalCreatedEvent();
});
</script>
