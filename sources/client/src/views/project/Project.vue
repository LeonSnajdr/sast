<template>
    <template v-if="project">
        <v-row-single>
            <ProjectPlaceholders />
        </v-row-single>
        <v-row-single>
            <ProjectTaskSets />
        </v-row-single>
    </template>
</template>

<script setup lang="ts">
import ProjectPlaceholders from "@/views/project/placeholders/ProjectPlaceholders.vue";
import ProjectTaskSets from "@/views/project/taskSets/ProjectTaskSets.vue";

const props = defineProps<{
    projectId: string;
}>();

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);

watch(
    () => props.projectId,
    async () => {
        projectStore.loadProject(props.projectId);
    },
    { immediate: true }
);
</script>
