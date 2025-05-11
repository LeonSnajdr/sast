<template>
    <VAppBar>
        <TerminalTabs />
        <TerminalActionCreate />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
definePageMeta({
    projectSwitchName: "index-project-id-terminal"
});
</script>
