<template>
    <div v-for="listProject in projectStore.listProjects" :key="listProject.id">
        <RouterLink :to="{ name: 'project', params: { projectId: listProject.id } }">{{ listProject.name }}</RouterLink>
    </div>
</template>

<script setup lang="ts">
import { useProjectStore } from "@/stores/projectStore";
import { onBeforeMount } from "vue";

const projectStore = useProjectStore();

onBeforeMount(() => {
    projectStore.loadListProjects();
});
</script>
