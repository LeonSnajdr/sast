<template>
    <VAppBar>
        <PtySessionTabs />
        <VSpacer />
        <PtySessionActionSpawn />
    </VAppBar>

    <div class="h-100">
        <NuxtPage />
    </div>
</template>

<script setup lang="ts">
const ptySessionStore = usePtySessionStore();

onBeforeMount(() => {
    ptySessionStore.loadAll();
});
</script>
