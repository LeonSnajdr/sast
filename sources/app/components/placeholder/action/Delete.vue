<template>
    <VBtn :loading="isLoading">
        {{ $t("action.delete") }}
        <ConfirmationDialog
            @confirm="placeholderDelete"
            :message="$t('placeholder.delete.description', { placeholderName: placeholder.name })"
            :title="$t('placeholder.delete.title')"
            icon="mdi-label"
            iconColor="error"
        />
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    placeholder: PlaceholderContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

const placeholderDelete = async () => {
    isLoading.value = true;

    const deleteResult = await commands.placeholderDeleteOne(props.placeholder.id);

    isLoading.value = false;

    if (deleteResult.status == "error") {
        notify.error(t("placeholder.delete.error"));
        return;
    }

    notify.success(t("placeholder.delete.success", { placeholderName: props.placeholder.name }));

    navigateTo({ name: "index-project-id-placeholder", params: { id: selectedProject.value.id } });

    isDialogOpen.value = false;
};
</script>
