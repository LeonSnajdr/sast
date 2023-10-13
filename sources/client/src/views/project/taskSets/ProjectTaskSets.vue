<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSets.title") }}
            <v-spacer />
            <v-icon @click="inEditMode = !inEditMode" :icon="inEditMode ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <v-list>
                <template v-for="(taskSet, index) in project.task_sets" :key="taskSet.id">
                    <ProjectTaskSetListItemEdit v-if="inEditMode" v-model:taskSet="project.task_sets[index]" />
                    <ProjectTaskSetListItemView v-else v-model:taskSet="project.task_sets[index]" />
                </template>

                <ProjectTaskSetListItemCreate v-if="inEditMode" />
            </v-list>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectTaskSetListItemView from "@/views/project/taskSets/listItems/ProjectTaskSetListItemView.vue";
import ProjectTaskSetListItemCreate from "@/views/project/taskSets/listItems/ProjectTaskSetListItemCreate.vue";
import ProjectTaskSetListItemEdit from "@/views/project/taskSets/listItems/ProjectTaskSetListItemEdit.vue";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);

const inEditMode = ref(false);
</script>
