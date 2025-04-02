<template>
    <VIcon :color="placeholderIconMap[visibility].color" :icon="placeholderIconMap[visibility].icon" />
</template>

<script setup lang="ts">
defineProps<{
    visibility: PlaceholderVisibility;
}>();

const placeholderIconMap: Record<PlaceholderVisibility, { icon: string; color: string }> = {
    Global: {
        icon: "mdi-web",
        color: "info"
    },
    Project: {
        icon: "mdi-map-marker",
        color: "success"
    }
};
</script>
