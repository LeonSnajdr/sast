<template>
    <VChip @click:close.prevent="props.destroy()" :color="''" class="mx-1" density="comfortable" variant="tonal" closable>
        <template #prepend>
            <PlaceholderIcon :visibility="props.item.placeholderVisibility!" class="ml-n1 mr-1" />
        </template>
        {{ props.item.placeholderName }}
    </VChip>
</template>

<script setup lang="ts">
import type { InsertProps } from "vue-text-insert";

const props = defineProps<InsertProps<PlaceholderInsertTileContract>>();
</script>
