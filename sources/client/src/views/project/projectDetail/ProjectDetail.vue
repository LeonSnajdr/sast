<template>
    <v-card class="h-100">
        <v-card-title>
            {{ project.name }}
        </v-card-title>
    </v-card>
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

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
