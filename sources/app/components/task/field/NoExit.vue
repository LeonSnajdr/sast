<template>
    <VRowSingle>
        <VCheckbox v-bind="$attrs" v-model="noExit" :label="$t('task.field.noExit')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const noExit = defineModel<boolean>();
</script>
