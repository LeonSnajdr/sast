<template>
    <VBtn
        @click="placeholderSave()"
        :disabled
        :loading="isPlaceholderSaving"
        color="success"
        prependIcon="mdi-content-save"
        variant="flat"
        v-tooltip="$t('keybind.controlS.tooltip')"
    >
        {{ $t("action.save") }}
    </VBtn>
</template>

<script setup lang="ts">
const props = defineProps<{
    placeholder: PlaceholderContract;
    disabled: boolean;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();

const { selectedProject } = storeToRefs(projectStore);
const { isPlaceholderSaving, savePlaceholder } = usePlaceholder();

useHotkey("cmd+s", () => placeholderSave(), { inputs: true });

const placeholderSave = async () => {
    if (props.disabled) return;

    const isSaved = await savePlaceholder(props.placeholder);

    if (!isSaved) {
        return;
    }

    notify.success(t("action.save.success", { type: t("placeholder.singular"), name: props.placeholder.name }));
    navigateTo({ name: "index-project-id-placeholder", params: { id: selectedProject.value.id } });
};
</script>
