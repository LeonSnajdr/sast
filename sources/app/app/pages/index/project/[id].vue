<template>
    <ProjectDrawer />
    <NuxtPage />
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const route = useRoute("index-project-id");

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const taskSetStore = useTaskSetStore();
const taskSetSessionStore = useTaskSetSessionStore();

let unlistenTaskSetSessionStartedEvent: UnlistenFn | undefined;
let unlistenTaskSetSessionFinishedEvent: UnlistenFn | undefined;
let unlistenTaskSetSessionTaskUpdatedEvent: UnlistenFn | undefined;

definePageMeta({
    redirect(to) {
        const id = (to.params as { id: string }).id;
        return { name: "index-project-id-terminal", params: { id } };
    }
});

onBeforeUnmount(() => {
    unlistenTaskSetSessionStartedEvent?.();
    unlistenTaskSetSessionFinishedEvent?.();
    unlistenTaskSetSessionTaskUpdatedEvent?.();
});

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

await projectStore.openProject(route.params.id);
await Promise.all([placeholderStore.loadAll(), taskStore.loadAll(), taskSetStore.loadAll(), loadTaskSetSessions()]);
</script>
