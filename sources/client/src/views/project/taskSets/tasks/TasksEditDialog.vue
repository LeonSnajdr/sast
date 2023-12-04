<template>
    <v-dialog v-model="dialog" width="900">
        <v-card v-if="editTaskSet">
            <v-card-title>{{ editTaskSet.name }}</v-card-title>
            <v-card-text>
                <draggable v-model="editTaskSet.tasks" @end="taskSetOrderChanged" itemKey="id">
                    <template #item="{ element: task }">
                        <TaskEdit :task="task" />
                    </template>
                </draggable>

                <TaskCreate v-model:taskSet="editTaskSet" />
            </v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn @click="dialog = false">{{ $t("close") }}</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import TaskEdit from "@/views/project/taskSets/tasks/TaskEdit.vue";
import TaskCreate from "@/views/project/taskSets/tasks/TaskCreate.vue";
import draggable from "vuedraggable";
import OrderService from "@/services/OrderService";
import type { UpdateTaskContract } from "@/bindings";
import * as commands from "@/bindings";

const taskSetStore = useTaskSetStore();

const { editTaskSet } = storeToRefs(taskSetStore);

const taskSetOrderChanged = async () => {
    const taskUpdateContracts = OrderService.getItemListWithUpdatedOrders<UpdateTaskContract>(editTaskSet.value.tasks);

    await commands.updateTasks(taskUpdateContracts);
    await taskSetStore.loadTaskSetList();
};

const dialog = computed({
    get() {
        return editTaskSet.value != undefined;
    },
    set(newValue) {
        if (newValue) return;

        editTaskSet.value = undefined;
    }
});
</script>
