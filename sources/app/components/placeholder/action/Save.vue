<template>
    <BaseBtnIcon
        @click="placeholderSave({ closeAfterSave: true })"
        :loading="isLoading"
        color="success"
        icon="mdi-content-save"
        variant="flat"
        v-tooltip="$t('keybind.controlS.tooltip')"
    >
        {{ $t("action.save") }}
    </BaseBtnIcon>
</template>

<script setup lang="ts">
const props = defineProps<{
    placeholder: PlaceholderContract;
    disabled: boolean;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const placeholderStore = usePlaceholderStore();

const { selectedProject } = storeToRefs(projectStore);

const isDialogOpen = ref(false);
const isLoading = ref(false);

useKeybind(["control", "s"], () => placeholderSave({ closeAfterSave: false }));

const placeholderSave = async (options: { closeAfterSave: boolean }) => {
    if (props.disabled) return;

    isLoading.value = true;

    const updateContract: PlaceholderUpdateContract = {
        id: props.placeholder.id,
        projectId: props.placeholder.projectId,
        name: props.placeholder.name,
        value: props.placeholder.value,
        visibility: props.placeholder.visibility,
        kind: props.placeholder.kind,
        source: props.placeholder.source
    };

    const saveResult = await commands.placeholderUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("placeholder.singular"), name: props.placeholder.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("placeholder.singular"), name: props.placeholder.name }));

    isDialogOpen.value = false;

    placeholderStore.loadAll();

    if (options.closeAfterSave) {
        navigateTo({ name: "index-project-id-placeholder", params: { id: selectedProject.value.id } });
    }
};
</script>
