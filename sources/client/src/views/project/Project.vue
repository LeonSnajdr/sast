<template>
    <div class="grid grid-cols-7 gap-4">
        <ProjectList class="col-span-2" />
        <ProjectDetail v-if="project" v-model:project="project" class="col-span-5" />
    </div>
</template>

<script setup lang="ts">
import ProjectList from "./ProjectList.vue";
import ProjectDetail from "./projectDetail/ProjectDetail.vue";
import { ref, watch } from "vue";
import type { FullProjectContract } from "@/bindings";
import * as commands from "@/bindings";
import { useToast } from "primevue/usetoast";

const props = defineProps<{
    projectId?: string;
}>();

const toast = useToast();

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
        toast.add({ severity: "error", summary: "Error", detail: "Loading project failed", life: 3000 });
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
