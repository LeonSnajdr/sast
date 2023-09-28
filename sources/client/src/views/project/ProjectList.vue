<template>
    <div v-for="listProject in listProjects" :key="listProject.id">
        <RouterLink :to="{ name: 'project', params: { projectId: listProject.id } }">{{ listProject.name }}</RouterLink>
    </div>
</template>

<script setup lang="ts">
import type { ListProjectContract } from "@/bindings";
import { getListProjects } from "@/bindings";
import { useToast } from "primevue/usetoast";
import { onBeforeMount, ref } from "vue";

const toast = useToast();

const listProjects = ref<ListProjectContract[]>([]);

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
</script>
