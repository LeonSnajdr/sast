<template>
    <v-btn icon size="x-small">
        <v-icon :icon="icon" :color="color" size="20" />
    </v-btn>
</template>

<script setup lang="ts">
defineProps<{
    icon: string;
    color?: string;
}>();
</script>
