<template>
    <VIconBtn @click="expanded = !expanded" :rotate="expanded ? 180 : 0" icon="mdi-chevron-up" />
</template>

<script setup lang="ts">
const expanded = defineModel<boolean>({ required: true });
</script>
