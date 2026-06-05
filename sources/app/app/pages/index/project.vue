<template>
    <ProjectWindowBarContent />
    <NuxtPage />
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const { switchProjectDebounced } = useProject();

const terminalStore = useTerminalStore();
const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);

let unlistenTerminalClosedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalUpdatedEvent: UnlistenFn;
let unlistenTerminalShellStatusChangedEvent: UnlistenFn;

const hotkeyCleanups: (() => void)[] = [];

onBeforeMount(async () => {
    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen((eventData) => {
        terminalStore.created(eventData.payload);
    });

    unlistenTerminalUpdatedEvent = await events.terminalUpdatedEvent.listen((eventData) => {
        terminalStore.updated(eventData.payload);
    });

    unlistenTerminalClosedEvent = await events.terminalClosedEvent.listen((eventData) => {
        terminalStore.closed(eventData.payload);
    });

    unlistenTerminalShellStatusChangedEvent = await events.terminalShellStatusChangedEvent.listen((eventData) => {
        terminalStore.statusChanged(eventData.payload);
    });

    await terminalStore.loadAll();
});

onBeforeUnmount(() => {
    unlistenTerminalClosedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalUpdatedEvent();
    unlistenTerminalShellStatusChangedEvent();
});

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
    setupHotkeys();
});

onBeforeUnmount(cleanupHotkeys);

watch(allProjects, setupHotkeys);
</script>
