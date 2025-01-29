<template>
    <VContainer :id="`pty-session-${route.params.sessionId}`" class="h-100" />
</template>

<script setup lang="ts">
const route = useRoute("index-project-id-pty-sessionId");
</script>
