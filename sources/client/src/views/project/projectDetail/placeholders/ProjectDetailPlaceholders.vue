<template>
    <ProjectDetailPlaceholdersEdit v-if="inEditMode" v-model:inEditMode="inEditMode" />
    <ProjectDetailPlaceholdersShow v-else v-model:inEditMode="inEditMode" />
</template>

<script setup lang="ts">
import ProjectDetailPlaceholdersEdit from "./ProjectDetailPlaceholdersEdit.vue";
import ProjectDetailPlaceholdersShow from "./ProjectDetailPlaceholdersShow.vue";

const inEditMode = ref(false);
</script>
