<template>
    <ProjectList />
    <v-container fluid>
        <slot></slot>
    </v-container>
</template>

<script setup lang="ts">
import ProjectList from "@/views/project/ProjectList.vue";
</script>
