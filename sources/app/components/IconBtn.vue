<template>
    <VBtn size="x-small" variant="text">
        <VIcon :color="iconColor" :icon="icon" size="large" />
        <slot />
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
    iconColor?: string;
}>();
</script>
