<template>
    <VBtn size="x-small" variant="text">
        <VIcon :icon="icon" />
        <slot />
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
    iconSize?: string;
}>();
</script>
