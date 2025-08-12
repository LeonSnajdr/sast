<template>
    <VBtn
        @click="saveTaskSet()"
        :disabled
        :loading="isLoading"
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
    taskSet: TaskSetContract;
    disabled: boolean;
    keybindDisabled: boolean;
}>();

const notify = useNotify();
const { t } = useI18n();

const projectStore = useProjectStore();
const taskSetStore = useTaskSetStore();

const { selectedProject } = storeToRefs(projectStore);

const isLoading = ref(false);

useHotkey(
    "cmd+s",
    () => {
        if (props.keybindDisabled) return;

        saveTaskSet();
    },
    { inputs: true }
);

const saveTaskSet = async () => {
    if (props.disabled) return;

    isLoading.value = true;

    const updateContract: TaskSetUpdateContract = {
        id: props.taskSet.id,
        name: props.taskSet.name,
        tasks: props.taskSet.tasks
    };

    const saveResult = await commands.taskSetUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    taskSetStore.loadAll();

    navigateTo({ name: "index-project-id-taskSet", params: { id: selectedProject.value.id } });
};
</script>
