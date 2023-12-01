<template>
    <v-dialog v-model="dialog" width="900">
        <v-card v-if="editTaskSet">
            <v-card-title>{{ editTaskSet.name }}</v-card-title>
            <v-card-text>
                <TaskEdit v-for="(task, index) in editTaskSet.tasks" :key="task.id" v-model:taskSet="editTaskSet" v-model:task="editTaskSet.tasks[index]" />
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

const taskSetStore = useTaskSetStore();

const { editTaskSet } = storeToRefs(taskSetStore);

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
