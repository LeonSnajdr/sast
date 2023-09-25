<template>
    <h2>Projectlist</h2>
    <div v-for="listProject in projectStore.listProjects" :key="listProject.id">
        <p>{{ listProject.name }}</p>
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
