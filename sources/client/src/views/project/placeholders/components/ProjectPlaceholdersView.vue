<template>
    <v-chip-group>
        <v-chip v-for="placeholder in project.placeholders" :key="placeholder.id">
            <strong>{{ placeholder.name }}</strong>
            <span>: {{ placeholder.value }}</span>
        </v-chip>
    </v-chip-group>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();
const { project } = storeToRefs(projectStore);
</script>
