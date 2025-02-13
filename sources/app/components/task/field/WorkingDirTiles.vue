<template>
    <VRowSingle>
        <PlaceholderInsertEditor v-model="workingDirTiles" :label="$t('task.field.workingDirTiles')" />
    </VRowSingle>
</template>

<script setup lang="ts">
const workingDirTiles = defineModel<PlaceholderInsertTileContract[]>({ required: true });
</script>
