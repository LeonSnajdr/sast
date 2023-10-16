<template>
    <template v-if="project">
        <ProjectPlaceholders class="mb-4" />
        <ProjectTaskSets />
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
