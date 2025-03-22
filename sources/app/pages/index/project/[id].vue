<template>
    <template v-if="!isLoading">
        <ProjectDrawer />
        <NuxtPage />
    </template>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const route = useRoute("index-project-id");

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const terminalStore = useTerminalStore();

const { isLoading, selectedProject } = storeToRefs(projectStore);

let unlistenTerminalDeletedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalShellSpawnedEvent: UnlistenFn;
let unlistenTerminalShellKilledEvent: UnlistenFn;

onBeforeMount(async () => {
    await loadProject();
    await loadPlaceholders();
    await loadTasks();
    await loadTerminals();
});

onBeforeUnmount(() => {
    selectedProject.value = {} as ProjectContract;

    unlistenTerminalDeletedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalShellKilledEvent();
    unlistenTerminalShellSpawnedEvent();
});

const loadProject = async () => {
    await projectStore.loadProject(route.params.id);
};

const loadPlaceholders = async () => {
    await placeholderStore.loadAll();
};

const loadTasks = async () => {
    await taskStore.loadAll();
};

const loadTerminals = async () => {
    await terminalStore.loadAll();

    unlistenTerminalDeletedEvent = await events.terminalDeletedEvent.listen(() => {
        terminalStore.loadAll();
    });

    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen(() => {
        terminalStore.loadAll();
    });

    unlistenTerminalShellSpawnedEvent = await events.terminalShellSpawnedEvent.listen(() => {
        terminalStore.loadAll();
    });

    unlistenTerminalShellKilledEvent = await events.terminalShellKilledEvent.listen(() => {
        terminalStore.loadAll();
    });
};
</script>
