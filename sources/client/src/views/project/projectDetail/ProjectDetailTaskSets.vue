<template>
    <h2>Task sets</h2>

    <div v-for="taskSet in project.task_sets" :key="taskSet.id">
        <ProjectDetailTaskSet :taskSet="taskSet" />
    </div>

    <div>
        <InputText v-model="craeteTaskSetName" size="small"></InputText>
        <Btn @click="createTaskSet" label="Crate"></Btn>
    </div>
</template>

<script setup lang="ts">
import type { FullProjectContract, CreateTaskSetContract } from "@/bindings";
import * as commands from "@/bindings";
import { useToast } from "primevue/usetoast";
import { ref } from "vue";
import ProjectDetailTaskSet from "./ProjectDetailTaskSet.vue";

const { project } = defineModels<{
    project: FullProjectContract;
}>();

const toast = useToast();

const craeteTaskSetName = ref("");

const createTaskSet = async () => {
    const createContract: CreateTaskSetContract = {
        project_id: project.value.id,
        name: craeteTaskSetName.value
    };

    try {
        const newContract = await commands.createTaskSet(createContract);

        project.value.task_sets.push(newContract);

        toast.add({ severity: "success", detail: "TaskSet created", group: "br", life: 3000 });
    } catch (error) {
        console.error("Creating placeholder failed", error);
        toast.add({ severity: "error", detail: "Placeholder creation failed", group: "br", life: 3000 });
    } finally {
        craeteTaskSetName.value = "";
    }
};
</script>
