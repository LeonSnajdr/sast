<template>
    <Toast></Toast>
    <div class="card flex justify-content-center" v-if="projects">
        <Button label="Check" icon="pi pi-check" />
        <InputText type="text" v-model="newProjectName" />
        <Button label="Create" @click="create" />
        <div v-for="project in projects" :key="project.id">
            <h3>- {{ project.name }}</h3>
        </div>
    </div>
</template>

<script setup lang="ts">
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Toast from "primevue/toast";
import { useToast } from "primevue/usetoast";
import * as commands from "./bindings";
import { onBeforeMount, ref } from "vue";

const toast = useToast();

const newProjectName = ref<string>("");
const projects = ref<commands.Project[]>();

onBeforeMount(async () => {
    await loadProjects();
});

async function loadProjects() {
    projects.value = await commands.getProjects();
}

async function create(): Promise<void> {
    if (newProjectName.value == null) {
        return;
    }
    console.log("create", newProjectName.value);

    await commands.createProject({ name: newProjectName.value });
    await loadProjects();

    toast.add({ severity: "success", summary: "Success", detail: "Created project successfully", life: 3000 });
}
</script>
