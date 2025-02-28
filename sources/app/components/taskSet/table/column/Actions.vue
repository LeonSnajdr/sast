<template>
    <div @click.prevent.stop class="d-flex">
        <BaseBtnIcon @click="start()" :disabled="hasRunningPtySession" :loading="isStarting" color="success" icon="mdi-play" />
        <BaseBtnIcon @click="restart()" :disabled="!hasRunningPtySession" :loading="isRestarting" color="info" icon="mdi-autorenew" />
        <BaseBtnIcon @click="stop()" :disabled="!hasRunningPtySession" :loading="isStopping" color="error" icon="mdi-stop" />
        <BaseBtnIcon @click="navigateToTab()" :disabled="!hasRunningPtySession" color="secondary" icon="mdi-tab" />
    </div>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskSet: TaskSetInfoContract;
}>();

const notify = useNotify();
const { t } = useI18n();

const ptySessionStore = usePtySessionStore();

const { ptySessions } = storeToRefs(ptySessionStore);

const isStarting = ref(false);
const isStopping = ref(false);
const isRestarting = ref(false);

const hasRunningPtySession = computed(() => {
    return ptySessions.value.some((x) => x.taskSetId === props.taskSet.id);
});

const openPtySessions = computed(() => {
    return ptySessions.value.find((x) => x.taskSetId === props.taskSet.id);
});

const start = async () => {
    isStarting.value = true;

    const startResult = await commands.taskSetStartOne(props.taskSet.projectId, props.taskSet.id);

    if (startResult.status === "error") {
        notify.error(t("action.start.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: startResult.error });
        return;
    }

    notify.success(t("action.start.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    isStarting.value = false;
};

const restart = async () => {
    isRestarting.value = true;

    const restartResult = await commands.taskSetRestartOne(props.taskSet.projectId, props.taskSet.id);

    if (restartResult.status === "error") {
        notify.error(t("action.restart.error", { type: t("taskSet.singular"), name: props.taskSet.name }), { error: restartResult.error });
        return;
    }

    notify.success(t("action.restart.success", { type: t("taskSet.singular"), name: props.taskSet.name }));

    isRestarting.value = false;
};

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

const navigateToTab = () => {
    if (!openPtySessions.value) {
        return;
    }

    navigateTo({ name: "index-project-id-pty-sessionId", params: { id: openPtySessions.value.projectId, sessionId: openPtySessions.value.sessionId } });
};
</script>
