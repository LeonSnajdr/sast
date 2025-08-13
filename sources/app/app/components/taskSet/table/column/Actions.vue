<template>
    <div @click.prevent.stop class="d-flex ga-2">
        <div>
            <VIconBtn @click="start()" :disabled="!isStartable" :loading="isStarting" color="success" icon="mdi-play" />
            <VTooltip :disabled="isStartable || isStarting" activator="parent">
                <p>{{ $t("taskSet.action.start.disabled") }}</p>
                <p v-if="!hasTasks">- {{ $t("taskSet.action.start.disabled.hasTasks") }}</p>
                <p v-if="hasRunningTerminal">- {{ $t("taskSet.action.start.disabled.hasRunningTerminal") }}</p>
                <p v-if="hasRunningTaskSetSession">- {{ $t("taskSet.action.start.disabled.hasRunningTaskSetSession") }}</p>
            </VTooltip>
        </div>
        <div>
            <VTooltip :disabled="isRestartable || isRestarting" activator="parent">
                <p>{{ $t("taskSet.action.restart.disabled") }}</p>
                <p v-if="isStarting || isStopping">- {{ $t("taskSet.action.restart.disabled.otherAction") }}</p>
                <p v-if="!hasRunningTerminal">- {{ $t("taskSet.action.restart.disabled.hasNoRunningTerminal") }}</p>
                <p v-if="hasRunningTaskSetSession">- {{ $t("taskSet.action.start.disabled.hasRunningTaskSetSession") }}</p>
            </VTooltip>
            <VIconBtn @click="restart()" :disabled="!isRestartable" :loading="isRestarting" color="info" icon="mdi-autorenew" />
        </div>
        <VIconBtn @click="stop()" :disabled="!hasRunningTerminal" :loading="isStopping" color="error" icon="mdi-stop" />
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskSet: TaskSetInfoContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const terminalStore = useTerminalStore();
const taskSetSessionStore = useTaskSetSessionStore();

const { terminals } = storeToRefs(terminalStore);
const { sessions } = storeToRefs(taskSetSessionStore);

const isStopping = ref(false);

const isStartable = computed(() => {
    return !hasRunningTerminal.value && !hasRunningTaskSetSession.value && hasTasks.value;
});

const isRestartable = computed(() => {
    return hasRunningTerminal.value && !isStarting.value && !isStopping.value && !hasRunningTaskSetSession.value;
});

const hasTasks = computed(() => {
    return props.taskSet.taskIds.length > 0;
});

const hasRunningTerminal = computed(() => {
    return terminals.value.some((x) => (x.task ? props.taskSet.taskIds.includes(x.task.id) : false));
});

const hasRunningTaskSetSession = computed(() =>
    sessions.value.some(({ tasks }) => tasks.some(({ status, taskId }) => ["NotStarted", "Running"].includes(status) && props.taskSet.taskIds.includes(taskId)))
);

const start = async () => {
    const startPromise = commands.taskSetStartOne(props.taskSet.projectId, props.taskSet.id);

    notify.success(t("action.start.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    const startResult = await startPromise;

    if (startResult.status === "error") {
        notify.error(t("action.start.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: startResult.error });
        return;
    }
};

const isStarting = computed(() => {
    return sessions.value.some((session) => session.taskSetId === props.taskSet.id && session.status === "Running" && session.kind == "Start");
});

const restart = async () => {
    const restartPromise = commands.taskSetRestartOne(props.taskSet.projectId, props.taskSet.id);

    notify.success(t("action.restart.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    const restartResult = await restartPromise;

    if (restartResult.status === "error") {
        notify.error(t("action.restart.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: restartResult.error });
        return;
    }
};

const isRestarting = computed(() => {
    return sessions.value.some((session) => session.taskSetId === props.taskSet.id && session.status === "Running" && session.kind == "Restart");
});

const stop = async () => {
    isStopping.value = true;

    const stopResult = await commands.taskSetStopOne(props.taskSet.id);

    if (stopResult.status === "error") {
        notify.error(t("action.stop.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: stopResult.error });
        return;
    }

    notify.success(t("action.stop.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    isStopping.value = false;
};
</script>
