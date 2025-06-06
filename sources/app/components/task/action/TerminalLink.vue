<template>
    <TerminalShellStatusBadge :shellStatus="terminal?.shellStatus" offsetX="12" offsetY="2">
        <BaseBtnIcon @click="navigateToTerminal()" :disabled="!terminal" color="secondary" icon="mdi-tab" />
    </TerminalShellStatusBadge>
</template>

<script setup lang="ts">
const props = defineProps<{
    taskId: string;
}>();

const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);

const terminal = computed(() => {
    return terminals.value.find((x) => x.task?.id === props.taskId);
});

const navigateToTerminal = () => {
    if (!terminal.value) {
        return;
    }

    navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: terminal.value.projectId, terminalId: terminal.value.id } });
};
</script>
