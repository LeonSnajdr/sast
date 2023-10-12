<template>
    <ProjectDetailTaskSetEdit v-if="inEditMode" />
    <ProjectDetailTaskSetsView v-else />
</template>

<script setup lang="ts">
import ProjectDetailTaskSetsView from "./ProjectDetailTaskSetsView.vue";
import ProjectDetailTaskSetEdit from "./ProjectDetailTaskSetEdit.vue";

const inEditMode = ref(false);
</script>
