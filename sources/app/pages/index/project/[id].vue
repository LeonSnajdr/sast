<template>
    <template v-if="selectedProject.id">
        <ProjectDrawer :key="selectedProject.id" />
        <NuxtPage :key="selectedProject.id" />
    </template>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const route = useRoute("index-project-id");

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const taskSetStore = useTaskSetStore();
const taskSetSessionStore = useTaskSetSessionStore();
const terminalStore = useTerminalStore();

const { selectedProject } = storeToRefs(projectStore);

let unlistenTerminalClosedEvent: UnlistenFn;
let unlistenTerminalCreatedEvent: UnlistenFn;
let unlistenTerminalShellStatusChangedEvent: UnlistenFn;

let unlistenTaskSetSessionStartedEvent: UnlistenFn;
let unlistenTaskSetSessionFinishedEvent: UnlistenFn;
let unlistenTaskSetSessionTaskStatusChangedEvent: UnlistenFn;

onBeforeMount(async () => {
    await projectStore.openProject(route.params.id);
    await placeholderStore.loadAll();
    await taskStore.loadAll();
    await taskSetStore.loadAll();
    await loadTerminals();
    await loadTaskSetSessions();
});

onBeforeUnmount(() => {
    unlistenTerminalClosedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalShellStatusChangedEvent();
    unlistenTaskSetSessionStartedEvent();
    unlistenTaskSetSessionFinishedEvent();
    unlistenTaskSetSessionTaskStatusChangedEvent();
});

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

const loadTaskSetSessions = async () => {
    await taskSetSessionStore.loadAll();

    unlistenTaskSetSessionStartedEvent = await events.taskSetSessionStartedEvent.listen(() => {
        taskSetSessionStore.loadAll();
    });

    unlistenTaskSetSessionFinishedEvent = await events.taskSetSessionFinishedEvent.listen(() => {
        taskSetSessionStore.loadAll();
    });

    unlistenTaskSetSessionTaskStatusChangedEvent = await events.taskSetSessionTaskStatusChangedEvent.listen((eventData) => {
        // TODO do not a full reload here
        taskSetSessionStore.loadAll();
    });
};
</script>
