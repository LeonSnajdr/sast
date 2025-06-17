<template>
    <BaseBtnIcon :loading="isLoading" icon="mdi-delete" variant="flat">
        {{ $t("action.delete") }}
        <BaseDialogConfirm
            @confirm="deleteTask"
            :message="$t('action.delete.description', { type: $t('taskSet.singular'), name: taskSet.name })"
            :title="$t('action.delete.title', { type: $t('taskSet.singular') })"
            icon="mdi-checkbox-multiple-marked-circle-outline"
            iconColor="error"
        />
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskSet: TaskSetContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const taskSetStore = useTaskSetStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const deleteTask = async () => {
    isLoading.value = true;

    const deleteResult = await commands.taskSetDeleteOne(props.taskSet.id);

    isLoading.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: deleteResult.error });
        return;
    }

    notify.success(t("action.delete.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    taskSetStore.loadAll();

    isDialogOpen.value = false;

    navigateTo({ name: "index-project-id-taskSet", params: { id: selectedProject.value.id } });
};
</script>
