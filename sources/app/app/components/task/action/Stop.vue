<template>
    <VIconBtn @click="stop()" :disabled="!hasRunningTerminal" :loading="isStopping" color="error" icon="mdi-stop" />
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const notify = useNotify();

const terminalStore = useTerminalStore();
const { t } = useI18n();

const { terminals } = storeToRefs(terminalStore);

const isStopping = ref(false);

const hasRunningTerminal = computed(() => {
    return terminals.value.some((x) => (x.task ? x.task.id === props.task.id : false));
});

const stop = async () => {
    isStopping.value = true;

    const stopResult = await commands.taskStopOne(props.task.id);

    if (stopResult.status === "error") {
        notify.error(t("action.stop.error", { type: t("task.singular"), name: props.task.name }), { error: stopResult.error });
        return;
    }

    notify.success(t("action.stop.success", { type: t("task.singular"), name: props.task.name }));

    isStopping.value = false;
};
</script>
