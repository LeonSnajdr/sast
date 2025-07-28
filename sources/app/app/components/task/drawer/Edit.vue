<template>
    <BaseDrawerEdit v-model="isDrawerOpen" :type="$t('task.singular')">
        <template v-if="task" #actions>
            <TaskActionSave @saved="taskSaved" :disabled="!isTaskValid" :task />
        </template>
        <template v-if="task" #content>
            <TaskFieldContainer v-model="task" v-model:isValid="isTaskValid" />
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

const isTaskValid = ref<boolean | null>(false);

const { task, loadTask } = useTask();

watch(
    () => props.taskId,
    () => {
        if (!props.taskId) {
            task.value = undefined;
            return;
        }

        loadTask(props.taskId);
    }
);

const taskSaved = (task: TaskContract) => {
    isDrawerOpen.value = false;
    emit("saved", task);
};
</script>
