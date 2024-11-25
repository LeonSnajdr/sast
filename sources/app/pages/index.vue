<template>
    <NuxtPage />
</template>

<script setup lang="ts">
const presentation = usePresentation();

onBeforeMount(() => {
    presentation.applySetting();
});

definePageMeta({
    middleware: "initialize"
});
</script>
