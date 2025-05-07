<template>
    <BaseBtnIcon color="error" variant="flat">
        <BaseDialogConfirm
            @confirm="deleteProject()"
            :message="$t('action.delete.description', { type: $t('project.singular'), name: project.name })"
            :title="$t('action.delete.title', { type: $t('project.singular') })"
            icon="mdi-folder"
            iconColor="error"
        />
        {{ $t("action.delete") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    project: ProjectContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const deleteProject = async () => {
    const deleteResult = await commands.projectDeleteOne(props.project.id);

    if (deleteResult.status == "error") {
        notify.error(t("action.delete.error", { type: t("project.singular"), name: props.project.name }), { error: deleteResult.error });
        return;
    }

    notify.success(t("action.delete.success", { type: t("project.singular"), name: props.project.name }));

    navigateTo({ name: "index" });
};
</script>
