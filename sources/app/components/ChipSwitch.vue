<template>
    <VChip @click="isSelected = !isSelected" :color="isSelected ? 'primary' : ''" :variant="isSelected ? 'flat' : 'tonal'" density="comfortable">
        <slot v-if="!text" />
        {{ text }}
    </VChip>
</template>

<script setup lang="ts" generic="T">
defineProps<{
    text?: string;
}>();

const isSelected = defineModel<boolean>({ required: true });
</script>
