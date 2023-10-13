<template>
    <v-row-single v-if="!inOptions">
        <v-text-field
            v-model="task.command"
            @click:append="inOptions = !inOptions"
            :loading="loading"
            prependInnerIcon="mdi-apple-keyboard-command"
            appendIcon="mdi-cog"
        />
    </v-row-single>
    <v-row v-else>
        <v-col cols="8">
            <v-text-field v-model="task.working_directory" :loading="loading" prependInnerIcon="mdi-folder" />
        </v-col>
        <v-col cols="4">
            <v-text-field v-model="task.delay" :loading="loading" type="number" prependInnerIcon="mdi-clock">
                <template #append>
                    <v-icon @click="deleteTask" icon="mdi-delete" color="error" />
                    <v-icon @click="inOptions = !inOptions" icon="mdi-cog" />
                </template>
            </v-text-field>
        </v-col>
    </v-row>
</template>

<script setup lang="ts">
import type { Task } from "@/bindings";
import * as commands from "@/bindings";
import { remove } from "lodash";

const loading = ref(false);
const inOptions = ref(false);

const { task, taskSet } = defineModels<{
    taskSet: commands.FullTaskSetContract;
    task: Task;
}>();

const deleteTask = async () => {
    loading.value = true;

    try {
        await commands.deleteTask(task.value.id);
        remove(taskSet.value.tasks, task.value);
    } catch (error) {
        console.error("Could not delete task", error);
    } finally {
        loading.value = false;
    }
};
</script>
