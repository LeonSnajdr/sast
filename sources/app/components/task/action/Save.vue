<template>
    <BaseBtnIcon @click="taskSave()" :loading="isLoading" color="success" icon="mdi-content-save" variant="flat">
        {{ $t("action.save") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const emit = defineEmits<{
    saved: [task: TaskContract];
}>();

const props = defineProps<{
    task: TaskContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const taskStore = useTaskStore();

const isLoading = ref(false);

const taskSave = async () => {
    isLoading.value = true;

    const updateContract: TaskUpdateContract = {
        id: props.task.id,
        name: props.task.name,
        tabName: props.task.tabName,
        noExit: props.task.noExit,
        commandTiles: props.task.commandTiles,
        workingDirTiles: props.task.workingDirTiles
    };

    const saveResult = await commands.taskUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("task.singular"), name: props.task.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("task.singular"), name: props.task.name }));

    taskStore.loadAll();

    emit("saved", saveResult.data);
};
</script>
