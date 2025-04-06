<template>
    <BaseBtnIcon @click="restart()" :disabled="!hasRunningTerminal" :loading="isRestarting" color="info" icon="mdi-autorenew" />
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const notify = useNotify();

const terminalStore = useTerminalStore();
const { t } = useI18n();

const { terminals } = storeToRefs(terminalStore);

const isRestarting = ref(false);

const hasRunningTerminal = computed(() => {
    return terminals.value.some((x) => (x.task ? x.task.id === props.task.id : false));
});

const restart = async () => {
    isRestarting.value = true;

    const restartResult = await commands.taskRestartOne(props.task.id);

    if (restartResult.status === "error") {
        notify.error(t("action.restart.error", { type: t("task.singular"), name: props.task.name }), { error: restartResult.error });
        return;
    }

    notify.success(t("action.restart.success", { type: t("task.singular"), name: props.task.name }));

    isRestarting.value = false;
};
</script>
