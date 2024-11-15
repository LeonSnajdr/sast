<template>
    <ProjectDrawer />
    <VContainer>
        <NuxtPage />
    </VContainer>
</template>

<script setup lang="ts">
const route = useRoute("project-id");
const i18n = useI18n();
const notify = useNotify();

const projectStore = useProjectStore();

const { isLoading, selectedProject } = storeToRefs(projectStore);

onBeforeMount(() => {
    loadProject();
});

onBeforeUnmount(() => {
    selectedProject.value = {} as ProjectContract;
});

const loadProject = async () => {
    isLoading.value = true;

    const projectResult = await commands.openProject(route.params.id);

    isLoading.value = false;

    if (projectResult.status == "error") {
        console.log(projectResult);

        notify.error(i18n.t("project.load.failed"));
        return;
    }

    selectedProject.value = projectResult.data;
};

definePageMeta({
    middleware: "initialize"
});
</script>
