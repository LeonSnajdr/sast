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
let unlistenTerminalUpdatedEvent: UnlistenFn;
let unlistenTerminalShellStatusChangedEvent: UnlistenFn;

let unlistenTaskSetSessionStartedEvent: UnlistenFn;
let unlistenTaskSetSessionFinishedEvent: UnlistenFn;
let unlistenTaskSetSessionTaskUpdatedEvent: UnlistenFn;

definePageMeta({
    redirect(to) {
        const id = (to.params as { id: string }).id;
        return { name: "index-project-id-terminal", params: { id } };
    }
});

onBeforeMount(async () => {
    await projectStore.openProject(route.params.id);
    await placeholderStore.loadAll();
    await taskStore.loadAll();
    await taskSetStore.loadAll();

    // NOTE: Terminal/TaskSet logic is required globaly in the project to show indicators and status in many places
    await loadTerminals();
    await loadTaskSetSessions();
});

onBeforeUnmount(() => {
    unlistenTerminalClosedEvent();
    unlistenTerminalCreatedEvent();
    unlistenTerminalUpdatedEvent();
    unlistenTerminalShellStatusChangedEvent();
    unlistenTaskSetSessionStartedEvent();
    unlistenTaskSetSessionFinishedEvent();
    unlistenTaskSetSessionTaskUpdatedEvent();
});

const loadTerminals = async () => {
    await terminalStore.loadAll();

    unlistenTerminalCreatedEvent = await events.terminalCreatedEvent.listen(() => {
        terminalStore.loadAll();
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
};

const loadTaskSetSessions = async () => {
    await taskSetSessionStore.loadAll();

    unlistenTaskSetSessionStartedEvent = await events.taskSetSessionStartedEvent.listen(() => {
        taskSetSessionStore.loadAll();
    });

    unlistenTaskSetSessionFinishedEvent = await events.taskSetSessionFinishedEvent.listen(() => {
        taskSetSessionStore.loadAll();
    });

    unlistenTaskSetSessionTaskUpdatedEvent = await events.taskSetSessionUpdatedEvent.listen((eventData) => {
        taskSetSessionStore.updated(eventData.payload);
    });
};
</script>
