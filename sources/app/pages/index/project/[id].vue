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
const ptySessionStore = usePtySessionStore();

const { isLoading, selectedProject } = storeToRefs(projectStore);

let unlistenTerminalDeletedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalShellSpawnedEvent: UnlistenFn;
let unlistenTerminalShellKilledEvent: UnlistenFn;

onBeforeMount(async () => {
    await loadProject();
    await loadPlaceholders();
    await loadTasks();
    await loadPtySessions();
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

const loadPtySessions = async () => {
    await ptySessionStore.loadAll();

    unlistenTerminalDeletedEvent = await events.terminalDeletedEvent.listen(() => {
        ptySessionStore.loadAll();
    });

    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen(() => {
        ptySessionStore.loadAll();
    });

    unlistenTerminalShellSpawnedEvent = await events.terminalShellSpawnedEvent.listen(() => {
        ptySessionStore.loadAll();
    });

    unlistenTerminalShellKilledEvent = await events.terminalShellKilledEvent.listen(() => {
        ptySessionStore.loadAll();
    });
};
</script>
