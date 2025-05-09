<template>
    <template v-if="selectedProject.id">
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

const { selectedProject } = storeToRefs(projectStore);

let unlistenTerminalClosedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalShellStatusChangedEvent: UnlistenFn;

onBeforeMount(async () => {
    await openProject();
    await loadPlaceholders();
    await loadTasks();
    await loadTerminals();
});

onBeforeUnmount(() => {
    unlistenTerminalClosedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalShellStatusChangedEvent();
});

const openProject = async () => {
    await projectStore.openProject(route.params.id);
};

const loadPlaceholders = async () => {
    await placeholderStore.loadAll();
};

const loadTasks = async () => {
    await taskStore.loadAll();
};

const loadTerminals = async () => {
    // NOTE: Terminal logic is required globaly in the project to show indicators and status in some places

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
