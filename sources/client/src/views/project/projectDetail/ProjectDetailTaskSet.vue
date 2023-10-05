<template>
    <Btn label="run" :loading="loading" icon="pi pi-search" @click="startTaskSet()" />
    <RouterLink :to="{ name: 'taskSet', params: { projectId: taskSet.project_id, taskSetId: taskSet.id } }">{{ taskSet.name }}</RouterLink>
</template>

<script setup lang="ts">
import type { TaskSet } from "@/bindings";
import * as commands from "@/bindings";
import { useNotificationStore } from "@/stores/notificationStore";
import { ref } from "vue";

const props = defineProps<{
    taskSet: TaskSet;
}>();

const notify = useNotificationStore();

const loading = ref(false);

const startTaskSet = async () => {
    try {
        loading.value = true;
        await commands.startTaskSet(props.taskSet.id);
    } catch (error) {
        console.error(error);
        notify.error("TODO");
    } finally {
        loading.value = false;
    }
};
</script>
