<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800">
        <VCard>
            <VCardTitle>
                <VIcon color="warning" icon="mdi-folder-marker" />
                {{ $t("title.select", { type: $t("project.singular") }) }}
            </VCardTitle>
            <VCardText>
                <VTextField
                    v-model="search"
                    :label="$t('search.filter')"
                    :persistentPlaceholder="false"
                    appendInnerIcon="mdi-filter-outline"
                    autofocus
                    clearable
                />
                <VList>
                    <VListItem v-for="project in filteredProjects" :key="project.id" :to="{ name: 'index-project-id-home', params: { id: project.id } }">
                        <VListItemTitle>{{ project.name }}</VListItemTitle>
                        <VListItemSubtitle>{{ useLocaleTimeAgo(project.dateLastOpened) }}</VListItemSubtitle>
                        <template #append>
                            <VIcon icon="mdi-arrow-right" />
                        </template>
                    </VListItem>

                    <VListSubheader v-if="filteredProjects.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
                </VList>
            </VCardText>
            <VCardActions>
                <BaseBtnIcon @click="isDialogOpen = false">{{ $t("action.close") }}</BaseBtnIcon>
            </VCardActions>
        </VCard>
    </VDialog>
</template>

<script setup lang="ts">
const { t } = useI18n();
const notify = useNotify();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const search = ref<string>();

const projects = ref<ProjectContract[]>([]);

onBeforeMount(() => {
    loadProjects();
});

const loadProjects = async () => {
    isLoading.value = true;

    const projectsResult = await commands.projectGetAll();

    isLoading.value = false;

    if (projectsResult.status == "error") {
        notify.error(t("project.load.failed"), { error: projectsResult.error });
        return;
    }

    projects.value = projectsResult.data;
};

const filteredProjects = computed(() => {
    const lowerCaseSearch = search.value?.toLocaleLowerCase() ?? "";

    return projects.value.filter((x) => {
        const isMatchingSearch = x.name.toLowerCase().includes(lowerCaseSearch);
        const isCurrentProject = selectedProject.value.id === x.id;

        return isMatchingSearch && !isCurrentProject;
    });
});

watch(isDialogOpen, () => {
    search.value = undefined;
});
</script>
