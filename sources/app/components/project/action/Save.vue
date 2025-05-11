<template>
    <BaseBtnIcon
        @click="projectSave()"
        :disabled
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
const emit = defineEmits<{
    saved: [project: ProjectContract];
}>();

const props = defineProps<{
    project: ProjectContract;
    disabled: boolean;
}>();

const notify = useNotify();
const { t } = useI18n();

const isLoading = ref(false);

useKeybind(["control", "s"], () => projectSave());

const projectSave = async () => {
    if (props.disabled) return;

    isLoading.value = true;

    const updateContract: ProjectUpdateContract = {
        id: props.project.id,
        name: props.project.name,
        favorite: props.project.favorite,
        quickSwitchKeybind: props.project.quickSwitchKeybind
    };

    const saveResult = await commands.projectUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("project.singular"), name: props.project.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("project.singular"), name: props.project.name }));

    emit("saved", saveResult.data);
};
</script>
