<template>
    <VNavigationDrawer location="left" permanent>
        <VContainer>
            <VCombobox v-model="selectedProjectId" :items="projects" :returnObject="false" density="compact" itemTitle="name" itemValue="id" />
        </VContainer>
        <VDivider />
        <VList>
            <VListItem
                v-for="page in subPages"
                :key="page.name"
                :prependIcon="page.icon"
                :title="page.name"
                :to="{ name: 'project-id', params: { id: selectedProjectId } }"
                link
            />
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
const projectStore = useProjectStore();

const { projects, selectedProjectId } = storeToRefs(projectStore);

const subPages: { icon: string; name: string }[] = [
    {
        icon: "mdi-label-multiple-outline",
        name: "Platzhalter"
    }
];
</script>
