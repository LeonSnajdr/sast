<template>
    <VBtn
        @click="taskSave()"
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
const emit = defineEmits<{
    saved: [task: TaskContract];
}>();

const props = defineProps<{
    task: TaskContract;
    disabled: boolean;
}>();

const notify = useNotify();
const { t } = useI18n();

const taskStore = useTaskStore();

const isLoading = ref(false);

useHotkey("cmd+s", () => taskSave(), { inputs: true });

const taskSave = async () => {
    if (props.disabled) return;

    isLoading.value = true;

    const updateContract: TaskUpdateContract = {
        id: props.task.id,
        name: props.task.name,
        tabName: props.task.tabName,
        noExit: props.task.noExit,
        forceKill: props.task.forceKill,
        historyPersistence: props.task.historyPersistence,
        commandTiles: props.task.commandTiles,
        workingDirTiles: props.task.workingDirTiles
    };

    const saveResult = await commands.taskUpdateOne(updateContract);

    isLoading.value = false;

    if (saveResult.status == "error") {
        notify.error(t("action.save.error", { type: t("task.singular"), name: props.task.name }), { error: saveResult.error });
        return;
    }

    notify.success(t("action.save.success", { type: t("task.singular"), name: props.task.name }));

    taskStore.loadAll();

    emit("saved", saveResult.data);
};
</script>
