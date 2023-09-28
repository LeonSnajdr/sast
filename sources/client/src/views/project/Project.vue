<template>
    <div class="grid">
        <div class="col-3">
            <ProjectList />
        </div>
        <div class="col-9">
            <ProjectDetail />
        </div>
    </div>
</template>

<script setup lang="ts">
import ProjectList from "./ProjectList.vue";
import ProjectDetail from "./projectDetail/ProjectDetail.vue";
import { watch } from "vue";
import { useProjectStore } from "@/stores/projectStore";

const props = defineProps<{
    projectId?: string;
}>();

const projectStore = useProjectStore();

watch(
    () => props.projectId,
    () => {
        projectStore.loadProject(props.projectId);
    },
    { immediate: true }
);
</script>
