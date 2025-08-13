<template>
    <VBtn :loading="isDeleting" color="error" variant="flat">
        <BaseDialogConfirm
            @confirm="deleteProject()"
            :message="$t('action.delete.description', { type: $t('project.singular'), name: project.name })"
            :title="$t('action.delete.title', { type: $t('project.singular') })"
            icon="mdi-folder"
            iconColor="error"
        />
        {{ $t("action.delete") }}
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    project: ProjectContract;
}>();

const projectStore = useProjectStore();

const notify = useNotify();
const { t } = useI18n();

const isDeleting = ref(false);

const deleteProject = async () => {
    isDeleting.value = true;
    const deleteResult = await commands.projectDeleteOne(props.project.id);
    isDeleting.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("project.singular"), name: props.project.name }), { error: deleteResult.error });
        return;
    }

    notify.success(t("action.delete.success", { type: t("project.singular"), name: props.project.name }));

    await projectStore.loadAllProjects();

    navigateTo({ name: "index" });
};
</script>
