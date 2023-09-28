<template>
    <h1>{{ project.name }}</h1>
    <Placeholders v-model:project="project" />
    <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
</template>

<script setup lang="ts">
import Placeholders from "./Placeholders.vue";
import type { FullProjectContract } from "@/bindings";

const { project } = defineModels<{
    project: FullProjectContract;
}>();
</script>
