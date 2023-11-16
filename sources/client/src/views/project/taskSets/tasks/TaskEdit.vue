<template>
    <v-row-single v-if="!inOptions">
        <v-text-field v-model="task.command" :loading="loading" prependInnerIcon="mdi-apple-keyboard-command">
            <template #append>
                <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
            </template>
        </v-text-field>
    </v-row-single>
    <v-row v-else>
        <v-col cols="8">
            <v-text-field v-model="task.working_directory" :loading="loading" prependInnerIcon="mdi-folder" />
        </v-col>
        <v-col cols="4">
            <v-text-field v-model="task.delay" :loading="loading" type="number" prependInnerIcon="mdi-clock">
                <template #append>
                    <v-btn-icon @click="deleteTask" icon="mdi-delete" />
                    <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
                </template>
            </v-text-field>
        </v-col>
    </v-row>
</template>

<script setup lang="ts">
import type { Task, UpdateTaskContract, FullTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { remove } from "lodash";

const loading = ref(false);
const inOptions = ref(false);

const { task, taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
    task: Task;
}>();

const notify = useNotificationStore();

watch(task.value, () => taskChanged());

const taskChanged = async () => {
    loading.value = true;

    const updateContract: UpdateTaskContract = {
        id: task.value.id,
        command: task.value.command,
        working_directory: task.value.working_directory,
        delay: Number(task.value.delay)
    };

    try {
        await commands.updateTask(updateContract);
    } catch (error) {
        console.log("Failed updating task", error);
        notify.error("taskSetTaskEdit.update.error");
    } finally {
        loading.value = false;
    }
};

const deleteTask = async () => {
    loading.value = true;

    try {
        await commands.deleteTask(task.value.id);
        remove(taskSet.value.tasks, task.value);
        notify.success("taskSetTaskEdit.delete.success");
    } catch (error) {
        console.error("Could not delete task", error);
        notify.error("taskSetTaskEdit.delete.error");
    } finally {
        loading.value = false;
    }
};
</script>
