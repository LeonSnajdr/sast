<template>
    <v-row-single v-if="!inOptions">
        <v-text-field v-model="internalTask.command" prependIcon="mdi-drag" prependInnerIcon="mdi-apple-keyboard-command">
            <template #append>
                <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
            </template>
        </v-text-field>
    </v-row-single>
    <v-row v-else>
        <v-col cols="8">
            <v-text-field v-model="internalTask.working_directory" prependIcon="mdi-drag" prependInnerIcon="mdi-folder" />
        </v-col>
        <v-col cols="4">
            <v-text-field v-model="internalTask.delay" type="number" prependInnerIcon="mdi-clock">
                <template #append>
                    <v-btn-icon @click="deleteTask" icon="mdi-delete" />
                    <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
                </template>
            </v-text-field>
        </v-col>
    </v-row>
</template>

<script setup lang="ts">
import type { Task, UpdateTaskContract } from "@/bindings";
import * as commands from "@/bindings";

const props = defineProps<{
    task: Task;
}>();

const notify = useNotificationStore();
const taskSetStore = useTaskSetStore();

const internalTask = ref<Task>();
const inOptions = ref(false);

onBeforeMount(() => {
    internalTask.value = Object.create(props.task);
});

watch(internalTask, () => taskChanged(), { deep: true });

const taskChanged = async () => {
    const updateContract: UpdateTaskContract = {
        id: internalTask.value.id,
        order: internalTask.value.order,
        command: internalTask.value.command,
        working_directory: internalTask.value.working_directory,
        delay: Number(internalTask.value.delay)
    };

    try {
        await commands.updateTask(updateContract);
    } catch (error) {
        console.log("Failed updating task", error);
        notify.error("taskSetTaskEdit.update.error");
    }
};

const deleteTask = async () => {
    try {
        await commands.deleteTask(internalTask.value.id);
        await taskSetStore.loadEditTaskSet();
        notify.success("taskSetTaskEdit.delete.success");
    } catch (error) {
        console.error("Could not delete task", error);
        notify.error("taskSetTaskEdit.delete.error");
    }
};
</script>
