<template>
    <ProjectPlaceholdersEdit v-if="inEditMode" v-model:inEditMode="inEditMode" />
    <ProjectPlaceholdersView v-else v-model:inEditMode="inEditMode" />
</template>

<script setup lang="ts">
import ProjectPlaceholdersEdit from "./ProjectPlaceholdersEdit.vue";
import ProjectPlaceholdersView from "./ProjectPlaceholdersView.vue";

const inEditMode = ref(false);
</script>
