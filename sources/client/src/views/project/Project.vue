<template>
    <ProjectList />
    <ProjectDetail v-if="project" />
</template>

<script setup lang="ts">
import { useProjectStore } from "@/stores/projectStore";
import ProjectList from "./ProjectList.vue";
import ProjectDetail from "./projectDetail/ProjectDetail.vue";
import { storeToRefs } from "pinia";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
