<template>
    <h1>{{ project.name }}</h1>
    <!--<Card>
        <template #content>
            
            <ProjectDetailPlaceholders v-model:project="project" />
            <ProjectDetailTaskSets v-model:project="project" />
            <RouterLink :to="{ name: 'project' }">Close project</RouterLink>
        </template>
    </Card>-->
</template>

<script setup lang="ts">
import { useProjectStore } from "@/stores/projectStore";
import ProjectDetailPlaceholders from "./ProjectDetailPlaceholders.vue";
import ProjectDetailTaskSets from "./ProjectDetailTaskSets.vue";
import { storeToRefs } from "pinia";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
