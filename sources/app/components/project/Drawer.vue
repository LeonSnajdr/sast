<template>
    <VNavigationDrawer location="left" permanent>
        <VList class="mt-n2">
            <VListItem height="50">
                <template #prepend>
                    <VIcon color="info" icon="mdi-folder-open" />
                </template>
                <VListItemTitle>{{ selectedProject.name }}</VListItemTitle>
                <template #append>
                    <IconBtn icon="mdi-swap-horizontal">
                        <ProjectSelectDialog />
                    </IconBtn>
                </template>
            </VListItem>
            <VDivider />
            <VListItem v-for="page in subPages" :key="page.name" :prependIcon="page.icon" :title="page.name" :to="page.to" link />
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const subPages = computed((): { icon: string; name: string; to: RouteLocationRaw }[] => {
    return [
        {
            icon: "mdi-home",
            name: "Home",
            to: { name: "project-id-home", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-label-multiple-outline",
            name: "Platzhalter",
            to: { name: "project-id-placeholder", params: { id: selectedProject.value.id } }
        },
        {
            icon: "mdi-label-multiple-outline",
            name: "Tasks",
            to: { name: "project-id-tasks", params: { id: selectedProject.value.id } }
        }
    ];
});
</script>
