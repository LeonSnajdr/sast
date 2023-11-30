<template>
    <template v-if="selectedProjectId">
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

const { selectedProjectId } = storeToRefs(projectStore);
const { inTaskSetEdit } = storeToRefs(taskSetStore);

watch(
    () => props.projectId,
    async () => {
        selectedProjectId.value = props.projectId;
        inTaskSetEdit.value = false;
    },
    { immediate: true }
);
</script>
