<template>
    <template v-if="!isLoading">
        <ProjectDrawer />
        <NuxtPage />
    </template>
</template>

<script setup lang="ts">
import type { UnlistenFn } from "@tauri-apps/api/event";

const route = useRoute("index-project-id");
const i18n = useI18n();
const notify = useNotify();

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();
const taskStore = useTaskStore();
const ptySessionStore = usePtySessionStore();

const { isLoading, selectedProject } = storeToRefs(projectStore);

let unlistenPtyKilledEvent: UnlistenFn;
let unlistenPtySpawnedEvent: UnlistenFn;

onBeforeMount(async () => {
    await loadProject();
    await loadPlaceholders();
    await loadTasks();
    await loadPtySessions();
});

onBeforeUnmount(() => {
    selectedProject.value = {} as ProjectContract;

    unlistenPtyKilledEvent();
    unlistenPtySpawnedEvent();
});

const loadProject = async () => {
    isLoading.value = true;

    const projectResult = await commands.projectOpen(route.params.id);

    isLoading.value = false;

    if (projectResult.status == "error") {
        console.log(projectResult);

        notify.error(i18n.t("project.load.failed"), { error: projectResult.error });
        return;
    }

    selectedProject.value = projectResult.data;
};

const loadPlaceholders = async () => {
    await placeholderStore.loadAll();
};

const loadTasks = async () => {
    await taskStore.loadAll();
};

const loadPtySessions = async () => {
    await ptySessionStore.loadAll();

    unlistenPtyKilledEvent = await events.ptySessionKilledEvent.listen(() => {
        ptySessionStore.loadAll();
    });

    unlistenPtySpawnedEvent = await events.ptySessionSpawnedEvent.listen(() => {
        ptySessionStore.loadAll();
    });
};
</script>
