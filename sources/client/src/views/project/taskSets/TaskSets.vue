<template>
    <v-card>
        <v-card-title>
            {{ $t("taskSets.title") }}
            <v-spacer />
            <v-btn-icon @click="inTaskSetEdit = !inTaskSetEdit" :icon="inTaskSetEdit ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inTaskSetEdit">
                <TaskSetEdit v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
                <TaskSetCreate />
            </template>
            <template v-else>
                <v-list v-if="project.task_sets.length > 0">
                    <TaskSetView v-for="(taskSet, index) in project.task_sets" :key="taskSet.id" v-model:taskSet="project.task_sets[index]" />
                </v-list>
                <span v-else>{{ $t("taskSets.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import TaskSetEdit from "@/views/project/taskSets/TaskSetEdit.vue";
import TaskSetCreate from "@/views/project/taskSets/TaskSetCreate.vue";
import TaskSetView from "@/views/project/taskSets/TaskSetView.vue";

const projectStore = useProjectStore();

const { project, inTaskSetEdit } = storeToRefs(projectStore);
</script>
