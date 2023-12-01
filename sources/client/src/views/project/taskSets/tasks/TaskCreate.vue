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
import type { CreateTaskContract } from "@/bindings";
import * as commands from "@/bindings";
import OrderService from "@/services/OrderService";

const notify = useNotificationStore();
const taskSetStore = useTaskSetStore();

const { editTaskSet } = storeToRefs(taskSetStore);

const taskCommand = ref("");

const createTask = async () => {
    //TODO add default working directory in settings

    const followingOrderNumber = OrderService.getFollowingOrderNumber(editTaskSet.value.tasks);

    const createTask: CreateTaskContract = {
        order: followingOrderNumber,
        task_set_id: editTaskSet.value.id,
        command: taskCommand.value,
        delay: 0,
        working_directory: "c:/"
    };

    try {
        await commands.createTask(createTask);
        await taskSetStore.loadEditTaskSet();
        notify.success("taskSetTaskCreate.create.success");
    } catch (error) {
        console.error("Could not create task", error);
        notify.error("taskSetTaskCreate.create.error");
    } finally {
        taskCommand.value = "";
    }
};
</script>
