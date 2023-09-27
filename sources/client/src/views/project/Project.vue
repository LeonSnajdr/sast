<template>
    <div class="grid">
        <div class="col-3" style="border: 1px solid red">
            <ProjectList />
        </div>
        <div class="col-9">
            <ProjectDetail v-if="projectId" />
        </div>
    </div>
</template>

<script setup lang="ts">
import ProjectList from "./ProjectList.vue";
import ProjectDetail from "./ProjectDetail.vue";
import { watch } from "vue";
import { useProjectStore } from "@/stores/projectStore";

const props = defineProps<{
    projectId?: string;
}>();

const projectStore = useProjectStore();

watch(
    () => props.projectId,
    () => {
        projectStore.projectId = props.projectId;
    },
    { immediate: true }
);
</script>
