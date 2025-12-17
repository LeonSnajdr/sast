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
const taskSetStore = useTaskSetStore();
const taskSetSessionStore = useTaskSetSessionStore();

const { selectedProject } = storeToRefs(projectStore);

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

    // NOTE: TaskSet logic is required globaly in the project to show indicators and status in many places
    await loadTaskSetSessions();
});

onBeforeUnmount(() => {
    unlistenTaskSetSessionStartedEvent();
    unlistenTaskSetSessionFinishedEvent();
    unlistenTaskSetSessionTaskUpdatedEvent();
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
</script>
