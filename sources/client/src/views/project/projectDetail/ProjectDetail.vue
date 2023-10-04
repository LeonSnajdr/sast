<template>
    <h1 class="bg-red-700">{{ project.name }}</h1>
    <ProjectDetailPlaceholders v-model:project="project" />
    <ProjectDetailTaskSets v-model:project="project" />
    <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
</template>

<script setup lang="ts">
import ProjectDetailPlaceholders from "./ProjectDetailPlaceholders.vue";
import ProjectDetailTaskSets from "./ProjectDetailTaskSets.vue";
import type { FullProjectContract } from "@/bindings";

const { project } = defineModels<{
    project: FullProjectContract;
}>();
</script>
