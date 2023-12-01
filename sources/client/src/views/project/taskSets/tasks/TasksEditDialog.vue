<template>
    <v-dialog v-model="dialog" width="900">
        <v-card v-if="taskSet">
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
import * as commands from "@/bindings";

const taskSetStore = useTaskSetStore();

const { popupEditTaskSetId } = storeToRefs(taskSetStore);

const taskSet = ref<FullTaskSetContract>();
const dialog = ref(false);

watch(popupEditTaskSetId, async () => {
    if (!popupEditTaskSetId.value) {
        return;
    }

    taskSet.value = await commands.getFullTaskSet(popupEditTaskSetId.value);
    dialog.value = true;
});

watch(dialog, () => {
    if (dialog.value) {
        return;
    }

    popupEditTaskSetId.value = "";
});
</script>
