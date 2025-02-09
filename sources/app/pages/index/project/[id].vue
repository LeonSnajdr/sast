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
const ptySessionStore = usePtySessionStore();

const { isLoading, selectedProject } = storeToRefs(projectStore);

let unlistenPtyUpdatedEvents: UnlistenFn;

onBeforeMount(async () => {
    await loadProject();
    await loadPtySessions();
});

onBeforeUnmount(() => {
    selectedProject.value = {} as ProjectContract;

    unlistenPtyUpdatedEvents();
});

const loadProject = async () => {
    isLoading.value = true;

    const projectResult = await commands.projectOpen(route.params.id);

    isLoading.value = false;

    if (projectResult.status == "error") {
        console.log(projectResult);

        notify.error(i18n.t("project.load.failed"));
        return;
    }

    selectedProject.value = projectResult.data;
};

const loadPtySessions = async () => {
    await ptySessionStore.loadAll();

    unlistenPtyUpdatedEvents = await events.ptySessionsUpdatedEvent.listen(() => {
        ptySessionStore.loadAll();
    });
};
</script>
