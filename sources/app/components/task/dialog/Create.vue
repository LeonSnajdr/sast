<template>
    <BaseDialogCreate v-model="isDialogOpen" :emptyElement :type="$t('task.singular')" icon="mdi-checkbox-marked-circle-outline">
        <template #actions="{ isFormValid, element }">
            <TaskActionCreate @created="taskCreated" :disabled="!isFormValid" :task="element" />
        </template>
        <template #fields="{ element: task }">
            <TaskFieldName v-model="task.name" />
            <VRow>
                <VCol cols="9">
                    <TaskFieldTabName v-model="task.tabName" />
                </VCol>
                <VCol cols="3">
                    <TaskFieldNoExit v-model="task.noExit" />
                </VCol>
            </VRow>
            <TaskFieldCommandTiles v-model="task.commandTiles" />
            <TaskFieldWorkingDirTiles v-model="task.workingDirTiles" />
        </template>
    </BaseDialogCreate>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    created: [id: string];
}>();

const isDialogOpen = defineModel<boolean>({ required: true });

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const emptyElement: TaskCreateContract = {
    projectId: selectedProject.value.id,
    name: "",
    tabName: null,
    noExit: true,
    workingDirTiles: [] as PlaceholderInsertTileContract[],
    commandTiles: [] as PlaceholderInsertTileContract[]
};

const taskCreated = (id: string) => {
    isDialogOpen.value = false;
    emit("created", id);
};
</script>
