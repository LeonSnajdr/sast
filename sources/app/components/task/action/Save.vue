<template>
    <IconBtn @click="taskSave()" :loading="isLoading" color="success" icon="mdi-content-save" variant="flat">
        {{ $t("action.save") }}
    </IconBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const taskSave = async () => {
    isLoading.value = true;

    const updateContract: TaskUpdateContract = {
        id: props.task.id,
        name: props.task.name,
        blocking: props.task.blocking,
        commandTiles: props.task.commandTiles,
        workingDirTiles: props.task.workingDirTiles
    };

    const saveResult = await commands.taskUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("task.title"), name: props.task.name }));
        return;
    }

    notify.success(t("action.save.success", { type: t("task.title"), name: props.task.name }));

    navigateTo({ name: "index-project-id-task", params: { id: selectedProject.value.id } });

    isDialogOpen.value = false;
};
</script>
