<template>
    <Btn label="run" :loading="loading" icon="pi pi-search" @click="startTaskSet()" />
    <RouterLink :to="{ name: 'taskSet', params: { projectId: taskSet.project_id, taskSetId: taskSet.id } }">{{ taskSet.name }}</RouterLink>
</template>

<script setup lang="ts">
import type { TaskSet } from "@/bindings";
import * as commands from "@/bindings";
import { useToast } from "primevue/usetoast";
import { ref } from "vue";

const props = defineProps<{
    taskSet: TaskSet;
}>();

const toast = useToast();

const loading = ref(false);

const startTaskSet = async () => {
    console.log("start");

    try {
        loading.value = true;
        await commands.startTaskSet(props.taskSet.id);
    } catch (error) {
        console.error(error);
        toast.add({ severity: "error", detail: "Taskset start failed", group: "br", life: 3000 });
    } finally {
        console.log("finish");

        loading.value = false;
    }
};
</script>
