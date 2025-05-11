<template>
    <BaseDrawerCreate v-model="isDrawerOpen" v-model:element="task" :emptyElement :type="$t('task.singular')">
        <template #actions>
            <TaskActionCreate @created="taskCreated" :disabled="!isFormValid" :task />
        </template>
        <template #content>
            <TaskFieldContainer v-model="task" v-model:isValid="isFormValid" />
        </template>
    </BaseDrawerCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [task: TaskContract];
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });

const task = ref({} as TaskCreateContract);
const isFormValid = ref<boolean | null>(false);

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement: TaskCreateContract = {
    projectId: selectedProject.value.id,
    name: "",
    tabName: null,
    noExit: false,
    forceKill: false,
    historyPersistence: "OnError",
    workingDirTiles: [] as PlaceholderInsertTileContract[],
    commandTiles: [] as PlaceholderInsertTileContract[]
};

const taskCreated = (task: TaskContract) => {
    isDrawerOpen.value = false;
    emit("created", task);
};
</script>
