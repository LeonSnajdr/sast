<template>
    <v-chip>
        <strong>{{ placeholder.name }}</strong>
        <span>: {{ placeholder.value }}</span>
    </v-chip>
</template>

<script setup lang="ts">
import type { Placeholder } from "@/bindings";

defineProps<{
    placeholder: Placeholder;
}>();
</script>
