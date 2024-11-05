<template>
    <div v-if="projectStore.projects.length > 0">
        <ProjectDrawer />
        <VContainer>
            <NuxtPage />
        </VContainer>
    </div>
    <div v-else>
        <VEmptyState headline="Keine Projekte vorhanden" icon="mdi-information-outline" title="Beginne damit eines anzulegen">
            <VBtn>Neues anlegen</VBtn>
        </VEmptyState>
    </div>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();

onBeforeMount(async () => {
    await projectStore.loadProjects();
});
</script>
