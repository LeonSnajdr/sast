<template>
    <BaseDrawerEdit v-model="isDrawerOpen" :type="$t('task.singular')">
        <template v-if="task" #actions>
            <TaskActionSave @saved="taskSaved" :task />
        </template>
        <template v-if="task" #fields>
            <TaskFieldName v-model="task.name" autofocus />
            <TaskFieldTabName v-model="task.tabName" />
            <VRow>
                <VCol>
                    <TaskFieldNoExit v-model="task.noExit" />
                </VCol>
                <VCol>
                    <TaskFieldForceKill v-model="task.forceKill" />
                </VCol>
            </VRow>
            <TaskFieldHistoryPersistence v-model="task.historyPersistence" />
            <TaskFieldCommandTiles v-model="task.commandTiles" />
            <TaskFieldWorkingDirTiles v-model="task.workingDirTiles" />
        </template>
    </BaseDrawerEdit>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    saved: [task: TaskContract];
}>();

const props = defineProps<{
    taskId?: string;
}>();

const isDrawerOpen = defineModel<boolean>({ required: true });

const { task, loadTask } = useTask();

watch(
    () => props.taskId,
    () => {
        if (!props.taskId) return;

        loadTask(props.taskId);
    }
);

const taskSaved = (task: TaskContract) => {
    isDrawerOpen.value = false;
    emit("saved", task);
};
</script>
