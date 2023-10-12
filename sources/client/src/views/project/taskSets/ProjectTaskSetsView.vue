<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSetsView.title") }}
            <v-spacer />
            <v-icon @click="inEditMode = true" icon="mdi-pencil" />
        </v-card-title>
        <v-card-text>
            <v-list>
                <template v-for="taskSet in project.task_sets" :key="taskSet.id"> <ProjectTaskSetListItem :taskSet="taskSet" /></template>
            </v-list>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectTaskSetListItem from "./ProjectTaskSetListItem.vue";

const { inEditMode } = defineModels<{
    inEditMode: boolean;
}>();

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
