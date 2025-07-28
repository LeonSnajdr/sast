<template>
    <VAutocomplete
        :key="selectedTask?.id"
        v-model="selectedTask"
        :items="notAddedTasks"
        :label="$t('taskSetTask.search')"
        :loading="isLoading"
        :persistentPlaceholder="false"
        itemTitle="name"
        clearable
        returnObject
    />

    <BaseBtnIcon @click="addTask()" :disabled="!selectedTask" icon="mdi-plus">{{ $t("action.add") }}</BaseBtnIcon>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    add: [id: TaskInfoContract];
}>();

const props = defineProps<{
    taskSetTasks: TaskSetTaskInfoContract[];
}>();

const taskStore = useTaskStore();

const { isLoading, tasks } = storeToRefs(taskStore);

const selectedTask = ref<TaskInfoContract>();

const addTask = () => {
    if (!selectedTask.value) return;

    emit("add", selectedTask.value);

    selectedTask.value = undefined;
};

const notAddedTasks = computed(() => {
    const alreadyContainedTaskIds = props.taskSetTasks.map((x) => x.taskId) ?? [];

    return tasks.value.filter((x) => !alreadyContainedTaskIds.includes(x.id));
});
</script>
