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
                    <VListItem v-for="project in filteredProjects" :key="project.id" @click="switchProject(project)">
                        <VListItemTitle>{{ project.name }}</VListItemTitle>
                        <VListItemSubtitle>
                            {{ $t("date.opened", { date: useLocaleTimeAgo(project.dateLastOpened).value }) }}
                        </VListItemSubtitle>
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
const route = useRoute();

const projectStore = useProjectStore();

const { selectedProject, allProjects } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const search = ref<string>();

onBeforeMount(() => {
    projectStore.loadAllProjects();
});

const switchProject = async (project: ProjectContract) => {
    const isProjectRoute = route.matched.some((match) => match.name === "index-project-id");

    if (!isProjectRoute) return;

    const match = lodFindLast(route.matched, (match) => {
        return match.meta.projectSwitchName != undefined;
    });

    const name = match ? match.meta.projectSwitchName : "index-project-id-home";

    await navigateTo({ name: name as "/", params: { id: project.id } });
};

const filteredProjects = computed(() => {
    const lowerCaseSearch = search.value?.toLocaleLowerCase() ?? "";

    return allProjects.value.filter((x) => {
        const isMatchingSearch = x.name.toLowerCase().includes(lowerCaseSearch);
        const isCurrentProject = selectedProject.value.id === x.id;

        return isMatchingSearch && !isCurrentProject;
    });
});

watch(isDialogOpen, () => {
    search.value = undefined;
});
</script>
