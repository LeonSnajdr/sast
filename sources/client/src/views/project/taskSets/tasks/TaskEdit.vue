<template>
    <div>
        <!-- Div is needed to preserve the draggability, otherwise the reference to the object is lost-->
        <v-row-single v-if="!inOptions">
            <v-text-field v-model="internalTask.command" :placeholder="$t('taskEdit.input.command')" @update:modelValue="taskChanged" prependIcon="mdi-drag">
                <template #append>
                    <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
                </template>
            </v-text-field>
        </v-row-single>
        <v-row v-else>
            <v-col cols="8">
                <v-text-field v-model="internalTask.working_directory" @update:modelValue="taskChanged" prependIcon="mdi-drag" prependInnerIcon="mdi-folder" />
            </v-col>
            <v-col cols="4">
                <v-text-field v-model="internalTask.delay" @update:modelValue="taskChanged" type="number" prependInnerIcon="mdi-clock">
                    <template #append>
                        <v-btn-icon icon="mdi-delete">
                            <ConfirmationDialog :message="$t('taskEdit.delete.confirmation')" @confirmAction="deleteTask" />
                        </v-btn-icon>
                        <v-btn-icon @click="inOptions = !inOptions" icon="mdi-cog" />
                    </template>
                </v-text-field>
            </v-col>
        </v-row>
    </div>
</template>

<script setup lang="ts">
import { debounce } from "lodash";
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

const taskChanged = debounce(async () => {
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
        notify.error("taskEdit.update.error");
    }
}, 500);

const deleteTask = async () => {
    try {
        await commands.deleteTask(internalTask.value.id);
        await taskSetStore.loadEditTaskSet();
        notify.success("taskEdit.delete.success");
    } catch (error) {
        console.error("Could not delete task", error);
        notify.error("taskEdit.delete.error");
    }
};
</script>
