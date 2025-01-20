<template>
    <VDialog v-model="isDialogOpen" activator="parent" width="800" eager>
        <VCard>
            <VCardTitle>
                <VIcon color="success" icon="mdi-label" />
                {{ $t("placeholder.delete.title") }}
            </VCardTitle>
            <VCardText>
                {{ $t("placeholder.delete.description", { placeholderName: placeholder.name }) }}
            </VCardText>
            <VCardActions>
                <VBtn @click="placeholderDelete()" :loading="isLoading">{{ $t("action.delete") }}</VBtn>
                <VBtn @click="isDialogOpen = false">{{ $t("action.close") }}</VBtn>
            </VCardActions>
        </VCard>
    </VDialog>
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

    isDialogOpen.value = false;

    navigateTo({ name: "index-project-id-placeholder", params: { id: selectedProject.value.id } });
};
</script>
