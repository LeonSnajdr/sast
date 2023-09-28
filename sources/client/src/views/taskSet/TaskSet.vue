<template>
    <RouterLink :to="{ name: 'project', params: { projectId: projectId } }">back</RouterLink>
    <div v-if="taskSet">
        <h1>{{ taskSet.name }}</h1>

        <template v-for="task in taskSet.tasks" :key="task.id">
            <InputText v-model="task.command" />
        </template>
        <div>
            <InputText v-model="createTaskInput" placeholder="Command"></InputText>
            <Btn @click="createCommand">Create command</Btn>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
import type { FullSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { useToast } from "primevue/usetoast";

const props = defineProps<{
    projectId: string;
    taskSetId: string;
}>();

const toast = useToast();

const taskSet = ref<FullSetContract>();

const createTaskInput = ref("");

onBeforeMount(() => {
    loadTaskSet();
});

const loadTaskSet = async () => {
    try {
        const fullTaskSet = await commands.getFullTaskSet(props.taskSetId);
        taskSet.value = fullTaskSet ?? undefined;
    } catch (error) {
        console.error("Loading taskset failed", error);
        toast.add({ severity: "error", summary: "Error", detail: "Loading taskset failed", life: 3000 });
    }
};

const createCommand = async () => {
    //
};
</script>
