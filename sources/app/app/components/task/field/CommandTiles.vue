<template>
    <VRowSingle>
        <PlaceholderInsertEditor v-model="commandTiles" :label="$t('task.field.commandTiles')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const commandTiles = defineModel<PlaceholderInsertTileContract[]>({ required: true });
</script>
