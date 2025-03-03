<template>
    <BaseDrawerEdit v-model="isDrawerOpen" :type="$t('task.singular')">
        <template v-if="task" #actions>
            <TaskActionSave @saved="taskSaved" :task />
        </template>
        <template v-if="task" #fields>
            <TaskFieldName v-model="task.name" />
            <VRow>
                <VCol cols="8">
                    <TaskFieldTabName v-model="task.tabName" />
                </VCol>
                <VCol cols="4">
                    <TaskFieldNoExit v-model="task.noExit" />
                </VCol>
            </VRow>
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
