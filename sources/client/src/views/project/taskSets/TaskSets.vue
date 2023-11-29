<template>
    <v-card>
        <v-card-title>
            {{ $t("taskSets.title") }}
            <v-spacer />
            <v-btn-icon @click="inTaskSetEdit = !inTaskSetEdit" :icon="inTaskSetEdit ? 'mdi-close' : 'mdi-pencil'" />
        </v-card-title>
        <v-card-text>
            <template v-if="inTaskSetEdit">
                <draggable v-model="taskSets" @end="taskSetOrderChanged" itemKey="id">
                    <template #item="{ element: taskSet }">
                        <TaskSetEdit :taskSet="taskSet" />
                    </template>
                </draggable>

                <TaskSetCreate />
            </template>
            <template v-else>
                <v-list v-if="taskSets.length > 0">
                    <TaskSetView v-for="taskSet in taskSets" :key="taskSet.id" :taskSet="taskSet" />
                </v-list>
                <span v-else>{{ $t("taskSets.noItems") }}</span>
            </template>
        </v-card-text>
    </v-card>
</template>

<script setup lang="ts">
import TaskSetEdit from "@/views/project/taskSets/TaskSetEdit.vue";
import TaskSetCreate from "@/views/project/taskSets/TaskSetCreate.vue";
import TaskSetView from "@/views/project/taskSets/TaskSetView.vue";
import draggable from "vuedraggable";

const projectStore = useProjectStore();
const taskSetStore = useTaskSetStore();

const { selectedProjectId } = storeToRefs(projectStore);
const { taskSets, inTaskSetEdit } = storeToRefs(taskSetStore);

watch(
    selectedProjectId,
    async () => {
        await taskSetStore.loadTaskSetList();
    },
    { immediate: true }
);

const taskSetOrderChanged = () => {
    console.log(taskSets.value.map((m) => m.name));
};
</script>
