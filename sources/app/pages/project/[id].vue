<template>
    <div>
        <ProjectDrawer />
        <VContainer>
            <NuxtPage />
        </VContainer>
    </div>
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

const loadProject = async () => {
    isLoading.value = true;

    const projectResult = await commands.getProject(Number.parseInt(route.params.id));

    isLoading.value = false;

    if (projectResult.status == "error") {
        console.log(projectResult);

        notify.error(i18n.t("project.load.failed"));
        return;
    }

    selectedProject.value = projectResult.data;
};
</script>
