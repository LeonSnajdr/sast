<template>
    <ProjectTaskSetsEdit v-if="inEditMode" v-model:inEditMode="inEditMode" />
    <ProjectTaskSetsView v-else v-model:inEditMode="inEditMode" />
</template>

<script setup lang="ts">
import ProjectTaskSetsView from "./ProjectTaskSetsView.vue";
import ProjectTaskSetsEdit from "./ProjectTaskSetsEdit.vue";

const inEditMode = ref(false);
</script>
