<template>
    <v-card>
        <v-card-title>
            {{ $t("projectDetailTaskSetsView.title") }}
            <v-spacer />
            <v-icon icon="mdi-plus" />
        </v-card-title>
        <v-card-text>
            <v-list>
                <template v-for="taskSet in project.task_sets" :key="taskSet.id">
                    <ProjectDetailTaskSetView :taskSet="taskSet" />
                </template>
            </v-list>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import ProjectDetailTaskSetView from "./ProjectTaskSetView.vue";

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);
</script>
