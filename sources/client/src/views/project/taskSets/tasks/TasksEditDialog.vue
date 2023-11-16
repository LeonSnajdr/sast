<template>
    <v-dialog v-model="dialog" activator="parent" width="900">
        <v-card>
            <v-card-title>{{ taskSet.name }}</v-card-title>
            <v-card-text>
                <TaskEdit v-for="(task, index) in taskSet.tasks" :key="task.id" v-model:taskSet="taskSet" v-model:task="taskSet.tasks[index]" />
                <TaskCreate v-model:taskSet="taskSet" />
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import TaskEdit from "@/views/project/taskSets/tasks/TaskEdit.vue";
import TaskCreate from "@/views/project/taskSets/tasks/TaskCreate.vue";
import type { FullTaskSetContract } from "@/bindings";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();

const dialog = ref(false);
</script>
