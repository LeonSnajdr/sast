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
const taskSetStore = useTaskSetStore();

const { project } = storeToRefs(projectStore);

watch(
    () => props.projectId,
    async () => {
        projectStore.resetPageState();
        await projectStore.loadProject(props.projectId);
        await taskSetStore.loadTaskSets();
    },
    { immediate: true }
);
</script>
