<template>
    <Card>
        <template #title>
            <h1>Sast</h1>
        </template>
        <template #content>
            <div v-for="listProject in listProjects" :key="listProject.id">
                <RouterLink :to="{ name: 'project', params: { projectId: listProject.id } }" class="bg-primary text-surface rounded-sm">{{
                    listProject.name
                }}</RouterLink>
            </div>
        </template>
        <template #footer>
            <div class="flex">
                <InputText v-model="addProjectName" class="w-11" />
                <Btn @click="addProject" label="crate" />
            </div>
        </template>
    </Card>
</template>

<script setup lang="ts">
import type { CreateProjectContract, ListProjectContract } from "@/bindings";
import { createProject, getListProjects } from "@/bindings";
import { useToast } from "primevue/usetoast";
import { onBeforeMount, ref } from "vue";

const toast = useToast();

const listProjects = ref<ListProjectContract[]>([]);
const addProjectName = ref("");

onBeforeMount(() => {
    loadListProjects();
});

const loadListProjects = async () => {
    try {
        listProjects.value = await getListProjects();
    } catch (error) {
        console.error("Loading list projects failed", error);
        toast.add({ severity: "error", summary: "Error", detail: "Loading list projects failed", life: 3000 });
    }
};

const addProject = async () => {
    const createContract: CreateProjectContract = {
        name: addProjectName.value
    };

    try {
        await createProject(createContract);
        await loadListProjects();
        toast.add({ severity: "success", summary: "Error", detail: "Successfully created project", life: 3000 });
    } catch (error) {
        console.error("Project creation failed", error);
        toast.add({ severity: "error", summary: "Error", detail: "Creating project failed", life: 3000 });
    }
};
</script>
