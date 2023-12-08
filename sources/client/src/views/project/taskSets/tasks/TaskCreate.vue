<template>
    <v-row-single>
        <v-text-field v-model="taskCommand" :placeholder="$t('taskCreate.input.command')" prependIcon="mdi-blur-off">
            <template #append>
                <v-btn-icon @click="createTask" icon="mdi-plus" />
            </template>
        </v-text-field>
    </v-row-single>
</template>

<script setup lang="ts">
import type { CreateTaskContract } from "@/bindings";
import * as commands from "@/bindings";
import OrderService from "@/services/OrderService";

const notify = useNotificationStore();
const settingsStore = useSettingsStore();
const taskSetStore = useTaskSetStore();

const { settings } = storeToRefs(settingsStore);
const { editTaskSet } = storeToRefs(taskSetStore);

const taskCommand = ref("");

const createTask = async () => {
    const followingOrderNumber = OrderService.getFollowingOrderNumber(editTaskSet.value.tasks);

    const createTask: CreateTaskContract = {
        order: followingOrderNumber,
        task_set_id: editTaskSet.value.id,
        command: taskCommand.value,
        delay: 0,
        working_directory: settings.value.default_dir
    };

    try {
        await commands.createTask(createTask);
        await taskSetStore.loadEditTaskSet();
        notify.success("taskCreate.create.success");
    } catch (error) {
        console.error("Could not create task", error);
        notify.error("taskCreate.create.error");
    } finally {
        taskCommand.value = "";
    }
};
</script>
