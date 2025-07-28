<template>
    <TerminalShellStatusBadge :shellStatus="!disabled ? terminal?.shellStatus : undefined" offsetX="12" offsetY="2">
        <BaseBtnIcon @click="navigateToTerminal()" :disabled="!terminal || disabled" color="secondary" icon="mdi-tab" />
    </TerminalShellStatusBadge>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskId: string;
    taskSetId?: string;
    disabled?: boolean;
}>();

const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);

const terminal = computed(() => {
    return terminals.value.find((x) => {
        const matchesTask = x.task?.id === props.taskId;
        const matchesTaskSet = props.taskSetId ? x.taskSet?.id === props.taskSetId : true;

        return matchesTask && matchesTaskSet;
    });
});

const navigateToTerminal = () => {
    if (!terminal.value) {
        return;
    }

    navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: terminal.value.projectId, terminalId: terminal.value.id } });
};
</script>
