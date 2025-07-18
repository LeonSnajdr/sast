<template>
    <VRowSingle>
        <VCheckbox v-bind="$attrs" v-model="forceKill" :label="$t('task.field.forceKill')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const forceKill = defineModel<boolean>();
</script>
