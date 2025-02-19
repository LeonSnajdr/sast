<template>
    <VRowSingle>
        <VCheckbox v-model="noExit" :label="$t('task.field.noExit')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const noExit = defineModel<boolean>();
</script>
