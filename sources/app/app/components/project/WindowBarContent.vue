<template>
    <Teleport to="#window-bar-content" defer>
        <VBtn v-for="project in allProjects" :key="project.id" :to="getSwitchProjectLocationRef(project).value" density="compact">
            {{ project.name }}
        </VBtn>
    </Teleport>
</template>
<script setup lang="ts">
const { getSwitchProjectLocationRef } = useProject();

const projectStore = useProjectStore();

const { allProjects } = storeToRefs(projectStore);
</script>
