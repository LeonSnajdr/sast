<template>
    <InputText v-model="task.command" @update:modelValue="taskUpdated" />
    <InputText v-model="task.working_directory" @update:modelValue="taskUpdated" />
    <InputNumber v-model="task.delay" @update:modelValue="taskUpdated"></InputNumber>
    <Btn @click="deleteTask" label="delete" />
</template>

<script setup lang="ts">
import type { Task, UpdateTaskContract } from "@/bindings";
import * as command from "@/bindings";

const emit = defineEmits<{
    deleted: [void];
}>();

const { task } = defineModels<{
    task: Task;
}>();

const taskUpdated = async () => {
    const updateContract: UpdateTaskContract = {
        id: task.value.id,
        command: task.value.command,
        working_directory: task.value.working_directory,
        delay: task.value.delay
    };

    await command.updateTask(updateContract);
};

const deleteTask = async () => {
    await command.deleteTask(task.value.id);
    emit("deleted");
};
</script>
