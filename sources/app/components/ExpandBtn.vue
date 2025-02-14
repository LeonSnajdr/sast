<template>
    <IconBtn @click="expanded = !expanded" :icon="expanded ? 'mdi-chevron-up' : 'mdi-chevron-down'" />
</template>

<script setup lang="ts">
const expanded = defineModel<boolean>({ required: true });
</script>
