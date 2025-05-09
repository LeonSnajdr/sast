<template>
    <VMenu activator="parent" openOnHover>
        <VList>
            <VListItem v-for="(project, index) in filteredProjects" :key="project.id" @click="switchProject(project)">
                <VListItemTitle>{{ project.name }}</VListItemTitle>
                <VListItemSubtitle>
                    {{ $t("date.opened", { date: useLocaleTimeAgo(project.dateLastOpened).value }) }}
                </VListItemSubtitle>
                <template #append>
                    <VChip class="ml-2" color="secondary" density="compact" variant="outlined" label>Ctrl+{{ index + 1 }}</VChip>
                </template>
            </VListItem>
            <VListSubheader v-if="filteredProjects.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
const route = useRoute();

const { current: pressedKeys } = useMagicKeys();

const projectStore = useProjectStore();

const { selectedProject, allProjects } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const search = ref<string>();

onBeforeMount(() => {
    projectStore.loadAllProjects();
});

whenever(
    () => pressedKeys.has("control") && lodRange(1, 10).some((num) => pressedKeys.has(num.toString())),
    () => {
        const pressedNumber = lodRange(1, 10).find((num) => pressedKeys.has(num.toString()));

        if (!pressedNumber) return;

        const project = allProjects.value[pressedNumber - 1];

        if (!project) return;

        switchProject(project);
    }
);

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
