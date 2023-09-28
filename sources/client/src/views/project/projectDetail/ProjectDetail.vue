<template>
    <div v-if="projectStore.project">
        <h1>{{ projectStore.project.name }}</h1>
        <Placeholders :placeholders="projectStore.project.placeholders" />
        <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
    </div>
</template>

<script setup lang="ts">
import { useProjectStore } from "@/stores/projectStore";
import Placeholders from "./Placeholders.vue";

const projectStore = useProjectStore();
</script>
