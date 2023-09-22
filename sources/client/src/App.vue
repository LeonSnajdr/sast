<template>
    <Toast></Toast>
    <div class="card flex justify-content-center" v-if="projects">
        <Button @click="loadProjects">Refresh</Button>
        <br />
        <hr />
        <InputText type="text" v-model="newProjectName" />
        <Button label="Create" @click="create" />
        <br />
        <hr />
        <div v-for="project in projects" :key="project.id">
            <InputText v-model="newPlaceholderName"></InputText>
            <Button label="Create Placehlder" @click="createPlaceholderForProject(project)"></Button>
            <Button label="Rename" @click="renameProject(project)" />
            <Button label="Del" @click="deleteProject(project)" />
            <InputText></InputText>
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
const newPlaceholderName = ref<string>("");
const projects = ref<commands.Project[]>();

onBeforeMount(async () => {
    await loadProjects();
});

async function loadProjects() {
    projects.value = await commands.getProjects();

    console.log(projects.value);
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

async function deleteProject(project: commands.Project): Promise<void> {
    await commands.deleteProject(project.id);
    await loadProjects();

    toast.add({ severity: "success", summary: "Success", detail: "Deleted project successfully", life: 3000 });
}

async function renameProject(project: commands.Project): Promise<void> {
    const updateContract: commands.UpdateProjectContract = {
        id: project.id,
        name: project.name
    };

    await commands.updateProject(updateContract);

    toast.add({ severity: "success", summary: "Success", detail: "Updated project successfully", life: 3000 });
}

async function createPlaceholderForProject(project: commands.Project) {
    const create: commands.CreatePlaceholderContract = {
        name: newPlaceholderName.value,
        project_id: project.id,
        variety: "Select"
    };

    await commands.createPlaceholder(create);
}
</script>
