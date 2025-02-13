<template>
    <VChip
        @click:close.prevent="props.destroy()"
        :prependIcon="iconMap[props.item.placeholderVisibility!]"
        class="mx-1"
        size="small"
        :color="''"
        variant="tonal"
        closable
    >
        {{ props.item.placeholderName }}
    </VChip>
</template>

<script setup lang="ts">
import type { InsertProps } from "vue-text-insert";

const props = defineProps<InsertProps<PlaceholderInsertTileContract>>();

const iconMap: Record<PlaceholderVisibility, string> = {
    Global: "mdi-web",
    Project: "mdi-map-marker-radius"
};
</script>
