<template>
    <TerminalDrawer />

    <VAppBar>
        <TerminalTabs />
        <TerminalActionCreate />
        <VSpacer />
        <TaskSetSessionBtnInfo @click="isDrawerOpen = !isDrawerOpen" :active="isDrawerOpen" />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    projectSwitchName: "index-project-id-terminal"
});

const terminalDrawerStore = useTerminalDrawerStore();

const { isDrawerOpen } = storeToRefs(terminalDrawerStore);
</script>
