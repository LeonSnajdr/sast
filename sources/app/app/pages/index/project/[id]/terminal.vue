<template>
    <TaskSetDrawer v-model="isTaskSetDrawerOpen" />

    <VAppBar>
        <TerminalTabs />
        <TerminalActionCreate />
        <VSpacer />
        <TaskSetSessionBtnInfo @click="isTaskSetDrawerOpen = !isTaskSetDrawerOpen" :active="isTaskSetDrawerOpen" />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    projectSwitchName: "index-project-id-terminal"
});

const isTaskSetDrawerOpen = ref(false);
</script>
