<template>
    <BaseBtnIcon :loading="isLoading" icon="mdi-delete" variant="flat">
        {{ $t("action.delete") }}
        <BaseDialogConfirm
            @confirm="deleteTask"
            :message="$t('action.delete.description', { type: $t('task.singular'), name: task.name })"
            :title="$t('action.delete.title', { type: $t('task.singular') })"
            icon="mdi-checkbox-marked-circle-outline"
            iconColor="error"
        />
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const taskStore = useTaskStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const deleteTask = async () => {
    isLoading.value = true;

    const deleteResult = await commands.taskDeleteOne(props.task.id);

    isLoading.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("task.singular"), name: props.task.name }), { error: deleteResult.error });
        return;
    }

    notify.success(t("action.delete.success", { type: t("task.singular"), name: props.task.name }));

    taskStore.loadAll();

    isDialogOpen.value = false;

    navigateTo({ name: "index-project-id-task", params: { id: selectedProject.value.id } });
};
</script>
