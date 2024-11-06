<template>
    <VSnackbarQueue v-model="notifications" location="bottom right" closable />
</template>

<script setup lang="ts">
const { notifications } = useNotify();
</script>
