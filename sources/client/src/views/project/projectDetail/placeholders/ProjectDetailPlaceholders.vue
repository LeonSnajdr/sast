<template>
    <ProjectDetailPlaceholdersEdit v-if="inEditMode" v-model:inEditMode="inEditMode" />
    <projectDetailPlaceholdersView v-else v-model:inEditMode="inEditMode" />
</template>

<script setup lang="ts">
import ProjectDetailPlaceholdersEdit from "./ProjectDetailPlaceholdersEdit.vue";
import projectDetailPlaceholdersView from "./ProjectDetailPlaceholdersView.vue";

const inEditMode = ref(false);
</script>
