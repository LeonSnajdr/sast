<template>
    <VRowSingle>
        <VSwitch v-model="blocking" :label="$t('task.field.blocking')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const blocking = defineModel<boolean>();
</script>
