<template>
    <BaseBtnIcon @click="navigateToTerminal()" :disabled="!terminal" color="secondary" icon="mdi-tab" />
</template>

<script setup lang="ts">
const props = defineProps<{
    task: TaskInfoContract;
}>();

const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);

const terminal = computed(() => {
    return terminals.value.find((x) => (x.task ? x.task.id === props.task.id : false));
});

const navigateToTerminal = () => {
    if (!terminal.value) {
        return;
    }

    navigateTo({ name: "index-project-id-terminal-terminalId", params: { id: terminal.value.projectId, terminalId: terminal.value.id } });
};
</script>
