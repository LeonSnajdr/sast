<template>
    <VBtn size="small" variant="text">
        <span class="d-flex ga-1 align-center">
            <VIcon v-if="icon" :color="iconColor" :icon="icon" size="large" />
            <slot />
        </span>
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    icon?: string;
    iconColor?: string;
}>();
</script>
