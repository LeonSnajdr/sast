<template>
    <template v-if="!isLoading && selectedProject.id">
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

let unlistenTerminalClosedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalShellStatusChangedEvent: UnlistenFn;

onBeforeMount(async () => {
    await loadProject();
    await loadPlaceholders();
    await loadTasks();
    await loadTerminals();
});

onBeforeUnmount(() => {
    selectedProject.value = {} as ProjectContract;

    unlistenTerminalClosedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalShellStatusChangedEvent();
});

const loadProject = async () => {
    console.log("load project", route.params.id);

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

    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen(() => {
        terminalStore.loadAll();
    });

    unlistenTerminalShellStatusChangedEvent = await events.terminalShellStatusChangedEvent.listen((eventData) => {
        terminalStore.statusChanged(eventData.payload.id, eventData.payload.status);
    });

    unlistenTerminalClosedEvent = await events.terminalClosedEvent.listen((eventData) => {
        terminalStore.closed(eventData.payload);
    });
};
</script>
