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
                    <VHotkey v-if="project.quickSwitchKeybind" :keys="project.quickSwitchKeybind" class="ml-2" />
                </template>
            </VListItem>
            <VListSubheader v-if="allProjects.length == 0" class="text-center">{{ $t("search.noResults") }}</VListSubheader>
        </VList>
    </VMenu>
</template>

<script setup lang="ts">
const { switchProject } = useProject();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);
</script>
