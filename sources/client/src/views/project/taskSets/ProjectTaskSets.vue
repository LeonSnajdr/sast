<template>
    <v-card>
        <v-card-title>
            {{ $t("projectTaskSets.title") }}
            <v-spacer />
            <v-btn-icon @click="inTaskSetEdit = !inTaskSetEdit" :icon="inTaskSetEdit ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inTaskSetEdit">
                <ProjectTaskSetEdit v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
                <ProjectTaskSetCreate />
            </template>
            <template v-else>
                <v-list v-if="project.task_sets.length > 0">
                    <ProjectTaskSetView v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
                </v-list>
                <span v-else>{{ $t("projectTaskSets.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectTaskSetEdit from "@/views/project/taskSets/components/ProjectTaskSetEdit.vue";
import ProjectTaskSetCreate from "@/views/project/taskSets/components/ProjectTaskSetCreate.vue";
import ProjectTaskSetView from "@/views/project/taskSets/components/ProjectTaskSetView.vue";

const projectPageStore = useProjectPageStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const { inTaskSetEdit } = storeToRefs(projectPageStore);
</script>
