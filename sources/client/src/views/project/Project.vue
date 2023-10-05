<template>
    <div>
        <ProjectList />
        <!--<ProjectDetail v-if="project" v-model:project="project" />-->
    </div>
</template>

<script setup lang="ts">
import ProjectList from "./ProjectList.vue";
import ProjectDetail from "./projectDetail/ProjectDetail.vue";
import { ref, watch } from "vue";
import type { FullProjectContract } from "@/bindings";
import { useNotificationStore } from "@/stores/notificationStore";
import * as commands from "@/bindings";

const props = defineProps<{
    projectId?: string;
}>();

const notify = useNotificationStore();

const project = ref<FullProjectContract>();

const loadProject = async () => {
    try {
        if (!props.projectId) {
            project.value = undefined;
            return;
        }

        const fullProject = await commands.getFullProject(props.projectId);
        project.value = fullProject ?? undefined;
    } catch (error) {
        console.error("Loading project failed", error);
        notify.error("TODO");
    }
};

watch(
    () => props.projectId,
    () => {
        loadProject();
    },
    { immediate: true }
);
</script>
