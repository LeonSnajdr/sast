<template>
    <BaseDrawer>
        <template #headerLeft>
            <v-btn-icon :to="{ name: 'settingsGeneral' }" icon="mdi-cog" />
        </template>

        <template #headerRight>
            <v-btn-icon icon="mdi-plus">
                <ProjectCreateDialog />
            </v-btn-icon>
        </template>

        <template #default>
            <v-list v-if="listProjects.length > 0">
                <v-list-item v-for="listProject in listProjects" :key="listProject.id" :to="{ name: 'project', params: { projectId: listProject.id } }" link>
                    <v-list-item-title>{{ listProject.name }}</v-list-item-title>
                </v-list-item>
            </v-list>
            <span v-else>{{ $t("projectList.noItems") }}</span>
        </template>
    </BaseDrawer>
</template>

<script setup lang="ts">
import ProjectCreateDialog from "./ProjectCreateDialog.vue";

const projectStore = useProjectStore();

const { listProjects } = storeToRefs(projectStore);

onBeforeMount(() => {
    projectStore.loadListProjects();
});
</script>
