<template>
    <RouterLink :to="{ name: 'project', params: { projectId: projectId } }">back</RouterLink>
    <div v-if="taskSet">
        <h1>{{ taskSet.name }}</h1>

        <template v-for="(task, index) in taskSet.tasks" :key="task.id">
            <Task v-model:task="taskSet.tasks[index]" @deleted="taskSet.tasks.splice(index, 1)" />
        </template>
        <div>
            <InputText v-model="taskCommand" placeholder="Command"></InputText>
            <InputText v-model="taskWorkingDirectory" placeholder="Workig dir"></InputText>
            <InputNumber v-model="taskDelay" placeholder="Delay"></InputNumber>
            <Btn @click="createCommand">Create command</Btn>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onBeforeMount, ref } from "vue";
import type { FullSetContract, CreateTaskContract } from "@/bindings";
import * as commands from "@/bindings";
import Task from "./Task.vue";
import { useNotificationStore } from "@/stores/notificationStore";

const props = defineProps<{
    projectId: string;
    taskSetId: string;
}>();

const notify = useNotificationStore();

const taskSet = ref<FullSetContract>();

const taskCommand = ref("");
const taskWorkingDirectory = ref("");
const taskDelay = ref(0);

onBeforeMount(() => {
    loadTaskSet();
});

const loadTaskSet = async () => {
    try {
        const fullTaskSet = await commands.getFullTaskSet(props.taskSetId);
        taskSet.value = fullTaskSet ?? undefined;
    } catch (error) {
        console.error("Loading taskset failed", error);
        notify.error("TOODO");
    }
};

const createCommand = async () => {
    try {
        const createContract: CreateTaskContract = {
            command: taskCommand.value,
            working_directory: taskWorkingDirectory.value,
            delay: taskDelay.value,
            task_set_id: props.taskSetId
        };

        const createdTask = await commands.createTask(createContract);

        taskSet.value?.tasks.push(createdTask);
    } catch (error) {
        console.error("Creating task failed", error);
        notify.error("TODO");
    }
};
</script>
