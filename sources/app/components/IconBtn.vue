<template>
    <VBtn size="x-small" variant="text">
        <VIcon :icon="icon" size="large" />
        <slot />
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
}>();
</script>
