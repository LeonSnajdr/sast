<template>
    <VNavigationDrawer location="left" permanent>
        <VContainer>
            <VCombobox v-model="selectedProject" :items="projects" density="compact" clearable> hallo </VCombobox>
        </VContainer>
        <VDivider />
        <VListItem title="List Item 1" link />
        <VListItem title="List Item 2" link />
        <VListItem title="List Item 3" link />
    </VNavigationDrawer>
</template>

<script setup lang="ts">
const selectedProject = ref<string>("");
const projects = ref<string[]>(["Metis", "Asimov"]);
</script>
