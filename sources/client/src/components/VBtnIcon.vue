<template>
    <v-btn :disabled="disabled" :loading="loading" icon size="x-small" color="transparent" :to="to">
        <v-icon :icon="icon" :color="color" />
        <slot></slot>
    </v-btn>
</template>

<script setup lang="ts">
import type { RouteLocationRaw } from "vue-router";

defineProps<{
    icon: string;
    color?: string;
    disabled?: boolean;
    loading?: boolean;
    to?: RouteLocationRaw;
}>();
</script>
