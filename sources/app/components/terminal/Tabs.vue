<template>
    <VSlideGroup showArrows>
        <VSlideGroupItem v-for="terminal in terminals" :key="terminal.id">
            <TerminalTab :terminal />
        </VSlideGroupItem>
    </VSlideGroup>
</template>

<script setup lang="ts">
const terminalStore = useTerminalStore();

const { terminals } = storeToRefs(terminalStore);
</script>
