<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSets.title") }}
            <v-spacer />
            <v-btn-icon @click="inEditMode = !inEditMode" :icon="inEditMode ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inEditMode">
                <ProjectTaskSetEdit v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
                <ProjectTaskSetCreate />
            </template>
            <v-list v-else>
                <ProjectTaskSetView v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
            </v-list>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectTaskSetEdit from "@/views/project/taskSets/components/ProjectTaskSetEdit.vue";
import ProjectTaskSetCreate from "@/views/project/taskSets/components/ProjectTaskSetCreate.vue";
import ProjectTaskSetView from "@/views/project/taskSets/components/ProjectTaskSetView.vue";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);

const inEditMode = ref(false);
</script>
