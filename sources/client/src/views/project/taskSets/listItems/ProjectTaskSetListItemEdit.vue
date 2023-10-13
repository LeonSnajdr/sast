<template>
    <v-list-item-single>
        <v-text-field v-model="taskSet.description" :loading="loading" @update:modelValue="taskSetChanged" label="taskSet.name">
            <template #append>
                <div>
                    <v-icon icon="mdi-tune"> </v-icon>
                    <ProjectTaskSetEditDialog v-model:taskSet="taskSet" />
                </div>

                <v-icon @click="deleteTaskSet" icon="mdi-delete" color="error"> </v-icon>
            </template>
        </v-text-field>
    </v-list-item-single>
</template>

<script setup lang="ts">
import ProjectTaskSetEditDialog from "../ProjectTaskSetEditDialog.vue";
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
        notify.error("projectTaskSetListItemEdit.update.error");
    } finally {
        loading.value = false;
    }
};

const deleteTaskSet = async () => {
    loading.value = true;

    try {
        await commands.deleteTaskSet(taskSet.value.id);
        notify.success("projectTaskSetListItemEdit.delete.success");
        remove(project.value.task_sets, taskSet.value);
    } catch (error) {
        console.error("The taskset could not be deleted", error);
        notify.error("projectTaskSetListItemEdit.delete.error");
    } finally {
        loading.value = false;
    }
};
</script>
