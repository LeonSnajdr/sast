<template>
    <VRow>
        <VCol :class="colClass">
            <slot />
        </VCol>
    </VRow>
</template>

<script setup lang="ts">
defineProps<{
    colClass?: string;
}>();
</script>
