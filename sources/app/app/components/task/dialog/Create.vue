<template>
    <BaseDialogCreate v-model="isDialogOpen" v-model:element="task" :emptyElement :type="$t('task.singular')" icon="mdi-checkbox-marked-circle-outline">
        <template #content>
            <TaskFieldFavorite v-model="task.favorite" />
            <TaskFieldContainer v-model="task" v-model:isValid="isTaskValid" />
        </template>
        <template #actions>
            <TaskActionCreate @created="taskCreated" :disabled="!isTaskValid" :task />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [task: TaskContract];
}>();

const task = ref({} as TaskCreateContract);
const isTaskValid = ref<boolean | null>(false);
const isDialogOpen = ref(false);

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement = computed<TaskCreateContract>(() => ({
    projectId: selectedProject.value.id,
    name: "",
    favorite: false,
    tabName: null,
    noExit: false,
    forceKill: false,
    historyPersistence: "OnError",
    workingDirTiles: [] as PlaceholderInsertTileContract[],
    commandTiles: [] as PlaceholderInsertTileContract[]
}));

const taskCreated = (task: TaskContract) => {
    isDialogOpen.value = false;
    emit("created", task);
};
</script>
