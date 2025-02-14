<template>
    <IconBtn :loading="isLoading" variant="flat" icon="mdi-delete">
        {{ $t("action.delete") }}
        <ConfirmationDialog
            @confirm="deleteTask"
            :message="$t('action.delete.description', { type: $t('task.title'), name: task.name })"
            :title="$t('action.delete.title', { type: $t('task.title') })"
            icon="mdi-checkbox-marked-circle-outline"
            iconColor="error"
        />
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

const deleteTask = async () => {
    isLoading.value = true;

    const deleteResult = await commands.taskDeleteOne(props.task.id);

    isLoading.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("task.title"), name: props.task.name }));
        console.error(deleteResult);
        return;
    }

    notify.success(t("action.delete.success", { type: t("task.title"), name: props.task.name }));

    navigateTo({ name: "index-project-id-task", params: { id: selectedProject.value.id } });

    isDialogOpen.value = false;
};
</script>
