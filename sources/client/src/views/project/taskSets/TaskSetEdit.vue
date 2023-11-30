<template>
    <v-row-single>
        <v-text-field v-model="internalTaskSet.description" @update:modelValue="taskSetChanged" :label="taskSet.name">
            <template #prepend>
                <v-icon icon="mdi-drag"></v-icon>
            </template>

            <template #append>
                <v-btn-icon icon="mdi-tune">
                    <!-- <TasksEditDialog /> -->
                </v-btn-icon>

                <v-btn-icon @click="deleteTaskSet" icon="mdi-delete" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import type { FullTaskSetContract, UpdateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";

const props = defineProps<{
    taskSet: FullTaskSetContract;
}>();

const notify = useNotificationStore();
const taskSetStore = useTaskSetStore();

const internalTaskSet = ref<FullTaskSetContract>();

onBeforeMount(() => {
    internalTaskSet.value = Object.create(props.taskSet);
});

const taskSetChanged = async () => {
    const updateContract: UpdateTaskSetContract = {
        order: internalTaskSet.value.order,
        id: internalTaskSet.value.id,
        description: internalTaskSet.value.description
    };

    try {
        await commands.updateTaskSet(updateContract);
        await taskSetStore.loadTaskSetList();
    } catch (error) {
        console.error("The taskset could not be updated", error);
        notify.error("taskSetEdit.update.error");
    }
};

const deleteTaskSet = async () => {
    try {
        await commands.deleteTaskSet(internalTaskSet.value.id);
        await taskSetStore.loadTaskSetList();

        notify.success("taskSetEdit.delete.success");
    } catch (error) {
        console.error("The taskset could not be deleted", error);
        notify.error("taskSetEdit.delete.error");
    }
};
</script>
