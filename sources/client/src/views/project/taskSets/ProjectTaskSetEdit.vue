<template>
    <v-row-single>
        <v-text-field v-model="taskSet.description" :loading="loading" @update:modelValue="taskSetChanged" :label="taskSet.name">
            <template #append>
                <v-btn-icon icon="mdi-tune">
                    <ProjectTaskSetTasksEditDialog v-model:taskSet="taskSet" />
                </v-btn-icon>

                <v-btn-icon @click="deleteTaskSet" icon="mdi-delete" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import ProjectTaskSetTasksEditDialog from "@/views/project/taskSets/tasks/ProjectTaskSetTasksEditDialog.vue";
import type { FullTaskSetContract, UpdateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { remove } from "lodash";

const { taskSet } = defineModels<{
    taskSet: FullTaskSetContract;
}>();

const notify = useNotificationStore();
const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
const loading = ref(false);

const taskSetChanged = async () => {
    loading.value = true;

    const updateContract: UpdateTaskSetContract = {
        id: taskSet.value.id,
        description: taskSet.value.description
    };

    try {
        await commands.updateTaskSet(updateContract);
    } catch (error) {
        console.error("The taskset could not be updated", error);
        notify.error("projectTaskSetEdit.update.error");
    } finally {
        loading.value = false;
    }
};

const deleteTaskSet = async () => {
    loading.value = true;

    try {
        await commands.deleteTaskSet(taskSet.value.id);
        notify.success("projectTaskSetEdit.delete.success");
        remove(project.value.task_sets, taskSet.value);
    } catch (error) {
        console.error("The taskset could not be deleted", error);
        notify.error("projectTaskSetEdit.delete.error");
    } finally {
        loading.value = false;
    }
};
</script>
