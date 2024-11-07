<template>
    <VNavigationDrawer location="left" permanent>
        <VList>
            <VListItem v-for="page in subPages" :key="page.name" :prependIcon="page.icon" :title="page.name" :to="page.to" link />
        </VList>
    </VNavigationDrawer>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const subPages: { icon: string; name: string; to: RouteLocationRaw }[] = [
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
</script>
