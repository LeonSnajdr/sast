<template>
    <BaseBtnIcon @click="start()" :disabled="hasRunningTerminal" :loading="isStarting" color="success" icon="mdi-play" />
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const notify = useNotify();

const terminalStore = useTerminalStore();
const { t } = useI18n();

const { terminals } = storeToRefs(terminalStore);

const isStarting = ref(false);

const hasRunningTerminal = computed(() => {
    return terminals.value.some((x) => (x.task ? x.task.id === props.task.id : false));
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
</script>
