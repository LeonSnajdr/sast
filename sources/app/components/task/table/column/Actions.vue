<template>
    <div @click.prevent.stop class="d-flex">
        <BaseBtnIcon @click="start()" :disabled="hasRunningTerminal" :loading="isStarting" color="success" icon="mdi-play" />
        <BaseBtnIcon @click="restart()" :disabled="!hasRunningTerminal" :loading="isRestarting" color="info" icon="mdi-autorenew" />
        <BaseBtnIcon @click="stop()" :disabled="!hasRunningTerminal" :loading="isStopping" color="error" icon="mdi-stop" />
        <BaseBtnIcon @click="navigateToTab()" :disabled="!hasRunningTerminal" color="secondary" icon="mdi-tab" />
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);

const isStarting = ref(false);
const isStopping = ref(false);
const isRestarting = ref(false);

const hasRunningTerminal = computed(() => {
    return terminals.value.some((x) => x.taskId === props.task.id);
});

const terminal = computed(() => {
    return terminals.value.find((x) => x.taskId === props.task.id);
});

const start = async () => {
    isStarting.value = true;

    const startResult = await commands.taskStartOne(props.task.projectId, props.task.id);

    if (startResult.status === "error") {
        notify.error(t("action.start.error", { type: t("task.singular"), name: props.task.name }), { error: startResult.error });
        return;
    }

    notify.success(t("action.start.success", { type: t("task.singular"), name: props.task.name }));

    isStarting.value = false;
};

const restart = async () => {
    isRestarting.value = true;

    const restartResult = await commands.taskRestartOne(props.task.projectId, props.task.id);

    if (restartResult.status === "error") {
        notify.error(t("action.restart.error", { type: t("task.singular"), name: props.task.name }), { error: restartResult.error });
        return;
    }

    notify.success(t("action.restart.success", { type: t("task.singular"), name: props.task.name }));

    isRestarting.value = false;
};

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

const navigateToTab = () => {
    if (!terminal.value) {
        return;
    }

    navigateTo({ name: "index-project-id-pty-sessionId", params: { id: terminal.value.projectId, sessionId: terminal.value.id } });
};
</script>
