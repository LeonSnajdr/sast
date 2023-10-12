<template>
    <ProjectTaskSetEdit v-if="inEditMode" />
    <ProjectTaskSetsView v-else />
</template>

<script setup lang="ts">
import ProjectTaskSetsView from "./ProjectTaskSetsView.vue";
import ProjectTaskSetEdit from "./ProjectTaskSetEdit.vue";

const inEditMode = ref(false);
</script>
