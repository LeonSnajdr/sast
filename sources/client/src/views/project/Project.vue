<template>
    <ProjectDetail v-if="project" />
</template>

<script setup lang="ts">
import ProjectDetail from "./projectDetail/ProjectDetail.vue";

const props = defineProps<{
    projectId: string;
}>();

const projectStore = useProjectStore();

const { project } = storeToRefs(projectStore);

watch(
    () => props.projectId,
    async () => {
        projectStore.loadProject(props.projectId);
    }
);
</script>
