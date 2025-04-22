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

<style lang="scss" scoped>
@use "@/assets/styles/settings";

:deep(.v-slide-group__next) {
    margin-right: -(settings.$spacer * 3);
}
</style>
