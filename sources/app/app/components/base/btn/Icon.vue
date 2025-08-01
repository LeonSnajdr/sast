<template>
    <VBtn class="text-body-2" variant="text">
        <VIcon v-if="icon" :color="iconColor" :icon="icon" />
        <slot />
    </VBtn>
</template>

<script setup lang="ts">
defineProps<{
    icon?: string;
    iconColor?: string;
}>();
</script>

<style lang="scss" scoped>
@use "@/assets/styles/settings";

:deep(.v-btn__content) {
    display: flex;
    gap: settings.$spacer;
}
</style>
