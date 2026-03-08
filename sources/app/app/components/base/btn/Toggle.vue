<template>
    <VIconBtn @click="active = !active" />
</template>

<script setup lang="ts">
const active = defineModel<boolean>({ required: true });
</script>
