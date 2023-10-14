<template>
    <v-btn :disabled="disabled" :loading="loading" icon size="x-small">
        <v-icon :icon="icon" :color="color" />
        <slot></slot>
    </v-btn>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
    color?: string;
    disabled?: boolean;
    loading?: boolean;
}>();
</script>
