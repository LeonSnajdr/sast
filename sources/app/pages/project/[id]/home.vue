<template>
    <div>{{ selectedProject.id }}</div>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);
</script>
