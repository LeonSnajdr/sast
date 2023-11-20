<template>
    <template v-if="project">
        <v-row-single>
            <Placeholders />
        </v-row-single>
        <v-row-single>
            <TaskSets />
        </v-row-single>
    </template>
</template>

<script setup lang="ts">
import Placeholders from "@/views/project/placeholders/Placeholders.vue";
import TaskSets from "@/views/project/taskSets/TaskSets.vue";

const props = defineProps<{
    projectId: string;
}>();

const projectStore = useProjectStore();

const { selectedProjectId, project } = storeToRefs(projectStore);

watch(
    () => props.projectId,
    async () => {
        selectedProjectId.value = props.projectId;
        projectStore.resetPageState();
    },
    { immediate: true }
);
</script>
