<template>
    <v-row-single>
        <v-text-field v-model="taskCommand" prependInnerIcon="mdi-apple-keyboard-command">
            <template #append>
                <v-btn-icon @click="createTask" icon="mdi-plus" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import type { CreateTaskContract, FullTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();

const notify = useNotificationStore();
const taskCommand = ref("");

const createTask = async () => {
    const createTask: CreateTaskContract = {
        task_set_id: taskSet.value.id,
        command: taskCommand.value,
        delay: 0,
        working_directory: "c:/"
    };

    try {
        const createdTask = await commands.createTask(createTask);
        taskSet.value.tasks.push(createdTask);
        notify.success("projectTaskSetTaskCreate.create.success");
    } catch (error) {
        console.error("Could not create task", error);
        notify.error("projectTaskSetTaskCreate.create.error");
    }
};
</script>
