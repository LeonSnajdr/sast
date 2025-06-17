<template>
    <VMenu :openDelay="0" activator="parent" openOnHover>
        <VList maxHeight="400" maxWidth="400">
            <VListItem v-for="project in allProjects" :key="project.id" @click="switchProject(project)">
                <VListItemTitle>
                    <VIcon v-if="project.favorite" color="warning" icon="mdi-star" size="small" />
                    {{ project.name }}
                </VListItemTitle>
                <VListItemSubtitle>
                    {{ $t("date.opened", { date: useLocaleTimeAgo(project.dateLastOpened).value }) }}
                </VListItemSubtitle>
                <template #append>
                    <VChip v-if="project.quickSwitchKeybind" class="ml-2" color="secondary" density="compact" variant="outlined" label>
                        {{ project.quickSwitchKeybind }}
                    </VChip>
                </template>
            </VListItem>
            <VListSubheader v-if="allProjects.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
const { switchProject } = useProject();
const { current: pressedKeys } = useMagicKeys();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);

onBeforeMount(() => {
    projectStore.loadAllProjects();
});

const isMatchingProject = (project: ProjectContract) => {
    if (!project.quickSwitchKeybind) return false;

    const keys = project.quickSwitchKeybind
        .toLowerCase()
        .split("+")
        .map((k) => k.trim());

    return keys.every((key) => pressedKeys.has(key));
};

whenever(
    () => allProjects.value.some((project) => isMatchingProject(project)),
    () => {
        const project = allProjects.value.find((project) => isMatchingProject(project));

        if (!project) return;

        switchProject(project);
    }
);
</script>
